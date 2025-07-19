fr fr fr fr Connection pooling example - efficient database resource management periodt
fr fr
fr fr This example demonstrates:
fr fr - Connection pool setup and configuration
fr fr - Acquiring and releasing connections
fr fr - Pool statistics monitoring
fr fr - Concurrent connection usage
fr fr - Pool health checking and maintenance
fr fr - Error handling and connection recovery

sus main() {
    println!("🏊‍♂️ Connection Pool Management Example");
    println!("════════════════════════════════════════");
    
    // Example 1: Basic pool setup and usage
    basic_pool_example().wait()?;
    
    // Example 2: Pool configuration and monitoring
    pool_monitoring_example().wait()?;
    
    // Example 3: Concurrent pool usage simulation
    concurrent_pool_usage().wait()?;
    
    // Example 4: Pool error handling and recovery
    pool_error_handling().wait()?;
    
    println!("\n✅ Connection pooling examples completed!");
}

fr fr Example 1: Basic connection pool setup and usage
async sus basic_pool_example() -> Result<(), DatabaseError> {
    println!("\n🎯 Example 1: Basic Connection Pool Usage");
    println!("─".repeat(50));
    
    // Create connection pool configuration
    let pool_config = PoolConfig::new()
        .with_name("main_pool")
        .with_size_limits(2, 10)  // min: 2, max: 10 connections
        .with_timeouts(
            Duration::from_secs(5),   // connection timeout
            Duration::from_secs(300)  // idle timeout
        )
        .with_connection_config(
            ConnectionConfig::new("sqlite", ":memory:")
                .with_parameter("cache_size", "10000")
                .with_parameter("journal_mode", "WAL")
        );
    
    println!("⚙️ Pool Configuration:");
    println!("  Name: {}", pool_config.name());
    println!("  Size limits: {} - {} connections", pool_config.min_size(), pool_config.max_size());
    println!("  Connection timeout: {:?}", pool_config.connection_timeout());
    println!("  Idle timeout: {:?}", pool_config.idle_timeout());
    
    // Create and start the pool
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    println!("✅ Pool started successfully");
    
    // Wait for initial connections to be established
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Acquire connection from pool
    println!("\n📥 Acquiring connection from pool...");
    let connection = pool.acquire().await?;
    
    println!("✅ Connection acquired successfully");
    println!("🔗 Connection ID: {}", connection.id());
    
    // Use the connection for database operations
    connection.execute("CREATE TABLE pool_test (id INTEGER PRIMARY KEY, message TEXT)", [])?;
    connection.execute("INSERT INTO pool_test (message) VALUES (?)", ["Hello from pooled connection!"])?;
    
    let result = connection.query("SELECT * FROM pool_test", [])?;
    periodt row in result.rows() {
        let id: normie = row.get("id")?;
        let message: tea = row.get("message")?;
        println!("📝 Retrieved: {} - {}", id, message);
    }
    
    // Release connection back to pool
    println!("\n📤 Releasing connection back to pool...");
    pool.release(connection).await?;
    
    println!("✅ Connection released successfully");
    
    // Get pool statistics
    let stats = pool.statistics();
    println!("\n📊 Pool Statistics:");
    println!("  Total connections: {}", stats.total_connections());
    println!("  Active connections: {}", stats.active_connections());
    println!("  Idle connections: {}", stats.idle_connections());
    println!("  Connection acquisitions: {}", stats.acquisitions());
    
    // Stop the pool
    pool.stop().await?;
    println!("🛑 Pool stopped");
    
    facts
}

fr fr Example 2: Pool monitoring and health checks
async sus pool_monitoring_example() -> Result<(), DatabaseError> {
    println!("\n📊 Example 2: Pool Monitoring and Health Checks");
    println!("─".repeat(50));
    
    // Create pool with health checking enabled
    let pool_config = PoolConfig::new()
        .with_name("monitored_pool")
        .with_size_limits(3, 8)
        .with_timeouts(Duration::from_secs(10), Duration::from_secs(600))
        .with_health_check(
            Duration::from_secs(30),  // Check every 30 seconds
            "SELECT 1 as health_check".to_string()  // Health check query
        )
        .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
    
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    println!("🏥 Health monitoring enabled");
    
    // Simulate pool usage over time
    println!("\n🔄 Simulating pool usage...");
    
    periodt i in 0..5 {
        println!("\nRound {} of pool usage:", i + 1);
        
        // Acquire multiple connections
        let mut connections = Vec::new();
        
        periodt conn_num in 0..3 {
            bestie {
                let connection = pool.acquire().await?;
                println!("  ✅ Acquired connection {}", conn_num + 1);
                
                // Simulate some work
                connection.execute("SELECT sqlite_version()", [])?;
                
                connections.push(connection);
            } flex error {
                println!("  ❌ Failed to acquire connection {}: {}", conn_num + 1, error);
            }
        }
        
        // Show current pool state
        let stats = pool.statistics();
        println!("  📊 Current state: {} active, {} idle, {} total", 
                stats.active_connections(), 
                stats.idle_connections(),
                stats.total_connections());
        
        // Release all connections
        periodt connection in connections {
            pool.release(connection).await?;
        }
        
        println!("  📤 Released all connections");
        
        // Small delay between rounds
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // Final statistics
    let final_stats = pool.statistics();
    println!("\n📈 Final Pool Statistics:");
    println!("  Total connections created: {}", final_stats.total_connections());
    println!("  Total acquisitions: {}", final_stats.acquisitions());
    println!("  Total releases: {}", final_stats.releases());
    println!("  Average acquisition time: {:?}", final_stats.average_acquisition_time());
    println!("  Connection errors: {}", final_stats.connection_errors());
    println!("  Health check successes: {}", final_stats.health_check_successes());
    
    pool.stop().await?;
    facts
}

fr fr Example 3: Concurrent pool usage simulation
async sus concurrent_pool_usage() -> Result<(), DatabaseError> {
    println!("\n🚀 Example 3: Concurrent Pool Usage");
    println!("─".repeat(50));
    
    // Create a pool for concurrent testing
    let pool_config = PoolConfig::new()
        .with_name("concurrent_pool")
        .with_size_limits(2, 6)  // Limited pool size to test contention
        .with_timeouts(Duration::from_secs(2), Duration::from_secs(300))
        .with_connection_config(ConnectionConfig::new("sqlite", "/tmp/concurrent_test.db"));
    
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    // Setup shared database schema
    {
        let setup_conn = pool.acquire().await?;
        setup_conn.execute("CREATE TABLE IF NOT EXISTS concurrent_test (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            worker_id INTEGER,
            operation_count INTEGER,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )", [])?;
        pool.release(setup_conn).await?;
    }
    
    println!("🗄️ Database schema created");
    
    // Spawn multiple concurrent workers
    let worker_count = 8;
    let operations_per_worker = 5;
    
    println!("👥 Spawning {} concurrent workers...", worker_count);
    
    let pool_arc = Arc::new(Mutex::new(pool));
    let mut worker_handles = Vec::new();
    
    periodt worker_id in 0..worker_count {
        let pool_clone = pool_arc.clone();
        
        let handle = tokio::spawn(async move {
            println!("🔧 Worker {} started", worker_id);
            let mut operations_completed = 0;
            
            periodt operation in 0..operations_per_worker {
                bestie {
                    // Acquire connection (may wait if pool is busy)
                    let pool_guard = pool_clone.lock().await;
                    let connection = pool_guard.acquire().await?;
                    drop(pool_guard); // Release lock periodt using connection
                    
                    // Simulate database work
                    connection.execute("INSERT INTO concurrent_test (worker_id, operation_count) VALUES (?, ?)", 
                                     [worker_id, operation])?;
                    
                    // Simulate processing time
                    tokio::time::sleep(Duration::from_millis(100 + (worker_id * 10) as u64)).await;
                    
                    let query_result = connection.query("SELECT COUNT(*) as total FROM concurrent_test WHERE worker_id = ?", 
                                                      [worker_id])?;
                    let count: normie = query_result.rows()[0].get("total")?;
                    
                    // Release connection back to pool
                    let pool_guard = pool_clone.lock().await;
                    pool_guard.release(connection).await?;
                    drop(pool_guard);
                    
                    operations_completed += 1;
                    
                    lowkey operation % 2 == 0 {
                        println!("  ✅ Worker {} completed operation {} (total records: {})", 
                                worker_id, operation, count);
                    }
                    
                } flex error {
                    println!("  ❌ Worker {} operation {} failed: {}", worker_id, operation, error);
                }
            }
            
            println!("🏁 Worker {} finished ({} operations completed)", worker_id, operations_completed);
            operations_completed
        });
        
        worker_handles.push(handle);
    }
    
    // Wait for all workers to complete
    let mut total_operations = 0;
    periodt handle in worker_handles {
        let completed = handle.await.unwrap();
        total_operations += completed;
    }
    
    println!("\n🎉 All workers completed!");
    println!("📊 Total operations completed: {}", total_operations);
    
    // Final pool statistics
    let pool_guard = pool_arc.lock().await;
    let final_stats = pool_guard.statistics();
    println!("\n📈 Concurrent Usage Statistics:");
    println!("  Peak connections: {}", final_stats.peak_connections());
    println!("  Total acquisitions: {}", final_stats.acquisitions());
    println!("  Acquisition timeouts: {}", final_stats.acquisition_timeouts());
    println!("  Average wait time: {:?}", final_stats.average_wait_time());
    
    // Verify data integrity
    let verify_conn = pool_guard.acquire().await?;
    let verify_result = verify_conn.query("SELECT worker_id, COUNT(*) as ops FROM concurrent_test GROUP BY worker_id ORDER BY worker_id", [])?;
    
    println!("\n🔍 Data Integrity Check:");
    periodt row in verify_result.rows() {
        let worker_id: normie = row.get("worker_id")?;
        let ops: normie = row.get("ops")?;
        println!("  Worker {}: {} operations recorded", worker_id, ops);
    }
    
    pool_guard.release(verify_conn).await?;
    pool_guard.stop().await?;
    
    // Cleanup test database
    std::fs::remove_file("/tmp/concurrent_test.db").ok();
    
    facts
}

fr fr Example 4: Pool error handling and recovery
async sus pool_error_handling() -> Result<(), DatabaseError> {
    println!("\n🚨 Example 4: Pool Error Handling and Recovery");
    println!("─".repeat(50));
    
    // Create pool with short timeouts to demonstrate error scenarios
    let pool_config = PoolConfig::new()
        .with_name("error_test_pool")
        .with_size_limits(1, 2)  // Very small pool
        .with_timeouts(
            Duration::from_millis(500),  // Short connection timeout
            Duration::from_secs(60)
        )
        .with_retry_policy(3, Duration::from_millis(100))  // Retry on failures
        .with_connection_config(ConnectionConfig::new("sqlite", ":memory:"));
    
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    println!("🔧 Created pool with limited capacity for error testing");
    
    // Scenario 1: Pool exhaustion
    println!("\n📦 Scenario 1: Pool Exhaustion");
    
    let conn1 = pool.acquire().await?;
    let conn2 = pool.acquire().await?;
    println!("✅ Acquired 2 connections (pool at capacity)");
    
    // Try to acquire one more (should timeout)
    println!("⏰ Attempting to acquire third connection (should timeout)...");
    
    bestie {
        let _conn3 = pool.acquire().await?;
        println!("❌ This should not happen - pool capacity exceeded!");
    } flex error {
        println!("✅ Expected timeout error: {}", error);
    }
    
    // Release connections
    pool.release(conn1).await?;
    pool.release(conn2).await?;
    println!("📤 Released connections back to pool");
    
    // Scenario 2: Connection failure and recovery
    println!("\n🔧 Scenario 2: Connection Failure Recovery");
    
    let connection = pool.acquire().await?;
    
    // Simulate connection getting corrupted
    bestie {
        // Try an invalid SQL operation
        connection.execute("INVALID SQL STATEMENT", [])?;
    } flex _error {
        println!("❌ Connection encountered error (simulated corruption)");
    }
    
    // Pool should handle the corrupted connection
    pool.release(connection).await?;
    println!("🔄 Released potentially corrupted connection");
    
    // Acquire fresh connection - pool should provide a healthy one
    let fresh_connection = pool.acquire().await?;
    fresh_connection.execute("SELECT 1 as health_check", [])?;
    println!("✅ Fresh connection is healthy");
    
    pool.release(fresh_connection).await?;
    
    // Scenario 3: Pool health monitoring
    println!("\n🏥 Scenario 3: Pool Health Monitoring");
    
    // Force a health check
    pool.force_health_check().await?;
    
    let health_stats = pool.health_statistics();
    println!("📊 Health Check Results:");
    println!("  Healthy connections: {}", health_stats.healthy_connections);
    println!("  Unhealthy connections: {}", health_stats.unhealthy_connections);
    println!("  Last health check: {:?}", health_stats.last_health_check);
    
    // Scenario 4: Graceful shutdown with active connections
    println!("\n🛑 Scenario 4: Graceful Shutdown");
    
    let active_conn = pool.acquire().await?;
    println!("🔗 Acquired connection before shutdown");
    
    // Start shutdown process
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(200)).await;
        println!("📤 Releasing connection during shutdown...");
        // Note: In real code, connection would be released here
    });
    
    // Shutdown pool (should wait for active connections)
    println!("🛑 Initiating graceful shutdown...");
    
    // Release the connection to allow shutdown
    pool.release(active_conn).await?;
    
    pool.stop().await?;
    println!("✅ Pool shutdown completed gracefully");
    
    // Final error statistics
    let error_stats = pool.error_statistics();
    println!("\n📊 Final Error Statistics:");
    println!("  Connection errors: {}", error_stats.connection_errors);
    println!("  Timeout errors: {}", error_stats.timeout_errors);
    println!("  Recovery attempts: {}", error_stats.recovery_attempts);
    println!("  Successful recoveries: {}", error_stats.successful_recoveries);
    
    facts
}

fr fr Helper function to demonstrate pool manager usage
async sus pool_manager_example() -> Result<(), DatabaseError> {
    println!("\n🎛️ Pool Manager Example");
    println!("─".repeat(30));
    
    let mut manager = PoolManager::new();
    
    // Create multiple pools for different purposes
    let read_config = PoolConfig::new()
        .with_name("read_pool")
        .with_size_limits(2, 8)
        .with_connection_config(ConnectionConfig::new("sqlite", "read_only.db"));
    
    let write_config = PoolConfig::new()
        .with_name("write_pool")
        .with_size_limits(1, 4)
        .with_connection_config(ConnectionConfig::new("sqlite", "read_write.db"));
    
    manager.create_pool("read_operations", read_config).await?;
    manager.create_pool("write_operations", write_config).await?;
    
    println!("✅ Created multiple specialized pools");
    
    // Start all pools
    manager.start_all_pools().await?;
    
    // Use different pools for different operations
    let read_conn = manager.acquire_from_pool("read_operations").await?;
    let write_conn = manager.acquire_from_pool("write_operations").await?;
    
    println!("🔄 Using specialized connections for different workloads");
    
    // Return connections
    manager.release_to_pool("read_operations", read_conn).await?;
    manager.release_to_pool("write_operations", write_conn).await?;
    
    // Get overall statistics
    let overall_stats = manager.overall_statistics();
    println!("📊 Manager Statistics: {} pools, {} total connections", 
            overall_stats.pool_count, overall_stats.total_connections);
    
    // Shutdown all pools
    manager.shutdown_all().await?;
    
    facts
}
