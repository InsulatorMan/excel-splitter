use calamine::{Data, CellErrorType};

/// Convert calamine cell value to string representation
pub fn convert_cell_value(value: &Data) -> String {
    match value {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => {
            // Format float nicely
            if f.fract() == 0.0 {
                format!("{:.0}", f)
            } else {
                f.to_string()
            }
        },
        Data::Int(i) => i.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::DateTime(d) => {
            // Format as Excel date serial number or string
            d.as_f64().map(|v| v.to_string()).unwrap_or_else(|| d.to_string())
        },
        Data::Duration(d) => d.to_string(),
        Data::Error(e) => match e {
            CellErrorType::Div0 => "#DIV/0!".to_string(),
            CellErrorType::NA => "#N/A".to_string(),
            CellErrorType::Name => "#NAME?".to_string(),
            CellErrorType::Null => "#NULL!".to_string(),
            CellErrorType::Num => "#NUM!".to_string(),
            CellErrorType::Ref => "#REF!".to_string(),
            CellErrorType::Value => "#VALUE!".to_string(),
            _ => "#ERROR".to_string(),
        },
        _ => value.to_string(),
    }
}

/// Detect if a string looks like a number
pub fn is_numeric(value: &str) -> bool {
    value.parse::<f64>().is_ok()
}

/// Format a value based on its content type
pub fn format_value(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    
    // Try to preserve formulas
    if value.starts_with('=') {
        return value.to_string();
    }
    
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_cell_value() {
        assert_eq!(convert_cell_value(&Data::Empty), "");
        assert_eq!(convert_cell_value(&Data::String("test".to_string())), "test");
        assert_eq!(convert_cell_value(&Data::Int(42)), "42");
        assert_eq!(convert_cell_value(&Data::Float(3.14)), "3.14");
        assert_eq!(convert_cell_value(&Data::Bool(true)), "true");
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("123"));
        assert!(is_numeric("3.14"));
        assert!(!is_numeric("abc"));
        assert!(!is_numeric(""));
    }
}
