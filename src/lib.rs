pub mod format;

use anyhow::{Context, Result};
use calamine::{open_workbook, Data, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ExcelSplitter {
    filepath: String,
    workbook: Xlsx<fs::File>,
    pub sheet_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SplitResult {
    pub group: String,
    pub rows: usize,
    pub filepath: String,
}

impl ExcelSplitter {
    pub fn new(filepath: &str) -> Result<Self> {
        let workbook = open_workbook(filepath)
            .with_context(|| format!("Failed to open workbook: {}", filepath))?;
        
        let sheet_names = workbook.sheet_names().to_vec();
        
        Ok(Self {
            filepath: filepath.to_string(),
            workbook,
            sheet_names,
        })
    }

    pub fn split_by_column(
        &mut self,
        split_sheets: &[String],
        header_row: usize,
        split_column: &str,
        public_sheets: &[String],
        output_dir: &str,
        template: &str,
    ) -> Result<Vec<SplitResult>> {
        fs::create_dir_all(output_dir)?;
        
        let groups = self.get_all_groups(split_sheets, header_row, split_column)?;
        let base_name = Path::new(&self.filepath)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        
        let mut results = Vec::new();
        
        for group_key in &groups {
            let safe_key = sanitize_filename(group_key);
            let filename = template
                .replace("{base}", base_name)
                .replace("{group}", &safe_key);
            let output_path = Path::new(output_dir).join(&filename);
            
            let mut workbook = Workbook::new();
            let mut total_rows = 0;
            
            // Process each split sheet
            for sheet_name in split_sheets {
                let row_count = self.write_split_sheet(
                    &mut workbook,
                    sheet_name,
                    header_row,
                    split_column,
                    group_key,
                )?;
                total_rows += row_count;
            }
            
            // Copy public sheets
            for pub_name in public_sheets {
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
        
        Ok(results)
    }

    fn get_all_groups(
        &mut self,
        split_sheets: &[String],
        header_row: usize,
        split_column: &str,
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
                    "(empty)".to_string()
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
        
        // Copy header rows
        for row_idx in 0..header_row {
            for col_idx_write in 0..range.width() {
                if let Some(value) = range.get_value((row_idx as u32, col_idx_write as u32)) {
                    let cell_value = crate::format::convert_cell_value(value);
                    worksheet.write(row_idx as u16, col_idx_write as u16, cell_value)?;
                }
            }
        }
        
        // Copy data rows for this group
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
                        let cell_value = crate::format::convert_cell_value(value);
                        worksheet.write(target_row as u16, col_idx_write as u16, cell_value)?;
                    }
                }
                target_row += 1;
                row_count += 1;
            }
        }
        
        // Auto-fit columns
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
                    let cell_value = crate::format::convert_cell_value(value);
                    worksheet.write(row_idx as u16, col_idx as u16, cell_value)?;
                }
            }
        }
        
        // Auto-fit columns
        for col_idx in 0..range.width() {
            worksheet.set_column_width(col_idx as u16, 15.0)?;
        }
        
        Ok(())
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
