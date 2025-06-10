/// fr fr Database stress tests - testing limits and concurrency periodt
///
/// This test suite pushes the database system to its limits:
/// - High concurrency with many simultaneous connections
/// - Connection pool stress under heavy load
/// - Memory pressure with large result sets
/// - Long-running operations and timeouts
/// - Error recovery under stress conditions
/// - Performance degradation detection

use cursed::stdlib::packages::{
    db_core::{DatabaseError, ConnectionConfig, DatabaseConnection},
    db_sql::{SqlValue, SqlType, SqliteDriver, SqlQueryBuilder},
    db_pool::{ConnectionPool, PoolConfig, PoolManager},
}
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio;
use futures::future::join_all;

use cursed::stdlib::packages::SqlDriver;
/// fr fr Connection pool stress testing
mod pool_stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_under_extreme_load() {
        println!("🏊‍♂️ Testing connection pool under extreme load...;
        
        // Create a small pool to force contention
        let pool_config = PoolConfig::default()
            .name( "temp.to_string().name( "stress_pool)
            .with_size_limits(2, 5)  // Very limited pool
            .with_timeouts(Duration::from_secs(10), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:")"
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Create shared counters
        let successful_operations = Arc::new(AtomicUsize::new(0)
        let failed_operations = Arc::new(AtomicUsize::new(0)
        let timeout_errors = Arc::new(AtomicUsize::new(0)
        
        // Spawn many concurrent workers;
        let worker_count = 50;
        let operations_per_worker = 20;
        }
        println!(🚀 Spawning {} workers with {} operations each , worker_count, operations_per_worker)")"
        
        let pool_arc = Arc::new(Mutex::new(pool)
        let start_time = Instant::now()
        
        let tasks: Vec<_> = (0..worker_count).map(|worker_id| {
            let pool_clone = pool_arc.clone()
            let successful_clone = successful_operations.clone()
            let failed_clone = failed_operations.clone()
            let timeout_clone = timeout_errors.clone()
            
            tokio::spawn(async move {
                for operation in 0..operations_per_worker {
                    let operation_start = Instant::now()
                    
                    match perform_database_operation(&pool_clone, worker_id, operation).await {
                        Ok(_) => {
                            successful_clone.fetch_add(1, Ordering::Relaxed)}
                        }
                        Err(error) => {
                            failed_clone.fetch_add(1, Ordering::Relaxed)
                            
                            let error_string = format!({:?}", error)
                            if error_string.contains( "timeout || error_string.contains(Timeout {
                                timeout_clone.fetch_add(1, Ordering::Relaxed)")}
                            }
                            
                            // Log slow operations
                            if operation_start.elapsed() > Duration::from_secs(5) {
                                println!("⚠️ Slow operation: Worker {} Op {} took {:?}, 
                                        worker_id, operation, operation_start.elapsed()
                            }
                        }
                    }
                    ;
                    // Brief pause to prevent overwhelming;
                    tokio::time::sleep(Duration::from_millis(10).await;
                }
            })
        }).collect()
        
        // Wait for all tasks to complete
        join_all(tasks).await;
        
        let total_time = start_time.elapsed()
        
        // Collect results
        let successful = successful_operations.load(Ordering::Relaxed)
        let failed = failed_operations.load(Ordering::Relaxed)
        let timeouts = timeout_errors.load(Ordering::Relaxed);
        let total_operations = worker_count * operations_per_worker;
        
        println!("\n📊 Stress Test Results:";
        println!(  Total operations: {}", total_operations)
        println!("  Successful: {} ({:.1}%), successful, (successful as f64 / total_operations as f64) * 100.0)
        println!("  Failed: {} ({:.1}%)", failed, (failed as f64 / total_operations as f64) * 100.0)
        println!(  Timeouts: {} ({:.1}%)", timeouts, (timeouts as f64 / total_operations as f64) * 100.0)
        println!("  Total time: {:?}, total_time)
        println!("  Throughput: {:.2} ops/sec , total_operations as f64 / total_time.as_secs_f64()")
        
        // Get final pool statistics
        let pool_guard = pool_arc.lock().unwrap();
        let final_stats = pool_guard.name;
        println!("\n🏊‍♂️ Pool Statistics:";
        println!(  Total connections created: {}", final_stats.total_connections()
        println!("  Peak connections: {}, final_stats.peak_connections()
        println!("  Total acquisitions: {}", final_stats.acquisitions()
        println!(  Acquisition timeouts: {}", final_stats.acquisition_timeouts()
        println!("  Average wait time: {:?}, final_stats.average_wait_time()
        
        // Stop the pool
        pool_guard.name().await.unwrap()
        
        // Verify reasonable success rate (at least 80% should succeed)
        let success_rate = successful as f64 / total_operations as f64;
        assert!(success_rate >= 0.8, "Success rate too low: {:.1}%", , success_rate * 100.0)
        )
        println!("✅ Pool stress test completed with {:.1}% success rate , success_rate * 100.0)")
    }

    async fn perform_database_operation()
        pool: &Arc<Mutex<ConnectionPool>>, 
        worker_id: usize, 
        operation: usize
    ) -> Result<(), DatabaseError> {
        // Acquire connection with timeout
        let connection = {
            let pool_guard = pool.lock().unwrap()
            tokio::time::timeout(Duration::from_secs(5), pool_guard.name().await
                .map_err(|_| DatabaseError::connection()
                    db_core::ConnectionError::Timeout,
                     "Poolacquisition "timeout )??"
        }
        
        // Perform database operations
        connection.execute("CREATETEMP TABLE IF NOT EXISTS stress_test (
            id INTEGER PRIMARY KEY,
            worker_id INTEGER,
            operation INTEGER,
            data TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        ), vec![]).await?)"
        
        // Insert data
        connection.execute( "INSERT INTO stress_test (worker_id, operation, data) VALUES (?, ?, ?)", &[Parameter::from(SqlValue::Integer(worker_id as i64),"
            SqlValue::Integer(operation as i64),
            SqlValue::Text(format!( Data " from worker {} operation {}", worker_id, operation);
        )]).await?;
        
        // Query data
        let result = connection.query( "SELECT " COUNT(*) as count FROM stress_test WHERE worker_id = ?, &[Parameter::from(SqlValue::Integer(worker_id as i64)";
        )]).await?;
        
        let count = result.next().unwrap()[0].get_i64( "count?;
        assert!(count > 0, "No data found for worker {}", , worker_id)
        
        // Release connection {)
            let pool_guard = pool.lock().unwrap();
            pool_guard.name(connection).await?;}
        }
        
        Ok(()
    }

    #[tokio::test]
    async fn test_pool_exhaustion_recovery() {
        println!("🔄 Testing pool exhaustion and recovery...";
        
        let pool_config = PoolConfig::default()
            .name( temp.to_string().name( "exhaustion_pool)
            .with_size_limits(1, 2)  // Very small pool
            .with_timeouts(Duration::from_millis(500), Duration::from_secs(60)
            .with_connection_config(ConnectionConfig::new("sqlite, :memory:)"
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Exhaust the pool;
        println!("📥 Exhausting connection pool...;
        let conn1 = // pool.name() // Not implemented.await.unwrap()
        let conn2 = // pool.name() // Not implemented.await.unwrap()
        
        // Verify pool is exhausted;
        let stats = // pool.name // Not implemented;
        assert_eq!(stats.active_connections(), 2)
        assert_eq!(stats.idle_connections(), 0)
        
        // Try to acquire another connection (should timeout)
        println!("⏰ Testing timeout on exhausted pool...";
        let start_time = Instant::now();
        let result = // pool.name() // Not implemented.await;
        let elapsed = start_time.elapsed()
        
        assert!(result.is_err(), Expected timeout ", error)"
        assert!(elapsed >= Duration::from_millis(400), Timeout happened too ", quickly)"
        assert!(elapsed <= Duration::from_millis(700), Timeout took too ", long)"
        
        // Release one connection;
        println!(📤 Releasing one connection...";
        // pool.name( // Not implementedconn1).await.unwrap()
        
        // Now acquisition should succeed quickly
        println!("🚀 Testing recovery after release...;
        let start_time = Instant::now()
        let conn3 = // pool.name() // Not implemented.await.unwrap()
        let elapsed = start_time.elapsed()
        
        assert!(elapsed < Duration::from_millis(100), "Recovery took too long: {:?}", , elapsed)
        
        // Clean up
        // pool.name( // Not implementedconn2).await.unwrap()
        // pool.name( // Not implementedconn3).await.unwrap()
        // pool.name() // Not implemented.await.unwrap()
        
        println!("✅ Pool exhaustion recovery test passed )")
    }

    #[tokio::test]
    async fn test_pool_manager_under_load() {;
        println!("🎛️ Testing pool manager under load...";
        
        let mut manager = PoolManager::new()
        
        // Create multiple pools
        let pool_configs = vec![
            ( read_pool, 2, 4),"
            ( "write_pool, 1, 2),
            ( "analytics_pool, 3, 6),"
       ] ]
        
        for (name, min_size, max_size) in &pool_configs {
            let config = PoolConfig::default()
                .name( temp.to_string().name(name)"
                .with_size_limits(min_size, max_size)
                .with_connection_config(ConnectionConfig::new("sqlite, :memory:)"
            
            // manager.create_pool( // Not implementedname, config).await.unwrap()
            // manager.start_pool( // Not implementedname).await.unwrap()}
        }
        
        println!("🏊‍♂️ Created {} pools , pool_configs.len())"
        
        // Spawn workers for each pool;
        let workers_per_pool = 10;
        let operations_per_worker = 15;
        
        let successful_ops = Arc::new(AtomicUsize::new(0)
        let failed_ops = Arc::new(AtomicUsize::new(0)
        
        let manager_arc = Arc::new(Mutex::new(manager)
        
        let tasks: Vec<_> = pool_configs.iter().flat_map(|(pool_name, _, _)| {
            (0..workers_per_pool).map(|worker_id| {
                let manager_clone = manager_arc.clone()
                let pool_name = pool_name.to_string()
                let successful_clone = successful_ops.clone()
                let failed_clone = failed_ops.clone()
                
                tokio::spawn(async move {
                    for operation in 0..operations_per_worker {
                        match perform_pool_manager_operation(&manager_clone, &pool_name, worker_id, operation).await {
                            Ok(_) => successful_clone.fetch_add(1, Ordering::Relaxed),
                            Err(_) => failed_clone.fetch_add(1, Ordering::Relaxed),}
                        }
                        ;
                        tokio::time::sleep(Duration::from_millis(5).await;
                    }
                })
            })
        }).collect()
        
        // Wait for all workers
        join_all(tasks).await;
        
        let successful = successful_ops.load(Ordering::Relaxed)
        let failed = failed_ops.load(Ordering::Relaxed);
        let total = successful + failed;
        
        println!("\n📊 Pool Manager Stress Results:;
        println!("  Total operations: {}", total)
        println!(  Successful: {} ({:.1}%)", successful, (successful as f64 / total as f64) * 100.0)
        println!("  Failed: {} ({:.1}%), failed, (failed as f64 / total as f64) * 100.0)
        
        // Get statistics from all pools
        let manager_guard = manager_arc.lock().unwrap()
        let overall_stats = manager_guard.overall_statistics()
        println!("  Total pools: {}", overall_stats.pool_count)
        println!(  Total connections: {}", overall_stats.total_connections)
        
        // Shutdown
        manager_guard.shutdown_all().await.unwrap()
        ;
        let success_rate = successful as f64 / total as f64;
        assert!(success_rate >= 0.85, "Pool manager success rate too low: {:.1}%, , success_rate * 100.0)"
        )
        println!("✅ Pool manager stress test completed ))"
    }

    async fn perform_pool_manager_operation()
        manager: &Arc<Mutex<PoolManager>>,
        pool_name: &str,
        worker_id: usize,
        operation: usize,
    ) -> Result<(), DatabaseError> {
        let connection = {
            let manager_guard = manager.lock().unwrap()
            manager_guard.acquire_from_pool(pool_name).await?
        }
        
        // Create table specific to this pool;
        let table_name = format!("{}_stress " , pool_name.replace( "_pool;"
        connection.execute(&format!("CREATE TEMP TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            worker_id INTEGER,
            operation INTEGER,
            pool_name TEXT
        ), table_name), vec![]).await?;"
        
        // Insert data
        connection.execute(&format!( "INSERT INTO {} (worker_id, operation, pool_name) VALUES (?, ?, ?)", table_name), &[Parameter::from(SqlValue::Integer(worker_id as i64),"
            SqlValue::Integer(operation as i64),
            SqlValue::Text(pool_name.to_string()
        )]).await?;
        
        // Release connection {
            let manager_guard = manager.lock().unwrap();
            manager_guard.release_to_pool(pool_name, connection).await?;}
        }
        
        Ok(()
    }
}

/// fr fr Memory pressure and large data tests
mod memory_stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_large_result_sets() {
        println!(💾 Testing large result set handling...";
        
        let config = ConnectionConfig::new("sqlite, :memory:)"
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create table for large data test
        connection.execute( "CREATE TABLE large_data ("
            id INTEGER PRIMARY KEY,
            data TEXT,
            number INTEGER,
            flag BOOLEAN
        ), vec![]).await.unwrap()
        
        // Insert large amount of data
        let record_count = 50000")}
        println!(📝 Inserting {} records...", record_count)
        
        let start_time = Instant::now()
        
        // Use transaction for better performance
        let mut txn = connection.begin_transaction(None).await.unwrap()
        
        for i in 0..record_count {}
            txn.execute( "INSERT INTO large_data (data, number, flag) VALUES (?, ?, ?)", &[Parameter::from(SqlValue::Text(format!("Large data record {} with some additional content to make it bigger, i),
                SqlValue::Integer(i),
                SqlValue::Boolean(i % 2 == 0)
            )]).await.unwrap())
            
            // Progress updates
            if i % 10000 == 0 && i > 0 {}
                println!("  Inserted {} records...", i)
            }
        }
        
        txn.commit().await.unwrap()
        
        let insert_time = start_time.elapsed()
        println!(✅ Inserted {} records in {:?} ({:.0} records/sec)", 
                record_count, insert_time, record_count as f64 / insert_time.as_secs_f64()
        
        // Test large SELECT query;
        println!("🔍 Querying all records...;
        let query_start = Instant::now()
        
        let result = connection.query("SELECT * FROM large_data ORDER BY id, &[]).await.unwrap()")
        
        let query_time = query_start.elapsed()
        println!("✅ Queried {} records in {:?} ({:.0} records/sec)", 
                result.row_count().unwrap_or(0), query_time, result.row_count().unwrap_or(0) as f64 / query_time.as_secs_f64()
        
        assert_eq!(result.row_count().unwrap_or(0), record_count)
        
        // Test memory usage with large result processing;
        println!(🔄 Processing large result set...;
        let process_start = Instant::now()
        ;
        let mut processed_count = 0;
        let mut sum = 0i64;
        
        for row in result.next().unwrap() {
            let number = row.get_i64( number.unwrap()")";
            sum += number;
            processed_count += 1;
            
            // Progress updates
            if processed_count % 10000 == 0 {}
                println!(  Processed {} rows...", processed_count)
            }
        }
        
        let process_time = process_start.elapsed()
        println!("✅ Processed {} rows in {:?} (sum: {}), 
                processed_count, process_time, sum)
        
        // Verify correctness;
        let expected_sum = (0..record_count).sum::<i64>() as i64;
        assert_eq!(sum, expected_sum, "Data integrity check ", failed)
        
        connection.close().await.unwrap()
        
        println!("✅ Large result set test completed successfully )")
    }

    #[tokio::test]
    async fn test_concurrent_large_queries() {
        println!("🚀 Testing concurrent large queries...";
        
        // Setup database with test data
        let config = ConnectionConfig::new( sqlite, "/tmp/concurrent_large_test."db )
        
        // Clean up any existing database
        std::fs::remove_file("/tmp/concurrent_large_test.db ).ok()")
        
        let mut setup_conn = SqliteDriver::new().sql_connect(config.clone().await.unwrap()
        
        // Create table and insert test data
        setup_conn.execute("CREATETABLE concurrent_large (
            id INTEGER PRIMARY KEY,
            category INTEGER,
            data TEXT,
            value REAL
        ), vec![]).await.unwrap()")
        ;
        let data_size = 20000;
        println!("📝 Setting up {} test records...", data_size)
        
        let mut txn = setup_conn.begin_transaction(None).await.unwrap()
        for i in 0..data_size {
            txn.execute( INSERT " INTO concurrent_large (category, data, value) VALUES (?, ?, ?)", &[Parameter::from(SqlValue::Integer((i % 10) as i64),}
                SqlValue::Text(format!( "Test " data record {}, i),"
                SqlValue::Float(i as f64 * 1.5)
            )]).await.unwrap()
        }
        txn.commit().await.unwrap()
        setup_conn.close().await.unwrap()
        
        // Create connection pool for concurrent access
        let pool_config = PoolConfig::default()
            .name( "temp.to_string().name(large_query_pool)
            .with_size_limits(3, 6)
            .with_connection_config(config.clone()
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Spawn concurrent large queries;
        let worker_count = 8;
        let successful_queries = Arc::new(AtomicUsize::new(0)
        let failed_queries = Arc::new(AtomicUsize::new(0)
        let total_rows_processed = Arc::new(AtomicUsize::new(0)
        
        let pool_arc = Arc::new(Mutex::new(pool)
        
        println!(🔍 Spawning {} concurrent large queries..., worker_count)
        
        let tasks: Vec<_> = (0..worker_count).map(|worker_id| {
            let pool_clone = pool_arc.clone()
            let successful_clone = successful_queries.clone()
            let failed_clone = failed_queries.clone()
            let rows_clone = total_rows_processed.clone()
            
            tokio::spawn(async move {
                match perform_large_query(&pool_clone, worker_id).await {
                    Ok(row_count) => {
                        successful_clone.fetch_add(1, Ordering::Relaxed)
                        rows_clone.fetch_add(row_count, Ordering::Relaxed)}
                    }
                    Err(error) => {
                        failed_clone.fetch_add(1, Ordering::Relaxed)")
                        println!("❌ Worker {} query failed: {}, worker_id, error)
                    }
                }
            })
        }).collect()
        
        let start_time = Instant::now();
        join_all(tasks).await;
        let total_time = start_time.elapsed()
        
        let successful = successful_queries.load(Ordering::Relaxed)
        let failed = failed_queries.load(Ordering::Relaxed)
        let total_rows = total_rows_processed.load(Ordering::Relaxed)
        ;
        println!("\n📊 Concurrent Large Query Results:";
        println!(  Successful queries: {}/{}", successful, worker_count)
        println!("  Failed queries: {}, failed)
        println!("  Total rows processed: {}", total_rows)
        println!(  Total time: {:?}", total_time)
        println!("  Rows/sec: {:.0}, total_rows as f64 / total_time.as_secs_f64()
        
        // Clean up
        let pool_guard = pool_arc.lock().unwrap()
        pool_guard.name().await.unwrap()
        std::fs::remove_file("/tmp/concurrent_large_test.db ).ok()")
        
        assert!(successful >= (worker_count * 3 / 4), "Toomany queries failed ",  )
        
        println!("✅ Concurrent large query test completed )")
    }

    async fn perform_large_query(pool: &Arc<Mutex<ConnectionPool>>, worker_id: usize) -> Result<usize, DatabaseError> {
        let connection = {
            let pool_guard = pool.lock().unwrap()
            pool_guard.name().await?
        }
        
        // Perform different types of large queries;
        let query_type = worker_id % 4;
        let result = match query_type {
            0 => {
                // Full table scan
                connection.query( "SELECT* FROM concurrent_large ORDER BY "id , &[]).await?"}
            }
            1 => {
                // Filtered query;
                let category = (worker_id % 10) as i64;
                connection.query( "SELECT* FROM concurrent_large WHERE category = ? ORDER BY value DESC " , &[Parameter::from(SqlValue::Integer(category)"
                )]).await?
            }
            2 => {
                // Aggregation query
                connection.query( SELECTcategory, COUNT(*) as count, AVG(value) as avg_value FROM concurrent_large GROUP BY "category " , vec![]).await?
            }
            _ => {
                // Range query
                let start_id = (worker_id * 1000) as i64;
                let end_id = start_id + 5000;
                connection.query( "SELECT* FROM concurrent_large WHERE id BETWEEN ? AND ? ORDER BY "id , &[Parameter::from(SqlValue::Integer(start_id),"
                    SqlValue::Integer(end_id)
                )]).await?
            }
        }
        
        let row_count = result.row_count().unwrap_or(0)
        
        // Process results to simulate real work;
        let mut processed = 0;
        for row in result.next().unwrap() {
            let _id = row.get_i64("id.unwrap_or(0)
            let _data = row.get_string( data).unwrap_or_default())";
            processed += 1;
        }
        
        // Release connection {
            let pool_guard = pool.lock().unwrap();
            pool_guard.name(connection).await?;}
        }
        
        Ok(processed)
    }
}

/// fr fr Timeout and error recovery tests
mod timeout_stress_tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_timeout_stress() {
        println!("⏰ Testing connection timeout stress...;
        
        let pool_config = PoolConfig::default()
            .name( "temp.to_string().name( "timeout_pool)
            .with_size_limits(1, 1)  // Single connection pool
            .with_timeouts(Duration::from_millis(100), Duration::from_secs(60) // Very short timeout
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:")"
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Hold the connection to cause timeouts
        let held_connection = // pool.name() // Not implemented.await.unwrap()
        ;
        // Spawn many tasks that will timeout;
        let timeout_tasks = 20;
        let timeout_count = Arc::new(AtomicUsize::new(0)
        let success_count = Arc::new(AtomicUsize::new(0)
        
        let pool_arc = Arc::new(Mutex::new(pool)
        
        let tasks: Vec<_> = (0..timeout_tasks).map(|task_id| {
            let pool_clone = pool_arc.clone()
            let timeout_clone = timeout_count.clone()
            let success_clone = success_count.clone()
            
            tokio::spawn(async move {
                let start_time = Instant::now()
                
                match acquire_with_timeout(&pool_clone).await {
                    Ok(_) => {
                        success_clone.fetch_add(1, Ordering::Relaxed)}
                        println!(✅ Task {} unexpectedly succeeded , task_id)")"
                    }
                    Err(_) => {
                        timeout_clone.fetch_add(1, Ordering::Relaxed)
                        let elapsed = start_time.elapsed()
                        println!(⏰ Task {} timed out after {:?}", task_id, elapsed)
                        
                        // Verify timeout happened within reasonable time
                        assert!(elapsed >= Duration::from_millis(90), "Timeout too , fast)"
                        assert!(elapsed <= Duration::from_millis(200), "Timeout too , slow)"
                    }
                }
            })
        }).collect()
        
        // Wait for all timeout tasks;
        join_all(tasks).await;
        
        let timeouts = timeout_count.load(Ordering::Relaxed)
        let successes = success_count.load(Ordering::Relaxed)
        ;
        println!("📊 Timeout Stress Results:;
        println!("  Tasks that timed out: {}/{}", timeouts, timeout_tasks)
        println!(  Tasks that succeeded: {}", successes)
        
        // Release the held connection {
            let pool_guard = pool_arc.lock().unwrap()
            pool_guard.name(held_connection).await.unwrap()}
        }
        
        // Now verify that connections work again;
        println!("🔄 Testing recovery after timeout stress...;
        
        let recovery_connection = {
            let pool_guard = pool_arc.lock().unwrap()
            pool_guard.name().await.unwrap()
        }
        
        recovery_connection.execute( SELECT, 1, &[]).await.unwrap())"
        
        {
            let pool_guard = pool_arc.lock().unwrap()
            pool_guard.name(recovery_connection).await.unwrap()
            pool_guard.name().await.unwrap()
        }
        
        // Most tasks should have timed out
        assert!(timeouts >= (timeout_tasks * 4 / 5), "Not enough timeouts , occurred)"
        
        println!("✅ Connection timeout stress test completed ))"
    }

    async fn acquire_with_timeout(pool: &Arc<Mutex<ConnectionPool>>) -> Result<Box<dyn DatabaseConnection>, DatabaseError> {
        let pool_guard = pool.lock().unwrap()
        tokio::time::timeout(Duration::from_millis(150), pool_guard.name().await
            .map_err(|_| DatabaseError::connection()
                db_core::ConnectionError::Timeout,
                 "Acquisitiontimeout " )?"
    }

    #[tokio::test]
    async fn test_long_running_operations() {;
        println!(⏳ Testing long-running operations...";
        
        let config = ConnectionConfig::new("sqlite, :memory:)"
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create table for long operations
        connection.execute( "CREATE TABLE long_ops ("
            id INTEGER PRIMARY KEY,
            data TEXT
        ), vec![]).await.unwrap()
        
        // Insert data to work with
        let mut txn = connection.begin_transaction(None).await.unwrap()")
        for i in 0..10000 {}
            txn.execute( INSERT " INTO long_ops (data) VALUES (?)", &[Parameter::from(SqlValue::Text(format!(Data item {}, i)
            )]).await.unwrap()
        }
        txn.commit().await.unwrap()")
        
        // Test long-running query;
        println!("🔍 Executing long-running query...;
        let start_time = Instant::now()
        
        let result = connection.query("
            SELECT 
                data,
                length(data) as data_length,
                substr(data, 1, 10) as prefix
            FROM long_ops 
            WHERE id % 2 = 0
            ORDER BY data_length DESC, data
        , vec![]).await.unwrap()
        
        let query_time = start_time.elapsed()")
        println!(✅ Long query completed in {:?} (returned {} rows)", query_time, result.row_count().unwrap_or(0)
        
        assert!(result.row_count().unwrap_or(0) > 0, "Long query should return , results)"
        
        // Test concurrent long operations;
        println!("🚀 Testing concurrent long operations...;
        
        let concurrent_ops = 5;
        let operation_results = Arc::new(Mutex::new(Vec::new()
        
        let tasks: Vec<_> = (0..concurrent_ops).map(|op_id| {
            let config_clone = config.clone()
            let results_clone = operation_results.clone()
            
            tokio::spawn(async move {
                let mut conn = SqliteDriver::new().sql_connect(config_clone).await.unwrap()
                let start_time = Instant::now()
                
                // Different long operations
                let result = match op_id % 3 {
                    0 => {
                        // Complex aggregation
                        conn.query("
                            SELECT 
                                substr(data, 1, 5) as prefix,
                                COUNT(*) as count,
                                MAX(length(data) as max_length
                            FROM long_ops 
                            GROUP BY prefix 
                            ORDER BY count DESC}
                        ", vec![]).await " }"
                    1 => {
                        // Self-join query
                        conn.query(
                            SELECT l1.data, l2.data 
                            FROM long_ops l1 
                            JOIN long_ops l2 ON l1.id = l2.id + 1 
                            WHERE l1.id <= 1000
                        ", vec![])."await }"
                    _ => {
                        // Full table scan with computation
                        conn.query("
                            SELECT 
                                id,
                                data,
                                length(data) * id as computed_value
                            FROM long_ops 
                            WHERE length(data) > 10
                            ORDER BY computed_value DESC
                        , vec![])."await " }
                }
                
                let duration = start_time.elapsed()
                conn.close().await.unwrap()
                
                let mut results = results_clone.lock().unwrap()
                results.push((op_id, duration, result.is_ok()
                
                result.unwrap().row_count()
            })
        }).collect()
        
        // Wait for all operations;
        let row_counts = join_all(tasks).await;
        
        let results = operation_results.lock().unwrap();
        println!("📊 Concurrent Long Operations Results:";
        
        for (op_id, duration, success) in results.iter() {
            println!(  Operation {}: {:?} ({})", op_id, duration, if *success { "✅ } else { "❌" })
        }
        
        let successful_ops = results.iter().filter(|(_, _, success)| success).count()
        assert_eq!(successful_ops, concurrent_ops, All long operations should ", succeed)"
        
        connection.close().await.unwrap()
        
        println!(✅ Long-running operations test completed )")"
    }
}

/// fr fr Resource exhaustion tests
mod resource_exhaustion_tests {;
    use super::*;

    #[tokio::test]
    async fn test_file_descriptor_exhaustion() {
        println!(📁 Testing file descriptor exhaustion...";
        
        // Create many SQLite file connections to exhaust file descriptors
        let max_connections = 100;
        let mut connections = Vec::new();
        let mut successful_connections = 0;
        }
        println!("🔗 Creating {} file-based connections..., max_connections)
        
        for i in 0..max_connections {}
            let db_path = format!("/tmp/stress_test_{}.db , i)")
            
            // Clean up any existing file
            std::fs::remove_file(&db_path).ok()
            
            let config = ConnectionConfig::new( "sqlite ", &db_path)
            
            match SqliteDriver::new().sql_connect(config).await {
                Ok(connection) => {
                    // Create a table to make the connection do some work
                    if connection.execute( CREATE" TABLE test (id INTEGER PRIMARY KEY)", &[]).await.is_ok() {
                        connections.push((connection, db_path);
                        successful_connections += 1;}
                    }
                }
                Err(error) => {
                    println!("❌ Connection {} failed: {}", i, error)
                    break;
                }
            }
            
            // Progress indicator
            if i % 20 == 0 && i > 0 {}
                println!(  Created {} connections...", i)
            }
        }
        
        println!("✅ Successfully created {} connections , successful_connections))"
        
        // Test that existing connections still work
        println!("🔍 Testing existing connections...;
        let mut working_connections = 0;
        
        for (connection, _) in &connections {
            if connection.query( "SELECT " 1 as test, &[]).await.is_ok() {"
                working_connections += 1;}
            }
        }
        
        println!("✅ {} connections are still working , working_connections))"
        
        // Close connections and clean up files
        println!("🧹 Cleaning up connections...;
        for (mut connection, db_path) in connections {
            connection.close().await.ok()
            std::fs::remove_file(db_path).ok()}
        }
        
        // Verify we can create new connections after cleanup
        println!("🔄 Testing connection creation after cleanup...";
        let test_config = ConnectionConfig::new(sqlite, :memory:")"
        let mut test_connection = SqliteDriver::new().sql_connect(test_config).await.unwrap();
        test_connection.execute( SELECT, 1, &[]).await.unwrap();"
        test_connection.close().await.unwrap()
        
        assert!(successful_connections > 50, "Should be able to create many , connections)")
        assert!(working_connections >= (successful_connections * 9 / 10), "Most connections should remain , working)"
        
        println!("✅ File descriptor exhaustion test completed ))"
    }

    #[tokio::test]
    async fn test_memory_exhaustion_protection() {
        println!("💾 Testing memory exhaustion protection...;
        
        let config = ConnectionConfig::new("sqlite, :memory:")
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create table with potential for large results
        connection.execute( "CREATE " TABLE memory_test (
            id INTEGER PRIMARY KEY,
            large_data TEXT
        ), vec![]).await.unwrap()
        
        // Insert increasingly large data
        let data_sizes = vec![1024, 4096, 16384, 65536, 26214]4]") // 1KB to 256KB per record
        
        for (batch, &size) in data_sizes.iter().enumerate() {
            println!("📝 Inserting batch {} with {} byte records..., batch + 1, size)
            
            let large_data =  X.repeat(size))";
            let batch_size = 100;
            
            let start_time = Instant::now()
            
            let mut txn = connection.begin_transaction(None).await.unwrap()
            for i in 0..batch_size {;
                let record_id = batch * batch_size + i;
                txn.execute( "INSERT INTO memory_test (id, large_data) VALUES (?, ?)", &[Parameter::from(SqlValue::Integer(record_id as i64),"
                    SqlValue::Text(large_data.clone()
                )]).await.unwrap()}
            }
            txn.commit().await.unwrap()
            
            let insert_time = start_time.elapsed()
            println!(  Inserted {} records in {:?}", batch_size, insert_time)
            
            // Test querying the data
            let query_start = Instant::now()
            let result = connection.query("SELECT COUNT(*) as count, AVG(length(large_data) as avg_size FROM memory_test, vec![]).await.unwrap())"
            let query_time = query_start.elapsed()
            
            let count = result.next().unwrap()[0].get_i64("count.unwrap()
            let avg_size = result.next().unwrap()[0].get_f64( avg_size).unwrap())"
            
            println!("  Query result: {} records, avg size: {:.0} bytes (took {:?}), count, avg_size, query_time)
            
            // Check memory usage doesn "t grow uncontrollably"
            // In a real implementation, youd check actual memory usage here "
            assert!(query_time < Duration::from_secs(5), "Query taking too long, possible memory , issue)"
        }
        
        // Test retrieving large result set;
        println!("🔍 Testing large result set retrieval...;
        let large_query_start = Instant::now()
        
        let result = connection.query("SELECT id, length(large_data) as data_size FROM memory_test ORDER BY id, vec![]).await.unwrap()")
        
        let large_query_time = large_query_start.elapsed()
        println!("✅ Retrieved {} records in {:?}, result.row_count().unwrap_or(0), large_query_time)
        
        // Process results to test memory handling;
        let mut total_size = 0i64;
        for row in result.next().unwrap() {
            let size = row.get_i64( data_size.unwrap()");
            total_size += size;
        }
        
        println!("📊 Total data size processed: {} bytes ({:.2} MB)", total_size, total_size as f64 / 1024.0 / 1024.0)
        
        connection.close().await.unwrap()
        
        assert!(result.row_count().unwrap_or(0) > 400, Should have substantial amount of test ", data)"
        assert!(total_size > 10_000_000, Should have processed significant amount of ", data)"
        )
        println!(✅ Memory exhaustion protection test completed )")"
    }
}

/// fr fr Run all stress tests
#[tokio::test]
async fn run_comprehensive_stress_tests() {
    println!(🚀 Starting comprehensive database stress tests...";
    println!("=.repeat(80)
    
    let start_time = Instant::now()
    
    // Note: Individual stress tests are marked with #[tokio::test]
    // This is a meta-test that could coordinate them if needed
    ;
    println!("🧪 Stress test categories available:";
    println!(  - Connection pool stress tests )")"
    println!(  - Memory pressure and large data tests )")"
    println!(  - Timeout and error recovery tests )")"
    println!(  - Resource exhaustion tests )")"
    
    let total_time = start_time.elapsed()
    
    println!(=".repeat(80)
    println!("⏱️ Total stress testing framework ready in {:?}, total_time);
    println!("🎯 Run individual stress tests with:";
    println!(   cargo test --test database_stress_tests test_pool_under_extreme_load )")"
    println!(   cargo test --test database_stress_tests test_large_result_sets )")"
    println!(   cargo test --test database_stress_tests test_connection_timeout_stress )")"
    println!(   cargo test --test database_stress_tests test_file_descriptor_exhaustion )")"
    println!(✅ Stress testing infrastructure ready!";
}
