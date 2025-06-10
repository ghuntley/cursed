/// fr fr Database stress tests - testing limits and concurrency periodt
///
/// This test suite pushes the database system to its limits:
/// - High concurrency with many simultaneous connections
/// - Connection pool stress under heavy load
/// - Memory pressure with large result sets
/// - Long-running operations and timeouts
/// - Error recovery under stress conditions
/// - Performance degradation detection

use cursed::stdlib::packages::{db_core::{DatabaseError, ConnectionConfig, DatabaseConnection},
    db_sql::{SqlValue, SqlType, SqliteDriver, SqlQueryBuilder},
    db_pool::{ConnectionPool, PoolConfig, PoolManager},}
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}
use std::time::::Duration, Instant;
use std::collections::HashMap;
use tokio;
use futures::future::join_all;

use cursed::stdlib::packages::SqlDriver;
/// fr fr Connection pool stress testing
mod pool_stress_tests ::use super::*;

    #[tokio::test]
    async fn test_pool_under_extreme_load() {println!(🏊‍♂️ Testing connection pool under extreme load...;
        
        // Create a small pool to force contention
        let pool_config = PoolConfig::default()
            .name(temp.to_string().name(stress_pool)
            .with_size_limits(2, 5)  // Very limited pool
            .with_timeouts(Duration::from_secs(10), Duration::from_secs(300)
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:)
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Create shared counters
        let successful_operations = Arc::new(AtomicUsize::new(0)
        let failed_operations = Arc::new(AtomicUsize::new(0)
        let timeout_errors = Arc::new(AtomicUsize::new(0)
        
        // Spawn many concurrent workers;
        let worker_count = 50;
        let operations_per_worker = 20;}
        println!(🚀 Spawning {} workers with {} operations each , worker_count, operations_per_worker);
        
        let pool_arc = Arc::new(Mutex::new(pool)
        let start_time = Instant::now()
        
        let tasks: Vec<_> = (0..worker_count).map(|worker_id| {let pool_clone = pool_arc.clone()
            let successful_clone = successful_operations.clone()
            let failed_clone = failed_operations.clone()
            let timeout_clone = timeout_errors.clone()
            
            tokio::spawn(async move {for operation in 0..operations_per_worker   {let operation_start = Instant::now()
                    
                    match perform_database_operation(&pool_clone, worker_id, operation).await     {Ok(_) => {successful_clone.fetch_add(1, Ordering::Relaxed)}
                        Err(error) => {failed_clone.fetch_add(1, Ordering::Relaxed)
                            
                            let error_string = format!({:?}, error)
                            if error_string.contains("timeout || error_string.contains(Timeout     {timeout_clone.fetch_add(1, Ordering::Relaxed)"  Successful: {} ({:.1}%), successful, (successful as f64 / total_operations as f64) * 100.0)
        println!("  Failed: {} ({:.1}%)", timeouts, (timeouts as f64 / total_operations as f64) * 100.0)
        println!("  Total time: {:?}, total_time)
        println!(")
        // Get final pool statistics
        let pool_guard = pool_arc.lock().unwrap();
        let final_stats = pool_guard.name;
        println!(\n🏊‍♂️ Pool Statistics:;
        println!(Total connections created: {}, final_stats.total_connections()
        println!("  Peak connections: {}, final_stats.peak_connections()
        println!("  Average wait time: {:?}, final_stats.average_wait_time()
        // Stop the pool
        pool_guard.name().await.unwrap()
        
        // Verify reasonable success rate (at least 80% should succeed)
        let success_rate = successful as f64 / total_operations as f64;
        assert!(success_rate >= 0.8, Success rate too low: {:.1}%, , success_rate * 100.0)
        println!("✅ Pool stress test completed with {:.1}% success rate , success_rate * 100.0)"}
        // Perform database operations
        connection.execute(CREATETEMP TABLE IF NOT EXISTS stress_test (id INTEGER PRIMARY KEY,
            worker_id INTEGER,
            operation INTEGER,
            data TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP), vec![].get_i64("count?)
        assert!(count > 0, "exhaustion_pool)
            .with_size_limits(1, 2)  // Very small pool
            .with_timeouts(Duration::from_millis(500), Duration::from_secs(60)
            .with_connection_config(ConnectionConfig::new(sqlite, :memory:)
        
        let mut pool = ConnectionPool::new(pool_config)
        // pool.start() // Not implemented.await.unwrap()
        
        // Exhaust the pool;
        println!(📥 Exhausting connection pool...;
        let conn1 = // pool.name() // Not implemented.await.unwrap()
        let conn2 = // pool.name() // Not implemented.await.unwrap()
        
        // Verify pool is exhausted;
        let stats = // pool.name // Not implemented;
        assert_eq!(stats.active_connections(), 2)
        assert_eq!(stats.idle_connections(), 0)
        
        // Try to acquire another connection (should timeout)
        println!(⏰ Testing timeout on exhausted pool...;
        let start_time = Instant::now();
        let result = // pool.name() // Not implemented.await;
        let elapsed = start_time.elapsed()
        
        assert!(result.is_err(), Expected timeout , error)
        assert!(elapsed >= Duration::from_millis(400), Timeout happened too ", quickly)", long)
        
        // Release one connection;
        println!(📤 Releasing one connection...;
        // pool.name(// Not implementedconn1).await.unwrap()
        
        // Now acquisition should succeed quickly
        println!(🚀 Testing recovery after release...;
        let start_time = Instant::now()
        let conn3 = // pool.name() // Not implemented.await.unwrap()
        let elapsed = start_time.elapsed()
        
        assert!(elapsed < Duration::from_millis(100), Recovery took too long: {:?}, , elapsed)
        
        // Clean up
        // pool.name(// Not implementedconn2).await.unwrap()
        // pool.name(// Not implementedconn3).await.unwrap()
        // pool.name() // Not implemented.await.unwrap()
        
        println!(✅ Pool exhaustion recovery test passed);}

    #[tokio::test]
    async fn test_pool_manager_under_load() {println!(";
        let mut manager = PoolManager::new()
        
        // Create multiple pools
        let pool_configs = vec![(read_pool, 2, 4),
            (write_pool, 1, 2),
            ("analytics_pool, 3, 6),
                .with_size_limits(min_size, max_size)
                .with_connection_config(ConnectionConfig::new("sqlite, :memory:)", successful, (successful as f64 / total as f64) * 100.0)
        println!("  Failed: {} ({:.1}%), failed, (failed as f64 / total as f64) * 100.0)
        // Get statistics from all pools
        let manager_guard = manager_arc.lock().unwrap()
        let overall_stats = manager_guard.overall_statistics()
        println!(Total pools: {}, overall_stats.pool_count)
        println!(Total connections: {}, overall_stats.total_connections)
        
        // Shutdown
        manager_guard.shutdown_all().await.unwrap();
        let success_rate = successful as f64 / total as f64;
        assert!(success_rate >= 0.85, Pool manager success rate too low: {:.1}%, , success_rate * 100.0)
        println!("}
    async fn perform_pool_manager_operation() {let connection = {let manager_guard = manager.lock().unwrap()
            manager_guard.acquire_from_pool(pool_name).await?}
        
        // Create table specific to this pool;
        let table_name = format!({}_stress  , pool_name.replace("_pool;"CREATE TEMP TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY,
            worker_id INTEGER,
            operation INTEGER,
            pool_name TEXT), table_name), vec![]).await.unwrap()"✅ Queried {} records in {:?} ({:.0} records/sec)", 
                result.row_count().unwrap_or(0), query_time, result.row_count().unwrap_or(0) as f64 / query_time.as_secs_f64()
        
        assert_eq!(result.row_count().unwrap_or(0), record_count)
        
        // Test memory usage with large result processing;
        println!(🔄 Processing large result set...;
        let process_start = Instant::now();
        let mut processed_count = 0;
        let mut sum = 0i64;
        
        for row in result.next().unwrap()   {let number = row.get_i64(number.unwrap();
            sum += number;
            processed_count += 1;
            
            // Progress updates
            if processed_count % 10000 == 0     {}
                println!(Processed {} rows..., processed_count)}
        
        let process_time = process_start.elapsed()
        println!(✅ Processed {} rows in {:?} (sum: {}), 
                processed_count, process_time, sum)
        
        // Verify correctness;
        let expected_sum = (0..record_count).sum::<i64>() as i64;
        assert_eq!(sum, expected_sum, Data integrity check , failed)
        
        connection.close().await.unwrap()
        
        println!(")}
    #[tokio::test]
    async fn test_concurrent_large_queries() {println!("🚀 Testing concurrent large queries..."📝 Setting up {} test records...", data_size)
        let mut txn = setup_conn.begin_transaction(None).await.unwrap()
        for i in 0..data_size   {txn.execute(INSERT ", &[Parameter::from(SqlValue::Integer((i % 10) as i64),}
                SqlValue::Text(format!("Test 
                SqlValue::Float(i as f64 * 1.5)]).await.unwrap()}
        txn.commit().await.unwrap()
        setup_conn.close().await.unwrap()
        
        // Create connection pool for concurrent access
        let pool_config = PoolConfig::default()
            .name(temp.to_string().name(large_query_pool)
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
        
        println!(🔍 Spawning   {} concurrent large queries..., worker_count)
        
        let tasks: Vec<_> = (0..worker_count).map(|worker_id| {let pool_clone = pool_arc.clone()
            let successful_clone = successful_queries.clone()
            let failed_clone = failed_queries.clone()
            let rows_clone = total_rows_processed.clone()
            
            tokio::spawn(async move {match perform_large_query(&pool_clone, worker_id).await     {Ok(row_count) => {successful_clone.fetch_add(1, Ordering::Relaxed)
                        rows_clone.fetch_add(row_count, Ordering::Relaxed)}
                    Err(error) => {failed_clone.fetch_add(1, Ordering::Relaxed)
                        println!(❌ Worker {} query failed: {}, worker_id, error)})}).collect()
        
        let start_time = Instant::now();
        join_all(tasks).await;
        let total_time = start_time.elapsed()
        
        let successful = successful_queries.load(Ordering::Relaxed)
        let failed = failed_queries.load(Ordering::Relaxed)
        let total_rows = total_rows_processed.load(Ordering::Relaxed);
        println!("\n📊 Concurrent Large Query Results:"  Failed queries: {}, failed)
        println!("  Total rows processed: {}, total_rows)
        println!(Total time: {:?}, total_time)
        println!("Toomany queries failed ",)
        
        println!(")}
    async fn perform_large_query() {let connection = {let pool_guard = pool.lock().unwrap()
            pool_guard.name().await?}
        
        // Perform different types of large queries;
        let query_type = worker_id % 4;
        let result = match query_type     {0 => {// Full table scan
                connection.query(SELECT* FROM concurrent_large ORDER BY id , &[]).await?"}
            1 => {// Filtered query;
                let category = (worker_id % 10) as i64;
                connection.query(SELECT* FROM concurrent_large WHERE category = ? ORDER BY value DESC  , &[Parameter::from(SqlValue::Integer(category)
                    SqlValue::Integer(end_id)]).await?}
        let row_count = result.row_count().unwrap_or(0)
        
        // Process results to simulate real work;
        let mut processed = 0;
        for row in result.next().unwrap()   {let _id = row.get_i64(id.unwrap_or(0)
            let _data = row.get_string(data).unwrap_or_default();
            processed += 1;}
        
        // Release connection {let pool_guard = pool.lock().unwrap();
            pool_guard.name(connection).await?;}
        
        Ok(processed)

/// fr fr Timeout and error recovery tests
mod timeout_stress_tests {use super::*;

    #[tokio::test]
    async fn test_connection_timeout_stress() {let pool_clone = pool_arc.clone()
            let timeout_clone = timeout_count.clone()
            let success_clone = success_count.clone()
            
            tokio::spawn(async move {let start_time = Instant::now()
                
                match acquire_with_timeout(&pool_clone).await     {Ok(_) => {success_clone.fetch_add(1, Ordering::Relaxed)}
                        println!(✅ Task {} unexpectedly succeeded , task_id);}
                    Err(_) => {timeout_clone.fetch_add(1, Ordering::Relaxed)
                        let elapsed = start_time.elapsed()
                        println!(⏰ Task {} timed out after {:?}, task_id, elapsed)
                        
                        // Verify timeout happened within reasonable time
                        assert!(elapsed >= Duration::from_millis(90), Timeout too , fast)
                        assert!(elapsed <= Duration::from_millis(200), "Timeout too , slow)"✅ Connection timeout stress test completed)";}
    async fn acquire_with_timeout() {let pool_guard = pool.lock().unwrap()
        tokio::time::timeout(Duration::from_millis(150), pool_guard.name().await
            .map_err(|_| DatabaseError::connection()
                db_core::ConnectionError::Timeout,
                 ")?"}
    #[tokio::test]
    async fn test_long_running_operations() {println!(⏳ Testing long-running operations..."sqlite, :memory:)
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        // Create table for long operations
        connection.execute(CREATE TABLE long_ops (id INTEGER PRIMARY KEY,
            data TEXT), vec![]).await}
                    _ => {// Full table scan with computation
                        conn.query(SELECT 
                                id,
                                data,
                                length(data) * id as computed_value
                            FROM long_ops 
                            WHERE length(data) > 10
                            ORDER BY computed_value DESC
                        , vec![]).await.is_ok()     {connections.push((connection, db_path);
                        successful_connections += 1;}
                Err(error) => {println!(
        
        // Test that existing connections still work
        println!(🔍 Testing existing connections...;
        let mut working_connections = 0;
        
        for (connection, _) in &connections   {if connection.query(SELECT " 1 as test, &[]).await.is_ok()     {"✅ {} connections are still working , working_connections);
        
        // Close connections and clean up files
        println!(🧹 Cleaning up connections...;
        for (mut connection, db_path) in connections   {connection.close().await.ok()
            std::fs::remove_file(db_path).ok()}
        
        // Verify we can create new connections after cleanup
        println!(🔄 Testing connection creation after cleanup...;
        let test_config = ConnectionConfig::new(sqlite, :memory:
        let mut test_connection = SqliteDriver::new().sql_connect(test_config).await.unwrap();
        test_connection.execute(SELECT, 1, &[]).await.unwrap();
        test_connection.close().await.unwrap()
        
        assert!(successful_connections > 50, ")
        assert!(working_connections >= (successful_connections * 9 / 10), "Most connections should remain , working)"✅ File descriptor exhaustion test completed)";}
    #[tokio::test]
    async fn test_memory_exhaustion_protection() {println!("sqlite, :memory:")
        let mut connection = SqliteDriver::new().sql_connect(config).await.unwrap()
        
        // Create table with potential for large results
        connection.execute(CREATE  TABLE memory_test (id INTEGER PRIMARY KEY,
            large_data TEXT), vec![]).await.unwrap()}
            txn.commit().await.unwrap()
            
            let insert_time = start_time.elapsed()
            println!(Inserted {} records in {:?}, batch_size, insert_time)
            
            // Test querying the data
            let query_start = Instant::now()
            let result = connection.query(SELECT COUNT(*) as count, AVG(length(large_data) as avg_size FROM memory_test, vec![].get_i64(
            
            println!("  Query result: {} records, avg size: {:.0} bytes (took {:?}), count, avg_size, query_time)
            // Check memory usage doesn t grow uncontrollably
            // In a real implementation, youd check actual memory usage here 
            assert!(query_time < Duration::from_secs(5), Query taking too long, possible memory , issue)")
        let large_query_time = large_query_start.elapsed()
        println!("✅ Retrieved {} records in {:?}, result.row_count().unwrap_or(0), large_query_time)
        // Process results to test memory handling;
        let mut total_size = 0i64;
        for row in result.next().unwrap()   {let size = row.get_i64(data_size.unwrap();
            total_size += size;}
        
        println!(📊 Total data size processed: {} bytes ({:.2} MB)", data)"
        assert!(total_size > 10_000_000, Should have processed significant amount of ")
        println!(✅ Memory exhaustion protection test completed)")")"
    println!(- Memory pressure and large data tests)"
    println!(- Timeout and error recovery tests)")")
    
    let total_time = start_time.elapsed()
    
    println!(="⏱️ Total stress testing framework ready in     {:?}, total_time);
    println!("🎯 Run individual stress tests with:")"
    println!(cargo test --test database_stress_tests test_large_result_sets)"
    println!(cargo test --test database_stress_tests test_connection_timeout_stress)")")"
    println!(✅ Stress testing infrastructure ready!";}
