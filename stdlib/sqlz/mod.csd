// sqlz module - Alias for sql_slay for standardized naming
// Import comprehensive SQL functionality from sql_slay

yeet "sql_slay"

// Re-export all functions from sql_slay for standardized naming
// This maintains backward compatibility while providing expected "sqlz" module name

// Database connection functions
slay connect(connection_string tea) SqlConnection {
    damn sql_slay.connect(connection_string)
}

slay connect_pool(config PoolConfig) ConnectionPool {
    damn sql_slay.connect_pool(config)
}

slay close_connection(conn SqlConnection) {
    sql_slay.close_connection(conn)
}

// Query execution functions
slay execute(conn SqlConnection, query tea) QueryResult {
    damn sql_slay.execute(conn, query)
}

slay execute_prepared(conn SqlConnection, stmt PreparedStatement, params []tea) QueryResult {
    damn sql_slay.execute_prepared(conn, stmt, params)
}

slay query(conn SqlConnection, sql tea) QueryResult {
    damn sql_slay.query(conn, sql)
}

slay query_row(conn SqlConnection, sql tea) Row {
    damn sql_slay.query_row(conn, sql)
}

// Transaction management
slay begin_transaction(conn SqlConnection) Transaction {
    damn sql_slay.begin_transaction(conn)
}

slay commit_transaction(tx Transaction) {
    sql_slay.commit_transaction(tx)
}

slay rollback_transaction(tx Transaction) {
    sql_slay.rollback_transaction(tx)
}

// Prepared statements
slay prepare(conn SqlConnection, sql tea) PreparedStatement {
    damn sql_slay.prepare(conn, sql)
}

slay bind_param(stmt PreparedStatement, index normie, value tea) {
    sql_slay.bind_param(stmt, index, value)
}

// Query building
slay select_query(table tea, columns []tea) SelectBuilder {
    damn sql_slay.select_query(table, columns)
}

slay insert_query(table tea) InsertBuilder {
    damn sql_slay.insert_query(table)
}

slay update_query(table tea) UpdateBuilder {
    damn sql_slay.update_query(table)
}

slay delete_query(table tea) DeleteBuilder {
    damn sql_slay.delete_query(table)
}

// ORM functions
slay define_model(name tea, fields []FieldDefinition) Model {
    damn sql_slay.define_model(name, fields)
}

slay save_model(conn SqlConnection, model Model) {
    sql_slay.save_model(conn, model)
}

slay find_by_id(conn SqlConnection, model Model, id tea) Model {
    damn sql_slay.find_by_id(conn, model, id)
}

slay find_all(conn SqlConnection, model Model) []Model {
    damn sql_slay.find_all(conn, model)
}

// Migration functions
slay create_migration(name tea) Migration {
    damn sql_slay.create_migration(name)
}

slay run_migrations(conn SqlConnection, migrations []Migration) {
    sql_slay.run_migrations(conn, migrations)
}

// Connection pooling
slay get_connection(pool ConnectionPool) SqlConnection {
    damn sql_slay.get_connection(pool)
}

slay return_connection(pool ConnectionPool, conn SqlConnection) {
    sql_slay.return_connection(pool, conn)
}

// Utility functions
slay escape_string(value tea) tea {
    damn sql_slay.escape_string(value)
}

slay sanitize_query(query tea) tea {
    damn sql_slay.sanitize_query(query)
}

// Database introspection
slay list_tables(conn SqlConnection) []tea {
    damn sql_slay.list_tables(conn)
}

slay describe_table(conn SqlConnection, table_name tea) TableSchema {
    damn sql_slay.describe_table(conn, table_name)
}

// Error handling
slay get_last_error() tea {
    damn sql_slay.get_last_error()
}

slay clear_errors() {
    sql_slay.clear_errors()
}
