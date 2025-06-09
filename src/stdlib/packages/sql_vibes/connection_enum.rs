/// fr fr Database connection enum - supports different database types periodt
use crate::stdlib::packages::sql_vibes::{
    drivers::{
        sqlite::{SqliteConnection, SqlitePreparedStatement, SqliteTransaction},
        postgres::{PostgresConnection, PostgresPreparedStatement, PostgresTransaction},
        mysql::{MySqlConnection, MySqlPreparedStatement, MySqlTransaction},
        mock::{MockConnection, MockPreparedStatement, MockTransaction},
    },
    DatabaseConnection as DatabaseConnectionTrait,
    PreparedStatement as PreparedStatementTrait,
    Transaction as TransactionTrait,
    SqlResult, SqlError, Parameter, ResultSet
};

/// fr fr Database connection enum - polymorphic connection support
#[derive(Debug)]
pub enum DatabaseConnection {
    /// SQLite connection
    Sqlite(SqliteConnection),
    
    /// PostgreSQL connection
    Postgres(PostgresConnection),
    
    /// MySQL connection
    MySql(MySqlConnection),
    
    /// Mock connection for testing
    Mock(MockConnection),
}

impl DatabaseConnectionTrait for DatabaseConnection {
    fn execute_query(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<ResultSet> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.execute_query(sql, params),
            DatabaseConnection::Postgres(conn) => conn.execute_query(sql, params),
            DatabaseConnection::MySql(conn) => conn.execute_query(sql, params),
            DatabaseConnection::Mock(conn) => conn.execute_query(sql, params),
        }
    }
    
    fn execute_statement(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<u64> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.execute_statement(sql, params),
            DatabaseConnection::Postgres(conn) => conn.execute_statement(sql, params),
            DatabaseConnection::MySql(conn) => conn.execute_statement(sql, params),
            DatabaseConnection::Mock(conn) => conn.execute_statement(sql, params),
        }
    }
    
    fn prepare_statement(&mut self, sql: &str) -> SqlResult<PreparedStatement> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.prepare_statement(sql),
            DatabaseConnection::Postgres(conn) => conn.prepare_statement(sql),
            DatabaseConnection::MySql(conn) => conn.prepare_statement(sql),
            DatabaseConnection::Mock(conn) => conn.prepare_statement(sql),
        }
    }
    
    fn begin_transaction(&mut self) -> SqlResult<Box<dyn TransactionTrait>> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.begin_transaction(),
            DatabaseConnection::Postgres(conn) => conn.begin_transaction(),
            DatabaseConnection::MySql(conn) => conn.begin_transaction(),
            DatabaseConnection::Mock(conn) => conn.begin_transaction(),
        }
    }
    
    fn is_alive(&self) -> bool {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.is_alive(),
            DatabaseConnection::Postgres(conn) => conn.is_alive(),
            DatabaseConnection::MySql(conn) => conn.is_alive(),
            DatabaseConnection::Mock(conn) => conn.is_alive(),
        }
    }
    
    fn close(&mut self) -> SqlResult<()> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.close(),
            DatabaseConnection::Postgres(conn) => conn.close(),
            DatabaseConnection::MySql(conn) => conn.close(),
            DatabaseConnection::Mock(conn) => conn.close(),
        }
    }
    
    fn connection_info(&self) -> crate::stdlib::packages::sql_vibes::ConnectionInfo {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.connection_info(),
            DatabaseConnection::Postgres(conn) => conn.connection_info(),
            DatabaseConnection::MySql(conn) => conn.connection_info(),
            DatabaseConnection::Mock(conn) => conn.connection_info(),
        }
    }
    
    fn execute_batch(&mut self, statements: &[(&str, &[Parameter])]) -> SqlResult<Vec<SqlResult<u64>>> {
        match self {
            DatabaseConnection::Sqlite(conn) => conn.execute_batch(statements),
            DatabaseConnection::Postgres(conn) => conn.execute_batch(statements),
            DatabaseConnection::MySql(conn) => conn.execute_batch(statements),
            DatabaseConnection::Mock(conn) => conn.execute_batch(statements),
        }
    }
}

/// fr fr Prepared statement enum - polymorphic prepared statement support
#[derive(Debug)]
pub enum PreparedStatement {
    /// SQLite prepared statement
    Sqlite(SqlitePreparedStatement),
    
    /// PostgreSQL prepared statement
    Postgres(PostgresPreparedStatement),
    
    /// MySQL prepared statement
    MySql(MySqlPreparedStatement),
    
    /// Mock prepared statement
    Mock(MockPreparedStatement),
}

impl PreparedStatementTrait for PreparedStatement {
    fn execute(&mut self, params: &[Parameter]) -> SqlResult<ResultSet> {
        match self {
            PreparedStatement::Sqlite(stmt) => stmt.execute(params),
            PreparedStatement::Postgres(stmt) => stmt.execute(params),
            PreparedStatement::MySql(stmt) => stmt.execute(params),
            PreparedStatement::Mock(stmt) => stmt.execute(params),
        }
    }
    
    fn execute_update(&mut self, params: &[Parameter]) -> SqlResult<u64> {
        match self {
            PreparedStatement::Sqlite(stmt) => stmt.execute_update(params),
            PreparedStatement::Postgres(stmt) => stmt.execute_update(params),
            PreparedStatement::MySql(stmt) => stmt.execute_update(params),
            PreparedStatement::Mock(stmt) => stmt.execute_update(params),
        }
    }
    
    fn sql(&self) -> &str {
        match self {
            PreparedStatement::Sqlite(stmt) => stmt.sql(),
            PreparedStatement::Postgres(stmt) => stmt.sql(),
            PreparedStatement::MySql(stmt) => stmt.sql(),
            PreparedStatement::Mock(stmt) => stmt.sql(),
        }
    }
    
    fn parameter_count(&self) -> usize {
        match self {
            PreparedStatement::Sqlite(stmt) => stmt.parameter_count(),
            PreparedStatement::Postgres(stmt) => stmt.parameter_count(),
            PreparedStatement::MySql(stmt) => stmt.parameter_count(),
            PreparedStatement::Mock(stmt) => stmt.parameter_count(),
        }
    }
    
    fn close(&mut self) -> SqlResult<()> {
        match self {
            PreparedStatement::Sqlite(stmt) => stmt.close(),
            PreparedStatement::Postgres(stmt) => stmt.close(),
            PreparedStatement::MySql(stmt) => stmt.close(),
            PreparedStatement::Mock(stmt) => stmt.close(),
        }
    }
}
