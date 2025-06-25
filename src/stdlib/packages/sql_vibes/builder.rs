/// fr fr SQL query builder with CURSED-style method chaining - build queries like a boss
// use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, Parameter};
use crate::error::CursedError;
use std::collections::{HashMap, BTreeMap};
use std::fmt;

/// fr fr Main query builder trait - all builders implement this periodt
pub trait QueryBuilder {
    /// sus Build the final SQL query string
    fn build(&self) -> SqlResult<String>;
    
    /// facts Get the parameters for this query
    fn parameters(&self) -> Vec<Parameter>;
    
    /// lowkey Get the parameter count
    fn parameter_count(&self) -> usize {
        self.parameters().len()
    /// highkey Validate the query structure
    fn validate(&self) -> SqlResult<()>;
    
    /// periodt Clone this builder
    fn clone_builder(&self) -> Box<dyn QueryBuilder>;
/// fr fr SELECT query builder - for when you need to yolo some data
#[derive(Debug, Clone)]
pub struct SelectBuilder {
    /// Columns to select
    
    /// Table name
    
    /// JOIN clauses
    
    /// WHERE conditions
    
    /// GROUP BY columns
    
    /// HAVING conditions
    
    /// ORDER BY clauses
    
    /// LIMIT count
    
    /// OFFSET count
    
    /// Parameters for the query
    
    /// Parameter counter for generating unique parameter names
impl SelectBuilder {
    /// sus Create new SELECT builder with columns
    pub fn new(columns: &[&str]) -> Self {
        Self {
        }
    }
    
    /// facts Specify the table to select from
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    /// lowkey Add a JOIN clause
    pub fn join(mut self, join_type: JoinType, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
        });
        self
    /// highkey Add INNER JOIN - most common type periodt
    pub fn inner_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Inner, table, condition)
    /// periodt Add LEFT JOIN - for when you need all the left side bestie
    pub fn left_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Left, table, condition)
    /// bestie Add RIGHT JOIN - less common but still valid
    pub fn right_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Right, table, condition)
    /// flex Add WHERE condition with parameter
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
        });
        self
    /// yolo Add WHERE condition with custom expression
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
        });
        self
    /// slay Add OR WHERE condition
    pub fn or_where(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
        });
        self
    /// nocap Add WHERE IN condition
    pub fn where_in(mut self, column: &str, values: &[SqlValue]) -> Self {
        let param_names: Vec<String> = (0..values.len())
            .map(|_| self.next_param_name())
            .collect();
        
        for (i, value) in values.iter().enumerate() {
            self.parameters.push(Parameter::new(param_names[i].clone(), value.clone()));
        let params_str = param_names.iter()
            .map(|name| format!("${}", name))
            .collect::<Vec<_>>()
            .join(", ");
        
        self.where_conditions.push(WhereCondition {
        });
        self
    /// oop Add WHERE LIKE condition for pattern matching
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), SqlValue::String(pattern.to_string())));
        self.where_conditions.push(WhereCondition {
        });
        self
    /// vibes Add GROUP BY clause
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    /// mood Add HAVING condition (for GROUP BY results)
    pub fn having(mut self, condition: &str) -> Self {
        self.having_conditions.push(WhereCondition {
        });
        self
    /// energy Add ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by.push(OrderByClause {
        });
        self
    /// basic Order by ascending (default vibe)
    pub fn order_asc(self, column: &str) -> Self {
        self.order_by(column, OrderDirection::Ascending)
    /// iconic Order by descending (reverse vibe)
    pub fn order_desc(self, column: &str) -> Self {
        self.order_by(column, OrderDirection::Descending)
    /// queen Add LIMIT clause
    pub fn limit(mut self, count: u64) -> Self {
        self.limit = Some(count);
        self
    /// king Add OFFSET clause
    pub fn offset(mut self, count: u64) -> Self {
        self.offset = Some(count);
        self
    /// bussin Get the columns being selected
    pub fn get_columns(&self) -> &[String] {
        &self.columns
    /// chef Get the table name
    pub fn get_table(&self) -> Option<&str> {
        self.table.as_deref()
    /// Generate next parameter name
    fn next_param_name(&mut self) -> String {
        self.param_counter += 1;
        format!("param_{}", self.param_counter)
    }
}

impl QueryBuilder for SelectBuilder {
    fn build(&self) -> SqlResult<String> {
        self.validate()?;
        
        let mut query = String::new();
        
        // SELECT clause
        query.push_str("SELECT ");
        query.push_str(&self.columns.join(", "));
        
        // FROM clause
        if let Some(table) = &self.table {
            query.push_str(&format!(" FROM {}", table));
        // JOIN clauses
        for join in &self.joins {
                join.join_type, join.table, join.condition));
        // WHERE clause
        if !self.where_conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.where_conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.operator));
                }
                query.push_str(&condition.expression);
            }
        }
        
        // GROUP BY clause
        if !self.group_by.is_empty() {
            query.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        // HAVING clause
        if !self.having_conditions.is_empty() {
            query.push_str(" HAVING ");
            for (i, condition) in self.having_conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.operator));
                }
                query.push_str(&condition.expression);
            }
        }
        
        // ORDER BY clause
        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            let order_strs: Vec<String> = self.order_by.iter()
                .map(|order| format!("{} {}", order.column, order.direction))
                .collect();
            query.push_str(&order_strs.join(", "));
        // LIMIT clause
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        // OFFSET clause
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        Ok(query)
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    fn validate(&self) -> SqlResult<()> {
        if self.columns.is_empty() {
            return Err(SqlError::query("SELECT must have at least one column - that's basic SQL bestie".to_string()));
        Ok(())
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr INSERT query builder - for adding new data periodt
#[derive(Debug, Clone)]
pub struct InsertBuilder {
    /// Table to insert into
    
    /// Columns to insert
    
    /// Values to insert (multiple rows supported)
    
    /// ON CONFLICT/DUPLICATE KEY handling
    
    /// Parameters for the query
    
    /// Parameter counter
impl InsertBuilder {
    /// sus Create new INSERT builder for table
    pub fn new(table: &str) -> Self {
        Self {
        }
    }
    
    /// facts Set the columns to insert
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    /// lowkey Add values for one row
    pub fn values(mut self, values: &[SqlValue]) -> Self {
        self.values.push(values.to_vec());
        self
    /// highkey Add multiple rows of values
    pub fn values_batch(mut self, rows: &[&[SqlValue]]) -> Self {
        for row in rows {
            self.values.push(row.to_vec());
        }
        self
    /// periodt Handle conflicts with IGNORE strategy
    pub fn on_conflict_ignore(mut self) -> Self {
        self.conflict_resolution = Some(ConflictResolution::Ignore);
        self
    /// bestie Handle conflicts with UPDATE strategy
    pub fn on_conflict_update(mut self, updates: HashMap<String, SqlValue>) -> Self {
        self.conflict_resolution = Some(ConflictResolution::Update(updates));
        self
    /// flex Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    /// yolo Get the columns
    pub fn get_columns(&self) -> &[String] {
        &self.columns
    /// Generate next parameter name
    fn next_param_name(&mut self) -> String {
        self.param_counter += 1;
        format!("param_{}", self.param_counter)
    }
}

impl QueryBuilder for InsertBuilder {
    fn build(&self) -> SqlResult<String> {
        self.validate()?;
        
        let mut query = String::new();
        
        // INSERT clause
        query.push_str(&format!("INSERT INTO {}", self.table));
        
        // Columns
        if !self.columns.is_empty() {
            query.push_str(&format!(" ({})", self.columns.join(", ")));
        // VALUES
        query.push_str(" VALUES ");
        let value_strings: Vec<String> = self.values.iter()
            .map(|row| {
                let placeholders: Vec<String> = row.iter()
                    .enumerate()
                    .map(|(i, _)| format!("${}", i + 1))
                    .collect();
                format!("({})", placeholders.join(", "))
            })
            .collect();
        query.push_str(&value_strings.join(", "));
        
        // Conflict resolution
        if let Some(resolution) = &self.conflict_resolution {
            match resolution {
                ConflictResolution::Ignore => {
                    query.push_str(" ON CONFLICT DO NOTHING");
                }
                ConflictResolution::Update(updates) => {
                    query.push_str(" ON CONFLICT DO UPDATE SET ");
                    let update_strs: Vec<String> = updates.iter()
                        .map(|(col, _)| format!("{} = EXCLUDED.{}", col, col))
                        .collect();
                    query.push_str(&update_strs.join(", "));
                }
            }
        Ok(query)
    fn parameters(&self) -> Vec<Parameter> {
        let mut params = Vec::new();
        let mut counter = 1;
        
        for row in &self.values {
            for value in row {
                params.push(Parameter::new(format!("param_{}", counter), value.clone()));
                counter += 1;
            }
        }
        
        params
    fn validate(&self) -> SqlResult<()> {
        if self.values.is_empty() {
            return Err(SqlError::query("INSERT must have at least one row of values - can't insert nothing bestie".to_string()));
        if !self.columns.is_empty() {
            for row in &self.values {
                if row.len() != self.columns.len() {
                    return Err(SqlError::query("Number of values must match number of columns - that's sus af".to_string()));
                }
            }
        Ok(())
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr UPDATE query builder - for changing existing data
#[derive(Debug, Clone)]
pub struct UpdateBuilder {
    /// Table to update
    
    /// SET clauses (column = value)
    
    /// WHERE conditions
    
    /// Parameters for the query
    
    /// Parameter counter
impl UpdateBuilder {
    /// sus Create new UPDATE builder for table
    pub fn new(table: &str) -> Self {
        Self {
        }
    }
    
    /// facts Set a column to a value
    pub fn set(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.set_clauses.push(SetClause {
        });
        self
    /// lowkey Set a column with a custom expression
    pub fn set_expr(mut self, column: &str, expression: &str) -> Self {
        self.set_clauses.push(SetClause {
        });
        self
    /// highkey Add WHERE condition
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
        });
        self
    /// periodt Add custom WHERE condition
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
        });
        self
    /// bestie Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    /// Generate next parameter name
    fn next_param_name(&mut self) -> String {
        self.param_counter += 1;
        format!("param_{}", self.param_counter)
    }
}

impl QueryBuilder for UpdateBuilder {
    fn build(&self) -> SqlResult<String> {
        self.validate()?;
        
        let mut query = String::new();
        
        // UPDATE clause
        query.push_str(&format!("UPDATE {}", self.table));
        
        // SET clause
        query.push_str(" SET ");
        let set_strs: Vec<String> = self.set_clauses.iter()
            .map(|clause| format!("{} = {}", clause.column, clause.expression))
            .collect();
        query.push_str(&set_strs.join(", "));
        
        // WHERE clause
        if !self.where_conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.where_conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.operator));
                }
                query.push_str(&condition.expression);
            }
        }
        
        Ok(query)
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    fn validate(&self) -> SqlResult<()> {
        if self.set_clauses.is_empty() {
            return Err(SqlError::query("UPDATE must have at least one SET clause - can't update nothing bestie".to_string()));
        Ok(())
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr DELETE query builder - for removing data periodt
#[derive(Debug, Clone)]
pub struct DeleteBuilder {
    /// Table to delete from
    
    /// WHERE conditions
    
    /// Parameters for the query
    
    /// Parameter counter
impl DeleteBuilder {
    /// sus Create new DELETE builder for table
    pub fn new(table: &str) -> Self {
        Self {
        }
    }
    
    /// facts Add WHERE condition
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
        });
        self
    /// lowkey Add custom WHERE condition
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
        });
        self
    /// highkey Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    /// Generate next parameter name
    fn next_param_name(&mut self) -> String {
        self.param_counter += 1;
        format!("param_{}", self.param_counter)
    }
}

impl QueryBuilder for DeleteBuilder {
    fn build(&self) -> SqlResult<String> {
        self.validate()?;
        
        let mut query = String::new();
        
        // DELETE clause
        query.push_str(&format!("DELETE FROM {}", self.table));
        
        // WHERE clause
        if !self.where_conditions.is_empty() {
            query.push_str(" WHERE ");
            for (i, condition) in self.where_conditions.iter().enumerate() {
                if i > 0 {
                    query.push_str(&format!(" {} ", condition.operator));
                }
                query.push_str(&condition.expression);
            }
        }
        
        Ok(query)
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    fn validate(&self) -> SqlResult<()> {
        // DELETE without WHERE is dangerous but valid
        // Could add a safety check here if desired
        Ok(())
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr JOIN types supported by query builder
#[derive(Debug, Clone)]
pub enum JoinType {
impl fmt::Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr WHERE condition operator
#[derive(Debug, Clone)]
enum WhereOperator {
impl fmt::Display for WhereOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr ORDER BY direction
#[derive(Debug, Clone)]
pub enum OrderDirection {
impl fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// fr fr Conflict resolution strategy for INSERT
#[derive(Debug, Clone)]
enum ConflictResolution {
/// fr fr JOIN clause structure
#[derive(Debug, Clone)]
struct JoinClause {
/// fr fr WHERE condition structure
#[derive(Debug, Clone)]
struct WhereCondition {
/// fr fr SET clause for UPDATE statements
#[derive(Debug, Clone)]
struct SetClause {
/// fr fr ORDER BY clause structure
#[derive(Debug, Clone)]
struct OrderByClause {
