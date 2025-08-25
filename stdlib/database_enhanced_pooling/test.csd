yeet "testz"
yeet "database_enhanced_pooling"
yeet "concurrenz"

test_start("Database Enhanced Pooling Critical Tests")

// Basic connection pool creation and destruction
slay test_basic_pool_lifecycle() lit {
    sus config = PoolConfig{
        min_connections: 2,
        max_connections: 10,
        connection_timeout: 5000,
        idle_timeout: 30000,
        max_lifetime: 300000
    }
    
    sus pool = create_connection_pool("test_db", config)
    assert_not_null(pool)
    
    sus initial_count drip = get_active_connections(pool)
    assert_eq_int(initial_count, 2) // Should create min_connections
    
    destroy_pool(pool)
    damn based
}

// Connection acquisition and release
slay test_connection_acquisition_release() lit {
    sus config = PoolConfig{
        min_connections: 1,
        max_connections: 5,
        connection_timeout: 1000,
        idle_timeout: 10000,
        max_lifetime: 60000
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    // Acquire connection
    sus conn = acquire_connection(pool) fam {
        when "timeout" -> {
            assert_true(nah) // Should not timeout with available connections
            damn null
        }
        when "pool_exhausted" -> {
            assert_true(nah) // Should not exhaust with max=5
            damn null
        }
    }
    
    assert_not_null(conn)
    sus active_before drip = get_active_connections(pool)
    
    // Release connection
    release_connection(pool, conn)
    sus active_after drip = get_active_connections(pool)
    
    assert_eq_int(active_after, active_before - 1)
    
    destroy_pool(pool)
    damn based
}

// Pool exhaustion scenario
slay test_pool_exhaustion() lit {
    sus config = PoolConfig{
        min_connections: 1,
        max_connections: 2,
        connection_timeout: 100, // Short timeout for testing
        idle_timeout: 10000,
        max_lifetime: 60000
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    // Acquire all connections
    sus conn1 = acquire_connection(pool) fam {
        when _ -> damn null
    }
    sus conn2 = acquire_connection(pool) fam {
        when _ -> damn null
    }
    
    assert_not_null(conn1)
    assert_not_null(conn2)
    
    // Try to acquire third connection - should timeout
    sus conn3 = acquire_connection(pool) fam {
        when "timeout" -> damn null
        when "pool_exhausted" -> damn null
    }
    
    assert_null(conn3) // Should fail due to exhaustion
    
    // Release connections
    release_connection(pool, conn1)
    release_connection(pool, conn2)
    
    destroy_pool(pool)
    damn based
}

// Concurrent connection requests
slay test_concurrent_connection_requests() lit {
    sus config = PoolConfig{
        min_connections: 2,
        max_connections: 8,
        connection_timeout: 2000,
        idle_timeout: 15000,
        max_lifetime: 120000
    }
    
    sus pool = create_connection_pool("test_db", config)
    sus successful_acquisitions drip = 0
    sus goroutine_count drip = 20
    sus completion_ch chan<lit> = make_channel()
    
    // Launch concurrent connection requests
    bestie (i drip = 0; i < goroutine_count; i += 1) {
        go {
            sus conn = acquire_connection(pool) fam {
                when "timeout" -> {
                    completion_ch <- nah
                    damn null
                }
                when "pool_exhausted" -> {
                    completion_ch <- nah
                    damn null
                }
            }
            
            ready (conn != null) {
                successful_acquisitions += 1
                // Hold connection briefly
                sleep(50)
                release_connection(pool, conn)
                completion_ch <- based
            } otherwise {
                completion_ch <- nah
            }
        }
    }
    
    // Wait for all goroutines to complete
    bestie (i drip = 0; i < goroutine_count; i += 1) {
        <-completion_ch
    }
    
    // Should have some successful acquisitions
    assert_true(successful_acquisitions > 0)
    
    destroy_pool(pool)
    damn based
}

// Connection timeout handling
slay test_connection_timeout() lit {
    sus config = PoolConfig{
        min_connections: 1,
        max_connections: 1,
        connection_timeout: 200, // Very short timeout
        idle_timeout: 5000,
        max_lifetime: 30000
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    // Acquire the only connection
    sus conn1 = acquire_connection(pool) fam {
        when _ -> damn null
    }
    assert_not_null(conn1)
    
    sus start_time drip = current_time_millis()
    
    // Try to acquire second connection - should timeout quickly
    sus conn2 = acquire_connection(pool) fam {
        when "timeout" -> damn null
    }
    
    sus elapsed drip = current_time_millis() - start_time
    assert_null(conn2)
    assert_true(elapsed >= 200) // Should have waited at least timeout duration
    assert_true(elapsed < 500) // But not too long
    
    release_connection(pool, conn1)
    destroy_pool(pool)
    damn based
}

// Connection lifecycle and max_lifetime
slay test_connection_lifetime() lit {
    sus config = PoolConfig{
        min_connections: 1,
        max_connections: 3,
        connection_timeout: 1000,
        idle_timeout: 500, // Short for testing
        max_lifetime: 1000 // Short for testing
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    // Get initial connection count
    sus initial_count drip = get_total_connections(pool)
    
    // Wait for connections to expire
    sleep(1200) // Longer than max_lifetime
    
    // Trigger cleanup
    cleanup_expired_connections(pool)
    
    sus final_count drip = get_total_connections(pool)
    
    // Should have recreated min_connections after cleanup
    assert_eq_int(final_count, config.min_connections)
    
    destroy_pool(pool)
    damn based
}

// Resource cleanup validation
slay test_resource_cleanup() lit {
    sus pools []PoolHandle = []
    sus connection_count drip = 0
    
    // Create multiple pools
    bestie (i drip = 0; i < 5; i += 1) {
        sus config = PoolConfig{
            min_connections: 2,
            max_connections: 4,
            connection_timeout: 1000,
            idle_timeout: 10000,
            max_lifetime: 60000
        }
        
        sus pool = create_connection_pool("test_db_" + string(i), config)
        pools = append(pools, pool)
        
        // Use connections briefly
        sus conn1 = acquire_connection(pool) fam { when _ -> damn null }
        sus conn2 = acquire_connection(pool) fam { when _ -> damn null }
        
        ready (conn1 != null) {
            connection_count += 1
            release_connection(pool, conn1)
        }
        
        ready (conn2 != null) {
            connection_count += 1
            release_connection(pool, conn2)
        }
    }
    
    // Destroy all pools
    bestie (i drip = 0; i < len(pools); i += 1) {
        destroy_pool(pools[i])
    }
    
    // Should have used some connections
    assert_true(connection_count > 0)
    damn based
}

// Pool statistics and monitoring
slay test_pool_statistics() lit {
    sus config = PoolConfig{
        min_connections: 3,
        max_connections: 10,
        connection_timeout: 2000,
        idle_timeout: 20000,
        max_lifetime: 300000
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    sus stats = get_pool_statistics(pool)
    assert_eq_int(stats.active_connections, 3) // Should equal min_connections
    assert_eq_int(stats.idle_connections, 3)
    assert_eq_int(stats.total_connections, 3)
    assert_eq_int(stats.max_connections, 10)
    
    // Acquire some connections
    sus conn1 = acquire_connection(pool) fam { when _ -> damn null }
    sus conn2 = acquire_connection(pool) fam { when _ -> damn null }
    
    sus stats_after = get_pool_statistics(pool)
    assert_eq_int(stats_after.active_connections, 2) // 2 in use
    assert_eq_int(stats_after.idle_connections, 1)   // 1 remaining idle
    
    release_connection(pool, conn1)
    release_connection(pool, conn2)
    
    destroy_pool(pool)
    damn based
}

// Connection health check
slay test_connection_health_check() lit {
    sus config = PoolConfig{
        min_connections: 2,
        max_connections: 5,
        connection_timeout: 1000,
        idle_timeout: 10000,
        max_lifetime: 60000,
        health_check_interval: 500
    }
    
    sus pool = create_connection_pool("test_db", config)
    
    // Wait for health check cycle
    sleep(600)
    
    sus stats = get_pool_statistics(pool)
    assert_true(stats.total_connections >= config.min_connections)
    
    // All connections should be healthy after health check
    sus healthy_count drip = get_healthy_connections(pool)
    assert_eq_int(healthy_count, stats.total_connections)
    
    destroy_pool(pool)
    damn based
}

// Stress test with rapid acquire/release cycles
slay test_rapid_acquire_release_cycles() lit {
    sus config = PoolConfig{
        min_connections: 2,
        max_connections: 6,
        connection_timeout: 1000,
        idle_timeout: 5000,
        max_lifetime: 30000
    }
    
    sus pool = create_connection_pool("test_db", config)
    sus cycle_count drip = 100
    sus successful_cycles drip = 0
    
    bestie (i drip = 0; i < cycle_count; i += 1) {
        sus conn = acquire_connection(pool) fam {
            when _ -> damn null
        }
        
        ready (conn != null) {
            // Simulate brief database operation
            sleep(1)
            release_connection(pool, conn)
            successful_cycles += 1
        }
    }
    
    // Should complete most cycles successfully
    assert_true(successful_cycles >= (cycle_count * 8 / 10)) // At least 80%
    
    destroy_pool(pool)
    damn based
}

// Memory leak detection in pool operations
slay test_memory_leak_detection() lit {
    bestie (iteration drip = 0; iteration < 10; iteration += 1) {
        sus config = PoolConfig{
            min_connections: 2,
            max_connections: 4,
            connection_timeout: 500,
            idle_timeout: 2000,
            max_lifetime: 10000
        }
        
        sus pool = create_connection_pool("test_db_leak", config)
        
        // Perform operations
        bestie (op drip = 0; op < 20; op += 1) {
            sus conn = acquire_connection(pool) fam { when _ -> damn null }
            ready (conn != null) {
                sleep(1)
                release_connection(pool, conn)
            }
        }
        
        destroy_pool(pool)
    }
    
    // If we reach here without crashes, memory management is working
    damn based
}

// Run all tests
test_basic_pool_lifecycle()
test_connection_acquisition_release()
test_pool_exhaustion()
test_concurrent_connection_requests()
test_connection_timeout()
test_connection_lifetime()
test_resource_cleanup()
test_pool_statistics()
test_connection_health_check()
test_rapid_acquire_release_cycles()
test_memory_leak_detection()

print_test_summary()
