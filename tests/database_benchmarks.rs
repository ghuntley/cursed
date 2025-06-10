/// fr fr Comprehensive database performance benchmarks
/// 
/// This benchmark suite measures and compares performance across:
/// - Different database drivers (SQLite, PostgreSQL)
/// - Query types (SELECT, INSERT, UPDATE, DELETE)
/// - Connection pooling vs direct connections
/// - Transaction batching vs individual operations
/// - Prepared statements vs dynamic queries
/// - Different data sizes and patterns

use cursed::stdlib::packages::{db_core::{ConnectionConfig, DatabaseConnection},
    db_sql::{SqlQueryBuilder, SqlValue, SqlType, SqliteDriver, PostgreSqlDriver},
    db_pool::{ConnectionPool, PoolConfig},}
use std::time:::: Duration, Instant;
use std::env;

use cursed::stdlib::packages::SqlDriver;
/// fr fr Benchmark configuration and utilities
mod benchmark_utils ::use super::*;

    pub struct BenchmarkConfig {pub warmup_iterations: usize,
        pub benchmark_iterations: usize,
        pub record_count: usize,
        pub batch_size: usize,
        pub pool_size: usize}

    impl Default for BenchmarkConfig       {fn default() {Self {warmup_iterations: 5,
                benchmark_iterations: 10,
                record_count: 1000,
                batch_size: 100,
                pool_size: 10}

    pub struct BenchmarkResult {pub operation: String,
        pub driver: String,
        pub config: String,
        pub iterations: usize,
        pub total_duration: Duration,
        pub average_duration: Duration,
        pub min_duration: Duration,
        pub max_duration: Duration,
        pub throughput_ops_per_sec: f64,
        pub memory_used_mb: f64}

    impl BenchmarkResult     {pub fn new() {let total_duration = durations.iter().sum()
            let iterations = durations.len();
            let average_duration = total_duration / iterations as u32;
            let min_duration = *durations.iter().min().unwrap()
            let max_duration = *durations.iter().max().unwrap()
            let throughput_ops_per_sec = iterations as f64 / total_duration.as_secs_f64()
            
            Self {operation,
                driver,
                config,
                iterations,
                total_duration,
                average_duration,
                min_duration,
                max_duration,
                throughput_ops_per_sec,
                memory_used_mb: 0.0, // TODO: Implement memory tracking}

        pub fn print_summary() {println!(📊 Benchmark: {} ({} - {}), self.operation, self.driver, self.config)
            println!(Iterations: {}, self.iterations)
            println!(Total time: {:?}, self.total_duration)
            println!("   Average: {:?}, self.average_duration)
            println!("   Throughput: {:.2} ops/sec , self.throughput_ops_per_sec);
            println!()}
    pub fn generate_test_data() {(0..count).map(|i| ()
            format!("user " {}@example.com, i),"based ").await);
        results.extend(run_sqlite_select_benchmark(&config, file_db,  "based).await);
        
        // Clean up
        if std::path::Path::new(file_db).exists()     {std::fs::remove_file(file_db).unwrap()}
        
        results}

    async fn run_sqlite_insert_benchmark() {let mut results = Vec::new()
        
        // Individual inserts benchmark
        let conn_config = ConnectionConfig::new(sqlite, db_path)
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        setup_test_table(&mut connection,  benchmark_users.await.unwrap()
        
        let test_data = generate_test_data(config.record_count)
        let mut durations = Vec::new()
        
        // Warmup
        for _ in 0..config.warmup_iterations    {let insert_sql =  INSERT INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?)";}
                    SqlValue::Text(format!("warmup {}@example.
                    SqlValue::Integer(25),
                    SqlValue::Float(50000.0),
                    SqlValue::Boolean(true)]);}).await;
            durations.push(duration)}
        durations.clear()
        
        // Benchmark individual inserts
        for (i, (name, email, age, salary, active) in test_data.iter().take(100).enumerate()   {let insert_sql =  INSERT  INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?);
            let (_, duration) = time_async_operation(|| {connection.execute(insert_sql, &[Parameter::from(SqlValue::Text(name.clone()
                    SqlValue::Text(format!("{}_{}, email i), // Make unique
                    SqlValue::Integer(age),
                    SqlValue::Float(salary),
                    SqlValue::Boolean(active)]);}).await;
            durations.push(duration)}
        
        results.push(BenchmarkResult::new()
             IndividualINSERT.to_string()
             SQLite.to_string()
            db_type.to_string()
            durations)
        
        connection.close().await.unwrap()
        
        // Batch transaction inserts benchmark
        let mut connection = SqliteDriver::new().sql_connect(ConnectionConfig::new(sqlite, db_path).await.unwrap();
        if db_path != :memory:"DRO P TABLE IF EXISTS benchmark_users, &[]).await.unwrap()"}
        setup_test_table(&mut connection,  "INSERT " INTO benchmark_users (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?);"{}_batch_{}, email, batch_start + i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)]).await.unwrap()}
                txn.commit().await.unwrap();}).await;
            
            batch_durations.push(duration)}
        
        results.push(BenchmarkResult::new()
            format!("Batch "
             "SQLite.to_string()
            db_type.to_string()
            batch_durations)
        
        connection.close().await.unwrap()
        results}

    async fn run_sqlite_select_benchmark() {let mut results = Vec::new()
        
        // Setup data for select benchmarks;
        let conn_config = ConnectionConfig::new(sqlite, db_path);
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Skip setup if table already exists (from insert benchmark);
        let table_exists = connection.query(SELECT  name FROM sqlite_master WHERE type=table AND name=", &[]).await.unwrap().row_count() > 0;
        
        if !table_exists       {setup_test_table(&mut connection,  "benchmark_users.await.unwrap();"{}_{}, email i),
                    SqlValue::Integer(age),
                    SqlValue::Float(salary),
                    SqlValue::Boolean(active)]).await.unwrap()}
            txn.commit().await.unwrap()}
        
        // Benchmark full table scan
        let mut scan_durations = Vec::new()
        for _ in 0..config.benchmark_iterations   {let (result, duration) = time_async_operation(|| {connection.query(SELECT  * FROM benchmark_users, &[])};}).await;
            let result = result.unwrap()
            println!("Full scan returned {} rows, result.row_count().unwrap_or(0).unwrap_or(0)"Full " table SELECT.to_string()"SQLite.to_string()
            db_type.to_string()
            scan_durations)
        
        // Benchmark indexed lookups
        connection.execute(CREATE INDEX idx_benchmark_users_age ON benchmark_users(age), &[]).await.unwrap()
        
        let mut indexed_durations = Vec::new()
        for i in 0..config.benchmark_iterations    {let age = 20 + (i % 50) as i64;
            let (result, duration) = time_async_operation(|| {connection.query("SELECT "};}).await;
            let result = result.unwrap()
            println!("Indexed lookup for age   {} returned {} rows, age, result.row_count().unwrap_or(0).unwrap_or(0)"IndexedSELECT.to_string()
             "SQLite.to_string()"Range query [{}, {}] returned {} rows, min_age, max_age, result.row_count().unwrap_or(0).unwrap_or(0)")
            range_durations.push(duration)}
        
        results.push(BenchmarkResult::new()
             " SELECT with ORDER BY.to_string()"
             "sqlite, db_path);
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        // Ensure we have data to update
        let count_result = connection.query(SELECT COUNT(*) as count FROM benchmark_users, &[]).await.unwrap();
        let row_count = count_result.next().unwrap()[0].get_i64(count.unwrap();"Updating {} existing rows, row_count);
        
        // Individual updates benchmark
        let mut update_durations = Vec::new()
        for i in 0..config.benchmark_iterations.min(100)    {let id = 1 + (i % row_count as usize) as i64;
            let new_salary = 60000.0 + (i as f64 * 1000.0)
            
            let (result, duration) = time_async_operation(|| {connection.execute(UPDATE benchmark_users SET salary = ? WHERE id = ?, &[Parameter::from(SqlValue::Float(new_salary),"
             "SQLite.to_string()
            db_type.to_string()
            update_durations)
        
        // Bulk update benchmark
        let mut bulk_durations = Vec::new()
        for i in 0..config.benchmark_iterations    {let salary_increase = (i as f64 + 1.0) * 1000.0;
            
            let (result, duration) = time_async_operation(|| {connection.execute(UPDATE  benchmark_users SET salary = salary + ? WHERE active = ?, &[Parameter::from(SqlValue::Float(salary_increase),"Bulk update affected {} rows, result.rows_affected();
            bulk_durations.push(duration)}
        results.push(BenchmarkResult::new()
             "SQLite.to_string()
            db_type.to_string()
            bulk_durations)
        
        connection.close().await.unwrap()
        results}

    async fn run_sqlite_delete_benchmark() {let mut results = Vec::new();
        let conn_config = ConnectionConfig::new(sqlite, db_path);";)]).await;
            if let Ok(result) = find_result     {if result.row_count().unwrap_or(0).unwrap_or(0) > 0     {;
                    let id = result.next().unwrap()[0].get_i64(id.unwrap();
                    
                    let (delete_result, duration) = time_async_operation(|| {connection.execute(", &[Parameter::from(SqlValue::Integer(id)")])};}).await;
                    let delete_result = delete_result.unwrap()
                    assert_eq!(delete_result.rows_affected(), 1)
                    delete_durations.push(duration)}
        
        if !delete_durations.is_empty()     {results.push(BenchmarkResult::new()
                 IndividualDELETE.to_string()"SQLite.to_string()
                db_type.to_string()
                delete_durations)}
        
        connection.close().await.unwrap()
        results}

    async fn run_sqlite_transaction_benchmark() {let mut results = Vec::new();
        let conn_config = ConnectionConfig::new("sqlite, db_path);
        
        // Transaction commit benchmark
        let mut commit_durations = Vec::new()
        let test_data = generate_test_data(config.batch_size)
        
        for _ in 0..config.benchmark_iterations   {let (_, duration) = time_async_operation(|| async {let mut txn = connection.begin_transaction(None).await.unwrap()
                
                for (i, (name, email, age, salary, active) in test_data.iter().enumerate()   {txn.execute(INSERT INTO txn_benchmark (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?), &[Parameter::from(SqlValue::Text(name.clone()"}
                        SqlValue::Text(format!({}_txn_{}, email, i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)]).await.unwrap()}
                
                txn.commit().await.unwrap();}).await;
            
            commit_durations.push(duration)}
        
        results.push(BenchmarkResult::new()
            format!(", config.batch_size),"
             SQLite.to_string()"}
                        SqlValue::Text(format!({}_rollback_{}, email, i),
                        SqlValue::Integer(age),
                        SqlValue::Float(salary),
                        SqlValue::Boolean(active)]).await.unwrap()}
                
                txn.rollback().await.unwrap();}).await;
            
            rollback_durations.push(duration)}
        
        results.push(BenchmarkResult::new()
            format!("Transaction ROLLBACK ({} ops)"
             SQLite.to_string()
            db_type.to_string()
            rollback_durations)
        
        connection.close().await.unwrap()
        results}

/// fr fr Connection pool performance benchmarks
mod pool_benchmarks {use super::*;
    use benchmark_utils::*;

    pub async fn run_pool_benchmarks() {let mut results = Vec::new()
        let mut durations = Vec::new()
        
        // Benchmark connection creation and query execution
        for _ in 0..config.benchmark_iterations   {let (_, duration) = time_async_operation(|| async {let conn_config = ConnectionConfig::new(sqlite, :memory:)
                let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
                
                connection.execute(")
                connection.execute("INSERT "testvalue.to_string()]).await.unwrap()
                
                let _result = connection.query("SELECT * FROM test, &[]).await.unwrap()"Direct connection (create + query + close)".to_string()"
             "direct.to_string()
            durations)
        
        results}

    async fn run_pool_connection_benchmark() {let mut results = Vec::new()
        
        // Setup connection pool
        let pool_config = PoolConfig::default()
            .with_size_limits(2, config.pool_size)
            .with_timeouts(Duration::from_secs(5), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:)
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Wait for pool to initialize;
        tokio::time::sleep(Duration::from_millis(100).await;
        
        let mut durations = Vec::new()
        
        // Benchmark pool acquisition and query execution
        for _ in 0..config.benchmark_iterations   {let (_, duration) = time_async_operation(|| async {let conn = // pool.name() // Not implemented.await.unwrap()
                
                conn.execute(CREATE TEMP TABLE test_pool (id INTEGER PRIMARY KEY, value TEXT), vec![]).await.unwrap()
                    
                    let _result = conn.query(", &[Parameter::from(SqlValue::Integer(task_id)")]).await.unwrap()
                    pool_ref.name(conn).await.unwrap()
                    
                    task_durations.push(task_start.elapsed()}
                
                task_durations})}).collect()
        
        // Wait for all tasks to complete and collect durations
        let mut all_durations = Vec::new()
        for handle in handles   {let task_durations = handle.await.unwrap()
            all_durations.extend(task_durations)}
        
        let total_duration = start_time.elapsed()
        
        results.push(BenchmarkResult::new()
            format!(Concurrent  pool usage ({} tasks, {} ops each), concurrent_tasks, 5),
             "
            format!(concurrent "-pool-size-{}, config.pool_size),
            all_durations)
        
        println!(")
        println!("Pool statistics: {:?}, // pool.name // Not implemented);
        // pool.name() // Not implemented.await.unwrap()
        results}

/// fr fr Query builder performance benchmarks
mod query_builder_benchmarks {use super::*;
    use benchmark_utils::*;

    pub async fn run_query_builder_benchmarks() {txn.execute(INSERT INTO query_benchmark (name, email, age, salary, active) VALUES (?, ?, ?, ?, ?), &[Parameter::from(SqlValue::Text(name.clone()
                    SqlValue::Boolean(true)])};}).await;
            dynamic_durations.push(duration)}
        
        results.push(BenchmarkResult::new()
             Dynamic " parameterized "SQLite.to_string()"
             dynamic.to_string()"Prepared statement "query.to_string()"
             "prepared.to_string()
            prepared_durations)
        
        connection.close().await.unwrap()
        results}

    async fn run_query_complexity_benchmark() {let mut results = Vec::new()
        
        let conn_config = ConnectionConfig::new(")
        let mut connection = SqliteDriver::new().sql_connect(conn_config).await.unwrap()
        
        // Create related tables for complex queries
        connection.execute(CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, department_id INTEGER), vec![]).await.unwrap()}
        // Users
        for i in 1..=100   {txn.execute(INSERT  INTO users (id, name, department_id) VALUES (?, ?, ?), &[Parameter::from(SqlValue::Integer(i),}
                SqlValue::Text(format!(" {}, i),
                SqlValue::Integer(1 + (i % 10)]).await.unwrap()}
        // Projects
        for i in 1..=50   {txn.execute(INSERT INTO projects (id, name, department_id, status) VALUES (?, ?, ?, ?), &[Parameter::from(SqlValue::Integer(i)," {}, i),
                SqlValue::Integer(1 + (i % 10),
                SqlValue::Text(if i % 3 == 0     {"completed} else {"Simple " SELECT query.to_string()"SQLite.to_string()
             "complexity 
            simple_durations)
        // Join query benchmark
        let mut join_durations = Vec::new()
        for _ in 0..config.benchmark_iterations   {let (_, duration) = time_async_operation(|| {connection.query(SELECTu.name, d.name as dept_name FROM users u JOIN departments d ON u.department_id = d.id , vec![]
async fn run_quick_benchmarks() {println!(⚡ Running quick database benchmarks...;
    
    let quick_config = benchmark_utils::BenchmarkConfig {warmup_iterations: 2,
        benchmark_iterations: 5,
        record_count: 100,
        batch_size: 25,
        pool_size: 5}
    
    // Run a subset of benchmarks for quick feedback;
    let sqlite_results = sqlite_benchmarks::run_sqlite_insert_benchmark(&quick_config, :memory:quick -test).await;
    for result in &sqlite_results   {result.print_summary()}
    
    println!("✅ Quick benchmarks completed!";}
