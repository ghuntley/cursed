/// fr fr SQLite driver implementation - the lightweight champion periodt

use crate::stdlib::packages::{
    db_core::{DatabaseResult as DbResult, ConnectionConfig, DatabaseConnection, DriverFeature},
    db_sql::{SqlDriver, SqlConnection, SqlDialect, SqlDialectTrait, SqlFeature}
};
use async_trait::async_trait;

/// fr fr SQLite driver
#[derive(Debug)]
pub struct SqliteDriver {
    name: String,
    version: String,
}

/// fr fr SQLite connection
#[derive(Debug)]
pub struct SqliteConnection {
    connection_id: String,
}

/// fr fr SQLite error
#[derive(Debug)]
pub struct SqliteError {
    message: String,
}

impl SqliteDriver {
    pub fn new() -> Self {
        Self {
            name: "sqlite".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl crate::stdlib::packages::db_core::DatabaseDriver for SqliteDriver {
    async fn connect(&self, _config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>> {
        Ok(Box::new(SqliteConnection {
            connection_id: "sqlite_conn_1".to_string(),
        }))
    }

    fn driver_info(&self) -> crate::stdlib::packages::db_core::DriverInfo {
        crate::stdlib::packages::db_core::DriverInfo::new(
            &self.name,
            &self.version,
            "SQLite database driver",
            "CURSED"
        )
    }

    fn supports_feature(&self, _feature: DriverFeature) -> bool {
        true
    }

    fn sql_dialect(&self) -> crate::stdlib::packages::db_sql::SqlDialect {
        crate::stdlib::packages::db_sql::SqlDialect::SQLite
    }

    fn validate_connection_string(&self, _connection_string: &str) -> DbResult<()> {
        Ok(())
    }
}

#[async_trait]
impl SqlDriver for SqliteDriver {
    async fn sql_connect(&self, _config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        Ok(Box::new(SqliteConnection {
            connection_id: format!("sqlite_sql_conn_{}", uuid::Uuid::new_v4()),
        }))
    }

    fn sql_dialect(&self) -> Box<dyn SqlDialectTrait> {
        Box::new(crate::stdlib::packages::db_sql::SqliteDialect::new())
    }

    fn supported_types(&self) -> Vec<crate::stdlib::packages::db_sql::SqlType> {
        vec![
            crate::stdlib::packages::db_sql::SqlType::Integer,
            crate::stdlib::packages::db_sql::SqlType::Text,
            crate::stdlib::packages::db_sql::SqlType::Boolean,
            crate::stdlib::packages::db_sql::SqlType::Blob,
        ]
    }

    fn supports_sql_feature(&self, feature: SqlFeature) -> bool {
        // SQLite has some limitations
        match feature {
            SqlFeature::ForeignKeys => true,
            SqlFeature::Triggers => true,
            SqlFeature::Views => true,
            SqlFeature::Indexes => true,
            SqlFeature::WindowFunctions => true,
            SqlFeature::CommonTableExpressions => true,
            SqlFeature::JsonSupport => true,
            _ => false, // Many advanced features not supported
        }
    }

    fn configuration_options(&self) -> Vec<crate::stdlib::packages::db_sql::ConfigurationOption> {
        vec![]
    }

    fn validate_sql(&self, _sql: &str) -> DbResult<()> {
        Ok(())
    }

    fn performance_info(&self) -> crate::stdlib::packages::db_sql::DriverPerformanceInfo {
        crate::stdlib::packages::db_sql::DriverPerformanceInfo {
            connection_time: std::time::Duration::from_millis(10),
            query_overhead: std::time::Duration::from_micros(10),
            max_connections: Some(1), // SQLite is single-threaded
            connection_pooling: false,
            statement_caching: true,
            batch_operations: true,
            streaming_results: false,
        }
    }

    fn limitations(&self) -> crate::stdlib::packages::db_sql::DriverLimitations {
        crate::stdlib::packages::db_sql::DriverLimitations {
            max_statement_length: Some(1000000),
            max_parameters: Some(999),
            max_identifier_length: Some(255),
            max_string_length: Some(1000000000),
            max_numeric_precision: Some(15),
            max_columns: Some(2000),
            max_rows: None,
            unsupported_features: vec![
                SqlFeature::StoredProcedures,
                SqlFeature::UserDefinedFunctions,
                SqlFeature::MaterializedViews,
                SqlFeature::Partitioning,
            ],
        }
    }
}

impl std::fmt::Display for SqliteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SQLite Error: {}", self.message)
    }
}

impl std::error::Error for SqliteError {}
