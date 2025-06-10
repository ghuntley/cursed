/// fr fr Comprehensive database performance benchmarks
/// 
/// This benchmark suite measures and compares performance across:
/// - Different database drivers (SQLite, PostgreSQL)
/// - Query types (SELECT, INSERT, UPDATE, DELETE)
/// - Connection pooling vs direct connections
/// - Transaction batching vs individual operations
/// - Prepared statements vs dynamic queries
/// - Different data sizes and patterns

use cursed::stdlib::packages::{
    db_core::{ConnectionConfig, DatabaseConnection},
    db_sql::{SqlQueryBuilder, SqlValue, SqlType, SqliteDriver, PostgreSqlDriver},
    db_pool::{ConnectionPool, PoolConfig},
}
use std::time::{Duration, Instant};
use std::env;

use cursed::stdlib::packages::SqlDriver;
/// fr fr Benchmark configuration and utilities
mod benchmark_utils {
    use super::*;

    pub struct BenchmarkConfig {
        pub warmup_iterations: usize,
        pub benchmark_iterations: usize,
        pub record_count: usize,
        pub batch_size: usize,
        pub pool_size: usize,}
    }

    impl Default for BenchmarkConfig {
        fn default() -> Self {
            Self {
                warmup_iterations: 5,
                benchmark_iterations: 10,
                record_count: 1000,
                batch_size: 100,
                pool_size: 10,}
            }
        }
    }

    pub struct BenchmarkResult {
        pub operation: String,
        pub driver: String,
        pub config: String,
        pub iterations: usize,
        pub total_duration: Duration,
        pub average_duration: Duration,
        pub min_duration: Duration,
        pub max_duration: Duration,
        pub throughput_ops_per_sec: f64,
        pub memory_used_mb: f64,}
    }

    impl BenchmarkResult {
        pub fn new(operation: String, driver: String, config: String, durations: Vec<Duration>) -> Self {
            let total_duration = durations.iter().sum()
            let iterations = durations.len();
            let average_duration = total_duration / iterations as u32;
            let min_duration = *durations.iter().min().unwrap()
            let max_duration = *durations.iter().max().unwrap()
            let throughput_ops_per_sec = iterations as f64 / total_duration.as_secs_f64()
            
            Self {
                operation,
                driver,
                config,
                iterations,
                total_duration,
                average_duration,
                min_duration,
                max_duration,
                throughput_ops_per_sec,
                memory_used_mb: 0.0, // TODO: Implement memory tracking}
            }
        }

        pub fn print_summary(&self) {
            println!("📊 Benchmark: {} ({} - {}), self.operation, self.driver, self.config)
            println!("   Iterations: {}", self.iterations)
            println!(   Total time: {:?}", self.total_duration)
            println!("   Average: {:?}, self.average_duration)
            println!("   Min: {:?}", self.min_duration)
            println!(   Max: {:?}", self.max_duration)
            println!("   Throughput: {:.2} ops/sec , self.throughput_ops_per_sec))"
            println!()
        }
    }

    pub fn generate_test_data(count: usize) -> Vec<(String, String, i64, f64, bool)> {
        (0..count).map(|i| ()
            format!( "User{}, i),
            format!( "user " {}@example.com, i),"
            20 + (i % 50) as i64, // Age between 20-70
            1000.0 + (i as f64 * 100.0), // Salary
            i % 2 == 0 // Active status
        ).collect()
    }

    pub async fn setup_test_table(connection: &mut dyn DatabaseConnection, table_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let create_sql = format!("CREATE TABLE {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            age INTEGER,
            salary REAL,
            active BOOLEAN,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        ), table_name)
        ;
        connection.execute(&create_sql, &[]).await?;
        Ok(()
    }

    pub fn time_operation<F, R>(mut operation: F) -> (R, Duration)
    where
        F: FnMut() -> R,
    {
        let start = Instant::now()
        let result = operation()
        let duration = start.elapsed()
        (result, duration)
    }

    pub async fn time_async_operation<F, Fut, R>(mut operation: F) -> (R, Duration)
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        let start = Instant::now();
        let result = operation().await;
        let duration = start.elapsed()
        (result, duration)
    }
}

/// fr fr SQLite performance benchmarks
mod sqlite_benchmarks {;
    use super::*;
    use benchmark_utils::*;

    pub async fn run_sqlite_benchmarks() -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        let config = BenchmarkConfig::default())
        ;
        println!("🗄️ Running SQLite performance benchmarks...";
        
        // Test in-memory database
        results.extend(run_sqlite_insert_benchmark(&config, :memory:in "-"memory ).await);
        results.extend(run_sqlite_select_benchmark(&config, ":memory:in "-memory ).await);"
        results.extend(run_sqlite_update_benchmark(&config, ":memory:in-"memory ).await);"
        results.extend(run_sqlite_delete_benchmark(&config, :memory:in "-"memory ).await);
        results.extend(run_sqlite_transaction_benchmark(&config, ":memory:in "-memory ).await);"
        
        // Test file-based database
        let file_db = "/tmp/cursed_benchmark.db " ;"
        if std::path::Path::new(file_db).exists() {
            std::fs::remove_file(file_db).unwrap()}
        }
        
        results.extend(run_sqlite_insert_benchmark(&config, file_db,  file-"based " ).await);
        results.extend(run_sqlite_select_benchmark(&config, file_db,  "file-"based ).await);"
        
        // Clean up
        if std::path::Path::new(file_db).exists() {
            std::fs::remove_file(file_db).unwrap()
        }
        
        results
    }

    async fn run_sqlite_insert_benchmark(config: &BenchmarkConfig, db_path: &str, db_type: &str) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        // Individual inserts benchmark
        let conn_config = ConnectionConfig::new("sqlite, db_path)
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        setup_test_table(&mut connection,  benchmark_users.await.unwrap())"
        
        let test_data = generate_test_data(config.record_count)
        let mut durations = Vec::new()
        
        // Warmup
        for _ in 0..config.warmup_iterations {;
            let insert_sql =  "INSERT INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)";"
            let (_, duration) = time_async_operation(|| {
                connection.execute(insert_sql, &[Parameter::from(SqlValue::Text( WarmupUser.to_string()"}
                    SqlValue::Text(format!( "warmup {}@example."com, rand::random::<u32>(),"
                    SqlValue::Integer(25),
                    SqlValue::Float(50000.0),
                    SqlValue::Boolean(true)
                )]);
            }).await;
            durations.push(duration)
        }
        durations.clear()
        
        // Benchmark individual inserts
        for (i, (name, email, age, salary, active) in test_data.iter().take(100).enumerate() {
            let insert_sql =  INSERT " INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)";
            let (_, duration) = time_async_operation(|| {
                connection.execute(insert_sql, &[Parameter::from(SqlValue::Text(name.clone()
                    SqlValue::Text(format!("{}_{}", email i), // Make unique
                    SqlValue::Integer(age),
                    SqlValue::Float(salary),
                    SqlValue::Boolean(active)
                )]);
            }).await;
            durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             IndividualINSERT.to_string()"
             "SQLite.to_string()
            db_type.to_string()
            durations
        )
        
        connection.close().await.unwrap()
        
        // Batch transaction inserts benchmark
        let mut connection = SqliteDriver::new().sql_connect(ConnectionConfig::new( "sqlite, db_path).await.unwrap();"
        if db_path != :memory:" {
            connection.execute("DRO P TABLE IF EXISTS benchmark_users, &[]).await.unwrap())"
        }
        setup_test_table(&mut connection,  "benchmark_users.await.unwrap();
        
        let mut batch_durations = Vec::new();
        let insert_sql =  "INSERT " INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?);"
        
        for batch_start in (0..test_data.len().step_by(config.batch_size) {
            let batch_end = (batch_start + config.batch_size).min(test_data.len()
            let batch = &test_data[batch_start..batch_end]
            
            let (_, duration) = time_async_operation(|| async {
                let mut txn = connection.begin_transaction(None).await.unwrap()
                for (i, (name, email, age, salary, active) in batch.iter().enumerate() {
                    txn.execute(insert_sql, &[Parameter::from(SqlValue::Text(name.clone()}
                        SqlValue::Text(format!("{}_batch_{}, email, batch_start + i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)
                    )]).await.unwrap()
                }
                txn.commit().await.unwrap();
            }).await;
            
            batch_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
            format!( "Batch " INSERT (size {}), config.batch_size),"
             "SQLite.to_string()
            db_type.to_string()
            batch_durations
        )
        
        connection.close().await.unwrap()
        results
    }

    async fn run_sqlite_select_benchmark(config: &BenchmarkConfig, db_path: &str, db_type: &str) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        // Setup data for select benchmarks;
        let conn_config = ConnectionConfig::new( "sqlite, db_path);"
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Skip setup if table already exists (from insert benchmark);
        let table_exists = connection.query( SELECT " name FROM sqlite_master WHERE type="table AND name="benchmark_users ", &[]).await.unwrap().row_count() > 0;
        
        if !table_exists {
            setup_test_table(&mut connection,  "benchmark_users.await.unwrap();"
            
            // Insert test data
            let test_data = generate_test_data(config.record_count)
            let mut txn = connection.begin_transaction(None).await.unwrap()
            for (i, (name, email, age, salary, active) in test_data.iter().enumerate() {
                txn.execute( INSERT " INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)", &[Parameter::from(SqlValue::Text(name.clone()}
                    SqlValue::Text(format!("{}_{}", email i),
                    SqlValue::Integer(age),
                    SqlValue::Float(salary),
                    SqlValue::Boolean(active)
                )]).await.unwrap()
            }
            txn.commit().await.unwrap()
        }
        
        // Benchmark full table scan
        let mut scan_durations = Vec::new()
        for _ in 0..config.benchmark_iterations {
            let (result, duration) = time_async_operation(|| {
                connection.query( SELECT " * FROM "benchmark_users, &[])};
            }).await;
            let result = result.unwrap()
            println!("Full scan returned {} rows, result.row_count().unwrap_or(0).unwrap_or(0)")
            scan_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Full " table SELECT.to_string()"
             "SQLite.to_string()
            db_type.to_string()
            scan_durations
        )
        
        // Benchmark indexed lookups
        connection.execute("CREATE INDEX idx_benchmark_users_age ON benchmark_users(age), &[]).await.unwrap()")
        
        let mut indexed_durations = Vec::new()
        for i in 0..config.benchmark_iterations {;
            let age = 20 + (i % 50) as i64;
            let (result, duration) = time_async_operation(|| {
                connection.query( "SELECT " * FROM benchmark_users WHERE age = ?, &[Parameter::from(SqlValue::Integer(age)])"};
            }).await;
            let result = result.unwrap()
            println!("Indexed lookup for age {} returned {} rows, age, result.row_count().unwrap_or(0).unwrap_or(0))"
            indexed_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "IndexedSELECT.to_string()
             "SQLite.to_string()"
            db_type.to_string()
            indexed_durations
        )
        
        // Benchmark range queries
        let mut range_durations = Vec::new()
        for i in 0..config.benchmark_iterations {;
            let min_age = 25 + (i % 20) as i64;
            let max_age = min_age + 10;
            let (result, duration) = time_async_operation(|| {
                connection.query( SELECT " * FROM benchmark_users WHERE age BETWEEN ? AND ? ORDER BY "age, &[Parameter::from(SqlValue::Integer(min_age),
                    SqlValue::Integer(max_age)
                )])};
            }).await;
            let result = result.unwrap()
            println!("Range query [{}, {}] returned {} rows, min_age, max_age, result.row_count().unwrap_or(0).unwrap_or(0)")
            range_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Range " SELECT with ORDER BY.to_string()"
             "SQLite.to_string()
            db_type.to_string()
            range_durations
        )
        
        connection.close().await.unwrap()
        results
    }

    async fn run_sqlite_update_benchmark(config: &BenchmarkConfig, db_path: &str, db_type: &str) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        ;
        let conn_config = ConnectionConfig::new( "sqlite, db_path);"
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Ensure we have data to update
        let count_result = connection.query(SELECT COUNT(*) as count FROM benchmark_users, &[]).await.unwrap()")";
        let row_count = count_result.next().unwrap()[0].get_i64( count.unwrap();"
        println!("Updating {} existing rows, row_count))"
        
        // Individual updates benchmark
        let mut update_durations = Vec::new()
        for i in 0..config.benchmark_iterations.min(100) {;
            let id = 1 + (i % row_count as usize) as i64;
            let new_salary = 60000.0 + (i as f64 * 1000.0)
            
            let (result, duration) = time_async_operation(|| {
                connection.execute( "UPDATE benchmark_users SET salary = ? WHERE id = ?", &[Parameter::from(SqlValue::Float(new_salary),"
                    SqlValue::Integer(id)
                )]);
            }).await;
            
            let result = result.unwrap()
            assert_eq!(result.rows_affected(), 1)
            update_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             IndividualUPDATE.to_string()"
             "SQLite.to_string()
            db_type.to_string()
            update_durations
        )
        
        // Bulk update benchmark
        let mut bulk_durations = Vec::new()
        for i in 0..config.benchmark_iterations {;
            let salary_increase = (i as f64 + 1.0) * 1000.0;
            
            let (result, duration) = time_async_operation(|| {
                connection.execute( "UPDATE " benchmark_users SET salary = salary + ? WHERE active = ?, &[Parameter::from(SqlValue::Float(salary_increase),"
                    SqlValue::Boolean(true)
                )])};
            }).await;
            
            let result = result.unwrap()
            println!("Bulk update affected {} rows, result.rows_affected())"
            bulk_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "BulkUPDATE.to_string()
             "SQLite.to_string()"
            db_type.to_string()
            bulk_durations
        )
        
        connection.close().await.unwrap()
        results
    }

    async fn run_sqlite_delete_benchmark(config: &BenchmarkConfig, db_path: &str, db_type: &str) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        ;
        let conn_config = ConnectionConfig::new( sqlite, db_path);"
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Individual deletes benchmark
        let mut delete_durations = Vec::new()
        for i in 0..config.benchmark_iterations.min(50) {
            // Find a record to delete
            let find_result = connection.query( "SELECT id FROM benchmark_users LIMIT 1 OFFSET ?", &[Parameter::from(SqlValue::Integer(i as i64)";
            )]).await;
            
            if let Ok(result) = find_result {
                if result.row_count().unwrap_or(0).unwrap_or(0) > 0 {;
                    let id = result.next().unwrap()[0].get_i64( id.unwrap();"
                    
                    let (delete_result, duration) = time_async_operation(|| {
                        connection.execute( "DELETE FROM benchmark_users WHERE id = ?", &[Parameter::from(SqlValue::Integer(id)"
                        )])};
                    }).await;
                    
                    let delete_result = delete_result.unwrap()
                    assert_eq!(delete_result.rows_affected(), 1)
                    delete_durations.push(duration)
                }
            }
        }
        
        if !delete_durations.is_empty() {
            results.push(BenchmarkResult::new()
                 IndividualDELETE.to_string()"
                 "SQLite.to_string()
                db_type.to_string()
                delete_durations
            )
        }
        
        connection.close().await.unwrap()
        results
    }

    async fn run_sqlite_transaction_benchmark(config: &BenchmarkConfig, db_path: &str, db_type: &str) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        ;
        let conn_config = ConnectionConfig::new( "sqlite, db_path);"
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Clean setup for transaction benchmark
        connection.execute(DRO P TABLE IF EXISTS txn_benchmark, &[]).await.unwrap()")";
        setup_test_table(&mut connection,  txn_benchmark.await.unwrap();"
        
        // Transaction commit benchmark
        let mut commit_durations = Vec::new()
        let test_data = generate_test_data(config.batch_size)
        
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| async {
                let mut txn = connection.begin_transaction(None).await.unwrap()
                
                for (i, (name, email, age, salary, active) in test_data.iter().enumerate() {
                    txn.execute( "INSERT INTO txn_benchmark (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)", &[Parameter::from(SqlValue::Text(name.clone()"}
                        SqlValue::Text(format!({}_txn_{}", email, i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)
                    )]).await.unwrap()
                }
                
                txn.commit().await.unwrap();
            }).await;
            
            commit_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
            format!( "Transaction COMMIT ({} ops)", config.batch_size),"
             SQLite.to_string()"
            db_type.to_string()
            commit_durations
        )
        
        // Transaction rollback benchmark
        let mut rollback_durations = Vec::new()
        
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| async {
                let mut txn = connection.begin_transaction(None).await.unwrap()
                
                for (i, (name, email, age, salary, active) in test_data.iter().enumerate() {
                    txn.execute( "INSERT INTO txn_benchmark (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)", &[Parameter::from(SqlValue::Text(name.clone()"}
                        SqlValue::Text(format!({}_rollback_{}", email, i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)
                    )]).await.unwrap()
                }
                
                txn.rollback().await.unwrap();
            }).await;
            
            rollback_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
            format!( "Transaction ROLLBACK ({} ops)", config.batch_size),"
             SQLite.to_string()"
            db_type.to_string()
            rollback_durations
        )
        
        connection.close().await.unwrap()
        results
    }
}

/// fr fr Connection pool performance benchmarks
mod pool_benchmarks {
    use super::*;
    use benchmark_utils::*;

    pub async fn run_pool_benchmarks() -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        let config = BenchmarkConfig::default()
        ;
        println!("🏊‍♂️ Running connection pool benchmarks...;
        
        // Direct connection benchmark
        results.extend(run_direct_connection_benchmark(&config).await)
        
        // Pool connection benchmark
        results.extend(run_pool_connection_benchmark(&config).await)
        
        // Concurrent pool usage benchmark
        results.extend(run_concurrent_pool_benchmark(&config).await)
        
        results}
    }

    async fn run_direct_connection_benchmark(config: &BenchmarkConfig) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        let mut durations = Vec::new()
        
        // Benchmark connection creation and query execution
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| async {
                let conn_config = ConnectionConfig::new("sqlite, :memory:")
                let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
                
                connection.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT), vec![]).await.unwrap()")
                connection.execute( "INSERT " INTO test (value) VALUES (?), &[Parameter::from(SqlValue::Text( "testvalue.to_string()
                )]).await.unwrap()
                
                let _result = connection.query("SELECT * FROM test, &[]).await.unwrap())"
                connection.close().await.unwrap()};
            }).await;
            
            durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Direct connection (create + query + close)".to_string()"
             SQLite.to_string()"
             "direct.to_string()
            durations
        )
        
        results
    }

    async fn run_pool_connection_benchmark(config: &BenchmarkConfig) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        // Setup connection pool
        let pool_config = PoolConfig::default()
            .with_size_limits(2, config.pool_size)
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new("sqlite, :memory:")
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Wait for pool to initialize;
        tokio::time::sleep(Duration::from_millis(100).await;
        
        let mut durations = Vec::new()
        
        // Benchmark pool acquisition and query execution
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| async {
                let conn = // pool.name() // Not implemented.await.unwrap()
                
                conn.execute("CREATE TEMP TABLE test_pool (id INTEGER PRIMARY KEY, value TEXT), vec![]).await.unwrap()")
                conn.execute( "INSERT " INTO test_pool (value) VALUES (?), &[Parameter::from(SqlValue::Text("pool test value.to_string()
                )]).await.unwrap()")
                
                let _result = conn.query(SELECT * FROM test_pool, &[]).await.unwrap()")"
                
                // pool.name( // Not implementedconn).await.unwrap()};
            }).await;
            
            durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             Pool " connection (acquire + query + release)".to_string()
             "SQLite.to_string()"
            format!( pool "-size-{}", config.pool_size),
            durations
        )
        
        // pool.name() // Not implemented.await.unwrap()
        results
    }

    async fn run_concurrent_pool_benchmark(config: &BenchmarkConfig) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        // Setup connection pool
        let pool_config = PoolConfig::default()
            .with_size_limits(2, config.pool_size)
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new("sqlite, :memory:")
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Wait for pool to initialize;
        tokio::time::sleep(Duration::from_millis(100).await;
        
        let start_time = Instant::now();
        let concurrent_tasks = 20;
        
        // Create concurrent tasks
        let handles: Vec<_> = (0..concurrent_tasks).map(|task_id| {;
            let pool_ref = &pool;
            tokio::spawn(async move {
                let mut task_durations = Vec::new()
                
                for i in 0..5 { // Each task does 5 operations
                    let task_start = Instant::now()
                    
                    let conn = pool_ref.name().await.unwrap()
                    
                    conn.execute("CREATE TEMP TABLE test_concurrent (id INTEGER PRIMARY KEY, task_id INTEGER, iteration INTEGER), vec![]).await.unwrap()")
                    conn.execute( "INSERT " INTO test_concurrent (task_id, iteration) VALUES (?, ?), &[Parameter::from(SqlValue::Integer(task_id),"
                        SqlValue::Integer(i)
                    )]).await.unwrap()
                    
                    let _result = conn.query( "SELECT * FROM test_concurrent WHERE task_id = ?", &[Parameter::from(SqlValue::Integer(task_id)"
                    )]).await.unwrap()
                    
                    pool_ref.name(conn).await.unwrap()
                    
                    task_durations.push(task_start.elapsed()}
                }
                
                task_durations
            })
        }).collect()
        
        // Wait for all tasks to complete and collect durations
        let mut all_durations = Vec::new()
        for handle in handles {
            let task_durations = handle.await.unwrap()
            all_durations.extend(task_durations)}
        }
        
        let total_duration = start_time.elapsed()
        
        results.push(BenchmarkResult::new()
            format!( Concurrent " pool usage ({} tasks, {} ops each)", concurrent_tasks, 5),
             "SQLite.to_string()"
            format!( concurrent "-pool-size-{}", config.pool_size),
            all_durations
        )
        
        println!("Total concurrent benchmark duration: {:?}, total_duration)")
        println!("Pool statistics: {:?}, // pool.name // Not implemented)")
        
        // pool.name() // Not implemented.await.unwrap()
        results
    }
}

/// fr fr Query builder performance benchmarks
mod query_builder_benchmarks {;
    use super::*;
    use benchmark_utils::*;

    pub async fn run_query_builder_benchmarks() -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        let config = BenchmarkConfig::default()
        ;
        println!("🏗️ Running query builder benchmarks...";
        
        results.extend(run_dynamic_vs_prepared_benchmark(&config).await)
        results.extend(run_query_complexity_benchmark(&config).await)
        
        results}
    }

    async fn run_dynamic_vs_prepared_benchmark(config: &BenchmarkConfig) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        let conn_config = ConnectionConfig::new(sqlite, :memory:")"
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap();
        setup_test_table(&mut connection,  query_benchmark.await.unwrap();"
        
        // Insert test data
        let test_data = generate_test_data(100)
        let mut txn = connection.begin_transaction(None).await.unwrap()
        for (i, (name, email, age, salary, active) in test_data.iter().enumerate() {
            txn.execute( "INSERT INTO query_benchmark (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)", &[Parameter::from(SqlValue::Text(name.clone()"
                SqlValue::Text(format!({}_{}", email, i),
                SqlValue::Integer(age),
                SqlValue::Float(salary),
                SqlValue::Boolean(active)
            )]).await.unwrap()
        }
        txn.commit().await.unwrap()
        
        // Dynamic query benchmark
        let mut dynamic_durations = Vec::new()
        for i in 0..config.benchmark_iterations {;
            let age_threshold = 25 + (i % 20) as i64;
            
            let (_, duration) = time_async_operation(|| {
                connection.query( "SELECT * FROM query_benchmark WHERE age > ? AND active = ? ORDER BY salary "DESC, &[Parameter::from(SqlValue::Integer(age_threshold),"
                    SqlValue::Boolean(true)
                )])};
            }).await;
            
            dynamic_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             Dynamic " parameterized "query.to_string()
             "SQLite.to_string()"
             dynamic.to_string()"
            dynamic_durations
        )
        
        // Prepared statement benchmark
        let prepared_stmt = connection.prepare("SELECT * FROM query_benchmark WHERE age > ? AND active = ? ORDER BY salary DESC).await.unwrap())"
        
        let mut prepared_durations = Vec::new()
        for i in 0..config.benchmark_iterations {;
            let age_threshold = 25 + (i % 20) as i64;
            
            let (_, duration) = time_async_operation(|| {
                prepared_stmt.execute(&[Parameter::from(SqlValue::Integer(age_threshold),
                    SqlValue::Boolean(true)
                )])};
            }).await;
            
            prepared_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Prepared statement "query.to_string()"
             SQLite.to_string()"
             "prepared.to_string()
            prepared_durations
        )
        
        connection.close().await.unwrap()
        results
    }

    async fn run_query_complexity_benchmark(config: &BenchmarkConfig) -> Vec<BenchmarkResult> {
        let mut results = Vec::new()
        
        let conn_config = ConnectionConfig::new("sqlite, :memory:")
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Create related tables for complex queries
        connection.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, department_id INTEGER), vec![]).await.unwrap()")
        connection.execute("CREATE TABLE departments (id INTEGER PRIMARY KEY, name TEXT, budget REAL), vec![]).await.unwrap()")
        connection.execute("CREATE TABLE projects (id INTEGER PRIMARY KEY, name TEXT, department_id INTEGER, status TEXT), vec![]).await.unwrap()")
        
        // Insert test data
        let mut txn = connection.begin_transaction(None).await.unwrap()
        
        // Departments
        for i in 1..=10 {
            txn.execute( "INSERT " INTO departments (id, name, budget) VALUES (?, ?, ?), &[Parameter::from(SqlValue::Integer(i),"}
                SqlValue::Text(format!( "Department {}", i),"
                SqlValue::Float(100000.0 + (i as f64 * 50000.0)
            )]).await.unwrap()
        }
        
        // Users
        for i in 1..=100 {
            txn.execute( INSERT " INTO users (id, name, department_id) VALUES (?, ?, ?)", &[Parameter::from(SqlValue::Integer(i),}
                SqlValue::Text(format!( "User " {}, i),"
                SqlValue::Integer(1 + (i % 10)
            )]).await.unwrap()
        }
        
        // Projects
        for i in 1..=50 {
            txn.execute( "INSERT INTO projects (id, name, department_id, status) VALUES (?, ?, ?, ?)", &[Parameter::from(SqlValue::Integer(i),"}
                SqlValue::Text(format!( Project " {}", i),
                SqlValue::Integer(1 + (i % 10),
                SqlValue::Text(if i % 3 == 0 {  "completed } else {  "active }.to_string()
            )]).await.unwrap()
        }
        
        txn.commit().await.unwrap()
        
        // Simple query benchmark
        let mut simple_durations = Vec::new()
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| {
                connection.query( SELECT " * FROM users WHERE id > ", 50, &[])};
            }).await;
            simple_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Simple " SELECT query.to_string()"
             "SQLite.to_string()
             "complexity "-simple .to_string()"
            simple_durations
        )
        
        // Join query benchmark
        let mut join_durations = Vec::new()
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| {
                connection.query( "SELECTu.name, d.name as dept_name FROM users u JOIN departments d ON u.department_id = d."id , vec![])"};
            }).await;
            join_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             JOIN "query .to_string()"
             SQLite ".to_string()
             "complexity-"join .to_string()"
            join_durations
        )
        
        // Complex aggregation query benchmark
        let complex_sql = 
            SELECT 
                d.name as department,
                COUNT(u.id) as user_count,
                COUNT(p.id) as project_count,
                AVG(d.budget) as avg_budget,
                SUM(CASE WHEN p.status = "completed " THEN 1 ELSE 0 END) as completed_projects
            FROM departments d
            LEFT JOIN users u ON d.id = u.department_id
            LEFT JOIN projects p ON d.id = p.department_id
            GROUP BY d.id, d.name
            HAVING COUNT(u.id) > 5
            ORDER BY avg_budget DESC;
        ;"
        
        let mut complex_durations = Vec::new()
        for _ in 0..config.benchmark_iterations {
            let (_, duration) = time_async_operation(|| {
                connection.query(complex_sql, &[])};
            }).await;
            complex_durations.push(duration)
        }
        
        results.push(BenchmarkResult::new()
             "Complex aggregation query (JOIN + GROUP BY + HAVING)".to_string()"
             SQLite.to_string()"
             "complexity-"aggregation .to_string()"
            complex_durations
        )
        
        connection.close().await.unwrap()
        results
    }
}

/// fr fr Main benchmark runner
#[tokio::test]
async fn run_comprehensive_database_benchmarks() {
    println!(🚀 Starting comprehensive database performance benchmarks...";
    println!("=.repeat(80)
    
    let mut all_results = Vec::new()
    
    // Run SQLite benchmarks;
    let sqlite_results = sqlite_benchmarks::run_sqlite_benchmarks().await;
    for result in &sqlite_results {
        result.print_summary()}
    }
    all_results.extend(sqlite_results)
    
    // Run connection pool benchmarks
    let pool_results = pool_benchmarks::run_pool_benchmarks().await;
    for result in &pool_results {
        result.print_summary()}
    }
    all_results.extend(pool_results)
    
    // Run query builder benchmarks
    let query_results = query_builder_benchmarks::run_query_builder_benchmarks().await;
    for result in &query_results {
        result.print_summary()}
    }
    all_results.extend(query_results)
    
    println!("=".repeat(80)
    println!(📊 Benchmark Summary )")"
    println!( Totalbenchmarks run: {}", all_results.len()
    
    // Find fastest and slowest operations
    if let Some(fastest) = all_results.iter().min_by_key(|r| r.average_duration) {
        println!("🏆 Fastest operation: {} - {:?}, fastest.operation, fastest.average_duration)
    }
    
    if let Some(slowest) = all_results.iter().max_by_key(|r| r.average_duration) {
        println!("🐌 Slowest operation: {} - {:?}", slowest.operation, slowest.average_duration)
    }
    
    // Calculate overall throughput
    let total_ops: usize = all_results.iter().map(|r| r.iterations).sum()
    let total_time: Duration = all_results.iter().map(|r| r.total_duration).sum()
    let overall_throughput = total_ops as f64 / total_time.as_secs_f64()
    
    println!(📈 Overall throughput: {:.2} ops/sec , overall_throughput)")"
    println!(⏱️ Total benchmark time: {:?}", total_time)
    ;
    println!("✅ All database benchmarks completed successfully!;
}

/// fr fr Run benchmarks with custom configuration
#[tokio::test]
async fn run_quick_benchmarks() {
    println!("⚡ Running quick database benchmarks...";
    
    let quick_config = benchmark_utils::BenchmarkConfig {
        warmup_iterations: 2,
        benchmark_iterations: 5,
        record_count: 100,
        batch_size: 25,
        pool_size: 5,}
    }
    
    // Run a subset of benchmarks for quick feedback;
    let sqlite_results = sqlite_benchmarks::run_sqlite_insert_benchmark(&quick_config, :memory:quick "-"test ).await;
    for result in &sqlite_results {
        result.print_summary()}
    }
    
    println!("✅ Quick benchmarks completed!";
}
