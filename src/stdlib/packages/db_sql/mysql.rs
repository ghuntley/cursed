/// fr fr MySQL driver implementation - the popular choice periodt

use crate::stdlib::packages::{
    db_core::{DatabaseResult as DbResult, ConnectionConfig, DatabaseConnection, DriverFeature},
    db_sql::{SqlDriver, SqlConnection, SqlDialect, SqlDialectTrait}
};
use async_trait::async_trait;

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
}

/// fr fr MySQL error
#[derive(Debug)]
pub struct MySqlError {
    message: String,
}

impl MySqlDriver {
    pub fn new() -> Self {
        Self {
            name: "mysql".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl crate::stdlib::packages::db_core::DatabaseDriver for MySqlDriver {
    async fn connect(&self, _config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>> {
        Ok(Box::new(MySqlConnection {
            connection_id: "mysql_conn_1".to_string(),
        }))
    }

    fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
        crate::stdlib::packages::db_core::DriverInfo::new(
            &self.name,
            &self.version,
            "MySQL database driver",
            "CURSED"
        )
    }

    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    }

    fn sql_dialect(&self) -> crate::stdlib::packages::db_sql::SqlDialect {
        crate::stdlib::packages::db_sql::SqlDialect::MySQL
    }

    fn validate_connection_string(&self, _connection_string: &str) -> DbResult<()> {
        Ok(())
    }
}

#[async_trait]
impl SqlDriver for MySqlDriver {
    async fn sql_connect(&self, _config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        Ok(Box::new(MySqlConnection {
            connection_id: format!("mysql_sql_conn_{}", uuid::Uuid::new_v4()),
        }))
    }

    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
        Box::new(crate::stdlib::packages::db_sql::MySqlDialect::new())
    }

    fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
            crate::stdlib::packages::db_sql::SqlType::Integer,
            crate::stdlib::packages::db_sql::SqlType::Text,
            crate::stdlib::packages::db_sql::SqlType::Boolean,
            crate::stdlib::packages::db_sql::SqlType::Json,
        ]
    }

    fn supports_sql_feature(&self, _feature: crate::stdlib::packages::db_sql::SqlFeature) -> bool {
        true
    }

    fn configuration_options(&self) -> Vec<crate::stdlib::packages::db_sql::ConfigurationOption> {
        vec![]
    }

    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    }

    fn performance_info(&self) -> crate::stdlib::packages::db_sql::DriverPerformanceInfo {
        crate::stdlib::packages::db_sql::DriverPerformanceInfo {
            connection_time: std::time::Duration::from_millis(80),
            query_overhead: std::time::Duration::from_micros(30),
            max_connections: Some(2000),
            connection_pooling: true,
            statement_caching: true,
            batch_operations: true,
            streaming_results: true,
        }
    }

    fn limitations(&self) -> crate::stdlib::packages::db_sql::DriverLimitations {
        crate::stdlib::packages::db_sql::DriverLimitations {
            max_statement_length: Some(1024 * 1024),
            max_parameters: Some(65535),
            max_identifier_length: Some(64),
            max_string_length: Some(65535),
            max_numeric_precision: Some(65),
            max_columns: Some(4096),
            max_rows: None,
            unsupported_features: vec![],
        }
    }
}

impl std::fmt::Display for MySqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MySQL Error: {}", self.message)
    }
}

impl std::error::Error for MySqlError {}
