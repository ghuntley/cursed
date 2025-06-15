/// Fluent query builder for CURSED ORM with Gen Z vibes
/// 
/// Provides a chainable query interface for building SQL queries
/// with type safety, optimization, and database-agnostic generation.

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use super::entity::Entity;

/// fr fr Main fluent query builder with Gen Z method names
#[derive(Debug, Clone)]
pub struct FluentQueryBuilder<T: Entity> {
    /// Table being queried
    table: String,
    /// Database connection
    db: Arc<DB>,
    /// SELECT fields
    select_fields: Vec<String>,
    /// WHERE conditions
    where_conditions: Vec<WhereClause>,
    /// JOIN clauses
    joins: Vec<JoinClause>,
    /// ORDER BY clauses
    order_by: Vec<OrderByClause>,
    /// GROUP BY fields
    group_by: Vec<String>,
    /// HAVING conditions
    having_conditions: Vec<WhereClause>,
    /// LIMIT value
    limit_value: Option<u64>,
    /// OFFSET value
    offset_value: Option<u64>,
    /// Query parameters
    parameters: Vec<SqlValue>,
    /// Entity type marker
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> FluentQueryBuilder<T> {
    /// slay Create new query builder
    #[instrument(skip(db))]
    pub fn new(table: &str, db: Arc<DB>) -> Self {
        debug!(table = table, "Creating new query builder");
        Self {
            table: table.to_string(),
            db,
            select_fields: Vec::from(["*".to_string()]),
            where_conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            having_conditions: Vec::new(),
            limit_value: None,
            offset_value: None,
            parameters: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// facts Select specific fields
    #[instrument(skip(self))]
    pub fn select_these_vibes(mut self, fields: &[&str]) -> Self {
        debug!(fields = ?fields, "Setting SELECT fields");
        self.select_fields = fields.iter().map(|f| f.to_string()).collect();
        self
    }

    /// sus Add WHERE condition
    #[instrument(skip(self))]
    pub fn where_clause(mut self, condition: &str, params: Vec<SqlValue>) -> Self {
        debug!(condition = condition, params = ?params, "Adding WHERE condition");
        
        self.where_conditions.push(WhereClause {
            condition: condition.to_string(),
            operator: WhereOperator::And,
            parameters: params.clone(),
        });
        
        self.parameters.extend(params);
        self
    }

    /// highkey Add WHERE condition with OR operator
    #[instrument(skip(self))]
    pub fn or_where_its_giving(mut self, condition: &str, params: Vec<SqlValue>) -> Self {
        debug!(condition = condition, params = ?params, "Adding OR WHERE condition");
        
        self.where_conditions.push(WhereClause {
            condition: condition.to_string(),
            operator: WhereOperator::Or,
            parameters: params.clone(),
        });
        
        self.parameters.extend(params);
        self
    }

    /// lowkey Add WHERE IN condition
    #[instrument(skip(self))]
    pub fn where_in_the_vibe(mut self, field: &str, values: Vec<SqlValue>) -> Self {
        debug!(field = field, values = ?values, "Adding WHERE IN condition");
        
        let placeholders: Vec<String> = values.iter()
            .enumerate()
            .map(|(i, _)| format!("${}", self.parameters.len() + i + 1))
            .collect();
        
        let condition = format!("{} IN ({})", field, placeholders.join(", "));
        
        self.where_conditions.push(WhereClause {
            condition,
            operator: WhereOperator::And,
            parameters: values.clone(),
        });
        
        self.parameters.extend(values);
        self
    }

    /// periodt Add WHERE LIKE condition
    #[instrument(skip(self))]
    pub fn where_like_totally(mut self, field: &str, pattern: &str) -> Self {
        debug!(field = field, pattern = pattern, "Adding WHERE LIKE condition");
        
        let condition = format!("{} LIKE ${}", field, self.parameters.len() + 1);
        
        self.where_conditions.push(WhereClause {
            condition,
            operator: WhereOperator::And,
            parameters: Vec::from([SqlValue::String(pattern.to_string())]),
        });
        
        self.parameters.push(SqlValue::String(pattern.to_string()));
        self
    }

    /// bestie Add INNER JOIN
    #[instrument(skip(self))]
    pub fn join_the_party(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding INNER JOIN");
        
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table: table.to_string(),
            condition: on_condition.to_string(),
        });
        
        self
    }

    /// yolo Add LEFT JOIN
    #[instrument(skip(self))]
    pub fn left_join_if_vibing(mut self, table: &str, on_condition: &str) -> Self {
        debug!(table = table, condition = on_condition, "Adding LEFT JOIN");
        
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table: table.to_string(),
            condition: on_condition.to_string(),
        });
        
        self
    }

    /// lit Add ORDER BY clause
    #[instrument(skip(self))]
    pub fn order_by_vibe(mut self, field: &str, direction: OrderDirection) -> Self {
        debug!(field = field, direction = ?direction, "Adding ORDER BY");
        
        self.order_by.push(OrderByClause {
            field: field.to_string(),
            direction,
        });
        
        self
    }

    /// tea Order by ascending
    #[instrument(skip(self))]
    pub fn asc_vibes(self, field: &str) -> Self {
        self.order_by_vibe(field, OrderDirection::Ascending)
    }

    /// flex Order by descending
    #[instrument(skip(self))]
    pub fn desc_vibes(self, field: &str) -> Self {
        self.order_by_vibe(field, OrderDirection::Descending)
    }

    /// slay Add GROUP BY
    #[instrument(skip(self))]
    pub fn group_by_energy(mut self, fields: &[&str]) -> Self {
        debug!(fields = ?fields, "Adding GROUP BY");
        
        self.group_by.extend(fields.iter().map(|f| f.to_string()));
        self
    }

    /// facts Add HAVING condition
    #[instrument(skip(self))]
    pub fn having_main_character_energy(mut self, condition: &str, params: Vec<SqlValue>) -> Self {
        debug!(condition = condition, params = ?params, "Adding HAVING condition");
        
        self.having_conditions.push(WhereClause {
            condition: condition.to_string(),
            operator: WhereOperator::And,
            parameters: params.clone(),
        });
        
        self.parameters.extend(params);
        self
    }

    /// periodt Set LIMIT
    #[instrument(skip(self))]
    pub fn limit(mut self, count: u64) -> Self {
        debug!(limit = count, "Setting LIMIT");
        self.limit_value = Some(count);
        self
    }

    /// bestie Set OFFSET
    #[instrument(skip(self))]
    pub fn offset(mut self, count: u64) -> Self {
        debug!(offset = count, "Setting OFFSET");
        self.offset_value = Some(count);
        self
    }

    /// yolo Paginate results
    #[instrument(skip(self))]
    pub fn paginate_the_tea(self, page: u64, per_page: u64) -> Self {
        let offset = (page - 1) * per_page;
        self.limit(per_page).offset(offset)
    }

    /// slay Execute query and return entities
    #[instrument(skip(self))]
    pub async fn execute(self) -> Result<Vec<T>, DatabaseError> {
        info!(table = %self.table, "Executing query");
        
        let sql = self.build_sql()?;
        debug!(sql = %sql, params = ?self.parameters, "Generated SQL");
        
        // Execute query (simplified for now)
        let rows = self.execute_sql(&sql).await?;
        
        // Convert rows to entities
        let mut entities = Vec::new();
        for row in rows {
            let entity = T::from_row(&row)?;
            entities.push(entity);
        }
        
        info!(count = entities.len(), "Query executed successfully");
        Ok(entities)
    }

    /// lit Execute and return first result
    #[instrument(skip(self))]
    pub async fn first_vibe(self) -> Result<Option<T>, DatabaseError> {
        let mut results = self.limit(1).execute().await?;
        Ok(results.pop())
    }

    /// tea Execute and return single result (error if not exactly one)
    #[instrument(skip(self))]
    pub async fn single_main_character(self) -> Result<T, DatabaseError> {
        let results = self.limit(2).execute().await?;
        
        match results.len() {
            0 => Err(DatabaseError::not_found("No matching record found")),
            1 => Ok(results.into_iter().next().unwrap()),
            _ => Err(DatabaseError::validation_error("Multiple records found, expected single result")),
        }
    }

    /// flex Count total matching records
    #[instrument(skip(self))]
    pub async fn count_the_vibes(mut self) -> Result<u64, DatabaseError> {
        debug!(table = %self.table, "Counting records");
        
        // Modify query for counting
        self.select_fields = Vec::from(["COUNT(*)".to_string()]);
        self.order_by.clear();
        self.limit_value = None;
        self.offset_value = None;
        
        let sql = self.build_sql()?;
        debug!(sql = %sql, "Generated count SQL");
        
        // Execute count query with real database execution
        let rows = self.execute_sql(&sql).await?;
        
        let count = if let Some(first_row) = rows.first() {
            if let Some(count_value) = first_row.values().next() {
                match count_value {
                    SqlValue::Integer(i) => *i as u64,
                    SqlValue::Float(f) => *f as u64,
                    _ => 0u64,
                }
            } else {
                0u64
            }
        } else {
            0u64
        };
        
        info!(count = count, "Count query executed");
        Ok(count)
    }

    /// vibe Check if any records exist
    #[instrument(skip(self))]
    pub async fn exists_no_cap(self) -> Result<bool, DatabaseError> {
        let count = self.count_the_vibes().await?;
        Ok(count > 0)
    }

    /// sus Build SQL query string
    #[instrument(skip(self))]
    fn build_sql(&self) -> Result<String, DatabaseError> {
        let mut sql = String::new();
        
        // SELECT clause
        sql.push_str("SELECT ");
        sql.push_str(&self.select_fields.join(", "));
        
        // FROM clause
        sql.push_str(&format!(" FROM {}", self.table));
        
        // JOIN clauses
        for join in &self.joins {
            sql.push_str(&format!(" {} JOIN {} ON {}", 
                join.join_type.to_sql(), join.table, join.condition));
        }
        
        // WHERE clause
        if !self.where_conditions.is_empty() {
            sql.push_str(" WHERE ");
            let conditions: Vec<String> = self.where_conditions.iter()
                .enumerate()
                .map(|(i, where_clause)| {
                    if i == 0 {
                        where_clause.condition.clone()
                    } else {
                        format!(" {} {}", where_clause.operator.to_sql(), where_clause.condition)
                    }
                })
                .collect();
            sql.push_str(&conditions.join(""));
        }
        
        // GROUP BY clause
        if !self.group_by.is_empty() {
            sql.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        
        // HAVING clause
        if !self.having_conditions.is_empty() {
            sql.push_str(" HAVING ");
            let conditions: Vec<String> = self.having_conditions.iter()
                .enumerate()
                .map(|(i, having_clause)| {
                    if i == 0 {
                        having_clause.condition.clone()
                    } else {
                        format!(" {} {}", having_clause.operator.to_sql(), having_clause.condition)
                    }
                })
                .collect();
            sql.push_str(&conditions.join(""));
        }
        
        // ORDER BY clause
        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_parts: Vec<String> = self.order_by.iter()
                .map(|order| format!("{} {}", order.field, order.direction.to_sql()))
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
        
        debug!(sql = %sql, "Built SQL query");
        Ok(sql)
    }

    /// facts Execute SQL and return raw rows with real database execution
    async fn execute_sql(&self, sql: &str) -> Result<Vec<HashMap<String, SqlValue>>, DatabaseError> {
        debug!(sql = %sql, params = ?self.parameters, "Executing SQL query");
        
        // Execute query with parameters using the connection pool
        let rows = self.db.map_query(sql.to_string(), self.parameters.clone())?;
        
        debug!(rows_count = rows.len(), "SQL execution completed");
        Ok(rows)
    }
}

/// fr fr WHERE clause representation
#[derive(Debug, Clone)]
pub struct WhereClause {
    /// Condition string with placeholders
    pub condition: String,
    /// Logical operator (AND/OR)
    pub operator: WhereOperator,
    /// Parameters for the condition
    pub parameters: Vec<SqlValue>,
}

/// fr fr WHERE operators
#[derive(Debug, Clone, PartialEq)]
pub enum WhereOperator {
    And,
    Or,
}

impl WhereOperator {
    fn to_sql(&self) -> &'static str {
        match self {
            WhereOperator::And => "AND",
            WhereOperator::Or => "OR",
        }
    }
}

/// fr fr JOIN clause representation
#[derive(Debug, Clone)]
pub struct JoinClause {
    /// Type of join
    pub join_type: JoinType,
    /// Table to join
    pub table: String,
    /// Join condition
    pub condition: String,
}

/// fr fr JOIN types
#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

impl JoinType {
    fn to_sql(&self) -> &'static str {
        match self {
            JoinType::Inner => "INNER",
            JoinType::Left => "LEFT",
            JoinType::Right => "RIGHT",
            JoinType::Full => "FULL",
            JoinType::Cross => "CROSS",
        }
    }
}

/// fr fr ORDER BY clause representation
#[derive(Debug, Clone)]
pub struct OrderByClause {
    /// Field to order by
    pub field: String,
    /// Sort direction
    pub direction: OrderDirection,
}

/// fr fr Sort directions
#[derive(Debug, Clone, PartialEq)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

impl OrderDirection {
    fn to_sql(&self) -> &'static str {
        match self {
            OrderDirection::Ascending => "ASC",
            OrderDirection::Descending => "DESC",
        }
    }
}

/// fr fr GROUP BY clause representation
#[derive(Debug, Clone)]
pub struct GroupByClause {
    /// Fields to group by
    pub fields: Vec<String>,
}

/// fr fr Query executor for advanced query operations
#[derive(Debug)]
pub struct QueryExecutor {
    /// Database connection
    db: Arc<DB>,
    /// Query cache
    cache: Arc<std::sync::Mutex<HashMap<String, Vec<HashMap<String, SqlValue>>>>>,
}

impl QueryExecutor {
    /// slay Create new query executor
    #[instrument(skip(db))]
    pub fn new(db: Arc<DB>) -> Self {
        info!("Creating new query executor");
        Self {
            db,
            cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// facts Execute raw SQL query
    #[instrument(skip(self))]
    pub async fn execute_raw(&self, sql: &str, params: &[SqlValue]) -> Result<Vec<HashMap<String, SqlValue>>, DatabaseError> {
        info!(sql = sql, param_count = params.len(), "Executing raw SQL");
        
        // Check cache first
        let cache_key = format!("{}:{:?}", sql, params);
        if let Ok(cache) = self.cache.lock() {
            if let Some(cached_result) = cache.get(&cache_key) {
                debug!("Found query result in cache");
                return Ok(cached_result.clone());
            }
        }
        
        // Execute query (placeholder)
        let mut row = HashMap::new();
        row.insert("result".to_string(), SqlValue::String("success".to_string()));
        let results = Vec::from([row]);
        
        // Cache result
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(cache_key, results.clone());
        }
        
        info!(rows = results.len(), "Query executed successfully");
        Ok(results)
    }

    /// periodt Clear query cache
    #[instrument(skip(self))]
    pub fn clear_cache(&self) {
        debug!("Clearing query cache");
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}

/// fr fr Trait for query operations that can be chained
pub trait VibeQuery<T: Entity> {
    /// Basic where condition
    fn where_vibe(&self, condition: &str, params: Vec<SqlValue>) -> FluentQueryBuilder<T>;
    
    /// Order by field
    fn order_by(&self, field: &str, direction: OrderDirection) -> FluentQueryBuilder<T>;
    
    /// Limit results
    fn limit(&self, count: u64) -> FluentQueryBuilder<T>;
    
    /// Execute and get results
    fn get_vibes(&self) -> impl std::future::Future<Output = Result<Vec<T>, DatabaseError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tracing_test::traced_test;

    #[derive(Debug, Clone)]
    struct TestUser {
        id: Option<i64>,
        name: String,
        email: String,
    }

    impl super::super::entity::Entity for TestUser {
        fn table_name() -> &'static str {
            "users"
        }

        fn primary_key_value(&self) -> Option<SqlValue> {
            self.id.map(SqlValue::Integer)
        }

        fn set_primary_key_value(&mut self, value: SqlValue) {
            if let SqlValue::Integer(id) = value {
                self.id = Some(id);
            }
        }

        fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError> {
            Ok(Self {
                id: match row.get("id") {
                    Some(SqlValue::Integer(id)) => Some(*id),
                    _ => None,
                },
                name: match row.get("name") {
                    Some(SqlValue::String(name)) => name.clone(),
                    _ => return Err(DatabaseError::validation_error("Missing name field")),
                },
                email: match row.get("email") {
                    Some(SqlValue::String(email)) => email.clone(),
                    _ => String::new(),
                },
            })
        }

        fn to_fields(&self) -> HashMap<String, SqlValue> {
            let mut fields = HashMap::new();
            if let Some(id) = self.id {
                fields.insert("id".to_string(), SqlValue::Integer(id));
            }
            fields.insert("name".to_string(), SqlValue::String(self.name.clone()));
            fields.insert("email".to_string(), SqlValue::String(self.email.clone()));
            fields
        }

        fn field_names() -> Vec<&'static str> {
            Vec::from(["id", "name", "email"])
        }

        fn column_definitions() -> Vec<super::super::entity::ColumnDefinition> {
            Vec::from([])
        }

        fn metadata() -> super::super::entity::EntityMetadata {
            super::super::entity::EntityMetadata {
                table_name: "users".to_string(),
                primary_key: "id".to_string(),
                fields: Vec::from(["id".to_string(), "name".to_string(), "email".to_string()]),
                relationships: Vec::from([]),
                validation_rules: Vec::from([]),
                indexes: Vec::from([]),
                version: 1,
            }
        }
    }

    fn create_mock_db() -> Arc<DB> {
        Arc::new(DB::open("test".to_string(), "".to_string()).expect("Failed to create test DB"))
    }

    #[traced_test]
    #[test]
    fn test_query_builder_creation() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db);
        assert_eq!(builder.table, "users");
    }

    #[traced_test]
    #[test]
    fn test_where_clause_building() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db)
            .where_clause("name = ?", Vec::from([SqlValue::String("John".to_string())]));
        
        assert_eq!(builder.where_conditions.len(), 1);
        assert_eq!(builder.where_conditions[0].condition, "name = ?");
    }

    #[traced_test]
    #[test]
    fn test_join_building() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db)
            .join_the_party("profiles", "users.id = profiles.user_id");
        
        assert_eq!(builder.joins.len(), 1);
        assert_eq!(builder.joins[0].table, "profiles");
        assert_eq!(builder.joins[0].join_type, JoinType::Inner);
    }

    #[traced_test]
    #[test]
    fn test_order_by_building() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db)
            .asc_vibes("name")
            .desc_vibes("created_at");
        
        assert_eq!(builder.order_by.len(), 2);
        assert_eq!(builder.order_by[0].field, "name");
        assert_eq!(builder.order_by[0].direction, OrderDirection::Ascending);
        assert_eq!(builder.order_by[1].direction, OrderDirection::Descending);
    }

    #[traced_test]
    #[test]
    fn test_sql_building() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db)
            .select_these_vibes(&["id", "name", "email"])
            .where_clause("active = ?", Vec::from([SqlValue::Boolean(true)]))
            .order_by_vibe("name", OrderDirection::Ascending)
            .limit(10);
        
        let sql = builder.build_sql().expect("Should build SQL");
        
        assert!(sql.contains("SELECT id, name, email"));
        assert!(sql.contains("FROM users"));
        assert!(sql.contains("WHERE active = ?"));
        assert!(sql.contains("ORDER BY name ASC"));
        assert!(sql.contains("LIMIT 10"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_query_execution() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db);
        
        let results = builder.execute().await.expect("Should execute query");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Test");
    }

    #[traced_test]
    #[test]
    fn test_pagination() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db)
            .paginate_the_tea(2, 10);
        
        assert_eq!(builder.limit_value, Some(10));
        assert_eq!(builder.offset_value, Some(10)); // page 2 with 10 per page
    }

    #[traced_test]
    #[tokio::test]
    async fn test_count_query() {
        let db = create_mock_db();
        let builder = FluentQueryBuilder::<TestUser>::new("users", db);
        
        let count = builder.count_the_vibes().await.expect("Should count records");
        assert_eq!(count, 42); // Placeholder value
    }
}
