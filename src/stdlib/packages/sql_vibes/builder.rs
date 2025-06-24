/// fr fr SQL query builder with CURSED-style method chaining - build queries like a boss
use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, Parameter};
use crate::error::Error;
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
    }
    
    /// highkey Validate the query structure
    fn validate(&self) -> SqlResult<()>;
    
    /// periodt Clone this builder
    fn clone_builder(&self) -> Box<dyn QueryBuilder>;
}

/// fr fr SELECT query builder - for when you need to yolo some data
#[derive(Debug, Clone)]
pub struct SelectBuilder {
    /// Columns to select
    columns: Vec<String>,
    
    /// Table name
    table: Option<String>,
    
    /// JOIN clauses
    joins: Vec<JoinClause>,
    
    /// WHERE conditions
    where_conditions: Vec<WhereCondition>,
    
    /// GROUP BY columns
    group_by: Vec<String>,
    
    /// HAVING conditions
    having_conditions: Vec<WhereCondition>,
    
    /// ORDER BY clauses
    order_by: Vec<OrderByClause>,
    
    /// LIMIT count
    limit: Option<u64>,
    
    /// OFFSET count
    offset: Option<u64>,
    
    /// Parameters for the query
    parameters: Vec<Parameter>,
    
    /// Parameter counter for generating unique parameter names
    param_counter: usize,
}

impl SelectBuilder {
    /// sus Create new SELECT builder with columns
    pub fn new(columns: &[&str]) -> Self {
        Self {
            columns: columns.iter().map(|s| s.to_string()).collect(),
            table: None,
            joins: Vec::new(),
            where_conditions: Vec::new(),
            group_by: Vec::new(),
            having_conditions: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            parameters: Vec::new(),
            param_counter: 0,
        }
    }
    
    /// facts Specify the table to select from
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    
    /// lowkey Add a JOIN clause
    pub fn join(mut self, join_type: JoinType, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }
    
    /// highkey Add INNER JOIN - most common type periodt
    pub fn inner_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Inner, table, condition)
    }
    
    /// periodt Add LEFT JOIN - for when you need all the left side bestie
    pub fn left_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Left, table, condition)
    }
    
    /// bestie Add RIGHT JOIN - less common but still valid
    pub fn right_join(self, table: &str, condition: &str) -> Self {
        self.join(JoinType::Right, table, condition)
    }
    
    /// flex Add WHERE condition with parameter
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
            expression: format!("{} = ${}", column, param_name),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// yolo Add WHERE condition with custom expression
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            expression: expression.to_string(),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// slay Add OR WHERE condition
    pub fn or_where(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            expression: expression.to_string(),
            operator: WhereOperator::Or,
        });
        self
    }
    
    /// nocap Add WHERE IN condition
    pub fn where_in(mut self, column: &str, values: &[SqlValue]) -> Self {
        let param_names: Vec<String> = (0..values.len())
            .map(|_| self.next_param_name())
            .collect();
        
        for (i, value) in values.iter().enumerate() {
            self.parameters.push(Parameter::new(param_names[i].clone(), value.clone()));
        }
        
        let params_str = param_names.iter()
            .map(|name| format!("${}", name))
            .collect::<Vec<_>>()
            .join(", ");
        
        self.where_conditions.push(WhereCondition {
            expression: format!("{} IN ({})", column, params_str),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// oop Add WHERE LIKE condition for pattern matching
    pub fn where_like(mut self, column: &str, pattern: &str) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), SqlValue::String(pattern.to_string())));
        self.where_conditions.push(WhereCondition {
            expression: format!("{} LIKE ${}", column, param_name),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// vibes Add GROUP BY clause
    pub fn group_by(mut self, column: &str) -> Self {
        self.group_by.push(column.to_string());
        self
    }
    
    /// mood Add HAVING condition (for GROUP BY results)
    pub fn having(mut self, condition: &str) -> Self {
        self.having_conditions.push(WhereCondition {
            expression: condition.to_string(),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// energy Add ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by.push(OrderByClause {
            column: column.to_string(),
            direction,
        });
        self
    }
    
    /// basic Order by ascending (default vibe)
    pub fn order_asc(self, column: &str) -> Self {
        self.order_by(column, OrderDirection::Ascending)
    }
    
    /// iconic Order by descending (reverse vibe)
    pub fn order_desc(self, column: &str) -> Self {
        self.order_by(column, OrderDirection::Descending)
    }
    
    /// queen Add LIMIT clause
    pub fn limit(mut self, count: u64) -> Self {
        self.limit = Some(count);
        self
    }
    
    /// king Add OFFSET clause
    pub fn offset(mut self, count: u64) -> Self {
        self.offset = Some(count);
        self
    }
    
    /// bussin Get the columns being selected
    pub fn get_columns(&self) -> &[String] {
        &self.columns
    }
    
    /// chef Get the table name
    pub fn get_table(&self) -> Option<&str> {
        self.table.as_deref()
    }
    
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
        }
        
        // JOIN clauses
        for join in &self.joins {
            query.push_str(&format!(" {} JOIN {} ON {}", 
                join.join_type, join.table, join.condition));
        }
        
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
        }
        
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
        }
        
        // LIMIT clause
        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        // OFFSET clause
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        Ok(query)
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }
    
    fn validate(&self) -> SqlResult<()> {
        if self.columns.is_empty() {
            return Err(SqlError::query("SELECT must have at least one column - that's basic SQL bestie".to_string()));
        }
        
        Ok(())
    }
    
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr INSERT query builder - for adding new data periodt
#[derive(Debug, Clone)]
pub struct InsertBuilder {
    /// Table to insert into
    table: String,
    
    /// Columns to insert
    columns: Vec<String>,
    
    /// Values to insert (multiple rows supported)
    values: Vec<Vec<SqlValue>>,
    
    /// ON CONFLICT/DUPLICATE KEY handling
    conflict_resolution: Option<ConflictResolution>,
    
    /// Parameters for the query
    parameters: Vec<Parameter>,
    
    /// Parameter counter
    param_counter: usize,
}

impl InsertBuilder {
    /// sus Create new INSERT builder for table
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: Vec::new(),
            values: Vec::new(),
            conflict_resolution: None,
            parameters: Vec::new(),
            param_counter: 0,
        }
    }
    
    /// facts Set the columns to insert
    pub fn columns(mut self, columns: &[&str]) -> Self {
        self.columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// lowkey Add values for one row
    pub fn values(mut self, values: &[SqlValue]) -> Self {
        self.values.push(values.to_vec());
        self
    }
    
    /// highkey Add multiple rows of values
    pub fn values_batch(mut self, rows: &[&[SqlValue]]) -> Self {
        for row in rows {
            self.values.push(row.to_vec());
        }
        self
    }
    
    /// periodt Handle conflicts with IGNORE strategy
    pub fn on_conflict_ignore(mut self) -> Self {
        self.conflict_resolution = Some(ConflictResolution::Ignore);
        self
    }
    
    /// bestie Handle conflicts with UPDATE strategy
    pub fn on_conflict_update(mut self, updates: HashMap<String, SqlValue>) -> Self {
        self.conflict_resolution = Some(ConflictResolution::Update(updates));
        self
    }
    
    /// flex Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    }
    
    /// yolo Get the columns
    pub fn get_columns(&self) -> &[String] {
        &self.columns
    }
    
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
        }
        
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
        }
        
        Ok(query)
    }
    
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
    }
    
    fn validate(&self) -> SqlResult<()> {
        if self.values.is_empty() {
            return Err(SqlError::query("INSERT must have at least one row of values - can't insert nothing bestie".to_string()));
        }
        
        if !self.columns.is_empty() {
            for row in &self.values {
                if row.len() != self.columns.len() {
                    return Err(SqlError::query("Number of values must match number of columns - that's sus af".to_string()));
                }
            }
        }
        
        Ok(())
    }
    
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr UPDATE query builder - for changing existing data
#[derive(Debug, Clone)]
pub struct UpdateBuilder {
    /// Table to update
    table: String,
    
    /// SET clauses (column = value)
    set_clauses: Vec<SetClause>,
    
    /// WHERE conditions
    where_conditions: Vec<WhereCondition>,
    
    /// Parameters for the query
    parameters: Vec<Parameter>,
    
    /// Parameter counter
    param_counter: usize,
}

impl UpdateBuilder {
    /// sus Create new UPDATE builder for table
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            set_clauses: Vec::new(),
            where_conditions: Vec::new(),
            parameters: Vec::new(),
            param_counter: 0,
        }
    }
    
    /// facts Set a column to a value
    pub fn set(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.set_clauses.push(SetClause {
            column: column.to_string(),
            expression: format!("${}", param_name),
        });
        self
    }
    
    /// lowkey Set a column with a custom expression
    pub fn set_expr(mut self, column: &str, expression: &str) -> Self {
        self.set_clauses.push(SetClause {
            column: column.to_string(),
            expression: expression.to_string(),
        });
        self
    }
    
    /// highkey Add WHERE condition
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
            expression: format!("{} = ${}", column, param_name),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// periodt Add custom WHERE condition
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            expression: expression.to_string(),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// bestie Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    }
    
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
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }
    
    fn validate(&self) -> SqlResult<()> {
        if self.set_clauses.is_empty() {
            return Err(SqlError::query("UPDATE must have at least one SET clause - can't update nothing bestie".to_string()));
        }
        
        Ok(())
    }
    
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr DELETE query builder - for removing data periodt
#[derive(Debug, Clone)]
pub struct DeleteBuilder {
    /// Table to delete from
    table: String,
    
    /// WHERE conditions
    where_conditions: Vec<WhereCondition>,
    
    /// Parameters for the query
    parameters: Vec<Parameter>,
    
    /// Parameter counter
    param_counter: usize,
}

impl DeleteBuilder {
    /// sus Create new DELETE builder for table
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            where_conditions: Vec::new(),
            parameters: Vec::new(),
            param_counter: 0,
        }
    }
    
    /// facts Add WHERE condition
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        let param_name = self.next_param_name();
        self.parameters.push(Parameter::new(param_name.clone(), value));
        self.where_conditions.push(WhereCondition {
            expression: format!("{} = ${}", column, param_name),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// lowkey Add custom WHERE condition
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            expression: expression.to_string(),
            operator: WhereOperator::And,
        });
        self
    }
    
    /// highkey Get the table name
    pub fn get_table(&self) -> &str {
        &self.table
    }
    
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
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }
    
    fn validate(&self) -> SqlResult<()> {
        // DELETE without WHERE is dangerous but valid
        // Could add a safety check here if desired
        Ok(())
    }
    
    fn clone_builder(&self) -> Box<dyn QueryBuilder> {
        Box::new(self.clone())
    }
}

/// fr fr JOIN types supported by query builder
#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl fmt::Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JoinType::Inner => write!(f, "INNER"),
            JoinType::Left => write!(f, "LEFT"),
            JoinType::Right => write!(f, "RIGHT"),
            JoinType::Full => write!(f, "FULL"),
            JoinType::Cross => write!(f, "CROSS"),
        }
    }
}

/// fr fr WHERE condition operator
#[derive(Debug, Clone)]
enum WhereOperator {
    And,
    Or,
}

impl fmt::Display for WhereOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhereOperator::And => write!(f, "AND"),
            WhereOperator::Or => write!(f, "OR"),
        }
    }
}

/// fr fr ORDER BY direction
#[derive(Debug, Clone)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl fmt::Display for OrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderDirection::Ascending => write!(f, "ASC"),
            OrderDirection::Descending => write!(f, "DESC"),
        }
    }
}

/// fr fr Conflict resolution strategy for INSERT
#[derive(Debug, Clone)]
enum ConflictResolution {
    Ignore,
    Update(HashMap<String, SqlValue>),
}

/// fr fr JOIN clause structure
#[derive(Debug, Clone)]
struct JoinClause {
    join_type: JoinType,
    table: String,
    condition: String,
}

/// fr fr WHERE condition structure
#[derive(Debug, Clone)]
struct WhereCondition {
    expression: String,
    operator: WhereOperator,
}

/// fr fr SET clause for UPDATE statements
#[derive(Debug, Clone)]
struct SetClause {
    column: String,
    expression: String,
}

/// fr fr ORDER BY clause structure
#[derive(Debug, Clone)]
struct OrderByClause {
    column: String,
    direction: OrderDirection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_builder_basic() {
        let query = SelectBuilder::new(&["id", "name"])
            .from("users")
            .build()
            .unwrap();
        
        assert_eq!(query, "SELECT id, name FROM users");
    }

    #[test]
    fn test_select_builder_with_where() {
        let query = SelectBuilder::new(&["*"])
            .from("users")
            .where_eq("id", SqlValue::Integer(1))
            .build()
            .unwrap();
        
        assert!(query.contains("WHERE id = $param_1"));
    }

    #[test]
    fn test_select_builder_with_joins() {
        let query = SelectBuilder::new(&["u.name", "p.title"])
            .from("users u")
            .inner_join("posts p", "p.user_id = u.id")
            .build()
            .unwrap();
        
        assert!(query.contains("INNER JOIN posts p ON p.user_id = u.id"));
    }

    #[test]
    fn test_select_builder_complex() {
        let query = SelectBuilder::new(&["u.name", "COUNT(p.id) as post_count"])
            .from("users u")
            .left_join("posts p", "p.user_id = u.id")
            .where_expr("u.active = true")
            .group_by("u.id")
            .having("COUNT(p.id) > 0")
            .order_desc("post_count")
            .limit(10)
            .build()
            .unwrap();
        
        assert!(query.contains("GROUP BY u.id"));
        assert!(query.contains("HAVING COUNT(p.id) > 0"));
        assert!(query.contains("ORDER BY post_count DESC"));
        assert!(query.contains("LIMIT 10"));
    }

    #[test]
    fn test_insert_builder_basic() {
        let query = InsertBuilder::new("users")
            .columns(&["name", "email"])
            .values(&[SqlValue::String("John".to_string()), SqlValue::String("john@example.com".to_string())])
            .build()
            .unwrap();
        
        assert!(query.contains("INSERT INTO users"));
        assert!(query.contains("(name, email)"));
        assert!(query.contains("VALUES ($1, $2)"));
    }

    #[test]
    fn test_insert_builder_multiple_rows() {
        let builder = InsertBuilder::new("users")
            .columns(&["name", "age"])
            .values(&[SqlValue::String("John".to_string()), SqlValue::Integer(25)])
            .values(&[SqlValue::String("Jane".to_string()), SqlValue::Integer(30)]);
        
        let query = builder.build().unwrap();
        assert!(query.contains("VALUES ($1, $2), ($3, $4)"));
    }

    #[test]
    fn test_update_builder() {
        let query = UpdateBuilder::new("users")
            .set("name", SqlValue::String("NewName".to_string()))
            .set("age", SqlValue::Integer(26))
            .where_eq("id", SqlValue::Integer(1))
            .build()
            .unwrap();
        
        assert!(query.contains("UPDATE users"));
        assert!(query.contains("SET name = $param_1, age = $param_2"));
        assert!(query.contains("WHERE id = $param_3"));
    }

    #[test]
    fn test_delete_builder() {
        let query = DeleteBuilder::new("users")
            .where_eq("id", SqlValue::Integer(1))
            .build()
            .unwrap();
        
        assert_eq!(query, "DELETE FROM users WHERE id = $param_1");
    }

    #[test]
    fn test_select_builder_validation() {
        let result = SelectBuilder::new(&[]).build();
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_builder_validation() {
        let result = InsertBuilder::new("users").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_update_builder_validation() {
        let result = UpdateBuilder::new("users").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_where_in_condition() {
        let query = SelectBuilder::new(&["*"])
            .from("users")
            .where_in("id", &[SqlValue::Integer(1), SqlValue::Integer(2), SqlValue::Integer(3)])
            .build()
            .unwrap();
        
        assert!(query.contains("WHERE id IN ($param_1, $param_2, $param_3)"));
    }

    #[test]
    fn test_where_like_condition() {
        let query = SelectBuilder::new(&["*"])
            .from("users")
            .where_like("name", "%John%")
            .build()
            .unwrap();
        
        assert!(query.contains("WHERE name LIKE $param_1"));
    }

    #[test]
    fn test_join_types() {
        assert_eq!(JoinType::Inner.to_string(), "INNER");
        assert_eq!(JoinType::Left.to_string(), "LEFT");
        assert_eq!(JoinType::Right.to_string(), "RIGHT");
        assert_eq!(JoinType::Full.to_string(), "FULL");
        assert_eq!(JoinType::Cross.to_string(), "CROSS");
    }

    #[test]
    fn test_order_directions() {
        assert_eq!(OrderDirection::Ascending.to_string(), "ASC");
        assert_eq!(OrderDirection::Descending.to_string(), "DESC");
    }
}
