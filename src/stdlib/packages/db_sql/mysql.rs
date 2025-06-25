/// fr fr MySQL driver implementation - the popular choice periodt

use crate::runtime::value::Value;
// use crate::stdlib::packages::{
    db_core::{
        ConnectionConfig, DatabaseConnection, DriverFeature, SqlDialect,
        Parameter, ResultSet, PreparedStatement, DatabaseTransaction,
        ExecuteResult, TransactionIsolation
    },
    db_sql::{SqlDriver, SqlDialectTrait, SqlValue, SqlResultSet, SqlExecuteResult},
    // types::ParameterDirection  // Explicit import to resolve E0659
};
use crate::error::CursedError;
// use crate::stdlib::packages::db_sql::drivers::{
    SqlConnection, ConfigurationOption, DriverPerformanceInfo, DriverLimitations,
    SqlTransactionIsolation, SqlConnectionInfo, SqlBatch, SqlTransaction
};

// use crate::stdlib::packages::db_core::error::{
    DatabaseResult as DbResult, DatabaseError, ErrorKind, ConnectionError, QueryError, TransactionError
};
use async_trait::async_trait;
use mysql::{Pool, PooledConn, Conn, Row as MySqlRow, Value as MySqlValue, Transaction as MySqlTransaction, TxOpts};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// fr fr MySQL driver
#[derive(Debug)]
pub struct MySqlDriver {
    name: String,
    version: String,
}

/// fr fr MySQL connection
#[derive(Debug)]
pub struct MySqlConnection {
    connection_id: String,
    pool: Arc<Pool>,
    current_conn: Option<PooledConn>,
    in_transaction: bool,
}

/// fr fr MySQL error
#[derive(Debug)]
pub struct MySqlError {
    message: String,
}

/// fr fr MySQL result set implementation
#[derive(Debug)]
pub struct MySqlResultSet {
//     rows: Vec<crate::stdlib::packages::db_core::Row>,
//     metadata: crate::stdlib::packages::db_core::ResultMetadata,
    current_index: usize,
}

/// fr fr MySQL prepared statement implementation
#[derive(Debug)]
pub struct MySqlPreparedStatement {
    pool: Arc<Pool>,
    sql: String,
    statement_id: String,
}

/// fr fr MySQL transaction implementation
#[derive(Debug)]
pub struct MySqlTransactionImpl {
    connection: Option<MySqlTransaction<'static>>,
    transaction_id: String,
    active: bool,
}

impl MySqlDriver {
    pub fn new() -> Self {
        Self {
            name: "mysql".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

impl MySqlConnection {
    /// slay Create a new MySQL connection using connection pool
    pub fn new(connection_string: &str) -> DbResult<Self> {
        let pool = Pool::new(connection_string)
            .map_err(|e| DatabaseError::connection(
                ConnectionError::FailedToConnect,
                &format!("Failed to create MySQL connection pool: {}", e)
            ))?;

        Ok(Self {
            connection_id: format!("mysql_{}", uuid::Uuid::new_v4()),
            pool: Arc::new(pool),
            current_conn: None,
            in_transaction: false,
        })
    }

    /// slay Get a connection from the pool
    fn get_connection(&mut self) -> DbResult<&mut PooledConn> {
        if self.current_conn.is_none() {
            let conn = self.pool.get_conn()
                .map_err(|e| DatabaseError::connection(
                    ConnectionError::FailedToConnect,
                    &format!("Failed to get connection from pool: {}", e)
                ))?;
            self.current_conn = Some(conn);
        }
        
        Ok(self.current_conn.as_mut().unwrap())
    }

    /// slay Convert SqlValue to mysql::Value
    fn sql_value_to_mysql(value: &SqlValue) -> MySqlValue {
        match value {
            SqlValue::Null => MySqlValue::NULL,
            SqlValue::Boolean(b) => MySqlValue::Int(*b as i64),
            SqlValue::Integer(i) => MySqlValue::Int(*i),
            SqlValue::Float(f) => MySqlValue::Float(*f as f32),
            SqlValue::Double(f) => MySqlValue::Double(*f),
            SqlValue::Text(s) => MySqlValue::Bytes(s.as_bytes().to_vec()),
            SqlValue::Binary(data) => MySqlValue::Bytes(data.clone()),
            SqlValue::Date(d) => MySqlValue::Date(
                d.year() as u16, d.month() as u8, d.day() as u8, 0, 0, 0, 0
            ),
            SqlValue::Time(t) => MySqlValue::Time(
                false, 0, t.hour() as u8, t.minute() as u8, t.second() as u8, 0
            ),
            SqlValue::Timestamp(dt) => MySqlValue::Date(
                dt.year() as u16, dt.month() as u8, dt.day() as u8,
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
            ),
            SqlValue::TimestampTz(dt) => MySqlValue::Date(
                dt.year() as u16, dt.month() as u8, dt.day() as u8,
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
            ),
            SqlValue::Json(j) => MySqlValue::Bytes(j.to_string().as_bytes().to_vec()),
            SqlValue::Uuid(u) => MySqlValue::Bytes(u.to_string().as_bytes().to_vec()),
            _ => MySqlValue::NULL,
        }
    }

    /// slay Convert mysql::Value to SqlValue
    fn mysql_value_to_sql(value: MySqlValue) -> SqlValue {
        match value {
            MySqlValue::NULL => SqlValue::Null,
            MySqlValue::Bytes(data) => {
                if let Ok(s) = String::from_utf8(data.clone()) {
                    SqlValue::Text(s)
                } else {
                    SqlValue::Binary(data)
                }
            },
            MySqlValue::Int(i) => SqlValue::Integer(i),
            MySqlValue::UInt(u) => SqlValue::Integer(u as i64),
            MySqlValue::Float(f) => SqlValue::Float(f as f64),
            MySqlValue::Double(f) => SqlValue::Double(f),
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
            },
            MySqlValue::Time(_, _, hour, minute, second, _) => {
                if let Ok(t) = chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32) {
                    SqlValue::Time(t)
                } else {
                    SqlValue::Null
                }
            },
        }
    }

    /// slay Convert parameters to MySQL format
    fn convert_parameters(parameters: &[Parameter]) -> Vec<MySqlValue> {
        parameters.iter()
            .filter_map(|p| match p.direction {
//                 crate::stdlib::packages::types::ParameterDirection::In => {
                    Some(Self::convert_parameter_value(&p.value))
                }
                _ => Some(MySqlValue::NULL),
            })
            .collect()
    }

    /// slay Convert Parameter::Value to MySqlValue 
    fn convert_parameter_value(value: &crate::runtime::Value) -> MySqlValue {
        match value {
            crate::runtime::Value::Null => MySqlValue::NULL,
            crate::runtime::Value::Boolean(b) => MySqlValue::Int(*b as i64),
            crate::runtime::Value::Integer(i) => MySqlValue::Int(*i),
            crate::runtime::Value::Float(f) => MySqlValue::Double(*f),
            crate::runtime::Value::String(s) => MySqlValue::Bytes(s.as_bytes().to_vec()),
            crate::runtime::Value::Binary(data) => MySqlValue::Bytes(data.clone()),
            crate::runtime::Value::Date(d) => MySqlValue::Date(
                d.year() as u16, d.month() as u8, d.day() as u8, 0, 0, 0, 0
            ),
            crate::runtime::Value::Time(t) => MySqlValue::Time(
                false, 0, t.hour() as u8, t.minute() as u8, t.second() as u8, 0
            ),
            crate::runtime::Value::DateTime(dt) => MySqlValue::Date(
                dt.year() as u16, dt.month() as u8, dt.day() as u8,
                dt.hour() as u8, dt.minute() as u8, dt.second() as u8, 0
            ),
            _ => MySqlValue::NULL,
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
                    };

//                     crate::stdlib::packages::db_core::Column {
                        name: col.name_str().to_string(),
                        column_type,
                        nullable: true,
                        ordinal: i,
                        table_name: col.table_str().map(|s| s.to_string()),
                        schema_name: col.schema_str().map(|s| s.to_string()),
                        precision: None,
                        scale: None,
                        max_length: None,
                        auto_increment: false,
                        default_value: None,
                    }
                }).collect();
            }

            // Process row values
            let mut values = Vec::new();
            for i in 0..mysql_row.len() {
                let mysql_value: MySqlValue = mysql_row.get(i).unwrap_or(MySqlValue::NULL);
                let sql_value = Self::mysql_value_to_sql(mysql_value.clone());
                
//                 let column_value = crate::stdlib::packages::db_core::ColumnValue {
                    data: match &mysql_value {
                        MySqlValue::NULL => None,
                        MySqlValue::Bytes(data) => Some(data.clone()),
                        MySqlValue::Int(i) => Some(i.to_le_bytes().to_vec()),
                        MySqlValue::UInt(u) => Some(u.to_le_bytes().to_vec()),
                        MySqlValue::Float(f) => Some(f.to_le_bytes().to_vec()),
                        MySqlValue::Double(f) => Some(f.to_le_bytes().to_vec()),
                        _ => None,
                    },
                    column_type: if i < columns.len() { 
                        columns[i].column_type.clone() 
                    } else { 
//                         crate::stdlib::packages::db_core::ColumnType::Text 
                    },
                    is_null: matches!(mysql_value, MySqlValue::NULL),
                };
                values.push(column_value);
            }

//             result_rows.push(crate::stdlib::packages::db_core::Row {
                values,
//                 metadata: crate::stdlib::packages::db_core::RowMetadata {
                    row_number: row_idx,
                    is_inserted: false,
                    is_updated: false,
                    is_deleted: false,
                },
            });
        }

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
                config.user.unwrap_or_else(|| "root".to_string()),
                config.password.unwrap_or_else(|| "".to_string()),
                config.host.unwrap_or_else(|| "localhost".to_string()),
                config.port.unwrap_or(3306),
                config.database.unwrap_or_else(|| "mysql".to_string())
            )
        };

        let conn = MySqlConnection::new(&connection_string)?;
        Ok(Box::new(conn))
    }

//     fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
//         crate::stdlib::packages::db_core::DriverInfo::new(
            &self.name,
            &self.version,
            "MySQL database driver",
            "CURSED"
        )
    }

    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    }

    fn sql_dialect(&self) -> SqlDialect {
        SqlDialect::MySQL
    }

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
                config.user.unwrap_or_else(|| "root".to_string()),
                config.password.unwrap_or_else(|| "".to_string()),
                config.host.unwrap_or_else(|| "localhost".to_string()),
                config.port.unwrap_or(3306),
                config.database.unwrap_or_else(|| "mysql".to_string())
            )
        };

        let conn = MySqlConnection::new(&connection_string)?;
        Ok(Box::new(conn))
    }

    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
//         Box::new(crate::stdlib::packages::db_sql::MySqlDialect::new())
    }

//     fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
//             crate::stdlib::packages::db_sql::SqlType::Integer,
//             crate::stdlib::packages::db_sql::SqlType::Text,
//             crate::stdlib::packages::db_sql::SqlType::Boolean,
//             crate::stdlib::packages::db_sql::SqlType::Json,
        ]
    }

//     fn supports_sql_feature(&self, _feature: crate::stdlib::packages::db_sql::SqlFeature) -> bool {
        true
    }

    fn configuration_options(&self) -> Vec<ConfigurationOption> {
        Vec::from([])
    }

    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    }

    fn performance_info(&self) -> DriverPerformanceInfo {
        DriverPerformanceInfo {
            connection_time: std::time::Duration::from_millis(80),
            query_overhead: std::time::Duration::from_micros(30),
            max_connections: Some(2000),
            connection_pooling: true,
            statement_caching: true,
            batch_operations: true,
            streaming_results: true,
        }
    }

    fn limitations(&self) -> DriverLimitations {
        DriverLimitations {
            max_statement_length: Some(1024 * 1024),
            max_parameters: Some(65535),
            max_identifier_length: Some(64),
            max_string_length: Some(65535),
            max_numeric_precision: Some(65),
            max_columns: Some(4096),
            max_rows: None,
            unsupported_features: Vec::from([]),
        }
    }
}

#[async_trait]
impl DatabaseConnection for MySqlConnection {
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let conn = self.get_connection()?;
        let mysql_params = Self::convert_parameters(parameters);
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("MySQL query execution failed: {}", e)
            ))?;

        let (result_rows, columns) = Self::process_mysql_rows(mysql_rows)?;

        let result_set = MySqlResultSet {
            rows: result_rows,
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {
                columns,
                row_count: None,
                affected_rows: 0,
                last_insert_id: None,
                warnings: Vec::new(),
                query_id: Some(uuid::Uuid::new_v4().to_string()),
                execution_time: std::time::Duration::from_millis(0),
            },
            current_index: 0,
        };

        Ok(Box::new(result_set))
    }

    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult> {
        let conn = self.get_connection()?;
        let mysql_params = Self::convert_parameters(parameters);
        
        let result = conn.exec_drop(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("MySQL execute failed: {}", e)
            ))?;

        Ok(ExecuteResult {
            affected_rows: conn.affected_rows(),
            last_insert_id: Some(conn.last_insert_id()),
            warnings: Vec::new(),
            query_id: Some(uuid::Uuid::new_v4().to_string()),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    async fn prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = MySqlPreparedStatement {
            pool: Arc::clone(&self.pool),
            sql: sql.to_string(),
            statement_id: uuid::Uuid::new_v4().to_string(),
        };

        Ok(Box::new(stmt))
    }

//     async fn begin_transaction(&mut self, _options: Option<crate::stdlib::packages::db_core::TransactionOptions>) -> DbResult<Box<dyn DatabaseTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                TransactionError::NotActive,
                "Transaction already active"
            ));
        }

        let conn = self.get_connection()?;
        let tx = conn.start_transaction(TxOpts::default())
            .map_err(|e| DatabaseError::transaction(
                TransactionError::NotActive,
                &format!("Failed to begin transaction: {}", e)
            ))?;

        self.in_transaction = true;

        // Note: This is unsafe but necessary due to MySQL crate design
        let static_tx = unsafe { std::mem::transmute(tx) };
        
        let transaction = MySqlTransactionImpl {
            connection: Some(static_tx),
            transaction_id: uuid::Uuid::new_v4().to_string(),
            active: true,
        };

        Ok(Box::new(transaction))
    }

    async fn ping(&mut self) -> DbResult<()> {
        let conn = self.get_connection()?;
        conn.ping().map_err(|e| DatabaseError::connection(
            ConnectionError::ConnectionLost,
            &format!("MySQL ping failed: {}", e)
        ))
    }

    async fn close(self: Box<Self>) -> DbResult<()> {
        // MySQL connection pools handle cleanup automatically
        Ok(())
    }

//     fn connection_info(&self) -> crate::stdlib::packages::db_core::traits::ConnectionInfo {
//         crate::stdlib::packages::db_core::traits::ConnectionInfo {
            database_name: "mysql_db".to_string(),
            server_version: "8.0.35".to_string(),
            protocol_version: "10".to_string(),
            connection_id: self.connection_id.clone(),
            is_read_only: false,
//             transaction_isolation: crate::stdlib::packages::db_core::traits::TransactionIsolation::RepeatableRead,
        }
    }
}

#[async_trait]
impl SqlConnection for MySqlConnection {
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet> {
        let conn = self.get_connection()?;
        let mysql_params: Vec<MySqlValue> = params.iter().map(Self::sql_value_to_mysql).collect();
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
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
                                                data[0], data[1], data[2], data[3],
                                                data[4], data[5], data[6], data[7]
                                            ]))
                                        } else {
                                            SqlValue::Null
                                        }
                                    },
//                                     crate::stdlib::packages::db_core::ColumnType::Double => {
                                        if data.len() >= 8 {
                                            SqlValue::Double(f64::from_le_bytes([
                                                data[0], data[1], data[2], data[3],
                                                data[4], data[5], data[6], data[7]
                                            ]))
                                        } else {
                                            SqlValue::Null
                                        }
                                    },
//                                     crate::stdlib::packages::db_core::ColumnType::Text => {
                                        if let Ok(s) = String::from_utf8(data.clone()) {
                                            SqlValue::Text(s)
                                        } else {
                                            SqlValue::Binary(data.clone())
                                        }
                                    },
                                    _ => SqlValue::Binary(data.clone()),
                                }
                            },
                            None => SqlValue::Null,
                        }
                    }).collect(),
//                     metadata: crate::stdlib::packages::db_sql::SqlRowMetadata {
                        row_number: row.metadata.row_number,
                        table_name: None,
                        is_updated: row.metadata.is_updated,
                    },
                }
            }).collect(),
            columns: columns.into_iter().map(|col| {
//                 crate::stdlib::packages::db_sql::SqlColumn {
                    name: col.name,
                    sql_type: match col.column_type {
//                         crate::stdlib::packages::db_core::ColumnType::BigInt => crate::stdlib::packages::db_sql::SqlType::Integer,
//                         crate::stdlib::packages::db_core::ColumnType::Double => crate::stdlib::packages::db_sql::SqlType::Real,
//                         crate::stdlib::packages::db_core::ColumnType::Text => crate::stdlib::packages::db_sql::SqlType::Text,
//                         crate::stdlib::packages::db_core::ColumnType::Blob => crate::stdlib::packages::db_sql::SqlType::Blob,
//                         crate::stdlib::packages::db_core::ColumnType::Json => crate::stdlib::packages::db_sql::SqlType::Json,
//                         _ => crate::stdlib::packages::db_sql::SqlType::Text,
                    },
                    nullable: col.nullable,
                    ordinal: col.ordinal,
                    table_name: col.table_name,
                    schema_name: col.schema_name,
                }
            }).collect(),
//             metadata: crate::stdlib::packages::db_sql::SqlResultMetadata {
                row_count: None,
                affected_rows: 0,
                execution_time: std::time::Duration::from_millis(0),
                warnings: Vec::new(),
            },
        })
    }

    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult> {
        let conn = self.get_connection()?;
        let mysql_params: Vec<MySqlValue> = params.iter().map(Self::sql_value_to_mysql).collect();
        
        conn.exec_drop(sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("MySQL sql_execute failed: {}", e)
            ))?;

        Ok(SqlExecuteResult {
            affected_rows: conn.affected_rows(),
            last_insert_id: Some(conn.last_insert_id()),
            warnings: Vec::new(),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    async fn sql_prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>> {
        let stmt = MySqlPreparedStatement {
            pool: Arc::clone(&self.pool),
            sql: sql.to_string(),
            statement_id: uuid::Uuid::new_v4().to_string(),
        };

        Ok(Box::new(stmt))
    }

    async fn sql_begin_transaction(&mut self, _isolation: Option<SqlTransactionIsolation>) -> DbResult<Box<dyn SqlTransaction>> {
        if self.in_transaction {
            return Err(DatabaseError::transaction(
                TransactionError::NotActive,
                "Transaction already active"
            ));
        }

        let conn = self.get_connection()?;
        let tx = conn.start_transaction(TxOpts::default())
            .map_err(|e| DatabaseError::transaction(
                TransactionError::NotActive,
                &format!("Failed to begin SQL transaction: {}", e)
            ))?;

        self.in_transaction = true;

        // Note: This is unsafe but necessary due to MySQL crate design
        let static_tx = unsafe { std::mem::transmute(tx) };
        
        let transaction = MySqlTransactionImpl {
            connection: Some(static_tx),
            transaction_id: uuid::Uuid::new_v4().to_string(),
            active: true,
        };

        Ok(Box::new(transaction))
    }

    async fn sql_batch(&mut self, statements: &[SqlBatch]) -> DbResult<Vec<SqlExecuteResult>> {
        let mut results = Vec::new();
        
        for batch in statements {
            let result = self.sql_execute(&batch.sql, &batch.parameters).await?;
            results.push(result);
        }
        
        Ok(results)
    }

    fn sql_connection_info(&self) -> SqlConnectionInfo {
        SqlConnectionInfo {
            server_version: "8.0.35".to_string(),
            protocol_version: "10".to_string(),
            database_name: "mysql".to_string(),
            schema_name: None,
            character_set: "utf8mb4".to_string(),
            collation: "utf8mb4_unicode_ci".to_string(),
            time_zone: "SYSTEM".to_string(),
            auto_commit: true,
            read_only: false,
            isolation_level: SqlTransactionIsolation::RepeatableRead,
            capabilities: vec!["transactions".to_string(), "json".to_string(), "fulltext".to_string()],
        }
    }

    async fn set_sql_variable(&mut self, name: &str, value: &SqlValue) -> DbResult<()> {
        let conn = self.get_connection()?;
        let mysql_value = Self::sql_value_to_mysql(value);
        
        let sql = format!("SET @{} = ?", name);
        conn.exec_drop(sql, (mysql_value,))
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Failed to set MySQL variable: {}", e)
            ))
    }

    async fn get_sql_variable(&mut self, name: &str) -> DbResult<SqlValue> {
        let conn = self.get_connection()?;
        
        let sql = format!("SELECT @{}", name);
        let result: Option<MySqlValue> = conn.exec_first(sql, ())
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("Failed to get MySQL variable: {}", e)
            ))?;

        match result {
            Some(value) => Ok(Self::mysql_value_to_sql(value)),
            None => Ok(SqlValue::Null),
        }
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
    }

//     fn columns(&self) -> &[crate::stdlib::packages::db_core::Column] {
        &self.metadata.columns
    }

    fn has_next(&self) -> bool {
        self.current_index < self.rows.len()
    }

//     fn metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        &self.metadata
    }

    fn row_count(&self) -> Option<usize> {
        Some(self.rows.len())
    }

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
                ConnectionError::FailedToConnect,
                &format!("Failed to get connection from pool: {}", e)
            ))?;

        let mysql_params = MySqlConnection::convert_parameters(parameters);
        
        conn.exec_drop(&self.sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("MySQL prepared statement execution failed: {}", e)
            ))?;

//         Ok(crate::stdlib::packages::db_core::ExecuteResult {
            affected_rows: conn.affected_rows(),
            last_insert_id: Some(conn.last_insert_id()),
            warnings: Vec::new(),
            query_id: Some(self.statement_id.clone()),
            execution_time: std::time::Duration::from_millis(0),
        })
    }

    async fn query(&mut self, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        let mut conn = self.pool.get_conn()
            .map_err(|e| DatabaseError::connection(
                ConnectionError::FailedToConnect,
                &format!("Failed to get connection from pool: {}", e)
            ))?;

        let mysql_params = MySqlConnection::convert_parameters(parameters);
        
        let mysql_rows: Vec<MySqlRow> = conn.exec(&self.sql, mysql_params)
            .map_err(|e| DatabaseError::query(
                QueryError::ExecutionFailed,
                &format!("MySQL prepared query execution failed: {}", e)
            ))?;

        let (result_rows, columns) = MySqlConnection::process_mysql_rows(mysql_rows)?;

        let result_set = MySqlResultSet {
            rows: result_rows,
//             metadata: crate::stdlib::packages::db_core::ResultMetadata {
                columns,
                row_count: None,
                affected_rows: 0,
                last_insert_id: None,
                warnings: Vec::new(),
                query_id: Some(self.statement_id.clone()),
                execution_time: std::time::Duration::from_millis(0),
            },
            current_index: 0,
        };

        Ok(Box::new(result_set))
    }

    fn parameter_count(&self) -> usize {
        // MySQL doesn't expose parameter count directly, we'll estimate
        self.sql.matches('?').count()
    }

    fn sql(&self) -> &str {
        &self.sql
    }
    
    /// slay Get parameter metadata
//     fn parameter_metadata(&self) -> &[crate::stdlib::packages::db_core::ParameterMetadata] {
        // Placeholder implementation - would need to extract from MySQL statement
        &[]
    }
    
    /// slay Get result set metadata
//     fn result_metadata(&self) -> &crate::stdlib::packages::db_core::ResultMetadata {
        // Placeholder implementation - would need to extract from MySQL statement
//         static EMPTY_METADATA: std::sync::LazyLock<crate::stdlib::packages::db_core::ResultMetadata> = 
//             std::sync::LazyLock::new(|| crate::stdlib::packages::db_core::ResultMetadata {
                columns: vec![],
                total_rows: None,
                has_more_rows: false,
                name: None,
                schema_name: None,
                table_name: None,
                is_updatable: false,
//                 result_type: crate::stdlib::packages::db_core::result::ResultType::ForwardOnly,
            });
        &EMPTY_METADATA
    }
    
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
                TransactionError::AlreadyCommitted,
                "Transaction already completed"
            ));
        }

        if let Some(tx) = self.connection.take() {
            tx.commit().map_err(|e| DatabaseError::transaction(
                TransactionError::ConstraintViolation,
                &format!("Failed to commit MySQL transaction: {}", e)
            ))?;
        }

        self.active = false;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> DbResult<()> {
        if !self.active {
            return Err(DatabaseError::transaction(
                TransactionError::AlreadyRolledBack,
                "Transaction already completed"
            ));
        }

        if let Some(tx) = self.connection.take() {
            tx.rollback().map_err(|e| DatabaseError::transaction(
                TransactionError::ConstraintViolation,
                &format!("Failed to rollback MySQL transaction: {}", e)
            ))?;
        }

        self.active = false;
        Ok(())
    }

//     async fn savepoint(&mut self, name: &str) -> DbResult<crate::stdlib::packages::db_core::SavePoint> {
        if let Some(ref mut tx) = self.connection {
            tx.exec_drop(&format!("SAVEPOINT {}", name), ())
                .map_err(|e| DatabaseError::transaction(
                    TransactionError::ConstraintViolation,
                    &format!("Failed to create MySQL savepoint: {}", e)
                ))?;
        }

//         Ok(crate::stdlib::packages::db_core::SavePoint {
            name: name.to_string(),
            transaction_id: self.transaction_id.clone(),
            created_at: std::time::SystemTime::now(),
        })
    }

//     async fn rollback_to_savepoint(&mut self, savepoint: &crate::stdlib::packages::db_core::SavePoint) -> DbResult<()> {
        if let Some(ref mut tx) = self.connection {
            tx.exec_drop(&format!("ROLLBACK TO SAVEPOINT {}", savepoint.name), ())
                .map_err(|e| DatabaseError::transaction(
                    TransactionError::SavepointNotFound,
                    &format!("Failed to rollback to MySQL savepoint: {}", e)
                ))?;
        }

        Ok(())
    }

    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>> {
        if let Some(ref mut tx) = self.connection {
            let mysql_params = MySqlConnection::convert_parameters(parameters);
            
            let mysql_rows: Vec<MySqlRow> = tx.exec(sql, mysql_params)
                .map_err(|e| DatabaseError::query(
                    QueryError::ExecutionFailed,
                    &format!("MySQL transaction query failed: {}", e)
                ))?;

            let (result_rows, columns) = MySqlConnection::process_mysql_rows(mysql_rows)?;

            let result_set = MySqlResultSet {
                rows: result_rows,
//                 metadata: crate::stdlib::packages::db_core::ResultMetadata {
                    columns,
                    row_count: None,
                    affected_rows: 0,
                    last_insert_id: None,
                    warnings: Vec::new(),
                    query_id: Some(uuid::Uuid::new_v4().to_string()),
                    execution_time: std::time::Duration::from_millis(0),
                },
                current_index: 0,
            };

            Ok(Box::new(result_set))
        } else {
            Err(DatabaseError::transaction(
                TransactionError::NotActive,
                "Transaction not active"
            ))
        }
    }

//     async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<crate::stdlib::packages::db_core::ExecuteResult> {
        if let Some(ref mut tx) = self.connection {
            let mysql_params = MySqlConnection::convert_parameters(parameters);
            
            tx.exec_drop(sql, mysql_params)
                .map_err(|e| DatabaseError::query(
                    QueryError::ExecutionFailed,
                    &format!("MySQL transaction execute failed: {}", e)
                ))?;

//             Ok(crate::stdlib::packages::db_core::ExecuteResult {
                affected_rows: tx.affected_rows(),
                last_insert_id: Some(tx.last_insert_id()),
                warnings: Vec::new(),
                query_id: Some(uuid::Uuid::new_v4().to_string()),
                execution_time: std::time::Duration::from_millis(0),
            })
        } else {
            Err(DatabaseError::transaction(
                TransactionError::NotActive,
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
}

// impl std::fmt::Display for MySqlError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "MySQL CursedError: {}", self.message)
//     }
// }

// impl std::error::CursedError for MySqlError {}
// 