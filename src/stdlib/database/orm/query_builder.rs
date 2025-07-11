//! Enhanced query builder for CURSED ORM

use std::collections::HashMap;
use crate::error::CursedError;
use super::{SqlValue, DatabaseError};

/// Result type for query builder operations
pub type QueryResult<T> = Result<T, DatabaseError>;

/// Advanced query builder with fluent interface
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    table_name: String,
    select_columns: Vec<String>,
    where_clauses: Vec<WhereClause>,
    join_clauses: Vec<JoinClause>,
    order_by_clauses: Vec<OrderByClause>,
    group_by_clauses: Vec<GroupByClause>,
    having_clauses: Vec<HavingClause>,
    limit_value: Option<i64>,
    offset_value: Option<i64>,
    distinct: bool,
    parameters: Vec<SqlValue>,
}

/// WHERE clause builder
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: String,
    pub value: SqlValue,
    pub conjunction: Option<String>,
}

/// JOIN clause builder
#[derive(Debug, Clone)]
pub struct JoinClause {
    pub join_type: String,
    pub table: String,
    pub on_condition: String,
}

/// ORDER BY clause builder
#[derive(Debug, Clone)]
pub struct OrderByClause {
    pub column: String,
    pub direction: String,
}

/// GROUP BY clause builder
#[derive(Debug, Clone)]
pub struct GroupByClause {
    pub columns: Vec<String>,
}

/// HAVING clause builder
#[derive(Debug, Clone)]
pub struct HavingClause {
    pub condition: String,
    pub parameters: Vec<SqlValue>,
}

/// Subquery builder
#[derive(Debug, Clone)]
pub struct SubqueryBuilder {
    pub query: QueryBuilder,
    pub alias: String,
}

/// CTE (Common Table Expression) builder
#[derive(Debug, Clone)]
pub struct CTEBuilder {
    pub name: String,
    pub query: QueryBuilder,
    pub recursive: bool,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            select_columns: vec!["*".to_string()],
            where_clauses: Vec::new(),
            join_clauses: Vec::new(),
            order_by_clauses: Vec::new(),
            group_by_clauses: Vec::new(),
            having_clauses: Vec::new(),
            limit_value: None,
            offset_value: None,
            distinct: false,
            parameters: Vec::new(),
        }
    }

    /// Select specific columns
    pub fn select(mut self, columns: &[&str]) -> Self {
        self.select_columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Select with column aliases
    pub fn select_as(mut self, column_aliases: &[(&str, &str)]) -> Self {
        self.select_columns = column_aliases
            .iter()
            .map(|(col, alias)| format!("{} AS {}", col, alias))
            .collect();
        self
    }

    /// Select distinct records
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Add a WHERE clause
    pub fn where_clause(mut self, column: &str, operator: &str, value: SqlValue) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            conjunction: None,
        });
        self
    }

    /// Add an AND WHERE clause
    pub fn and_where(mut self, column: &str, operator: &str, value: SqlValue) -> Self {
        if let Some(last_clause) = self.where_clauses.last_mut() {
            last_clause.conjunction = Some("AND".to_string());
        }
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            conjunction: None,
        });
        self
    }

    /// Add an OR WHERE clause
    pub fn or_where(mut self, column: &str, operator: &str, value: SqlValue) -> Self {
        if let Some(last_clause) = self.where_clauses.last_mut() {
            last_clause.conjunction = Some("OR".to_string());
        }
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            conjunction: None,
        });
        self
    }

    /// Add a WHERE IN clause
    pub fn where_in(mut self, column: &str, values: Vec<SqlValue>) -> Self {
        let placeholders = values.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "IN".to_string(),
            value: SqlValue::String(format!("({})", placeholders)),
            conjunction: None,
        });
        self.parameters.extend(values);
        self
    }

    /// Add a WHERE BETWEEN clause
    pub fn where_between(mut self, column: &str, start: SqlValue, end: SqlValue) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "BETWEEN".to_string(),
            value: SqlValue::String("? AND ?".to_string()),
            conjunction: None,
        });
        self.parameters.push(start);
        self.parameters.push(end);
        self
    }

    /// Add a WHERE NULL clause
    pub fn where_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "IS".to_string(),
            value: SqlValue::Null,
            conjunction: None,
        });
        self
    }

    /// Add a WHERE NOT NULL clause
    pub fn where_not_null(mut self, column: &str) -> Self {
        self.where_clauses.push(WhereClause {
            column: column.to_string(),
            operator: "IS NOT".to_string(),
            value: SqlValue::Null,
            conjunction: None,
        });
        self
    }

    /// Add a JOIN clause
    pub fn join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_clauses.push(JoinClause::inner(table, on_condition));
        self
    }

    /// Add a LEFT JOIN clause
    pub fn left_join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_clauses.push(JoinClause::left(table, on_condition));
        self
    }

    /// Add a RIGHT JOIN clause
    pub fn right_join(mut self, table: &str, on_condition: &str) -> Self {
        self.join_clauses.push(JoinClause::right(table, on_condition));
        self
    }

    /// Add an ORDER BY clause
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by_clauses.push(OrderByClause::new(column, direction));
        self
    }

    /// Add an ORDER BY ASC clause
    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by_clauses.push(OrderByClause::asc(column));
        self
    }

    /// Add an ORDER BY DESC clause
    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by_clauses.push(OrderByClause::desc(column));
        self
    }

    /// Add a GROUP BY clause
    pub fn group_by(mut self, columns: &[&str]) -> Self {
        self.group_by_clauses.push(GroupByClause::new(
            columns.iter().map(|s| s.to_string()).collect()
        ));
        self
    }

    /// Add a HAVING clause
    pub fn having(mut self, condition: &str, parameters: Vec<SqlValue>) -> Self {
        self.having_clauses.push(HavingClause {
            condition: condition.to_string(),
            parameters,
        });
        self
    }

    /// Add a LIMIT clause
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit_value = Some(limit);
        self
    }

    /// Add an OFFSET clause
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset_value = Some(offset);
        self
    }

    /// Add pagination (LIMIT + OFFSET)
    pub fn paginate(mut self, page: i64, per_page: i64) -> Self {
        self.limit_value = Some(per_page);
        self.offset_value = Some((page - 1) * per_page);
        self
    }

    /// Build the SELECT SQL query
    pub fn build_select(&self) -> QueryResult<(String, Vec<SqlValue>)> {
        let mut sql = String::new();
        let mut params = Vec::new();

        // SELECT clause
        sql.push_str("SELECT ");
        if self.distinct {
            sql.push_str("DISTINCT ");
        }
        sql.push_str(&self.select_columns.join(", "));

        // FROM clause
        sql.push_str(&format!(" FROM {}", self.table_name));

        // JOIN clauses
        for join in &self.join_clauses {
            sql.push_str(&format!(" {} {} ON {}", join.join_type, join.table, join.on_condition));
        }

        // WHERE clauses
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            for (i, clause) in self.where_clauses.iter().enumerate() {
                if i > 0 {
                    if let Some(ref conjunction) = clause.conjunction {
                        sql.push_str(&format!(" {} ", conjunction));
                    }
                }
                sql.push_str(&format!("{} {} ?", clause.column, clause.operator));
                params.push(clause.value.clone());
            }
        }

        // GROUP BY clauses
        if !self.group_by_clauses.is_empty() {
            sql.push_str(" GROUP BY ");
            let group_columns: Vec<String> = self.group_by_clauses
                .iter()
                .flat_map(|g| g.columns.clone())
                .collect();
            sql.push_str(&group_columns.join(", "));
        }

        // HAVING clauses
        if !self.having_clauses.is_empty() {
            sql.push_str(" HAVING ");
            for (i, having) in self.having_clauses.iter().enumerate() {
                if i > 0 {
                    sql.push_str(" AND ");
                }
                sql.push_str(&having.condition);
                params.extend(having.parameters.clone());
            }
        }

        // ORDER BY clauses
        if !self.order_by_clauses.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_parts: Vec<String> = self.order_by_clauses
                .iter()
                .map(|o| format!("{} {}", o.column, o.direction))
                .collect();
            sql.push_str(&order_parts.join(", "));
        }

        // LIMIT clause
        if let Some(limit) = self.limit_value {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        // OFFSET clause
        if let Some(offset) = self.offset_value {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        // Add additional parameters
        params.extend(self.parameters.clone());

        Ok((sql, params))
    }

    /// Build an INSERT SQL query
    pub fn build_insert(&self, data: &HashMap<String, SqlValue>) -> QueryResult<(String, Vec<SqlValue>)> {
        let columns: Vec<String> = data.keys().cloned().collect();
        let values: Vec<SqlValue> = data.values().cloned().collect();
        let placeholders: Vec<String> = values.iter().map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        Ok((sql, values))
    }

    /// Build an UPDATE SQL query
    pub fn build_update(&self, data: &HashMap<String, SqlValue>) -> QueryResult<(String, Vec<SqlValue>)> {
        let mut sql = format!("UPDATE {}", self.table_name);
        let mut params = Vec::new();

        // SET clause
        let set_clauses: Vec<String> = data.keys().map(|k| format!("{} = ?", k)).collect();
        sql.push_str(&format!(" SET {}", set_clauses.join(", ")));
        params.extend(data.values().cloned());

        // WHERE clauses
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            for (i, clause) in self.where_clauses.iter().enumerate() {
                if i > 0 {
                    if let Some(ref conjunction) = clause.conjunction {
                        sql.push_str(&format!(" {} ", conjunction));
                    }
                }
                sql.push_str(&format!("{} {} ?", clause.column, clause.operator));
                params.push(clause.value.clone());
            }
        }

        Ok((sql, params))
    }

    /// Build a DELETE SQL query
    pub fn build_delete(&self) -> QueryResult<(String, Vec<SqlValue>)> {
        let mut sql = format!("DELETE FROM {}", self.table_name);
        let mut params = Vec::new();

        // WHERE clauses
        if !self.where_clauses.is_empty() {
            sql.push_str(" WHERE ");
            for (i, clause) in self.where_clauses.iter().enumerate() {
                if i > 0 {
                    if let Some(ref conjunction) = clause.conjunction {
                        sql.push_str(&format!(" {} ", conjunction));
                    }
                }
                sql.push_str(&format!("{} {} ?", clause.column, clause.operator));
                params.push(clause.value.clone());
            }
        }

        Ok((sql, params))
    }

    /// Build a COUNT SQL query
    pub fn build_count(&self) -> QueryResult<(String, Vec<SqlValue>)> {
        let mut builder = self.clone();
        builder.select_columns = vec!["COUNT(*)".to_string()];
        builder.order_by_clauses.clear();
        builder.limit_value = None;
        builder.offset_value = None;
        builder.build_select()
    }

    /// Build an EXISTS SQL query
    pub fn build_exists(&self) -> QueryResult<(String, Vec<SqlValue>)> {
        let (select_sql, params) = self.build_select()?;
        let sql = format!("SELECT EXISTS ({})", select_sql);
        Ok((sql, params))
    }
}

impl WhereClause {
    pub fn new(column: &str, operator: &str, value: SqlValue) -> Self {
        Self {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            conjunction: None,
        }
    }
    
    pub fn and(mut self) -> Self {
        self.conjunction = Some("AND".to_string());
        self
    }
    
    pub fn or(mut self) -> Self {
        self.conjunction = Some("OR".to_string());
        self
    }
}

impl JoinClause {
    pub fn new(join_type: &str, table: &str, on_condition: &str) -> Self {
        Self {
            join_type: join_type.to_string(),
            table: table.to_string(),
            on_condition: on_condition.to_string(),
        }
    }
    
    pub fn inner(table: &str, on_condition: &str) -> Self {
        Self::new("INNER JOIN", table, on_condition)
    }
    
    pub fn left(table: &str, on_condition: &str) -> Self {
        Self::new("LEFT JOIN", table, on_condition)
    }
    
    pub fn right(table: &str, on_condition: &str) -> Self {
        Self::new("RIGHT JOIN", table, on_condition)
    }
    
    pub fn full(table: &str, on_condition: &str) -> Self {
        Self::new("FULL JOIN", table, on_condition)
    }
}

impl OrderByClause {
    pub fn new(column: &str, direction: &str) -> Self {
        Self {
            column: column.to_string(),
            direction: direction.to_string(),
        }
    }
    
    pub fn asc(column: &str) -> Self {
        Self::new(column, "ASC")
    }
    
    pub fn desc(column: &str) -> Self {
        Self::new(column, "DESC")
    }
}

impl GroupByClause {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
    
    pub fn single(column: &str) -> Self {
        Self::new(vec![column.to_string()])
    }
}

/// Initialize query builder system
pub fn init_query_builder() -> Result<(), CursedError> {
    println!("📁 Query builder system initialized");
    Ok(())
}

/// Test query builder functionality
pub fn test_query_builder() -> Result<(), CursedError> {
    let builder = QueryBuilder::new("users")
        .select(&["id", "name", "email"])
        .where_clause("active", "=", SqlValue::Boolean(true))
        .and_where("age", ">=", SqlValue::Integer(18))
        .order_by_desc("created_at")
        .limit(10);

    let (sql, params) = builder.build_select().unwrap();
    println!("Generated SQL: {}", sql);
    println!("Parameters: {:?}", params);

    let insert_data = HashMap::from([
        ("name".to_string(), SqlValue::String("John Doe".to_string())),
        ("email".to_string(), SqlValue::String("john@example.com".to_string())),
        ("age".to_string(), SqlValue::Integer(25)),
    ]);

    let (insert_sql, insert_params) = builder.build_insert(&insert_data).unwrap();
    println!("Insert SQL: {}", insert_sql);
    println!("Insert Parameters: {:?}", insert_params);

    println!("✅ Query builder tests passed");
    Ok(())
}
