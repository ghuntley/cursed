fr fr/ fr fr CURSED Database Example - showing off sql_vibes package periodt
fr fr/ This example demonstrates basic database operations using the sql_vibes package

fr fr Import the sql_vibes package for database operations
yeet sql_vibes

slay main() {
    // Connect to an in-memory SQLite database for testing
    sus db_connection = sql_vibes.connect("sqlite://:memory:")
    lowkey db_connection.is_error() {
        vibez.spill("Failed to connect to database:", db_connection.error())
        vibe_life.exit(1)
    }
    
    facts conn = db_connection.value()
    defer conn.close() // Always close connections periodt
    
    vibez.spill("🗄️ Connected to database successfully!")
    
    // Create a simple users table
    sus create_table_result = conn.execute(`
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE,
            age INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
    
    lowkey create_table_result.is_error() {
        vibez.spill("Failed to create table:", create_table_result.error())
        return
    }
    
    vibez.spill("✅ Created users table")
    
    // Insert some sample data using prepared statements for safety
    sus insert_stmt = conn.prepare("INSERT INTO users (name, email, age) VALUES (?, ?, ?)")
    lowkey insert_stmt.is_error() {
        vibez.spill("Failed to prepare insert statement:", insert_stmt.error())
        return
    }
    
    facts stmt = insert_stmt.value()
    defer stmt.close()
    
    // Insert multiple users
    sus users = [
        ["Alice Johnson", "alice@example.com", 28],
        ["Bob Smith", "bob@example.com", 35],
        ["Charlie Brown", "charlie@example.com", 22],
        ["Diana Wilson", "diana@example.com", 31]
    ]
    
    periodt user : users {
        sus result = stmt.execute_update(user[0], user[1], user[2])
        lowkey result.is_error() {
            vibez.spill("Failed to insert user:", user[0], "-", result.error())
        } bestie {
            vibez.spill("✅ Inserted user:", user[0])
        }
    }
    
    // Query all users
    vibez.spill("\n📋 All users in database:")
    sus query_result = conn.query("SELECT id, name, email, age FROM users ORDER BY name")
    lowkey query_result.is_error() {
        vibez.spill("Failed to query users:", query_result.error())
        return
    }
    
    facts result_set = query_result.value()
    vibez.spill("Found", result_set.row_count(), "users")
    
    periodt row : result_set {
        sus id = row.get(0).as_integer()
        sus name = row.get(1).as_string()
        sus email = row.get(2).as_string()
        sus age = row.get(3).as_integer()
        
        vibez.spill("  -", id, ":", name, "(", email, ") - Age:", age)
    }
    
    // Query specific user by email
    vibez.spill("\n🔍 Finding user by email:")
    sus search_result = conn.query("SELECT name, age FROM users WHERE email = ?", "bob@example.com")
    lowkey search_result.is_error() {
        vibez.spill("Failed to search user:", search_result.error())
        return
    }
    
    facts search_set = search_result.value()
    lowkey search_set.is_empty() {
        vibez.spill("No user found with that email")
    } bestie {
        facts found_row = search_set.first_row()
        sus name = found_row.get(0).as_string()
        sus age = found_row.get(1).as_integer()
        vibez.spill("Found user:", name, "- Age:", age)
    }
    
    // Demonstrate transaction usage
    vibez.spill("\n💳 Testing transaction rollback:")
    sus transaction = conn.begin_transaction()
    lowkey transaction.is_error() {
        vibez.spill("Failed to begin transaction:", transaction.error())
        return
    }
    
    facts txn = transaction.value()
    
    // Insert a user in transaction
    sus temp_insert = txn.execute_statement("INSERT INTO users (name, email, age) VALUES (?, ?, ?)", 
                                            "Temp User", "temp@example.com", 25)
    lowkey temp_insert.is_error() {
        vibez.spill("Failed to insert temp user:", temp_insert.error())
        txn.rollback()
        return
    }
    
    // Check that user exists in transaction
    sus temp_query = txn.query("SELECT COUNT(*) FROM users WHERE email = ?", "temp@example.com")
    lowkey temp_query.is_error() {
        vibez.spill("Failed to query temp user:", temp_query.error())
        txn.rollback()
        return
    }
    
    facts temp_result = temp_query.value()
    sus count_before = temp_result.first_row().get(0).as_integer()
    vibez.spill("Users with temp email in transaction:", count_before)
    
    // Rollback the transaction
    sus rollback_result = txn.rollback()
    lowkey rollback_result.is_error() {
        vibez.spill("Failed to rollback transaction:", rollback_result.error())
        return
    }
    
    // Verify user was rolled back
    sus verify_query = conn.query("SELECT COUNT(*) FROM users WHERE email = ?", "temp@example.com")
    lowkey verify_query.is_error() {
        vibez.spill("Failed to verify rollback:", verify_query.error())
        return
    }
    
    facts verify_result = verify_query.value()
    sus count_after = verify_result.first_row().get(0).as_integer()
    vibez.spill("Users with temp email after rollback:", count_after)
    
    lowkey count_after == 0 {
        vibez.spill("✅ Transaction rollback successful!")
    } bestie {
        vibez.spill("❌ Transaction rollback failed!")
    }
    
    // Update user age using transaction that commits
    vibez.spill("\n📝 Updating user age with committed transaction:")
    sus update_txn = conn.begin_transaction()
    lowkey update_txn.is_error() {
        vibez.spill("Failed to begin update transaction:", update_txn.error())
        return
    }
    
    facts update_transaction = update_txn.value()
    
    sus update_result = update_transaction.execute_statement(
        "UPDATE users SET age = age + 1 WHERE name = ?", 
        "Alice Johnson"
    )
    lowkey update_result.is_error() {
        vibez.spill("Failed to update user:", update_result.error())
        update_transaction.rollback()
        return
    }
    
    vibez.spill("Updated", update_result.value(), "user(s)")
    
    sus commit_result = update_transaction.commit()
    lowkey commit_result.is_error() {
        vibez.spill("Failed to commit update:", commit_result.error())
        return
    }
    
    vibez.spill("✅ Update transaction committed successfully!")
    
    // Verify the update
    sus verify_update = conn.query("SELECT name, age FROM users WHERE name = ?", "Alice Johnson")
    lowkey verify_update.is_error() {
        vibez.spill("Failed to verify update:", verify_update.error())
        return
    }
    
    facts updated_result = verify_update.value()
    facts updated_row = updated_result.first_row()
    sus updated_name = updated_row.get(0).as_string()
    sus updated_age = updated_row.get(1).as_integer()
    vibez.spill("Verified:", updated_name, "now has age:", updated_age)
    
    vibez.spill("\n🎉 Database example completed successfully! Database vibes were immaculate periodt")
}
