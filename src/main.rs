#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use calamine::{open_workbook, Data, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{command, generate_context, generate_handler, Manager, State, Window};

mod format;
use format::convert_cell_value;

// 数据结构定义
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SplitConfig {
    input_file: String,
    split_sheets: Vec<String>,
    header_row: usize,
    split_column: String,
    public_sheets: Vec<String>,
    output_dir: String,
    filename_template: String,
    empty_handling: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SplitResult {
    group: String,
    rows: usize,
    filepath: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SheetInfo {
    name: String,
    columns: Vec<String>,
    data_rows: usize,
    total_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileInfo {
    filepath: String,
    sheet_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProgressUpdate {
    current: usize,
    total: usize,
    message: String,
}

// Excel 拆分器核心逻辑
struct ExcelSplitter {
    filepath: String,
    workbook: Xlsx<fs::File>,
    sheet_names: Vec<String>,
}

impl ExcelSplitter {
    fn new(filepath: &str) -> Result<Self> {
        let workbook = open_workbook(filepath)
            .with_context(|| format!("Failed to open workbook: {}", filepath))?;
        
        let sheet_names = workbook.sheet_names().to_vec();
        
        Ok(Self {
            filepath: filepath.to_string(),
            workbook,
            sheet_names,
        })
    }

    fn get_sheet_info(&mut self, sheet_name: &str, header_row: usize) -> Result<SheetInfo> {
        let range = self.workbook.worksheet_range(sheet_name)
            .with_context(|| format!("Sheet '{}' not found", sheet_name))?
            .with_context(|| format!("Failed to read sheet '{}'", sheet_name))?;

        let mut columns = Vec::new();
        let header_row_idx = header_row.saturating_sub(1);
        
        for col_idx in 0..range.width() {
            if let Some(cell) = range.get_value((header_row_idx as u32, col_idx as u32)) {
                let col_name = cell.to_string();
                if !col_name.is_empty() {
                    columns.push(col_name);
                }
            }
        }

        let data_rows = if range.height() > header_row {
            range.height() - header_row
        } else {
            0
        };

        Ok(SheetInfo {
            name: sheet_name.to_string(),
            columns,
            data_rows,
            total_rows: range.height(),
        })
    }

    fn get_all_groups(
        &mut self,
        split_sheets: &[String],
        header_row: usize,
        split_column: &str,
        empty_handling: &str,
    ) -> Result<Vec<String>> {
        let mut groups: Vec<String> = Vec::new();
        
        for sheet_name in split_sheets {
            let range = self.workbook.worksheet_range(sheet_name)
                .with_context(|| format!("Sheet '{}' not found", sheet_name))?
                .with_context(|| format!("Failed to read sheet '{}'", sheet_name))?;

            let col_idx = self.find_column_index(&range, header_row, split_column)?;
            
            for row_idx in header_row..range.height() {
                let value = if let Some(cell) = range.get_value((row_idx as u32, col_idx as u32)) {
                    cell.to_string()
                } else {
                    String::new()
                };

                let group_value = if value.trim().is_empty() {
                    match empty_handling {
                        "skip" => continue,
                        "stop" => anyhow::bail!("Empty value found in column '{}' at row {}", split_column, row_idx + 1),
                        _ => "(empty)".to_string(),
                    }
                } else {
                    value
                };

                if !groups.contains(&group_value) {
                    groups.push(group_value);
                }
            }
        }
        
        groups.sort();
        Ok(groups)
    }

    fn find_column_index(
        &self,
        range: &calamine::Range<Data>,
        header_row: usize,
        column_name: &str,
    ) -> Result<usize> {
        let header_row_idx = header_row.saturating_sub(1);
        
        for col_idx in 0..range.width() {
            if let Some(cell) = range.get_value((header_row_idx as u32, col_idx as u32)) {
                if cell.to_string().trim() == column_name {
                    return Ok(col_idx);
                }
            }
        }
        
        anyhow::bail!("Column '{}' not found in header row {}", column_name, header_row)
    }

    fn split_by_column(
        &mut self,
        config: &SplitConfig,
        window: &Window,
    ) -> Result<Vec<SplitResult>> {
        fs::create_dir_all(&config.output_dir)?;
        
        let groups = self.get_all_groups(
            &config.split_sheets,
            config.header_row,
            &config.split_column,
            &config.empty_handling,
        )?;
        
        let base_name = Path::new(&self.filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let mut results = Vec::new();
        let total = groups.len();
        
        for (idx, group_key) in groups.iter().enumerate() {
            // 发送进度更新
            let progress = ProgressUpdate {
                current: idx + 1,
                total,
                message: format!("Processing: {}", group_key),
            };
            let _ = window.emit("split-progress", &progress);
            
            let safe_key = sanitize_filename(group_key);
            let filename = config.filename_template
                .replace("{base}", base_name)
                .replace("{group}", &safe_key);
            let output_path = Path::new(&config.output_dir).join(&filename);
            
            let mut workbook = Workbook::new();
            let mut total_rows = 0;
            
            // 处理每个拆分 sheet
            for sheet_name in &config.split_sheets {
                let row_count = self.write_split_sheet(
                    &mut workbook,
                    sheet_name,
                    config.header_row,
                    &config.split_column,
                    group_key,
                )?;
                total_rows += row_count;
            }
            
            // 复制公共 sheets
            for pub_name in &config.public_sheets {
                if self.sheet_names.contains(pub_name) {
                    self.copy_public_sheet(&mut workbook, pub_name)?;
                }
            }
            
            workbook.save(&output_path)?;
            
            results.push(SplitResult {
                group: group_key.clone(),
                rows: total_rows,
                filepath: output_path.to_string_lossy().to_string(),
            });
        }
        
        // 发送完成事件
        let _ = window.emit("split-complete", &results);
        
        Ok(results)
    }

    fn write_split_sheet(
        &mut self,
        workbook: &mut Workbook,
        sheet_name: &str,
        header_row: usize,
        split_column: &str,
        group_key: &str,
    ) -> Result<usize> {
        let range = self.workbook.worksheet_range(sheet_name)
            .with_context(|| format!("Sheet '{}' not found", sheet_name))?
            .with_context(|| format!("Failed to read sheet '{}'", sheet_name))?;
        
        let col_idx = self.find_column_index(&range, header_row, split_column)?;
        let mut worksheet = workbook.add_worksheet(sheet_name)?;
        
        // 复制表头行
        for row_idx in 0..header_row {
            for col_idx_write in 0..range.width() {
                if let Some(value) = range.get_value((row_idx as u32, col_idx_write as u32)) {
                    let cell_value = convert_cell_value(value);
                    worksheet.write(row_idx as u16, col_idx_write as u16, cell_value)?;
                }
            }
        }
        
        // 复制该分组的数据行
        let mut target_row = header_row;
        let mut row_count = 0;
        
        for row_idx in header_row..range.height() {
            let cell_value = if let Some(cell) = range.get_value((row_idx as u32, col_idx as u32)) {
                cell.to_string()
            } else {
                String::new()
            };

            let is_match = if cell_value.trim().is_empty() {
                group_key == "(empty)"
            } else {
                cell_value == group_key
            };

            if is_match {
                for col_idx_write in 0..range.width() {
                    if let Some(value) = range.get_value((row_idx as u32, col_idx_write as u32)) {
                        let cell_value = convert_cell_value(value);
                        worksheet.write(target_row as u16, col_idx_write as u16, cell_value)?;
                    }
                }
                target_row += 1;
                row_count += 1;
            }
        }
        
        // 自动调整列宽
        for col_idx in 0..range.width() {
            worksheet.set_column_width(col_idx as u16, 15.0)?;
        }
        
        Ok(row_count)
    }

    fn copy_public_sheet(
        &mut self,
        workbook: &mut Workbook,
        sheet_name: &str,
    ) -> Result<()> {
        let range = self.workbook.worksheet_range(sheet_name)
            .with_context(|| format!("Sheet '{}' not found", sheet_name))?
            .with_context(|| format!("Failed to read sheet '{}'", sheet_name))?;
        
        let mut worksheet = workbook.add_worksheet(sheet_name)?;
        
        for row_idx in 0..range.height() {
            for col_idx in 0..range.width() {
                if let Some(value) = range.get_value((row_idx as u32, col_idx as u32)) {
                    let cell_value = convert_cell_value(value);
                    worksheet.write(row_idx as u16, col_idx as u16, cell_value)?;
                }
            }
        }
        
        // 自动调整列宽
        for col_idx in 0..range.width() {
            worksheet.set_column_width(col_idx as u16, 15.0)?;
        }
        
        Ok(())
    }

    fn preview_data(&mut self, sheet_name: &str, max_rows: usize) -> Result<Vec<Vec<String>>> {
        let range = self.workbook.worksheet_range(sheet_name)
            .with_context(|| format!("Sheet '{}' not found", sheet_name))?
            .with_context(|| format!("Failed to read sheet '{}'", sheet_name))?;
        
        let mut preview = Vec::new();
        let rows_to_show = max_rows.min(range.height());
        
        for row_idx in 0..rows_to_show {
            let mut row_data = Vec::new();
            for col_idx in 0..range.width() {
                let value = if let Some(cell) = range.get_value((row_idx as u32, col_idx as u32)) {
                    cell.to_string()
                } else {
                    String::new()
                };
                row_data.push(value);
            }
            preview.push(row_data);
        }
        
        Ok(preview)
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

// Tauri 命令
#[command]
async fn load_excel_file(filepath: String) -> Result<FileInfo, String> {
    let splitter = ExcelSplitter::new(&filepath)
        .map_err(|e| e.to_string())?;
    
    Ok(FileInfo {
        filepath,
        sheet_names: splitter.sheet_names,
    })
}

#[command]
async fn get_sheet_columns(filepath: String, sheet_name: String, header_row: usize) -> Result<Vec<String>, String> {
    let mut splitter = ExcelSplitter::new(&filepath)
        .map_err(|e| e.to_string())?;
    
    let info = splitter.get_sheet_info(&sheet_name, header_row)
        .map_err(|e| e.to_string())?;
    
    Ok(info.columns)
}

#[command]
async fn preview_sheet_data(filepath: String, sheet_name: String, max_rows: usize) -> Result<Vec<Vec<String>>, String> {
    let mut splitter = ExcelSplitter::new(&filepath)
        .map_err(|e| e.to_string())?;
    
    splitter.preview_data(&sheet_name, max_rows)
        .map_err(|e| e.to_string())
}

#[command]
async fn split_excel(
    window: Window,
    config: SplitConfig,
) -> Result<Vec<SplitResult>, String> {
    let mut splitter = ExcelSplitter::new(&config.input_file)
        .map_err(|e| e.to_string())?;
    
    splitter.split_by_column(&config, &window)
        .map_err(|e| e.to_string())
}

#[command]
async fn select_file() -> Result<Option<String>, String> {
    // 这个命令需要在前端通过 Tauri API 调用对话框
    Ok(None)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            load_excel_file,
            get_sheet_columns,
            preview_sheet_data,
            split_excel,
            select_file,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
