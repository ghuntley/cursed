/// fr fr Mock database driver for testing - fake it till you make it periodt
use crate::stdlib::packages::sql_vibes::{
    DatabaseDriver, DatabaseConnection, PreparedStatement, Transaction,
    ConnectionConfig, DriverInfo, DriverFeature, ConnectionInfo, TransactionState, TransactionIsolation,
    SqlResult, SqlError, SqlValue, Row, ResultSet, Parameter
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

/// fr fr Mock driver for testing - simulates database operations without real database
#[derive(Debug)]
pub struct MockDriver {
    name: String,
    version: String,
    behavior: Arc<Mutex<MockBehavior>>,
}

/// fr fr Mock behavior configuration - control how the mock behaves
#[derive(Debug, Clone)]
pub struct MockBehavior {
    pub should_fail_connection: bool,
    pub should_fail_queries: bool,
    pub should_timeout: bool,
    pub connection_delay: Duration,
    pub query_delay: Duration,
    pub mock_data: HashMap<String, ResultSet>,
    pub connection_count: usize,
    pub query_count: usize,
}

impl Default for MockBehavior {
    fn default() -> Self {
        Self {
            should_fail_connection: false,
            should_fail_queries: false,
            should_timeout: false,
            connection_delay: Duration::from_millis(10),
            query_delay: Duration::from_millis(5),
            mock_data: HashMap::new(),
            connection_count: 0,
            query_count: 0,
        }
    }
}

impl MockDriver {
    /// sus Create new mock driver
    pub fn new() -> Self {
        Self {
            name: "mock".to_string(),
            version: "1.0.0".to_string(),
            behavior: Arc::new(Mutex::new(MockBehavior::default())),
        }
    }
    
    /// facts Create mock driver with custom behavior
    pub fn with_behavior(behavior: MockBehavior) -> Self {
        Self {
            name: "mock".to_string(),
            version: "1.0.0".to_string(),
            behavior: Arc::new(Mutex::new(behavior)),
        }
    }
    
    /// lowkey Set mock behavior for testing
    pub fn set_behavior(&self, behavior: MockBehavior) {
        if let Ok(mut b) = self.behavior.lock() {
            *b = behavior;
        }
    }
    
    /// highkey Add mock data for specific queries
    pub fn add_mock_data(&self, sql: String, result_set: ResultSet) {
        if let Ok(mut behavior) = self.behavior.lock() {
            behavior.mock_data.insert(sql, result_set);
        }
    }
    
    /// periodt Get statistics about mock usage
    pub fn get_stats(&self) -> Option<(usize, usize)> {
        if let Ok(behavior) = self.behavior.lock() {
            Some((behavior.connection_count, behavior.query_count))
        } else {
            None
        }
    }
    
    /// bestie Reset mock statistics
    pub fn reset_stats(&self) {
        if let Ok(mut behavior) = self.behavior.lock() {
            behavior.connection_count = 0;
            behavior.query_count = 0;
        }
    }
}

impl Default for MockDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseDriver for MockDriver {
    fn connect(&self, config: ConnectionConfig) -> SqlResult<DatabaseConnection> {
        // Increment connection count
        if let Ok(mut behavior) = self.behavior.lock() {
            behavior.connection_count += 1;
            
            // Check if we should fail
            if behavior.should_fail_connection {
                return Err(SqlError::connection("Mock driver configured to fail connections - testing failure scenario bestie".to_string()));
            }
            
            // Simulate connection delay
            if behavior.connection_delay > Duration::ZERO {
                std::thread::sleep(behavior.connection_delay);
            }
        }
        
        // Validate connection string format
        if config.connection_string.is_empty() {
            return Err(SqlError::connection("Connection string cannot be empty - even for mock driver bestie".to_string()));
        }
        
        Ok(DatabaseConnection::Mock(MockConnection::new(config, self.behavior.clone())?))
    }
    
    fn driver_info(&self) -> DriverInfo {
        DriverInfo {
            name: self.name.clone(),
            version: self.version.clone(),
            supported_versions: vec!["any".to_string()],
            features: vec![
                DriverFeature::PreparedStatements,
                DriverFeature::Transactions,
                DriverFeature::Savepoints,
                DriverFeature::BatchExecution,
                DriverFeature::ConnectionPooling,
                DriverFeature::SslEncryption,
                DriverFeature::AsyncOperations,
                DriverFeature::CustomTypes,
                DriverFeature::StreamingResults,
                DriverFeature::JsonSupport,
                DriverFeature::FullTextSearch,
                DriverFeature::StoredProcedures,
                DriverFeature::WindowFunctions,
                DriverFeature::CommonTableExpressions,
            ],
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("testing_only".to_string(), "true".to_string());
                meta.insert("supports_everything".to_string(), "true".to_string());
                meta.insert("no_real_storage".to_string(), "true".to_string());
                meta
            },
        }
    }
    
    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        // Mock driver supports everything for testing
        true
    }
    
    fn validate_connection_string(&self, connection_string: &str) -> SqlResult<()> {
        if connection_string.is_empty() {
            return Err(SqlError::connection("Connection string cannot be empty - even for mock driver bestie".to_string()));
        }
        
        // Mock driver accepts any non-empty connection string
        Ok(())
    }
}

/// fr fr Mock connection implementation
#[derive(Debug)]
pub struct MockConnection {
    config: ConnectionConfig,
    connected_at: Instant,
    transaction_state: TransactionState,
    connection_id: u64,
    is_open: bool,
    behavior: Arc<Mutex<MockBehavior>>,
}

impl MockConnection {
    /// sus Create new mock connection
    pub fn new(config: ConnectionConfig, behavior: Arc<Mutex<MockBehavior>>) -> SqlResult<Self> {
        Ok(Self {
            config,
            connected_at: Instant::now(),
            transaction_state: TransactionState::None,
            connection_id: rand::random(),
            is_open: true,
            behavior,
        })
    }
}

impl DatabaseConnection for MockConnection {
    fn execute_query(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<ResultSet> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't execute queries bestie".to_string()));
        }
        
        // Increment query count and check behavior
        if let Ok(mut behavior) = self.behavior.lock() {
            behavior.query_count += 1;
            
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
            
            if behavior.should_timeout {
                return Err(SqlError::query("Mock driver simulating timeout - testing timeout scenario bestie".to_string()));
            }
            
            // Simulate query delay
            if behavior.query_delay > Duration::ZERO {
                std::thread::sleep(behavior.query_delay);
            }
            
            // Check for pre-configured mock data
            if let Some(result_set) = behavior.mock_data.get(sql) {
                return Ok(result_set.clone());
            }
        }
        
        validate_mock_sql(sql)?;
        validate_mock_parameters(params)?;
        
        // Generate mock result set
        Ok(create_mock_result_set(sql, params))
    }
    
    fn execute_statement(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<u64> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't execute statements bestie".to_string()));
        }
        
        // Check behavior
        if let Ok(mut behavior) = self.behavior.lock() {
            behavior.query_count += 1;
            
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
            
            // Simulate query delay
            if behavior.query_delay > Duration::ZERO {
                std::thread::sleep(behavior.query_delay);
            }
        }
        
        validate_mock_sql(sql)?;
        validate_mock_parameters(params)?;
        
        // Mock affected rows
        if sql.trim().to_uppercase().starts_with("INSERT") {
            Ok(1) // Mock: inserted 1 row
        } else if sql.trim().to_uppercase().starts_with("UPDATE") || 
                  sql.trim().to_uppercase().starts_with("DELETE") {
            Ok(params.len() as u64) // Mock: affected rows based on parameters
        } else {
            Ok(0)
        }
    }
    
    fn prepare_statement(&mut self, sql: &str) -> SqlResult<PreparedStatement> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't prepare statements bestie".to_string()));
        }
        
        validate_mock_sql(sql)?;
        
        Ok(PreparedStatement::Mock(MockPreparedStatement::new(
            sql.to_string(),
            count_mock_parameters(sql),
            self.connection_id,
            self.behavior.clone()
        )?))
    }
    
    fn begin_transaction(&mut self) -> SqlResult<Box<dyn Transaction>> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't start transaction bestie".to_string()));
        }
        
        if self.transaction_state != TransactionState::None {
            return Err(SqlError::connection("Transaction already active - finish current transaction first periodt".to_string()));
        }
        
        self.transaction_state = TransactionState::Active;
        
        Ok(Box::new(MockTransaction::new(
            self.connection_id,
            TransactionIsolation::ReadCommitted,
            self.behavior.clone()
        )?))
    }
    
    fn is_alive(&self) -> bool {
        self.is_open
    }
    
    fn close(&mut self) -> SqlResult<()> {
        if self.transaction_state == TransactionState::Active {
            return Err(SqlError::connection("Cannot close connection with active transaction - commit or rollback first bestie".to_string()));
        }
        
        self.is_open = false;
        Ok(())
    }
    
    fn connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            server_version: "Mock Database 1.0.0".to_string(),
            database_name: "mock_database".to_string(),
            username: "mock_user".to_string(),
            host: "mock_host".to_string(),
            port: 9999,
            connection_id: Some(self.connection_id),
            transaction_state: self.transaction_state,
            uptime: self.connected_at.elapsed(),
            server_properties: {
                let mut props = HashMap::new();
                props.insert("mock_driver".to_string(), "true".to_string());
                props.insert("testing_mode".to_string(), "enabled".to_string());
                props
            },
        }
    }
    
    fn execute_batch(&mut self, statements: &[(&str, &[Parameter])]) -> SqlResult<Vec<SqlResult<u64>>> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't execute batch bestie".to_string()));
        }
        
        let mut results = Vec::new();
        
        for (sql, params) in statements {
            let result = self.execute_statement(sql, params);
            results.push(result);
        }
        
        Ok(results)
    }
}

/// fr fr Mock prepared statement implementation
#[derive(Debug)]
pub struct MockPreparedStatement {
    sql: String,
    parameter_count: usize,
    connection_id: u64,
    is_closed: bool,
    behavior: Arc<Mutex<MockBehavior>>,
}

impl MockPreparedStatement {
    pub fn new(sql: String, parameter_count: usize, connection_id: u64, behavior: Arc<Mutex<MockBehavior>>) -> SqlResult<Self> {
        Ok(Self {
            sql,
            parameter_count,
            connection_id,
            is_closed: false,
            behavior,
        })
    }
}

impl PreparedStatement for MockPreparedStatement {
    fn execute(&mut self, params: &[Parameter]) -> SqlResult<ResultSet> {
        if self.is_closed {
            return Err(SqlError::connection("Prepared statement is closed - can't execute bestie".to_string()));
        }
        
        if params.len() != self.parameter_count {
            return Err(SqlError::connection(
                format!("Parameter count mismatch: expected {}, got {} - that's sus bestie", 
                    self.parameter_count, params.len())
            ));
        }
        
        // Check behavior
        if let Ok(behavior) = self.behavior.lock() {
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
            
            // Check for pre-configured mock data
            if let Some(result_set) = behavior.mock_data.get(&self.sql) {
                return Ok(result_set.clone());
            }
        }
        
        validate_mock_parameters(params)?;
        
        // Mock result set
        Ok(create_mock_result_set(&self.sql, params))
    }
    
    fn execute_update(&mut self, params: &[Parameter]) -> SqlResult<u64> {
        if self.is_closed {
            return Err(SqlError::connection("Prepared statement is closed - can't execute bestie".to_string()));
        }
        
        if params.len() != self.parameter_count {
            return Err(SqlError::connection(
                format!("Parameter count mismatch: expected {}, got {} - that's sus bestie", 
                    self.parameter_count, params.len())
            ));
        }
        
        // Check behavior
        if let Ok(behavior) = self.behavior.lock() {
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
        }
        
        validate_mock_parameters(params)?;
        
        // Mock affected rows
        Ok(1)
    }
    
    fn sql(&self) -> &str {
        &self.sql
    }
    
    fn parameter_count(&self) -> usize {
        self.parameter_count
    }
    
    fn close(&mut self) -> SqlResult<()> {
        self.is_closed = true;
        Ok(())
    }
}

/// fr fr Mock transaction implementation
#[derive(Debug)]
pub struct MockTransaction {
    connection_id: u64,
    isolation_level: TransactionIsolation,
    is_active: bool,
    savepoints: Vec<String>,
    behavior: Arc<Mutex<MockBehavior>>,
}

impl MockTransaction {
    pub fn new(connection_id: u64, isolation_level: TransactionIsolation, behavior: Arc<Mutex<MockBehavior>>) -> SqlResult<Self> {
        Ok(Self {
            connection_id,
            isolation_level,
            is_active: true,
            savepoints: Vec::new(),
            behavior,
        })
    }
}

impl Transaction for MockTransaction {
    fn execute_query(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<ResultSet> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - that's sus bestie".to_string()));
        }
        
        // Check behavior
        if let Ok(behavior) = self.behavior.lock() {
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
            
            // Check for pre-configured mock data
            if let Some(result_set) = behavior.mock_data.get(sql) {
                return Ok(result_set.clone());
            }
        }
        
        validate_mock_sql(sql)?;
        validate_mock_parameters(params)?;
        
        Ok(create_mock_result_set(sql, params))
    }
    
    fn execute_statement(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<u64> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - that's sus bestie".to_string()));
        }
        
        // Check behavior
        if let Ok(behavior) = self.behavior.lock() {
            if behavior.should_fail_queries {
                return Err(SqlError::query("Mock driver configured to fail queries - testing failure scenario bestie".to_string()));
            }
        }
        
        validate_mock_sql(sql)?;
        validate_mock_parameters(params)?;
        
        Ok(1) // Mock affected rows
    }
    
    fn commit(mut self: Box<Self>) -> SqlResult<()> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - can't commit bestie".to_string()));
        }
        
        self.is_active = false;
        Ok(())
    }
    
    fn rollback(mut self: Box<Self>) -> SqlResult<()> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - can't rollback bestie".to_string()));
        }
        
        self.is_active = false;
        Ok(())
    }
    
    fn savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - can't create savepoint bestie".to_string()));
        }
        
        if self.savepoints.contains(&name.to_string()) {
            return Err(SqlError::connection(format!("Savepoint '{}' already exists - choose different name periodt", name)));
        }
        
        self.savepoints.push(name.to_string());
        Ok(())
    }
    
    fn rollback_to_savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - can't rollback to savepoint bestie".to_string()));
        }
        
        if !self.savepoints.contains(&name.to_string()) {
            return Err(SqlError::connection(format!("Savepoint '{}' does not exist - check the name bestie", name)));
        }
        
        // Remove savepoints created after this one
        if let Some(pos) = self.savepoints.iter().position(|sp| sp == name) {
            self.savepoints.truncate(pos + 1);
        }
        
        Ok(())
    }
    
    fn release_savepoint(&mut self, name: &str) -> SqlResult<()> {
        if !self.is_active {
            return Err(SqlError::connection("Transaction is not active - can't release savepoint bestie".to_string()));
        }
        
        if let Some(pos) = self.savepoints.iter().position(|sp| sp == name) {
            self.savepoints.remove(pos);
            Ok(())
        } else {
            Err(SqlError::connection(format!("Savepoint '{}' does not exist - check the name bestie", name)))
        }
    }
    
    fn isolation_level(&self) -> TransactionIsolation {
        self.isolation_level
    }
}

// Helper functions for mock implementation

/// Validate mock SQL statement
fn validate_mock_sql(sql: &str) -> SqlResult<()> {
    if sql.trim().is_empty() {
        return Err(SqlError::query("SQL statement cannot be empty - even for mock driver bestie".to_string()));
    }
    
    Ok(()) // Mock driver is permissive
}

/// Validate mock parameters
fn validate_mock_parameters(params: &[Parameter]) -> SqlResult<()> {
    for (i, param) in params.iter().enumerate() {
        match param {
            Parameter::Named { name, value: _ } => {
                if name.is_empty() {
                    return Err(SqlError::query(format!("Parameter {} has empty name - that's sus bestie", i)));
                }
            },
            Parameter::Positional { index: _, value: _ } => {
                // Positional parameters are generally fine
            }
        }
    }
    Ok(())
}

/// Count parameters in mock SQL statement
fn count_mock_parameters(sql: &str) -> usize {
    // Mock driver supports both ? and $1, $2, etc.
    let question_marks = sql.matches('?').count();
    let dollar_params = (1..=20)
        .map(|i| format!("${}", i))
        .filter(|param| sql.contains(param))
        .count();
    
    std::cmp::max(question_marks, dollar_params)
}

/// Create mock result set for testing
fn create_mock_result_set(sql: &str, _params: &[Parameter]) -> ResultSet {
    let sql_upper = sql.trim().to_uppercase();
    
    if sql_upper.starts_with("SELECT") {
        // Mock SELECT result
        let columns = vec!["id".to_string(), "name".to_string(), "value".to_string(), "timestamp".to_string()];
        let rows = vec![
            Row::new(vec![
                SqlValue::Integer(1),
                SqlValue::Text("Mock Record 1".to_string()),
                SqlValue::Text("Value 1".to_string()),
                SqlValue::Text("2024-01-01T00:00:00Z".to_string()),
            ]),
            Row::new(vec![
                SqlValue::Integer(2),
                SqlValue::Text("Mock Record 2".to_string()),
                SqlValue::Text("Value 2".to_string()),
                SqlValue::Text("2024-01-02T00:00:00Z".to_string()),
            ]),
            Row::new(vec![
                SqlValue::Integer(3),
                SqlValue::Text("Mock Record 3".to_string()),
                SqlValue::Null,
                SqlValue::Text("2024-01-03T00:00:00Z".to_string()),
            ]),
        ];
        
        ResultSet::new(columns, rows)
    } else {
        // Empty result set for non-SELECT statements
        ResultSet::new(vec![], vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_driver_creation() {
        let driver = MockDriver::new();
        let info = driver.driver_info();
        
        assert_eq!(info.name, "mock");
        assert!(driver.supports_feature(DriverFeature::PreparedStatements));
        assert!(driver.supports_feature(DriverFeature::JsonSupport));
        assert!(driver.supports_feature(DriverFeature::SslEncryption));
    }

    #[test]
    fn test_mock_behavior_configuration() {
        let mut behavior = MockBehavior::default();
        behavior.should_fail_connection = true;
        
        let driver = MockDriver::with_behavior(behavior);
        let config = ConnectionConfig::new("mock://test".to_string());
        
        assert!(driver.connect(config).is_err());
    }

    #[test]
    fn test_mock_data_configuration() {
        let driver = MockDriver::new();
        let columns = vec!["test_col".to_string()];
        let rows = vec![Row::new(vec![SqlValue::Text("test_value".to_string())])];
        let result_set = ResultSet::new(columns, rows);
        
        driver.add_mock_data("SELECT * FROM test".to_string(), result_set);
        
        let config = ConnectionConfig::new("mock://test".to_string());
        let mut conn = driver.connect(config).unwrap();
        
        let result = conn.execute_query("SELECT * FROM test", &[]).unwrap();
        assert_eq!(result.columns().len(), 1);
        assert_eq!(result.rows().len(), 1);
    }

    #[test]
    fn test_mock_statistics() {
        let driver = MockDriver::new();
        assert_eq!(driver.get_stats(), Some((0, 0)));
        
        let config = ConnectionConfig::new("mock://test".to_string());
        let mut conn = driver.connect(config).unwrap();
        
        assert_eq!(driver.get_stats(), Some((1, 0))); // 1 connection, 0 queries
        
        let _ = conn.execute_query("SELECT 1", &[]);
        assert_eq!(driver.get_stats(), Some((1, 1))); // 1 connection, 1 query
        
        driver.reset_stats();
        assert_eq!(driver.get_stats(), Some((0, 0)));
    }

    #[test]
    fn test_count_mock_parameters() {
        assert_eq!(count_mock_parameters("SELECT * FROM users WHERE id = ?"), 1);
        assert_eq!(count_mock_parameters("SELECT * FROM users WHERE id = $1"), 1);
        assert_eq!(count_mock_parameters("SELECT * FROM users WHERE id = ? AND name = ?"), 2);
        assert_eq!(count_mock_parameters("SELECT * FROM users WHERE id = $1 AND name = $2"), 2);
        assert_eq!(count_mock_parameters("SELECT * FROM users"), 0);
    }

    #[test]
    fn test_validate_mock_sql() {
        assert!(validate_mock_sql("SELECT * FROM users").is_ok());
        assert!(validate_mock_sql("INSERT INTO users (name) VALUES (?)").is_ok());
        assert!(validate_mock_sql("").is_err());
        // Mock driver is permissive, so even dangerous SQL is allowed for testing
        assert!(validate_mock_sql("SELECT * FROM users; DROP TABLE users").is_ok());
    }
}
