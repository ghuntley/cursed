yeet "testz"
yeet "database"

test_start("database Tests")

// Test Database Configuration Creation
test_case("Database Configuration Creation") {
    sus config DatabaseConfig = {
        db_type: DB_POSTGRES,
        host: "localhost",
        port: 5432,
        database: "test_db",
        username: "test_user",
        password: "test_pass",
        connection_string: "postgresql://test_user:test_pass@localhost:5432/test_db",
        pool_size: 10,
        timeout: 30
    }
    
    assert_eq_int(config.db_type, DB_POSTGRES)
    assert_eq_string(config.host, "localhost")
    assert_eq_int(config.port, 5432)
    assert_eq_string(config.database, "test_db")
    assert_eq_int(config.pool_size, 10)
}

// Test Connection Pool Creation
test_case("Connection Pool Creation") {
    sus config DatabaseConfig = {
        db_type: DB_MYSQL,
        host: "127.0.0.1",
        port: 3306,
        database: "cursed_db",
        username: "cursed_user",
        password: "cursed_pass",
        connection_string: "mysql://cursed_user:cursed_pass@127.0.0.1:3306/cursed_db",
        pool_size: 5,
        timeout: 60
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    
    assert_eq_int(pool.max_connections, 5)
    assert_eq_int(pool.current_connections, 0)
    assert_eq_int(pool.config.db_type, DB_MYSQL)
}

// Test Connection Acquisition and Release
test_case("Connection Pool - Acquire and Release") {
    sus config DatabaseConfig = {
        db_type: DB_SQLITE,
        host: "",
        port: 0,
        database: ":memory:",
        username: "",
        password: "",
        connection_string: "sqlite://:memory:",
        pool_size: 3,
        timeout: 30
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    sus connection tea = acquire_connection(pool)
    
    assert(string_length(connection) > 0)
    assert_eq_int(pool.current_connections, 1)
    
    release_connection(pool, connection)
    assert_eq_int(pool.current_connections, 0)
}

// Test Query Execution
test_case("Query Execution - SELECT") {
    sus config DatabaseConfig = {
        db_type: DB_SQLITE,
        host: "",
        port: 0,
        database: ":memory:",
        username: "",
        password: "",
        connection_string: "sqlite://:memory:",
        pool_size: 1,
        timeout: 30
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    sus connection tea = acquire_connection(pool)
    
    sus query tea = "SELECT 1 as test_column, 'hello' as message"
    sus result QueryResult = execute_query(connection, query)
    
    assert(result.success)
    assert_eq_int(len(result.rows), 1)
    assert_eq_int(len(result.columns), 2)
    assert_eq_string(result.columns[0], "test_column")
    assert_eq_string(result.columns[1], "message")
    
    release_connection(pool, connection)
}

// Test Transaction Management
test_case("Transaction Management") {
    sus config DatabaseConfig = {
        db_type: DB_POSTGRES,
        host: "localhost",
        port: 5432,
        database: "test_db",
        username: "test_user",
        password: "test_pass",
        connection_string: "postgresql://test_user:test_pass@localhost:5432/test_db",
        pool_size: 1,
        timeout: 30
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    sus connection tea = acquire_connection(pool)
    
    sus transaction Transaction = begin_transaction(connection, "READ_COMMITTED")
    
    assert(transaction.is_active)
    assert_eq_string(transaction.isolation_level, "READ_COMMITTED")
    assert(string_length(transaction.transaction_id) > 0)
    
    sus commit_result lit = commit_transaction(transaction)
    assert(commit_result)
    
    release_connection(pool, connection)
}

// Test Connection String Building
test_case("Connection String Building") {
    sus postgres_config DatabaseConfig = {
        db_type: DB_POSTGRES,
        host: "db.example.com",
        port: 5432,
        database: "production",
        username: "admin",
        password: "secret123",
        connection_string: "",
        pool_size: 20,
        timeout: 45
    }
    
    sus connection_string tea = build_connection_string(postgres_config)
    assert(string_contains(connection_string, "postgresql://"))
    assert(string_contains(connection_string, "admin"))
    assert(string_contains(connection_string, "db.example.com"))
    assert(string_contains(connection_string, "5432"))
    assert(string_contains(connection_string, "production"))
}

// Test Error Handling
test_case("Error Handling - Invalid Query") {
    sus config DatabaseConfig = {
        db_type: DB_SQLITE,
        host: "",
        port: 0,
        database: ":memory:",
        username: "",
        password: "",
        connection_string: "sqlite://:memory:",
        pool_size: 1,
        timeout: 30
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    sus connection tea = acquire_connection(pool)
    
    sus invalid_query tea = "INVALID SQL SYNTAX HERE"
    sus result QueryResult = execute_query(connection, invalid_query)
    
    assert(!result.success)
    assert(string_length(result.error_message) > 0)
    assert_eq_int(len(result.rows), 0)
    
    release_connection(pool, connection)
}

// Test Pool Overflow Handling
test_case("Connection Pool - Overflow Handling") {
    sus config DatabaseConfig = {
        db_type: DB_MYSQL,
        host: "localhost",
        port: 3306,
        database: "test_db",
        username: "test_user",
        password: "test_pass",
        connection_string: "mysql://test_user:test_pass@localhost:3306/test_db",
        pool_size: 2,
        timeout: 5
    }
    
    sus pool ConnectionPool = create_connection_pool(config)
    
    sus conn1 tea = acquire_connection(pool)
    sus conn2 tea = acquire_connection(pool)
    
    assert_eq_int(pool.current_connections, 2)
    
    // This should timeout since pool is full
    sus start_time drip = get_current_time_ms()
    sus conn3 tea = acquire_connection_with_timeout(pool, 1000)
    sus elapsed_time drip = get_current_time_ms() - start_time
    
    assert(elapsed_time >= 1000)
    assert_eq_string(conn3, "")  // Should be empty due to timeout
    
    release_connection(pool, conn1)
    release_connection(pool, conn2)
}

// Test Database Type Validation
test_case("Database Type Validation") {
    assert(is_valid_database_type(DB_POSTGRES))
    assert(is_valid_database_type(DB_MYSQL))
    assert(is_valid_database_type(DB_SQLITE))
    assert(!is_valid_database_type(999))  // Invalid type
    
    sus type_name tea = get_database_type_name(DB_POSTGRES)
    assert_eq_string(type_name, "PostgreSQL")
    
    type_name = get_database_type_name(DB_MYSQL)
    assert_eq_string(type_name, "MySQL")
    
    type_name = get_database_type_name(DB_SQLITE)
    assert_eq_string(type_name, "SQLite")
}

print_test_summary()
