/// fr fr Database metadata - knowing your database inside and out periodt
///
/// This module provides comprehensive metadata about database structures,
/// including tables, columns, indexes, and constraints. Knowledge is power bestie!

// use crate::stdlib::packages::db_core::{ColumnType, DatabaseValue};
// use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};
use std::collections::HashMap;

/// fr fr Complete database metadata
#[derive(Debug, Clone)]
pub struct DatabaseMetadata {
    pub database_name: String,
    pub database_version: String,
    pub schemas: Vec<SchemaInfo>,
    pub tables: Vec<TableMetadata>,
    pub views: Vec<ViewInfo>,
    pub indexes: Vec<IndexInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
    pub constraints: Vec<ConstraintInfo>,
    pub procedures: Vec<ProcedureInfo>,
    pub functions: Vec<FunctionInfo>,
    pub statistics: Vec<StatisticsInfo>,
    pub permissions: Vec<PermissionInfo>,
}

impl DatabaseMetadata {
    /// slay Create new database metadata
    pub fn new(database_name: &str, database_version: &str) -> Self {
        Self {
            database_name: database_name.to_string(),
            database_version: database_version.to_string(),
            schemas: Vec::new(),
            tables: Vec::new(),
            views: Vec::new(),
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
            constraints: Vec::new(),
            procedures: Vec::new(),
            functions: Vec::new(),
            statistics: Vec::new(),
            permissions: Vec::new(),
        }
    }

    /// slay Get table by name
    pub fn get_table(&self, name: &str) -> Option<&TableMetadata> {
        self.tables.iter().find(|table| table.name == name)
    }

    /// slay Get schema by name
    pub fn get_schema(&self, name: &str) -> Option<&SchemaInfo> {
        self.schemas.iter().find(|schema| schema.name == name)
    }

    /// slay Get tables in schema
    pub fn get_tables_in_schema(&self, schema_name: &str) -> Vec<&TableMetadata> {
        self.tables
            .iter()
            .filter(|table| table.schema_name.as_ref() == Some(&schema_name.to_string()))
            .collect()
    }

    /// slay Add table metadata
    pub fn add_table(&mut self, table: TableMetadata) {
        self.tables.push(table);
    }

    /// slay Add schema info
    pub fn add_schema(&mut self, schema: SchemaInfo) {
        self.schemas.push(schema);
    }
}

/// fr fr Table metadata information
#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub schema_name: Option<String>,
    pub table_type: TableType,
    pub columns: Vec<ColumnInfo>,
    pub primary_key: Option<PrimaryKeyInfo>,
    pub foreign_keys: Vec<ForeignKeyInfo>,
    pub indexes: Vec<IndexInfo>,
    pub constraints: Vec<ConstraintInfo>,
    pub row_count: Option<u64>,
    pub size_bytes: Option<u64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub comment: Option<String>,
}

impl TableMetadata {
    /// slay Create new table metadata
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            schema_name: None,
            table_type: TableType::Table,
            columns: Vec::new(),
            primary_key: None,
            foreign_keys: Vec::new(),
            indexes: Vec::new(),
            constraints: Vec::new(),
            row_count: None,
            size_bytes: None,
            created_at: None,
            modified_at: None,
            comment: None,
        }
    }

    /// slay Get column by name
    pub fn get_column(&self, name: &str) -> Option<&ColumnInfo> {
        self.columns.iter().find(|col| col.name == name)
    }

    /// slay Add column
    pub fn add_column(&mut self, column: ColumnInfo) {
        self.columns.push(column);
    }

    /// slay Get primary key columns
    pub fn get_primary_key_columns(&self) -> Vec<&str> {
        if let Some(pk) = &self.primary_key {
            pk.columns.iter().map(|s| s.as_str()).collect()
        } else {
            Vec::new()
        }
    }
}

/// fr fr Column information
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: ColumnType,
    pub is_nullable: bool,
    pub default_value: Option<DatabaseValue>,
    pub is_auto_increment: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub max_length: Option<u32>,
    pub precision: Option<u32>,
    pub scale: Option<u32>,
    pub collation: Option<String>,
    pub comment: Option<String>,
    pub ordinal_position: u32,
}

impl ColumnInfo {
    /// slay Create new column info
    pub fn new(name: &str, data_type: ColumnType, ordinal_position: u32) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            is_nullable: true,
            default_value: None,
            is_auto_increment: false,
            is_primary_key: false,
            is_unique: false,
            max_length: None,
            precision: None,
            scale: None,
            collation: None,
            comment: None,
            ordinal_position,
        }
    }
}

/// fr fr Index information
#[derive(Debug, Clone)]
pub struct IndexInfo {
    pub name: String,
    pub table_name: String,
    pub schema_name: Option<String>,
    pub index_type: IndexType,
    pub is_unique: bool,
    pub is_primary: bool,
    pub columns: Vec<IndexColumnInfo>,
    pub filter_condition: Option<String>,
    pub size_bytes: Option<u64>,
    pub row_count: Option<u64>,
    pub comment: Option<String>,
}

impl IndexInfo {
    /// slay Create new index info
    pub fn new(name: &str, table_name: &str) -> Self {
        Self {
            name: name.to_string(),
            table_name: table_name.to_string(),
            schema_name: None,
            index_type: IndexType::BTree,
            is_unique: false,
            is_primary: false,
            columns: Vec::new(),
            filter_condition: None,
            size_bytes: None,
            row_count: None,
            comment: None,
        }
    }
}

/// fr fr Index column information
#[derive(Debug, Clone)]
pub struct IndexColumnInfo {
    pub column_name: String,
    pub ordinal_position: u32,
    pub sort_order: SortOrder,
    pub is_included: bool,
}

/// fr fr Foreign key information
#[derive(Debug, Clone)]
pub struct ForeignKeyInfo {
    pub name: String,
    pub table_name: String,
    pub schema_name: Option<String>,
    pub columns: Vec<String>,
    pub referenced_table_name: String,
    pub referenced_schema_name: Option<String>,
    pub referenced_columns: Vec<String>,
    pub on_delete: ReferentialAction,
    pub on_update: ReferentialAction,
    pub is_deferrable: bool,
    pub is_initially_deferred: bool,
}

/// fr fr Constraint information
#[derive(Debug, Clone)]
pub struct ConstraintInfo {
    pub name: String,
    pub table_name: String,
    pub schema_name: Option<String>,
    pub constraint_type: ConstraintType,
    pub columns: Vec<String>,
    pub check_clause: Option<String>,
    pub is_deferrable: bool,
    pub is_initially_deferred: bool,
}

/// fr fr Schema information
#[derive(Debug, Clone)]
pub struct SchemaInfo {
    pub name: String,
    pub owner: Option<String>,
    pub default_charset: Option<String>,
    pub default_collation: Option<String>,
    pub comment: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl SchemaInfo {
    /// slay Create new schema info
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            owner: None,
            default_charset: None,
            default_collation: None,
            comment: None,
            created_at: None,
        }
    }
}

/// fr fr Primary key information
#[derive(Debug, Clone)]
pub struct PrimaryKeyInfo {
    pub name: String,
    pub columns: Vec<String>,
}

/// fr fr Statistics information
#[derive(Debug, Clone)]
pub struct StatisticsInfo {
    pub table_name: String,
    pub schema_name: Option<String>,
    pub column_name: Option<String>,
    pub index_name: Option<String>,
    pub statistic_type: StatisticType,
    pub value: DatabaseValue,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

/// fr fr View information
#[derive(Debug, Clone)]
pub struct ViewInfo {
    pub name: String,
    pub schema_name: Option<String>,
    pub definition: String,
    pub is_updatable: bool,
    pub check_option: Option<String>,
    pub columns: Vec<ColumnInfo>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub comment: Option<String>,
}

/// fr fr Procedure information
#[derive(Debug, Clone)]
pub struct ProcedureInfo {
    pub name: String,
    pub schema_name: Option<String>,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: Option<ColumnType>,
    pub language: String,
    pub definition: String,
    pub is_deterministic: bool,
    pub security_type: SecurityType,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub comment: Option<String>,
}

/// fr fr Function information
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub schema_name: Option<String>,
    pub parameters: Vec<ParameterInfo>,
    pub return_type: ColumnType,
    pub language: String,
    pub definition: String,
    pub is_deterministic: bool,
    pub security_type: SecurityType,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub comment: Option<String>,
}

/// fr fr Parameter information for procedures/functions
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub data_type: ColumnType,
    pub parameter_mode: ParameterMode,
    pub default_value: Option<DatabaseValue>,
    pub ordinal_position: u32,
}

/// fr fr Permission information
#[derive(Debug, Clone)]
pub struct PermissionInfo {
    pub grantee: String,
    pub grantor: String,
    pub object_name: String,
    pub object_type: ObjectType,
    pub privilege: String,
    pub is_grantable: bool,
    pub granted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// fr fr Enumerations for metadata types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    Table,
    View,
    MaterializedView,
    TemporaryTable,
    SystemTable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexType {
    BTree,
    Hash,
    Bitmap,
    Gin,
    Gist,
    Spgist,
    FullText,
    Spatial,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
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
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    NotNull,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatisticType {
    RowCount,
    IndexCardinality,
    ColumnCardinality,
    DataLength,
    IndexLength,
    Selectivity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityType {
    Definer,
    Invoker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterMode {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
    Table,
    View,
    Procedure,
    Function,
    Schema,
    Database,
}

