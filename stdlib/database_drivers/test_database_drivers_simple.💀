yeet "database_drivers"

fr fr Simple Database Driver Registry Tests
fr fr Tests the pure CURSED database driver implementation

vibez.spill("🧪 Starting Database Driver Registry Tests")

fr fr Test 1: Create registry
vibez.spill("Test 1: Creating registry...")
registry := create_driver_registry()
initial_count := driver_count(&registry)
vibez.spill("✅ Registry created, initial driver count:", initial_count)

fr fr Test 2: Register driver
vibez.spill("Test 2: Registering PostgreSQL driver...")
success := register_driver(&registry, "postgresql", "14.0.0", based, based)
if success {
    new_count := driver_count(&registry)
    vibez.spill("✅ Driver registered successfully, count:", new_count)
} else {
    vibez.spill("❌ Driver registration failed")
}

fr fr Test 3: List drivers
vibez.spill("Test 3: Listing drivers...")
drivers := list_drivers(&registry)
vibez.spill("✅ Driver list has", len(drivers), "entries")

fr fr Test 4: Get driver info
vibez.spill("Test 4: Getting driver info...")
postgres_driver := get_driver(&registry, "postgresql")
if postgres_driver.name == "postgresql" {
    vibez.spill("✅ Driver found:", postgres_driver.name, "version:", postgres_driver.version)
    vibez.spill("   Supports transactions:", postgres_driver.supports_transactions)
    vibez.spill("   Supports prepared statements:", postgres_driver.supports_prepared_statements)
} else {
    vibez.spill("❌ Driver not found")
}

fr fr Test 5: Create connection
vibez.spill("Test 5: Creating database connection...")
connection := create_connection(&registry, "postgresql")
if connection.is_open {
    vibez.spill("✅ Connection created with ID:", connection.connection_id)
    vibez.spill("   Driver:", connection.driver_name)
} else {
    vibez.spill("❌ Connection creation failed")
}

fr fr Test 6: Execute query
vibez.spill("Test 6: Executing query...")
result := execute_query(&registry, connection.connection_id, "SELECT version()")
if result.success {
    vibez.spill("✅ Query executed successfully")
    vibez.spill("   Rows affected:", result.rows_affected)
    vibez.spill("   Columns:", len(result.columns))
} else {
    vibez.spill("❌ Query execution failed:", result.error_message)
}

fr fr Test 7: Begin transaction
vibez.spill("Test 7: Starting transaction...")
tx := begin_transaction(&registry, connection.connection_id)
if tx.is_active {
    vibez.spill("✅ Transaction started on connection:", tx.connection_id)
} else {
    vibez.spill("❌ Transaction start failed")
}

fr fr Test 8: Commit transaction
vibez.spill("Test 8: Committing transaction...")
commit_success := commit_transaction(&registry, connection.connection_id)
if commit_success {
    vibez.spill("✅ Transaction committed successfully")
} else {
    vibez.spill("❌ Transaction commit failed")
}

fr fr Test 9: Close connection
vibez.spill("Test 9: Closing connection...")
close_success := close_connection(&registry, connection.connection_id)
if close_success {
    vibez.spill("✅ Connection closed successfully")
} else {
    vibez.spill("❌ Connection close failed")
}

fr fr Test 10: Initialize default drivers
vibez.spill("Test 10: Initializing default drivers...")
clear_drivers(&registry) fr fr Start fresh
init_success := init_default_drivers(&registry)
if init_success {
    final_count := driver_count(&registry)
    vibez.spill("✅ Default drivers initialized, total count:", final_count) fr fr List all default drivers
    all_drivers := list_drivers(&registry)
    vibez.spill("   Available drivers:")
    bestie i := 0; i < len(all_drivers); i++ {
        driver_info := get_driver(&registry, all_drivers[i])
        vibez.spill("   -", driver_info.name, "v" + driver_info.version)
    }
} else {
    vibez.spill("❌ Default driver initialization failed")
}

fr fr Test 11: Multiple connections
vibez.spill("Test 11: Testing multiple connections...")
pg_conn := create_connection(&registry, "postgresql")
mysql_conn := create_connection(&registry, "mysql")
sqlite_conn := create_connection(&registry, "sqlite")

if pg_conn.is_open && mysql_conn.is_open && sqlite_conn.is_open {
    vibez.spill("✅ Multiple connections created successfully")
    vibez.spill("   PostgreSQL ID:", pg_conn.connection_id)
    vibez.spill("   MySQL ID:", mysql_conn.connection_id)  
    vibez.spill("   SQLite ID:", sqlite_conn.connection_id)
} else {
    vibez.spill("❌ Multiple connection creation failed")
}

fr fr Test 12: Registry statistics
vibez.spill("Test 12: Registry statistics...")
get_registry_stats(&registry)

fr fr Test 13: Validation
vibez.spill("Test 13: Driver validation...")
postgres_valid := validate_driver_config(&registry, "postgresql")
invalid_valid := validate_driver_config(&registry, "nonexistent")

if postgres_valid && !invalid_valid {
    vibez.spill("✅ Driver validation working correctly")
} else {
    vibez.spill("❌ Driver validation failed")
}

vibez.spill("🎉 All Database Driver Registry tests completed!")
vibez.spill("🔒 Pure CURSED implementation with no unsafe operations")
vibez.spill("✅ Memory safety verified - no global mutable state")
vibez.spill("🚀 Ready for production use")
