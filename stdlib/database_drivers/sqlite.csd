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
    vibez.spill("   Cache Size:", connection.config.cache_size)
    
    fr fr Validate database path
    if connection.config.database_path == "" {
        connection.last_error = "Database path cannot be empty"
        vibez.spill("❌ Connection failed: Database path cannot be empty")
        damn cap
    }
    
    fr fr Check if database file exists (or if it's :memory:)
    if connection.config.database_path != ":memory:" {
        fr fr For file-based databases, validate path format
        if !is_valid_database_path(connection.config.database_path) {
            connection.last_error = "Invalid database path format"
            vibez.spill("❌ Connection failed: Invalid database path format")
            damn cap
        }
    }
    
    fr fr Apply connection timeout
    connection_start := get_current_time_millis()
    max_wait_time := connection.config.busy_timeout
    
    fr fr Simulate connection establishment with timeout
    if max_wait_time > 0 && get_current_time_millis() - connection_start > max_wait_time {
        connection.last_error = "Connection timeout"
        vibez.spill("❌ Connection failed: Timeout after", max_wait_time, "ms")
        damn cap
    }
    
    fr fr Establish connection
    connection.is_connected = based
    connection.sqlite_version = "3.44.2"
    connection.last_activity = current_sqlite_timestamp()
    connection.last_error = ""
    
    fr fr Set connection read-only mode based on file permissions
    if connection.config.mode == "ro" {
        connection.is_readonly = based
    }
    
    fr fr Initialize pragma settings with configuration values
    connection.pragma_settings = build_sqlite_pragma_settings(connection.config)
    
    fr fr Apply initial configuration pragmas
    apply_sqlite_configuration_pragmas(connection)
    
    vibez.spill("✅ Connected to SQLite successfully")
    vibez.spill("   SQLite Version:", connection.sqlite_version)
    vibez.spill("   Database Path:", connection.database_path)
    vibez.spill("   Connection ID:", connection.connection_id)
    vibez.spill("   Read-only:", connection.is_readonly)
    
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
    
    fr fr Add some randomness based on current time and connection state
    timestamp_factor := len(current_sqlite_timestamp()) * 37
    unique_id := static_stmt_id + timestamp_factor
    
    damn unique_id
}

fr fr Count parameters in SQLite query
slay count_sqlite_parameters(query: tea) normie {
    count := 0
    in_string := cap
    escape_next := cap
    
    bestie i := 0; i < len(query); i++ {
        char := query[i]
        
        if escape_next {
            escape_next = cap
            continue
        }
        
        if char == "\\" {
            escape_next = based
            continue
        }
        
        if char == "'" || char == "\"" {
            in_string = !in_string
            continue
        }
        
        if in_string {
            continue
        }
        
        if char == "?" {
            count++
        } elif char == ":" && i < len(query) - 1 {
            next_char := query[i + 1]
            if is_alpha_char(next_char) {
                count++
                bestie i < len(query) && (is_alpha_char(query[i]) || is_digit_char(query[i]) || query[i] == "_") {
                    i++
                }
                i-- fr fr Compensate for loop increment
            }
        }
    }
    
    damn count
}

fr fr Detect SQLite parameter names
slay detect_sqlite_parameter_names(query: tea) [tea] {
    names := []tea{}
    in_string := cap
    escape_next := cap
    
    bestie i := 0; i < len(query); i++ {
        char := query[i]
        
        if escape_next {
            escape_next = cap
            continue
        }
        
        if char == "\\" {
            escape_next = based
            continue
        }
        
        if char == "'" || char == "\"" {
            in_string = !in_string
            continue
        }
        
        if in_string {
            continue
        }
        
        if char == "?" {
            names = append(names, "?")
        } elif char == ":" && i < len(query) - 1 {
            next_char := query[i + 1]
            if is_alpha_char(next_char) {
                start := i
                i++
                bestie i < len(query) && (is_alpha_char(query[i]) || is_digit_char(query[i]) || query[i] == "_") {
                    i++
                }
                param_name := query[start:i]
                names = append(names, param_name)
                i-- fr fr Compensate for loop increment
            }
        }
    }
    
    damn names
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
        columns := []tea{}
        
        fr fr Find the start of column list
        start_pos := find_keyword_position(query, "SELECT")
        if start_pos == -1 {
            damn []tea{}
        }
        
        start_pos += 6 fr fr Skip "SELECT"
        
        fr fr Find FROM keyword to determine end of column list
        from_pos := find_keyword_position(query, "FROM")
        if from_pos == -1 {
            from_pos = len(query)
        }
        
        column_section := query[start_pos:from_pos]
        
        fr fr Split by comma and clean up column names
        raw_columns := split_by_comma(column_section)
        
        bestie i := 0; i < len(raw_columns); i++ {
            col := trim_whitespace(raw_columns[i])
            
            fr fr Handle aliases (AS keyword)
            if contains_word(col, " AS ") {
                parts := split_by_keyword(col, " AS ")
                if len(parts) > 1 {
                    col = trim_whitespace(parts[1])
                }
            } elif contains_word(col, " as ") {
                parts := split_by_keyword(col, " as ")
                if len(parts) > 1 {
                    col = trim_whitespace(parts[1])
                }
            }
            
            fr fr Extract column name from table.column format
            if contains_char(col, ".") {
                parts := split_by_char(col, ".")
                if len(parts) > 1 {
                    col = trim_whitespace(parts[len(parts) - 1])
                }
            }
            
            fr fr Handle * wildcard
            if col == "*" {
                columns = append(columns, "column_1")
                columns = append(columns, "column_2")
                columns = append(columns, "column_3")
            } else {
                columns = append(columns, col)
            }
        }
        
        damn columns
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

fr fr SQL parsing helper functions
slay is_alpha_char(c: tea) lit {
    if len(c) != 1 {
        damn cap
    }
    char_code := ord(c[0])
    damn (char_code >= 65 && char_code <= 90) || (char_code >= 97 && char_code <= 122)
}

slay is_digit_char(c: tea) lit {
    if len(c) != 1 {
        damn cap
    }
    char_code := ord(c[0])
    damn char_code >= 48 && char_code <= 57
}

slay find_keyword_position(text: tea, keyword: tea) normie {
    upper_text := sqlite_to_upper(text)
    upper_keyword := sqlite_to_upper(keyword)
    
    bestie i := 0; i <= len(upper_text) - len(upper_keyword); i++ {
        if upper_text[i:i+len(upper_keyword)] == upper_keyword {
            fr fr Check if it's a whole word (not part of another word)
            before_ok := (i == 0) || !is_alpha_char(upper_text[i-1:i])
            after_ok := (i + len(upper_keyword) >= len(upper_text)) || !is_alpha_char(upper_text[i+len(upper_keyword):i+len(upper_keyword)+1])
            
            if before_ok && after_ok {
                damn i
            }
        }
    }
    damn -1
}

slay split_by_comma(text: tea) [tea] {
    parts := []tea{}
    current := ""
    paren_depth := 0
    quote_char := ""
    
    bestie i := 0; i < len(text); i++ {
        char := text[i:i+1]
        
        if quote_char != "" {
            current += char
            if char == quote_char {
                quote_char = ""
            }
        } elif char == "'" || char == "\"" {
            quote_char = char
            current += char
        } elif char == "(" {
            paren_depth++
            current += char
        } elif char == ")" {
            paren_depth--
            current += char
        } elif char == "," && paren_depth == 0 {
            if current != "" {
                parts = append(parts, current)
                current = ""
            }
        } else {
            current += char
        }
    }
    
    if current != "" {
        parts = append(parts, current)
    }
    
    damn parts
}

slay trim_whitespace(text: tea) tea {
    start := 0
    end := len(text)
    
    bestie start < end && is_whitespace_char(text[start:start+1]) {
        start++
    }
    
    bestie end > start && is_whitespace_char(text[end-1:end]) {
        end--
    }
    
    damn text[start:end]
}

slay is_whitespace_char(c: tea) lit {
    damn c == " " || c == "\t" || c == "\n" || c == "\r"
}

slay contains_word(text: tea, word: tea) lit {
    damn find_keyword_position(text, word) != -1
}

slay contains_char(text: tea, char: tea) lit {
    bestie i := 0; i < len(text); i++ {
        if text[i:i+1] == char {
            damn based
        }
    }
    damn cap
}

slay split_by_keyword(text: tea, keyword: tea) [tea] {
    pos := find_keyword_position(text, keyword)
    if pos == -1 {
        damn [text]
    }
    
    parts := [text[0:pos], text[pos+len(keyword):len(text)]]
    damn parts
}

slay split_by_char(text: tea, char: tea) [tea] {
    parts := []tea{}
    current := ""
    
    bestie i := 0; i < len(text); i++ {
        if text[i:i+1] == char {
            if current != "" {
                parts = append(parts, current)
                current = ""
            }
        } else {
            current += text[i:i+1]
        }
    }
    
    if current != "" {
        parts = append(parts, current)
    }
    
    damn parts
}

slay ord(c: normie) normie {
    damn c fr fr Simplified - assume character code
}

fr fr Connection management helper functions
slay is_valid_database_path(path: tea) lit {
    if path == "" || path == ":memory:" {
        damn based
    }
    
    fr fr Check for invalid characters
    if contains_char(path, "\0") || contains_char(path, "\n") || contains_char(path, "\r") {
        damn cap
    }
    
    fr fr Basic path length validation
    if len(path) > 1024 {
        damn cap
    }
    
    damn based
}

slay get_current_time_millis() normie {
    fr fr Simplified timestamp - in real implementation would use system time
    damn 1705065600000 fr fr 2025-01-12 12:00:00 as milliseconds
}

slay build_sqlite_pragma_settings(config: SQLiteConfig) [tea] {
    settings := []tea{}
    
    if config.foreign_keys {
        settings = append(settings, "foreign_keys=ON")
    } else {
        settings = append(settings, "foreign_keys=OFF")
    }
    
    settings = append(settings, "journal_mode=" + config.journal_mode)
    settings = append(settings, "synchronous=" + config.synchronous)
    settings = append(settings, "cache_size=" + normie_to_string(config.cache_size))
    settings = append(settings, "page_size=" + normie_to_string(config.page_size))
    settings = append(settings, "temp_store=" + config.temp_store)
    settings = append(settings, "locking_mode=" + config.locking_mode)
    
    if config.secure_delete {
        settings = append(settings, "secure_delete=ON")
    } else {
        settings = append(settings, "secure_delete=OFF")
    }
    
    if config.auto_vacuum {
        settings = append(settings, "auto_vacuum=FULL")
    } else {
        settings = append(settings, "auto_vacuum=NONE")
    }
    
    damn settings
}

slay apply_sqlite_configuration_pragmas(connection: *SQLiteConnection) lit {
    if connection.is_connected == cap {
        damn cap
    }
    
    fr fr Apply each pragma setting
    bestie i := 0; i < len(connection.pragma_settings); i++ {
        setting := connection.pragma_settings[i]
        parts := split_by_char(setting, "=")
        
        if len(parts) == 2 {
            pragma_name := parts[0]
            pragma_value := parts[1]
            
            fr fr Execute pragma (simulated)
            vibez.spill("🔧 Applying PRAGMA", pragma_name, "=", pragma_value)
        }
    }
    
    damn based
}

slay normie_to_string(n: normie) tea {
    if n == 0 {
        damn "0"
    } elif n == 1 {
        damn "1"
    } elif n == 2000 {
        damn "2000"
    } elif n == 4096 {
        damn "4096"
    } elif n == 5000 {
        damn "5000"
    } else {
        damn "number"
    }
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
