/// fr fr Database metadata management - knowing your schema periodt
///
/// This module provides database schema introspection, metadata extraction,
/// and information about tables, columns, indexes, and constraints. Knowledge is power bestie!

use crate::stdlib::packages::db_core::{
    DatabaseResult as DbResult, DatabaseError, ErrorKind
};
use std::collections::HashMap;
use std::time::SystemTime;

/// fr fr Main database metadata interface
#[derive(Debug, Clone)]
pub struct DatabaseMetadata {
    /// Database name
    pub database_name: String,
    /// Database version
    pub version: String,
    /// Schema information
    pub schemas: Vec<SchemaInfo>,
    /// Database-level properties
    pub properties: HashMap<String, String>,
    /// When metadata was last updated
    pub last_updated: SystemTime,
}

/// fr fr Schema information
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    /// Schema name
    pub name: String,
    /// Schema owner
    pub owner: Option<String>,
    /// Tables in this schema
    pub tables: Vec<TableMetadata>,
    /// Views in this schema
    pub views: Vec<ViewMetadata>,
    /// Functions/procedures in this schema
    pub functions: Vec<FunctionMetadata>,
    /// Schema comment
    pub comment: Option<String>,
}

/// fr fr Table metadata
#[derive(Debug, Clone)]
pub struct TableMetadata {
    /// Table name
    pub name: String,
    /// Schema name
    pub schema_name: String,
    /// Table type (BASE TABLE, VIEW, etc.)
    pub table_type: TableType,
    /// Column information
    pub columns: Vec<ColumnInfo>,
    /// Primary key information
    pub primary_key: Option<PrimaryKeyInfo>,
    /// Foreign key constraints
    pub foreign_keys: Vec<ForeignKeyInfo>,
    /// Unique constraints
    pub unique_constraints: Vec<UniqueConstraintInfo>,
    /// Check constraints
    pub check_constraints: Vec<CheckConstraintInfo>,
    /// Index information
    pub indexes: Vec<IndexInfo>,
    /// Table statistics
    pub statistics: Option<StatisticsInfo>,
    /// Table comment
    pub comment: Option<String>,
}

/// fr fr Column information
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    /// Column name
    pub name: String,
    /// Column ordinal position (1-based)
    pub ordinal_position: u32,
    /// Data type
    pub data_type: String,
    /// Maximum length (for string/binary types)
    pub character_maximum_length: Option<u32>,
    /// Numeric precision
    pub numeric_precision: Option<u32>,
    /// Numeric scale
    pub numeric_scale: Option<u32>,
    /// Whether column is nullable
    pub is_nullable: bool,
    /// Default value
    pub column_default: Option<String>,
    /// Whether column is auto-increment
    pub is_auto_increment: bool,
    /// Column comment
    pub comment: Option<String>,
}

/// fr fr Primary key information
#[derive(Debug, Clone)]
pub struct PrimaryKeyInfo {
    /// Constraint name
    pub constraint_name: String,
    /// Column names in primary key
    pub column_names: Vec<String>,
}

/// fr fr Foreign key constraint information
#[derive(Debug, Clone)]
pub struct ForeignKeyInfo {
    /// Constraint name
    pub constraint_name: String,
    /// Local column names
    pub column_names: Vec<String>,
    /// Referenced table schema
    pub referenced_table_schema: String,
    /// Referenced table name
    pub referenced_table_name: String,
    /// Referenced column names
    pub referenced_column_names: Vec<String>,
    /// Update rule
    pub update_rule: ReferentialAction,
    /// Delete rule
    pub delete_rule: ReferentialAction,
}

/// fr fr Unique constraint information
#[derive(Debug, Clone)]
pub struct UniqueConstraintInfo {
    /// Constraint name
    pub constraint_name: String,
    /// Column names in constraint
    pub column_names: Vec<String>,
}

/// fr fr Check constraint information
#[derive(Debug, Clone)]
pub struct CheckConstraintInfo {
    /// Constraint name
    pub constraint_name: String,
    /// Check expression
    pub check_clause: String,
}

/// fr fr Index information
#[derive(Debug, Clone)]
pub struct IndexInfo {
    /// Index name
    pub index_name: String,
    /// Whether index is unique
    pub is_unique: bool,
    /// Whether index is primary
    pub is_primary: bool,
    /// Index type (BTREE, HASH, etc.)
    pub index_type: String,
    /// Column information
    pub columns: Vec<IndexColumnInfo>,
    /// Index size in bytes
    pub size_bytes: Option<u64>,
    /// Index cardinality
    pub cardinality: Option<u64>,
}

/// fr fr Index column information
#[derive(Debug, Clone)]
pub struct IndexColumnInfo {
    /// Column name
    pub column_name: String,
    /// Column position in index (1-based)
    pub ordinal_position: u32,
    /// Sort order (ASC/DESC)
    pub sort_order: SortOrder,
    /// Whether column allows nulls in index
    pub is_nullable: bool,
}

/// fr fr View metadata
#[derive(Debug, Clone)]
pub struct ViewMetadata {
    /// View name
    pub name: String,
    /// Schema name
    pub schema_name: String,
    /// View definition (SQL)
    pub definition: String,
    /// Whether view is updatable
    pub is_updatable: bool,
    /// View comment
    pub comment: Option<String>,
}

/// fr fr Function/procedure metadata
#[derive(Debug, Clone)]
pub struct FunctionMetadata {
    /// Function name
    pub name: String,
    /// Schema name
    pub schema_name: String,
    /// Function type
    pub function_type: FunctionType,
    /// Return type
    pub return_type: Option<String>,
    /// Parameter information
    pub parameters: Vec<ParameterMetadata>,
    /// Function definition
    pub definition: Option<String>,
    /// Programming language
    pub language: String,
    /// Function comment
    pub comment: Option<String>,
}

/// fr fr Parameter metadata for functions
#[derive(Debug, Clone)]
pub struct ParameterMetadata {
    /// Parameter name
    pub name: String,
    /// Parameter position (1-based)
    pub ordinal_position: u32,
    /// Parameter mode (IN, OUT, INOUT)
    pub parameter_mode: ParameterMode,
    /// Data type
    pub data_type: String,
}

/// fr fr Table statistics
#[derive(Debug, Clone)]
pub struct StatisticsInfo {
    /// Number of rows (approximate)
    pub row_count: Option<u64>,
    /// Table size in bytes
    pub size_bytes: Option<u64>,
    /// Index size in bytes
    pub index_size_bytes: Option<u64>,
    /// When statistics were last updated
    pub last_analyzed: Option<SystemTime>,
}

/// fr fr Enumerations for metadata types

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    BaseTable,
    View,
    SystemTable,
    SystemView,
    Temporary,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferentialAction {
    NoAction,
    Restrict,
    Cascade,
    SetNull,
    SetDefault,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionType {
    Function,
    Procedure,
    Aggregate,
    Window,
    Table,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterMode {
    In,
    Out,
    InOut,
}

impl DatabaseMetadata {
    /// slay Create new database metadata
    pub fn new(database_name: &str, version: &str) -> Self {
        Self {
            database_name: database_name.to_string(),
            version: version.to_string(),
            schemas: Vec::new(),
            properties: HashMap::new(),
            last_updated: SystemTime::now(),
        }
    }

    /// slay Add schema to database
    pub fn add_schema(&mut self, schema: SchemaInfo) {
        self.schemas.push(schema);
        self.last_updated = SystemTime::now();
    }

    /// slay Find schema by name
    pub fn find_schema(&self, name: &str) -> Option<&SchemaInfo> {
        self.schemas.iter().find(|s| s.name == name)
    }

    /// slay Find table by name (searches all schemas)
    pub fn find_table(&self, table_name: &str) -> Option<&TableMetadata> {
        for schema in &self.schemas {
            if let Some(table) = schema.tables.iter().find(|t| t.name == table_name) {
                return Some(table);
            }
        }
        None
    }

    /// slay Find table by schema and name
    pub fn find_table_in_schema(&self, schema_name: &str, table_name: &str) -> Option<&TableMetadata> {
        self.find_schema(schema_name)?
            .tables.iter()
            .find(|t| t.name == table_name)
    }

    /// slay Get all table names
    pub fn get_all_table_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for schema in &self.schemas {
            for table in &schema.tables {
                names.push(format!("{}.{}", schema.name, table.name));
            }
        }
        names
    }
}

impl SchemaInfo {
    /// slay Create new schema info
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            owner: None,
            tables: Vec::new(),
            views: Vec::new(),
            functions: Vec::new(),
            comment: None,
        }
    }

    /// slay Add table to schema
    pub fn add_table(&mut self, table: TableMetadata) {
        self.tables.push(table);
    }

    /// slay Find table by name
    pub fn find_table(&self, name: &str) -> Option<&TableMetadata> {
        self.tables.iter().find(|t| t.name == name)
    }
}

impl TableMetadata {
    /// slay Create new table metadata
    pub fn new(name: &str, schema_name: &str, table_type: TableType) -> Self {
        Self {
            name: name.to_string(),
            schema_name: schema_name.to_string(),
            table_type,
            columns: Vec::new(),
            primary_key: None,
            foreign_keys: Vec::new(),
            unique_constraints: Vec::new(),
            check_constraints: Vec::new(),
            indexes: Vec::new(),
            statistics: None,
            comment: None,
        }
    }

    /// slay Add column to table
    pub fn add_column(&mut self, column: ColumnInfo) {
        self.columns.push(column);
    }

    /// slay Find column by name
    pub fn find_column(&self, name: &str) -> Option<&ColumnInfo> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// slay Get primary key column names
    pub fn get_primary_key_columns(&self) -> Vec<&str> {
        if let Some(pk) = &self.primary_key {
            pk.column_names.iter().map(|s| s.as_str()).collect()
        } else {
            Vec::new()
        }
    }

    /// slay Check if table has primary key
    pub fn has_primary_key(&self) -> bool {
        self.primary_key.is_some()
    }

    /// slay Get foreign key constraints
    pub fn get_foreign_keys(&self) -> &[ForeignKeyInfo] {
        &self.foreign_keys
    }

    /// slay Get indexes
    pub fn get_indexes(&self) -> &[IndexInfo] {
        &self.indexes
    }

    /// slay Get full table name (schema.table)
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.schema_name, self.name)
    }
}

impl ColumnInfo {
    /// slay Create new column info
    pub fn new(name: &str, data_type: &str, ordinal_position: u32) -> Self {
        Self {
            name: name.to_string(),
            ordinal_position,
            data_type: data_type.to_string(),
            character_maximum_length: None,
            numeric_precision: None,
            numeric_scale: None,
            is_nullable: true,
            column_default: None,
            is_auto_increment: false,
            comment: None,
        }
    }

    /// slay Set column as not nullable
    pub fn not_null(mut self) -> Self {
        self.is_nullable = false;
        self
    }

    /// slay Set default value
    pub fn with_default(mut self, default: &str) -> Self {
        self.column_default = Some(default.to_string());
        self
    }

    /// slay Set as auto-increment
    pub fn auto_increment(mut self) -> Self {
        self.is_auto_increment = true;
        self
    }

    /// slay Set character length
    pub fn with_max_length(mut self, length: u32) -> Self {
        self.character_maximum_length = Some(length);
        self
    }

    /// slay Set numeric precision and scale
    pub fn with_precision_scale(mut self, precision: u32, scale: u32) -> Self {
        self.numeric_precision = Some(precision);
        self.numeric_scale = Some(scale);
        self
    }
}

impl IndexInfo {
    /// slay Create new index info
    pub fn new(name: &str, is_unique: bool) -> Self {
        Self {
            index_name: name.to_string(),
            is_unique,
            is_primary: false,
            index_type: "BTREE".to_string(),
            columns: Vec::new(),
            size_bytes: None,
            cardinality: None,
        }
    }

    /// slay Add column to index
    pub fn add_column(&mut self, column: IndexColumnInfo) {
        self.columns.push(column);
    }

    /// slay Set as primary index
    pub fn primary(mut self) -> Self {
        self.is_primary = true;
        self.is_unique = true;
        self
    }

    /// slay Set index type
    pub fn with_type(mut self, index_type: &str) -> Self {
        self.index_type = index_type.to_string();
        self
    }
}

impl IndexColumnInfo {
    /// slay Create new index column info
    pub fn new(column_name: &str, ordinal_position: u32, sort_order: SortOrder) -> Self {
        Self {
            column_name: column_name.to_string(),
            ordinal_position,
            sort_order,
            is_nullable: true,
        }
    }

    /// slay Set as ascending
    pub fn ascending(mut self) -> Self {
        self.sort_order = SortOrder::Ascending;
        self
    }

    /// slay Set as descending
    pub fn descending(mut self) -> Self {
        self.sort_order = SortOrder::Descending;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_metadata_creation() {
        let mut db_meta = DatabaseMetadata::new("test_db", "1.0");
        assert_eq!(db_meta.database_name, "test_db");
        assert_eq!(db_meta.version, "1.0");
        assert!(db_meta.schemas.is_empty());

        let schema = SchemaInfo::new("public");
        db_meta.add_schema(schema);
        assert_eq!(db_meta.schemas.len(), 1);
    }

    #[test]
    fn test_table_metadata() {
        let mut table = TableMetadata::new("users", "public", TableType::BaseTable);
        
        let id_column = ColumnInfo::new("id", "INTEGER", 1)
            .not_null()
            .auto_increment();
        
        let name_column = ColumnInfo::new("name", "VARCHAR", 2)
            .with_max_length(255)
            .not_null();

        table.add_column(id_column);
        table.add_column(name_column);

        assert_eq!(table.columns.len(), 2);
        assert!(table.find_column("id").is_some());
        assert!(table.find_column("name").is_some());
        assert!(table.find_column("nonexistent").is_none());
        assert_eq!(table.full_name(), "public.users");
    }

    #[test]
    fn test_column_info_builder() {
        let column = ColumnInfo::new("price", "DECIMAL", 1)
            .with_precision_scale(10, 2)
            .with_default("0.00")
            .not_null();

        assert_eq!(column.name, "price");
        assert_eq!(column.data_type, "DECIMAL");
        assert_eq!(column.numeric_precision, Some(10));
        assert_eq!(column.numeric_scale, Some(2));
        assert_eq!(column.column_default, Some("0.00".to_string()));
        assert!(!column.is_nullable);
    }

    #[test]
    fn test_index_info() {
        let mut index = IndexInfo::new("idx_users_email", true)
            .with_type("BTREE");

        index.add_column(IndexColumnInfo::new("email", 1, SortOrder::Ascending));

        assert!(index.is_unique);
        assert_eq!(index.index_type, "BTREE");
        assert_eq!(index.columns.len(), 1);
        assert_eq!(index.columns[0].column_name, "email");
    }

    #[test]
    fn test_schema_operations() {
        let mut schema = SchemaInfo::new("app_schema");
        
        let table1 = TableMetadata::new("users", "app_schema", TableType::BaseTable);
        let table2 = TableMetadata::new("orders", "app_schema", TableType::BaseTable);

        schema.add_table(table1);
        schema.add_table(table2);

        assert_eq!(schema.tables.len(), 2);
        assert!(schema.find_table("users").is_some());
        assert!(schema.find_table("orders").is_some());
        assert!(schema.find_table("nonexistent").is_none());
    }

    #[test]
    fn test_metadata_search() {
        let mut db_meta = DatabaseMetadata::new("test_db", "1.0");
        
        let mut schema = SchemaInfo::new("public");
        let table = TableMetadata::new("users", "public", TableType::BaseTable);
        schema.add_table(table);
        db_meta.add_schema(schema);

        assert!(db_meta.find_schema("public").is_some());
        assert!(db_meta.find_table("users").is_some());
        assert!(db_meta.find_table_in_schema("public", "users").is_some());
        assert!(db_meta.find_table_in_schema("public", "nonexistent").is_none());

        let table_names = db_meta.get_all_table_names();
        assert_eq!(table_names, vec!["public.users"]);
    }

    #[test]
    fn test_enums() {
        assert_eq!(TableType::BaseTable, TableType::BaseTable);
        assert_ne!(TableType::BaseTable, TableType::View);

        assert_eq!(SortOrder::Ascending, SortOrder::Ascending);
        assert_ne!(SortOrder::Ascending, SortOrder::Descending);

        assert_eq!(FunctionType::Function, FunctionType::Function);
        assert_ne!(FunctionType::Function, FunctionType::Procedure);
    }
}
