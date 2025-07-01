//! Fluent query builder for CURSED ORM

use std::collections::HashMap;
use std::sync::Arc;
use super::super::{SqlValue, DatabaseError, DatabaseConnection};
use super::Entity;

/// Fluent query builder with Gen Z vibes
pub struct FluentQueryBuilder<T: Entity> {
    connection: Arc<dyn DatabaseConnection>,
    table_name: String,
    where_clauses: Vec<String>,
    where_params: Vec<SqlValue>,
    order_by: Vec<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> FluentQueryBuilder<T> {
    pub fn new(connection: Arc<dyn DatabaseConnection>) -> Self {
        Self {
            connection,
            table_name: T::table_name().to_string(),
            where_clauses: Vec::new(),
            where_params: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Add a WHERE clause with Gen Z naming
    pub fn where_clause(mut self, clause: &str, params: Vec<SqlValue>) -> Self {
        self.where_clauses.push(clause.to_string());
        self.where_params.extend(params);
        self
    }
    
    /// Add ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by.push(format!("{} {}", column, direction));
        self
    }
    
    /// Set LIMIT
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set OFFSET
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// Execute query and return entities
    pub async fn execute(&self) -> Result<Vec<T>, DatabaseError> {
        let mut sql = format!("SELECT * FROM {}", self.table_name);
        
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }
        
        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(&self.order_by.join(", "));
        }
        
        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        let result = self.connection.query(sql, self.where_params.clone())
            .map_err(|e| DatabaseError::query(&format!("Query execution failed: {}", e)))?;
        
        let mut entities = Vec::new();
        for row in result.rows() {
            let row_map = row.to_hashmap();
            let entity = T::from_row(&row_map)?;
            entities.push(entity);
        }
        
        Ok(entities)
    }
    
    /// Count the vibes (Gen Z for counting records)
    pub async fn count_the_vibes(&self) -> Result<i64, DatabaseError> {
        let mut sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
        
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.where_clauses.join(" AND "));
        }
        
        let result = self.connection.query(sql, self.where_params.clone())
            .map_err(|e| DatabaseError::query(&format!("Count query failed: {}", e)))?;
        
        if let Some(row) = result.rows().first() {
            let row_map = row.to_hashmap();
            if let Some(SqlValue::Integer(count)) = row_map.get("count") {
                Ok(*count)
            } else {
                Err(DatabaseError::query("Invalid count result"))
            }
        } else {
            Ok(0)
        }
    }
}
