/// fr fr Object-relational mapping utilities

/// fr fr Table mapper for struct-to-table mapping
#[derive(Debug, Clone)]
pub struct TableMapper {
    pub table_name: String,
    pub columns: Vec<ColumnMapper>,
}

/// fr fr Column mapper for field-to-column mapping
#[derive(Debug, Clone)]
pub struct ColumnMapper {
    pub field_name: String,
    pub column_name: String,
    pub column_type: String,
}

impl TableMapper {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: Vec::new(),
        }
    }
}

impl ColumnMapper {
    pub fn new(field_name: &str, column_name: &str, column_type: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            column_name: column_name.to_string(),
            column_type: column_type.to_string(),
        }
    }
}
