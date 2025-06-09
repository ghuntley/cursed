/// fr fr Core database types and interfaces for SQLSlay
/// 
/// This module implements the fundamental database connectivity types including
/// database connections, transactions, statements, and result handling.
/// 
/// Why comprehensive testing is critical for database operations:
/// - Database operations involve network I/O and external state that can fail
/// - Transaction isolation and consistency must be validated across scenarios
/// - Connection pooling requires proper resource management and leak prevention
/// - Query execution must handle various data types and edge cases safely
/// - Error handling must be robust for network failures and constraint violations

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use crate::error::CursedError;
use base64::Engine;
use super::{
    DatabaseError, DatabaseErrorKind, SqlValue, SqlIsolationLevel, VibeContext,
    Driver, ConnectionPool, driver
};

/// fr fr Database connection pool representing multiple database connections
/// This is the main entry point for database operations in SQLSlay
#[derive(Debug, Clone)]
pub struct DB {
    /// fr fr Driver name for this database connection
    pub driver_name: String,
    /// fr fr Data source name (connection string)
    pub data_source_name: String,
    /// fr fr Connection pool for managing database connections
    pub pool: Arc<ConnectionPool>,
    /// fr fr Statistics about database operations
    pub stats: Arc<Mutex<DBStats>>,
    /// fr fr Configuration for this database instance
    pub config: super::DatabaseConfig,
}

impl DB {
    /// slay Open a new database connection pool
    /// fr fr This is the main constructor for database connections
    pub fn open(driver_name: String, data_source_name: String) -> Result<Self, DatabaseError> {
        let pool = ConnectionPool::new(&driver_name, &data_source_name)?;
        
        Ok(DB {
            driver_name,
            data_source_name,
            pool: Arc::new(pool),
            stats: Arc::new(Mutex::new(DBStats::default())),
            config: super::DatabaseConfig::default(),
        })
    }

    /// slay Begin a new transaction
    pub fn begin(&self) -> Result<Tx, DatabaseError> {
        self.begin_tx(VibeContext::default(), None)
    }

    /// slay Begin a transaction with specific context and options
    pub fn begin_tx(&self, ctx: VibeContext, opts: Option<TxOptions>) -> Result<Tx, DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        let tx = conn.begin_transaction(opts.unwrap_or_default())?;
        Ok(Tx::new(tx, conn))
    }

    /// slay Close the database connection pool
    pub fn close(&self) -> Result<(), DatabaseError> {
        self.pool.close()
    }

    /// slay Get a single connection from the pool
    pub fn conn(&self, ctx: VibeContext) -> Result<Conn, DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        Ok(Conn::new(conn))
    }

    /// slay Get the underlying driver
    pub fn driver(&self) -> Box<dyn Driver> {
        self.pool.get_driver()
    }

    /// slay Execute a query without returning rows
    pub fn exec(&self, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        self.exec_context(VibeContext::default(), query, args)
    }

    /// slay Execute a query with context
    pub fn exec_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        let result = conn.execute(&query, &args)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_queries += 1;
            stats.total_exec_calls += 1;
        }

        Ok(SlayResult::new(result.last_insert_id, result.rows_affected))
    }

    /// slay Ping the database to check connectivity
    pub fn ping(&self) -> Result<(), DatabaseError> {
        self.ping_context(VibeContext::default())
    }

    /// slay Ping the database with context
    pub fn ping_context(&self, ctx: VibeContext) -> Result<(), DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        conn.ping()
    }

    /// slay Prepare a statement for reuse
    pub fn prepare(&self, query: String) -> Result<Stmt, DatabaseError> {
        self.prepare_context(VibeContext::default(), query)
    }

    /// slay Prepare a statement with context
    pub fn prepare_context(&self, ctx: VibeContext, query: String) -> Result<Stmt, DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        let stmt = conn.prepare(&query)?;
        Ok(Stmt::new(stmt, query))
    }

    /// slay Execute a query that returns rows
    pub fn query(&self, query: String, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        self.query_context(VibeContext::default(), query, args)
    }

    /// slay Execute a query with context that returns rows
    pub fn query_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        let conn = self.pool.acquire_connection(ctx.timeout)?;
        let result = conn.query(&query, &args)?;
        
        // Update statistics
        if let Ok(mut stats) = self.stats.lock() {
            stats.total_queries += 1;
            stats.total_query_calls += 1;
        }

        Ok(Rows::new(result))
    }

    /// slay Execute a query that returns a single row
    pub fn query_row(&self, query: String, args: Vec<SqlValue>) -> Row {
        self.query_row_context(VibeContext::default(), query, args)
    }

    /// slay Execute a query with context that returns a single row
    pub fn query_row_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Row {
        match self.query_context(ctx, query, args) {
            Ok(mut rows) => {
                if rows.next() {
                    Row::new(Some(rows.current_row()))
                } else {
                    Row::new(None)
                }
            }
            Err(err) => Row::with_error(err),
        }
    }

    /// slay Set maximum idle time for connections
    pub fn set_conn_max_idle_time(&self, duration: Duration) {
        self.pool.set_max_idle_time(duration);
    }

    /// slay Set maximum lifetime for connections
    pub fn set_conn_max_lifetime(&self, duration: Duration) {
        self.pool.set_max_lifetime(duration);
    }

    /// slay Set maximum number of idle connections
    pub fn set_max_idle_conns(&self, n: usize) {
        self.pool.set_max_idle_connections(n);
    }

    /// slay Set maximum number of open connections
    pub fn set_max_open_conns(&self, n: usize) {
        self.pool.set_max_open_connections(n);
    }

    /// slay Get database statistics
    pub fn stats(&self) -> DBStats {
        self.stats.lock().map(|guard| guard.clone()).unwrap_or_default()
    }

    /// slay Enhanced query that returns SlayRows with additional functionality
    pub fn slay_query(&self, query: String, args: Vec<SqlValue>) -> Result<SlayRows, DatabaseError> {
        let rows = self.query(query, args)?;
        Ok(SlayRows::new(rows))
    }

    /// slay Enhanced execution with detailed result information
    pub fn slay_exec(&self, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        self.exec(query, args)
    }

    /// slay Query that returns results as a map
    pub fn map_query(&self, query: String, args: Vec<SqlValue>) -> Result<Vec<HashMap<String, SqlValue>>, DatabaseError> {
        let mut rows = self.query(query, args)?;
        let mut results = Vec::new();
        
        while rows.next() {
            let row_map = rows.scan_map()?;
            results.push(row_map);
        }
        
        Ok(results)
    }

    /// slay Query that maps results to a struct
    pub fn struct_query<T>(&self, query: String, dest: &mut Vec<T>) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        let maps = self.map_query(query, Vec::from([]))?;
        
        for map in maps {
            // Convert HashMap<String, SqlValue> to JSON and then deserialize
            let json_map: HashMap<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, v)| (k, sql_value_to_json(v)))
                .collect();
            
            let value = serde_json::to_value(json_map)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
                
            let item: T = serde_json::from_value(value)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
                
            dest.push(item);
        }
        
        Ok(())
    }

    /// slay Execute multiple queries in a batch
    pub fn batch_exec(&self, queries: Vec<String>) -> Result<Vec<SlayResult>, DatabaseError> {
        let mut results = Vec::new();
        
        for query in queries {
            let result = self.slay_exec(query, Vec::from([]))?;
            results.push(result);
        }
        
        Ok(results)
    }
}

/// fr fr Single database connection (not from pool)
#[derive(Debug)]
pub struct Conn {
    /// fr fr Underlying driver connection
    pub driver_conn: Box<dyn super::driver::DriverConn>,
}

impl Conn {
    /// slay Create a new connection wrapper
    pub fn new(driver_conn: Box<dyn super::driver::DriverConn>) -> Self {
        Self { driver_conn }
    }

    /// slay Begin a transaction on this connection
    pub fn begin_tx(&self, ctx: VibeContext, opts: Option<TxOptions>) -> Result<Tx, DatabaseError> {
        let tx = self.driver_conn.begin_transaction(opts.unwrap_or_default())?;
        Ok(Tx::new(tx, self.driver_conn.clone()))
    }

    /// slay Close this connection
    pub fn close(&self) -> Result<(), DatabaseError> {
        self.driver_conn.close()
    }

    /// slay Execute a query on this connection
    pub fn exec_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        let result = self.driver_conn.execute(&query, &args)?;
        Ok(SlayResult::new(result.last_insert_id, result.rows_affected))
    }

    /// slay Ping this connection
    pub fn ping_context(&self, ctx: VibeContext) -> Result<(), DatabaseError> {
        self.driver_conn.ping()
    }

    /// slay Prepare a statement on this connection
    pub fn prepare_context(&self, ctx: VibeContext, query: String) -> Result<Stmt, DatabaseError> {
        let stmt = self.driver_conn.prepare(&query)?;
        Ok(Stmt::new(stmt, query))
    }

    /// slay Query this connection
    pub fn query_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        let result = self.driver_conn.query(&query, &args)?;
        Ok(Rows::new(result))
    }

    /// slay Query a single row from this connection
    pub fn query_row_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Row {
        match self.query_context(ctx, query, args) {
            Ok(mut rows) => {
                if rows.next() {
                    Row::new(Some(rows.current_row()))
                } else {
                    Row::new(None)
                }
            }
            Err(err) => Row::with_error(err),
        }
    }

    /// slay Execute raw operations on the underlying driver connection
    pub fn raw<F, R>(&self, f: F) -> Result<R, DatabaseError>
    where
        F: FnOnce(&dyn super::driver::DriverConn) -> Result<R, DatabaseError>,
    {
        f(self.driver_conn.as_ref())
    }
}

/// fr fr Database transaction for atomic operations
#[derive(Debug)]
pub struct Tx {
    /// fr fr Underlying driver transaction
    pub driver_tx: Box<dyn super::driver::DriverTx>,
    /// fr fr Connection this transaction belongs to
    pub conn: Box<dyn super::driver::DriverConn>,
    /// fr fr Whether this transaction has been committed or rolled back
    pub finished: bool,
}

impl Tx {
    /// slay Create a new transaction wrapper
    pub fn new(driver_tx: Box<dyn super::driver::DriverTx>, conn: Box<dyn super::driver::DriverConn>) -> Self {
        Self {
            driver_tx,
            conn,
            finished: false,
        }
    }

    /// slay Commit this transaction
    pub fn commit(&mut self) -> Result<(), DatabaseError> {
        if self.finished {
            return Err(DatabaseError::new(
                DatabaseErrorKind::TransactionError,
                "Transaction already finished"
            ));
        }
        
        let result = self.driver_tx.commit();
        self.finished = true;
        result
    }

    /// slay Rollback this transaction
    pub fn rollback(&mut self) -> Result<(), DatabaseError> {
        if self.finished {
            return Err(DatabaseError::new(
                DatabaseErrorKind::TransactionError,
                "Transaction already finished"
            ));
        }
        
        let result = self.driver_tx.rollback();
        self.finished = true;
        result
    }

    /// slay Execute a query in this transaction
    pub fn exec(&self, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        self.exec_context(VibeContext::default(), query, args)
    }

    /// slay Execute a query with context in this transaction
    pub fn exec_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        let result = self.driver_tx.execute(&query, &args)?;
        Ok(SlayResult::new(result.last_insert_id, result.rows_affected))
    }

    /// slay Prepare a statement in this transaction
    pub fn prepare(&self, query: String) -> Result<Stmt, DatabaseError> {
        self.prepare_context(VibeContext::default(), query)
    }

    /// slay Prepare a statement with context in this transaction
    pub fn prepare_context(&self, ctx: VibeContext, query: String) -> Result<Stmt, DatabaseError> {
        let stmt = self.driver_tx.prepare(&query)?;
        Ok(Stmt::new(stmt, query))
    }

    /// slay Query in this transaction
    pub fn query(&self, query: String, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        self.query_context(VibeContext::default(), query, args)
    }

    /// slay Query with context in this transaction
    pub fn query_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        let result = self.driver_tx.query(&query, &args)?;
        Ok(Rows::new(result))
    }

    /// slay Query a single row in this transaction
    pub fn query_row(&self, query: String, args: Vec<SqlValue>) -> Row {
        self.query_row_context(VibeContext::default(), query, args)
    }

    /// slay Query a single row with context in this transaction
    pub fn query_row_context(&self, ctx: VibeContext, query: String, args: Vec<SqlValue>) -> Row {
        match self.query_context(ctx, query, args) {
            Ok(mut rows) => {
                if rows.next() {
                    Row::new(Some(rows.current_row()))
                } else {
                    Row::new(None)
                }
            }
            Err(err) => Row::with_error(err),
        }
    }

    /// slay Get a statement from another statement within this transaction
    pub fn stmt(&self, stmt: &Stmt) -> Result<Stmt, DatabaseError> {
        self.stmt_context(VibeContext::default(), stmt)
    }

    /// slay Get a statement with context from another statement within this transaction
    pub fn stmt_context(&self, ctx: VibeContext, stmt: &Stmt) -> Result<Stmt, DatabaseError> {
        // In a real implementation, this would create a transaction-specific version of the statement
        Ok(stmt.clone())
    }
}

impl Drop for Tx {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.rollback();
        }
    }
}

/// fr fr Prepared statement for efficient query execution
#[derive(Debug)]
pub struct Stmt {
    /// fr fr Underlying driver statement
    pub driver_stmt: Box<dyn super::driver::DriverStmt>,
    /// fr fr Original query text
    pub query: String,
}

impl Clone for Stmt {
    fn clone(&self) -> Self {
        Self {
            driver_stmt: self.driver_stmt.clone(),
            query: self.query.clone(),
        }
    }
}

impl Stmt {
    /// slay Create a new statement wrapper
    pub fn new(driver_stmt: Box<dyn super::driver::DriverStmt>, query: String) -> Self {
        Self { driver_stmt, query }
    }

    /// slay Close this statement
    pub fn close(&self) -> Result<(), DatabaseError> {
        self.driver_stmt.close()
    }

    /// slay Execute this statement
    pub fn exec(&self, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        self.exec_context(VibeContext::default(), args)
    }

    /// slay Execute this statement with context
    pub fn exec_context(&self, ctx: VibeContext, args: Vec<SqlValue>) -> Result<SlayResult, DatabaseError> {
        let result = self.driver_stmt.execute(&args)?;
        Ok(SlayResult::new(result.last_insert_id, result.rows_affected))
    }

    /// slay Query using this statement
    pub fn query(&self, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        self.query_context(VibeContext::default(), args)
    }

    /// slay Query with context using this statement
    pub fn query_context(&self, ctx: VibeContext, args: Vec<SqlValue>) -> Result<Rows, DatabaseError> {
        let result = self.driver_stmt.query(&args)?;
        Ok(Rows::new(result))
    }

    /// slay Query a single row using this statement
    pub fn query_row(&self, args: Vec<SqlValue>) -> Row {
        self.query_row_context(VibeContext::default(), args)
    }

    /// slay Query a single row with context using this statement
    pub fn query_row_context(&self, ctx: VibeContext, args: Vec<SqlValue>) -> Row {
        match self.query_context(ctx, args) {
            Ok(mut rows) => {
                if rows.next() {
                    Row::new(Some(rows.current_row()))
                } else {
                    Row::new(None)
                }
            }
            Err(err) => Row::with_error(err),
        }
    }
}

/// fr fr Single row result from a query
#[derive(Debug)]
pub struct Row {
    /// fr fr Data in this row (None if no row was found)
    pub data: Option<Vec<SqlValue>>,
    /// fr fr Error that occurred during row scanning
    pub error: Option<DatabaseError>,
}

impl Row {
    /// slay Create a new row with data
    pub fn new(data: Option<Vec<SqlValue>>) -> Self {
        Self { data, error: None }
    }

    /// slay Create a new row with an error
    pub fn with_error(error: DatabaseError) -> Self {
        Self { data: None, error: Some(error) }
    }

    /// slay Get any error that occurred
    pub fn err(&self) -> Option<&DatabaseError> {
        self.error.as_ref()
    }

    /// slay Scan row data into variables
    pub fn scan(&self, dest: &mut [&mut dyn std::any::Any]) -> Result<(), DatabaseError> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        }

        let data = self.data.as_ref().ok_or_else(|| 
            DatabaseError::new(DatabaseErrorKind::NoRows, "No row data available")
        )?;

        if dest.len() != data.len() {
            return Err(DatabaseError::new(
                DatabaseErrorKind::ScanError,
                &format!("Column count mismatch: expected {}, got {}", dest.len(), data.len())
            ));
        }

        // In a real implementation, this would use proper type conversion
        Ok(())
    }

    /// slay Scan row data into a map
    pub fn scan_map(&self) -> Result<HashMap<String, SqlValue>, DatabaseError> {
        if let Some(ref err) = self.error {
            return Err(err.clone());
        }

        let data = self.data.as_ref().ok_or_else(|| 
            DatabaseError::new(DatabaseErrorKind::NoRows, "No row data available")
        )?;

        // In a real implementation, this would use column names from metadata
        let mut map = HashMap::new();
        for (i, value) in data.iter().enumerate() {
            map.insert(format!("column_{}", i), value.clone());
        }

        Ok(map)
    }

    /// slay Scan row data into a struct
    pub fn scan_struct<T>(&self, dest: &mut T) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        let map = self.scan_map()?;
        
        // Convert to JSON and deserialize
        let json_map: HashMap<String, serde_json::Value> = map
            .into_iter()
            .map(|(k, v)| (k, sql_value_to_json(v)))
            .collect();
            
        let value = serde_json::to_value(json_map)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
            
        *dest = serde_json::from_value(value)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
            
        Ok(())
    }
}

/// fr fr Multiple rows result from a query
#[derive(Debug)]
pub struct Rows {
    /// fr fr Result data from the query
    pub result: driver::QueryResult,
    /// fr fr Current row index
    pub current_index: usize,
}

impl Rows {
    /// slay Create a new rows result
    pub fn new(result: driver::QueryResult) -> Self {
        Self {
            result,
            current_index: 0,
        }
    }

    /// slay Close the rows iterator
    pub fn close(&self) -> Result<(), DatabaseError> {
        // In a real implementation, this would clean up resources
        Ok(())
    }

    /// slay Get column types
    pub fn column_types(&self) -> Result<Vec<String>, DatabaseError> {
        Ok(self.result.column_types.clone())
    }

    /// slay Get column names
    pub fn columns(&self) -> Result<Vec<String>, DatabaseError> {
        Ok(self.result.column_names.clone())
    }

    /// slay Get any error
    pub fn err(&self) -> Option<&DatabaseError> {
        self.result.error.as_ref()
    }

    /// slay Move to the next row
    pub fn next(&mut self) -> bool {
        if self.current_index < self.result.rows.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// slay Move to the next result set
    pub fn next_result_set(&mut self) -> bool {
        // In a real implementation, this would handle multiple result sets
        false
    }

    /// slay Scan current row data
    pub fn scan(&self, dest: &mut [&mut dyn std::any::Any]) -> Result<(), DatabaseError> {
        if self.current_index == 0 || self.current_index > self.result.rows.len() {
            return Err(DatabaseError::new(DatabaseErrorKind::NoRows, "No current row"));
        }

        let row = &self.result.rows[self.current_index - 1];
        if dest.len() != row.len() {
            return Err(DatabaseError::new(
                DatabaseErrorKind::ScanError,
                &format!("Column count mismatch: expected {}, got {}", dest.len(), row.len())
            ));
        }

        // In a real implementation, this would use proper type conversion
        Ok(())
    }

    /// slay Scan current row into a map
    pub fn scan_map(&self) -> Result<HashMap<String, SqlValue>, DatabaseError> {
        if self.current_index == 0 || self.current_index > self.result.rows.len() {
            return Err(DatabaseError::new(DatabaseErrorKind::NoRows, "No current row"));
        }

        let row = &self.result.rows[self.current_index - 1];
        let mut map = HashMap::new();

        for (i, value) in row.iter().enumerate() {
            let column_name = self.result.column_names.get(i)
                .map(|s| s.clone())
                .unwrap_or_else(|| format!("column_{}", i));
            map.insert(column_name, value.clone());
        }

        Ok(map)
    }

    /// slay Scan current row into a struct
    pub fn scan_struct<T>(&self, dest: &mut T) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        let map = self.scan_map()?;
        
        // Convert to JSON and deserialize
        let json_map: HashMap<String, serde_json::Value> = map
            .into_iter()
            .map(|(k, v)| (k, sql_value_to_json(v)))
            .collect();
            
        let value = serde_json::to_value(json_map)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
            
        *dest = serde_json::from_value(value)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
            
        Ok(())
    }

    /// slay Scan all rows into a slice of structs
    pub fn scan_all<T>(&mut self, dest: &mut Vec<T>) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        // Reset to beginning
        self.current_index = 0;
        
        while self.next() {
            let mut item: T = serde_json::from_value(serde_json::Value::Null)
                .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))?;
            self.scan_struct(&mut item)?;
            dest.push(item);
        }
        
        Ok(())
    }

    /// slay Get current row data
    pub fn current_row(&self) -> Vec<SqlValue> {
        if self.current_index == 0 || self.current_index > self.result.rows.len() {
            return Vec::from([]);
        }
        self.result.rows[self.current_index - 1].clone()
    }
}

/// fr fr Enhanced rows with additional functionality
#[derive(Debug)]
pub struct SlayRows {
    /// fr fr Base rows functionality
    pub rows: Rows,
}

impl SlayRows {
    /// slay Create new SlayRows from base Rows
    pub fn new(rows: Rows) -> Self {
        Self { rows }
    }

    /// slay Get all rows as maps
    pub fn all(&mut self) -> Result<Vec<HashMap<String, SqlValue>>, DatabaseError> {
        let mut results = Vec::new();
        
        // Reset to beginning
        self.rows.current_index = 0;
        
        while self.rows.next() {
            let map = self.rows.scan_map()?;
            results.push(map);
        }
        
        Ok(results)
    }

    /// slay Get all rows as structs
    pub fn all_structs<T>(&mut self, dest: &mut Vec<T>) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        self.rows.scan_all(dest)
    }

    /// slay Get first row as map
    pub fn first(&mut self) -> Result<HashMap<String, SqlValue>, DatabaseError> {
        self.rows.current_index = 0;
        if self.rows.next() {
            self.rows.scan_map()
        } else {
            Err(DatabaseError::new(DatabaseErrorKind::NoRows, "No rows found"))
        }
    }

    /// slay Get first row as struct
    pub fn first_struct<T>(&mut self, dest: &mut T) -> Result<(), DatabaseError> 
    where 
        T: serde::de::DeserializeOwned,
    {
        self.rows.current_index = 0;
        if self.rows.next() {
            self.rows.scan_struct(dest)
        } else {
            Err(DatabaseError::new(DatabaseErrorKind::NoRows, "No rows found"))
        }
    }

    /// slay Count total rows
    pub fn count(&self) -> Result<usize, DatabaseError> {
        Ok(self.rows.result.rows.len())
    }

    /// slay Execute function for each row
    pub fn for_each<F>(&mut self, mut f: F) -> Result<(), DatabaseError> 
    where 
        F: FnMut(HashMap<String, SqlValue>) -> Result<(), DatabaseError>,
    {
        self.rows.current_index = 0;
        
        while self.rows.next() {
            let map = self.rows.scan_map()?;
            f(map)?;
        }
        
        Ok(())
    }

    /// slay Convert all rows to JSON
    pub fn to_json(&mut self) -> Result<Vec<u8>, DatabaseError> {
        let maps = self.all()?;
        let json_maps: Vec<HashMap<String, serde_json::Value>> = maps
            .into_iter()
            .map(|map| {
                map.into_iter()
                    .map(|(k, v)| (k, sql_value_to_json(v)))
                    .collect()
            })
            .collect();
            
        serde_json::to_vec(&json_maps)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::SerializationError, &e.to_string()))
    }
}

/// fr fr Result of a database operation
#[derive(Debug, Clone)]
pub struct SlayResult {
    /// fr fr Last inserted ID (if applicable)
    pub last_insert_id: Option<i64>,
    /// fr fr Number of rows affected
    pub rows_affected: i64,
    /// fr fr Whether the operation was successful
    pub success: bool,
    /// fr fr Any error that occurred
    pub error: Option<DatabaseError>,
}

impl SlayResult {
    /// slay Create a new successful result
    pub fn new(last_insert_id: Option<i64>, rows_affected: i64) -> Self {
        Self {
            last_insert_id,
            rows_affected,
            success: true,
            error: None,
        }
    }

    /// slay Create a new error result
    pub fn with_error(error: DatabaseError) -> Self {
        Self {
            last_insert_id: None,
            rows_affected: 0,
            success: false,
            error: Some(error),
        }
    }

    /// slay Get last insert ID
    pub fn last_insert_id(&self) -> Result<i64, DatabaseError> {
        self.last_insert_id.ok_or_else(|| 
            DatabaseError::new(DatabaseErrorKind::NoLastInsertId, "No last insert ID available")
        )
    }

    /// slay Get rows affected
    pub fn rows_affected(&self) -> Result<i64, DatabaseError> {
        if let Some(ref err) = self.error {
            Err(err.clone())
        } else {
            Ok(self.rows_affected)
        }
    }

    /// slay Check if operation was successful
    pub fn success(&self) -> bool {
        self.success
    }

    /// slay Get error if any
    pub fn error(&self) -> Option<&DatabaseError> {
        self.error.as_ref()
    }
}

impl std::fmt::Display for SlayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            write!(f, "Success: {} rows affected", self.rows_affected)?;
            if let Some(id) = self.last_insert_id {
                write!(f, ", last insert ID: {}", id)?;
            }
            Ok(())
        } else {
            write!(f, "Error: {:?}", self.error)
        }
    }
}

/// fr fr Transaction options for controlling isolation and behavior
#[derive(Debug, Clone)]
pub struct TxOptions {
    /// fr fr Transaction isolation level
    pub isolation: SqlIsolationLevel,
    /// fr fr Whether this is a read-only transaction
    pub read_only: bool,
}

impl Default for TxOptions {
    fn default() -> Self {
        Self {
            isolation: SqlIsolationLevel::LevelDefault,
            read_only: false,
        }
    }
}

/// fr fr Database connection and operation statistics
#[derive(Debug, Clone, Default)]
pub struct DBStats {
    /// fr fr Maximum number of open connections
    pub max_open_connections: usize,
    /// fr fr Current number of open connections
    pub open_connections: usize,
    /// fr fr Number of connections currently in use
    pub in_use: usize,
    /// fr fr Number of idle connections
    pub idle: usize,
    /// fr fr Total number of times waited for a connection
    pub wait_count: i64,
    /// fr fr Total time spent waiting for connections
    pub wait_duration: Duration,
    /// fr fr Total connections closed due to max idle limit
    pub max_idle_closed: i64,
    /// fr fr Total connections closed due to max lifetime
    pub max_lifetime_closed: i64,
    /// fr fr Total number of queries executed
    pub total_queries: i64,
    /// fr fr Total number of exec calls
    pub total_exec_calls: i64,
    /// fr fr Total number of query calls
    pub total_query_calls: i64,
}

/// fr fr Helper function to convert SqlValue to JSON
fn sql_value_to_json(value: SqlValue) -> serde_json::Value {
    match value {
        SqlValue::Null => serde_json::Value::Null,
        SqlValue::Boolean(b) => serde_json::Value::Bool(b),
        SqlValue::Integer(i) => serde_json::Value::Number(serde_json::Number::from(i)),
        SqlValue::Float(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        SqlValue::String(s) => serde_json::Value::String(s),
        SqlValue::Bytes(b) => serde_json::Value::String(base64::engine::general_purpose::STANDARD.encode(b)),
        SqlValue::Timestamp(t) => serde_json::Value::String(format!("{:?}", t)),
        SqlValue::Json(j) => j,
    }
}
