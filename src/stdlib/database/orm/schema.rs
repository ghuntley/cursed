/// Schema management system for CURSED ORM
/// 
/// Provides schema introspection, comparison, and generation
/// with database-agnostic DDL creation for multiple databases.

use std::collections::HashMap;
use std::fmt::Debug;
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind};
use super::entity::{Entity, ColumnDefinition, SqlColumnType, IndexDefinition, ForeignKeyDefinition};

/// fr fr Database schema representation
#[derive(Debug, Clone)]
pub struct DatabaseSchema {
    /// Schema name
    pub name: String,
    /// Tables in the schema
    pub tables: HashMap<String, TableSchema>,
    /// Applied migrations
    pub applied_migrations: Vec<String>,
    /// Schema version
    pub version: String,
}

impl DatabaseSchema {
    /// slay Create new database schema
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new database schema");
        Self {
            name: "main".to_string(),
            tables: HashMap::new(),
            applied_migrations: Vec::new(),
            version: "0.0.0".to_string(),
        }
    }

    /// facts Add table to schema
    #[instrument(skip(self))]
    pub fn add_table(&mut self, table: TableSchema) {
        debug!(table = %table.name, "Adding table to schema");
        self.tables.insert(table.name.clone(), table);
    }

    /// periodt Remove table from schema
    #[instrument(skip(self))]
    pub fn remove_table(&mut self, table_name: &str) {
        debug!(table = table_name, "Removing table from schema");
        self.tables.remove(table_name);
    }

    /// bestie Get table by name
    pub fn get_table(&self, table_name: &str) -> Option<&TableSchema> {
        self.tables.get(table_name)
    }

    /// yolo Get mutable table by name
    pub fn get_table_mut(&mut self, table_name: &str) -> Option<&mut TableSchema> {
        self.tables.get_mut(table_name)
    }

    /// slay Get applied migrations
    pub fn get_applied_migrations(&self) -> &[String] {
        &self.applied_migrations
    }

    /// lit Add applied migration
    pub fn add_migration(&mut self, migration_version: String) {
        if !self.applied_migrations.contains(&migration_version) {
            self.applied_migrations.push(migration_version);
            self.applied_migrations.sort();
        }
    }

    /// tea Generate CREATE statements for all tables
    #[instrument(skip(self))]
    pub fn to_create_statements(&self, dialect: &str) -> Vec<String> {
        debug!(dialect = dialect, "Generating CREATE statements");
        
        let mut statements = Vec::new();
        
        // Generate table creation statements
        for table in self.tables.values() {
            statements.push(table.to_create_sql(dialect));
        }
        
        // Generate foreign key constraint statements
        for table in self.tables.values() {
            statements.extend(table.to_foreign_key_statements(dialect));
        }
        
        // Generate index creation statements
        for table in self.tables.values() {
            statements.extend(table.to_index_statements(dialect));
        }
        
        info!(statements = statements.len(), "Generated CREATE statements");
        statements
    }
}

/// fr fr Table schema representation
#[derive(Debug, Clone)]
pub struct TableSchema {
    /// Table name
    pub name: String,
    /// Columns in the table
    pub columns: HashMap<String, ColumnSchema>,
    /// Primary key columns
    pub primary_keys: Vec<String>,
    /// Foreign keys
    pub foreign_keys: Vec<ForeignKeyDefinition>,
    /// Indexes
    pub indexes: Vec<IndexDefinition>,
    /// Table constraints
    pub constraints: Vec<TableConstraint>,
}

impl TableSchema {
    /// slay Create new table schema
    #[instrument]
    pub fn new(name: &str) -> Self {
        debug!(table = name, "Creating new table schema");
        Self {
            name: name.to_string(),
            columns: HashMap::new(),
            primary_keys: Vec::new(),
            foreign_keys: Vec::new(),
            indexes: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// facts Create table schema from entity
    #[instrument]
    pub fn from_entity<T: Entity + Debug>() -> Result<Self, DatabaseError> {
        info!(entity = T::table_name(), "Creating table schema from entity");
        
        let mut table = Self::new(T::table_name());
        let column_definitions = T::column_definitions();
        
        for col_def in column_definitions {
            let column = ColumnSchema {
                name: col_def.name.clone(),
                sql_type: col_def.sql_type.clone(),
                nullable: col_def.nullable,
                default_value: col_def.default,
                auto_increment: col_def.primary_key && matches!(col_def.sql_type, SqlColumnType::Integer | SqlColumnType::BigInteger),
            };
            
            table.add_column(column);
            
            if col_def.primary_key {
                table.primary_keys.push(col_def.name);
            }
            
            if let Some(fk) = col_def.foreign_key {
                table.foreign_keys.push(fk);
            }
        }
        
        debug!(columns = table.columns.len(), "Table schema created from entity");
        Ok(table)
    }

    /// periodt Add column to table
    #[instrument(skip(self))]
    pub fn add_column(&mut self, column: ColumnSchema) {
        debug!(table = %self.name, column = %column.name, "Adding column to table");
        self.columns.insert(column.name.clone(), column);
    }

    /// bestie Remove column from table
    #[instrument(skip(self))]
    pub fn remove_column(&mut self, column_name: &str) {
        debug!(table = %self.name, column = column_name, "Removing column from table");
        self.columns.remove(column_name);
    }

    /// yolo Add index to table
    #[instrument(skip(self))]
    pub fn add_index(&mut self, index: IndexDefinition) {
        debug!(table = %self.name, index = %index.name, "Adding index to table");
        self.indexes.push(index);
    }

    /// slay Generate CREATE TABLE SQL
    #[instrument(skip(self))]
    pub fn to_create_sql(&self, dialect: &str) -> String {
        debug!(table = %self.name, dialect = dialect, "Generating CREATE TABLE SQL");
        
        let mut sql = format!("CREATE TABLE {} (\n", self.name);
        
        // Add columns
        let column_definitions: Vec<String> = self.columns.values()
            .map(|col| col.to_sql_definition(dialect))
            .collect();
        
        sql.push_str(&format!("  {}", column_definitions.join(",\n  ")));
        
        // Add primary key constraint
        if !self.primary_keys.is_empty() {
            sql.push_str(&format!(",\n  PRIMARY KEY ({})", self.primary_keys.join(", ")));
        }
        
        // Add table constraints
        for constraint in &self.constraints {
            sql.push_str(&format!(",\n  {}", constraint.to_sql(dialect)));
        }
        
        sql.push_str("\n);");
        
        debug!(sql = %sql, "Generated CREATE TABLE SQL");
        sql
    }

    /// lit Generate foreign key constraint statements
    #[instrument(skip(self))]
    pub fn to_foreign_key_statements(&self, dialect: &str) -> Vec<String> {
        debug!(table = %self.name, "Generating foreign key statements");
        
        self.foreign_keys.iter()
            .map(|fk| format!(
                "ALTER TABLE {} ADD CONSTRAINT fk_{}_{} FOREIGN KEY ({}) REFERENCES {} ({}) ON DELETE {} ON UPDATE {};",
                self.name,
                self.name,
                fk.referenced_table,
                self.name, // Assuming foreign key column name matches table name + "_id"
                fk.referenced_table,
                fk.referenced_column,
                fk.on_delete,
                fk.on_update
            ))
            .collect()
    }

    /// tea Generate index creation statements
    #[instrument(skip(self))]
    pub fn to_index_statements(&self, dialect: &str) -> Vec<String> {
        debug!(table = %self.name, "Generating index statements");
        
        self.indexes.iter()
            .map(|idx| {
                let unique_keyword = if idx.unique { "UNIQUE " } else { "" };
                let condition_clause = if let Some(condition) = &idx.condition {
                    format!(" WHERE {}", condition)
                } else {
                    String::new()
                };
                
                format!(
                    "CREATE {}INDEX {} ON {} ({}){};",
                    unique_keyword,
                    idx.name,
                    self.name,
                    idx.columns.join(", "),
                    condition_clause
                )
            })
            .collect()
    }
}

/// fr fr Column schema representation
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    /// Column name
    pub name: String,
    /// SQL data type
    pub sql_type: SqlColumnType,
    /// Whether column allows NULL
    pub nullable: bool,
    /// Default value
    pub default_value: Option<String>,
    /// Whether column is auto-increment
    pub auto_increment: bool,
}

impl ColumnSchema {
    /// facts Generate SQL column definition
    #[instrument(skip(self))]
    pub fn to_sql_definition(&self, dialect: &str) -> String {
        let mut sql = format!("{} {}", self.name, self.sql_type.to_sql(dialect));
        
        if self.auto_increment {
            match dialect {
                "postgresql" => sql.push_str(" SERIAL"),
                "mysql" => sql.push_str(" AUTO_INCREMENT"),
                "sqlite" => sql.push_str(" AUTOINCREMENT"),
                _ => {}
            }
        }
        
        if !self.nullable {
            sql.push_str(" NOT NULL");
        }
        
        if let Some(default) = &self.default_value {
            sql.push_str(&format!(" DEFAULT {}", default));
        }
        
        sql
    }
}

/// fr fr Table constraint types
#[derive(Debug, Clone)]
pub enum TableConstraint {
    /// Check constraint
    Check { name: String, condition: String },
    /// Unique constraint
    Unique { name: String, columns: Vec<String> },
    /// Custom constraint
    Custom { definition: String },
}

impl TableConstraint {
    /// Generate SQL for constraint
    pub fn to_sql(&self, _dialect: &str) -> String {
        match self {
            TableConstraint::Check { name, condition } => {
                format!("CONSTRAINT {} CHECK ({})", name, condition)
            }
            TableConstraint::Unique { name, columns } => {
                format!("CONSTRAINT {} UNIQUE ({})", name, columns.join(", "))
            }
            TableConstraint::Custom { definition } => definition.clone(),
        }
    }
}

/// fr fr Index schema representation (already defined in entity.rs)
pub use super::entity::IndexDefinition as IndexSchema;

/// fr fr Schema builder for fluent schema creation
#[derive(Debug)]
pub struct SchemaBuilder {
    /// Schema being built
    schema: DatabaseSchema,
}

impl SchemaBuilder {
    /// slay Create new schema builder
    #[instrument]
    pub fn new(name: &str) -> Self {
        info!(schema = name, "Creating new schema builder");
        Self {
            schema: DatabaseSchema {
                name: name.to_string(),
                tables: HashMap::new(),
                applied_migrations: Vec::new(),
                version: "1.0.0".to_string(),
            },
        }
    }

    /// facts Add table to schema
    #[instrument(skip(self, builder_fn))]
    pub fn table<F>(mut self, name: &str, builder_fn: F) -> Self
    where
        F: FnOnce(TableBuilder) -> TableBuilder,
    {
        debug!(table = name, "Adding table to schema");
        
        let table_builder = TableBuilder::new(name);
        let table = builder_fn(table_builder).build();
        self.schema.add_table(table);
        
        self
    }

    /// periodt Build the schema
    #[instrument(skip(self))]
    pub fn build(self) -> DatabaseSchema {
        info!(schema = %self.schema.name, tables = self.schema.tables.len(), "Building schema");
        self.schema
    }
}

/// fr fr Table builder for fluent table creation
#[derive(Debug)]
pub struct TableBuilder {
    /// Table being built
    table: TableSchema,
}

impl TableBuilder {
    /// slay Create new table builder
    pub fn new(name: &str) -> Self {
        Self {
            table: TableSchema::new(name),
        }
    }

    /// facts Add column to table
    #[instrument(skip(self))]
    pub fn column(mut self, name: &str, sql_type: SqlColumnType) -> ColumnBuilder {
        debug!(table = %self.table.name, column = name, "Adding column to table");
        ColumnBuilder::new(self, name, sql_type)
    }

    /// periodt Add primary key
    #[instrument(skip(self))]
    pub fn primary_key(mut self, columns: &[&str]) -> Self {
        debug!(table = %self.table.name, columns = ?columns, "Setting primary key");
        self.table.primary_keys = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// bestie Add index
    #[instrument(skip(self))]
    pub fn index(mut self, name: &str, columns: &[&str]) -> Self {
        debug!(table = %self.table.name, index = name, columns = ?columns, "Adding index");
        
        let index = IndexDefinition {
            name: name.to_string(),
            columns: columns.iter().map(|s| s.to_string()).collect(),
            unique: false,
            index_type: super::entity::IndexType::BTree,
            condition: None,
        };
        
        self.table.add_index(index);
        self
    }

    /// yolo Add unique index
    #[instrument(skip(self))]
    pub fn unique_index(mut self, name: &str, columns: &[&str]) -> Self {
        debug!(table = %self.table.name, index = name, columns = ?columns, "Adding unique index");
        
        let index = IndexDefinition {
            name: name.to_string(),
            columns: columns.iter().map(|s| s.to_string()).collect(),
            unique: true,
            index_type: super::entity::IndexType::BTree,
            condition: None,
        };
        
        self.table.add_index(index);
        self
    }

    /// slay Build the table
    pub fn build(self) -> TableSchema {
        self.table
    }

    /// Internal method to add column from ColumnBuilder
    fn add_column_schema(&mut self, column: ColumnSchema) {
        self.table.add_column(column);
    }
}

/// fr fr Column builder for fluent column creation
#[derive(Debug)]
pub struct ColumnBuilder {
    /// Table builder reference
    table_builder: TableBuilder,
    /// Column being built
    column: ColumnSchema,
}

impl ColumnBuilder {
    /// Create new column builder
    fn new(table_builder: TableBuilder, name: &str, sql_type: SqlColumnType) -> Self {
        Self {
            table_builder,
            column: ColumnSchema {
                name: name.to_string(),
                sql_type,
                nullable: true,
                default_value: None,
                auto_increment: false,
            },
        }
    }

    /// facts Make column not nullable
    pub fn not_null(mut self) -> Self {
        self.column.nullable = false;
        self
    }

    /// periodt Set default value
    pub fn default_value(mut self, value: &str) -> Self {
        self.column.default_value = Some(value.to_string());
        self
    }

    /// bestie Make column auto-increment
    pub fn auto_increment(mut self) -> Self {
        self.column.auto_increment = true;
        self
    }

    /// yolo Finish column and return to table builder
    pub fn end_column(mut self) -> TableBuilder {
        self.table_builder.add_column_schema(self.column);
        self.table_builder
    }
}

/// fr fr Schema comparator for finding differences between schemas
#[derive(Debug)]
pub struct SchemaComparator;

impl SchemaComparator {
    /// slay Compare two schemas and find differences
    #[instrument(skip(current, target))]
    pub fn compare(current: &DatabaseSchema, target: &DatabaseSchema) -> SchemaDiff {
        info!("Comparing database schemas");
        
        let mut diff = SchemaDiff::new();
        
        // Find added tables
        for (table_name, table_schema) in &target.tables {
            if !current.tables.contains_key(table_name) {
                diff.added_tables.push(table_schema.clone());
            }
        }
        
        // Find removed tables
        for (table_name, table_schema) in &current.tables {
            if !target.tables.contains_key(table_name) {
                diff.removed_tables.push(table_schema.clone());
            }
        }
        
        // Find modified tables
        for (table_name, target_table) in &target.tables {
            if let Some(current_table) = current.tables.get(table_name) {
                let table_diff = Self::compare_tables(current_table, target_table);
                if !table_diff.is_empty() {
                    diff.modified_tables.push(table_diff);
                }
            }
        }
        
        info!(
            added = diff.added_tables.len(),
            removed = diff.removed_tables.len(),
            modified = diff.modified_tables.len(),
            "Schema comparison completed"
        );
        
        diff
    }

    /// facts Compare two tables
    fn compare_tables(current: &TableSchema, target: &TableSchema) -> TableDiff {
        let mut diff = TableDiff::new(&target.name);
        
        // Find added columns
        for (col_name, col_schema) in &target.columns {
            if !current.columns.contains_key(col_name) {
                diff.added_columns.push(col_schema.clone());
            }
        }
        
        // Find removed columns
        for (col_name, col_schema) in &current.columns {
            if !target.columns.contains_key(col_name) {
                diff.removed_columns.push(col_schema.clone());
            }
        }
        
        // Find modified columns
        for (col_name, target_col) in &target.columns {
            if let Some(current_col) = current.columns.get(col_name) {
                if !Self::columns_equal(current_col, target_col) {
                    diff.modified_columns.push((current_col.clone(), target_col.clone()));
                }
            }
        }
        
        diff
    }

    /// periodt Check if two columns are equal
    fn columns_equal(current: &ColumnSchema, target: &ColumnSchema) -> bool {
        current.name == target.name &&
        current.sql_type == target.sql_type &&
        current.nullable == target.nullable &&
        current.default_value == target.default_value &&
        current.auto_increment == target.auto_increment
    }
}

/// fr fr Schema difference representation
#[derive(Debug, Clone)]
pub struct SchemaDiff {
    /// Tables added in target schema
    pub added_tables: Vec<TableSchema>,
    /// Tables removed from current schema
    pub removed_tables: Vec<TableSchema>,
    /// Tables modified between schemas
    pub modified_tables: Vec<TableDiff>,
}

impl SchemaDiff {
    fn new() -> Self {
        Self {
            added_tables: Vec::new(),
            removed_tables: Vec::new(),
            modified_tables: Vec::new(),
        }
    }

    /// Check if schema diff is empty
    pub fn is_empty(&self) -> bool {
        self.added_tables.is_empty() && 
        self.removed_tables.is_empty() && 
        self.modified_tables.is_empty()
    }
}

/// fr fr Table difference representation
#[derive(Debug, Clone)]
pub struct TableDiff {
    /// Table name
    pub table_name: String,
    /// Columns added
    pub added_columns: Vec<ColumnSchema>,
    /// Columns removed
    pub removed_columns: Vec<ColumnSchema>,
    /// Columns modified (current, target)
    pub modified_columns: Vec<(ColumnSchema, ColumnSchema)>,
}

impl TableDiff {
    fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            added_columns: Vec::new(),
            removed_columns: Vec::new(),
            modified_columns: Vec::new(),
        }
    }

    /// Check if table diff is empty
    pub fn is_empty(&self) -> bool {
        self.added_columns.is_empty() && 
        self.removed_columns.is_empty() && 
        self.modified_columns.is_empty()
    }
}

/// fr fr Schema migrator for applying schema changes
#[derive(Debug)]
pub struct SchemaMigrator;

impl SchemaMigrator {
    /// slay Generate migration SQL from schema diff
    #[instrument(skip(diff))]
    pub fn generate_migration_sql(diff: &SchemaDiff, dialect: &str) -> Vec<String> {
        info!("Generating migration SQL from schema diff");
        
        let mut statements = Vec::new();
        
        // Create new tables
        for table in &diff.added_tables {
            statements.push(table.to_create_sql(dialect));
        }
        
        // Drop removed tables
        for table in &diff.removed_tables {
            statements.push(format!("DROP TABLE {};", table.name));
        }
        
        // Modify existing tables
        for table_diff in &diff.modified_tables {
            statements.extend(Self::generate_table_modification_sql(table_diff, dialect));
        }
        
        info!(statements = statements.len(), "Generated migration SQL");
        statements
    }

    /// facts Generate SQL for table modifications
    fn generate_table_modification_sql(table_diff: &TableDiff, dialect: &str) -> Vec<String> {
        let mut statements = Vec::new();
        
        // Add new columns
        for column in &table_diff.added_columns {
            statements.push(format!(
                "ALTER TABLE {} ADD COLUMN {};",
                table_diff.table_name,
                column.to_sql_definition(dialect)
            ));
        }
        
        // Drop removed columns
        for column in &table_diff.removed_columns {
            statements.push(format!(
                "ALTER TABLE {} DROP COLUMN {};",
                table_diff.table_name,
                column.name
            ));
        }
        
        // Modify existing columns
        for (current, target) in &table_diff.modified_columns {
            match dialect {
                "postgresql" => {
                    statements.push(format!(
                        "ALTER TABLE {} ALTER COLUMN {} TYPE {};",
                        table_diff.table_name,
                        target.name,
                        target.sql_type.to_sql(dialect)
                    ));
                }
                "mysql" => {
                    statements.push(format!(
                        "ALTER TABLE {} MODIFY COLUMN {};",
                        table_diff.table_name,
                        target.to_sql_definition(dialect)
                    ));
                }
                _ => {
                    // SQLite doesn't support ALTER COLUMN, would need table recreation
                    warn!("Column modification not supported for dialect: {}", dialect);
                }
            }
        }
        
        statements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_database_schema_creation() {
        let schema = DatabaseSchema::new();
        
        assert_eq!(schema.name, "main");
        assert!(schema.tables.is_empty());
        assert!(schema.applied_migrations.is_empty());
    }

    #[traced_test]
    #[test]
    fn test_table_schema_creation() {
        let table = TableSchema::new("users");
        
        assert_eq!(table.name, "users");
        assert!(table.columns.is_empty());
        assert!(table.primary_keys.is_empty());
    }

    #[traced_test]
    #[test]
    fn test_schema_builder() {
        let schema = SchemaBuilder::new("test_db")
            .table("users", |table| {
                table
                    .column("id", SqlColumnType::BigInteger).auto_increment().not_null().end_column()
                    .column("name", SqlColumnType::VarChar { length: 255 }).not_null().end_column()
                    .column("email", SqlColumnType::VarChar { length: 255 }).end_column()
                    .primary_key(&["id"])
                    .index("idx_users_email", &["email"])
            })
            .build();
        
        assert_eq!(schema.name, "test_db");
        assert_eq!(schema.tables.len(), 1);
        
        let users_table = schema.get_table("users").unwrap();
        assert_eq!(users_table.columns.len(), 3);
        assert_eq!(users_table.primary_keys, Vec::from(["id"]));
        assert_eq!(users_table.indexes.len(), 1);
    }

    #[traced_test]
    #[test]
    fn test_table_create_sql() {
        let mut table = TableSchema::new("users");
        
        table.add_column(ColumnSchema {
            name: "id".to_string(),
            sql_type: SqlColumnType::BigInteger,
            nullable: false,
            default_value: None,
            auto_increment: true,
        });
        
        table.add_column(ColumnSchema {
            name: "name".to_string(),
            sql_type: SqlColumnType::VarChar { length: 255 },
            nullable: false,
            default_value: None,
            auto_increment: false,
        });
        
        table.primary_keys.push("id".to_string());
        
        let sql = table.to_create_sql("postgresql");
        
        assert!(sql.contains("CREATE TABLE users"));
        assert!(sql.contains("id BIGINT"));
        assert!(sql.contains("name VARCHAR(255)"));
        assert!(sql.contains("PRIMARY KEY (id)"));
    }

    #[traced_test]
    #[test]
    fn test_schema_comparison() {
        let mut current_schema = DatabaseSchema::new();
        let mut target_schema = DatabaseSchema::new();
        
        // Current schema has users table
        let mut users_table = TableSchema::new("users");
        users_table.add_column(ColumnSchema {
            name: "id".to_string(),
            sql_type: SqlColumnType::BigInteger,
            nullable: false,
            default_value: None,
            auto_increment: true,
        });
        current_schema.add_table(users_table);
        
        // Target schema has users table with additional column and posts table
        let mut users_table_target = TableSchema::new("users");
        users_table_target.add_column(ColumnSchema {
            name: "id".to_string(),
            sql_type: SqlColumnType::BigInteger,
            nullable: false,
            default_value: None,
            auto_increment: true,
        });
        users_table_target.add_column(ColumnSchema {
            name: "email".to_string(),
            sql_type: SqlColumnType::VarChar { length: 255 },
            nullable: true,
            default_value: None,
            auto_increment: false,
        });
        target_schema.add_table(users_table_target);
        
        let posts_table = TableSchema::new("posts");
        target_schema.add_table(posts_table);
        
        let diff = SchemaComparator::compare(&current_schema, &target_schema);
        
        assert_eq!(diff.added_tables.len(), 1);
        assert_eq!(diff.added_tables[0].name, "posts");
        assert_eq!(diff.modified_tables.len(), 1);
        assert_eq!(diff.modified_tables[0].added_columns.len(), 1);
        assert_eq!(diff.modified_tables[0].added_columns[0].name, "email");
    }

    #[traced_test]
    #[test]
    fn test_migration_sql_generation() {
        let mut diff = SchemaDiff::new();
        
        // Add a new table
        let mut new_table = TableSchema::new("posts");
        new_table.add_column(ColumnSchema {
            name: "id".to_string(),
            sql_type: SqlColumnType::BigInteger,
            nullable: false,
            default_value: None,
            auto_increment: true,
        });
        diff.added_tables.push(new_table);
        
        let sql_statements = SchemaMigrator::generate_migration_sql(&diff, "postgresql");
        
        assert_eq!(sql_statements.len(), 1);
        assert!(sql_statements[0].contains("CREATE TABLE posts"));
    }
}
