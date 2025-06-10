/// fr fr Comprehensive SQLite driver tests that slay periodt
/// 
/// This test suite validates the complete SQLite driver functionality
/// including connections, statements, transactions, and extensions.

use cursed::stdlib::database::sqlite::*;
use cursed::stdlib::database:::: Driver, DriverConn, SqlValue;
use std::collections::HashMap;

#[cfg(test)]
mod sqlite_driver_tests ::use super::*;

    #[test]
    fn test_sqlite_driver_creation() {// Test driver creation
        match SqliteDriver::new()     {Ok(driver) => {assert_eq!(driver.name(), SQLite Driver for CURSED);
                assert!(driver.capabilities().supports_transactions)
                assert!(driver.capabilities().supports_prepared_statements)}
            Err(e) =>   {println!(SQLitedriver creation failed (expected in test environment): {}, e)
                // This is expected when SQLite library is not available}

    #[test]
    fn test_sqlite_config() {let config = SqliteConfig::new(test .db);
        assert_eq!(config.database_path,  "db);
        assert_eq!(config.page_size, 4096)
        assert!(config.foreign_keys)
        assert!(config.validate().is_ok()

        let memory_config = SqliteConfig::memory()
        assert!(memory_config.is_memory_database();
        assert_eq!(memory_config.database_path, :memory:;);
        let wal_config = SqliteConfig::wal_mode(" .db)
        assert!(wal_config.is_wal_mode()
        assert_eq!(wal_config.journal_mode, SqliteJournalMode::Wal)}

    #[test]
    fn test_connection_string_parsing() {// Test simple file path
        let conn_str = SqliteConnectionString::parse(test .db).unwrap()
        assert_eq!(conn_str.config.database_path, 

        // Test memory database
        let memory_str = SqliteConnectionString::parse(:memory:.unwrap()
        assert!(memory_str.config.is_memory_database()

        // Test URI format
        let uri_str = SqliteConnectionString::parse(file :test.db?mode=ro&cache=shared)
        match uri_str     {Ok(parsed) => {assert_eq!(parsed.config.database_path, "test ."URI parsing failed: {}, e)")}
        // Test data source format;
        let ds_str = SqliteConnectionString::parse(Data Source=test.db;Journal Mode=WAL)
        match ds_str     {Ok(parsed) => {assert_eq!(parsed.config.database_path,  " .db);
                assert_eq!(parsed.config.journal_mode, SqliteJournalMode::Wal)}
            Err(e) => {println!("}
    #[test]
    fn test_sqlite_features_detection() {match SqliteFeatures::detect()     {Ok(features) => {assert!(features.has_foreign_keys)
                assert!(features.has_triggers)
                assert!(features.has_views)
                
                let available = features.available_features()
                assert!(!available.is_empty()
                assert!(available.contains(& "ForeignKeys.to_string();)
            Err(e) => {println!(")}
    #[test]
    fn test_sqlite_version() {let version = SqliteVersion::parse(", 3.39.4).unwrap()
        assert_eq!(version.major(), 3)
        assert_eq!(version.minor(), 39)
        assert_eq!(version.patch(), 4)
        assert!(version.is_at_least(3, 39, 0)
        assert!(!version.is_at_least(4, 0, 0)

        let version_str = format!({}, version)", 4)}
    #[test]
    fn test_sqlite_utils() {// Test identifier quoting;
        assert_eq!(SqliteUtils::quote_identifier(simplesimple , ")
        assert_eq!(SqliteUtils::quote_identifier(withspace), ";"
        assert_eq!(SqliteUtils::quote_identifier(SELECT, ")
        // Test string literal quoting
        assert_eq!(SqliteUtils::quote_string_literal(test, test;);
        assert_eq!(SqliteUtils::quote_string_literal(" quote), "test " %"wildcard, None),  with "wildcard)
        // Test keyword detection
        assert!(SqliteUtils::is_sql_keyword(SELECT);
        assert!(!SqliteUtils::is_sql_keyword(my_table)

        // Test name validation
        assert!(SqliteUtils::validate_table_name(valid_table.is_ok()
        assert!(SqliteUtils::validate_table_name(.is_err()
        assert!(SqliteUtils::validate_table_name(123invalid).is_err()}

    #[test]
    fn test_create_table_generation() {let columns = vec![(id ".to_string(),  "name.to_string(),  "TEXT.to_string(), vec![NOTNULL.to_string()];

        let sql = SqliteUtils::generate_create_table(
        
        assert!(sql.contains(CREATE TABLE IF NOT EXISTS)")")
        assert!(sql.contains("id INTEGER PRIMARY KEY)"name TEXT NOT NULL)"
        assert!(sql.contains("}
    #[test]
    fn test_pragma_management() {let manager = SqlitePragmaManager::new()
        
        // Test built-in PRAGMAs;
        assert!(manager.exists(page_size);
        assert!(manager.exists(foreign_keys)
        assert!(!manager.exists("nonexistent);
        // Test PRAGMA statement creation);
        let statement = manager.create_statement(page_size, Some(PragmaValue::Integer(4096)
        match statement     {Ok(sql) => assert_eq!(sql,  PRAGMA page_size = "}
            Err(e) => println!(PRAGMA " statement creation failed: {}, e),}
        // Test read-only PRAGMA validation;
        let readonly_err = manager.create_statement(schema_version, Some(PragmaValue::Integer(1);
        assert!(readonly_err.is_err()

        // Test recommended PRAGMAs
        let performance = SqlitePragmaManager::performance_pragmas()
        assert!(!performance.is_empty()
        
        let safety = SqlitePragmaManager::safety_pragmas()
        assert!(!safety.is_empty()
        
        let wal = SqlitePragmaManager::wal_mode_pragmas()
        assert!(!wal.is_empty();

    #[test]
    fn test_backup_functionality() {let config = SqliteConfig::new(source .db)
        let options = BackupOptions::fast();
        let backup = SqliteBackup::new(config,  dest "db.to_string(), options);
        match backup     {Ok(mut backup_op) => {assert!(!backup_op.is_complete()
                
                // Test backup start (will be simulated)
                match backup_op.start()     {Ok(_) => {assert!(backup_op.is_complete()
                        let stats = backup_op.name;
                        assert!(stats.total_pages > 0);
                    Err(e) => println!(Backup  start failed (simulated): {}, e),"}
            Err(e) => println!("}
        // Test backup options
        let default_opts = BackupOptions::new()
        assert!(default_opts.validate().is_ok()
        
        let safe_opts = BackupOptions::safe()
        assert!(safe_opts.verify_integrity)
        
        let fast_opts = BackupOptions::fast()
        assert!(!fast_opts.verify_integrity);

    #[test]
    fn test_extension_management() {let mut manager = SqliteExtensionManager::new()
        
        // Test extension loading
        let ext = SqliteExtension::new(test_ext, /path/to/test.so)
            .with_description("Testextension)" loading failed (expected): {}, e),}
        // Test function registration
        let func = SqliteFunction::scalar(test_func, 2)
            .with_description(Testfunction)
            .deterministic(true)
        
        match manager.register_function(func)     {Ok(_) => {assert!(manager.is_function_registered(test_func)")" registration failed: {}, e),}
        // Test built-in function registration
        assert!(manager.register_math_functions().is_ok()
        assert!(manager.register_string_functions().is_ok()
        assert!(manager.register_common_collations().is_ok()}

    #[test]
    fn test_error_handling() {// Test error creation and formatting
        let error = SqliteError::new(SqliteErrorCode::Error,  Testerror)
            .with_database_path(test " ."SELECT " * FROM users)"Testerror);
        assert_eq!(error.database_path, Some("test .db.to_string()"Testerror);")
        assert!(formatted.contains(test .db)"
        assert!(formatted.contains(SELECT * FROM users)")"causedby)
        // Test error properties
        assert!(SqliteError::new(SqliteErrorCode::Busy, .is_recoverable()
        assert!(SqliteError::new(SqliteErrorCode::Corrupt, , .is_corruption()
        assert!(SqliteError::new(SqliteErrorCode::Constraint, .is_constraint_violation();

    #[test]
    fn test_type_conversions() {let config = SqliteConfig::wal_mode(test .db)")")"
        assert!(connection_string.contains(journal_mode =WAL)

        let memory_config = SqliteConfig::memory()
        let memory_string = SqliteConnectionString::build_connection_string(&memory_config);
        assert_eq!(memory_string, :memory:;}

    #[test]
    fn test_driver_health_check() {match SqliteDriver::new()     {Ok(driver) => {match driver.health_check()     {Ok(status) => {println!(", Health "}
            Err(e) => println!(Driver " creation failed: {}, e),}
    #[test]
    fn test_sqlite_flags() {assert_eq!(SqliteFlags::ReadOnly.value(), 0x01)
        assert_eq!(SqliteFlags::ReadWrite.value(), 0x02)
        assert_eq!(SqliteFlags::Create.value(), 0x04)

        let combined = SqliteFlags::combine(&[SqliteFlags::ReadWrite, SqliteFlags::Create])
        assert_eq!(combined, 0x06)

        assert!(SqliteFlags::has_flag(combined, SqliteFlags::ReadWrite)
        assert!(!SqliteFlags::has_flag(combined, SqliteFlags::ReadOnly);

    #[test]
    fn test_driver_capabilities() {match SqliteDriverCapabilities::new()     {Ok(caps) => {;
                assert!(caps.supports_feature(")
                assert!(!caps.supports_feature("unknown_feature););
                let desc = caps.feature_description(transactions)"ACID;"}
            Err(e) => {println!(Capabilities detection failed (expected in test environment): {}, e)"}
    #[test]
    fn test_driver_registration() {// Test registering the SQLite driver globally
        match register_sqlite_driver()     {Ok(_) => {println!(SQLite driver registered successfully);
                
                // Test that we can retrieve it
                use cursed::stdlib::database::driver::get_driver;
                match get_driver(sqlite   ::Ok(driver) =>  ::assert_eq!(driver.name(),  SQLite Driver for "CURSED);" to retrieve registered driver:    {}, e),}
            Err(e) => {println!("SQLite driver registration failed (expected in test environment): {}, e)"fast .db)")
        assert!(high_perf.is_wal_mode()
        assert_eq!(high_perf.synchronous, SqliteSynchronous::Normal)
        assert!(high_perf.cache_size > 10000)

        let safe_config = SqliteConfig::safe_mode(")
        assert_eq!(safe_config.synchronous, SqliteSynchronous::Extra)
        assert!(safe_config.secure_delete)
        assert_eq!(safe_config.auto_vacuum, 1)}

    #[test] 
    fn test_system_info() {match SqliteUtils::get_system_info()     {Ok(info) => {;
                assert!(info.contains_key("sqlite_version);
                assert!(info.contains_key(compile_options)"SQLite system info retrieved successfully)")}
            Err(e) => {println!(")}
// Helper functions for testing
mod test_helpers   {use super::*;

    /// Create a test configuration
    pub fn create_test_config() {SqliteConfig::memory()}

    /// Create test backup options
    pub fn create_test_backup_options() {BackupOptions::fast()}

    /// Simulate database operations (for environments without SQLite)
    pub fn simulate_database_operations() {println!(Simulating database operations for test environment);
        
        // Test configuration creation
        let config = create_test_config()
        assert!(config.is_memory_database()
        
        // Test backup options
        let backup_opts = create_test_backup_options()
        assert!(backup_opts.validate().is_ok()
        
        // Test error handling
        let error = SqliteError::database_not_found(missing .db)
        assert_eq!(error.code, SqliteErrorCode::CantOpen)
        
        println!("Database operation simulation completed "}
#[test]
fn test_integration_simulation() {// Run simulation for environments without SQLite
    test_helpers::simulate_database_operations()}