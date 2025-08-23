fr fr Comprehensive Database Driver Testing
fr fr Tests PostgreSQL, MySQL, and SQLite connectivity with real drivers

yeet "vibez"
yeet "stringz"
yeet "testz"
yeet "dbz"

fr fr ===== Test Configuration =====

fr fr These can be overridden with environment variables
sus DEFAULT_POSTGRES_CONNECTION tea = "host=localhost port=5432 dbname=cursed_test user=postgres password=cursed123"
sus DEFAULT_MYSQL_CONNECTION tea = "host=localhost port=3306 database=cursed_test user=root password=cursed123"
sus DEFAULT_SQLITE_CONNECTION tea = ":memory:"

fr fr ===== PostgreSQL Tests =====

slay test_postgres_basic_connectivity() {
    vibez.spill("🔄 Testing PostgreSQL basic connectivity...")
    
    fr fr Test basic connection
    sus result QueryResult = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "SELECT version()")
    
    ready (result.success) {
        vibez.spill("✅ PostgreSQL connection successful")
        ready (result.rows.len() > 0 && result.rows[0].len() > 0) {
            vibez.spill("✅ PostgreSQL version:", result.rows[0][0])
        }
    } otherwise {
        vibez.spill("❌ PostgreSQL connection failed:", result.error_message)
        damn cringe
    }
    
    damn based
}

slay test_postgres_crud_operations() {
    vibez.spill("🔄 Testing PostgreSQL CRUD operations...")
    
    fr fr Create test table
    sus create_table_sql tea = `
        CREATE TABLE IF NOT EXISTS cursed_test_table (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            value INTEGER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    `
    
    sus result QueryResult = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, create_table_sql)
    ready (!result.success) {
        vibez.spill("❌ Failed to create PostgreSQL test table:", result.error_message)
        damn cringe
    }
    
    fr fr Clear any existing test data
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "DELETE FROM cursed_test_table WHERE name LIKE 'test_%'")
    
    fr fr INSERT test
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "INSERT INTO cursed_test_table (name, value) VALUES ('test_postgres', 42)")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ PostgreSQL INSERT failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ PostgreSQL INSERT successful, rows affected:", result.rows_affected)
    
    fr fr SELECT test
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "SELECT id, name, value FROM cursed_test_table WHERE name = 'test_postgres'")
    ready (!result.success || result.rows.len() == 0) {
        vibez.spill("❌ PostgreSQL SELECT failed:", result.error_message)
        damn cringe
    }
    
    vibez.spill("✅ PostgreSQL SELECT successful")
    vibez.spill("   Columns:", stringz.join(result.column_names, ", "))
    bestie (sus row []tea : result.rows) {
        vibez.spill("   Row:", stringz.join(row, " | "))
    }
    
    fr fr UPDATE test
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "UPDATE cursed_test_table SET value = 99 WHERE name = 'test_postgres'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ PostgreSQL UPDATE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ PostgreSQL UPDATE successful, rows affected:", result.rows_affected)
    
    fr fr DELETE test
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "DELETE FROM cursed_test_table WHERE name = 'test_postgres'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ PostgreSQL DELETE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ PostgreSQL DELETE successful, rows affected:", result.rows_affected)
    
    damn based
}

slay test_postgres_transactions() {
    vibez.spill("🔄 Testing PostgreSQL transactions...")
    
    fr fr Test transaction commit
    ready (!postgres_begin_transaction(DEFAULT_POSTGRES_CONNECTION)) {
        vibez.spill("❌ Failed to begin PostgreSQL transaction")
        damn cringe
    }
    
    sus result QueryResult = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, 
        "INSERT INTO cursed_test_table (name, value) VALUES ('test_transaction_commit', 123)")
    ready (!result.success) {
        postgres_rollback_transaction(DEFAULT_POSTGRES_CONNECTION)
        vibez.spill("❌ PostgreSQL transaction INSERT failed:", result.error_message)
        damn cringe
    }
    
    ready (!postgres_commit_transaction(DEFAULT_POSTGRES_CONNECTION)) {
        vibez.spill("❌ Failed to commit PostgreSQL transaction")
        damn cringe
    }
    vibez.spill("✅ PostgreSQL transaction commit successful")
    
    fr fr Test transaction rollback
    ready (!postgres_begin_transaction(DEFAULT_POSTGRES_CONNECTION)) {
        vibez.spill("❌ Failed to begin PostgreSQL rollback transaction")
        damn cringe
    }
    
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION,
        "INSERT INTO cursed_test_table (name, value) VALUES ('test_transaction_rollback', 456)")
    ready (!result.success) {
        postgres_rollback_transaction(DEFAULT_POSTGRES_CONNECTION)
        vibez.spill("❌ PostgreSQL rollback transaction INSERT failed")
        damn cringe
    }
    
    ready (!postgres_rollback_transaction(DEFAULT_POSTGRES_CONNECTION)) {
        vibez.spill("❌ Failed to rollback PostgreSQL transaction")
        damn cringe
    }
    vibez.spill("✅ PostgreSQL transaction rollback successful")
    
    fr fr Verify rollback worked
    result = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION,
        "SELECT COUNT(*) FROM cursed_test_table WHERE name = 'test_transaction_rollback'")
    ready (!result.success || result.rows.len() == 0 || stringz.to_int(result.rows[0][0]) != 0) {
        vibez.spill("❌ PostgreSQL rollback verification failed")
        damn cringe
    }
    vibez.spill("✅ PostgreSQL rollback verified - no records found as expected")
    
    fr fr Cleanup
    postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "DELETE FROM cursed_test_table WHERE name LIKE 'test_transaction_%'")
    
    damn based
}

fr fr ===== MySQL Tests =====

slay test_mysql_basic_connectivity() {
    vibez.spill("🔄 Testing MySQL basic connectivity...")
    
    sus result QueryResult = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "SELECT VERSION()")
    
    ready (result.success) {
        vibez.spill("✅ MySQL connection successful")
        ready (result.rows.len() > 0 && result.rows[0].len() > 0) {
            vibez.spill("✅ MySQL version:", result.rows[0][0])
        }
    } otherwise {
        vibez.spill("❌ MySQL connection failed:", result.error_message)
        damn cringe
    }
    
    damn based
}

slay test_mysql_crud_operations() {
    vibez.spill("🔄 Testing MySQL CRUD operations...")
    
    fr fr Create test table
    sus create_table_sql tea = `
        CREATE TABLE IF NOT EXISTS cursed_test_table (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            value INT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ) ENGINE=InnoDB
    `
    
    sus result QueryResult = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, create_table_sql)
    ready (!result.success) {
        vibez.spill("❌ Failed to create MySQL test table:", result.error_message)
        damn cringe
    }
    
    fr fr Clear any existing test data
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "DELETE FROM cursed_test_table WHERE name LIKE 'test_%'")
    
    fr fr INSERT test
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "INSERT INTO cursed_test_table (name, value) VALUES ('test_mysql', 42)")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ MySQL INSERT failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ MySQL INSERT successful, rows affected:", result.rows_affected)
    vibez.spill("   Last insert ID:", result.last_insert_id)
    
    fr fr SELECT test
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "SELECT id, name, value FROM cursed_test_table WHERE name = 'test_mysql'")
    ready (!result.success || result.rows.len() == 0) {
        vibez.spill("❌ MySQL SELECT failed:", result.error_message)
        damn cringe
    }
    
    vibez.spill("✅ MySQL SELECT successful")
    vibez.spill("   Columns:", stringz.join(result.column_names, ", "))
    bestie (sus row []tea : result.rows) {
        vibez.spill("   Row:", stringz.join(row, " | "))
    }
    
    fr fr UPDATE test
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "UPDATE cursed_test_table SET value = 99 WHERE name = 'test_mysql'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ MySQL UPDATE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ MySQL UPDATE successful, rows affected:", result.rows_affected)
    
    fr fr DELETE test
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "DELETE FROM cursed_test_table WHERE name = 'test_mysql'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ MySQL DELETE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ MySQL DELETE successful, rows affected:", result.rows_affected)
    
    damn based
}

slay test_mysql_transactions() {
    vibez.spill("🔄 Testing MySQL transactions...")
    
    fr fr Test transaction commit
    ready (!mysql_begin_transaction(DEFAULT_MYSQL_CONNECTION)) {
        vibez.spill("❌ Failed to begin MySQL transaction")
        damn cringe
    }
    
    sus result QueryResult = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION,
        "INSERT INTO cursed_test_table (name, value) VALUES ('test_transaction_commit', 123)")
    ready (!result.success) {
        mysql_rollback_transaction(DEFAULT_MYSQL_CONNECTION)
        vibez.spill("❌ MySQL transaction INSERT failed:", result.error_message)
        damn cringe
    }
    
    ready (!mysql_commit_transaction(DEFAULT_MYSQL_CONNECTION)) {
        vibez.spill("❌ Failed to commit MySQL transaction")
        damn cringe
    }
    vibez.spill("✅ MySQL transaction commit successful")
    
    fr fr Test transaction rollback
    ready (!mysql_begin_transaction(DEFAULT_MYSQL_CONNECTION)) {
        vibez.spill("❌ Failed to begin MySQL rollback transaction")
        damn cringe
    }
    
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION,
        "INSERT INTO cursed_test_table (name, value) VALUES ('test_transaction_rollback', 456)")
    ready (!result.success) {
        mysql_rollback_transaction(DEFAULT_MYSQL_CONNECTION)
        vibez.spill("❌ MySQL rollback transaction INSERT failed")
        damn cringe
    }
    
    ready (!mysql_rollback_transaction(DEFAULT_MYSQL_CONNECTION)) {
        vibez.spill("❌ Failed to rollback MySQL transaction")
        damn cringe
    }
    vibez.spill("✅ MySQL transaction rollback successful")
    
    fr fr Verify rollback worked
    result = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION,
        "SELECT COUNT(*) FROM cursed_test_table WHERE name = 'test_transaction_rollback'")
    ready (!result.success || result.rows.len() == 0 || stringz.to_int(result.rows[0][0]) != 0) {
        vibez.spill("❌ MySQL rollback verification failed")
        damn cringe
    }
    vibez.spill("✅ MySQL rollback verified - no records found as expected")
    
    fr fr Cleanup
    mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "DELETE FROM cursed_test_table WHERE name LIKE 'test_transaction_%'")
    
    damn based
}

fr fr ===== SQLite Tests =====

slay test_sqlite_basic_connectivity() {
    vibez.spill("🔄 Testing SQLite basic connectivity...")
    
    sus result QueryResult = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "SELECT sqlite_version()")
    
    ready (result.success) {
        vibez.spill("✅ SQLite connection successful")
        ready (result.rows.len() > 0 && result.rows[0].len() > 0) {
            vibez.spill("✅ SQLite version:", result.rows[0][0])
        }
    } otherwise {
        vibez.spill("❌ SQLite connection failed:", result.error_message)
        damn cringe
    }
    
    damn based
}

slay test_sqlite_crud_operations() {
    vibez.spill("🔄 Testing SQLite CRUD operations...")
    
    fr fr Create test table
    sus create_table_sql tea = `
        CREATE TABLE IF NOT EXISTS cursed_test_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            value INTEGER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    `
    
    sus result QueryResult = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, create_table_sql)
    ready (!result.success) {
        vibez.spill("❌ Failed to create SQLite test table:", result.error_message)
        damn cringe
    }
    
    fr fr Clear any existing test data
    result = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "DELETE FROM cursed_test_table WHERE name LIKE 'test_%'")
    
    fr fr INSERT test
    result = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "INSERT INTO cursed_test_table (name, value) VALUES ('test_sqlite', 42)")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ SQLite INSERT failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ SQLite INSERT successful, rows affected:", result.rows_affected)
    vibez.spill("   Last insert ID:", result.last_insert_id)
    
    fr fr SELECT test
    result = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "SELECT id, name, value FROM cursed_test_table WHERE name = 'test_sqlite'")
    ready (!result.success || result.rows.len() == 0) {
        vibez.spill("❌ SQLite SELECT failed:", result.error_message)
        damn cringe
    }
    
    vibez.spill("✅ SQLite SELECT successful")
    vibez.spill("   Columns:", stringz.join(result.column_names, ", "))
    bestie (sus row []tea : result.rows) {
        vibez.spill("   Row:", stringz.join(row, " | "))
    }
    
    fr fr UPDATE test
    result = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "UPDATE cursed_test_table SET value = 99 WHERE name = 'test_sqlite'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ SQLite UPDATE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ SQLite UPDATE successful, rows affected:", result.rows_affected)
    
    fr fr DELETE test
    result = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "DELETE FROM cursed_test_table WHERE name = 'test_sqlite'")
    ready (!result.success || result.rows_affected != 1) {
        vibez.spill("❌ SQLite DELETE failed:", result.error_message)
        damn cringe
    }
    vibez.spill("✅ SQLite DELETE successful, rows affected:", result.rows_affected)
    
    damn based
}

fr fr ===== Connection Pool Tests =====

slay test_connection_pooling() {
    vibez.spill("🔄 Testing database connection pooling...")
    
    fr fr PostgreSQL connection stats
    sus active drip, sus total drip = postgres_get_connection_stats()
    vibez.spill("✅ PostgreSQL connection pool: active =", active, "total =", total)
    
    fr fr MySQL connection stats
    sus mysql_active drip, sus mysql_total drip = mysql_get_connection_stats()
    vibez.spill("✅ MySQL connection pool: active =", mysql_active, "total =", mysql_total)
    
    fr fr Test multiple concurrent connections
    sus results []QueryResult = []
    bestie (sus i drip = 0; i < 5; i++) {
        sus result QueryResult = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "SELECT " + stringz.from_int(i) + " as test_value")
        results = results + [result]
        ready (!result.success) {
            vibez.spill("❌ Concurrent PostgreSQL query", i, "failed:", result.error_message)
            damn cringe
        }
    }
    vibez.spill("✅ Concurrent PostgreSQL queries successful")
    
    fr fr Verify results
    bestie (sus i drip = 0; i < results.len(); i++) {
        ready (results[i].rows.len() > 0 && stringz.to_int(results[i].rows[0][0]) == i) {
            vibez.spill("✅ Query", i, "result verified:", results[i].rows[0][0])
        } otherwise {
            vibez.spill("❌ Query", i, "result incorrect")
            damn cringe
        }
    }
    
    damn based
}

fr fr ===== Performance Tests =====

slay test_performance_benchmarks() {
    vibez.spill("🔄 Testing database performance benchmarks...")
    
    fr fr SQLite performance (in-memory)
    sus start_time drip = timez.get_current_timestamp_ms()
    bestie (sus i drip = 0; i < 100; i++) {
        sus result QueryResult = sqlite_real_query_simple(DEFAULT_SQLITE_CONNECTION, "SELECT " + stringz.from_int(i))
        ready (!result.success) {
            vibez.spill("❌ SQLite performance test failed at iteration", i)
            damn cringe
        }
    }
    sus sqlite_time drip = timez.get_current_timestamp_ms() - start_time
    vibez.spill("✅ SQLite: 100 queries in", sqlite_time, "ms")
    
    fr fr PostgreSQL performance
    start_time = timez.get_current_timestamp_ms()
    bestie (sus i drip = 0; i < 50; i++) {  // Fewer iterations for network db
        sus result QueryResult = postgres_real_query_simple(DEFAULT_POSTGRES_CONNECTION, "SELECT " + stringz.from_int(i))
        ready (!result.success) {
            vibez.spill("❌ PostgreSQL performance test failed at iteration", i)
            damn cringe
        }
    }
    sus postgres_time drip = timez.get_current_timestamp_ms() - start_time
    vibez.spill("✅ PostgreSQL: 50 queries in", postgres_time, "ms")
    
    fr fr MySQL performance
    start_time = timez.get_current_timestamp_ms()
    bestie (sus i drip = 0; i < 50; i++) {
        sus result QueryResult = mysql_real_query_simple(DEFAULT_MYSQL_CONNECTION, "SELECT " + stringz.from_int(i))
        ready (!result.success) {
            vibez.spill("❌ MySQL performance test failed at iteration", i)
            damn cringe
        }
    }
    sus mysql_time drip = timez.get_current_timestamp_ms() - start_time
    vibez.spill("✅ MySQL: 50 queries in", mysql_time, "ms")
    
    vibez.spill("📊 Performance Summary:")
    vibez.spill("   SQLite (in-memory): ", sqlite_time / 100.0, "ms per query")
    vibez.spill("   PostgreSQL:        ", postgres_time / 50.0, "ms per query") 
    vibez.spill("   MySQL:             ", mysql_time / 50.0, "ms per query")
    
    damn based
}

fr fr ===== Main Test Runner =====

slay main() {
    testz.test_start("Comprehensive Database Driver Testing")
    
    vibez.spill("🚀 Starting comprehensive database driver tests...")
    vibez.spill()
    
    fr fr Configuration info
    vibez.spill("📋 Test Configuration:")
    vibez.spill("   PostgreSQL:", DEFAULT_POSTGRES_CONNECTION)
    vibez.spill("   MySQL:     ", DEFAULT_MYSQL_CONNECTION)
    vibez.spill("   SQLite:    ", DEFAULT_SQLITE_CONNECTION)
    vibez.spill()
    
    sus all_passed lit = based
    
    fr fr SQLite Tests (should always work)
    ready (!test_sqlite_basic_connectivity()) {
        vibez.spill("❌ SQLite basic connectivity test failed")
        all_passed = cringe
    }
    
    ready (!test_sqlite_crud_operations()) {
        vibez.spill("❌ SQLite CRUD operations test failed") 
        all_passed = cringe
    }
    
    vibez.spill()
    
    fr fr PostgreSQL Tests (may fail if server not available)
    ready (test_postgres_basic_connectivity()) {
        ready (!test_postgres_crud_operations()) {
            vibez.spill("❌ PostgreSQL CRUD operations test failed")
            all_passed = cringe
        }
        
        ready (!test_postgres_transactions()) {
            vibez.spill("❌ PostgreSQL transactions test failed")
            all_passed = cringe
        }
    } otherwise {
        vibez.spill("⚠️ PostgreSQL server not available - skipping PostgreSQL tests")
    }
    
    vibez.spill()
    
    fr fr MySQL Tests (may fail if server not available)
    ready (test_mysql_basic_connectivity()) {
        ready (!test_mysql_crud_operations()) {
            vibez.spill("❌ MySQL CRUD operations test failed")
            all_passed = cringe
        }
        
        ready (!test_mysql_transactions()) {
            vibez.spill("❌ MySQL transactions test failed")
            all_passed = cringe
        }
    } otherwise {
        vibez.spill("⚠️ MySQL server not available - skipping MySQL tests")
    }
    
    vibez.spill()
    
    fr fr Connection pooling tests
    ready (!test_connection_pooling()) {
        vibez.spill("❌ Connection pooling test failed")
        all_passed = cringe
    }
    
    fr fr Performance tests
    ready (!test_performance_benchmarks()) {
        vibez.spill("❌ Performance benchmark test failed")
        all_passed = cringe
    }
    
    fr fr Cleanup connections
    postgres_close_all_connections()
    mysql_close_all_connections()
    
    vibez.spill()
    ready (all_passed) {
        vibez.spill("🎉 All available database tests passed!")
        testz.test_complete("All database tests successful")
    } otherwise {
        vibez.spill("💥 Some database tests failed!")
        testz.test_complete("Database test failures detected")
    }
    
    testz.print_test_summary()
}
