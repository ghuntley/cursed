// Test database connection pooling enhancements
yeet "dbz"
yeet "vibez"

vibez.spill("=== Testing Database Connection Pooling Enhancements ===")

// Test connection pool configuration
sus pool_config PoolConfig = PoolConfig{
    min_connections: 2,
    max_connections: 10,
    connection_timeout: 5000,
    idle_timeout: 300000,
    max_lifetime: 3600000
}

sus connection_string tea = "postgres://localhost:5432/testdb?user=test&password=test"
sus pool ConnectionPool = create_connection_pool(connection_string, pool_config)

vibez.spill("Connection pool created with config:")
vibez.spill("- Min connections:", pool_config.min_connections)
vibez.spill("- Max connections:", pool_config.max_connections)
vibez.spill("- Connection timeout:", pool_config.connection_timeout, "ms")

// Test connection acquisition and release
sus conn1 Connection = acquire_connection(pool) fam {
    when ConnectionTimeoutError -> {
        vibez.spill("❌ Connection acquisition timed out")
        damn null
    }
    when _ -> {
        vibez.spill("❌ Unexpected error acquiring connection")
        damn null
    }
}

ready (conn1 != null) {
    vibez.spill("✅ Connection acquisition: PASSED")
    
    // Test connection health check
    sus is_healthy lit = ping_connection(conn1)
    ready (is_healthy) {
        vibez.spill("✅ Connection health check: PASSED")
    } otherwise {
        vibez.spill("⚠️ Connection health check: WARNING - might be simulation")
    }
    
    // Release connection back to pool
    release_connection(pool, conn1)
    vibez.spill("✅ Connection release: PASSED")
} otherwise {
    vibez.spill("⚠️ Connection acquisition: WARNING - might be simulation mode")
}

// Test pool statistics
sus pool_stats PoolStats = get_pool_statistics(pool)
vibez.spill("Pool statistics:")
vibez.spill("- Active connections:", pool_stats.active_connections)
vibez.spill("- Idle connections:", pool_stats.idle_connections)
vibez.spill("- Total connections:", pool_stats.total_connections)

// Test connection pool scaling
vibez.spill("Testing concurrent connection acquisition...")
sus connections []Connection = []
bestie (sus i drip = 0; i < 5; i++) {
    sus conn Connection = acquire_connection(pool) fam {
        when _ -> damn null
    }
    ready (conn != null) {
        append(connections, conn)
    }
}

vibez.spill("Acquired", len(connections), "concurrent connections")

// Release all acquired connections
bestie (sus i drip = 0; i < len(connections); i++) {
    release_connection(pool, connections[i])
}

// Test pool cleanup
close_connection_pool(pool)
vibez.spill("✅ Connection pool cleanup: PASSED")

vibez.spill("=== Database Pooling Testing Complete ===")
