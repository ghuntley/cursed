squad DatabaseConnection {
    host: String,
    port: Int,
    username: String,
    password: String,
}

damn connect(host: String, port: Int) -> DatabaseConnection {
    DatabaseConnection {
        host: host,
        port: port,
        username: "admin",
        password: "secret",
    }
}

damn slay query(self, sql: String) -> QueryResult {
    QueryResult { rows: [], affected: 0 }
}

squad QueryResult {
    rows: String[],
    affected: Int,
}

collab Queryable {
    damn execute(self, query: String) -> QueryResult
}

damn slay execute_transaction(self, queries: String[]) -> Bool {
    true
}

squad ConnectionPool {
    max_connections: Int,
    active_connections: Int,
}

damn new ConnectionPool(max_size: Int) -> ConnectionPool {
    ConnectionPool {
        max_connections: max_size,
        active_connections: 0,
    }
}
