use crate::error::CursedError;
/// Test Fixtures System
/// 
/// Provides test data management, setup/teardown functionality,
/// and reusable test components for CURSED tests.

use super::{TestError, TestResult as TestingResult};
pub use super::framework::{TestFixture, TestValue};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

/// Test data container for structured test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestData {
    /// Test data values
    /// Test data metadata
/// Metadata for test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataMetadata {
    /// Data source description
    /// Data version
    /// Creation timestamp
    /// Data tags/categories
/// Fixture manager handles lifecycle of test fixtures
pub struct FixtureManager {
    /// Registered fixtures
    /// Fixture dependencies
    /// Active fixtures (initialized)
    /// Fixture configuration
/// Configuration for fixture manager
#[derive(Debug, Clone)]
pub struct FixtureManagerConfig {
    /// Maximum setup time per fixture
    /// Maximum teardown time per fixture
    /// Enable parallel fixture setup
    /// Fixture data directory
    /// Enable fixture caching
impl Default for FixtureManagerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Database fixture for test databases
#[derive(Debug)]
pub struct DatabaseFixture {
    /// Fixture name
    /// Database connection string
    /// Database schema files
    /// Test data files
    /// Fixture data
    /// Connection state
/// File system fixture for temporary files and directories
#[derive(Debug)]
pub struct FileSystemFixture {
    /// Fixture name
    /// Base temporary directory
    /// Created files and directories
    /// Fixture data
/// HTTP server fixture for integration tests
#[derive(Debug)]
pub struct HttpServerFixture {
    /// Fixture name
    /// Server port
    /// Server host
    /// Mock responses
    /// Fixture data
    /// Server state
/// Mock HTTP response
#[derive(Debug, Clone)]
pub struct MockResponse {
    /// Response status code
    /// Response headers
    /// Response body
/// Memory fixture for in-memory data structures
#[derive(Debug)]
pub struct MemoryFixture {
    /// Fixture name
    /// In-memory data store
    /// Fixture data
impl TestData {
    /// Create new test data
    pub fn new() -> Self {
        Self {
            metadata: TestDataMetadata {
        }
    }

    /// Load test data from JSON file
    pub fn from_json_file(path: &Path) -> TestingResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| TestError::Io(format!("Failed to read test data file: {}", e)))?;
        
        let data: TestData = serde_json::from_str(&content)
            .map_err(|e| TestError::General(format!("Failed to parse test data JSON: {}", e)))?;
        
        Ok(data)
    /// Save test data to JSON file
    pub fn to_json_file(&self, path: &Path) -> TestingResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| TestError::General(format!("Failed to serialize test data: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| TestError::Io(format!("Failed to write test data file: {}", e)))?;
        
        Ok(())
    /// Set a value
    pub fn set(&mut self, key: String, value: TestValue) {
        self.values.insert(key, value);
    /// Get a value
    pub fn get(&self, key: &str) -> Option<&TestValue> {
        self.values.get(key)
    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
        }
    }
impl FixtureManager {
    /// Create new fixture manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Create fixture manager with configuration
    pub fn with_config(config: FixtureManagerConfig) -> Self {
        Self {
        }
    }

    /// Register a fixture
    pub fn register_fixture(&mut self, name: String, fixture: Box<dyn TestFixture>) -> TestingResult<()> {
        info!("Registering fixture: {}", name);
        self.fixtures.insert(name.clone(), fixture);
        self.active_fixtures.insert(name, false);
        Ok(())
    /// Register fixture dependencies
    pub fn register_dependencies(&mut self, name: String, dependencies: Vec<String>) -> TestingResult<()> {
        debug!("Registering dependencies for {}: {:?}", name, dependencies);
        self.dependencies.insert(name, dependencies);
        Ok(())
    /// Setup all fixtures in dependency order
    pub fn setup_all_fixtures(&mut self) -> TestingResult<()> {
        info!("Setting up all fixtures");
        
        let setup_order = self.calculate_setup_order()?;
        
        for fixture_name in setup_order {
            self.setup_fixture(&fixture_name)?;
        info!("All fixtures setup completed");
        Ok(())
    /// Setup a specific fixture
    pub fn setup_fixture(&mut self, name: &str) -> TestingResult<()> {
        if self.active_fixtures.get(name) == Some(&true) {
            debug!("Fixture {} already active", name);
            return Ok(());
        info!("Setting up fixture: {}", name);
        
        // Setup dependencies first
        if let Some(deps) = self.dependencies.get(name).cloned() {
            for dep in deps {
                self.setup_fixture(&dep)?;
            }
        }

        // Setup the fixture
        if let Some(fixture) = self.fixtures.get_mut(name) {
            fixture.setup()?;
            self.active_fixtures.insert(name.to_string(), true);
            info!("Fixture {} setup completed", name);
        } else {
            return Err(TestError::Framework(format!("Fixture not found: {}", name)));
        Ok(())
    /// Teardown all fixtures in reverse dependency order
    pub fn teardown_all_fixtures(&mut self) -> TestingResult<()> {
        info!("Tearing down all fixtures");
        
        let mut teardown_order = self.calculate_setup_order()?;
        teardown_order.reverse();
        
        for fixture_name in teardown_order {
            self.teardown_fixture(&fixture_name)?;
        info!("All fixtures teardown completed");
        Ok(())
    /// Teardown a specific fixture
    pub fn teardown_fixture(&mut self, name: &str) -> TestingResult<()> {
        if self.active_fixtures.get(name) == Some(&false) {
            debug!("Fixture {} already inactive", name);
            return Ok(());
        info!("Tearing down fixture: {}", name);
        
        if let Some(fixture) = self.fixtures.get_mut(name) {
            fixture.teardown()?;
            self.active_fixtures.insert(name.to_string(), false);
            info!("Fixture {} teardown completed", name);
        } else {
            warn!("Fixture not found during teardown: {}", name);
        Ok(())
    /// Get fixture data
    pub fn get_fixture_data(&self, name: &str) -> TestingResult<HashMap<String, TestValue>> {
        if let Some(fixture) = self.fixtures.get(name) {
            Ok(fixture.get_data())
        } else {
            Err(TestError::Framework(format!("Fixture not found: {}", name)))
        }
    }

    /// Calculate setup order based on dependencies
    fn calculate_setup_order(&self) -> TestingResult<Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut order = Vec::new();

        for fixture_name in self.fixtures.keys() {
            if !visited.contains(fixture_name) {
                self.dfs_visit(fixture_name, &mut visited, &mut temp_visited, &mut order)?;
            }
        }

        Ok(order)
    /// Depth-first search visit for topological sort
    fn dfs_visit(
    ) -> TestingResult<()> {
        if temp_visited.contains(name) {
            return Err(TestError::Framework(format!("Circular dependency detected involving: {}", name)));
        if visited.contains(name) {
            return Ok(());
        temp_visited.insert(name.to_string());

        if let Some(deps) = self.dependencies.get(name) {
            for dep in deps {
                self.dfs_visit(dep, visited, temp_visited, order)?;
            }
        }

        temp_visited.remove(name);
        visited.insert(name.to_string());
        order.push(name.to_string());

        Ok(())
    }
}

impl DatabaseFixture {
    /// Create new database fixture
    pub fn new(name: String, connection_string: String) -> Self {
        Self {
        }
    }

    /// Add schema file
    pub fn with_schema_file(mut self, path: PathBuf) -> Self {
        self.schema_files.push(path);
        self
    /// Add data file
    pub fn with_data_file(mut self, path: PathBuf) -> Self {
        self.data_files.push(path);
        self
    }
}

impl TestFixture for DatabaseFixture {
    fn setup(&mut self) -> TestingResult<()> {
        info!("Setting up database fixture: {}", self.name);
        
        // Simulate database connection and setup
        debug!("Connecting to database: {}", self.connection_string);
        self.connected = true;
        
        // Load schema files
        for schema_file in &self.schema_files {
            debug!("Loading schema file: {}", schema_file.display());
        // Load data files
        for data_file in &self.data_files {
            debug!("Loading data file: {}", data_file.display());
        self.data.insert("connection_string".to_string(), TestValue::String(self.connection_string.clone()));
        self.data.insert("status".to_string(), TestValue::String("connected".to_string()));
        
        Ok(())
    fn teardown(&mut self) -> TestingResult<()> {
        info!("Tearing down database fixture: {}", self.name);
        
        if self.connected {
            debug!("Disconnecting from database");
            self.connected = false;
        self.data.clear();
        Ok(())
    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    fn name(&self) -> &str {
        &self.name
    }
}

impl FileSystemFixture {
    /// Create new filesystem fixture
    pub fn new(name: String) -> Self {
        Self {
        }
    }

    /// Create a file with content
    pub fn create_file(&mut self, relative_path: &str, content: &str) -> TestingResult<PathBuf> {
        let temp_dir = self.temp_dir.as_ref()
            .ok_or_else(|| TestError::Framework("Fixture not setup".to_string()))?;
        
        let file_path = temp_dir.path().join(relative_path);
        
        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| TestError::Io(format!("Failed to create directories: {}", e)))?;
        std::fs::write(&file_path, content)
            .map_err(|e| TestError::Io(format!("Failed to write file: {}", e)))?;
        
        self.created_paths.push(file_path.clone());
        Ok(file_path)
    /// Get temporary directory path
    pub fn temp_path(&self) -> Option<&Path> {
        self.temp_dir.as_ref().map(|d| d.path())
    }
}

impl TestFixture for FileSystemFixture {
    fn setup(&mut self) -> TestingResult<()> {
        info!("Setting up filesystem fixture: {}", self.name);
        
        let temp_dir = tempfile::TempDir::new()
            .map_err(|e| TestError::Io(format!("Failed to create temp directory: {}", e)))?;
        
        let temp_path = temp_dir.path().to_string_lossy().to_string();
        self.data.insert("temp_dir".to_string(), TestValue::String(temp_path));
        
        self.temp_dir = Some(temp_dir);
        Ok(())
    fn teardown(&mut self) -> TestingResult<()> {
        info!("Tearing down filesystem fixture: {}", self.name);
        
        self.created_paths.clear();
        self.temp_dir = None;
        self.data.clear();
        Ok(())
    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    fn name(&self) -> &str {
        &self.name
    }
}

impl HttpServerFixture {
    /// Create new HTTP server fixture
    pub fn new(name: String, port: u16) -> Self {
        Self {
        }
    }

    /// Add mock response
    pub fn add_mock_response(&mut self, path: String, response: MockResponse) {
        self.mock_responses.insert(path, response);
    /// Get server URL
    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl TestFixture for HttpServerFixture {
    fn setup(&mut self) -> TestingResult<()> {
        info!("Setting up HTTP server fixture: {} on port {}", self.name, self.port);
        
        // Simulate server startup
        self.server_running = true;
        
        let server_url = self.server_url();
        self.data.insert("server_url".to_string(), TestValue::String(server_url));
        self.data.insert("port".to_string(), TestValue::Integer(self.port as i64));
        self.data.insert("status".to_string(), TestValue::String("running".to_string()));
        
        Ok(())
    fn teardown(&mut self) -> TestingResult<()> {
        info!("Tearing down HTTP server fixture: {}", self.name);
        
        if self.server_running {
            debug!("Stopping HTTP server");
            self.server_running = false;
        self.data.clear();
        Ok(())
    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    fn name(&self) -> &str {
        &self.name
    }
}

impl MemoryFixture {
    /// Create new memory fixture
    pub fn new(name: String) -> Self {
        Self {
        }
    }

    /// Store a value in memory
    pub fn store(&self, key: String, value: TestValue) -> TestingResult<()> {
        if let Ok(mut store) = self.data_store.lock() {
            store.insert(key, value);
            Ok(())
        } else {
            Err(TestError::Framework("Failed to acquire memory store lock".to_string()))
        }
    }

    /// Retrieve a value from memory
    pub fn retrieve(&self, key: &str) -> TestingResult<Option<TestValue>> {
        if let Ok(store) = self.data_store.lock() {
            Ok(store.get(key).cloned())
        } else {
            Err(TestError::Framework("Failed to acquire memory store lock".to_string()))
        }
    }
impl TestFixture for MemoryFixture {
    fn setup(&mut self) -> TestingResult<()> {
        info!("Setting up memory fixture: {}", self.name);
        
        self.data.insert("status".to_string(), TestValue::String("initialized".to_string()));
        Ok(())
    fn teardown(&mut self) -> TestingResult<()> {
        info!("Tearing down memory fixture: {}", self.name);
        
        if let Ok(mut store) = self.data_store.lock() {
            store.clear();
        self.data.clear();
        Ok(())
    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    fn name(&self) -> &str {
        &self.name
    }
}

