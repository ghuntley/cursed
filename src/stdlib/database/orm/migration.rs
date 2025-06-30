//! I/O functionality for migration

use crate::error::CursedError;
use std::io::{self, Read, Write};
use std::collections::HashMap;

/// Result type for I/O operations
pub type IOResult<T> = Result<T, CursedError>;

/// Create table migration operation
#[derive(Debug, Clone)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
    pub constraints: Vec<TableConstraint>,
}

/// Drop table migration operation
#[derive(Debug, Clone)]
pub struct DropTable {
    pub table_name: String,
    pub if_exists: bool,
}

/// Add column migration operation
#[derive(Debug, Clone)]
pub struct AddColumn {
    pub table_name: String,
    pub column: ColumnDefinition,
}

/// Drop column migration operation
#[derive(Debug, Clone)]
pub struct DropColumn {
    pub table_name: String,
    pub column_name: String,
}

/// Add index migration operation
#[derive(Debug, Clone)]
pub struct AddIndex {
    pub table_name: String,
    pub index_name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

/// Column definition for migrations
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub auto_increment: bool,
}

/// Column types for database schemas
#[derive(Debug, Clone)]
pub enum ColumnType {
    Integer,
    BigInteger,
    SmallInteger,
    Float,
    Double,
    Decimal { precision: u8, scale: u8 },
    Boolean,
    Char { length: u16 },
    Varchar { length: u16 },
    Text,
    LongText,
    Binary { length: u16 },
    VarBinary { length: u16 },
    Blob,
    Date,
    Time,
    DateTime,
    Timestamp,
    Json,
    Uuid,
}

/// Table constraints for migrations
#[derive(Debug, Clone)]
pub enum TableConstraint {
    PrimaryKey { columns: Vec<String> },
    ForeignKey {
        columns: Vec<String>,
        references_table: String,
        references_columns: Vec<String>,
        on_delete: Option<String>,
        on_update: Option<String>,
    },
    Unique { columns: Vec<String> },
    Check { expression: String },
}

impl CreateTable {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    pub fn add_column(mut self, column: ColumnDefinition) -> Self {
        self.columns.push(column);
        self
    }
    
    pub fn add_constraint(mut self, constraint: TableConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }
}

impl DropTable {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            if_exists: false,
        }
    }
    
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }
}

impl AddColumn {
    pub fn new(table_name: &str, column: ColumnDefinition) -> Self {
        Self {
            table_name: table_name.to_string(),
            column,
        }
    }
}

impl DropColumn {
    pub fn new(table_name: &str, column_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            column_name: column_name.to_string(),
        }
    }
}

impl AddIndex {
    pub fn new(table_name: &str, index_name: &str, columns: Vec<String>) -> Self {
        Self {
            table_name: table_name.to_string(),
            index_name: index_name.to_string(),
            columns,
            unique: false,
        }
    }
    
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}

impl ColumnDefinition {
    pub fn new(name: &str, column_type: ColumnType) -> Self {
        Self {
            name: name.to_string(),
            column_type,
            nullable: true,
            default_value: None,
            auto_increment: false,
        }
    }
    
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }
    
    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = Some(value.to_string());
        self
    }
    
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }
}

/// I/O operations handler
pub struct IOHandler {
    buffer_size: usize,
}

impl IOHandler {
    /// Create a new I/O handler
    pub fn new() -> Self {
        Self {
            buffer_size: 8192,
        }
    }
    
    /// Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// Read from a reader
    pub fn read_all<R: Read>(&self, mut reader: R) -> IOResult<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)
            .map_err(|e| CursedError::runtime_error(&format!("Read error: {}", e)))?;
        Ok(buffer)
    }
    
    /// Write to a writer
    pub fn write_all<W: Write>(&self, mut writer: W, data: &[u8]) -> IOResult<()> {
        writer.write_all(data)
            .map_err(|e| CursedError::runtime_error(&format!("Write error: {}", e)))?;
        Ok(())
    }
    
    /// Read string from reader
    pub fn read_string<R: Read>(&self, reader: R) -> IOResult<String> {
        let bytes = self.read_all(reader)?;
        String::from_utf8(bytes)
            .map_err(|e| CursedError::runtime_error(&format!("UTF-8 decode error: {}", e)))
    }
    
    /// Write string to writer
    pub fn write_string<W: Write>(&self, writer: W, text: &str) -> IOResult<()> {
        self.write_all(writer, text.as_bytes())
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize I/O processing
pub fn init_migration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::runtime_error("I/O test failed"));
    }
    println!("📁 I/O processing (migration) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_migration() -> IOResult<()> {
    let handler = IOHandler::new();
    let test_string = "Hello, CURSED I/O!";
    let mut buffer = Vec::new();
    handler.write_string(&mut buffer, test_string)?;
    let result = handler.read_string(std::io::Cursor::new(&buffer))?;
    if result != test_string {
        return Err(CursedError::runtime_error("I/O string test failed"));
    }
    Ok(())
}
