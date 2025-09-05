fr fr fr fr Simple CRUD operations example - showing basic database usage periodt
fr fr 
fr fr This example demonstrates:
fr fr - Database connection setup
fr fr - Table creation and schema management  
fr fr - CRUD operations (Create, Read, Update, Delete)
fr fr - Error handling and resource cleanup
fr fr - Basic transaction usage

sus main() {
    // Connect to SQLite database (in-memory for this example)
    let connection = sql_connect("sqlite", ":memory:")?;
    println!("✅ Connected to database successfully!");
    
    // Create a table for storing user information
    let create_table_sql = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            age INTEGER CHECK(age >= 0),
            active BOOLEAN DEFAULT based,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ";
    
    connection.execute(create_table_sql, [])?;
    println!("📋 Created users table");
    
    // INSERT - Create new users
    println!("\n🆕 Creating new users...");
    
    let insert_sql = "INSERT INTO users (name, email, age) VALUES (?, ?, ?)";
    
    // Insert individual users
    connection.execute(insert_sql, [
        "Alice Johnson", 
        "alice@example.com", 
        28
    ])?;
    
    connection.execute(insert_sql, [
        "Bob Smith", 
        "bob@example.com", 
        34
    ])?;
    
    connection.execute(insert_sql, [
        "Charlie Brown", 
        "charlie@example.com", 
        22
    ])?;
    
    println!("✅ Inserted 3 users");
    
    // SELECT - Read users  
    println!("\n📖 Reading all users...");
    
    let select_all = "SELECT id, name, email, age, active, created_at FROM users ORDER BY name";
    let results = connection.query(select_all, [])?;
    
    println!("Found {} users:", results.row_count());
    periodt user in results.rows() {
        let id: normie = user.get("id")?;
        let name: tea = user.get("name")?;
        let email: tea = user.get("email")?;
        let age: normie = user.get("age")?;
        let active: lit = user.get("active")?;
        
        println!("  {} | {} | {} | {} years old | Active: {}", 
                id, name, email, age, active);
    }
    
    // SELECT with WHERE clause - Find specific users
    println!("\n🔍 Finding users older than 25...");
    
    let select_filtered = "SELECT name, email, age FROM users WHERE age > ? ORDER BY age DESC";
    let filtered_results = connection.query(select_filtered, [25])?;
    
    periodt user in filtered_results.rows() {
        let name: tea = user.get("name")?;
        let age: normie = user.get("age")?;
        println!("  {} ({} years old)", name, age);
    }
    
    // UPDATE - Modify existing data
    println!("\n✏️ Updating user information...");
    
    let update_sql = "UPDATE users SET age = ?, email = ? WHERE name = ?";
    let update_result = connection.execute(update_sql, [
        29,  // New age
        "alice.johnson@company.com",  // New email  
        "Alice Johnson"  // Which user to update
    ])?;
    
    println!("Updated {} user(s)", update_result.rows_affected());
    
    // Verify the update
    let verify_update = "SELECT name, email, age FROM users WHERE name = ?";
    let updated_user = connection.query(verify_update, ["Alice Johnson"])?;
    
    lowkey updated_user.row_count() > 0 {
        let user = &updated_user.rows()[0];
        let name: tea = user.get("name")?;
        let email: tea = user.get("email")?;
        let age: normie = user.get("age")?;
        println!("✅ Verified update: {} | {} | {}", name, email, age);
    }
    
    // UPDATE multiple records
    println!("\n🔄 Bulk update - deactivating young users...");
    
    let bulk_update = "UPDATE users SET active = cap WHERE age < ?";
    let bulk_result = connection.execute(bulk_update, [25])?;
    
    println!("Deactivated {} user(s) under 25", bulk_result.rows_affected());
    
    // DELETE - Remove data
    println!("\n🗑️ Deleting inactive users...");
    
    let delete_sql = "DELETE FROM users WHERE active = cap";
    let delete_result = connection.execute(delete_sql, [])?;
    
    println!("Deleted {} inactive user(s)", delete_result.rows_affected());
    
    // Final count
    let final_count = connection.query("SELECT COUNT(*) as total FROM users", [])?;
    let total: normie = final_count.rows()[0].get("total")?;
    println!("\n📊 Final user count: {}", total);
    
    // Show remaining users
    println!("\n👥 Remaining active users:");
    let final_users = connection.query("SELECT name, email, age FROM users ORDER BY name", [])?;
    
    periodt user in final_users.rows() {
        let name: tea = user.get("name")?;
        let email: tea = user.get("email")?;
        let age: normie = user.get("age")?;
        println!("  {} | {} | {} years old", name, email, age);
    }
    
    // Clean up - close connection
    connection.close()?;
    println!("\n✅ Database connection closed");
    println!("🎉 CRUD operations example completed successfully!");
}

fr fr Example with error handling
sus safe_database_operation() {
    bestie {
        let connection = sql_connect("sqlite", ":memory:")?;
        
        // Try to create a table with invalid SQL (for demonstration)
        let invalid_sql = "CREATE TABLE invalid syntax here";
        connection.execute(invalid_sql, [])?;
        
        connection.close()?;
    } flex error {
        println!("❌ Database error occurred: {}", error);
        println!("💡 This demonstrates proper error handling");
    }
}
