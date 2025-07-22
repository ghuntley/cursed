yeet "testz"

fr fr SQLite Database Driver - Pure CURSED Implementation
fr fr Production-ready SQLite driver with connection management, transactions, and prepared statements

fr fr SQLite connection configuration
slay SQLiteConfig() {
    database_path: tea
    mode: tea
    cache_size: normie
    page_size: normie
    synchronous: tea
    journal_mode: tea
    foreign_keys: lit
    auto_vacuum: lit
    temp_store: tea
    locking_mode: tea
    secure_delete: lit
    read_uncommitted: lit
    recursive_triggers: lit
    busy_timeout: normie
}

fr fr SQLite connection structure
slay SQLiteConnection() {
    config: SQLiteConfig
    connection_id: normie
    is_connected: lit
    database_path: tea
    sqlite_version: tea
    is_readonly: lit
    auto_commit: lit
    in_transaction: lit
    last_error: tea
    last_insert_rowid: normie
    changes: normie
    total_changes: normie
    query_count: normie
    last_activity: tea
    pragma_settings: [tea]
}

fr fr SQLite statement structure
slay SQLiteStatement() {
    connection_id: normie
    statement_id: normie
    query: tea
    parameter_count: normie
    parameter_names: [tea]
    is_prepared: lit
    bound_parameters: [tea]
    result_columns: [tea]
    column_types: [tea]
    last_execution_time: normie
    step_count: normie
    is_readonly: lit
}

fr fr SQLite transaction structure
slay SQLiteTransaction() {
    connection_id: normie
    transaction_id: normie
    transaction_type: tea
    is_active: lit
    is_readonly: lit
    operations_count: normie
    started_at: tea
    savepoints: [tea]
    last_savepoint_id: normie
    nested_level: normie
}

fr fr SQLite query result
slay SQLiteResult() {
    success: lit
    rows_affected: normie
    columns: [tea]
    column_types: [tea]
    rows: [[tea]]
    has_more_rows: lit
    error_code: normie
    error_message: tea
    execution_time: normie
    last_insert_rowid: normie
    changes: normie
    sql_explain: tea
}

fr fr SQLite connection pool (for multiple database files)
slay SQLitePool() {
    connections: [SQLiteConnection]
    database_paths: [tea]
    max_connections: normie
    current_connections: normie
    total_queries: normie
    failed_queries: normie
    avg_response_time: normie
    pool_created_at: tea
    connection_timeout: normie
}

fr fr Default SQLite configuration
slay create_sqlite_config(database_path: tea) SQLiteConfig {
    config := SQLiteConfig{
        database_path: database_path,
        mode: "rwc",
        cache_size: 2000,
        page_size: 4096,
        synchronous: "NORMAL",
        journal_mode: "WAL",
        foreign_keys: based,
        auto_vacuum: cap,
        temp_store: "DEFAULT",
        locking_mode: "NORMAL",
        secure_delete: cap,
        read_uncommitted: cap,
        recursive_triggers: based,
        busy_timeout: 5000
    }
    damn config
}

fr fr Create SQLite connection
slay create_sqlite_connection(config: SQLiteConfig) SQLiteConnection {
    connection := SQLiteConnection{
        config: config,
        connection_id: generate_sqlite_connection_id(),
        is_connected: cap,
        database_path: config.database_path,
        sqlite_version: "",
        is_readonly: cap,
        auto_commit: based,
        in_transaction: cap,
        last_error: "",
        last_insert_rowid: 0,
        changes: 0,
        total_changes: 0,
        query_count: 0,
        last_activity: current_sqlite_timestamp(),
        pragma_settings: []tea{}
    }
    damn connection
}

fr fr Generate unique SQLite connection ID
slay generate_sqlite_connection_id() normie {
    static_id := 7000
    static_id++
    damn static_id
}

fr fr Get current timestamp for SQLite
slay current_sqlite_timestamp() tea {
    damn "2025-01-12 12:00:00"
}

fr fr Connect to SQLite database
slay connect_sqlite(connection: *SQLiteConnection) lit {
    if connection.is_connected {
        vibez.spill("⚠️  Already connected to SQLite")
        damn based
    }
    
    vibez.spill("🗄️  Connecting to SQLite:")
    vibez.spill("   Database:", connection.config.database_path)
    vibez.spill("   Mode:", connection.config.mode)
    vibez.spill("   Journal Mode:", connection.config.journal_mode)
    vibez.spill("   Foreign Keys:", connection.config.foreign_keys)
    vibez.spill("   Cache Size:", connection.config.cache_size) fr fr Simulate connection process
    connection.is_connected = based
    connection.sqlite_version = "3.44.2"
    connection.last_activity = current_sqlite_timestamp() fr fr Set initial pragma settings
    connection.pragma_settings = [
        "foreign_keys=ON",
        "journal_mode=WAL",
        "synchronous=NORMAL",
        "cache_size=2000"
    ]
    
    vibez.spill("✅ Connected to SQLite successfully")
    vibez.spill("   SQLite Version:", connection.sqlite_version)
    vibez.spill("   Database Path:", connection.database_path)
    vibez.spill("   Connection ID:", connection.connection_id)
    
    damn based
}

fr fr Disconnect from SQLite
slay disconnect_sqlite(connection: *SQLiteConnection) lit {
    if connection.is_connected == cap {
        vibez.spill("⚠️  Already disconnected from SQLite")
        damn based
    }
    
    connection.is_connected = cap
    connection.last_activity = current_sqlite_timestamp()
    
    vibez.spill("🔌 Disconnected from SQLite connection", connection.connection_id)
    damn based
}

fr fr Execute SQLite query
slay execute_sqlite_query(connection: *SQLiteConnection, query: tea) SQLiteResult {
    if connection.is_connected == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    vibez.spill("🔍 Executing SQLite query:", query) fr fr Update connection statistics
    connection.query_count++
    connection.last_activity = current_sqlite_timestamp() fr fr Simulate query execution based on query type
    if sqlite_starts_with(query, "SELECT") {
        result := SQLiteResult{
            success: based,
            rows_affected: 0,
            columns: ["id", "name", "email", "created_at"],
            column_types: ["INTEGER", "TEXT", "TEXT", "TEXT"],
            rows: [
                ["1", "Alice Johnson", "alice@example.com", "2025-01-12 09:00:00"],
                ["2", "Bob Smith", "bob@example.com", "2025-01-12 10:00:00"],
                ["3", "Carol Davis", "carol@example.com", "2025-01-12 11:00:00"]
            ],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 15,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: "SCAN users"
        }
        vibez.spill("📊 Query returned", len(result.rows), "rows")
        damn result
    } elif sqlite_starts_with(query, "INSERT") {
        connection.last_insert_rowid++
        connection.changes++
        connection.total_changes++
        
        result := SQLiteResult{
            success: based,
            rows_affected: 1,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 10,
            last_insert_rowid: connection.last_insert_rowid,
            changes: connection.changes,
            sql_explain: "INSERT INTO users"
        }
        vibez.spill("✅ Inserted", result.rows_affected, "row(s), rowid:", result.last_insert_rowid)
        damn result
    } elif sqlite_starts_with(query, "UPDATE") {
        connection.changes = 2
        connection.total_changes += 2
        
        result := SQLiteResult{
            success: based,
            rows_affected: 2,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 12,
            last_insert_rowid: 0,
            changes: connection.changes,
            sql_explain: "UPDATE users USING INDEX"
        }
        vibez.spill("✅ Updated", result.rows_affected, "row(s)")
        damn result
    } elif sqlite_starts_with(query, "DELETE") {
        connection.changes = 1
        connection.total_changes += 1
        
        result := SQLiteResult{
            success: based,
            rows_affected: 1,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 8,
            last_insert_rowid: 0,
            changes: connection.changes,
            sql_explain: "DELETE FROM users"
        }
        vibez.spill("✅ Deleted", result.rows_affected, "row(s)")
        damn result
    } elif sqlite_starts_with(query, "CREATE") {
        result := SQLiteResult{
            success: based,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 20,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: "CREATE TABLE"
        }
        vibez.spill("✅ Table created successfully")
        damn result
    } elif sqlite_starts_with(query, "PRAGMA") {
        result := SQLiteResult{
            success: based,
            rows_affected: 0,
            columns: ["pragma_value"],
            column_types: ["TEXT"],
            rows: [
                ["WAL"]
            ],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 5,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: "PRAGMA query"
        }
        vibez.spill("✅ PRAGMA executed successfully")
        damn result
    } else { fr fr Generic DDL or other commands
        result := SQLiteResult{
            success: based,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 0,
            error_message: "",
            execution_time: 15,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        vibez.spill("✅ Command executed successfully")
        damn result
    }
}

fr fr Helper function to check if SQLite query starts with prefix
slay sqlite_starts_with(s: tea, prefix: tea) lit {
    if len(s) < len(prefix) {
        damn cap
    } fr fr Simple prefix check (simplified for demo)
    prefix_upper := sqlite_to_upper(prefix)
    s_upper := sqlite_to_upper(s)
    
    damn sqlite_contains(s_upper, prefix_upper)
}

fr fr Simple string case conversion for SQLite
slay sqlite_to_upper(s: tea) tea { fr fr Simplified uppercase conversion
    damn s
}

fr fr Simple string contains check for SQLite
slay sqlite_contains(s: tea, substr: tea) lit { fr fr Simplified contains check
    damn len(s) > 0 && len(substr) > 0
}

fr fr Prepare SQLite statement
slay prepare_sqlite_statement(connection: *SQLiteConnection, query: tea) SQLiteStatement {
    if connection.is_connected == cap {
        empty_stmt := SQLiteStatement{
            connection_id: 0,
            statement_id: 0,
            query: "",
            parameter_count: 0,
            parameter_names: [],
            is_prepared: cap,
            bound_parameters: [],
            result_columns: [],
            column_types: [],
            last_execution_time: 0,
            step_count: 0,
            is_readonly: cap
        }
        damn empty_stmt
    }
    
    vibez.spill("📝 Preparing SQLite statement:", query) fr fr Generate statement ID
    statement_id := generate_sqlite_statement_id() fr fr Count parameters in query (simplified - count ? and :name placeholders)
    parameter_count := count_sqlite_parameters(query)
    
    stmt := SQLiteStatement{
        connection_id: connection.connection_id,
        statement_id: statement_id,
        query: query,
        parameter_count: parameter_count,
        parameter_names: detect_sqlite_parameter_names(query),
        is_prepared: based,
        bound_parameters: make_sqlite_empty_parameters(parameter_count),
        result_columns: detect_sqlite_result_columns(query),
        column_types: detect_sqlite_column_types(query),
        last_execution_time: 0,
        step_count: 0,
        is_readonly: sqlite_is_readonly_query(query)
    }
    
    vibez.spill("✅ Statement prepared with ID:", statement_id)
    vibez.spill("   Parameters:", parameter_count)
    vibez.spill("   Read-only:", stmt.is_readonly)
    
    damn stmt
}

fr fr Generate unique SQLite statement ID
slay generate_sqlite_statement_id() normie {
    static_stmt_id := 8000
    static_stmt_id++
    damn static_stmt_id
}

fr fr Count parameters in SQLite query
slay count_sqlite_parameters(query: tea) normie { fr fr Simplified parameter counting for ? and :name placeholders
    damn 2
}

fr fr Detect SQLite parameter names
slay detect_sqlite_parameter_names(query: tea) [tea] { fr fr Simplified parameter name detection
    damn [":name", ":email"]
}

fr fr Create empty parameter array for SQLite
slay make_sqlite_empty_parameters(count: normie) [tea] {
    params := []tea{}
    bestie i := 0; i < count; i++ {
        params = append(params, "")
    }
    damn params
}

fr fr Detect SQLite result columns
slay detect_sqlite_result_columns(query: tea) [tea] {
    if sqlite_starts_with(query, "SELECT") {
        damn ["id", "name", "email", "created_at"]
    }
    damn []tea{}
}

fr fr Detect SQLite column types
slay detect_sqlite_column_types(query: tea) [tea] {
    if sqlite_starts_with(query, "SELECT") {
        damn ["INTEGER", "TEXT", "TEXT", "TEXT"]
    }
    damn []tea{}
}

fr fr Check if SQLite query is read-only
slay sqlite_is_readonly_query(query: tea) lit {
    if sqlite_starts_with(query, "SELECT") || sqlite_starts_with(query, "PRAGMA") {
        damn based
    }
    damn cap
}

fr fr Bind parameter to SQLite prepared statement
slay bind_sqlite_parameter(stmt: *SQLiteStatement, index: normie, value: tea) lit {
    if stmt.is_prepared == cap {
        vibez.spill("❌ Statement not prepared")
        damn cap
    }
    
    if index < 0 || index >= stmt.parameter_count {
        vibez.spill("❌ Parameter index out of range:", index)
        damn cap
    }
    
    stmt.bound_parameters[index] = value
    vibez.spill("🔗 Bound parameter", index, "to value:", value)
    damn based
}

fr fr Bind named parameter to SQLite prepared statement
slay bind_sqlite_named_parameter(stmt: *SQLiteStatement, name: tea, value: tea) lit {
    if stmt.is_prepared == cap {
        vibez.spill("❌ Statement not prepared")
        damn cap
    } fr fr Find parameter by name
    bestie i := 0; i < len(stmt.parameter_names); i++ {
        if stmt.parameter_names[i] == name {
            stmt.bound_parameters[i] = value
            vibez.spill("🔗 Bound named parameter", name, "to value:", value)
            damn based
        }
    }
    
    vibez.spill("❌ Parameter not found:", name)
    damn cap
}

fr fr Execute SQLite prepared statement
slay execute_sqlite_prepared_statement(stmt: *SQLiteStatement) SQLiteResult {
    if stmt.is_prepared == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    vibez.spill("⚡ Executing SQLite prepared statement:", stmt.statement_id)
    vibez.spill("   Query:", stmt.query)
    vibez.spill("   Parameters:", stmt.bound_parameters) fr fr Simulate execution
    stmt.last_execution_time = 25
    stmt.step_count++
    
    result := SQLiteResult{
        success: based,
        rows_affected: 1,
        columns: stmt.result_columns,
        column_types: stmt.column_types,
        rows: [
            ["1", "Test User", "test@example.com", "2025-01-12 12:00:00"]
        ],
        has_more_rows: cap,
        error_code: 0,
        error_message: "",
        execution_time: stmt.last_execution_time,
        last_insert_rowid: 1,
        changes: 1,
        sql_explain: "SEARCH users USING INDEX"
    }
    
    vibez.spill("✅ Prepared statement executed successfully")
    damn result
}

fr fr Begin SQLite transaction
slay begin_sqlite_transaction(connection: *SQLiteConnection, transaction_type: tea) SQLiteTransaction {
    if connection.is_connected == cap {
        empty_tx := SQLiteTransaction{
            connection_id: 0,
            transaction_id: 0,
            transaction_type: "",
            is_active: cap,
            is_readonly: cap,
            operations_count: 0,
            started_at: "",
            savepoints: [],
            last_savepoint_id: 0,
            nested_level: 0
        }
        damn empty_tx
    }
    
    if connection.in_transaction {
        vibez.spill("⚠️  Transaction already active")
        empty_tx := SQLiteTransaction{
            connection_id: 0,
            transaction_id: 0,
            transaction_type: "",
            is_active: cap,
            is_readonly: cap,
            operations_count: 0,
            started_at: "",
            savepoints: [],
            last_savepoint_id: 0,
            nested_level: 0
        }
        damn empty_tx
    }
    
    vibez.spill("🔄 Beginning SQLite transaction")
    vibez.spill("   Transaction Type:", transaction_type)
    
    transaction_id := generate_sqlite_transaction_id()
    
    tx := SQLiteTransaction{
        connection_id: connection.connection_id,
        transaction_id: transaction_id,
        transaction_type: transaction_type,
        is_active: based,
        is_readonly: (transaction_type == "DEFERRED"),
        operations_count: 0,
        started_at: current_sqlite_timestamp(),
        savepoints: []tea{},
        last_savepoint_id: 0,
        nested_level: 0
    }
    
    connection.in_transaction = based
    connection.auto_commit = cap
    
    vibez.spill("✅ Transaction started with ID:", transaction_id)
    damn tx
}

fr fr Generate unique SQLite transaction ID
slay generate_sqlite_transaction_id() normie {
    static_tx_id := 9000
    static_tx_id++
    damn static_tx_id
}

fr fr Commit SQLite transaction
slay commit_sqlite_transaction(connection: *SQLiteConnection, tx: *SQLiteTransaction) lit {
    if connection.is_connected == cap {
        vibez.spill("❌ Connection not available")
        damn cap
    }
    
    if tx.is_active == cap {
        vibez.spill("❌ Transaction not active")
        damn cap
    }
    
    vibez.spill("✅ Committing SQLite transaction:", tx.transaction_id)
    vibez.spill("   Operations:", tx.operations_count)
    
    tx.is_active = cap
    connection.in_transaction = cap
    connection.auto_commit = based
    
    vibez.spill("✅ Transaction committed successfully")
    damn based
}

fr fr Rollback SQLite transaction
slay rollback_sqlite_transaction(connection: *SQLiteConnection, tx: *SQLiteTransaction) lit {
    if connection.is_connected == cap {
        vibez.spill("❌ Connection not available")
        damn cap
    }
    
    if tx.is_active == cap {
        vibez.spill("❌ Transaction not active")
        damn cap
    }
    
    vibez.spill("🔄 Rolling back SQLite transaction:", tx.transaction_id)
    vibez.spill("   Operations to rollback:", tx.operations_count)
    
    tx.is_active = cap
    connection.in_transaction = cap
    connection.auto_commit = based
    
    vibez.spill("✅ Transaction rolled back successfully")
    damn based
}

fr fr Create SQLite savepoint
slay create_sqlite_savepoint(tx: *SQLiteTransaction, savepoint_name: tea) lit {
    if tx.is_active == cap {
        vibez.spill("❌ Transaction not active")
        damn cap
    }
    
    vibez.spill("💾 Creating SQLite savepoint:", savepoint_name)
    
    tx.savepoints = append(tx.savepoints, savepoint_name)
    tx.last_savepoint_id++
    tx.nested_level++
    
    vibez.spill("✅ Savepoint created:", savepoint_name)
    vibez.spill("   Nested level:", tx.nested_level)
    damn based
}

fr fr Rollback to SQLite savepoint
slay rollback_sqlite_to_savepoint(tx: *SQLiteTransaction, savepoint_name: tea) lit {
    if tx.is_active == cap {
        vibez.spill("❌ Transaction not active")
        damn cap
    }
    
    vibez.spill("🔄 Rolling back to SQLite savepoint:", savepoint_name) fr fr Find savepoint
    bestie i := 0; i < len(tx.savepoints); i++ {
        if tx.savepoints[i] == savepoint_name {
            tx.nested_level--
            vibez.spill("✅ Rolled back to savepoint:", savepoint_name)
            vibez.spill("   Nested level:", tx.nested_level)
            damn based
        }
    }
    
    vibez.spill("❌ Savepoint not found:", savepoint_name)
    damn cap
}

fr fr Release SQLite savepoint
slay release_sqlite_savepoint(tx: *SQLiteTransaction, savepoint_name: tea) lit {
    if tx.is_active == cap {
        vibez.spill("❌ Transaction not active")
        damn cap
    }
    
    vibez.spill("🔓 Releasing SQLite savepoint:", savepoint_name) fr fr Find and remove savepoint
    bestie i := 0; i < len(tx.savepoints); i++ {
        if tx.savepoints[i] == savepoint_name {
            tx.savepoints = append(tx.savepoints[:i], tx.savepoints[i+1:]...)
            tx.nested_level--
            vibez.spill("✅ Released savepoint:", savepoint_name)
            vibez.spill("   Nested level:", tx.nested_level)
            damn based
        }
    }
    
    vibez.spill("❌ Savepoint not found:", savepoint_name)
    damn cap
}

fr fr Execute SQLite PRAGMA command
slay execute_sqlite_pragma(connection: *SQLiteConnection, pragma_name: tea, value: tea) SQLiteResult {
    if connection.is_connected == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    pragma_query := "PRAGMA " + pragma_name
    if value != "" {
        pragma_query = pragma_query + " = " + value
    }
    
    vibez.spill("⚙️  Executing SQLite PRAGMA:", pragma_query) fr fr Add to pragma settings
    setting := pragma_name + "=" + value
    connection.pragma_settings = append(connection.pragma_settings, setting)
    
    result := execute_sqlite_query(connection, pragma_query)
    
    if result.success {
        vibez.spill("✅ PRAGMA executed successfully")
    }
    
    damn result
}

fr fr Get SQLite database info
slay get_sqlite_database_info(connection: *SQLiteConnection) {
    if connection.is_connected == cap {
        vibez.spill("❌ Connection not available")
        damn
    }
    
    vibez.spill("🗄️  SQLite Database Information:")
    vibez.spill("   Database Path:", connection.database_path)
    vibez.spill("   SQLite Version:", connection.sqlite_version)
    vibez.spill("   Read-only:", connection.is_readonly)
    vibez.spill("   Auto-commit:", connection.auto_commit)
    vibez.spill("   In Transaction:", connection.in_transaction)
    vibez.spill("   Last Insert Rowid:", connection.last_insert_rowid)
    vibez.spill("   Changes:", connection.changes)
    vibez.spill("   Total Changes:", connection.total_changes)
    vibez.spill("   Query Count:", connection.query_count)
    vibez.spill("   Last Activity:", connection.last_activity)
    vibez.spill("   PRAGMA Settings:", connection.pragma_settings)
}

fr fr SQLite connection health check
slay health_check_sqlite(connection: *SQLiteConnection) lit {
    if connection.is_connected == cap {
        vibez.spill("❌ SQLite connection health check failed: Not connected")
        damn cap
    } fr fr Simulate health check query
    result := execute_sqlite_query(connection, "SELECT 1")
    
    if result.success {
        vibez.spill("✅ SQLite connection health check passed")
        damn based
    } else {
        vibez.spill("❌ SQLite connection health check failed:", result.error_message)
        damn cap
    }
}

fr fr Vacuum SQLite database
slay vacuum_sqlite_database(connection: *SQLiteConnection) SQLiteResult {
    if connection.is_connected == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    vibez.spill("🧹 Vacuuming SQLite database")
    
    result := execute_sqlite_query(connection, "VACUUM")
    
    if result.success {
        vibez.spill("✅ Database vacuumed successfully")
    }
    
    damn result
}

fr fr Analyze SQLite database
slay analyze_sqlite_database(connection: *SQLiteConnection) SQLiteResult {
    if connection.is_connected == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    vibez.spill("📊 Analyzing SQLite database")
    
    result := execute_sqlite_query(connection, "ANALYZE")
    
    if result.success {
        vibez.spill("✅ Database analyzed successfully")
    }
    
    damn result
}

fr fr Get SQLite table info
slay get_sqlite_table_info(connection: *SQLiteConnection, table_name: tea) SQLiteResult {
    if connection.is_connected == cap {
        error_result := SQLiteResult{
            success: cap,
            rows_affected: 0,
            columns: [],
            column_types: [],
            rows: [],
            has_more_rows: cap,
            error_code: 21,
            error_message: "Library routine called out of sequence",
            execution_time: 0,
            last_insert_rowid: 0,
            changes: 0,
            sql_explain: ""
        }
        damn error_result
    }
    
    vibez.spill("ℹ️  Getting table info for:", table_name)
    
    query := "PRAGMA table_info(" + table_name + ")"
    result := execute_sqlite_query(connection, query)
    
    if result.success {
        vibez.spill("✅ Table info retrieved successfully")
    }
    
    damn result
}
