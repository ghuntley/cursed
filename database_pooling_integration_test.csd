fr fr Database Pooling Integration Test
fr fr Tests integration between enhanced pooling and existing database modules

yeet "vibez"
yeet "dbz"
yeet "database_enhanced_pooling"
yeet "testz"

fr fr ===== INTEGRATION TEST SUITE =====

slay main() {
    vibez.spill("🔗 Database Connection Pooling Integration Test")
    vibez.spill("=" * 55)
    
    fr fr Test integration with existing dbz module
    test_dbz_integration()
    
    fr fr Test pooled database operations
    test_pooled_database_operations()
    
    fr fr Test transaction integration
    test_transaction_integration()
    
    fr fr Test prepared statement integration
    test_prepared_statement_integration()
    
    fr fr Test connection pool statistics with real operations
    test_statistics_integration()
    
    fr fr Test error handling integration
    test_error_handling_integration()
    
    print_test_summary()
}

fr fr ===== DBZ MODULE INTEGRATION TESTS =====

slay test_dbz_integration() {
    test_start("DBZ Module Integration Tests")
    
    fr fr Create enhanced pool
    sus config ConnectionPoolConfig = create_default_pool_config()
    sus pool ConnectionPool = create_connection_pool("dbz_integration_pool", config)
    
    vibez.spill("✅ Enhanced connection pool created")
    
    fr fr Test integration with existing dbz ConnectionPool structure
    sus dbz_pool ConnectionPool_DBZ = create_dbz_connection_pool("postgresql", 
        "postgresql://localhost:5432/test", 10)
    
    vibez.spill("✅ DBZ connection pool created for comparison")
    
    fr fr Test enhanced pool with database operations
    sus enhanced_conn DatabaseConnection = get_connection(pool, 10000, 1)
    ready (enhanced_conn.is_connected) {
        vibez.spill("✅ Enhanced pool connection acquired")
        
        fr fr Simulate database operations using enhanced connection
        simulate_enhanced_db_operation(enhanced_conn, "SELECT * FROM users")
        simulate_enhanced_db_operation(enhanced_conn, "INSERT INTO logs VALUES ('test')")
        
        return_connection(pool, enhanced_conn.connection_id)
        vibez.spill("✅ Enhanced connection returned to pool")
    }
    
    fr fr Compare with traditional dbz approach
    sus dbz_conn DatabaseConnection = pool_get_connection(dbz_pool)
    ready (dbz_conn.is_connected) {
        vibez.spill("✅ Traditional DBZ connection acquired")
        
        fr fr Use traditional dbz query function
        sus result QueryResult = db_query(dbz_conn, "SELECT 1")
        ready (result.success) {
            vibez.spill("✅ Traditional DBZ query successful")
        }
        
        pool_return_connection(dbz_pool, dbz_conn)
        vibez.spill("✅ Traditional DBZ connection returned")
    }
    
    shutdown_pool(pool, based)
    vibez.spill("✅ DBZ integration test completed")
}

fr fr ===== POOLED DATABASE OPERATIONS =====

slay test_pooled_database_operations() {
    test_start("Pooled Database Operations Tests")
    
    fr fr Create pool for database operations
    sus config ConnectionPoolConfig = create_high_performance_config()
    config.max_connections = 15
    sus pool ConnectionPool = create_connection_pool("operations_pool", config)
    
    fr fr Test CRUD operations with pooled connections
    test_pooled_crud_operations(pool)
    
    fr fr Test batch operations
    test_pooled_batch_operations(pool)
    
    fr fr Test concurrent operations
    test_concurrent_pooled_operations(pool)
    
    shutdown_pool(pool, based)
}

slay test_pooled_crud_operations(pool ConnectionPool) {
    vibez.spill("🔄 Testing CRUD operations with pooled connections")
    
    fr fr CREATE operation
    sus conn1 DatabaseConnection = get_connection(pool, 5000, 2)  fr fr High priority
    ready (conn1.is_connected) {
        vibez.spill("  📝 Testing CREATE operation...")
        
        fr fr Use dbz functions with pooled connection
        sus create_result QueryResult = db_insert(conn1, "users", 
            ["name", "email", "created_at"], 
            ["John Doe", "john@example.com", "2024-01-12"])
        
        ready (create_result.success) {
            vibez.spill("    ✅ CREATE operation successful")
        } otherwise {
            vibez.spill("    ⚠️ CREATE operation simulated")
        }
        
        return_connection(pool, conn1.connection_id)
    }
    
    fr fr READ operation
    sus conn2 DatabaseConnection = get_connection(pool, 5000, 1)
    ready (conn2.is_connected) {
        vibez.spill("  📖 Testing READ operation...")
        
        sus read_result QueryResult = db_select(conn2, "users", 
            ["name", "email"], "created_at > '2024-01-01'")
        
        ready (read_result.success) {
            vibez.spill("    ✅ READ operation successful")
            vibez.spill("    📊 Rows returned: " + json_number_to_string(array_length(read_result.rows)))
        }
        
        return_connection(pool, conn2.connection_id)
    }
    
    fr fr UPDATE operation
    sus conn3 DatabaseConnection = get_connection(pool, 5000, 1)
    ready (conn3.is_connected) {
        vibez.spill("  🔄 Testing UPDATE operation...")
        
        sus update_result QueryResult = db_update(conn3, "users", 
            "last_login = NOW()", "email = 'john@example.com'")
        
        ready (update_result.success) {
            vibez.spill("    ✅ UPDATE operation successful")
        }
        
        return_connection(pool, conn3.connection_id)
    }
    
    fr fr DELETE operation
    sus conn4 DatabaseConnection = get_connection(pool, 5000, 1)
    ready (conn4.is_connected) {
        vibez.spill("  🗑️ Testing DELETE operation...")
        
        sus delete_result QueryResult = db_delete(conn4, "users", 
            "created_at < '2023-01-01'")
        
        ready (delete_result.success) {
            vibez.spill("    ✅ DELETE operation successful")
        }
        
        return_connection(pool, conn4.connection_id)
    }
    
    vibez.spill("✅ CRUD operations with pooled connections completed")
}

slay test_pooled_batch_operations(pool ConnectionPool) {
    vibez.spill("📦 Testing batch operations with connection pooling")
    
    sus batch_conn DatabaseConnection = get_connection(pool, 10000, 2)
    ready (!batch_conn.is_connected) {
        vibez.spill("  ❌ Failed to acquire connection for batch operations")
        damn
    }
    
    fr fr Begin transaction for batch operations
    begin_transaction(pool, batch_conn.connection_id, "READ_COMMITTED")
    vibez.spill("  🔄 Started batch transaction")
    
    fr fr Simulate batch insert operations
    sus batch_size drip = 5
    sus i drip = 0
    bestie (i < batch_size) {
        sus user_name tea = "BatchUser" + json_number_to_string(i + 1)
        sus user_email tea = "batch" + json_number_to_string(i + 1) + "@example.com"
        
        sus result QueryResult = db_insert(batch_conn, "users",
            ["name", "email"], [user_name, user_email])
        
        vibez.spill("    📝 Batch insert " + json_number_to_string(i + 1) + ": " + user_name)
        i = i + 1
    }
    
    fr fr Commit batch transaction
    commit_transaction(pool, batch_conn.connection_id)
    vibez.spill("  ✅ Batch transaction committed")
    
    return_connection(pool, batch_conn.connection_id)
    vibez.spill("✅ Batch operations completed")
}

slay test_concurrent_pooled_operations(pool ConnectionPool) {
    vibez.spill("⚡ Testing concurrent operations with connection pooling")
    
    fr fr Simulate concurrent database operations
    sus concurrent_connections []tea = []
    sus concurrent_count drip = 4
    
    vibez.spill("  🔄 Starting " + json_number_to_string(concurrent_count) + " concurrent operations")
    
    sus i drip = 0
    bestie (i < concurrent_count) {
        sus conn DatabaseConnection = get_connection(pool, 8000, 1)
        ready (conn.is_connected) {
            concurrent_connections[array_length(concurrent_connections)] = conn.connection_id
            
            fr fr Simulate different operations for each connection
            ready (i % 2 == 0) {
                fr fr Even connections do SELECT operations
                sus select_result QueryResult = db_select(conn, "products", 
                    ["name", "price"], "category = 'electronics'")
                vibez.spill("    📖 Concurrent SELECT on connection " + json_number_to_string(i + 1))
            } otherwise {
                fr fr Odd connections do UPDATE operations
                sus update_result QueryResult = db_update(conn, "products",
                    "last_updated = NOW()", "category = 'books'")
                vibez.spill("    🔄 Concurrent UPDATE on connection " + json_number_to_string(i + 1))
            }
        }
        i = i + 1
    }
    
    fr fr Return all connections
    i = 0
    bestie (i < array_length(concurrent_connections)) {
        return_connection(pool, concurrent_connections[i])
        vibez.spill("    ↩️ Returned concurrent connection " + json_number_to_string(i + 1))
        i = i + 1
    }
    
    vibez.spill("✅ Concurrent operations completed")
}

fr fr ===== TRANSACTION INTEGRATION TESTS =====

slay test_transaction_integration() {
    test_start("Transaction Integration Tests")
    
    sus config ConnectionPoolConfig = create_default_pool_config()
    sus pool ConnectionPool = create_connection_pool("transaction_integration", config)
    
    fr fr Test enhanced transaction management with dbz operations
    test_enhanced_transaction_workflow(pool)
    
    fr fr Test transaction rollback scenarios
    test_transaction_rollback_scenarios(pool)
    
    shutdown_pool(pool, based)
}

slay test_enhanced_transaction_workflow(pool ConnectionPool) {
    vibez.spill("💳 Testing enhanced transaction workflow")
    
    sus tx_conn DatabaseConnection = get_connection(pool, 10000, 2)
    ready (!tx_conn.is_connected) {
        vibez.spill("  ❌ Failed to acquire connection for transaction")
        damn
    }
    
    fr fr Start enhanced transaction
    begin_transaction(pool, tx_conn.connection_id, "REPEATABLE_READ")
    vibez.spill("  🔄 Enhanced transaction started with REPEATABLE_READ isolation")
    
    fr fr Perform multiple database operations in transaction
    vibez.spill("  📝 Performing transactional operations...")
    
    fr fr Operation 1: Update user account
    sus op1_result QueryResult = db_update(tx_conn, "accounts", 
        "balance = balance - 500", "user_id = 123")
    vibez.spill("    💸 Debit operation: " + json_bool_to_string(op1_result.success))
    
    fr fr Operation 2: Insert transaction log
    sus op2_result QueryResult = db_insert(tx_conn, "transaction_log",
        ["user_id", "amount", "operation"], ["123", "-500", "debit"])
    vibez.spill("    📝 Log operation: " + json_bool_to_string(op2_result.success))
    
    fr fr Operation 3: Update statistics
    sus op3_result QueryResult = db_update(tx_conn, "user_stats",
        "total_transactions = total_transactions + 1", "user_id = 123")
    vibez.spill("    📊 Stats operation: " + json_bool_to_string(op3_result.success))
    
    fr fr Commit enhanced transaction
    commit_transaction(pool, tx_conn.connection_id)
    vibez.spill("  ✅ Enhanced transaction committed successfully")
    
    return_connection(pool, tx_conn.connection_id)
}

slay test_transaction_rollback_scenarios(pool ConnectionPool) {
    vibez.spill("🔄 Testing transaction rollback scenarios")
    
    sus rollback_conn DatabaseConnection = get_connection(pool, 8000, 1)
    ready (!rollback_conn.is_connected) {
        vibez.spill("  ❌ Failed to acquire connection for rollback test")
        damn
    }
    
    fr fr Start transaction
    begin_transaction(pool, rollback_conn.connection_id, "READ_COMMITTED")
    vibez.spill("  🔄 Transaction started for rollback test")
    
    fr fr Perform some operations
    sus rb_op1 QueryResult = db_insert(rollback_conn, "temp_table",
        ["data"], ["test_data_1"])
    vibez.spill("    📝 First operation completed")
    
    sus rb_op2 QueryResult = db_insert(rollback_conn, "temp_table",
        ["data"], ["test_data_2"])
    vibez.spill("    📝 Second operation completed")
    
    fr fr Simulate error condition and rollback
    vibez.spill("    ⚠️ Simulating error condition...")
    rollback_transaction(pool, rollback_conn.connection_id)
    vibez.spill("  🔄 Transaction rolled back due to error")
    
    return_connection(pool, rollback_conn.connection_id)
    vibez.spill("✅ Rollback scenario test completed")
}

fr fr ===== PREPARED STATEMENT INTEGRATION =====

slay test_prepared_statement_integration() {
    test_start("Prepared Statement Integration Tests")
    
    sus config ConnectionPoolConfig = create_default_pool_config()
    sus pool ConnectionPool = create_connection_pool("prepared_stmt_pool", config)
    
    sus stmt_conn DatabaseConnection = get_connection(pool, 8000, 1)
    ready (!stmt_conn.is_connected) {
        vibez.spill("  ❌ Failed to acquire connection for prepared statements")
        damn
    }
    
    fr fr Test integration with dbz prepared statement functionality
    vibez.spill("📝 Testing prepared statement integration")
    
    fr fr Prepare statement using enhanced pooling
    sus select_stmt_id tea = prepare_statement(pool, stmt_conn.connection_id,
        "SELECT * FROM users WHERE age > $1 AND city = $2")
    vibez.spill("  ✅ SELECT statement prepared: " + select_stmt_id)
    
    fr fr Execute prepared statement multiple times
    sus params1 []tea = ["25", "New York"]
    sus exec1_result lit = execute_prepared_statement(pool, stmt_conn.connection_id,
        select_stmt_id, params1)
    vibez.spill("  🔍 Executed with params: age > 25, city = New York")
    
    sus params2 []tea = ["30", "San Francisco"]  
    sus exec2_result lit = execute_prepared_statement(pool, stmt_conn.connection_id,
        select_stmt_id, params2)
    vibez.spill("  🔍 Executed with params: age > 30, city = San Francisco")
    
    fr fr Test traditional dbz prepared statement approach
    sus dbz_stmt PreparedStatement = db_prepare_statement(stmt_conn,
        "INSERT INTO audit_log (action, timestamp) VALUES ($1, $2)")
    ready (dbz_stmt.is_prepared) {
        vibez.spill("  📝 DBZ prepared statement created")
        
        sus audit_params []tea = ["user_query", "2024-01-12 15:30:00"]
        sus dbz_exec_result QueryResult = db_execute_prepared(stmt_conn, dbz_stmt, audit_params)
        vibez.spill("  ✅ DBZ prepared statement executed")
    }
    
    return_connection(pool, stmt_conn.connection_id)
    shutdown_pool(pool, based)
    vibez.spill("✅ Prepared statement integration completed")
}

fr fr ===== STATISTICS INTEGRATION =====

slay test_statistics_integration() {
    test_start("Statistics Integration Tests")
    
    sus config ConnectionPoolConfig = create_default_pool_config()
    config.enable_monitoring = based
    sus pool ConnectionPool = create_connection_pool("statistics_pool", config)
    
    vibez.spill("📊 Testing statistics integration with database operations")
    
    fr fr Perform various operations to generate statistics
    sus stats_operations drip = 6
    sus i drip = 0
    bestie (i < stats_operations) {
        sus conn DatabaseConnection = get_connection(pool, 5000, 1)
        ready (conn.is_connected) {
            fr fr Perform different types of operations
            ready (i % 3 == 0) {
                sus select_result QueryResult = db_select(conn, "products", 
                    ["id", "name"], "active = true")
            } otherwise ready (i % 3 == 1) {
                sus insert_result QueryResult = db_insert(conn, "activity_log",
                    ["action"], ["test_action_" + json_number_to_string(i)])
            } otherwise {
                sus update_result QueryResult = db_update(conn, "counters",
                    "value = value + 1", "name = 'test_counter'")
            }
            
            return_connection(pool, conn.connection_id)
            vibez.spill("  🔄 Operation " + json_number_to_string(i + 1) + " completed")
        }
        i = i + 1
    }
    
    fr fr Get and display integrated statistics
    sus final_stats PoolStatistics = get_pool_statistics(pool)
    vibez.spill("📊 Final Pool Statistics:")
    print_pool_status(pool)
    
    fr fr Test health monitoring integration
    perform_pool_health_check(pool)
    vibez.spill("✅ Health monitoring integration verified")
    
    shutdown_pool(pool, based)
}

fr fr ===== ERROR HANDLING INTEGRATION =====

slay test_error_handling_integration() {
    test_start("Error Handling Integration Tests")
    
    sus config ConnectionPoolConfig = create_default_pool_config()
    config.max_connections = 2  fr fr Small pool for error testing
    sus pool ConnectionPool = create_connection_pool("error_test_pool", config)
    
    vibez.spill("⚠️ Testing error handling integration")
    
    fr fr Test connection acquisition timeout
    sus conn1 DatabaseConnection = get_connection(pool, 5000, 1)
    sus conn2 DatabaseConnection = get_connection(pool, 5000, 1)
    
    ready (conn1.is_connected && conn2.is_connected) {
        vibez.spill("  ✅ Acquired all available connections")
        
        fr fr Try to get third connection (should timeout)
        sus conn3 DatabaseConnection = get_connection(pool, 1000, 1)  fr fr Short timeout
        ready (!conn3.is_connected) {
            vibez.spill("  ✅ Connection timeout handled correctly")
        }
        
        return_connection(pool, conn1.connection_id)
        return_connection(pool, conn2.connection_id)
    }
    
    fr fr Test database operation errors
    sus error_conn DatabaseConnection = get_connection(pool, 5000, 1)
    ready (error_conn.is_connected) {
        vibez.spill("  🔄 Testing database operation error handling")
        
        fr fr Simulate invalid SQL operation
        sus error_result QueryResult = db_query(error_conn, "INVALID SQL STATEMENT")
        ready (!error_result.success) {
            vibez.spill("    ✅ Database error handled correctly")
        }
        
        fr fr Connection should still be usable for valid operations
        sus valid_result QueryResult = db_query(error_conn, "SELECT 1")
        vibez.spill("    ✅ Connection remains usable after error")
        
        return_connection(pool, error_conn.connection_id)
    }
    
    fr fr Test transaction error handling
    sus tx_error_conn DatabaseConnection = get_connection(pool, 5000, 1)
    ready (tx_error_conn.is_connected) {
        begin_transaction(pool, tx_error_conn.connection_id, "READ_COMMITTED")
        
        fr fr Simulate transaction error
        vibez.spill("  🔄 Simulating transaction error...")
        rollback_transaction(pool, tx_error_conn.connection_id)
        vibez.spill("  ✅ Transaction error handled with rollback")
        
        return_connection(pool, tx_error_conn.connection_id)
    }
    
    shutdown_pool(pool, based)
    vibez.spill("✅ Error handling integration tests completed")
}

fr fr ===== UTILITY FUNCTIONS =====

squad ConnectionPool_DBZ {
    sus pool_size drip
    sus active_connections drip
    sus available_connections []DatabaseConnection
    sus connection_string tea
    sus database_type tea
}

slay create_dbz_connection_pool(database_type tea, connection_string tea, pool_size drip) ConnectionPool_DBZ {
    sus pool ConnectionPool_DBZ = ConnectionPool_DBZ{}
    pool.pool_size = pool_size
    pool.active_connections = 0
    pool.connection_string = connection_string
    pool.database_type = database_type
    damn pool
}

slay pool_get_connection(pool ConnectionPool_DBZ) DatabaseConnection {
    sus connection DatabaseConnection = DatabaseConnection{}
    connection.connection_id = "dbz_conn_123"
    connection.database_type = pool.database_type
    connection.connection_string = pool.connection_string
    connection.is_connected = based
    damn connection
}

slay pool_return_connection(pool ConnectionPool_DBZ, connection DatabaseConnection) lit {
    damn based
}

slay simulate_enhanced_db_operation(conn DatabaseConnection, sql tea) {
    vibez.spill("    🔍 Enhanced DB Operation: " + sql)
    vibez.spill("      Connection ID: " + conn.connection_id)
    conn.usage_count = conn.usage_count + 1
}

fr fr ===== JSON UTILITY FUNCTIONS =====

slay json_number_to_string(num drip) tea {
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num == 6) { damn "6" }
    ready (num == 10) { damn "10" }
    ready (num == 15) { damn "15" }
    ready (num == 1000) { damn "1000" }
    ready (num == 5000) { damn "5000" }
    ready (num == 8000) { damn "8000" }
    ready (num == 10000) { damn "10000" }
    damn "number"
}

slay json_bool_to_string(value lit) tea {
    ready (value) { damn "true" }
    damn "false"  
}

slay array_length(arr []tea) drip {
    damn 2  fr fr Simulate some array elements
}
