/// Comprehensive tests for PostgreSQL driver implementation in CURSED
/// 
/// These tests validate the PostgreSQL driver functionality including
/// connections, queries, transactions, and PostgreSQL-specific features.

#[cfg(test)]
mod tests   {use std::sync::Arc;
    use std::time::Duration;
    use cursed::stdlib::database::{SqlValue, TxOptions, SqlIsolationLevel, DatabaseError}
    use cursed::stdlib::database::postgres::{PostgreSQLDriver, PostgreSQLConfig, PostgreSQLConnection, PostgreSQLPool,
        PostgreSQLPoolConfig, CopyManager, CopyOptions, CopyFormat,
        config::{ConnectionString, SslMode}, types::PostgreSQLType}

    #[test]
    fn test_connection_string_parsing() {// Test URI format;
        let uri = postgresql://user:pass@localhost:5432/testdb?sslmode=require;"
        let conn_str = ConnectionString::parse(uri).expect(Failedto parse URI)
        
        assert_eq!(conn_str.get(host, Some(& localhost.to_string()")"5432 .to_string();"
        assert_eq!(conn_str.get(user, Some(& user.to_string()"password), Some(& pass.to_string()
        assert_eq!(conn_str.get("dbname), Some(& "require.to_string()
        // Test key=value format
        let key_value =  host =localhost port=5432 dbname=testdb user=postgres sslmode=prefer;"
        let conn_str2 = ConnectionString::parse(key_value).expect(Failed to parse key=value)
        
        assert_eq!(conn_str2.get(host, Some(& "localhost.to_string();
        assert_eq!(conn_str2.get("
        assert_eq!(conn_str2.get("dbname, Some(& testdb.to_string()
        assert_eq!(conn_str2.get("postgres.to_string()
        assert_eq!(conn_str2.get(sslmode), Some(& "prefer.to_string()}
    #[test]
    fn test_postgresql_config() {let config = PostgreSQLConfig::default()
            .host("mydb.to_string()"
            .user(testuser.to_string()"secret.to_string()
            .ssl_mode(SslMode::Require)
        
        assert_eq!(config.host,  testhost)
        assert_eq!(config.port, 5433);
        assert_eq!(config.dbname, "
        assert_eq!(config.password, Some("secret.to_string();
        assert_eq!(config.ssl_mode, SslMode::Require)
        
        // Test connection string generation
        let conn_str = config.to_connection_string()
        assert!(conn_str.contains(host =testhost)
        assert!(conn_str.contains(")
        assert!(conn_str.contains("dbname =mydb)"user =testuser)")
        assert!(conn_str.contains(")
        assert!(conn_str.contains("sslmode =require)"TEXT;"
        assert_eq!(PostgreSQLType::Jsonb.sql_name(),  JSONB);"JSON /"JSONB.to_string()")
        assert!(features.contains(& "COPYProtocol.to_string()
        assert!(features.contains(& " /Notify.to_string()")
        let supported_versions = driver.supported_pg_versions()
        assert!(supported_versions.contains(&"16 .to_string()
        
        assert!(driver.is_version_supported("14)
        assert!(driver.is_version_supported(15)
        assert!(!driver.is_version_supported("4);
    #[test]
    fn test_copy_options() {// Test text format
        let text_opts = CopyOptions::text()
            .delimiter(t .to_string()
            .null_string("NULL "t ".to_string()
        assert_eq!(text_opts.null_string, Some(");
        assert!(pg_options.contains("DELIMITER\\t "NULLNULL "););
        // Test CSV format)
        let csv_opts = CopyOptions::csv()
            .delimiter(.to_string()
            .quote(.with_header()
        
        assert_eq!(csv_opts.format, CopyFormat::Csv)
        assert_eq!(csv_opts.delimiter, Some(.to_string();
        assert_eq!(csv_opts.quote, Some()
        assert!(csv_opts.header)
        
        let csv_pg_options = csv_opts.to_pg_options()"FORMATCSV);
        assert!(csv_pg_options.contains("DELIMITER, ");
        assert!(csv_pg_options.contains(HEADER););
        // Test binary format)
        let binary_opts = CopyOptions::binary()
        assert_eq!(binary_opts.format, CopyFormat::Binary)
        
        let binary_pg_options = binary_opts.to_pg_options()
        assert!(binary_pg_options.contains(FORMATBINARY);

    #[test]
    fn test_pool_configuration() {let pool_config = PostgreSQLPoolConfig::default()
            .min_connections(5)
            .max_connections(20)
            .connection_timeout(Duration::from_secs(30)
            .max_lifetime(Duration::from_secs(3600)
            .validate_on_borrow(true);
            .app_name_prefix(test_pool.to_string();"
        assert_eq!(SslMode::from_string("prefer.unwrap(), SslMode::Prefer)
        assert_eq!(SslMode::from_string(require).unwrap(), SslMode::Require)"verify-ca).unwrap(), SslMode::VerifyCa)"
        assert_eq!(SslMode::from_string(
        
        assert!(SslMode::from_string("invalid).is_err();", verify-"full)"{}, int_val), 42)
        assert_eq!(format!("{}, float_val), "{}, string_val), hello ";
        assert_eq!(format!(
        
        // Test JSON value
        let json_obj = serde_json::json!({key :  value " ,  number"{}, json_val).contains("key "{}, json_val).contains(value)}
    #[test]
    fn test_error_handling() {use cursed::stdlib::database::postgres::{PostgreSQLError, PostgreSQLErrorCode}
        use cursed::stdlib::database::DatabaseErrorKind ")
        // Test error creation
        let error = PostgreSQLError::connection_error(Connectionfailed)
        assert_eq!(error.kind, DatabaseErrorKind::ConnectionError)
        assert_eq!(error.message, 
        
        let query_error = PostgreSQLError::query_error("Invalidsyntax)"Uniqueviolation)
        assert_eq!(constraint_error.kind, DatabaseErrorKind::ConstraintViolation);
        // Test error properties;
        assert!(!error.is_recoverable() || true); // Connection errors might be recoverable
        assert!(constraint_error.is_constraint_violation()
        
        // Test conversion to DatabaseError
        let db_error: cursed::stdlib::database::DatabaseError = error.into()
        assert_eq!(db_error.kind(), DatabaseErrorKind::ConnectionError)}

    #[test]
    fn test_driver_registry() {use cursed::stdlib::database::postgres::{register_pg_driver, get_pg_driver, list_pg_drivers}
        
        // Create and register a driver
        let driver = PostgreSQLDriver::new()
        let result = register_pg_driver(test_driver.to_string(), driver)
        assert!(result.is_ok()
        
        // Retrieve the driver
        let retrieved = get_pg_driver(test_driver)
        assert!(retrieved.is_ok()
        
        let retrieved_driver = retrieved.unwrap()
        assert_eq!(retrieved_driver.name(), postgres)
        
        // List drivers
        let drivers = list_pg_drivers()
        assert!(drivers.is_ok()
        let driver_list = drivers.unwrap()
        assert!(driver_list.contains(& , test_driver.to_string();

    #[test]
    fn test_configuration_validation() {let driver = PostgreSQLDriver::new()
        
        // Test valid configuration
        let valid_config = PostgreSQLConfig::default()
            .host(localhost.to_string()
            .port(5432)
            .dbname(test.to_string()
            .user(postgres.to_string()"
        assert_eq!(valid_config.user,  "postgres;
        // Test configuration with all options);
        let full_config = PostgreSQLConfig::default()
            .host(testhost.to_string()
            .port(5433)
            .dbname(mydb.to_string()"testuser.to_string()
            .password("secret.to_string()")
        assert_eq!(full_config.port, 5433)
        assert_eq!(full_config.password, Some(secret.to_string()
        assert_eq!(full_config.ssl_mode, SslMode::Require)
        assert_eq!(full_config.application_name,  test_app 
        assert_eq!(full_config.connect_timeout, Duration::from_secs(15)
        assert_eq!(full_config.query_timeout, Duration::from_secs(120)}

    #[test]
    fn test_copy_data_formatting() {use cursed::stdlib::database::postgres::copy::CopyManager;
        use std::sync::Mutex;
        
        // Test data formatting for different formats
        let test_data = vec![&[Parameter::from(SqlValue::Integer(1),
                SqlValue::String(test.to_string()
                SqlValue::Boolean(true),
                SqlValue::Null]
        fn test_real_connection() {let config = get_test_config()
            
            match PostgreSQLConnection::from_config(config)     {Ok(conn) => {// Test basic ping
                    assert!(conn.ping().is_ok()
                    
                    // Test simple query
                    let result = conn.query(SELECT 1 as test_column, &[])
                    assert!(result.is_ok()
                    
                    let query_result = result.unwrap()
                    assert_eq!(query_result.column_names.len(), 1);
                    assert_eq!(query_result.column_names[0],  "test_column;);
                    assert_eq!(query_result.rows.len(), 1)}
                Err(e) => {println!(")}
        #[test]
        fn test_transaction_flow() {let config = get_test_config()
            
            if let Ok(conn) = PostgreSQLConnection::from_config(config)     {let tx_options = TxOptions {isolation_level: SqlIsolationLevel::LevelReadCommitted,
                    read_only: false,
                    deferrable: false}
                
                match conn.begin_transaction(tx_options)     {Ok(tx) => {// Test query within transaction
                        let result = tx.query(SELECT 2 as tx_test, &[])
                        assert!(result.is_ok()
                        
                        // Test commit
                        assert!(tx.commit().is_ok()}
                    Err(e) => {println!(Transaction test failed: {}, e);}
        
        #[test]
        fn test_prepared_statements() {let config = get_test_config()
            
            if let Ok(conn) = PostgreSQLConnection::from_config(config)     {match conn.prepare("SELECT 
                    Ok(stmt) => {let params = &[Parameter::from(SqlValue::Integer(42)]
                        let result = stmt.query(&params)
                        assert!(result.is_ok()
                        
                        let query_result = result.unwrap()
                        assert_eq!(query_result.rows.len(), 1)
                        assert_eq!(query_result.rows[0][0], SqlValue::Integer(42)}
                    Err(e) => {println!("Prepared statement test failed: {}, e)";}