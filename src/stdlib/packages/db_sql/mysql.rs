/// fr fr MySQL driver implementation - the popular choice periodt

use crate::runtime::value::Value;
// Placeholder imports disabled
    db_core::{
        ExecuteResult, TransactionIsolation
    // types::ParameterDirection  // Explicit import to resolve E0659
// };
use crate::error::CursedError;
// Placeholder imports disabled
    SqlTransactionIsolation, SqlConnectionInfo, SqlBatch, SqlTransaction
// };

// Placeholder imports disabled
    DatabaseResult as DbResult, DatabaseError, ErrorKind, ConnectionError, QueryError, TransactionError
// };
use async_trait::async_trait;
use mysql::{Pool, PooledConn, Conn, Row as MySqlRow, Value as MySqlValue, Transaction as MySqlTransaction, TxOpts};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// fr fr MySQL driver
#[derive(Debug)]
pub struct MySqlDriver {
/// fr fr MySQL connection
#[derive(Debug)]
pub struct MySqlConnection {
/// fr fr MySQL error
#[derive(Debug)]
pub struct MySqlError {
/// fr fr MySQL result set implementation
#[derive(Debug)]
pub struct MySqlResultSet {
//     rows: Vec<crate::stdlib::packages::db_core::Row>,
//     metadata: crate::stdlib::packages::db_core::ResultMetadata,
/// fr fr MySQL prepared statement implementation
#[derive(Debug)]
pub struct MySqlPreparedStatement {
/// fr fr MySQL transaction implementation
#[derive(Debug)]
pub struct MySqlTransactionImpl {
impl MySqlDriver {
    pub fn new() -> Self {
        Self {
        }
    }
impl MySqlConnection {
    /// slay Create a new MySQL connection using connection pool
    pub fn new(connection_string: &str) -> DbResult<Self> {
        let pool = Pool::new(connection_string)
            .map_err(|e| DatabaseError::connection(
                &format!("Failed to create MySQL connection pool: {}", e)
            ))?;

        Ok(Self {
        })
    /// slay Get a connection from the pool
    fn get_connection(&mut self) -> DbResult<&mut PooledConn> {
        if self.current_conn.is_none() {
            let conn = self.pool.get_conn()
                .map_err(|e| DatabaseError::connection(
                    &format!("Failed to get connection from pool: {}", e)
                ))?;
            self.current_conn = Some(conn);
        Ok(self.current_conn.as_mut().unwrap())
    /// slay Convert SqlValue to mysql::Value
    fn sql_value_to_mysql(value: &SqlValue) -> MySqlValue {
        match value {
            SqlValue::Date(d) => MySqlValue::Date(
                d.year() as u16, d.month() as u8, d.day() as u8, 0, 0, 0, 0
            SqlValue::Time(t) => MySqlValue::Time(
                false, 0, t.hour() as u8, t.minute() as u8, t.second() as u8, 0
            SqlValue::Timestamp(dt) => MySqlValue::Date(
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
            SqlValue::TimestampTz(dt) => MySqlValue::Date(
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
        }
    }

    /// slay Convert mysql::Value to SqlValue
    fn mysql_value_to_sql(value: MySqlValue) -> SqlValue {
        match value {
            MySqlValue::Bytes(data) => {
                if let Ok(s) = String::from_utf8(data.clone()) {
                    SqlValue::Text(s)
                } else {
                    SqlValue::Binary(data)
                }
            MySqlValue::Date(year, month, day, hour, minute, second, _) => {
                // Create a timestamp if time components are present
                if hour != 0 || minute != 0 || second != 0 {
                    if let Ok(dt) = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                        .and_then(|d| d.and_hms_opt(hour as u32, minute as u32, second as u32)) {
                        SqlValue::Timestamp(dt)
                    } else {
                        SqlValue::Null
                    }
                } else {
                    if let Ok(dt) = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32) {
                        SqlValue::Date(dt)
                    } else {
                        SqlValue::Null
                    }
                }
            MySqlValue::Time(_, _, hour, minute, second, _) => {
                if let Ok(t) = chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32) {
                    SqlValue::Time(t)
                } else {
                    SqlValue::Null
                }
        }
    }

    /// slay Convert parameters to MySQL format
    fn convert_parameters(parameters: &[Parameter]) -> Vec<MySqlValue> {
        parameters.iter()
            .filter_map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Some(Self::convert_parameter_value(&p.value))
                }
            })
            .collect()
    /// slay Convert Parameter::Value to MySqlValue 
    fn convert_parameter_value(value: &crate::runtime::Value) -> MySqlValue {
        match value {
            crate::runtime::Value::Date(d) => MySqlValue::Date(
                d.year() as u16, d.month() as u8, d.day() as u8, 0, 0, 0, 0
            crate::runtime::Value::Time(t) => MySqlValue::Time(
                false, 0, t.hour() as u8, t.minute() as u8, t.second() as u8, 0
            crate::runtime::Value::DateTime(dt) => MySqlValue::Date(
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
        }
    }

    /// slay Process MySQL rows into CURSED rows
//     fn process_mysql_rows(mysql_rows: Vec<MySqlRow>) -> DbResult<(Vec<crate::stdlib::packages::db_core::Row>, Vec<crate::stdlib::packages::db_core::Column>)> {
        let mut result_rows = Vec::new();
        let mut columns = Vec::new();

        for (row_idx, mysql_row) in mysql_rows.iter().enumerate() {
            if row_idx == 0 {
                // Build column metadata from first row
                columns = mysql_row.columns_ref().iter().enumerate().map(|(i, col)| {
                    let column_type = match col.column_type() {
                        mysql::consts::ColumnType::MYSQL_TYPE_TINY |
                        mysql::consts::ColumnType::MYSQL_TYPE_SHORT |
                        mysql::consts::ColumnType::MYSQL_TYPE_LONG |
//                         mysql::consts::ColumnType::MYSQL_TYPE_LONGLONG => crate::stdlib::packages::db_core::ColumnType::BigInt,
                        mysql::consts::ColumnType::MYSQL_TYPE_FLOAT |
//                         mysql::consts::ColumnType::MYSQL_TYPE_DOUBLE => crate::stdlib::packages::db_core::ColumnType::Double,
                        mysql::consts::ColumnType::MYSQL_TYPE_STRING |
                        mysql::consts::ColumnType::MYSQL_TYPE_VAR_STRING |
//                         mysql::consts::ColumnType::MYSQL_TYPE_VARCHAR => crate::stdlib::packages::db_core::ColumnType::Text,
                        mysql::consts::ColumnType::MYSQL_TYPE_BLOB |
                        mysql::consts::ColumnType::MYSQL_TYPE_MEDIUM_BLOB |
//                         mysql::consts::ColumnType::MYSQL_TYPE_LONG_BLOB => crate::stdlib::packages::db_core::ColumnType::Blob,
//                         mysql::consts::ColumnType::MYSQL_TYPE_DATE => crate::stdlib::packages::db_core::ColumnType::Date,
//                         mysql::consts::ColumnType::MYSQL_TYPE_TIME => crate::stdlib::packages::db_core::ColumnType::Time,
                        mysql::consts::ColumnType::MYSQL_TYPE_DATETIME |
//                         mysql::consts::ColumnType::MYSQL_TYPE_TIMESTAMP => crate::stdlib::packages::db_core::ColumnType::Timestamp,
//                         mysql::consts::ColumnType::MYSQL_TYPE_JSON => crate::stdlib::packages::db_core::ColumnType::Json,
//                         _ => crate::stdlib::packages::db_core::ColumnType::Text,

//                     crate::stdlib::packages::db_core::Column {
                    }
                }).collect();
            // Process row values
            let mut values = Vec::new();
            for i in 0..mysql_row.len() {
                let mysql_value: MySqlValue = mysql_row.get(i).unwrap_or(MySqlValue::NULL);
                let sql_value = Self::mysql_value_to_sql(mysql_value.clone());
                
//                 let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &mysql_value {
                    column_type: if i < columns.len() { 
                        columns[i].column_type.clone() 
                    } else { 
//                         crate::stdlib::packages::db_core::ColumnType::Text 
                values.push(column_value);
//             result_rows.push(crate::stdlib::packages::db_core::Row {
//                 metadata: crate::stdlib::packages::db_core::RowMetadata {
            });
        Ok((result_rows, columns))
    }
}

#[async_trait]
// impl crate::stdlib::packages::db_core::DatabaseDriver for MySqlDriver {
    async fn connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>> {
        let connection_string = if config.connection_string.starts_with("mysql://") {
            config.connection_string
        } else {
            // Build MySQL URL from components
            format!(
                "mysql://{}:{}@{}:{}/{}",
                config.database.unwrap_or_else(|| "mysql".to_string())
            )

        let conn = MySqlConnection::new(&connection_string)?;
        Ok(Box::new(conn))
//     fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
//         crate::stdlib::packages::db_core::DriverInfo::new(
            "CURSED"
        )
    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    fn sql_dialect(&self) -> SqlDialect {
        SqlDialect::MySQL
    fn validate_connection_string(&self, _connection_string: &str) -> DbResult<()> {
        Ok(())
    }
}

#[async_trait]
impl SqlDriver for MySqlDriver {
    async fn sql_connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        let connection_string = if config.connection_string.starts_with("mysql://") {
            config.connection_string
        } else {
            // Build MySQL URL from components
            format!(
                "mysql://{}:{}@{}:{}/{}",
                config.database.unwrap_or_else(|| "mysql".to_string())
            )

        let conn = MySqlConnection::new(&connection_string)?;
        Ok(Box::new(conn))
    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
//         Box::new(crate::stdlib::packages::db_sql::MySqlDialect::new())
//     fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
//             crate::stdlib::packages::db_sql::SqlType::Integer,
//             crate::stdlib::packages::db_sql::SqlType::Text,
//             crate::stdlib::packages::db_sql::SqlType::Boolean,
//             crate::stdlib::packages::db_sql::SqlType::Json,
        ]
//     fn supports_sql_feature(&self, _feature: crate::stdlib::packages::db_sql::SqlFeature) -> bool {
        true
    fn configuration_options(&self) -> Vec<ConfigurationOption> {
        Vec::from([])
    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    fn performance_info(&self) -> DriverPerformanceInfo {
        DriverPerformanceInfo {
        }
    }

    fn limitations(&self) -> DriverLimitations {
        DriverLimitations {
        }
    }
#[async_trait]
impl DatabaseConnection for MySqlConnection {
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.get_connection()?;
        let mysql_params = Self::convert_parameters(parameters);
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL query execution failed: {}", e)
            ))?;

        let (result_rows, columns) = Self::process_mysql_rows(mysql_rows)?;

        let result_set = MySqlResultSet {
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {

        Ok(Box::new(result_set))
    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult> {
        let conn = self.get_connection()?;
        let mysql_params = Self::convert_parameters(parameters);
        
        let result = conn.exec_drop(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL execute failed: {}", e)
            ))?;

        Ok(ExecuteResult {
        })
    async fn prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = MySqlPreparedStatement {

        Ok(Box::new(stmt))
//     async fn begin_transaction(&mut self, _options: Option<crate::stdlib::packages::db_core::TransactionOptions>) -> DbResult<Box<dyn DatabaseTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                "Transaction already active"
            ));
        let conn = self.get_connection()?;
        let tx = conn.start_transaction(TxOpts::default())
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        // Note: This is unsafe but necessary due to MySQL crate design
        let static_tx = unsafe { std::mem::transmute(tx) };
        
        let transaction = MySqlTransactionImpl {

        Ok(Box::new(transaction))
    async fn ping(&mut self) -> DbResult<()> {
        let conn = self.get_connection()?;
        conn.ping().map_err(|e| DatabaseError::connection(
            &format!("MySQL ping failed: {}", e)
        ))
    async fn close(self: Box<Self>) -> DbResult<()> {
        // MySQL connection pools handle cleanup automatically
        Ok(())
//     fn connection_info(&self) -> crate::stdlib::packages::db_core::traits::ConnectionInfo {
//         crate::stdlib::packages::db_core::traits::ConnectionInfo {
//             transaction_isolation: crate::stdlib::packages::db_core::traits::TransactionIsolation::RepeatableRead,
        }
    }
#[async_trait]
impl SqlConnection for MySqlConnection {
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet> {
        let conn = self.get_connection()?;
        let mysql_params: Vec<MySqlValue> = params.iter().map(Self::sql_value_to_mysql).collect();
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL sql_query execution failed: {}", e)
            ))?;

        let (result_rows, columns) = Self::process_mysql_rows(mysql_rows)?;

        Ok(SqlResultSet {
            rows: result_rows.into_iter().map(|row| {
//                 crate::stdlib::packages::db_sql::SqlRow {
                    values: row.values.into_iter().map(|col| {
                        match &col.data {
                            Some(data) => {
                                match col.column_type {
//                                     crate::stdlib::packages::db_core::ColumnType::BigInt => {
                                        if data.len() >= 8 {
                                            SqlValue::Integer(i64::from_le_bytes([
                                                data[4], data[5], data[6], data[7]
                                            ]))
                                        } else {
                                            SqlValue::Null
                                        }
//                                     crate::stdlib::packages::db_core::ColumnType::Double => {
                                        if data.len() >= 8 {
                                            SqlValue::Double(f64::from_le_bytes([
                                                data[4], data[5], data[6], data[7]
                                            ]))
                                        } else {
                                            SqlValue::Null
                                        }
//                                     crate::stdlib::packages::db_core::ColumnType::Text => {
                                        if let Ok(s) = String::from_utf8(data.clone()) {
                                            SqlValue::Text(s)
                                        } else {
                                            SqlValue::Binary(data.clone())
                                        }
                                }
                        }
//                     metadata: crate::stdlib::packages::db_sql::SqlRowMetadata {
                }
            columns: columns.into_iter().map(|col| {
//                 crate::stdlib::packages::db_sql::SqlColumn {
                    sql_type: match col.column_type {
//                         crate::stdlib::packages::db_core::ColumnType::BigInt => crate::stdlib::packages::db_sql::SqlType::Integer,
//                         crate::stdlib::packages::db_core::ColumnType::Double => crate::stdlib::packages::db_sql::SqlType::Real,
//                         crate::stdlib::packages::db_core::ColumnType::Text => crate::stdlib::packages::db_sql::SqlType::Text,
//                         crate::stdlib::packages::db_core::ColumnType::Blob => crate::stdlib::packages::db_sql::SqlType::Blob,
//                         crate::stdlib::packages::db_core::ColumnType::Json => crate::stdlib::packages::db_sql::SqlType::Json,
//                         _ => crate::stdlib::packages::db_sql::SqlType::Text,
                }
//             metadata: crate::stdlib::packages::db_sql::SqlResultMetadata {
        })
    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult> {
        let conn = self.get_connection()?;
        let mysql_params: Vec<MySqlValue> = params.iter().map(Self::sql_value_to_mysql).collect();
        
        conn.exec_drop(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL sql_execute failed: {}", e)
            ))?;

        Ok(SqlExecuteResult {
        })
    async fn sql_prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = MySqlPreparedStatement {

        Ok(Box::new(stmt))
    async fn sql_begin_transaction(&mut self, _isolation: Option<SqlTransactionIsolation>) -> DbResult<Box<dyn SqlTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                "Transaction already active"
            ));
        let conn = self.get_connection()?;
        let tx = conn.start_transaction(TxOpts::default())
            .map_err(|e| DatabaseError::transaction(
                &format!("Failed to begin SQL transaction: {}", e)
            ))?;

        self.in_transaction = true;

        // Note: This is unsafe but necessary due to MySQL crate design
        let static_tx = unsafe { std::mem::transmute(tx) };
        
        let transaction = MySqlTransactionImpl {

        Ok(Box::new(transaction))
    async fn sql_batch(&mut self, statements: &[SqlBatch]) -> DbResult<Vec<SqlExecuteResult>> {
        let mut results = Vec::new();
        
        for batch in statements {
            let result = self.sql_execute(&batch.sql, &batch.parameters).await?;
            results.push(result);
        Ok(results)
    fn sql_connection_info(&self) -> SqlConnectionInfo {
        SqlConnectionInfo {
        }
    }

    async fn set_sql_variable(&mut self, name: &str, value: &SqlValue) -> DbResult<()> {
        let conn = self.get_connection()?;
        let mysql_value = Self::sql_value_to_mysql(value);
        
        let sql = format!("SET @{} = ?", name);
        conn.exec_drop(sql, (mysql_value,))
            .map_err(|e| DatabaseError::query(
                &format!("Failed to set MySQL variable: {}", e)
            ))
    async fn get_sql_variable(&mut self, name: &str) -> DbResult<SqlValue> {
        let conn = self.get_connection()?;
        
        let sql = format!("SELECT @{}", name);
        let result: Option<MySqlValue> = conn.exec_first(sql, ())
            .map_err(|e| DatabaseError::query(
                &format!("Failed to get MySQL variable: {}", e)
            ))?;

        match result {
        }
    }
// Implement ResultSet trait for MySqlResultSet
impl ResultSet for MySqlResultSet {
//     fn next(&mut self) -> DbResult<Option<crate::stdlib::packages::db_core::Row>> {
        if self.current_index < self.rows.len() {
            let row = self.rows[self.current_index].clone();
            self.current_index += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

//     fn collect(&mut self) -> DbResult<Vec<crate::stdlib::packages::db_core::Row>> {
        let mut result = Vec::new();
        while let Some(row) = self.next()? {
            result.push(row);
        }
        Ok(result)
//     fn columns(&self) -> &[crate::stdlib::packages::db_core::Column] {
        &self.metadata.columns
    fn has_next(&self) -> bool {
        self.current_index < self.rows.len()
//     fn metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        &self.metadata
    fn row_count(&self) -> Option<usize> {
        Some(self.rows.len())
    fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

// Implement PreparedStatement trait for MySqlPreparedStatement
#[async_trait]
impl PreparedStatement for MySqlPreparedStatement {
//     async fn execute(&mut self, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        let mut conn = self.pool.get_conn()
            .map_err(|e| DatabaseError::connection(
                &format!("Failed to get connection from pool: {}", e)
            ))?;

        let mysql_params = MySqlConnection::convert_parameters(parameters);
        
        conn.exec_drop(&self.sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL prepared statement execution failed: {}", e)
            ))?;

//         Ok(crate::stdlib::packages::db_core::ExecuteResult {
        })
    async fn query(&mut self, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let mut conn = self.pool.get_conn()
            .map_err(|e| DatabaseError::connection(
                &format!("Failed to get connection from pool: {}", e)
            ))?;

        let mysql_params = MySqlConnection::convert_parameters(parameters);
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(&self.sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                &format!("MySQL prepared query execution failed: {}", e)
            ))?;

        let (result_rows, columns) = MySqlConnection::process_mysql_rows(mysql_rows)?;

        let result_set = MySqlResultSet {
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {

        Ok(Box::new(result_set))
    fn parameter_count(&self) -> usize {
        // MySQL doesn't expose parameter count directly, we'll estimate
        self.sql.matches('?').count()
    fn sql(&self) -> &str {
        &self.sql
    /// slay Get parameter metadata
//     fn parameter_metadata(&self) -> &[crate::stdlib::packages::db_core::ParameterMetadata] {
        // Placeholder implementation - would need to extract from MySQL statement
        &[]
    /// slay Get result set metadata
//     fn result_metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        // Placeholder implementation - would need to extract from MySQL statement
//         static EMPTY_METADATA: std::sync::LazyLock<crate::stdlib::packages::db_core::ResultMetadata> = 
//             std::sync::LazyLock::new(|| crate::stdlib::packages::db_core::ResultMetadata {
//                 result_type: crate::stdlib::packages::db_core::result::ResultType::ForwardOnly,
            });
        &EMPTY_METADATA
    /// slay Close the prepared statement
    async fn close(self: Box<Self>) -> DbResult<()> {
        // MySQL prepared statements are automatically cleaned up
        Ok(())
    }
}

// Implement DatabaseTransaction trait for MySqlTransactionImpl
#[async_trait]
impl DatabaseTransaction for MySqlTransactionImpl {
    async fn commit(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                "Transaction already completed"
            ));
        if let Some(tx) = self.connection.take() {
            tx.commit().map_err(|e| DatabaseError::transaction(
                &format!("Failed to commit MySQL transaction: {}", e)
            ))?;
        self.active = false;
        Ok(())
    async fn rollback(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                "Transaction already completed"
            ));
        if let Some(tx) = self.connection.take() {
            tx.rollback().map_err(|e| DatabaseError::transaction(
                &format!("Failed to rollback MySQL transaction: {}", e)
            ))?;
        self.active = false;
        Ok(())
//     async fn savepoint(&mut self, name: &str) -> DbResult<crate::stdlib::packages::db_core::SavePoint> {
        if let Some(ref mut tx) = self.connection {
            tx.exec_drop(&format!("SAVEPOINT {}", name), ())
                .map_err(|e| DatabaseError::transaction(
                    &format!("Failed to create MySQL savepoint: {}", e)
                ))?;
//         Ok(crate::stdlib::packages::db_core::SavePoint {
        })
//     async fn rollback_to_savepoint(&mut self, savepoint: &crate::stdlib::packages::db_core::SavePoint) -> DbResult<()> {
        if let Some(ref mut tx) = self.connection {
            tx.exec_drop(&format!("ROLLBACK TO SAVEPOINT {}", savepoint.name), ())
                .map_err(|e| DatabaseError::transaction(
                    &format!("Failed to rollback to MySQL savepoint: {}", e)
                ))?;
        Ok(())
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        if let Some(ref mut tx) = self.connection {
            let mysql_params = MySqlConnection::convert_parameters(parameters);
            
            let mysql_rows: Vec<MySqlRow> = tx.exec(sql, mysql_params)
                .map_err(|e| DatabaseError::query(
                    &format!("MySQL transaction query failed: {}", e)
                ))?;

            let (result_rows, columns) = MySqlConnection::process_mysql_rows(mysql_rows)?;

            let result_set = MySqlResultSet {
//                 metadata: crate::stdlib::packages::db_core::ResultMetadata {

            Ok(Box::new(result_set))
        } else {
            Err(DatabaseError::transaction(
                "Transaction not active"
            ))
        }
    }

//     async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        if let Some(ref mut tx) = self.connection {
            let mysql_params = MySqlConnection::convert_parameters(parameters);
            
            tx.exec_drop(sql, mysql_params)
                .map_err(|e| DatabaseError::query(
                    &format!("MySQL transaction execute failed: {}", e)
                ))?;

//             Ok(crate::stdlib::packages::db_core::ExecuteResult {
            })
        } else {
            Err(DatabaseError::transaction(
                "Transaction not active"
            ))
        }
    }

//     fn state(&self) -> crate::stdlib::packages::db_core::traits::TransactionState {
        if self.active {
//             crate::stdlib::packages::db_core::traits::TransactionState::Active
        } else {
//             crate::stdlib::packages::db_core::traits::TransactionState::Committed
        }
    }
// impl std::fmt::Display for MySqlError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "MySQL CursedError: {}", self.message)
//     }
// }

// impl std::error::CursedError for MySqlError {}
// 