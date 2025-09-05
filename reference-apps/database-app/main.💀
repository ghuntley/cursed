# CURSED Database Application - SQLite CRUD Operations with Connection Pooling
# Demonstrates: Database operations, SQL queries, connection management, error handling

yeet "vibez"
yeet "dbz"
yeet "stringz" 
yeet "jsonz"
yeet "timez"
yeet "arrayz"
yeet "concurrenz"

# User model
squad User {
    id drip
    name tea
    email tea
    created_at drip
    updated_at drip
    active lit
}

# Database connection pool
squad ConnectionPool {
    connections []dbz.Connection
    available_connections chan<dbz.Connection>
    max_connections drip
    current_connections drip
    connection_string tea
}

# Database operations interface
collab UserRepository {
    slay create_user(user User) yikes<User>
    slay get_user_by_id(id drip) yikes<User>
    slay get_all_users() yikes<[]User>
    slay update_user(user User) yikes<User>
    slay delete_user(id drip) yikes<lit>
    slay find_users_by_email(email tea) yikes<[]User>
}

# SQLite implementation of UserRepository
squad SQLiteUserRepository {
    pool *ConnectionPool
}

# Create connection pool
slay new_connection_pool(connection_string tea, max_connections drip) yikes<ConnectionPool> {
    sus pool ConnectionPool = {
        connections: [],
        available_connections: make_channel<dbz.Connection>(),
        max_connections: max_connections,
        current_connections: 0,
        connection_string: connection_string
    }
    
    # Create initial connections
    sus i drip = 0
    bestie (i < max_connections) {
        sus conn dbz.Connection = dbz.open(connection_string) fam {
            when _ -> yikes "failed to create database connection"
        }
        
        pool.connections = arrayz.append(pool.connections, conn)
        pool.available_connections <- conn
        pool.current_connections = pool.current_connections + 1
        
        i = i + 1
    }
    
    damn pool
}

# Get connection from pool
slay get_connection(pool *ConnectionPool) yikes<dbz.Connection> {
    sus conn dbz.Connection = <-pool.available_connections fam {
        when "channel_timeout" -> yikes "connection pool timeout"
        when _ -> yikes "failed to get connection"
    }
    
    damn conn
}

# Return connection to pool
slay return_connection(pool *ConnectionPool, conn dbz.Connection) {
    pool.available_connections <- conn
}

# Initialize database schema
slay initialize_schema(pool *ConnectionPool) yikes<lit> {
    sus conn dbz.Connection = get_connection(pool) fam {
        when _ -> yikes "failed to get connection for schema initialization"
    }
    defer return_connection(pool, conn)
    
    sus create_table_sql tea = `
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            active BOOLEAN DEFAULT 1
        );
        
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
        CREATE INDEX IF NOT EXISTS idx_users_active ON users(active);
    `
    
    dbz.execute(conn, create_table_sql) fam {
        when _ -> yikes "failed to create database schema"
    }
    
    damn based
}

# Create new user repository
slay new_user_repository(pool *ConnectionPool) SQLiteUserRepository {
    damn SQLiteUserRepository{pool: pool}
}

# Implement UserRepository interface
impl UserRepository for SQLiteUserRepository {
    slay create_user(user User) yikes<User> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus now drip = timez.now()
        sus insert_sql tea = `
            INSERT INTO users (name, email, created_at, updated_at, active)
            VALUES (?, ?, ?, ?, ?)
        `
        
        sus result dbz.Result = dbz.execute_params(conn, insert_sql, [
            user.name,
            user.email, 
            now,
            now,
            user.active
        ]) fam {
            when "unique_constraint_violation" -> yikes "user with this email already exists"
            when _ -> yikes "failed to create user"
        }
        
        sus new_user User = {
            id: dbz.last_insert_id(result),
            name: user.name,
            email: user.email,
            created_at: now,
            updated_at: now,
            active: user.active
        }
        
        damn new_user
    }
    
    slay get_user_by_id(id drip) yikes<User> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus select_sql tea = `
            SELECT id, name, email, created_at, updated_at, active
            FROM users WHERE id = ?
        `
        
        sus rows []dbz.Row = dbz.query_params(conn, select_sql, [id]) fam {
            when _ -> yikes "failed to query user"
        }
        
        ready (len(rows) == 0) {
            yikes "user not found"
        }
        
        sus row dbz.Row = rows[0]
        sus user User = {
            id: dbz.get_int(row, "id"),
            name: dbz.get_string(row, "name"),
            email: dbz.get_string(row, "email"),
            created_at: dbz.get_int(row, "created_at"),
            updated_at: dbz.get_int(row, "updated_at"),
            active: dbz.get_bool(row, "active")
        }
        
        damn user
    }
    
    slay get_all_users() yikes<[]User> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus select_sql tea = `
            SELECT id, name, email, created_at, updated_at, active
            FROM users ORDER BY created_at DESC
        `
        
        sus rows []dbz.Row = dbz.query(conn, select_sql) fam {
            when _ -> yikes "failed to query users"
        }
        
        sus users []User = []
        bestie (row in rows) {
            sus user User = {
                id: dbz.get_int(row, "id"),
                name: dbz.get_string(row, "name"),
                email: dbz.get_string(row, "email"),
                created_at: dbz.get_int(row, "created_at"),
                updated_at: dbz.get_int(row, "updated_at"),
                active: dbz.get_bool(row, "active")
            }
            users = arrayz.append(users, user)
        }
        
        damn users
    }
    
    slay update_user(user User) yikes<User> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus now drip = timez.now()
        sus update_sql tea = `
            UPDATE users 
            SET name = ?, email = ?, updated_at = ?, active = ?
            WHERE id = ?
        `
        
        dbz.execute_params(conn, update_sql, [
            user.name,
            user.email,
            now,
            user.active,
            user.id
        ]) fam {
            when "unique_constraint_violation" -> yikes "email already in use"
            when _ -> yikes "failed to update user"
        }
        
        # Return updated user
        sus updated_user User = {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
            updated_at: now,
            active: user.active
        }
        
        damn updated_user
    }
    
    slay delete_user(id drip) yikes<lit> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus delete_sql tea = "DELETE FROM users WHERE id = ?"
        
        sus result dbz.Result = dbz.execute_params(conn, delete_sql, [id]) fam {
            when _ -> yikes "failed to delete user"
        }
        
        ready (dbz.rows_affected(result) == 0) {
            yikes "user not found"
        }
        
        damn based
    }
    
    slay find_users_by_email(email tea) yikes<[]User> {
        sus conn dbz.Connection = get_connection(self.pool) fam {
            when _ -> yikes "failed to get database connection"
        }
        defer return_connection(self.pool, conn)
        
        sus search_email tea = "%" + email + "%"
        sus select_sql tea = `
            SELECT id, name, email, created_at, updated_at, active
            FROM users WHERE email LIKE ? ORDER BY name
        `
        
        sus rows []dbz.Row = dbz.query_params(conn, select_sql, [search_email]) fam {
            when _ -> yikes "failed to search users"
        }
        
        sus users []User = []
        bestie (row in rows) {
            sus user User = {
                id: dbz.get_int(row, "id"),
                name: dbz.get_string(row, "name"),
                email: dbz.get_string(row, "email"),
                created_at: dbz.get_int(row, "created_at"),
                updated_at: dbz.get_int(row, "updated_at"),
                active: dbz.get_bool(row, "active")
            }
            users = arrayz.append(users, user)
        }
        
        damn users
    }
}

# Utility functions
slay format_user_json(user User) tea {
    sus user_map map<tea, lit> = {
        "id": user.id,
        "name": user.name,
        "email": user.email,
        "created_at": timez.format_time(user.created_at, "2006-01-02T15:04:05Z"),
        "updated_at": timez.format_time(user.updated_at, "2006-01-02T15:04:05Z"),
        "active": user.active
    }
    
    damn jsonz.marshal(user_map) fam {
        when _ -> damn "{}"
    }
}

slay print_user_table(users []User) {
    vibez.spill("┌────────┬─────────────────────┬─────────────────────────┬────────────┬────────┐")
    vibez.spill("│   ID   │        Name         │         Email           │   Created  │ Active │")
    vibez.spill("├────────┼─────────────────────┼─────────────────────────┼────────────┼────────┤")
    
    bestie (user in users) {
        sus created_str tea = timez.format_time(user.created_at, "2006-01-02")
        sus active_str tea = ready (user.active) { damn "Yes" } otherwise { damn "No" }
        
        vibez.spill(stringz.format("│ %-6d │ %-19s │ %-23s │ %-10s │ %-6s │", 
            user.id, user.name, user.email, created_str, active_str))
    }
    
    vibez.spill("└────────┴─────────────────────┴─────────────────────────┴────────────┴────────┘")
}

# Demo functions
slay demo_create_users(repo UserRepository) {
    vibez.spill("🔨 Creating sample users...")
    
    sus sample_users []User = [
        {name: "Alice Johnson", email: "alice@example.com", active: based},
        {name: "Bob Smith", email: "bob@example.com", active: based},
        {name: "Charlie Brown", email: "charlie@example.com", active: false},
        {name: "Diana Prince", email: "diana@example.com", active: based},
        {name: "Eve Adams", email: "eve@example.com", active: based}
    ]
    
    bestie (user in sample_users) {
        sus created_user User = repo.create_user(user) fam {
            when "user with this email already exists" -> {
                vibez.spill("  ⚠️  User", user.email, "already exists")
                skip
            }
            when _ -> {
                vibez.spill("  ❌ Failed to create user:", user.email)
                skip
            }
        }
        
        vibez.spill("  ✅ Created user:", created_user.name, "(ID:", created_user.id, ")")
    }
    
    vibez.spill("")
}

slay demo_read_operations(repo UserRepository) {
    vibez.spill("📖 Reading user data...")
    
    # Get all users
    sus all_users []User = repo.get_all_users() fam {
        when _ -> {
            vibez.spill("  ❌ Failed to get users")
            damn
        }
    }
    
    vibez.spill("  📋 All users (", len(all_users), "total):")
    print_user_table(all_users)
    vibez.spill("")
    
    # Get specific user
    ready (len(all_users) > 0) {
        sus first_user User = all_users[0]
        sus retrieved_user User = repo.get_user_by_id(first_user.id) fam {
            when "user not found" -> {
                vibez.spill("  ❌ User not found")
                damn
            }
            when _ -> {
                vibez.spill("  ❌ Failed to get user")
                damn
            }
        }
        
        vibez.spill("  👤 Retrieved user by ID:", retrieved_user.id)
        vibez.spill("     JSON:", format_user_json(retrieved_user))
        vibez.spill("")
    }
    
    # Search users by email
    sus matching_users []User = repo.find_users_by_email("example.com") fam {
        when _ -> {
            vibez.spill("  ❌ Failed to search users")
            damn
        }
    }
    
    vibez.spill("  🔍 Users matching 'example.com' (", len(matching_users), "found):")
    bestie (user in matching_users) {
        vibez.spill("     -", user.name, "(", user.email, ")")
    }
    vibez.spill("")
}

slay demo_update_operations(repo UserRepository) {
    vibez.spill("✏️  Updating user data...")
    
    sus all_users []User = repo.get_all_users() fam {
        when _ -> damn
    }
    
    ready (len(all_users) > 0) {
        sus user_to_update User = all_users[0]
        user_to_update.name = user_to_update.name + " (Updated)"
        user_to_update.active = !user_to_update.active
        
        sus updated_user User = repo.update_user(user_to_update) fam {
            when "email already in use" -> {
                vibez.spill("  ⚠️  Email conflict during update")
                damn
            }
            when _ -> {
                vibez.spill("  ❌ Failed to update user")
                damn
            }
        }
        
        vibez.spill("  ✅ Updated user:", updated_user.name)
        vibez.spill("     New status: active =", updated_user.active)
        vibez.spill("")
    }
}

slay demo_delete_operations(repo UserRepository) {
    vibez.spill("🗑️  Deleting user data...")
    
    sus all_users []User = repo.get_all_users() fam {
        when _ -> damn
    }
    
    ready (len(all_users) > 0) {
        sus user_to_delete User = all_users[len(all_users) - 1]
        
        repo.delete_user(user_to_delete.id) fam {
            when "user not found" -> {
                vibez.spill("  ⚠️  User not found for deletion")
                damn
            }
            when _ -> {
                vibez.spill("  ❌ Failed to delete user")
                damn
            }
        }
        
        vibez.spill("  ✅ Deleted user:", user_to_delete.name, "(ID:", user_to_delete.id, ")")
        vibez.spill("")
    }
}

# Main application
slay main_character() {
    vibez.spill("🗄️  CURSED Database Application Demo")
    vibez.spill("=====================================")
    vibez.spill("")
    
    # Initialize database
    sus pool ConnectionPool = new_connection_pool("database.sqlite", 10) fam {
        when _ -> {
            vibez.spill("❌ Failed to create connection pool")
            damn
        }
    }
    
    initialize_schema(&pool) fam {
        when _ -> {
            vibez.spill("❌ Failed to initialize database schema")
            damn
        }
    }
    
    vibez.spill("✅ Database initialized successfully")
    vibez.spill("")
    
    # Create repository
    sus repo SQLiteUserRepository = new_user_repository(&pool)
    
    # Run demos
    demo_create_users(repo)
    demo_read_operations(repo)
    demo_update_operations(repo)
    demo_delete_operations(repo)
    
    # Final status
    sus final_users []User = repo.get_all_users() fam {
        when _ -> damn
    }
    
    vibez.spill("📊 Final database state:")
    print_user_table(final_users)
    
    vibez.spill("")
    vibez.spill("🎉 Database demo completed successfully!")
    vibez.spill("   - Connection pooling: ✅")
    vibez.spill("   - CRUD operations: ✅")
    vibez.spill("   - Error handling: ✅")
    vibez.spill("   - SQL injection protection: ✅")
}

# Run the application
main()
