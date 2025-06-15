/// fr fr MySQL type conversion and value handling
/// 
/// This module provides comprehensive type conversion between MySQL types
/// and CURSED SqlValue types, handling all MySQL data types properly.

use std::time::SystemTime;
use mysql::{Value as MySqlValue, Row};
use crate::stdlib::database::SqlValue;
use super::error::{MySqlError, MySqlResult};

/// Convert CURSED SqlValue to MySQL Value
pub fn convert_from_sql_value(value: &SqlValue) -> MySqlResult<MySqlValue> {
    match value {
        SqlValue::Null => Ok(MySqlValue::NULL),
        SqlValue::Boolean(b) => Ok(MySqlValue::Int(if *b { 1 } else { 0 })),
        SqlValue::Integer(i) => Ok(MySqlValue::Int(*i)),
        SqlValue::Float(f) => Ok(MySqlValue::Double(*f)),
        SqlValue::String(s) => Ok(MySqlValue::Bytes(s.as_bytes().to_vec())),
        SqlValue::Bytes(b) => Ok(MySqlValue::Bytes(b.clone())),
        SqlValue::Timestamp(ts) => {
            // Convert SystemTime to MySQL datetime string
            match ts.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(duration) => {
                    let secs = duration.as_secs();
                    let datetime = chrono::DateTime::from_timestamp(secs as i64, 0)
                        .unwrap_or_else(|| chrono::Utc::now());
                    let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                    Ok(MySqlValue::Bytes(formatted.as_bytes().to_vec()))
                }
                Err(_) => Err(MySqlError::type_conversion_error("SystemTime", "MySQL DATETIME")),
            }
        }
        SqlValue::Json(json) => {
            let json_str = serde_json::to_string(json)
                .map_err(|e| MySqlError::type_conversion_error("Json", &format!("MySQL JSON: {}", e)))?;
            Ok(MySqlValue::Bytes(json_str.as_bytes().to_vec()))
        }
    }
}

/// Convert MySQL Value to CURSED SqlValue
pub fn convert_to_sql_value(value: MySqlValue) -> MySqlResult<SqlValue> {
    match value {
        MySqlValue::NULL => Ok(SqlValue::Null),
        MySqlValue::Bytes(bytes) => {
            // Try to convert to string first, fall back to bytes
            match String::from_utf8(bytes.clone()) {
                Ok(s) => Ok(SqlValue::String(s)),
                Err(_) => Ok(SqlValue::Bytes(bytes)),
            }
        }
        MySqlValue::Int(i) => Ok(SqlValue::Integer(i)),
        MySqlValue::UInt(u) => {
            if u <= i64::MAX as u64 {
                Ok(SqlValue::Integer(u as i64))
            } else {
                // Convert large unsigned integers to string to avoid overflow
                Ok(SqlValue::String(u.to_string()))
            }
        }
        MySqlValue::Float(f) => Ok(SqlValue::Float(f as f64)),
        MySqlValue::Double(d) => Ok(SqlValue::Float(d)),
        MySqlValue::Date(year, month, day, hour, minute, second, microsecond) => {
            // Convert MySQL date/time to SystemTime
            let datetime = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32)
                .and_then(|date| {
                    date.and_hms_micro_opt(hour as u32, minute as u32, second as u32, microsecond)
                })
                .and_then(|dt| dt.and_utc().timestamp_opt().single())
                .map(|timestamp| SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64));
            
            match datetime {
                Some(ts) => Ok(SqlValue::Timestamp(ts)),
                None => {
                    // Fall back to string representation
                    let date_str = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", 
                                         year, month, day, hour, minute, second);
                    Ok(SqlValue::String(date_str))
                }
            }
        }
        MySqlValue::Time(is_negative, days, hours, minutes, seconds, microseconds) => {
            // Convert MySQL TIME to string representation
            let sign = if is_negative { "-" } else { "" };
            let time_str = if days > 0 {
                format!("{}{}:{:02}:{:02}:{:02}.{:06}", sign, days * 24 + hours as u32, minutes, seconds, microseconds)
            } else {
                format!("{}{:02}:{:02}:{:02}.{:06}", sign, hours, minutes, seconds, microseconds)
            };
            Ok(SqlValue::String(time_str))
        }
    }
}

/// Extract value from MySQL Row by column index
pub fn extract_value_by_index(row: &Row, index: usize) -> MySqlResult<SqlValue> {
    match row.get_opt::<MySqlValue, usize>(index) {
        Some(Ok(value)) => convert_to_sql_value(value),
        Some(Err(e)) => Err(MySqlError::type_conversion_error(
            "MySQL Row value",
            &format!("SqlValue (index {}): {}", index, e)
        )),
        None => Ok(SqlValue::Null),
    }
}

/// Extract value from MySQL Row by column name
pub fn extract_value_by_name(row: &Row, name: &str) -> MySqlResult<SqlValue> {
    match row.get_opt::<MySqlValue, &str>(name) {
        Some(Ok(value)) => convert_to_sql_value(value),
        Some(Err(e)) => Err(MySqlError::type_conversion_error(
            "MySQL Row value",
            &format!("SqlValue (column '{}'): {}", name, e)
        )),
        None => Ok(SqlValue::Null),
    }
}

/// Get column information from MySQL Row
pub fn get_column_info(row: &Row) -> (Vec<String>, Vec<String>) {
    let columns = row.columns_ref();
    let mut column_names = Vec::new();
    let mut column_types = Vec::new();
    
    for column in columns {
        column_names.push(column.name_str().to_string());
        column_types.push(format!("{:?}", column.column_type()));
    }
    
    (column_names, column_types)
}

/// Convert MySQL isolation level to CURSED isolation level
pub fn convert_isolation_level(level: crate::stdlib::database::SqlIsolationLevel) -> MySqlResult<mysql::IsolationLevel> {
    match level {
        crate::stdlib::database::SqlIsolationLevel::LevelReadUncommitted => Ok(mysql::IsolationLevel::ReadUncommitted),
        crate::stdlib::database::SqlIsolationLevel::LevelReadCommitted => Ok(mysql::IsolationLevel::ReadCommitted),
        crate::stdlib::database::SqlIsolationLevel::LevelRepeatableRead => Ok(mysql::IsolationLevel::RepeatableRead),
        crate::stdlib::database::SqlIsolationLevel::LevelSerializable => Ok(mysql::IsolationLevel::Serializable),
        _ => {
            // Default to READ COMMITTED for unsupported levels
            Ok(mysql::IsolationLevel::ReadCommitted)
        }
    }
}

/// Escape SQL string for safe query construction
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('\'', "\\'")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
     .replace('\0', "\\0")
}

/// Build parameter placeholders for prepared statements
pub fn build_placeholders(count: usize) -> String {
    if count == 0 {
        String::new()
    } else {
        "?".to_string().repeat(count).chars()
            .collect::<Vec<char>>()
            .chunks(1)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// Validate MySQL connection string format
pub fn validate_connection_string(dsn: &str) -> MySqlResult<()> {
    if dsn.is_empty() {
        return Err(MySqlError::configuration_error("Connection string cannot be empty"));
    }
    
    // Basic validation - should contain mysql:// or be in standard format
    if !dsn.starts_with("mysql://") && !dsn.contains("@") {
        return Err(MySqlError::configuration_error(
            "Invalid MySQL connection string format. Expected mysql://user:pass@host:port/database or user:pass@host:port/database"
        ));
    }
    
    Ok(())
}

/// Parse MySQL connection string into components
pub fn parse_connection_string(dsn: &str) -> MySqlResult<MySqlConnectionInfo> {
    validate_connection_string(dsn)?;
    
    let dsn = if dsn.starts_with("mysql://") {
        &dsn[8..] // Remove mysql:// prefix
    } else {
        dsn
    };
    
    // Parse user:pass@host:port/database format
    let parts: Vec<&str> = dsn.split('@').collect();
    if parts.len() != 2 {
        return Err(MySqlError::configuration_error("Invalid connection string format"));
    }
    
    let auth_part = parts[0];
    let host_db_part = parts[1];
    
    // Parse user:pass
    let auth_parts: Vec<&str> = auth_part.split(':').collect();
    let user = auth_parts[0].to_string();
    let password = if auth_parts.len() > 1 {
        auth_parts[1].to_string()
    } else {
        String::new()
    };
    
    // Parse host:port/database
    let host_db_parts: Vec<&str> = host_db_part.split('/').collect();
    if host_db_parts.len() != 2 {
        return Err(MySqlError::configuration_error("Database name missing from connection string"));
    }
    
    let host_port = host_db_parts[0];
    let database = host_db_parts[1].to_string();
    
    // Parse host:port
    let host_port_parts: Vec<&str> = host_port.split(':').collect();
    let host = host_port_parts[0].to_string();
    let port = if host_port_parts.len() > 1 {
        host_port_parts[1].parse::<u16>()
            .map_err(|_| MySqlError::configuration_error("Invalid port number"))?
    } else {
        3306 // Default MySQL port
    };
    
    Ok(MySqlConnectionInfo {
        host,
        port,
        user,
        password,
        database,
    })
}

/// MySQL connection information
#[derive(Debug, Clone)]
pub struct MySqlConnectionInfo {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sql_value_conversion() {
        // Test null conversion
        let null_value = SqlValue::Null;
        let mysql_null = convert_from_sql_value(&null_value).unwrap();
        assert!(matches!(mysql_null, MySqlValue::NULL));
        
        // Test integer conversion
        let int_value = SqlValue::Integer(42);
        let mysql_int = convert_from_sql_value(&int_value).unwrap();
        assert!(matches!(mysql_int, MySqlValue::Int(42)));
        
        // Test string conversion
        let str_value = SqlValue::String("test".to_string());
        let mysql_str = convert_from_sql_value(&str_value).unwrap();
        assert!(matches!(mysql_str, MySqlValue::Bytes(_)));
    }
    
    #[test]
    fn test_connection_string_parsing() {
        let dsn = "mysql://user:pass@localhost:3306/testdb";
        let info = parse_connection_string(dsn).unwrap();
        
        assert_eq!(info.user, "user");
        assert_eq!(info.password, "pass");
        assert_eq!(info.host, "localhost");
        assert_eq!(info.port, 3306);
        assert_eq!(info.database, "testdb");
    }
    
    #[test]
    fn test_placeholder_generation() {
        assert_eq!(build_placeholders(0), "");
        assert_eq!(build_placeholders(1), "?");
        assert_eq!(build_placeholders(3), "?, ?, ?");
    }
    
    #[test]
    fn test_string_escaping() {
        let input = "test'string\"with\nnewlines";
        let escaped = escape_string(input);
        assert!(escaped.contains("\\'"));
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\n"));
    }
}
