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

use cursed::stdlib::packages::{
    DatabaseConnection, DatabaseError, QueryError, ConnectionInfo,
    MockConnection, MySqlConnection, PostgreSqlConnection,
    SqliteConnection, SqlValue, SqlType, SqlQueryBuilder
}
use cursed::stdlib::database::{QueryResult, ExecuteResult}
use cursed::stdlib::db_core::{Parameter, TransactionOptions, ResultSet, Row, PreparedStatement, DatabaseTransaction}
use cursed::stdlib::db_core::{DatabaseResult as DbResult, ConnectionConfig, Transaction}
use cursed::stdlib::packages::db_pool::{ConnectionPool, PoolConfig};
use std::iter::repeat;
use std::collections::HashMap;
use std::time::{Duration, Instant}
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use tokio;
use rand::{thread_rng, Rng}
;
use cursed::stdlib::packages::SqlDriver;
/// fr fr Test configuration and environment management
pub mod test_config {
    use super::*;
    use std::env;

    #[derive(Debug, Clone)]
    pub struct TestConfig {
        pub sqlite_test_db: PathBuf,
        pub postgres_url: Option<String>,
        pub mysql_url: Option<String>,
        pub use_docker: bool,
        pub test_data_dir: PathBuf,
        pub cleanup_on_drop: bool,
        pub log_level: String,}
    }

        fn connection_info(&self) -> ConnectionInfo {
            ConnectionInfo {
                driver_name: "mock.to_string()"
                database_name: Some( "mock_db ".to_string()"
                host: Some( localhost ".to_string()
                port: Some(5432),
                username: Some( "mock_user.to_string()
                ssl_mode: None,
                connection_timeout: Some(30),
                query_timeout: Some(30),
                max_connections: Some(1),
                is_pooled: false,}
            }
        }
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                sqlite_test_db: PathBuf::from(":memory:",
                postgres_url: env::var( CURSED_POSTGRES_TEST_URL.ok()"
                mysql_url: env::var( "CURSED_MYSQL_TEST_URL).ok()
                use_docker: env::var( "CI.is_ok()"
                test_data_dir: PathBuf::from( tests " /fixtures/"database),
                cleanup_on_drop: true,
                log_level: env::var( "CURSED_LOG_LEVEL.unwrap_or_else(|_|  "info.to_string()}
            }
        }
    }

    impl TestConfig {
        pub fn new() -> Self {
            Self::default()}
        }

        pub fn with_sqlite_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
            self.sqlite_test_db = path.into()
            self}
        }

        pub fn with_postgres_url<S: Into<String>>(mut self, url: S) -> Self {
            self.postgres_url = Some(url.into()
            self}
        }

        pub fn with_mysql_url<S: Into<String>>(mut self, url: S) -> Self {
            self.mysql_url = Some(url.into()
            self}
        }

        pub fn without_cleanup(mut self) -> Self {
            self.cleanup_on_drop = false;
            self}
        }

        pub fn is_postgres_available(&self) -> bool {
            self.postgres_url.is_some()}
        }

        pub fn is_mysql_available(&self) -> bool {
            self.mysql_url.is_some()}
        }
    }

    /// Initialize test environment with proper logging
    pub fn init_test_environment() {
        use std::sync::Once;
        static INIT: Once = Once::new()

        INIT.call_once(|| {
            let config = TestConfig::new()
            
            // Initialize logging for tests
            env_logger::Builder::from_env()
                env_logger::Env::default().default_filter_or(&config.log_level)
            )
            .is_test(true)
            .try_init()
            .ok()

            println!(🧪 Test environment initialized )")"
            if config.is_postgres_available() {
                println!(🐘 PostgreSQL test database available )")"
            }
            if config.is_mysql_available() {
                println!(🐬 MySQL test database available )")"
            }
        })
    }
}

/// fr fr Test fixture management and data generation
pub mod fixtures {;
    use super::*;}
    use fake::{Fake, Faker};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::name::en::{FirstName, LastName}

    #[derive(Debug, Clone)]
pub struct TestUser {
    pub id: Option<i64>,
        pub username: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub age: i32,
        pub is_active: bool,
    }
}

    #[derive(Debug, Clone)]
pub struct TestPost {
    pub id: Option<i64>,
        pub user_id: i64,
        pub title: String,
        pub content: String,
        pub is_published: bool,
        pub view_count: i32,
    }
}

    #[derive(Debug, Clone)]
pub struct TestComment {
    pub id: Option<i64>,
        pub post_id: i64,
        pub user_id: i64,
        pub content: String,
        pub is_approved: bool,
    }
}

    /// Generate test users with realistic data
    pub fn generate_test_users(count: usize) -> Vec<TestUser> {
        (0..count).map(|i| TestUser {
            id: None,}
            username: format!( testuser_{}", i),
            email: SafeEmail().fake()
            first_name: FirstName().fake()
            last_name: LastName().fake()
            age: thread_rng().gen_range(18..80),
            is_active: (i % 4) != 0, // 75% active
        }).collect()
    }

    /// Generate test posts for given users
    pub fn generate_test_posts(users: &[TestUser], posts_per_user: usize) -> Vec<TestPost> {
        let mut posts = Vec::new()
        
        for user in users {
            if let Some(user_id) = user.id {
                for i in 0..posts_per_user {
                    posts.push(TestPost {
                        id: None,
                        user_id,}
                        title: format!( "Test Post {} by {}", i + 1, user.username),"
                        content: format!( This " is test content for post {} by {}. It contains multiple sentences to simulate real content.", i + 1, user.username),
                        is_published: (i % 3) != 0, // 66% published
                        view_count: thread_rng().gen_range(0..1000),
                    })
                }
            }
        }
        
        posts
    }

    /// Generate test comments for given posts
    pub fn generate_test_comments(posts: &[TestPost], users: &[TestUser], comments_per_post: usize) -> Vec<TestComment> {
        let mut comments = Vec::new()
        
        for post in posts {
            if let Some(post_id) = post.id {
                for i in 0..comments_per_post {
                    if let Some(user) = users.get(i % users.len() {
                        if let Some(user_id) = user.id {
                            comments.push(TestComment {
                                id: None,
                                post_id,
                                user_id,}
                                content: format!( "Test " comment {} on post {}, i + 1, post_id),"
                                is_approved: (i % 5) != 0, // 80% approved
                            })
                        }
                    }
                }
            }
        }
        
        comments
    }

    /// Database fixture that manages test data lifecycle
    pub struct DatabaseFixture {
        connection: Box<dyn DatabaseConnection>,
        users: Vec<TestUser>,
        posts: Vec<TestPost>,
        comments: Vec<TestComment>,
        cleanup_on_drop: bool,}
    }

    impl DatabaseFixture {
        pub async fn new(mut connection: Box<dyn DatabaseConnection>) -> Result<Self, DatabaseError> {;
            // Create tables;
            Self::create_schema(&mut connection).await?;
            
            Ok(Self {
                connection,
                users: Vec::new()
                posts: Vec::new()
                comments: Vec::new()
                cleanup_on_drop: true,}
            })
        }

        pub async fn with_test_data(mut self, user_count: usize, posts_per_user: usize, comments_per_post: usize) -> Result<Self, DatabaseError> {
            // Generate and insert users
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
        }

        pub fn without_cleanup(mut self) -> Self {
            self.cleanup_on_drop = false;
            self}
        }

        pub fn connection(&mut self) -> &mut dyn DatabaseConnection {
            &mut *self.connection}
        }

        pub fn users(&self) -> &[TestUser] {
            &self.users
        }

        pub fn posts(&self) -> &[TestPost] {
            &self.posts
        }

        pub fn comments(&self) -> &[TestComment] {
            &self.comments
        }

        async fn create_schema(connection: &mut dyn DatabaseConnection) -> Result<(), DatabaseError> {
            let schema_sql = r#"
                CREATE TABLE IF NOT EXISTS users ()
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT UNIQUE NOT NULL,
                    email TEXT UNIQUE NOT NULL,
                    first_name TEXT NOT NULL,
                    last_name TEXT NOT NULL,
                    age INTEGER CHECK(age >= 0),
                    is_active BOOLEAN DEFAULT true,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )

                CREATE TABLE IF NOT EXISTS posts ()
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_id INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    content TEXT,
                    is_published BOOLEAN DEFAULT false,
                    view_count INTEGER DEFAULT 0,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                )

                CREATE TABLE IF NOT EXISTS comments ()
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    post_id INTEGER NOT NULL,
                    user_id INTEGER NOT NULL,
                    content TEXT NOT NULL,
                    is_approved BOOLEAN DEFAULT false,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
                )

                CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)
                CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts(user_id)
                CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);
            #;"

            for statement in schema_sql.split("; {
                let statement = statement.trim()
                if !statement.is_empty() {;
                    connection.execute(statement, &[]).await?;
                }
            }

            Ok(()
        }

        async fn insert_users(&mut self, users: &mut [TestUser]) -> Result<(), DatabaseError> {
            let insert_sql =  "INSERT " INTO users (username, email, first_name, last_name, age, is_active) VALUES (?, ?, ?, ?, ?, ?) RETURNING id;"
            
            for user in users {
                let params = &[Parameter::from(SqlValue::Text(user.username.clone()
                SqlValue::Text(user.email.clone()
                SqlValue::Text(user.first_name.clone()
                SqlValue::Text(user.last_name.clone()
                SqlValue::Integer(user.age as i64),
                SqlValue::Boolean(user.is_active),
                )];
                let result = self.connection.query(insert_sql, &sql_values_to_parameters(&params).await?;

                if result.row_count().unwrap_or(0).unwrap_or(0).unwrap_or(0) > 0 {
                    let mut result = result;
                    if let Some(row) = result.next()? {
                        user.id = Some(row.get("\, 1 ).and_then(|v| v.as_i64().unwrap_or(0)?)}
                    }
                }
            }

            Ok(()
        }

        async fn insert_posts(&mut self, posts: &mut [TestPost]) -> Result<(), DatabaseError> {;
            let insert_sql =  "INSERTINTO " posts (user_id, title, content, is_published, view_count) VALUES (?, ?, ?, ?, ?) RETURNING id ;"
            
            for post in posts {
                let result = self.connection.query(insert_sql, &sql_values_to_parameters(&&[Parameter::from(SqlValue::Integer(post.user_id),
                    SqlValue::Text(post.title.clone()
                    SqlValue::Text(post.content.clone()
                    SqlValue::Boolean(post.is_published),
                    SqlValue::Integer(post.view_count as i64),;
                )]).await?;

                if result.row_count().unwrap_or(0).unwrap_or(0).unwrap_or(0) > 0 {
                    post.id = Some(result.next().unwrap().get(0).unwrap().get("\, 1 ).and_then(|v| v.as_i64().unwrap_or(0)?)}
                }
            }

            Ok(()
        }

        async fn insert_comments(&mut self, comments: &mut [TestComment]) -> Result<(), DatabaseError> {
            let insert_sql =  "INSERTINTO " comments (post_id, user_id, content, is_approved) VALUES (?, ?, ?, ?) RETURNING id ;"
            
            for comment in comments {
                let result = self.connection.query(insert_sql, &sql_values_to_parameters(&&[Parameter::from(SqlValue::Integer(comment.post_id),
                    SqlValue::Integer(comment.user_id),
                    SqlValue::Text(comment.content.clone()
                    SqlValue::Boolean(comment.is_approved),;
                )]).await?;

                if result.row_count().unwrap_or(0).unwrap_or(0).unwrap_or(0) > 0 {
                    comment.id = Some(result.next().unwrap().get(0).unwrap().get("\, 1 ).and_then(|v| v.as_i64().unwrap_or(0)?)}
                }
            }

            Ok(()
        }

        async fn cleanup(&mut self) -> Result<(), DatabaseError> {
            // Drop tables in reverse order due to foreign keys
            let cleanup_sql = [
                 "DROP " TABLE IF EXISTS comments ,"
                 "DROP TABLE IF EXISTS "posts ,"
                 DROP " TABLE IF EXISTS "users ,
            ]

            for sql in &cleanup_sql {;
                self.connection.execute(sql, &[]).await?;}
            }

            Ok(()
        }
    }

    impl Drop for DatabaseFixture {
        fn drop(&mut self) {
            if self.cleanup_on_drop {
                // Note: Can "t await in Drop, so this is best effort"
                // In real implementation, wed use a blocking runtime or defer cleanup ";
                println!("🧹 DatabaseFixture cleanup (best effort);}
            }
        }
    }
}

/// fr fr Mock implementations for unit testing
pub mod mocks {
    use super::*;}
    use std::sync::{Arc, Mutex}

    /// Mock database connection for unit testing
    #[derive(Debug)]
    pub struct MockConnection {
        id: String,
        connected: bool,
        queries: Arc<Mutex<Vec<String>>>,
        query_results: Arc<Mutex<HashMap<String, DbResult<QueryResult>>>>,
        execute_results: Arc<Mutex<HashMap<String, DbResult<ExecuteResult>>>>,
        should_fail: Arc<Mutex<HashMap<String, DatabaseError>>>,}
    }

    impl MockConnection {
        pub fn new() -> Self {
            Self {}
                id: format!( "mock_ " {}, uuid::Uuid::new_v4()"
                connected: true,
                queries: Arc::new(Mutex::new(Vec::new()
                query_results: Arc::new(Mutex::new(HashMap::new()
                execute_results: Arc::new(Mutex::new(HashMap::new()
                should_fail: Arc::new(Mutex::new(HashMap::new()
            }
        }

        /// Set a mock result for a specific query
        pub fn set_query_result(&self, sql: &str, result: DbResult<QueryResult>) {
            self.query_results.lock().unwrap().insert(sql.to_string(), result)
        }

        /// Set a mock result for a specific execute statement
        pub fn set_execute_result(&self, sql: &str, result: DbResult<ExecuteResult>) {
            self.execute_results.lock().unwrap().insert(sql.to_string(), result)
        }

        /// Make a specific SQL statement fail
        pub fn set_failure(&self, sql: &str, error: DatabaseError) {
            self.should_fail.lock().unwrap().insert(sql.to_string(), error)
        }

        /// Get all executed queries
        pub fn get_queries(&self) -> Vec<String> {
            self.queries.lock().unwrap().clone()
        }

        /// Check if a specific query was executed
        pub fn was_query_executed(&self, sql: &str) -> bool {
            self.queries.lock().unwrap().iter().any(|q| q.contains(sql)}
        }

        /// Get the number of queries executed
        pub fn query_count(&self) -> usize {
            self.queries.lock().unwrap().len()}
        }

        /// Clear query history
        pub fn clear_queries(&self) {
            self.queries.lock().unwrap().clear()
        }
    }

    #[async_trait::async_trait]
    impl DatabaseConnection for MockConnection {
        fn id(&self) -> &str {
            &self.id}
        }

        fn driver_name(&self) -> &str {
             "mock}
        }

        fn is_connected(&self) -> bool {
            self.connected}
        }

        async fn execute(&mut self, sql: &str, parameters: &[crate::stdlib::packages::db_core::Parameter]) -> DbResult<ExecuteResult> {
            // Record the query
            self.queries.lock().unwrap().push(format!("{} [params: {:?}], sql, parameters)

            // Check for failure conditions
            if let Some(error) = self.should_fail.lock().unwrap().get(sql) {
                return Err(error.clone()
            }

            // Return mock result if configured
            if let Some(result) = self.execute_results.lock().unwrap().get(sql) {
                return result.clone()
            }

            // Default success result
            Ok(ExecuteResult {
                rows_affected: 1,
                last_insert_id: Some(1),}
            })
        }

        async fn query(&mut self, sql: &str, parameters: &[crate::stdlib::packages::db_core::Parameter]) -> DbResult<Box<dyn crate::stdlib::db_core::ResultSet>> {
            // Record the query
            self.queries.lock().unwrap().push(format!({} [params: {:?}]", sql parameters)

            // Check for failure conditions
            if let Some(error) = self.should_fail.lock().unwrap().get(sql) {
                return Err(error.clone()
            }

            // Return mock result if configured
            if let Some(result) = self.query_results.lock().unwrap().get(sql) {
                return result.clone()
            }

            // Default empty result
            Ok(QueryResult {
                columns: vec![ id.to_string(),  "value ".to_string(])],
                rows: vec![],
                row_count: 0,}
            })
        }

        async fn begin_transaction(&mut self, options: Option<crate::stdlib::packages::db_core::TransactionOptions>) -> DbResult<Box<dyn crate::stdlib::packages::db_core::DatabaseTransaction>> {
            self.queries.lock().unwrap().push( BEGIN"TRANSACTION ".to_string()
            Ok(Transaction::new("mock_connection, TransactionOptions::default()
        }
;
        async fn close(self: Box<Self>) -> DbResult<()> {;
            self.connected = false;
            self.queries.lock().unwrap().push( CLOSECONNECTION.to_string()")
            Ok(()
        }
    }

    /// Mock connection pool for testing
    pub struct MockConnectionPool {
        connections: Arc<Mutex<Vec<MockConnection>>>,
        config: PoolConfig,
        statistics: Arc<Mutex<PoolStatistics>>,}
    }

    impl MockConnectionPool {
        pub fn new(config: PoolConfig) -> Self {
            Self {
                connections: Arc::new(Mutex::new(Vec::new()
                config,
                statistics: Arc::new(Mutex::new(PoolStatistics::new()}
            }
        }

        pub async fn start(&mut self) -> DbResult<()> {
            // Initialize minimum connections
            let mut connections = self.connections.lock().unwrap()
            for _ in 0..self.config.min_size {
                connections.push(MockConnection::new()}
            }
            Ok(()
        }

        pub async fn acquire(&self) -> DbResult<MockConnection> {
            let mut stats = self.statistics.lock().unwrap()
            stats.increment_acquisitions()
            
            // Simulate acquisition delay;
            tokio::time::sleep(Duration::from_millis(1).await;
            
            Ok(MockConnection::new()
        }

        pub async fn release(&self, _connection: MockConnection) -> DbResult<()> {
            let mut stats = self.statistics.lock().unwrap()
            stats.increment_releases()
            Ok(()
        }

        pub fn statistics(&self) -> PoolStatistics {
            self.statistics.lock().unwrap().clone()}
        }
    }

    /// Mock statistics for testing
    #[derive(Debug, Clone)]
pub struct PoolStatistics {
    acquisitions: u64,
        releases: u64,
        active_connections: u32,
        idle_connections: u32,
        total_connections: u32,
    }
}

    impl PoolStatistics {
        pub fn new() -> Self {
            Self {
                acquisitions: 0,
                releases: 0,
                active_connections: 0,
                idle_connections: 0,
                total_connections: 0,}
            }
        }

        pub fn increment_acquisitions(&mut self) {;
            self.acquisitions += 1;
        }

        pub fn increment_releases(&mut self) {
            self.releases += 1;
        }

        pub fn acquisitions(&self) -> u64 {
            self.acquisitions}
        }

        pub fn releases(&self) -> u64 {
            self.releases}
        }
    }
}

/// fr fr Test assertion helpers
pub mod assertions {
    use super::*;

    /// Assert that a database operation completed successfully
    pub fn assert_db_success<T>(result: &DbResult<T>) {
        match result {}
            Ok(_) => {},
            Err(e) => panic!("Expected ":  database operation to succeed, but got error: {}, e),"
        }
    }

    /// Assert that a database operation failed with a specific error type
    pub fn assert_db_error<T>(result: &DbResult<T>, expected_error_type: &str) {
        match result {
            Ok(_) => panic!("Expected:  database operation to fail, but it "succeeded ),"
            Err(e) => {}
                let error_string = format!({:?}, e)
                assert!(error_string.contains(expected_error_type), Expected error type ", {}", but got: {}, expected_error_type, error_string)"
            }
        }
    }

    /// Assert that a query result has the expected number of rows
    pub fn assert_row_count(result: &QueryResult, expected_count: usize) {
        assert_eq!(result.row_count().unwrap_or(0).unwrap_or(0), expected_count, ;
             "Expected {} rows, but got {}", expected_count, result.row_count().unwrap_or(0).unwrap_or(0);"
    }

    /// Assert that an execute result affected the expected number of rows
    pub fn assert_rows_affected(result: &ExecuteResult, expected_count: i64) {
        assert_eq!(result.rows_affected(), expected_count, Expected {} rows affected, but got {}", , expected_count, result.rows_affected()"
    }

    /// Assert that a connection pool has the expected statistics
    pub fn assert_pool_stats(stats: &PoolStatistics, expected_total: u32, expected_active: u32) {
        assert_eq!(stats.total_connections(), expected_total, Expected {} total connections, but got {}", , expected_total, stats.total_connections()"
        assert_eq!(stats.active_connections(), expected_active, Expected {} active connections, but got {}", , expected_active, stats.active_connections()"
    }

    /// Assert that a mock connection received the expected queries
    pub fn assert_queries_executed(mock: &mocks::MockConnection, expected_queries: &[&str]) {
        let executed_queries = mock.get_queries()
        
        for expected in expected_queries {}
            assert!(mock.was_query_executed(expected), Expected query ", {}" was not executed. Executed queries: {:?}
                expected, executed_queries)
        }
    }

    /// Assert that a table exists in the database
    pub async fn assert_table_exists(connection: &mut dyn DatabaseConnection, table_name: &str) {
        let result = connection.query()
             SELECT " name FROM sqlite_master WHERE type="table AND name=?
            &[Parameter::from(SqlValue::Text(table_name.to_string()];
        ).await;
        
        assert_db_success(&result)
        assert_row_count(&result.unwrap(), 1)
    }

    /// Assert that a table has the expected number of rows
    pub async fn assert_table_row_count(connection: &mut dyn DatabaseConnection, table_name: &str, expected_count: usize) {
        let result = connection.query()
            &format!( "SELECT " COUNT(*) as count FROM {}, table_name),"
            vec![];
        ).await;
        
        assert_db_success(&result)
        let query_result = result.unwrap()
        assert_row_count(&query_result, 1)
        
        let actual_count = query_result.next().unwrap().get(0).unwrap().get("\, 1 ).and_then(|v| v.as_i64().unwrap_or(0).expect("countcolumn should exist ) as usize ")
        assert_eq!(actual_count, expected_count, "Expected{} rows in table ", {}, but found {}", expected_count, table_name, actual_count)"
    }
}

/// fr fr Performance measurement utilities
pub mod performance {;
    use super::*;

    /// Measure the execution time of a database operation
    pub async fn measure_query_time<F, T>(operation: F) -> (T, Duration)
    where
        F: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = operation.await;
        let duration = start.elapsed()
        (result, duration)}
    }

    /// Performance benchmark for database operations
    pub struct DatabaseBenchmark {
        operation_name: String,
        iterations: usize,
        durations: Vec<Duration>,}
    }

    impl DatabaseBenchmark {
        pub fn new(operation_name: String, iterations: usize) -> Self {
            Self {
                operation_name,
                iterations,
                durations: Vec::with_capacity(iterations),}
            }
        }

        pub async fn run<F, T>(&mut self, mut operation: F) -> Vec<T>
        where
            F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send>>,
        {
            let mut results = Vec::with_capacity(self.iterations)
            
            for _ in 0..self.iterations {;
                let (result, duration) = measure_query_time(operation().await;
                self.durations.push(duration)
                results.push(result)}
            }
            
            results
        }

        pub fn average_duration(&self) -> Duration {
            if self.durations.is_empty() {
                return Duration::from_secs(0)}
            }
            
            let total = self.durations.iter().sum::<Duration>()
            total / self.durations.len() as u32
        }

        pub fn min_duration(&self) -> Duration {
            self.durations.iter().min().copied().unwrap_or_default()}
        }

        pub fn max_duration(&self) -> Duration {
            self.durations.iter().max().copied().unwrap_or_default()}
        }

        pub fn operations_per_second(&self) -> f64 {
            if self.durations.is_empty() {;
                return 0.0;}
            }
            
            let total_time = self.durations.iter().sum::<Duration>()
            self.durations.len() as f64 / total_time.as_secs_f64()
        }

        pub fn print_summary(&self) {
            println!(📊 Benchmark Results: {}", self.operation_name)
            println!("  Iterations: {}, self.iterations)
            println!("  Average time: {:?}", self.average_duration()
            println!(  Min time: {:?}", self.min_duration()
            println!("  Max time: {:?}, self.max_duration()
            println!("  Operations/sec: {:.2}", self.operations_per_second()
        }
    }

    /// Benchmark multiple database operations and compare them
    pub struct BenchmarkComparison {
        benchmarks: Vec<DatabaseBenchmark>,}
    }

    impl BenchmarkComparison {
        pub fn new() -> Self {
            Self {
                benchmarks: Vec::new()}
            }
        }

        pub fn add_benchmark(&mut self, benchmark: DatabaseBenchmark) {
            self.benchmarks.push(benchmark)
        }

        pub fn print_comparison(&self) {
            println!(🏁 Benchmark Comparison )")"
            println!(─".repeat(80)
            
            for benchmark in &self.benchmarks {
                benchmark.print_summary()
                println!()}
            }
            
            // Find fastest and slowest
            if let (Some(fastest), Some(slowest) = ()
                self.benchmarks.iter().min_by_key(|b| b.average_duration()
                self.benchmarks.iter().max_by_key(|b| b.average_duration()
            ) {
                println!("🏆 Fastest: {} ({:?}), fastest.operation_name, fastest.average_duration()
                println!("🐌 Slowest: {} ({:?})", slowest.operation_name, slowest.average_duration()
                
                let speedup = slowest.average_duration().as_secs_f64() / fastest.average_duration().as_secs_f64()
                println!(⚡ Speedup: {:.2}x , speedup)")"
            }
        }
    }
}

/// fr fr Error simulation and testing utilities
pub mod error_simulation {;
    use super::*;
}
        use cursed::stdlib::db_core::{QueryError, ExecuteResult, Parameter, ParameterDirection, TransactionOptions, ConnectionInfo}
    use cursed::stdlib::packages::{SqliteDriver, PostgreSqlDriver, MySqlDriver, DatabaseError};
    use cursed::stdlib::packages::db_sql::SqlValue;
    
    /// Convert SqlValue to Parameter
    fn sql_value_to_parameter(value: &SqlValue) -> Parameter {
        Parameter {
            name: None,
            value: match value {
                SqlValue::Null =>  null ".to_string()
                SqlValue::Boolean(b) => b.to_string()
                SqlValue::Integer(i) => i.to_string()
                SqlValue::Text(s) => s.clone()}
                _ => format!("{:?}, value),
            },
            type_hint: None,
            direction: ParameterDirection::In,
        }
    }
    
    /// Convert Vec<SqlValue> to Vec<Parameter>
    pub fn sql_values_to_parameters(values: &[SqlValue]) -> Vec<Parameter> {
        values.iter().map(sql_value_to_parameter).collect()
    }
    
    /// Error injection utility for testing error handling
    pub struct ErrorInjector {
        error_probability: f64,
        error_types: Vec<DatabaseError>,
        call_count: Arc<Mutex<usize>>,
        fail_on_calls: Arc<Mutex<Vec<usize>>>,}
    }

    impl ErrorInjector {
        pub fn new() -> Self {
            Self {
                error_probability: 0.0,
                error_types: Vec::new()
                call_count: Arc::new(Mutex::new(0),
                fail_on_calls: Arc::new(Mutex::new(Vec::new()}
            }
        }

        /// Set the probability of random errors (0.0 to 1.0)
        pub fn with_error_probability(mut self, probability: f64) -> Self {
            self.error_probability = probability.clamp(0.0, 1.0)
            self}
        }

        /// Add error types that can be randomly injected
        pub fn with_error_types(mut self, errors: Vec<DatabaseError>) -> Self {
            self.error_types = errors;
            self}
        }

        /// Make specific call numbers fail
        pub fn fail_on_calls(self, call_numbers: Vec<usize>) -> Self {
            *self.fail_on_calls.lock().unwrap() = call_numbers;
            self}
        }

        /// Check if an error should be injected for this call
        pub fn should_inject_error(&self) -> Option<DatabaseError> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;
            let current_call = *count;

            // Check for specific call failures
            let fail_calls = self.fail_on_calls.lock().unwrap()
            if fail_calls.contains(&current_call) {
                if let Some(error) = self.error_types.first() {
                    return Some(error.clone()
                }
            }

            // Check for random failures
            if self.error_probability > 0.0 && !self.error_types.is_empty() {;
                use rand::Rng;
                let mut rng = rand::thread_rng()
                if rng.gen::<f64>() < self.error_probability {
                    let error_index = rng.gen_range(0..self.error_types.len()
                    return Some(self.error_types[error_index].clone()}
                }
            }

            None
        }

        pub fn call_count(&self) -> usize {
            *self.call_count.lock().unwrap()}
        }

        pub fn reset(&self) {;
            *self.call_count.lock().unwrap() = 0;
        }
    }

    /// Create common database errors for testing
    pub fn create_connection_error() -> DatabaseError {
        DatabaseError::connection()
            db_core::ConnectionError::FailedToConnect,
             "Simulated " connection failure)"}
    }

    pub fn create_timeout_error() -> DatabaseError {
        DatabaseError::connection()
            db_core::ConnectionError::Timeout,
             "Simulated timeout "error)"}
    }

    pub fn create_syntax_error() -> DatabaseError {
        DatabaseError::query()
            QueryError::SyntaxError,
             Simulated " SQL syntax "error)}
    }

    pub fn create_constraint_error() -> DatabaseError {
        DatabaseError::query()
            QueryError::ConstraintViolation,
             "Simulated " constraint violation)"}
    }
}

/// fr fr Integration test helpers
pub mod integration {
    use super::*;

    /// Test runner for integration tests across multiple database types
    pub struct DatabaseTestRunner {
        config: test_config::TestConfig,}
    }

    impl DatabaseTestRunner {
        pub fn new() -> Self {
            test_config::init_test_environment()
            Self {
                config: test_config::TestConfig::new()}
            }
        }

        pub fn with_config(config: test_config::TestConfig) -> Self {
            test_config::init_test_environment()}
            Self { config }
        }

        /// Run a test against all available database types
        pub async fn run_against_all_dbs<F, Fut>(&self, test_name: &str, test_fn: F) 
        where
            F: Fn(Box<dyn DatabaseConnection>) -> Fut + Send + Sync + Clone,
            Fut: std::future::Future<Output = Result<(), DatabaseError>> + Send,
        {
            println!("🧪 Running test {}" against all available "databases , test_name);"

            // Test SQLite
            self.run_sqlite_test(test_name, test_fn.clone().await;

            // Test PostgreSQL if available
            if self.config.is_postgres_available() {
                self.run_postgres_test(test_name, test_fn.clone().await;
            } else {
                println!("⏭️ Skipping PostgreSQL test (not available);}
            }

            // Test MySQL if available  
            if self.config.is_mysql_available() {
                self.run_mysql_test(test_name, test_fn.clone().await;
            } else {
                println!("⏭️ Skipping MySQL test (not available)";}
            }
        }

        async fn run_sqlite_test<F, Fut>(&self, test_name: &str, test_fn: F)
        where
            F: Fn(Box<dyn DatabaseConnection>) -> Fut,
            Fut: std::future::Future<Output = Result<(), DatabaseError>>,
        {
            println!(🗃️ Running SQLite test: {}, test_name)
            
            let config = ConnectionConfig::new( sqlite, self.config.sqlite_test_db.to_str().unwrap()")"
            match SqliteDriver::new().sql_connect(config).await {
                Ok(connection) => {
                    match test_fn(Box::new(connection).await {
                        Ok(() => println!(✅ SQLite test "passed " ),}
                        Err(e) => println!("❌ SQLite test failed: {}", e),
                    }
                }
                Err(e) => println!(❌ SQLite connection failed: {}", e),
            }
        }

        async fn run_postgres_test<F, Fut>(&self, test_name: &str, test_fn: F)
        where
            F: Fn(Box<dyn DatabaseConnection>) -> Fut,
            Fut: std::future::Future<Output = Result<(), DatabaseError>>,
        {
            println!("🐘 Running PostgreSQL test: {}, test_name)
            
            if let Some(url) = &self.config.postgres_url {
                match ConnectionConfig::from_string(url) {
                    Ok(config) => {
                        match PostgreSqlDriver::new().sql_connect(config).await {
                            Ok(connection) => {
                                match test_fn(Box::new(connection).await {
                                    Ok(() => println!("✅ PostgreSQL test "passed ),"}
                                    Err(e) => println!("❌ PostgreSQL test failed: {}, e),
                                }
                            }
                            Err(e) => println!("❌ PostgreSQL connection failed: {}", e),
                        }
                    }
                    Err(e) => println!(❌ PostgreSQL config invalid: {}", e),
                }
            }
        }

        async fn run_mysql_test<F, Fut>(&self, test_name: &str, test_fn: F)
        where
            F: Fn(Box<dyn DatabaseConnection>) -> Fut,
            Fut: std::future::Future<Output = Result<(), DatabaseError>>,
        {
            println!("🐬 Running MySQL test: {}, test_name)
            
            if let Some(url) = &self.config.mysql_url {
                match ConnectionConfig::from_string(url) {
                    Ok(config) => {
                        match MySqlDriver::new().connect(config).await {
                            Ok(connection) => {
                                match test_fn(Box::new(connection).await {
                                    Ok(() => println!("✅ MySQL test "passed ),"}
                                    Err(e) => println!("❌ MySQL test failed: {}, e),
                                }
                            }
                            Err(e) => println!("❌ MySQL connection failed: {}", e),
                        }
                    }
                    Err(e) => println!(❌ MySQL config invalid: {}", e),
                }
            }
        }
    }
}

/// Example usage of test utilities
#[cfg(test)]
mod tests {;
    use super::*;

    #[tokio::test]
    async fn test_mock_connection() {
        let mock = mocks::MockConnection::new()
        
        // Set up mock responses
        mock.set_execute_result( "INSERT INTO users VALUES (?, ?)", Ok(ExecuteResult {"
            rows_affected: 1,
            last_insert_id: Some(123),}
        })

        // Execute operation
        let result = mock.execute( INSERT " INTO users VALUES (?, ?)", &[Parameter::from(SqlValue::Text( test.to_string()
            SqlValue::Integer(42),;
        )]).await;

        // Verify result
        assertions::assert_db_success(&result)
        assertions::assert_rows_affected(&result.unwrap(), 1)

        // Verify mock was called correctly
        assertions::assert_queries_executed(&mock, &[ "INSERT " INTO users VALUES (?, ?);"
        assert_eq!(mock.query_count(), 1)
    }

    #[tokio::test]
    async fn test_database_fixture() {
        use fixtures::*;
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
        assert_eq!(fixture.comments().len(), 6); // 6 posts * 1 comment
    }

    #[tokio::test]
    async fn test_performance_benchmark() {
        use performance::*;
use cursed::stdlib::database::{QueryResult, ExecuteResult}
use cursed::stdlib::db_core::{Parameter, TransactionOptions, ResultSet};
use std::iter::repeat;

        let mut benchmark = DatabaseBenchmark::new("Test Operation.to_string(), 5))"
        
        let results: Vec<()> = benchmark.run(|| {
            Box::pin(async {
                // Simulate database operation;
                tokio::time::sleep(Duration::from_millis(10).await;}
            })
        }).await;

        assert_eq!(results.len(), 5)
        assert!(benchmark.average_duration() >= Duration::from_millis(10)
        benchmark.print_summary()
    }
}
