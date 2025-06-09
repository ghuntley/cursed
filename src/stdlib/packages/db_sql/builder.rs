/// fr fr SQL Query Builder - building those queries with style periodt
///
/// This module provides a fluent API for building SQL queries in CURSED.
/// No more string concatenation hell bestie! Build queries the right way.

use crate::stdlib::packages::db_core::{DatabaseError, QueryError};
use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};
use crate::stdlib::packages::db_sql::{SqlValue, SqlType, SqlDialectTrait};
use std::collections::HashMap;

/// fr fr Main SQL query builder - the foundation periodt
#[derive(Debug)]
pub struct SqlQueryBuilder {
    dialect: Option<Box<dyn SqlDialectTrait>>,
    parameters: Vec<SqlValue>,
    parameter_map: HashMap<String, usize>,
}

impl Clone for SqlQueryBuilder {
    fn clone(&self) -> Self {
        Self {
            // Note: We don't clone the dialect trait object since it's not cloneable
            // The cloned builder will use default parameter placeholders
            dialect: None,
            parameters: self.parameters.clone(),
            parameter_map: self.parameter_map.clone(),
        }
    }
}

impl SqlQueryBuilder {
    /// slay Create a new query builder
    pub fn new() -> Self {
        Self {
            dialect: None,
            parameters: Vec::new(),
            parameter_map: HashMap::new(),
        }
    }

    /// slay Set the SQL dialect
    pub fn with_dialect(mut self, dialect: Box<dyn SqlDialectTrait>) -> Self {
        self.dialect = Some(dialect);
        self
    }

    /// slay Start building a SELECT query
    pub fn select(&mut self) -> SelectBuilder {
        SelectBuilder::new(self)
    }

    /// slay Start building an INSERT query
    pub fn insert(&mut self) -> InsertBuilder {
        InsertBuilder::new(self)
    }

    /// slay Start building an UPDATE query
    pub fn update(&mut self) -> UpdateBuilder {
        UpdateBuilder::new(self)
    }

    /// slay Start building a DELETE query
    pub fn delete(&mut self) -> DeleteBuilder {
        DeleteBuilder::new(self)
    }

    /// slay Start building a CREATE TABLE query
    pub fn create_table(&mut self) -> CreateTableBuilder {
        CreateTableBuilder::new(self)
    }

    /// slay Start building an ALTER TABLE query
    pub fn alter_table(&mut self) -> AlterTableBuilder {
        AlterTableBuilder::new(self)
    }

    /// slay Add a parameter and return its placeholder
    pub fn add_parameter(&mut self, value: SqlValue) -> String {
        let index = self.parameters.len();
        self.parameters.push(value);
        self.get_parameter_placeholder(index)
    }

    /// slay Add a named parameter
    pub fn add_named_parameter(&mut self, name: &str, value: SqlValue) -> String {
        let index = self.parameters.len();
        self.parameters.push(value);
        self.parameter_map.insert(name.to_string(), index);
        self.get_named_parameter_placeholder(name)
    }

    /// slay Get parameter placeholder for index
    fn get_parameter_placeholder(&self, index: usize) -> String {
        if let Some(dialect) = &self.dialect {
            dialect.parameter_placeholder(index)
        } else {
            "?".to_string() // Default placeholder
        }
    }

    /// slay Get named parameter placeholder
    fn get_named_parameter_placeholder(&self, name: &str) -> String {
        if let Some(dialect) = &self.dialect {
            dialect.named_parameter_placeholder(name)
        } else {
            format!(":{}", name) // Default named placeholder
        }
    }

    /// slay Get all parameters
    pub fn parameters(&self) -> &[SqlValue] {
        &self.parameters
    }

    /// slay Clear parameters
    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
        self.parameter_map.clear();
    }
}

/// fr fr SELECT query builder
#[derive(Debug)]
pub struct SelectBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    columns: Vec<String>,
    from_clause: Option<String>,
    joins: Vec<JoinClause>,
    where_conditions: Vec<String>,
    group_by: Vec<String>,
    having_conditions: Vec<String>,
    order_by: Vec<OrderClause>,
    limit_clause: Option<u64>,
    offset_clause: Option<u64>,
    distinct: bool,
}

impl<'a> SelectBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            columns: Vec::new(),
            from_clause: None,
            joins: Vec::new(),
            where_conditions: Vec::new(),
            group_by: Vec::new(),
            having_conditions: Vec::new(),
            order_by: Vec::new(),
            limit_clause: None,
            offset_clause: None,
            distinct: false,
        }
    }

    /// slay Add columns to select
    pub fn columns(mut self, columns: &[&str]) -> Self {
        for col in columns {
            self.columns.push(col.to_string());
        }
        self
    }

    /// slay Add a single column
    pub fn column(mut self, column: &str) -> Self {
        self.columns.push(column.to_string());
        self
    }

    /// slay Add column with alias
    pub fn column_as(mut self, column: &str, alias: &str) -> Self {
        self.columns.push(format!("{} AS {}", column, alias));
        self
    }

    /// slay Set DISTINCT
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// slay Set FROM clause
    pub fn from(mut self, table: &str) -> Self {
        self.from_clause = Some(table.to_string());
        self
    }

    /// slay Add INNER JOIN
    pub fn inner_join(mut self, table: &str, on_condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table: table.to_string(),
            condition: on_condition.to_string(),
        });
        self
    }

    /// slay Add LEFT JOIN
    pub fn left_join(mut self, table: &str, on_condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table: table.to_string(),
            condition: on_condition.to_string(),
        });
        self
    }

    /// slay Add RIGHT JOIN
    pub fn right_join(mut self, table: &str, on_condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Right,
            table: table.to_string(),
            condition: on_condition.to_string(),
        });
        self
    }

    /// slay Add WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_conditions.push(condition.to_string());
        self
    }

    /// slay Add WHERE condition with parameter
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let placeholder = self.builder.add_parameter(value);
        self.where_conditions.push(format!("{} = {}", column, placeholder));
        self
    }

    /// slay Add WHERE IN condition
    pub fn where_in(mut self, column: &str, values: Vec<SqlValue>) -> Self {
        let placeholders: Vec<String> = values.into_iter()
            .map(|v| self.builder.add_parameter(v))
            .collect();
        self.where_conditions.push(format!("{} IN ({})", column, placeholders.join(", ")));
        self
    }

    /// slay Add GROUP BY
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        for col in columns {
            self.group_by.push(col.to_string());
        }
        self
    }

    /// slay Add HAVING condition
    pub fn having(mut self, condition: &str) -> Self {
        self.having_conditions.push(condition.to_string());
        self
    }

    /// slay Add ORDER BY
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by.push(OrderClause {
            column: column.to_string(),
            direction,
        });
        self
    }

    /// slay Set LIMIT
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit_clause = Some(limit);
        self
    }

    /// slay Set OFFSET
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset_clause = Some(offset);
        self
    }

    /// slay Build the final SQL query
    pub fn build(self) -> DbResult<String> {
        let mut sql = String::new();

        // SELECT clause
        sql.push_str("SELECT ");
        if self.distinct {
            sql.push_str("DISTINCT ");
        }

        if self.columns.is_empty() {
            sql.push('*');
        } else {
            sql.push_str(&self.columns.join(", "));
        }

        // FROM clause
        if let Some(from) = &self.from_clause {
            sql.push_str(&format!(" FROM {}", from));
        } else {
            return Err(DatabaseError::query(
                QueryError::SyntaxError,
                "SELECT query requires FROM clause"
            ));
        }

        // JOIN clauses
        for join in &self.joins {
            sql.push_str(&format!(" {} JOIN {} ON {}", 
                join.join_type.to_sql(), join.table, join.condition));
        }

        // WHERE clause
        if !self.where_conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
        }

        // GROUP BY clause
        if !self.group_by.is_empty() {
            sql.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }

        // HAVING clause
        if !self.having_conditions.is_empty() {
            sql.push_str(&format!(" HAVING {}", self.having_conditions.join(" AND ")));
        }

        // ORDER BY clause
        if !self.order_by.is_empty() {
            let order_clauses: Vec<String> = self.order_by.iter()
                .map(|o| format!("{} {}", o.column, o.direction.to_sql()))
                .collect();
            sql.push_str(&format!(" ORDER BY {}", order_clauses.join(", ")));
        }

        // LIMIT clause
        if let Some(limit) = self.limit_clause {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        // OFFSET clause
        if let Some(offset) = self.offset_clause {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        Ok(sql)
    }
}

/// fr fr INSERT query builder
#[derive(Debug)]
pub struct InsertBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    table: Option<String>,
    columns: Vec<String>,
    values: Vec<Vec<SqlValue>>,
    on_conflict: Option<ConflictResolution>,
}

impl<'a> InsertBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            on_conflict: None,
        }
    }

    /// slay Set table to insert into
    pub fn into(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// slay Set columns
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|c| c.to_string()).collect();
        self
    }

    /// slay Add a row of values
    pub fn values(mut self, values: Vec<SqlValue>) -> Self {
        self.values.push(values);
        self
    }

    /// slay Set conflict resolution
    pub fn on_conflict(mut self, resolution: ConflictResolution) -> Self {
        self.on_conflict = Some(resolution);
        self
    }

    /// slay Build the INSERT query
    pub fn build(self) -> DbResult<String> {
        let table = self.table.ok_or_else(|| 
            DatabaseError::query(
                QueryError::SyntaxError,
                "INSERT query requires table name"
            )
        )?;

        if self.values.is_empty() {
            return Err(DatabaseError::query(
                QueryError::SyntaxError,
                "INSERT query requires at least one row of values"
            ));
        }

        let mut sql = format!("INSERT INTO {}", table);

        // Columns clause
        if !self.columns.is_empty() {
            sql.push_str(&format!(" ({})", self.columns.join(", ")));
        }

        // VALUES clause
        sql.push_str(" VALUES ");
        let value_clauses: Vec<String> = self.values.iter()
            .map(|row| {
                let placeholders: Vec<String> = row.iter()
                    .map(|_| "?".to_string()) // Simplified for now
                    .collect();
                format!("({})", placeholders.join(", "))
            })
            .collect();
        sql.push_str(&value_clauses.join(", "));

        // Conflict resolution
        if let Some(conflict) = &self.on_conflict {
            sql.push_str(&conflict.to_sql());
        }

        Ok(sql)
    }
}

/// fr fr UPDATE query builder
#[derive(Debug)]
pub struct UpdateBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    table: Option<String>,
    set_clauses: Vec<String>,
    where_conditions: Vec<String>,
    joins: Vec<JoinClause>,
}

impl<'a> UpdateBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            table: None,
            set_clauses: Vec::new(),
            where_conditions: Vec::new(),
            joins: Vec::new(),
        }
    }

    /// slay Set table to update
    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// slay Add SET clause
    pub fn set(mut self, column: &str, value: SqlValue) -> Self {
        let placeholder = self.builder.add_parameter(value);
        self.set_clauses.push(format!("{} = {}", column, placeholder));
        self
    }

    /// slay Add WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_conditions.push(condition.to_string());
        self
    }

    /// slay Build the UPDATE query
    pub fn build(self) -> DbResult<String> {
        let table = self.table.ok_or_else(|| 
            DatabaseError::query(
                QueryError::SyntaxError,
                "UPDATE query requires table name"
            )
        )?;

        if self.set_clauses.is_empty() {
            return Err(DatabaseError::query(
                QueryError::SyntaxError,
                "UPDATE query requires at least one SET clause"
            ));
        }

        let mut sql = format!("UPDATE {}", table);

        // SET clause
        sql.push_str(&format!(" SET {}", self.set_clauses.join(", ")));

        // WHERE clause
        if !self.where_conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
        }

        Ok(sql)
    }
}

/// fr fr DELETE query builder
#[derive(Debug)]
pub struct DeleteBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    table: Option<String>,
    where_conditions: Vec<String>,
}

impl<'a> DeleteBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            table: None,
            where_conditions: Vec::new(),
        }
    }

    /// slay Set table to delete from
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }

    /// slay Add WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_conditions.push(condition.to_string());
        self
    }

    /// slay Build the DELETE query
    pub fn build(self) -> DbResult<String> {
        let table = self.table.ok_or_else(|| 
            DatabaseError::query(
                crate::stdlib::packages::db_core::QueryError::SyntaxError,
                "DELETE query requires table name"
            )
        )?;

        let mut sql = format!("DELETE FROM {}", table);

        // WHERE clause
        if !self.where_conditions.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
        }

        Ok(sql)
    }
}

/// fr fr CREATE TABLE query builder
#[derive(Debug)]
pub struct CreateTableBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    table_name: Option<String>,
    columns: Vec<ColumnDefinition>,
    constraints: Vec<TableConstraint>,
    if_not_exists: bool,
}

impl<'a> CreateTableBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            table_name: None,
            columns: Vec::new(),
            constraints: Vec::new(),
            if_not_exists: false,
        }
    }

    /// slay Set table name
    pub fn table(mut self, name: &str) -> Self {
        self.table_name = Some(name.to_string());
        self
    }

    /// slay Set IF NOT EXISTS
    pub fn if_not_exists(mut self) -> Self {
        self.if_not_exists = true;
        self
    }

    /// slay Add column
    pub fn column(mut self, name: &str, sql_type: SqlType) -> ColumnBuilder<'a> {
        ColumnBuilder::new(self, name, sql_type)
    }

    /// slay Add constraint
    pub fn constraint(mut self, constraint: TableConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// slay Build the CREATE TABLE query
    pub fn build(self) -> DbResult<String> {
        let table_name = self.table_name.ok_or_else(|| 
            DatabaseError::query(
                crate::stdlib::packages::db_core::QueryError::SyntaxError,
                "CREATE TABLE query requires table name"
            )
        )?;

        if self.columns.is_empty() {
            return Err(DatabaseError::query(
                crate::stdlib::packages::db_core::QueryError::SyntaxError,
                "CREATE TABLE query requires at least one column"
            ));
        }

        let mut sql = String::from("CREATE TABLE ");
        if self.if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        sql.push_str(&table_name);

        // Columns
        sql.push_str(" (");
        let column_defs: Vec<String> = self.columns.iter()
            .map(|col| col.to_sql())
            .collect();
        sql.push_str(&column_defs.join(", "));

        // Constraints
        if !self.constraints.is_empty() {
            let constraint_defs: Vec<String> = self.constraints.iter()
                .map(|constraint| constraint.to_sql())
                .collect();
            sql.push_str(", ");
            sql.push_str(&constraint_defs.join(", "));
        }

        sql.push(')');
        Ok(sql)
    }
}

/// fr fr ALTER TABLE query builder
#[derive(Debug)]
pub struct AlterTableBuilder<'a> {
    builder: &'a mut SqlQueryBuilder,
    table_name: Option<String>,
    operations: Vec<AlterOperation>,
}

impl<'a> AlterTableBuilder<'a> {
    fn new(builder: &'a mut SqlQueryBuilder) -> Self {
        Self {
            builder,
            table_name: None,
            operations: Vec::new(),
        }
    }

    /// slay Set table name
    pub fn table(mut self, name: &str) -> Self {
        self.table_name = Some(name.to_string());
        self
    }

    /// slay Add column
    pub fn add_column(mut self, name: &str, sql_type: SqlType) -> Self {
        self.operations.push(AlterOperation::AddColumn(ColumnDefinition {
            name: name.to_string(),
            sql_type,
            nullable: true,
            default_value: None,
            auto_increment: false,
            primary_key: false,
            unique: false,
        }));
        self
    }

    /// slay Drop column
    pub fn drop_column(mut self, name: &str) -> Self {
        self.operations.push(AlterOperation::DropColumn(name.to_string()));
        self
    }

    /// slay Build the ALTER TABLE query
    pub fn build(self) -> DbResult<String> {
        let table_name = self.table_name.ok_or_else(|| 
            DatabaseError::query(
                crate::stdlib::packages::db_core::QueryError::SyntaxError,
                "ALTER TABLE query requires table name"
            )
        )?;

        if self.operations.is_empty() {
            return Err(DatabaseError::query(
                crate::stdlib::packages::db_core::QueryError::SyntaxError,
                "ALTER TABLE query requires at least one operation"
            ));
        }

        let mut sql = format!("ALTER TABLE {}", table_name);
        let operation_sql: Vec<String> = self.operations.iter()
            .map(|op| op.to_sql())
            .collect();
        sql.push_str(&format!(" {}", operation_sql.join(", ")));

        Ok(sql)
    }
}

/// fr fr Column builder for CREATE TABLE
#[derive(Debug)]
pub struct ColumnBuilder<'a> {
    create_builder: CreateTableBuilder<'a>,
    column: ColumnDefinition,
}

impl<'a> ColumnBuilder<'a> {
    fn new(create_builder: CreateTableBuilder<'a>, name: &str, sql_type: SqlType) -> Self {
        Self {
            create_builder,
            column: ColumnDefinition {
                name: name.to_string(),
                sql_type,
                nullable: true,
                default_value: None,
                auto_increment: false,
                primary_key: false,
                unique: false,
            },
        }
    }

    /// slay Set NOT NULL
    pub fn not_null(mut self) -> Self {
        self.column.nullable = false;
        self
    }

    /// slay Set default value
    pub fn default_value(mut self, value: SqlValue) -> Self {
        self.column.default_value = Some(value);
        self
    }

    /// slay Set PRIMARY KEY
    pub fn primary_key(mut self) -> Self {
        self.column.primary_key = true;
        self.column.nullable = false;
        self
    }

    /// slay Set UNIQUE
    pub fn unique(mut self) -> Self {
        self.column.unique = true;
        self
    }

    /// slay Set AUTO_INCREMENT
    pub fn auto_increment(mut self) -> Self {
        self.column.auto_increment = true;
        self
    }

    /// slay Finish column and return to table builder
    pub fn finish(mut self) -> CreateTableBuilder<'a> {
        self.create_builder.columns.push(self.column);
        self.create_builder
    }
}

/// fr fr Supporting types and enums

#[derive(Debug, Clone)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: String,
    pub condition: String,
}

#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl JoinType {
    fn to_sql(&self) -> &'static str {
        match self {
            JoinType::Inner => "INNER",
            JoinType::Left => "LEFT",
            JoinType::Right => "RIGHT",
            JoinType::Full => "FULL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderClause {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl OrderDirection {
    fn to_sql(&self) -> &'static str {
        match self {
            OrderDirection::Asc => "ASC",
            OrderDirection::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConflictResolution {
    Ignore,
    Replace,
    Update(Vec<String>),
}

impl ConflictResolution {
    fn to_sql(&self) -> String {
        match self {
            ConflictResolution::Ignore => " ON CONFLICT DO NOTHING".to_string(),
            ConflictResolution::Replace => " ON CONFLICT DO UPDATE".to_string(),
            ConflictResolution::Update(columns) => {
                format!(" ON CONFLICT DO UPDATE SET {}", columns.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub sql_type: SqlType,
    pub nullable: bool,
    pub default_value: Option<SqlValue>,
    pub auto_increment: bool,
    pub primary_key: bool,
    pub unique: bool,
}

impl ColumnDefinition {
    fn to_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.sql_type.to_sql());
        
        if !self.nullable {
            sql.push_str(" NOT NULL");
        }
        
        if let Some(default) = &self.default_value {
            sql.push_str(&format!(" DEFAULT {}", default.to_sql()));
        }
        
        if self.auto_increment {
            sql.push_str(" AUTO_INCREMENT");
        }
        
        if self.primary_key {
            sql.push_str(" PRIMARY KEY");
        }
        
        if self.unique {
            sql.push_str(" UNIQUE");
        }
        
        sql
    }
}

#[derive(Debug, Clone)]
pub enum TableConstraint {
    PrimaryKey(Vec<String>),
    ForeignKey(String, String, String), // column, ref_table, ref_column
    Unique(Vec<String>),
    Check(String),
}

impl TableConstraint {
    fn to_sql(&self) -> String {
        match self {
            TableConstraint::PrimaryKey(columns) => {
                format!("PRIMARY KEY ({})", columns.join(", "))
            }
            TableConstraint::ForeignKey(column, ref_table, ref_column) => {
                format!("FOREIGN KEY ({}) REFERENCES {}({})", column, ref_table, ref_column)
            }
            TableConstraint::Unique(columns) => {
                format!("UNIQUE ({})", columns.join(", "))
            }
            TableConstraint::Check(condition) => {
                format!("CHECK ({})", condition)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlterOperation {
    AddColumn(ColumnDefinition),
    DropColumn(String),
    ModifyColumn(ColumnDefinition),
    AddConstraint(TableConstraint),
    DropConstraint(String),
}

impl AlterOperation {
    fn to_sql(&self) -> String {
        match self {
            AlterOperation::AddColumn(column) => {
                format!("ADD COLUMN {}", column.to_sql())
            }
            AlterOperation::DropColumn(name) => {
                format!("DROP COLUMN {}", name)
            }
            AlterOperation::ModifyColumn(column) => {
                format!("MODIFY COLUMN {}", column.to_sql())
            }
            AlterOperation::AddConstraint(constraint) => {
                format!("ADD {}", constraint.to_sql())
            }
            AlterOperation::DropConstraint(name) => {
                format!("DROP CONSTRAINT {}", name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_builder() {
        let mut builder = SqlQueryBuilder::new();
        let sql = builder.select()
            .columns(&["id", "name", "email"])
            .from("users")
            .where_clause("active = true")
            .order_by("name", OrderDirection::Asc)
            .limit(10)
            .build()
            .unwrap();

        assert!(sql.contains("SELECT id, name, email"));
        assert!(sql.contains("FROM users"));
        assert!(sql.contains("WHERE active = true"));
        assert!(sql.contains("ORDER BY name ASC"));
        assert!(sql.contains("LIMIT 10"));
    }

    #[test]
    fn test_insert_builder() {
        let mut builder = SqlQueryBuilder::new();
        let sql = builder.insert()
            .into("users")
            .columns(&["name", "email"])
            .values(vec![
                SqlValue::Text("Alice".to_string()),
                SqlValue::Text("alice@example.com".to_string())
            ])
            .build()
            .unwrap();

        assert!(sql.contains("INSERT INTO users"));
        assert!(sql.contains("(name, email)"));
        assert!(sql.contains("VALUES"));
    }

    #[test]
    fn test_create_table_builder() {
        let mut builder = SqlQueryBuilder::new();
        let sql = builder.create_table()
            .table("users")
            .if_not_exists()
            .column("id", SqlType::Integer).primary_key().auto_increment().finish()
            .column("name", SqlType::Text).not_null().finish()
            .column("email", SqlType::Text).unique().finish()
            .build()
            .unwrap();

        assert!(sql.contains("CREATE TABLE IF NOT EXISTS users"));
        assert!(sql.contains("id"));
        assert!(sql.contains("PRIMARY KEY"));
        assert!(sql.contains("AUTO_INCREMENT"));
    }

    #[test]
    fn test_join_types() {
        assert_eq!(JoinType::Inner.to_sql(), "INNER");
        assert_eq!(JoinType::Left.to_sql(), "LEFT");
        assert_eq!(JoinType::Right.to_sql(), "RIGHT");
        assert_eq!(JoinType::Full.to_sql(), "FULL");
    }

    #[test]
    fn test_order_direction() {
        assert_eq!(OrderDirection::Asc.to_sql(), "ASC");
        assert_eq!(OrderDirection::Desc.to_sql(), "DESC");
    }
}
