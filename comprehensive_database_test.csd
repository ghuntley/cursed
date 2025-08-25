fr fr Comprehensive Database Driver Test
fr fr Tests all major database functionality with real implementations

yeet "vibez"
yeet "testz"
yeet "dbz"
yeet "database_complete"
yeet "database_drivers"

slay main() {
    vibez.spill("🔧 Starting comprehensive database implementation test...")
    
    fr fr Test utility function implementations
    test_utility_functions()
    
    fr fr Test connection pool functionality  
    test_connection_pooling()
    
    fr fr Test transaction management
    test_transaction_management()
    
    fr fr Test prepared statements
    test_prepared_statements()
    
    fr fr Test SQL parameter substitution
    test_sql_parameter_substitution()
    
    fr fr Test authentication parsing
    test_authentication_parsing()
    
    fr fr Test database schema operations
    test_schema_operations()
    
    fr fr Test driver registry
    test_driver_registry()
    
    vibez.spill("✅ Comprehensive database test completed successfully!")
}

slay test_utility_functions() {
    vibez.spill("📋 Testing utility functions...")
    
    fr fr Test connection ID generation
    sus id1 drip = dbz.generate_connection_id()
    sus id2 drip = dbz.generate_connection_id()
    testz.assert_not_equal_int(id1, id2, "Connection IDs should be unique")
    testz.assert_greater_than_int(id1, 0, "Connection ID should be positive")
    
    fr fr Test statement ID generation
    sus stmt1 drip = dbz.generate_statement_id()
    sus stmt2 drip = dbz.generate_statement_id()
    testz.assert_not_equal_int(stmt1, stmt2, "Statement IDs should be unique")
    testz.assert_greater_than_int(stmt1, 0, "Statement ID should be positive")
    
    fr fr Test current time functionality
    sus time1 drip = dbz.get_current_time_ms()
    sus time2 drip = dbz.get_current_time_ms()
    testz.assert_greater_than_or_equal_int(time2, time1, "Time should be monotonic")
    testz.assert_greater_than_int(time1, 1640000000000, "Time should be reasonable")
    
    vibez.spill("✅ Utility functions tests passed")
}

slay test_connection_pooling() {
    vibez.spill("🔗 Testing connection pooling...")
    
    fr fr Create database configuration
    sus config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
        driver_type: "postgresql",
        host: "localhost", 
        port: 5432,
        database_name: "testdb",
        username: "testuser",
        password: "testpass",
        ssl_enabled: based,
        timeout_seconds: 30,
        max_connections: 10,
        connection_lifetime_minutes: 60
    }
    
    fr fr Initialize connection pool
    sus pool database_complete.ConnectionPool = database_complete.init_connection_pool(config)
    testz.assert_equal_str(pool.driver.driver_name, "PostgreSQL", "Should create PostgreSQL driver")
    testz.assert_equal_int(pool.max_size, 10, "Pool max size should match config")
    testz.assert_equal_lit(pool.is_initialized, based, "Pool should be initialized")
    
    fr fr Test getting connection from pool
    sus conn_id tea = database_complete.get_connection(pool)
    testz.assert_not_equal_str(conn_id, "", "Should get valid connection ID")
    
    fr fr Test returning connection to pool
    sus returned lit = database_complete.return_connection(pool, conn_id)
    testz.assert_equal_lit(returned, based, "Should successfully return connection")
    
    vibez.spill("✅ Connection pooling tests passed")
}

slay test_transaction_management() {
    vibez.spill("🔄 Testing transaction management...")
    
    fr fr Create a mock connection
    sus conn_id tea = "test_conn_123"
    
    fr fr Begin transaction
    sus tx database_complete.Transaction = database_complete.begin_transaction(conn_id, "READ_COMMITTED")
    testz.assert_equal_str(tx.connection_id, conn_id, "Transaction should reference correct connection")
    testz.assert_equal_str(tx.isolation_level, "READ_COMMITTED", "Isolation level should match")
    testz.assert_equal_lit(tx.is_active, based, "Transaction should be active")
    testz.assert_equal_lit(tx.rollback_on_error, based, "Should rollback on error by default")
    
    fr fr Test commit
    sus commit_success lit = database_complete.commit_transaction(tx)
    testz.assert_equal_lit(commit_success, based, "Should successfully commit transaction")
    
    fr fr Test rollback
    sus tx2 database_complete.Transaction = database_complete.begin_transaction(conn_id, "SERIALIZABLE")
    sus rollback_success lit = database_complete.rollback_transaction(tx2)
    testz.assert_equal_lit(rollback_success, based, "Should successfully rollback transaction")
    
    vibez.spill("✅ Transaction management tests passed")
}

slay test_prepared_statements() {
    vibez.spill("📝 Testing prepared statements...")
    
    sus conn_id tea = "test_conn_456"
    sus sql tea = "SELECT * FROM users WHERE id = ? AND name = ?"
    
    fr fr Prepare statement
    sus stmt database_complete.PreparedStatement = database_complete.prepare_statement(conn_id, sql)
    testz.assert_not_equal_str(stmt, "", "Should generate statement ID")
    testz.assert_greater_than_int(stringz.length(stmt), 0, "Statement ID should not be empty")
    
    fr fr Test parameter counting  
    sus param_count drip = dbz.count_sql_parameters(sql)
    testz.assert_equal_int(param_count, 2, "Should count 2 parameters")
    
    fr fr Test parameter substitution
    sus params [tea] = ["123", "John Doe"]
    sus substituted tea = dbz.substitute_sql_parameters(sql, params)
    testz.assert_contains_str(substituted, "'123'", "Should substitute first parameter")
    testz.assert_contains_str(substituted, "'John Doe'", "Should substitute second parameter")
    testz.assert_not_contains_str(substituted, "?", "Should not contain unsubstituted parameters")
    
    vibez.spill("✅ Prepared statements tests passed")
}

slay test_sql_parameter_substitution() {
    vibez.spill("🔧 Testing SQL parameter substitution...")
    
    fr fr Test simple substitution
    sus sql1 tea = "SELECT * FROM users WHERE id = ?"
    sus params1 [tea] = ["42"]
    sus result1 tea = dbz.substitute_sql_parameters(sql1, params1)
    sus expected1 tea = "SELECT * FROM users WHERE id = '42'"
    testz.assert_equal_str(result1, expected1, "Simple parameter substitution")
    
    fr fr Test multiple parameters
    sus sql2 tea = "INSERT INTO users (name, age) VALUES (?, ?)"
    sus params2 [tea] = ["Alice", "30"]
    sus result2 tea = dbz.substitute_sql_parameters(sql2, params2)
    testz.assert_contains_str(result2, "'Alice'", "Should contain first parameter")
    testz.assert_contains_str(result2, "'30'", "Should contain second parameter")
    
    fr fr Test SQL injection prevention
    sus sql3 tea = "SELECT * FROM users WHERE name = ?"
    sus params3 [tea] = ["'; DROP TABLE users; --"]
    sus result3 tea = dbz.substitute_sql_parameters(sql3, params3)
    testz.assert_contains_str(result3, "''", "Should escape single quotes")
    
    vibez.spill("✅ SQL parameter substitution tests passed")
}

slay test_authentication_parsing() {
    vibez.spill("🔐 Testing authentication parsing...")
    
    fr fr Test integer parsing functions
    sus test_data tea = char(0) + char(1) + char(2) + char(3)
    sus parsed_int drip = dbz.parse_int32_be(test_data)
    testz.assert_greater_than_int(parsed_int, 0, "Should parse big-endian integer")
    
    sus test_data_le tea = char(3) + char(2) + char(1)
    sus parsed_le drip = dbz.parse_int24_le(test_data_le)
    testz.assert_greater_than_int(parsed_le, 0, "Should parse little-endian integer")
    
    fr fr Test byte parsing
    sus test_byte tea = char(65)  fr fr ASCII 'A'
    sus parsed_byte drip = dbz.parse_int8(test_byte)
    testz.assert_equal_int(parsed_byte, 65, "Should parse single byte")
    
    vibez.spill("✅ Authentication parsing tests passed")
}

slay test_schema_operations() {
    vibez.spill("🏗️ Testing database schema operations...")
    
    fr fr Create mock connection
    sus config database_complete.DatabaseConfig = database_complete.DatabaseConfig{
        driver_type: "sqlite",
        host: "",
        port: 0,
        database_name: ":memory:",
        username: "",
        password: "",
        ssl_enabled: cringe,
        timeout_seconds: 10,
        max_connections: 1,
        connection_lifetime_minutes: 30
    }
    
    sus conn_id tea = database_complete.db_connect(config)
    testz.assert_not_equal_str(conn_id, "", "Should create database connection")
    
    fr fr Test table creation
    sus columns [tea] = ["id INTEGER PRIMARY KEY", "name TEXT NOT NULL", "email TEXT UNIQUE"]
    sus created lit = database_complete.create_table(conn_id, "test_users", columns)
    testz.assert_equal_lit(created, based, "Should create table successfully")
    
    fr fr Test high-level operations
    sus insert_columns [tea] = ["name", "email"]
    sus insert_values [tea] = ["Test User", "test@example.com"]
    sus insert_result database_complete.QueryResult = database_complete.db_insert(conn_id, "test_users", insert_columns, insert_values)
    testz.assert_equal_lit(insert_result.has_more, cringe, "Insert should complete")
    
    vibez.spill("✅ Schema operations tests passed")
}

slay test_driver_registry() {
    vibez.spill("📦 Testing driver registry...")
    
    fr fr Create registry
    sus registry database_drivers.DriverRegistry = database_drivers.create_driver_registry()
    testz.assert_equal_int(registry.next_connection_id, 1, "Registry should start with connection ID 1")
    testz.assert_equal_int(stringz.length(registry.drivers), 0, "Registry should start empty")
    
    fr fr Register drivers
    sus postgres_registered lit = database_drivers.register_driver(&registry, "PostgreSQL", "14.0", based, based)
    testz.assert_equal_lit(postgres_registered, based, "Should register PostgreSQL driver")
    
    sus mysql_registered lit = database_drivers.register_driver(&registry, "MySQL", "8.0", based, based) 
    testz.assert_equal_lit(mysql_registered, based, "Should register MySQL driver")
    
    fr fr Test driver count
    sus driver_count normie = database_drivers.driver_count(&registry)
    testz.assert_equal_int(driver_count, 2, "Should have 2 registered drivers")
    
    fr fr Test driver lookup
    sus postgres_driver database_drivers.DriverInfo = database_drivers.get_driver(&registry, "PostgreSQL")
    testz.assert_equal_str(postgres_driver.name, "PostgreSQL", "Should find PostgreSQL driver")
    testz.assert_equal_str(postgres_driver.version, "14.0", "Driver version should match")
    
    vibez.spill("✅ Driver registry tests passed")
}

fr fr Run the main test
main()
