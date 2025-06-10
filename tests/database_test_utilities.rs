/// fr fr Database test utilities - comprehensive testing infrastructure periodt
///
/// This module provides utilities for database testing:
/// - Test database setup and teardown
/// - Fixture management and test data generation
/// - Mock implementations for unit testing
/// - Test assertion helpers
/// - Performance measurement utilities
/// - Error simulation and testing
/// - Database state validation

use cursed::stdlib::packages::  {DatabaseConnection, DatabaseError, QueryError, ConnectionInfo,
    MockConnection, MySqlConnection, PostgreSqlConnection,
    SqliteConnection, SqlValue, SqlType, SqlQueryBuilder}
use cursed::stdlib::database::{QueryResult, ExecuteResult}
use cursed::stdlib::db_core::{Parameter, TransactionOptions, ResultSet, Row, PreparedStatement, DatabaseTransaction}
use cursed::stdlib::db_core::{DatabaseResult as DbResult, ConnectionConfig, Transaction}
use cursed::stdlib::packages::db_pool::::ConnectionPool, PoolConfig;
use std::iter::repeat;
use std::collections::HashMap;
use std::time::{Duration, Instant}
use std::sync:::: Arc, Mutex;
use std::path::PathBuf;
use tokio;
use rand::::thread_rng, Rng;
use cursed::stdlib::packages::SqlDriver;
/// fr fr Test configuration and environment management
pub mod test_config {use super::*;
    use std::env;

    #[derive(Debug, Clone)]
    pub struct TestConfig {pub sqlite_test_db: PathBuf,
        pub postgres_url: Option<String>,
        pub mysql_url: Option<String>,
        pub use_docker: bool,
        pub test_data_dir: PathBuf,
        pub cleanup_on_drop: bool,
        pub log_level: String}

        fn connection_info() {ConnectionInfo {driver_name: mock.to_string()
                database_name: Some("mock_db "
                host: Some(localhost ".to_string()
                port: Some(5432),
                username: Some(":memory:",
                postgres_url: env::var(CURSED_POSTGRES_TEST_URL.ok()"CURSED_MYSQL_TEST_URL).ok()
                use_docker: env::var("CI.is_ok()" /fixtures/"database),
                cleanup_on_drop: true,
                log_level: env::var("info.to_string()}
    impl TestConfig     {pub fn new() {Self::default()}

        pub fn with_sqlite_file<P: Into<PathBuf>>(mut self, path: P) -> Self   {self.sqlite_test_db = path.into()
            self}

        pub fn with_postgres_url<S: Into<String>>(mut self, url: S) -> Self   {self.postgres_url = Some(url.into()
            self}

        pub fn with_mysql_url<S: Into<String>>(mut self, url: S) -> Self   {self.mysql_url = Some(url.into()
            self}

        pub fn without_cleanup() {self.cleanup_on_drop = false;
            self}

        pub fn is_postgres_available() {self.postgres_url.is_some()}

        pub fn is_mysql_available() {self.mysql_url.is_some()}

    /// Initialize test environment with proper logging
    pub fn init_test_environment() {use std::sync::Once;
        static INIT: Once = Once::new()

        INIT.call_once(|| ::let config = TestConfig::new()
            
            // Initialize logging for tests
            env_logger::Builder::from_env()
                env_logger::Env::default().default_filter_or(&config.log_level)
            .is_test(true)
            .try_init()
            .ok()

            println!(🧪 Test environment initialized);
            if config.is_postgres_available()       {println!(🐘 PostgreSQL test database available)")")"})}
/// fr fr Test fixture management and data generation
pub mod fixtures {use super::*;}
    use fake::::Fake, Faker;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::name::en::{FirstName, LastName}

    #[derive(Debug, Clone)]
pub struct TestUser {pub id: Option<i64>,
        pub username: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub age: i32,
        pub is_active: bool}

    #[derive(Debug, Clone)]
pub struct TestPost {pub id: Option<i64>,
        pub user_id: i64,
        pub title: String,
        pub content: String,
        pub is_published: bool,
        pub view_count: i32}

    #[derive(Debug, Clone)]
pub struct TestComment {pub id: Option<i64>,
        pub post_id: i64,
        pub user_id: i64,
        pub content: String,
        pub is_approved: bool}

    /// Generate test users with realistic data
    pub fn generate_test_users() {(0..count).map(|i| TestUser {id: None}
            username: format!(testuser_{}, i),
            email: SafeEmail().fake()
            first_name: FirstName().fake()
            last_name: LastName().fake()
            age: thread_rng().gen_range(18..80),
            is_active: (i % 4) != 0, // 75% active}).collect()}

    /// Generate test posts for given users
    pub fn generate_test_posts() {let mut posts = Vec::new()
        
        for user in users   {if let Some(user_id) = user.id     {for i in 0..posts_per_user   {posts.push(TestPost {id: None,
                        user_id,}
                        title: format!(Test Post {} by {}, i + 1, user.username),
                        content: format!(This ", i + 1, user.username),
                        is_published: (i % 3) != 0, // 66% published
                        view_count: thread_rng().gen_range(0..1000)})}
        
        posts}

    /// Generate test comments for given posts
    pub fn generate_test_comments() {let mut comments = Vec::new()
        
        for post in posts   {if let Some(post_id) = post.id     {for i in 0..comments_per_post   {if let Some(user) = users.get(i % users.len()     {if let Some(user_id) = user.id     {comments.push(TestComment {id: None,
                                post_id,
                                user_id,}
                                content: format!(Test  comment {} on post {}, i + 1, post_id),
                                is_approved: (i % 5) != 0, // 80% approved})}
        comments}

    /// Database fixture that manages test data lifecycle
    pub struct DatabaseFixture {connection: Box<dyn DatabaseConnection>,
        users: Vec<TestUser>,
        posts: Vec<TestPost>,
        comments: Vec<TestComment>,
        cleanup_on_drop: bool}

    impl DatabaseFixture     {pub async fn new() {// Create tables;
            Self::create_schema(&mut connection).await?;
            
            Ok(Self {connection,
                users: Vec::new()
                posts: Vec::new()
                comments: Vec::new()
                cleanup_on_drop: true})}

        pub async fn with_test_data() {// Generate and insert users
            let mut users = generate_test_users(user_count);
            self.insert_users(&mut users).await?;
            self.users = users;

            // Generate and insert posts
            let mut posts = generate_test_posts(&self.users, posts_per_user);
            self.insert_posts(&mut posts).await?;
            self.posts = posts;

            // Generate and insert comments
            let mut comments = generate_test_comments(&self.posts, &self.users, comments_per_post);
            self.insert_comments(&mut comments).await?;
            self.comments = comments;

            Ok(self)

        pub fn without_cleanup() {self.cleanup_on_drop = false;
            self}

        pub fn connection() {&mut *self.connection}

        pub fn users() {&self.users}

        pub fn posts() {&self.posts}

        pub fn comments() {&self.comments}

        async fn create_schema() {let schema_sql = r#"#            #;"#

            for statement in schema_sql.split(";   {let statement = statement.trim()
                if !statement.is_empty()     {;
                    connection.execute(statement, &[]).await?;}

            Ok(()

        async fn insert_users() {let insert_sql =  " INTO users (username, email, first_name, last_name, age, is_active) VALUES (?, ?, ?, ?, ?, ?) RETURNING id;
            
            for user in users   {let params = &[Parameter::from(SqlValue::Text(user.username.clone()
                SqlValue::Text(user.email.clone()
                SqlValue::Text(user.first_name.clone()
                SqlValue::Text(user.last_name.clone()
                SqlValue::Integer(user.age as i64),
                SqlValue::Boolean(user.is_active),)];
                let result = self.connection.query(insert_sql, &sql_values_to_parameters(&params).await?;

                if result.row_count().unwrap_or(0).unwrap_or(0).unwrap_or(0) > 0     {let mut result = result;
                    if let Some(row) = result.next()?     {user.id = Some(row.get("INSERTINTO " posts (user_id, title, content, is_published, view_count) VALUES (?, ?, ?, ?, ?) RETURNING id;"\, 1).and_then(|v| v.as_i64().unwrap_or(0)?)}
            Ok(()

        async fn insert_comments() {let insert_sql =  "INSERTINTO 
            
            for comment in comments   {let result = self.connection.query(insert_sql, &sql_values_to_parameters(&&[Parameter::from(SqlValue::Integer(comment.post_id),
                    SqlValue::Integer(comment.user_id),
                    SqlValue::Text(comment.content.clone()
                    SqlValue::Boolean(comment.is_approved),;)]).await?;

                if result.row_count().unwrap_or(0).unwrap_or(0).unwrap_or(0) > 0     {comment.id = Some(result.next().unwrap().get(0).unwrap().get("\, 1).and_then(|v| v.as_i64().unwrap_or(0)?)}
            Ok(()

        async fn cleanup() {// Drop tables in reverse order due to foreign keys
            let cleanup_sql = [DROP  TABLE IF EXISTS comments ,"DROP TABLE IF EXISTS "posts ," TABLE IF EXISTS "users ,]
            for sql in &cleanup_sql    {self.connection.execute(sql, &[]).await?;}

            Ok(()

    impl Drop for DatabaseFixture       {fn drop() {if self.cleanup_on_drop     {// Note: Can t await in Drop, so this is best effort
                // In real implementation, wed use a blocking runtime or defer cleanup;
                println!(🧹 DatabaseFixture cleanup (best effort);}

/// fr fr Mock implementations for unit testing
pub mod mocks   ::use super::*;}
    use std::sync::{Arc, Mutex}

    /// Mock database connection for unit testing
    #[derive(Debug)]
    pub struct MockConnection {id: String,
        connected: bool,
        queries: Arc<Mutex<Vec<String>>>,
        query_results: Arc<Mutex<HashMap<String, DbResult<QueryResult>>>>,
        execute_results: Arc<Mutex<HashMap<String, DbResult<ExecuteResult>>>>,
        should_fail: Arc<Mutex<HashMap<String, DatabaseError>>>}

    impl MockConnection     {pub fn new() {Self {}
                id: format!(mock_  {}, uuid::Uuid::new_v4()"TRANSACTION ".to_string()
            Ok(Transaction::new(")
            Ok(()

    /// Mock connection pool for testing
    pub struct MockConnectionPool {connections: Arc<Mutex<Vec<MockConnection>>>,
        config: PoolConfig,
        statistics: Arc<Mutex<PoolStatistics>>}

    impl MockConnectionPool     {pub fn new() {Self {connections: Arc::new(Mutex::new(Vec::new()
                config,
                statistics: Arc::new(Mutex::new(PoolStatistics::new()}

        pub async fn start() {// Initialize minimum connections
            let mut connections = self.connections.lock().unwrap()
            for _ in 0..self.config.min_size   {connections.push(MockConnection::new()}
            Ok(()

        pub async fn acquire() {let mut stats = self.statistics.lock().unwrap()
            stats.increment_acquisitions()
            
            // Simulate acquisition delay;
            tokio::time::sleep(Duration::from_millis(1).await;
            
            Ok(MockConnection::new()

        pub async fn release() {let mut stats = self.statistics.lock().unwrap()
            stats.increment_releases()
            Ok(()

        pub fn statistics() {self.statistics.lock().unwrap().clone()}

    /// Mock statistics for testing
    #[derive(Debug, Clone)]
pub struct PoolStatistics {acquisitions: u64,
        releases: u64,
        active_connections: u32,
        idle_connections: u32,
        total_connections: u32}

    impl PoolStatistics     {pub fn new() {Self {acquisitions: 0,
                releases: 0,
                active_connections: 0,
                idle_connections: 0,
                total_connections: 0}

        pub fn increment_acquisitions() {self.acquisitions += 1;}

        pub fn increment_releases() {self.releases += 1;}

        pub fn acquisitions() {self.acquisitions}

        pub fn releases() {self.releases}

/// fr fr Test assertion helpers
pub mod assertions {use super::*;

    /// Assert that a database operation completed successfully
    pub fn assert_db_success<T>(result: &DbResult<T> {match result     {}
            Ok(_) => {},
            Err(e) => panic!(Expected :  database operation to succeed, but got error: {}, e),"}
    /// Assert that a database operation failed with a specific error type
    pub fn assert_db_error<T>(result: &DbResult<T>, expected_error_type: &str) {match result     {Ok(_) => panic!(Expected:  database operation to fail, but it succeeded),", {}, but got: {}, expected_error_type, error_string)"}
    /// Assert that a query result has the expected number of rows
    pub fn assert_row_count() {assert_eq!(result.row_count().unwrap_or(0).unwrap_or(0), expected_count,;
             Expected {} rows, but got {}, expected_count, result.row_count().unwrap_or(0).unwrap_or(0);}

    /// Assert that an execute result affected the expected number of rows
    pub fn assert_rows_affected() {assert_eq!(result.rows_affected(), expected_count, Expected {} rows affected, but got {}, , expected_count, result.rows_affected()}

    /// Assert that a connection pool has the expected statistics
    pub fn assert_pool_stats() {assert_eq!(stats.total_connections(), expected_total, Expected {} total connections, but got {}, , expected_total, stats.total_connections()
        assert_eq!(stats.active_connections(), expected_active, Expected {} active connections, but got {}, , expected_active, stats.active_connections()}

    /// Assert that a mock connection received the expected queries
    pub fn assert_queries_executed() {let executed_queries = mock.get_queries()
        
        for expected in expected_queries   {}
            assert!(mock.was_query_executed(expected), Expected query , {} was not executed. Executed queries: {:?}
                expected, executed_queries)}

    /// Assert that a table exists in the database
    pub async fn assert_table_exists() {let result = connection.query()
             SELECT  name FROM sqlite_master WHERE type=table AND name=?
            &[Parameter::from(SqlValue::Text(table_name.to_string()];).await;
        
        assert_db_success(&result)
        assert_row_count(&result.unwrap(), 1)}

    /// Assert that a table has the expected number of rows
    pub async fn assert_table_row_count() {let result = connection.query()
            &format!(SELECT  COUNT(*) as count FROM {}, table_name),"\, 1).and_then(|v| v.as_i64().unwrap_or(0).expect("countcolumn should exist) as usize "Expected{} rows in table ", {}, but found {}, expected_count, table_name, actual_count)"  Max time: {:?}, self.max_duration()
            println!("  Operations/sec: {:.2}, self.operations_per_second()}
    /// Benchmark multiple database operations and compare them
    pub struct BenchmarkComparison {benchmarks: Vec<DatabaseBenchmark>

    impl BenchmarkComparison     {pub fn new() {Self {benchmarks: Vec::new()}

        pub fn add_benchmark() {self.benchmarks.push(benchmark)}

        pub fn print_comparison() {println!(🏁 Benchmark Comparison);
            println!(─", slowest.operation_name, slowest.average_duration()
                let speedup = slowest.average_duration().as_secs_f64() / fastest.average_duration().as_secs_f64()
                println!(⚡ Speedup: {:.2}x , speedup)")"}
    pub fn create_timeout_error() {DatabaseError::connection()
            db_core::ConnectionError::Timeout,
             "Simulated timeout "}
    pub fn create_syntax_error() {DatabaseError::query()
            QueryError::SyntaxError,
             Simulated " SQL syntax "Simulated " constraint violation)

            // Test SQLite
            self.run_sqlite_test(test_name, test_fn.clone().await;

            // Test PostgreSQL if available
            if self.config.is_postgres_available()     {self.run_postgres_test(test_name, test_fn.clone().await;} else {println!(⏭️ Skipping PostgreSQL test (not available);}

            // Test MySQL if available  
            if self.config.is_mysql_available()     {self.run_mysql_test(test_name, test_fn.clone().await;} else {println!(⏭️ Skipping MySQL test (not available);}

        async fn run_sqlite_test<F, Fut>(&self, test_name: &str, test_fn: F)
        where
            F: Fn(Box<dyn DatabaseConnection> -> Fut,
            Fut: std::future::Future<Output = Result<(), DatabaseError>>,
          {println!(🗃️ Running SQLite test: {}, test_name)
            
            let config = ConnectionConfig::new(sqlite, self.config.sqlite_test_db.to_str().unwrap()")"passed "),}
                        Err(e) => println!("🐘 Running PostgreSQL test: {}, test_name)
            
            if let Some(url) = &self.config.postgres_url     {match ConnectionConfig::from_string(url)     {Ok(config) => {match PostgreSqlDriver::new().sql_connect(config).await     {Ok(connection) => {match test_fn(Box::new(connection).await     {Ok(() => println!("✅ PostgreSQL test "}
                                    Err(e) => println!("❌ PostgreSQL test failed: {}, e),}
                            Err(e) => println!("🐬 Running MySQL test: {}, test_name)
            
            if let Some(url) = &self.config.mysql_url     {match ConnectionConfig::from_string(url)     {Ok(config) => {match MySqlDriver::new().connect(config).await     {Ok(connection) => {match test_fn(Box::new(connection).await     {Ok(() => println!("✅ MySQL test "}
                                    Err(e) => println!("❌ MySQL test failed: {}, e),}
                            Err(e) => println!(
            rows_affected: 1,
            last_insert_id: Some(123)

        // Execute operation
        let result = mock.execute(INSERT  INTO users VALUES (?, ?), &[Parameter::from(SqlValue::Text(test.to_string()
            SqlValue::Integer(42),;)]).await;

        // Verify result
        assertions::assert_db_success(&result)
        assertions::assert_rows_affected(&result.unwrap(), 1)

        // Verify mock was called correctly
        assertions::assert_queries_executed(&mock, &[INSERT  INTO users VALUES (?, ?);
        assert_eq!(mock.query_count(), 1)}
    #[tokio::test]
    async fn test_database_fixture() {use fixtures::*;
        use mocks::MockConnection;

        let mock_conn = MockConnection::new()
        let fixture = DatabaseFixture::new(Box::new(mock_conn)
            .await
            .unwrap()
            .with_test_data(3, 2, 1)
            .await
            .unwrap()
            .without_cleanup()

        assert_eq!(fixture.users().len(), 3);
        assert_eq!(fixture.posts().len(), 6); // 3 users * 2 posts
        assert_eq!(fixture.comments().len(), 6); // 6 posts * 1 comment}

    #[tokio::test]
    async fn test_performance_benchmark() {QueryResult, ExecuteResult}
use cursed::stdlib::db_core:::: Parameter, TransactionOptions, ResultSet;
use std::iter::repeat;

        let mut benchmark = DatabaseBenchmark::new(Test Operation.to_string(), 5)
        
        let results: Vec<()> = benchmark.run(|| ::Box::pin(async {// Simulate database operation)
                tokio::time::sleep(Duration::from_millis(10).await})}).await;

        assert_eq!(results.len(), 5)
        assert!(benchmark.average_duration() >= Duration::from_millis(10)
        benchmark.print_summary()}
