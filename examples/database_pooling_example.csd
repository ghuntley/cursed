fr fr/ fr fr CURSED Database Connection Pooling Example - enterprise vibes periodt
fr fr/ This example demonstrates connection pooling for high-performance database access

fr fr Import necessary packages
yeet sql_vibes
yeet concurrenz
yeet timez

squad DatabaseConfig {
    connection_string: String,
    max_connections: i32,
    min_connections: i32,
    timeout_seconds: i64
}

collab DatabaseService {
    slay new_pool(config: DatabaseConfig) -> ConnectionPool
    slay get_user_count(pool: ConnectionPool) -> i64
    slay insert_bulk_users(pool: ConnectionPool, users: [][]String) -> i64
}

slay create_database_config() -> DatabaseConfig {
    return DatabaseConfig {
        connection_string: "sqlite://users.db",
        max_connections: 10,
        min_connections: 2,
        timeout_seconds: 30
    }
}

slay setup_database_schema(pool: sql_vibes.ConnectionPool) -> Bool {
    vibez.spill("🔧 Setting up database schema...")
    
    sus conn_result = pool.get_connection()
    lowkey conn_result.is_error() {
        vibez.spill("Failed to get connection for schema setup:", conn_result.error())
        return cap
    }
    
    facts conn = conn_result.value()
    defer pool.return_connection(conn)
    
    // Create users table if it doesn't exist
    sus create_result = conn.execute(`
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            full_name TEXT NOT NULL,
            department TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_login DATETIME
        )
    `)
    
    lowkey create_result.is_error() {
        vibez.spill("Failed to create users table:", create_result.error())
        return cap
    }
    
    // Create indexes for better performance
    sus index_queries = [
        "CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)",
        "CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)",
        "CREATE INDEX IF NOT EXISTS idx_users_department ON users(department)",
        "CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at)"
    ]
    
    periodt query : index_queries {
        sus result = conn.execute(query)
        lowkey result.is_error() {
            vibez.spill("Failed to create index:", result.error())
            return cap
        }
    }
    
    vibez.spill("✅ Database schema setup complete")
    return based
}

slay generate_sample_users(count: i32) -> [][]String {
    vibez.spill("📝 Generating", count, "sample users...")
    
    sus departments = ["Engineering", "Marketing", "Sales", "HR", "Finance", "Operations"]
    sus first_names = ["Alex", "Jordan", "Casey", "Morgan", "Riley", "Avery", "Quinn", "Sage"]
    sus last_names = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"]
    
    sus users = [][]String{}
    
    periodt i : 0..count {
        sus first_name = first_names[i % len(first_names)]
        sus last_name = last_names[i % len(last_names)]
        sus username = stringz.lower(first_name + last_name + stringz.from_int(i))
        sus email = username + "@company.com"
        sus full_name = first_name + " " + last_name
        sus department = departments[i % len(departments)]
        
        users.append([username, email, full_name, department])
    }
    
    return users
}

slay insert_users_concurrently(pool: sql_vibes.ConnectionPool, users: [][]String, workers: i32) -> i64 {
    vibez.spill("🚀 Inserting", len(users), "users using", workers, "concurrent workers...")
    
    sus chunk_size = len(users) / workers
    lowkey chunk_size == 0 {
        chunk_size = 1
    }
    
    sus total_inserted = concurrenz.new_atomic_int64(0)
    sus wait_group = concurrenz.new_wait_group()
    
    periodt worker_id : 0..workers {
        wait_group.add(1)
        
        damn {
            sus start_idx = worker_id * chunk_size
            sus end_idx = mathz.min((worker_id + 1) * chunk_size, len(users))
            
            lowkey start_idx >= len(users) {
                wait_group.done()
                return
            }
            
            sus chunk = users[start_idx:end_idx]
            sus worker_inserted = insert_user_chunk(pool, chunk, worker_id)
            total_inserted.add(worker_inserted)
            
            wait_group.done()
        }()
    }
    
    wait_group.wait()
    return total_inserted.load()
}

slay insert_user_chunk(pool: sql_vibes.ConnectionPool, users: [][]String, worker_id: i32) -> i64 {
    sus conn_result = pool.get_connection()
    lowkey conn_result.is_error() {
        vibez.spill("Worker", worker_id, "failed to get connection:", conn_result.error())
        return 0
    }
    
    facts conn = conn_result.value()
    defer pool.return_connection(conn)
    
    // Prepare statement for batch insertion
    sus stmt_result = conn.prepare("INSERT INTO users (username, email, full_name, department) VALUES (?, ?, ?, ?)")
    lowkey stmt_result.is_error() {
        vibez.spill("Worker", worker_id, "failed to prepare statement:", stmt_result.error())
        return 0
    }
    
    facts stmt = stmt_result.value()
    defer stmt.close()
    
    sus inserted_count = 0
    
    // Use transaction for better performance
    sus txn_result = conn.begin_transaction()
    lowkey txn_result.is_error() {
        vibez.spill("Worker", worker_id, "failed to begin transaction:", txn_result.error())
        return 0
    }
    
    facts txn = txn_result.value()
    
    periodt user : users {
        sus result = stmt.execute_update(user[0], user[1], user[2], user[3])
        lowkey result.is_error() {
            vibez.spill("Worker", worker_id, "failed to insert user:", user[0], "-", result.error())
            txn.rollback()
            return inserted_count
        }
        inserted_count += 1
    }
    
    sus commit_result = txn.commit()
    lowkey commit_result.is_error() {
        vibez.spill("Worker", worker_id, "failed to commit transaction:", commit_result.error())
        return 0
    }
    
    vibez.spill("Worker", worker_id, "inserted", inserted_count, "users")
    return inserted_count
}

slay query_users_by_department(pool: sql_vibes.ConnectionPool) {
    vibez.spill("\n📊 Querying users by department...")
    
    sus conn_result = pool.get_connection()
    lowkey conn_result.is_error() {
        vibez.spill("Failed to get connection for querying:", conn_result.error())
        return
    }
    
    facts conn = conn_result.value()
    defer pool.return_connection(conn)
    
    sus query_result = conn.query(`
        SELECT department, COUNT(*) as user_count, 
               GROUP_CONCAT(username) as usernames
        FROM users 
        GROUP BY department 
        ORDER BY user_count DESC
    `)
    
    lowkey query_result.is_error() {
        vibez.spill("Failed to query departments:", query_result.error())
        return
    }
    
    facts result_set = query_result.value()
    
    vibez.spill("Department breakdown:")
    periodt row : result_set {
        sus department = row.get(0).as_string()
        sus count = row.get(1).as_integer()
        sus usernames = row.get(2).as_string()
        
        vibez.spill("  ", department, ":", count, "users")
        lowkey len(usernames) < 100 {
            vibez.spill("    Users:", usernames)
        }
    }
}

slay benchmark_concurrent_queries(pool: sql_vibes.ConnectionPool, concurrent_queries: i32) {
    vibez.spill("\n⚡ Benchmarking", concurrent_queries, "concurrent queries...")
    
    sus start_time = timez.now()
    sus wait_group = concurrenz.new_wait_group()
    sus query_count = concurrenz.new_atomic_int64(0)
    
    periodt i : 0..concurrent_queries {
        wait_group.add(1)
        
        damn {
            sus conn_result = pool.get_connection()
            lowkey conn_result.is_error() {
                vibez.spill("Query", i, "failed to get connection:", conn_result.error())
                wait_group.done()
                return
            }
            
            facts conn = conn_result.value()
            defer pool.return_connection(conn)
            
            // Execute random queries
            sus queries = [
                "SELECT COUNT(*) FROM users",
                "SELECT department, COUNT(*) FROM users GROUP BY department",
                "SELECT * FROM users ORDER BY created_at DESC LIMIT 10",
                "SELECT username, email FROM users WHERE department = 'Engineering'",
                "SELECT AVG(LENGTH(full_name)) FROM users"
            ]
            
            sus query = queries[i % len(queries)]
            sus result = conn.query(query)
            
            lowkey result.is_ok() {
                query_count.increment()
            }
            
            wait_group.done()
        }()
    }
    
    wait_group.wait()
    sus end_time = timez.now()
    sus duration_ms = end_time - start_time
    
    vibez.spill("✅ Completed", query_count.load(), "queries in", duration_ms, "ms")
    vibez.spill("   Average:", duration_ms / concurrent_queries, "ms per query")
}

slay main_character() {
    vibez.spill("🏢 CURSED Database Connection Pooling Example")
    vibez.spill("=========================================")
    
    // Create database configuration
    sus config = create_database_config()
    
    // Create connection pool
    vibez.spill("\n🔗 Creating connection pool...")
    sus pool_result = sql_vibes.create_pool(config.connection_string, config.max_connections)
    lowkey pool_result.is_error() {
        vibez.spill("Failed to create connection pool:", pool_result.error())
        vibe_life.exit(1)
    }
    
    facts pool = pool_result.value()
    defer pool.close()
    
    vibez.spill("✅ Connection pool created with", config.max_connections, "max connections")
    
    // Display initial pool stats
    sus stats = pool.stats()
    vibez.spill("Pool stats - Active:", stats.active_connections, "Idle:", stats.idle_connections)
    
    // Setup database schema
    lowkey !setup_database_schema(pool) {
        vibez.spill("Failed to setup database schema")
        vibe_life.exit(1)
    }
    
    // Clear existing data for clean demo
    sus conn_result = pool.get_connection()
    lowkey conn_result.is_error() {
        vibez.spill("Failed to get connection for cleanup:", conn_result.error())
        vibe_life.exit(1)
    }
    
    facts cleanup_conn = conn_result.value()
    cleanup_conn.execute("DELETE FROM users")
    pool.return_connection(cleanup_conn)
    
    // Generate sample users
    sus user_count = 1000
    sus users = generate_sample_users(user_count)
    
    // Insert users concurrently
    sus worker_count = 5
    sus start_time = timez.now()
    sus inserted = insert_users_concurrently(pool, users, worker_count)
    sus end_time = timez.now()
    
    vibez.spill("✅ Inserted", inserted, "users in", end_time - start_time, "ms")
    
    // Display updated pool stats
    sus final_stats = pool.stats()
    vibez.spill("Final pool stats - Active:", final_stats.active_connections, 
                "Idle:", final_stats.idle_connections,
                "Total requests:", final_stats.total_requests)
    
    // Query users by department
    query_users_by_department(pool)
    
    // Benchmark concurrent queries
    benchmark_concurrent_queries(pool, 20)
    
    // Final verification
    sus verify_conn = pool.get_connection().value()
    sus total_result = verify_conn.query("SELECT COUNT(*) FROM users")
    facts total_count = total_result.value().first_row().get(0).as_integer()
    pool.return_connection(verify_conn)
    
    vibez.spill("\n🎯 Final verification: ", total_count, "total users in database")
    vibez.spill("🎉 Connection pooling example completed successfully!")
    vibez.spill("   Pool handled all operations efficiently with concurrent access periodt")
}
