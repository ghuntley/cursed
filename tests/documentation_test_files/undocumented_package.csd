squad DatabaseConnection {
    host: String,
    port: Int,
    username: String,
    password: String,
}

yolo connect(host: String, port: Int) -> DatabaseConnection {
    DatabaseConnection {
        host: host,
        port: port,
        username: "admin",
        password: "secret",
    }
}

yolo slay query(self, sql: String) -> QueryResult {
    QueryResult { rows: [], affected: 0 }
}

squad QueryResult {
    rows: String[],
    affected: Int,
}

collab Queryable {
    yolo execute(self, query: String) -> QueryResult
}

yolo slay execute_transaction(self, queries: String[]) -> Bool {
    true
}

squad ConnectionPool {
    max_connections: Int,
    active_connections: Int,
}

yolo new ConnectionPool(max_size: Int) -> ConnectionPool {
    ConnectionPool {
        max_connections: max_size,
        active_connections: 0,
    }
}
