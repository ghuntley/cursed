//! Database query builder implementation

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::IOError;

/// Result type for builder operations
pub type BuilderResult<T> = Result<T, CursedError>;

/// Generic query builder
pub struct QueryBuilder {
    query_type: QueryType,
    table: Option<String>,
    columns: Vec<String>,
    values: Vec<String>,
    conditions: Vec<String>,
    joins: Vec<String>,
    order_by: Vec<String>,
    group_by: Vec<String>,
    having: Vec<String>,
    limit: Option<u64>,
    offset: Option<u64>,
}

/// SELECT query builder
pub struct SelectBuilder {
    builder: QueryBuilder,
}

/// INSERT query builder
pub struct InsertBuilder {
    builder: QueryBuilder,
}

/// UPDATE query builder
pub struct UpdateBuilder {
    builder: QueryBuilder,
}

/// DELETE query builder
pub struct DeleteBuilder {
    builder: QueryBuilder,
}

/// Query types
#[derive(Debug, Clone)]
enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new(query_type: QueryType) -> Self {
        Self {
            query_type,
            table: None,
            columns: Vec::new(),
            values: Vec::new(),
            conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Set the table name
    pub fn table(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    
    /// Add columns
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns.extend(columns.iter().map(|s| s.to_string()));
        self
    }
    
    /// Add a WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }
    
    /// Build the query
    pub fn build(&self) -> BuilderResult<String> {
        match self.query_type {
            QueryType::Select => self.build_select(),
            QueryType::Insert => self.build_insert(),
            QueryType::Update => self.build_update(),
            QueryType::Delete => self.build_delete(),
        }
    }
    
    fn build_select(&self) -> BuilderResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        let columns = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };
        
        let mut query = format!("SELECT {} FROM {}", columns, table);
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            query.push_str(&self.order_by.join(", "));
        }
        
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        Ok(query)
    }
    
    fn build_insert(&self) -> BuilderResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(CursedError::runtime_error(&"Columns required for INSERT".to_string()));
        }
        
        let columns = self.columns.join(", ");
        let placeholders = (0..self.columns.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
        
        Ok(format!("INSERT INTO {} ({}) VALUES ({})", table, columns, placeholders))
    }
    
    fn build_update(&self) -> BuilderResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        if self.columns.is_empty() {
            return Err(CursedError::runtime_error(&"Columns required for UPDATE".to_string()));
        }
        
        let set_clauses = self.columns.iter()
            .map(|col| format!("{} = ?", col))
            .collect::<Vec<_>>()
            .join(", ");
        
        let mut query = format!("UPDATE {} SET {}", table, set_clauses);
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        Ok(query)
    }
    
    fn build_delete(&self) -> BuilderResult<String> {
        let table = self.table.as_ref()
            .ok_or_else(|| IOError::Other("Table name required".to_string()))?;
        
        let mut query = format!("DELETE FROM {}", table);
        
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        
        Ok(query)
    }
}

impl SelectBuilder {
    /// Create a new SELECT builder
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new(QueryType::Select),
        }
    }
    
    /// Set the table to select from
    pub fn from(mut self, table: &str) -> Self {
        self.builder = self.builder.table(table);
        self
    }
    
    /// Set columns to select
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.builder = self.builder.columns(columns);
        self
    }
    
    /// Add a WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.builder = self.builder.where_clause(condition);
        self
    }
    
    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str) -> Self {
        self.builder.order_by.push(column.to_string());
        self
    }
    
    /// Set LIMIT
    pub fn limit(mut self, limit: u64) -> Self {
        self.builder.limit = Some(limit);
        self
    }
    
    /// Build the SELECT query
    pub fn build(self) -> BuilderResult<String> {
        self.builder.build()
    }
}

impl InsertBuilder {
    /// Create a new INSERT builder
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new(QueryType::Insert),
        }
    }
    
    /// Set the table to insert into
    pub fn into(mut self, table: &str) -> Self {
        self.builder = self.builder.table(table);
        self
    }
    
    /// Set columns to insert
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.builder = self.builder.columns(columns);
        self
    }
    
    /// Build the INSERT query
    pub fn build(self) -> BuilderResult<String> {
        self.builder.build()
    }
}

impl UpdateBuilder {
    /// Create a new UPDATE builder
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new(QueryType::Update),
        }
    }
    
    /// Set the table to update
    pub fn table(mut self, table: &str) -> Self {
        self.builder = self.builder.table(table);
        self
    }
    
    /// Set columns to update
    pub fn set(mut self, columns: &[&str]) -> Self {
        self.builder = self.builder.columns(columns);
        self
    }
    
    /// Add a WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.builder = self.builder.where_clause(condition);
        self
    }
    
    /// Build the UPDATE query
    pub fn build(self) -> BuilderResult<String> {
        self.builder.build()
    }
}

impl DeleteBuilder {
    /// Create a new DELETE builder
    pub fn new() -> Self {
        Self {
            builder: QueryBuilder::new(QueryType::Delete),
        }
    }
    
    /// Set the table to delete from
    pub fn from(mut self, table: &str) -> Self {
        self.builder = self.builder.table(table);
        self
    }
    
    /// Add a WHERE condition
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.builder = self.builder.where_clause(condition);
        self
    }
    
    /// Build the DELETE query
    pub fn build(self) -> BuilderResult<String> {
        self.builder.build()
    }
}

impl Default for SelectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InsertBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for UpdateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DeleteBuilder {
    fn default() -> Self {
        Self::new()
    }
}
