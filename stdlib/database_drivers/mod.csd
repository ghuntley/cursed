yeet "testz"

# Pure CURSED Database Driver Registry Implementation
# Eliminates unsafe global static access with safe state management

# Driver information structure
slay DriverInfo() {
    name: tea
    version: tea
    supports_transactions: lit
    supports_prepared_statements: lit
    connection_string: tea
    is_active: lit
}

# Connection information structure
slay ConnectionInfo() {
    driver_name: tea
    is_open: lit
    connection_id: normie
    last_query: tea
    transaction_active: lit
}

# Query result structure
slay QueryResult() {
    rows_affected: normie
    columns: [tea]
    has_data: lit
    error_message: tea
    success: lit
}

# Statement information structure
slay StatementInfo() {
    query: tea
    parameter_count: normie
    is_prepared: lit
    bound_parameters: [tea]
}

# Transaction information structure
slay TransactionInfo() {
    connection_id: normie
    is_active: lit
    operations_count: normie
    started_at: tea
}

# Driver registry structure - safe state container
slay DriverRegistry() {
    drivers: [DriverInfo]
    next_connection_id: normie
    active_connections: [ConnectionInfo]
    prepared_statements: [StatementInfo]
    active_transactions: [TransactionInfo]
}

# Create new driver registry instance
slay create_driver_registry() DriverRegistry {
    registry := DriverRegistry{
        drivers: [],
        next_connection_id: 1,
        active_connections: [],
        prepared_statements: [],
        active_transactions: []
    }
    damn registry
}

# Register a database driver
slay register_driver(registry: *DriverRegistry, name: tea, version: tea, 
                    supports_tx: lit, supports_prep: lit) lit {
    # Check if driver already exists
    bestie i := 0; i < len(registry.drivers); i++ {
        if registry.drivers[i].name == name {
            vibez.spill("⚠️  Driver already registered:", name)
            damn cap
        }
    }
    
    # Create new driver info
    driver := DriverInfo{
        name: name,
        version: version,
        supports_transactions: supports_tx,
        supports_prepared_statements: supports_prep,
        connection_string: "",
        is_active: based
    }
    
    # Add to registry
    registry.drivers = append(registry.drivers, driver)
    vibez.spill("📦 Registered database driver:", name, "version:", version)
    damn based
}

# Get driver by name
slay get_driver(registry: *DriverRegistry, name: tea) DriverInfo {
    bestie i := 0; i < len(registry.drivers); i++ {
        if registry.drivers[i].name == name {
            damn registry.drivers[i]
        }
    }
    
    # Return empty driver if not found
    empty_driver := DriverInfo{
        name: "",
        version: "",
        supports_transactions: cap,
        supports_prepared_statements: cap,
        connection_string: "",
        is_active: cap
    }
    damn empty_driver
}

# List all registered drivers
slay list_drivers(registry: *DriverRegistry) [tea] {
    driver_names := []tea{}
    bestie i := 0; i < len(registry.drivers); i++ {
        driver_names = append(driver_names, registry.drivers[i].name)
    }
    damn driver_names
}

# Count registered drivers
slay driver_count(registry: *DriverRegistry) normie {
    damn len(registry.drivers)
}

# Remove driver by name
slay unregister_driver(registry: *DriverRegistry, name: tea) lit {
    bestie i := 0; i < len(registry.drivers); i++ {
        if registry.drivers[i].name == name {
            # Remove driver from slice
            registry.drivers = append(registry.drivers[:i], registry.drivers[i+1:]...)
            vibez.spill("🗑️  Unregistered driver:", name)
            damn based
        }
    }
    damn cap
}

# Clear all drivers
slay clear_drivers(registry: *DriverRegistry) {
    registry.drivers = []DriverInfo{}
    vibez.spill("🧹 Cleared all drivers")
}

# Create database connection
slay create_connection(registry: *DriverRegistry, driver_name: tea) ConnectionInfo {
    driver := get_driver(registry, driver_name)
    if driver.name == "" {
        vibez.spill("❌ Driver not found:", driver_name)
        empty_conn := ConnectionInfo{
            driver_name: "",
            is_open: cap,
            connection_id: 0,
            last_query: "",
            transaction_active: cap
        }
        damn empty_conn
    }
    
    # Create new connection
    connection := ConnectionInfo{
        driver_name: driver_name,
        is_open: based,
        connection_id: registry.next_connection_id,
        last_query: "",
        transaction_active: cap
    }
    
    registry.next_connection_id++
    registry.active_connections = append(registry.active_connections, connection)
    vibez.spill("🔌 Created connection", connection.connection_id, "for driver:", driver_name)
    damn connection
}

# Execute query on connection
slay execute_query(registry: *DriverRegistry, connection_id: normie, query: tea) QueryResult {
    # Find connection
    bestie i := 0; i < len(registry.active_connections); i++ {
        if registry.active_connections[i].connection_id == connection_id {
            if registry.active_connections[i].is_open == cap {
                vibez.spill("❌ Connection", connection_id, "is closed")
                result := QueryResult{
                    rows_affected: 0,
                    columns: [],
                    has_data: cap,
                    error_message: "Connection closed",
                    success: cap
                }
                damn result
            }
            
            # Update last query
            registry.active_connections[i].last_query = query
            vibez.spill("🔍 Executing query on connection", connection_id, ":", query)
            
            # Simulate successful query execution
            result := QueryResult{
                rows_affected: 1,
                columns: ["id", "name", "created_at"],
                has_data: based,
                error_message: "",
                success: based
            }
            damn result
        }
    }
    
    # Connection not found
    vibez.spill("❌ Connection", connection_id, "not found")
    result := QueryResult{
        rows_affected: 0,
        columns: [],
        has_data: cap,
        error_message: "Connection not found",
        success: cap
    }
    damn result
}

# Prepare statement
slay prepare_statement(registry: *DriverRegistry, connection_id: normie, query: tea) StatementInfo {
    # Check if connection exists and supports prepared statements
    bestie i := 0; i < len(registry.active_connections); i++ {
        if registry.active_connections[i].connection_id == connection_id {
            driver := get_driver(registry, registry.active_connections[i].driver_name)
            if driver.supports_prepared_statements == cap {
                vibez.spill("❌ Driver does not support prepared statements")
                empty_stmt := StatementInfo{
                    query: "",
                    parameter_count: 0,
                    is_prepared: cap,
                    bound_parameters: []
                }
                damn empty_stmt
            }
            
            # Create prepared statement
            stmt := StatementInfo{
                query: query,
                parameter_count: 0,
                is_prepared: based,
                bound_parameters: []tea{}
            }
            
            registry.prepared_statements = append(registry.prepared_statements, stmt)
            vibez.spill("📝 Prepared statement:", query)
            damn stmt
        }
    }
    
    # Connection not found
    vibez.spill("❌ Connection", connection_id, "not found for statement preparation")
    empty_stmt := StatementInfo{
        query: "",
        parameter_count: 0,
        is_prepared: cap,
        bound_parameters: []
    }
    damn empty_stmt
}

# Begin transaction
slay begin_transaction(registry: *DriverRegistry, connection_id: normie) TransactionInfo {
    # Find connection
    bestie i := 0; i < len(registry.active_connections); i++ {
        if registry.active_connections[i].connection_id == connection_id {
            if registry.active_connections[i].transaction_active {
                vibez.spill("⚠️  Transaction already active on connection", connection_id)
                empty_tx := TransactionInfo{
                    connection_id: 0,
                    is_active: cap,
                    operations_count: 0,
                    started_at: ""
                }
                damn empty_tx
            }
            
            # Check if driver supports transactions
            driver := get_driver(registry, registry.active_connections[i].driver_name)
            if driver.supports_transactions == cap {
                vibez.spill("❌ Driver does not support transactions")
                empty_tx := TransactionInfo{
                    connection_id: 0,
                    is_active: cap,
                    operations_count: 0,
                    started_at: ""
                }
                damn empty_tx
            }
            
            # Create transaction
            tx := TransactionInfo{
                connection_id: connection_id,
                is_active: based,
                operations_count: 0,
                started_at: "2025-01-12 12:00:00"
            }
            
            registry.active_connections[i].transaction_active = based
            registry.active_transactions = append(registry.active_transactions, tx)
            vibez.spill("🔄 Started transaction on connection", connection_id)
            damn tx
        }
    }
    
    # Connection not found
    vibez.spill("❌ Connection", connection_id, "not found for transaction")
    empty_tx := TransactionInfo{
        connection_id: 0,
        is_active: cap,
        operations_count: 0,
        started_at: ""
    }
    damn empty_tx
}

# Commit transaction
slay commit_transaction(registry: *DriverRegistry, connection_id: normie) lit {
    # Find and commit transaction
    bestie i := 0; i < len(registry.active_transactions); i++ {
        if registry.active_transactions[i].connection_id == connection_id &&
           registry.active_transactions[i].is_active {
            
            # Remove transaction from active list
            registry.active_transactions = append(registry.active_transactions[:i], 
                                                 registry.active_transactions[i+1:]...)
            
            # Update connection status
            bestie j := 0; j < len(registry.active_connections); j++ {
                if registry.active_connections[j].connection_id == connection_id {
                    registry.active_connections[j].transaction_active = cap
                }
            }
            
            vibez.spill("✅ Committed transaction on connection", connection_id)
            damn based
        }
    }
    
    vibez.spill("❌ No active transaction found on connection", connection_id)
    damn cap
}

# Rollback transaction
slay rollback_transaction(registry: *DriverRegistry, connection_id: normie) lit {
    # Find and rollback transaction
    bestie i := 0; i < len(registry.active_transactions); i++ {
        if registry.active_transactions[i].connection_id == connection_id &&
           registry.active_transactions[i].is_active {
            
            # Remove transaction from active list
            registry.active_transactions = append(registry.active_transactions[:i], 
                                                 registry.active_transactions[i+1:]...)
            
            # Update connection status
            bestie j := 0; j < len(registry.active_connections); j++ {
                if registry.active_connections[j].connection_id == connection_id {
                    registry.active_connections[j].transaction_active = cap
                }
            }
            
            vibez.spill("🔄 Rolled back transaction on connection", connection_id)
            damn based
        }
    }
    
    vibez.spill("❌ No active transaction found on connection", connection_id)
    damn cap
}

# Close connection
slay close_connection(registry: *DriverRegistry, connection_id: normie) lit {
    bestie i := 0; i < len(registry.active_connections); i++ {
        if registry.active_connections[i].connection_id == connection_id {
            registry.active_connections[i].is_open = cap
            vibez.spill("🔌 Closed connection", connection_id)
            damn based
        }
    }
    
    vibez.spill("❌ Connection", connection_id, "not found")
    damn cap
}

# Get connection status
slay get_connection_status(registry: *DriverRegistry, connection_id: normie) ConnectionInfo {
    bestie i := 0; i < len(registry.active_connections); i++ {
        if registry.active_connections[i].connection_id == connection_id {
            damn registry.active_connections[i]
        }
    }
    
    # Return empty connection if not found
    empty_conn := ConnectionInfo{
        driver_name: "",
        is_open: cap,
        connection_id: 0,
        last_query: "",
        transaction_active: cap
    }
    damn empty_conn
}

# Initialize with default drivers
slay init_default_drivers(registry: *DriverRegistry) lit {
    sus success lit = based
    
    # Register PostgreSQL driver
    if register_driver(registry, "postgresql", "14.0.0", based, based) == cap {
        success = cap
    }
    
    # Register MySQL driver
    if register_driver(registry, "mysql", "8.0.0", based, based) == cap {
        success = cap
    }
    
    # Register SQLite driver
    if register_driver(registry, "sqlite", "3.39.0", based, based) == cap {
        success = cap
    }
    
    # Register Redis driver (no transactions/prepared statements)
    if register_driver(registry, "redis", "7.0.0", cap, cap) == cap {
        success = cap
    }
    
    # Register MongoDB driver
    if register_driver(registry, "mongodb", "6.0.0", based, cap) == cap {
        success = cap
    }
    
    if success {
        vibez.spill("🚀 Successfully initialized", driver_count(registry), "default database drivers")
    } else {
        vibez.spill("⚠️  Some drivers failed to initialize")
    }
    
    damn success
}

# Get registry statistics
slay get_registry_stats(registry: *DriverRegistry) {
    vibez.spill("📊 Database Driver Registry Statistics:")
    vibez.spill("   Registered drivers:", driver_count(registry))
    vibez.spill("   Active connections:", len(registry.active_connections))
    vibez.spill("   Prepared statements:", len(registry.prepared_statements))
    vibez.spill("   Active transactions:", len(registry.active_transactions))
    vibez.spill("   Next connection ID:", registry.next_connection_id)
}

# Validate driver configuration
slay validate_driver_config(registry: *DriverRegistry, driver_name: tea) lit {
    driver := get_driver(registry, driver_name)
    if driver.name == "" {
        vibez.spill("❌ Driver", driver_name, "not found")
        damn cap
    }
    
    if driver.is_active == cap {
        vibez.spill("❌ Driver", driver_name, "is not active")
        damn cap
    }
    
    vibez.spill("✅ Driver", driver_name, "configuration valid")
    damn based
}
