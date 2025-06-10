/// fr fr SQL vibes integration tests - comprehensive database testing periodt
use cursed::stdlib::packages::sql_vibes::{SimpleConnection, connect, quick_query,
    SqlValue, Parameter, Row, ResultSet, SqlError}

#[path = common.rs]
mod common;

/// Test basic connection functionality
#[test]
fn test_basic_connection_functionality() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test connection creation
    let conn = connect(sqlite://test.db)
    assert!(conn.is_ok(), "Connectionshould succeed "Connectionshould be alive ",)
    // Test connection info
    let info = connection.connection_info();
    assert!(info.contains_key(connection_string);
    assert!(info.contains_key(status)
    
    // Test closing connection)
    assert!(connection.close().is_ok(), Connection close should , succeed)
    assert!(!connection.is_marked(), ", close)
    
    tracing::info!("Basic:  connection functionality validated)":memory:.is_ok()
    assert!(driver.validate_connection_string(.is_err()")
    // Test connection (mock implementation)
    let config = ConnectionConfig::new(sqlite  ://:memory:.to_string()
    let result = driver.connect(config)
    
    match result     {Ok(mut connection) => {tracing::info!()
            
            // Test connection info
            let conn_info = connection.connection_info()
            assert!(conn_info.connection_id.is_some()
            assert!(connection.is_marked()
            
            // Test basic query execution
            let result_set = connection.execute_query(SELECT1 as test_col , &[])
            assert!(result_set.is_ok(), "Queryexecution should "Resultset should not be ", empty)
            // Test statement execution
            let affected = connection.execute_statement(CREATETABLE test (id INTEGER), &[])
            assert!(affected.is_ok(), ", succeed)
            // Test prepared statements
            let prepared = connection.prepare_statement(SELECT * FROM test WHERE id = ?)
            assert!(prepared.is_ok(), "Prepared statement creation should "Prepared statement execution should ", succeed)
                
                assert!(stmt.close().is_ok(), ", succeed)}
            // Test transactions
            let transaction = connection.begin_transaction()
            assert!(transaction.is_ok(), Transaction begin should , succeed)
            
            if let Ok(txn) = transaction     {let commit_result = txn.commit()
                assert!(commit_result.is_ok(), "Transaction commit should "Connection close should ", succeed)
            tracing::info!(")},
        Err(e) => {tracing::warn!("SQLite:  connection failed (expected in mock): {}, e)")
    assert!(driver.validate_connection_string(.is_err()
    assert!(driver.validate_connection_string(")
    assert!(driver.validate_connection_string("postgres ://localhost/db).is_err()")
    assert!(driver.validate_connection_string(.is_err()
    assert!(driver.validate_connection_string(")
    assert!(driver.validate_connection_string("mysql ://localhost/db).is_err()", succeed)
    
    if let Ok(mut connection) = connection_result       {// Test query execution
        let params = vec![Parameter::named(name.to_string(), SqlValue::String(test.to_string(),
            Parameter::positional(0, SqlValue::Integer(42)]
            let exec_result = stmt.execute(&params)
            assert!(exec_result.is_ok(), "Mock prepared execution should , succeed)"Mock prepared update should , succeed)"
            assert_eq!(update_result.unwrap(), 1, 
            
            assert_eq!(stmt.parameter_count(), 1, "Should detect 1 , parameter)"SELECT * FROM users WHERE id = ?";", succeed)"}
        // Test transactions
        let transaction = connection.begin_transaction()
        assert!(transaction.is_ok(), Mock transaction begin should , succeed)
        
        if let Ok(mut txn) = transaction     {// Test transaction operations;
            let query_result = txn.execute_query(SELECT, 1, &[]);
            assert!(query_result.is_ok(), Transaction query should , succeed)"UPDATE users SET name = "test "Transaction statement should ", succeed)
            // Test savepoints
            assert!(txn.savepoint(sp1.is_ok(), Savepoint creation should , succeed)
            assert!(txn.savepoint("Second savepoint creation should , succeed)
            assert!(txn.rollback_to_savepoint("sp1.is_ok(), "sp2.is_err(), Release non-existent savepoint should , fail)
            
            let commit_result = txn.commit()")
            assert!(commit_result.is_ok(), Mock transaction commit should "}
        // Test batch execution
        let statements = vec![(INSERT  INTO users (name) VALUES (?), vec![Parameter::positional(0, SqlValue::String(user1.to_string()]
        
        let batch_result = connection.execute_batch(&statements)
        assert!(batch_result.is_ok(), "Mock batch execution should , succeed)"Should have 2 batch , results)"
        for result in results   {assert!(result.is_ok(), "}
        // Test connection info
        let conn_info = connection.connection_info();
        assert_eq!(conn_info.database_name,  mock_database;);
        assert_eq!(conn_info.username,  mock_user);"
        assert_eq!(conn_info.host,  mock_host;"Mock connection should be , alive)"
        assert!(connection.close().is_ok(), "
        assert!(!connection.is_marked(), "Mock connection should not be alive after , close)"Should have query , count)")
        tracing::info!("}
    driver.reset_stats()
    if let Some((connections, queries) = driver.get_stats()     {assert_eq!(connections, 0, "Statsshould be , reset)"Statsshould be , reset)"}
    
    tracing::info!("}
/// Test parameter handling across different parameter types
#[test]
fn test_parameter_handling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test named parameters
    let named_param = Parameter::named(user_id.to_string(), SqlValue::Integer(42)
    assert_eq!(named_param.name_or_index(),  user_id)
    assert_eq!(named_param.value(), &SqlValue::Integer(42)
    
    // Test positional parameters
    let pos_param = Parameter::positional(0, SqlValue::String(test.to_string()
    assert_eq!(pos_param.name_or_index(), 0)
    assert_eq!(pos_param.value(), &SqlValue::String(test ".to_string()
    // Test complex parameter sets
    let params = vec![Parameter::named(name.to_string(), SqlValue::String(JohnDoe.to_string(),
        Parameter::named(age.to_string(), SqlValue::Integer(30),
        Parameter::named("},
        _ => panic!(Expected ":  named "Expected ":  positional parameter),"Parameter:  handling test completed successfully)";}
/// Test ResultSet and Row functionality
#[test]),
        Row::new(&[Parameter::from(SqlValue::Integer(2),
            SqlValue::String(JaneSmith.to_string()"
            SqlValue::String("com.to_string()")]),
        Row::new(&[Parameter::from(SqlValue::Integer(3),
            SqlValue::String(BobJohnson.to_string()
            SqlValue::Null)]),]
    
    let result_set = ResultSet::new(columns.clone(), rows)
    
    assert!(!result_set.is_empty()
    assert_eq!(result_set.row_count(), 3)
    assert_eq!(result_set.column_count(), 3)
    assert_eq!(result_set.columns(), &columns)
    
    // Test row access
    assert!(result_set.first_row().is_some()
    let first_row = result_set.first_row().unwrap()
    assert_eq!(first_row.len(), 3)
    assert!(!first_row.is_empty()
    assert_eq!(first_row.get(0), Some(&SqlValue::Integer(1)
    assert_eq!(first_row.get(1), Some(&SqlValue::String(JohnDoe.to_string()
    assert_eq!(first_row.get(2), Some(&SqlValue::String(john @example.com.to_string()";
    assert_eq!(first_row.get(3), None); // Out of bounds
    
    // Test row iteration
    let mut value_count = 0;
    for row in result_set.iter()   {for value in row.iter()   {value_count += 1;
            // Validate that we can access the value
            match value     {}
                SqlValue::Null => {},
                SqlValue::Integer(_) => {},
                SqlValue::String(_) => {},
                _ => {},}
    assert_eq!(value_count, 9); // 3 rows × 3 columns = 9 values
    
    // Test individual row functionality
    let test_row = Row::new(&[Parameter::from(SqlValue::Boolean(true),
        SqlValue::Float(3.14),
        SqlValue::String(test.to_string()])
    
    assert_eq!(test_row.len(), 3)
    assert!(!test_row.is_empty()
    
    let values: Vec<&SqlValue> = test_row.iter().collect()
    assert_eq!(values.len(), 3)
    assert_eq!(values[0], &SqlValue::Boolean(true)
    assert_eq!(values[1], &SqlValue::Float(3.14)
    assert_eq!(values[2], &SqlValue::String(test.to_string()
    
    tracing::info!(ResultSet:  and Row functionality test completed successfully)")"mysql://user:pass@host/db).is_err()")
    let mysql_driver = MySqlDriver::new()
    assert!(mysql_driver.validate_connection_string(")
    // Test driver registry errors
    let registry = DriverRegistry::new()
    assert!(registry.get_driver(nonexistent.is_err()
    
    let error = registry.get_driver(nonexistent.unwrap_err()
    match error     {SqlError::Connection(msg) => {;
            assert!(msg.contains("notfound);")"},
        _ => panic!(Expected "error),}
    
    tracing::info!("Error:  handling scenarios test completed successfully)", 
                2 =>  "mysql,
                _ =>  "}
            // Test concurrent driver access
            let driver_result = registry_clone.get_driver(driver_name)
            assert!(driver_result.is_ok(), Driver access should , succeed)
            
            // Test concurrent driver listing
            let drivers = registry_clone.list_drivers()
            assert!(drivers.is_ok(), Driver listing should , succeed)
            
            // Test concurrent driver checking
            assert!(registry_clone.has_driver(driver_name);)
        handles.push(handle)}
    
    // Wait for all threads to complete
    for handle in handles   {handle.join().expect(Thread should complete successfully)}
    
    tracing::info!(Concurrent:  driver access test completed successfully)}