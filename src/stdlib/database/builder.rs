/// fr fr Query builder implementation for SQLSlay
/// 
/// This module provides fluent query building capabilities for common SQL operations.

use std::collections::HashMap;
use super::{DatabaseError, DatabaseErrorKind, SqlValue, DB, SlayRows, SlayResult};

/// fr fr Base query builder
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    /// fr fr Built query parts
    parts: Vec<String>,
    /// fr fr Query parameters
    params: Vec<SqlValue>,
}

impl QueryBuilder {
    /// slay Create a new query builder
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            params: Vec::new(),
        }
    }

    /// slay Add a raw SQL part
    pub fn add_part(&mut self, part: String) {
        self.parts.push(part);
    }

    /// slay Add a parameter
    pub fn add_param(&mut self, param: SqlValue) {
        self.params.push(param);
    }

    /// slay Build the final query
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        (self.parts.join(" "), self.params.clone())
    }
}

/// fr fr SELECT query builder
#[derive(Debug, Clone)]
pub struct SelectBuilder {
    /// fr fr Table name
    table: String,
    /// fr fr Selected columns
    columns: Vec<String>,
    /// fr fr WHERE conditions
    where_conditions: Vec<String>,
    /// fr fr WHERE parameters
    where_params: Vec<SqlValue>,
    /// fr fr JOIN clauses
    joins: Vec<String>,
    /// fr fr ORDER BY clause
    order_by: Option<String>,
    /// fr fr GROUP BY clause
    group_by: Option<String>,
    /// fr fr HAVING clause
    having: Option<String>,
    /// fr fr HAVING parameters
    having_params: Vec<SqlValue>,
    /// fr fr LIMIT clause
    limit: Option<i32>,
    /// fr fr OFFSET clause
    offset: Option<i32>,
}

impl SelectBuilder {
    /// slay Create a new SELECT builder
    pub fn new(table: String) -> Self {
        Self {
            table,
            columns: vec!["*".to_string()],
            where_conditions: Vec::new(),
            where_params: Vec::new(),
            joins: Vec::new(),
            order_by: None,
            group_by: None,
            having: None,
            having_params: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    /// slay Set columns to select
    pub fn columns(mut self, cols: Vec<String>) -> Self {
        self.columns = cols;
        self
    }

    /// slay Set table to select from
    pub fn from(mut self, table: String) -> Self {
        self.table = table;
        self
    }

    /// slay Add WHERE condition
    pub fn r#where(mut self, condition: String, args: Vec<SqlValue>) -> Self {
        self.where_conditions.push(condition);
        self.where_params.extend(args);
        self
    }

    /// slay Add ORDER BY clause
    pub fn order_by(mut self, order_by: String) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// slay Add GROUP BY clause
    pub fn group_by(mut self, group_by: String) -> Self {
        self.group_by = Some(group_by);
        self
    }

    /// slay Add HAVING clause
    pub fn having(mut self, having: String, args: Vec<SqlValue>) -> Self {
        self.having = Some(having);
        self.having_params.extend(args);
        self
    }

    /// slay Add LIMIT clause
    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// slay Add OFFSET clause
    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// slay Add JOIN clause
    pub fn join(mut self, join_type: String, table: String, condition: String) -> Self {
        self.joins.push(format!("{} JOIN {} ON {}", join_type, table, condition));
        self
    }

    /// slay Build the SELECT query
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        let mut query = format!("SELECT {} FROM {}", self.columns.join(", "), self.table);
        let mut params = Vec::new();

        // Add JOINs
        for join in &self.joins {
            query.push_str(&format!(" {}", join));
        }

        // Add WHERE
        if !self.where_conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
            params.extend(self.where_params.clone());
        }

        // Add GROUP BY
        if let Some(ref group_by) = self.group_by {
            query.push_str(&format!(" GROUP BY {}", group_by));
        }

        // Add HAVING
        if let Some(ref having) = self.having {
            query.push_str(&format!(" HAVING {}", having));
            params.extend(self.having_params.clone());
        }

        // Add ORDER BY
        if let Some(ref order_by) = self.order_by {
            query.push_str(&format!(" ORDER BY {}", order_by));
        }

        // Add LIMIT
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        // Add OFFSET
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        (query, params)
    }

    /// slay Execute query and return SlayRows
    pub fn exec(&self, db: &DB) -> Result<SlayRows, DatabaseError> {
        let (query, params) = self.build();
        db.slay_query(query, params)
    }

    /// slay Execute query and return first row as map
    pub fn one(&self, db: &DB) -> Result<HashMap<String, SqlValue>, DatabaseError> {
        let mut result = self.exec(db)?;
        result.first()
    }

    /// slay Execute query and return all rows as maps
    pub fn all(&self, db: &DB) -> Result<Vec<HashMap<String, SqlValue>>, DatabaseError> {
        let mut result = self.exec(db)?;
        result.all()
    }

    /// slay Execute query and return count
    pub fn count(&self, db: &DB) -> Result<i64, DatabaseError> {
        let mut builder = self.clone();
        builder.columns = vec!["COUNT(*)".to_string()];
        let (query, params) = builder.build();
        
        let row = db.query_row(query, params);
        if let Some(data) = row.data {
            if let Some(SqlValue::Integer(count)) = data.get(0) {
                Ok(*count)
            } else {
                Err(DatabaseError::query_error("Invalid count result"))
            }
        } else {
            Err(DatabaseError::query_error("No count result"))
        }
    }
}

/// fr fr INSERT query builder
#[derive(Debug, Clone)]
pub struct InsertBuilder {
    /// fr fr Table name
    table: String,
    /// fr fr Column names
    columns: Vec<String>,
    /// fr fr Values to insert
    values: Vec<Vec<SqlValue>>,
}

impl InsertBuilder {
    /// slay Create a new INSERT builder
    pub fn new(table: String) -> Self {
        Self {
            table,
            columns: Vec::new(),
            values: Vec::new(),
        }
    }

    /// slay Set columns
    pub fn columns(mut self, cols: Vec<String>) -> Self {
        self.columns = cols;
        self
    }

    /// slay Add values for a single row
    pub fn values(mut self, values: Vec<SqlValue>) -> Self {
        self.values.push(values);
        self
    }

    /// slay Add a record (map of column -> value)
    pub fn record(mut self, record: HashMap<String, SqlValue>) -> Self {
        if self.columns.is_empty() {
            self.columns = record.keys().cloned().collect();
        }
        
        let mut row_values = Vec::new();
        for column in &self.columns {
            row_values.push(record.get(column).cloned().unwrap_or(SqlValue::Null));
        }
        self.values.push(row_values);
        self
    }

    /// slay Build the INSERT query
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        let placeholders: Vec<String> = (0..self.columns.len()).map(|_| "?".to_string()).collect();
        let values_clause = vec![format!("({})", placeholders.join(", ")); self.values.len()].join(", ");
        
        let query = format!(
            "INSERT INTO {} ({}) VALUES {}",
            self.table,
            self.columns.join(", "),
            values_clause
        );
        
        let mut params = Vec::new();
        for row in &self.values {
            params.extend(row.clone());
        }
        
        (query, params)
    }

    /// slay Execute the INSERT
    pub fn exec(&self, db: &DB) -> Result<SlayResult, DatabaseError> {
        let (query, params) = self.build();
        db.slay_exec(query, params)
    }

    /// slay Add multiple records for batch insert
    pub fn batch_insert(mut self, records: Vec<HashMap<String, SqlValue>>) -> Self {
        for record in records {
            self = self.record(record);
        }
        self
    }
}

/// fr fr UPDATE query builder
#[derive(Debug, Clone)]
pub struct UpdateBuilder {
    /// fr fr Table name
    table: String,
    /// fr fr SET clauses
    set_clauses: Vec<String>,
    /// fr fr SET parameters
    set_params: Vec<SqlValue>,
    /// fr fr WHERE conditions
    where_conditions: Vec<String>,
    /// fr fr WHERE parameters
    where_params: Vec<SqlValue>,
}

impl UpdateBuilder {
    /// slay Create a new UPDATE builder
    pub fn new(table: String) -> Self {
        Self {
            table,
            set_clauses: Vec::new(),
            set_params: Vec::new(),
            where_conditions: Vec::new(),
            where_params: Vec::new(),
        }
    }

    /// slay Add a SET clause
    pub fn set(mut self, column: String, value: SqlValue) -> Self {
        self.set_clauses.push(format!("{} = ?", column));
        self.set_params.push(value);
        self
    }

    /// slay Set multiple columns from a map
    pub fn set_map(mut self, data: HashMap<String, SqlValue>) -> Self {
        for (column, value) in data {
            self = self.set(column, value);
        }
        self
    }

    /// slay Add WHERE condition
    pub fn r#where(mut self, condition: String, args: Vec<SqlValue>) -> Self {
        self.where_conditions.push(condition);
        self.where_params.extend(args);
        self
    }

    /// slay Build the UPDATE query
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        let mut query = format!("UPDATE {} SET {}", self.table, self.set_clauses.join(", "));
        let mut params = self.set_params.clone();
        
        if !self.where_conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
            params.extend(self.where_params.clone());
        }
        
        (query, params)
    }

    /// slay Execute the UPDATE
    pub fn exec(&self, db: &DB) -> Result<SlayResult, DatabaseError> {
        let (query, params) = self.build();
        db.slay_exec(query, params)
    }
}

/// fr fr DELETE query builder
#[derive(Debug, Clone)]
pub struct DeleteBuilder {
    /// fr fr Table name
    table: String,
    /// fr fr WHERE conditions
    where_conditions: Vec<String>,
    /// fr fr WHERE parameters
    where_params: Vec<SqlValue>,
}

impl DeleteBuilder {
    /// slay Create a new DELETE builder
    pub fn new(table: String) -> Self {
        Self {
            table,
            where_conditions: Vec::new(),
            where_params: Vec::new(),
        }
    }

    /// slay Add WHERE condition
    pub fn r#where(mut self, condition: String, args: Vec<SqlValue>) -> Self {
        self.where_conditions.push(condition);
        self.where_params.extend(args);
        self
    }

    /// slay Build the DELETE query
    pub fn build(&self) -> (String, Vec<SqlValue>) {
        let mut query = format!("DELETE FROM {}", self.table);
        let params = self.where_params.clone();
        
        if !self.where_conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", self.where_conditions.join(" AND ")));
        }
        
        (query, params)
    }

    /// slay Execute the DELETE
    pub fn exec(&self, db: &DB) -> Result<SlayResult, DatabaseError> {
        let (query, params) = self.build();
        db.slay_exec(query, params)
    }
}

/// slay Create a new SELECT builder
pub fn new_select_builder(table: String) -> SelectBuilder {
    SelectBuilder::new(table)
}

/// slay Create a new INSERT builder
pub fn new_insert_builder(table: String) -> InsertBuilder {
    InsertBuilder::new(table)
}

/// slay Create a new UPDATE builder
pub fn new_update_builder(table: String) -> UpdateBuilder {
    UpdateBuilder::new(table)
}

/// slay Create a new DELETE builder
pub fn new_delete_builder(table: String) -> DeleteBuilder {
    DeleteBuilder::new(table)
}
