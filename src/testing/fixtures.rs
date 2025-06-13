/// Test Fixtures System
/// 
/// Provides test data management, setup/teardown functionality,
/// and reusable test components for CURSED tests.

use super::{TestError, TestResult};
use super::framework::{TestFixture, TestValue};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn};

/// Test data container for structured test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestData {
    /// Test data values
    pub values: HashMap<String, TestValue>,
    /// Test data metadata
    pub metadata: TestDataMetadata,
}

/// Metadata for test data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataMetadata {
    /// Data source description
    pub source: String,
    /// Data version
    pub version: String,
    /// Creation timestamp
    pub created_at: String,
    /// Data tags/categories
    pub tags: Vec<String>,
}

/// Fixture manager handles lifecycle of test fixtures
pub struct FixtureManager {
    /// Registered fixtures
    fixtures: HashMap<String, Box<dyn TestFixture>>,
    /// Fixture dependencies
    dependencies: HashMap<String, Vec<String>>,
    /// Active fixtures (initialized)
    active_fixtures: HashMap<String, bool>,
    /// Fixture configuration
    config: FixtureManagerConfig,
}

/// Configuration for fixture manager
#[derive(Debug, Clone)]
pub struct FixtureManagerConfig {
    /// Maximum setup time per fixture
    pub max_setup_time_seconds: u64,
    /// Maximum teardown time per fixture
    pub max_teardown_time_seconds: u64,
    /// Enable parallel fixture setup
    pub parallel_setup: bool,
    /// Fixture data directory
    pub data_directory: Option<PathBuf>,
    /// Enable fixture caching
    pub enable_caching: bool,
}

impl Default for FixtureManagerConfig {
    fn default() -> Self {
        Self {
            max_setup_time_seconds: 30,
            max_teardown_time_seconds: 10,
            parallel_setup: true,
            data_directory: None,
            enable_caching: true,
        }
    }
}

/// Database fixture for test databases
#[derive(Debug)]
pub struct DatabaseFixture {
    /// Fixture name
    name: String,
    /// Database connection string
    connection_string: String,
    /// Database schema files
    schema_files: Vec<PathBuf>,
    /// Test data files
    data_files: Vec<PathBuf>,
    /// Fixture data
    data: HashMap<String, TestValue>,
    /// Connection state
    connected: bool,
}

/// File system fixture for temporary files and directories
#[derive(Debug)]
pub struct FileSystemFixture {
    /// Fixture name
    name: String,
    /// Base temporary directory
    temp_dir: Option<tempfile::TempDir>,
    /// Created files and directories
    created_paths: Vec<PathBuf>,
    /// Fixture data
    data: HashMap<String, TestValue>,
}

/// HTTP server fixture for integration tests
#[derive(Debug)]
pub struct HttpServerFixture {
    /// Fixture name
    name: String,
    /// Server port
    port: u16,
    /// Server host
    host: String,
    /// Mock responses
    mock_responses: HashMap<String, MockResponse>,
    /// Fixture data
    data: HashMap<String, TestValue>,
    /// Server state
    server_running: bool,
}

/// Mock HTTP response
#[derive(Debug, Clone)]
pub struct MockResponse {
    /// Response status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: String,
}

/// Memory fixture for in-memory data structures
#[derive(Debug)]
pub struct MemoryFixture {
    /// Fixture name
    name: String,
    /// In-memory data store
    data_store: Arc<Mutex<HashMap<String, TestValue>>>,
    /// Fixture data
    data: HashMap<String, TestValue>,
}

impl TestData {
    /// Create new test data
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            metadata: TestDataMetadata {
                source: "manual".to_string(),
                version: "1.0".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                tags: vec![],
            },
        }
    }

    /// Load test data from JSON file
    pub fn from_json_file(path: &Path) -> TestResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| TestError::Io(format!("Failed to read test data file: {}", e)))?;
        
        let data: TestData = serde_json::from_str(&content)
            .map_err(|e| TestError::General(format!("Failed to parse test data JSON: {}", e)))?;
        
        Ok(data)
    }

    /// Save test data to JSON file
    pub fn to_json_file(&self, path: &Path) -> TestResult<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| TestError::General(format!("Failed to serialize test data: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| TestError::Io(format!("Failed to write test data file: {}", e)))?;
        
        Ok(())
    }

    /// Set a value
    pub fn set(&mut self, key: String, value: TestValue) {
        self.values.insert(key, value);
    }

    /// Get a value
    pub fn get(&self, key: &str) -> Option<&TestValue> {
        self.values.get(key)
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
        }
    }
}

impl FixtureManager {
    /// Create new fixture manager
    pub fn new() -> Self {
        Self {
            fixtures: HashMap::new(),
            dependencies: HashMap::new(),
            active_fixtures: HashMap::new(),
            config: FixtureManagerConfig::default(),
        }
    }

    /// Create fixture manager with configuration
    pub fn with_config(config: FixtureManagerConfig) -> Self {
        Self {
            fixtures: HashMap::new(),
            dependencies: HashMap::new(),
            active_fixtures: HashMap::new(),
            config,
        }
    }

    /// Register a fixture
    pub fn register_fixture(&mut self, name: String, fixture: Box<dyn TestFixture>) -> TestResult<()> {
        info!("Registering fixture: {}", name);
        self.fixtures.insert(name.clone(), fixture);
        self.active_fixtures.insert(name, false);
        Ok(())
    }

    /// Register fixture dependencies
    pub fn register_dependencies(&mut self, name: String, dependencies: Vec<String>) -> TestResult<()> {
        debug!("Registering dependencies for {}: {:?}", name, dependencies);
        self.dependencies.insert(name, dependencies);
        Ok(())
    }

    /// Setup all fixtures in dependency order
    pub fn setup_all_fixtures(&mut self) -> TestResult<()> {
        info!("Setting up all fixtures");
        
        let setup_order = self.calculate_setup_order()?;
        
        for fixture_name in setup_order {
            self.setup_fixture(&fixture_name)?;
        }
        
        info!("All fixtures setup completed");
        Ok(())
    }

    /// Setup a specific fixture
    pub fn setup_fixture(&mut self, name: &str) -> TestResult<()> {
        if self.active_fixtures.get(name) == Some(&true) {
            debug!("Fixture {} already active", name);
            return Ok(());
        }

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
        }

        Ok(())
    }

    /// Teardown all fixtures in reverse dependency order
    pub fn teardown_all_fixtures(&mut self) -> TestResult<()> {
        info!("Tearing down all fixtures");
        
        let mut teardown_order = self.calculate_setup_order()?;
        teardown_order.reverse();
        
        for fixture_name in teardown_order {
            self.teardown_fixture(&fixture_name)?;
        }
        
        info!("All fixtures teardown completed");
        Ok(())
    }

    /// Teardown a specific fixture
    pub fn teardown_fixture(&mut self, name: &str) -> TestResult<()> {
        if self.active_fixtures.get(name) == Some(&false) {
            debug!("Fixture {} already inactive", name);
            return Ok(());
        }

        info!("Tearing down fixture: {}", name);
        
        if let Some(fixture) = self.fixtures.get_mut(name) {
            fixture.teardown()?;
            self.active_fixtures.insert(name.to_string(), false);
            info!("Fixture {} teardown completed", name);
        } else {
            warn!("Fixture not found during teardown: {}", name);
        }

        Ok(())
    }

    /// Get fixture data
    pub fn get_fixture_data(&self, name: &str) -> TestResult<HashMap<String, TestValue>> {
        if let Some(fixture) = self.fixtures.get(name) {
            Ok(fixture.get_data())
        } else {
            Err(TestError::Framework(format!("Fixture not found: {}", name)))
        }
    }

    /// Calculate setup order based on dependencies
    fn calculate_setup_order(&self) -> TestResult<Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();
        let mut order = Vec::new();

        for fixture_name in self.fixtures.keys() {
            if !visited.contains(fixture_name) {
                self.dfs_visit(fixture_name, &mut visited, &mut temp_visited, &mut order)?;
            }
        }

        Ok(order)
    }

    /// Depth-first search visit for topological sort
    fn dfs_visit(
        &self,
        name: &str,
        visited: &mut std::collections::HashSet<String>,
        temp_visited: &mut std::collections::HashSet<String>,
        order: &mut Vec<String>,
    ) -> TestResult<()> {
        if temp_visited.contains(name) {
            return Err(TestError::Framework(format!("Circular dependency detected involving: {}", name)));
        }

        if visited.contains(name) {
            return Ok(());
        }

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
            name,
            connection_string,
            schema_files: Vec::new(),
            data_files: Vec::new(),
            data: HashMap::new(),
            connected: false,
        }
    }

    /// Add schema file
    pub fn with_schema_file(mut self, path: PathBuf) -> Self {
        self.schema_files.push(path);
        self
    }

    /// Add data file
    pub fn with_data_file(mut self, path: PathBuf) -> Self {
        self.data_files.push(path);
        self
    }
}

impl TestFixture for DatabaseFixture {
    fn setup(&mut self) -> TestResult<()> {
        info!("Setting up database fixture: {}", self.name);
        
        // Simulate database connection and setup
        debug!("Connecting to database: {}", self.connection_string);
        self.connected = true;
        
        // Load schema files
        for schema_file in &self.schema_files {
            debug!("Loading schema file: {}", schema_file.display());
        }
        
        // Load data files
        for data_file in &self.data_files {
            debug!("Loading data file: {}", data_file.display());
        }
        
        self.data.insert("connection_string".to_string(), TestValue::String(self.connection_string.clone()));
        self.data.insert("status".to_string(), TestValue::String("connected".to_string()));
        
        Ok(())
    }

    fn teardown(&mut self) -> TestResult<()> {
        info!("Tearing down database fixture: {}", self.name);
        
        if self.connected {
            debug!("Disconnecting from database");
            self.connected = false;
        }
        
        self.data.clear();
        Ok(())
    }

    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    }

    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl FileSystemFixture {
    /// Create new filesystem fixture
    pub fn new(name: String) -> Self {
        Self {
            name,
            temp_dir: None,
            created_paths: Vec::new(),
            data: HashMap::new(),
        }
    }

    /// Create a file with content
    pub fn create_file(&mut self, relative_path: &str, content: &str) -> TestResult<PathBuf> {
        let temp_dir = self.temp_dir.as_ref()
            .ok_or_else(|| TestError::Framework("Fixture not setup".to_string()))?;
        
        let file_path = temp_dir.path().join(relative_path);
        
        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| TestError::Io(format!("Failed to create directories: {}", e)))?;
        }
        
        std::fs::write(&file_path, content)
            .map_err(|e| TestError::Io(format!("Failed to write file: {}", e)))?;
        
        self.created_paths.push(file_path.clone());
        Ok(file_path)
    }

    /// Get temporary directory path
    pub fn temp_path(&self) -> Option<&Path> {
        self.temp_dir.as_ref().map(|d| d.path())
    }
}

impl TestFixture for FileSystemFixture {
    fn setup(&mut self) -> TestResult<()> {
        info!("Setting up filesystem fixture: {}", self.name);
        
        let temp_dir = tempfile::TempDir::new()
            .map_err(|e| TestError::Io(format!("Failed to create temp directory: {}", e)))?;
        
        let temp_path = temp_dir.path().to_string_lossy().to_string();
        self.data.insert("temp_dir".to_string(), TestValue::String(temp_path));
        
        self.temp_dir = Some(temp_dir);
        Ok(())
    }

    fn teardown(&mut self) -> TestResult<()> {
        info!("Tearing down filesystem fixture: {}", self.name);
        
        self.created_paths.clear();
        self.temp_dir = None;
        self.data.clear();
        Ok(())
    }

    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    }

    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl HttpServerFixture {
    /// Create new HTTP server fixture
    pub fn new(name: String, port: u16) -> Self {
        Self {
            name,
            port,
            host: "localhost".to_string(),
            mock_responses: HashMap::new(),
            data: HashMap::new(),
            server_running: false,
        }
    }

    /// Add mock response
    pub fn add_mock_response(&mut self, path: String, response: MockResponse) {
        self.mock_responses.insert(path, response);
    }

    /// Get server URL
    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

impl TestFixture for HttpServerFixture {
    fn setup(&mut self) -> TestResult<()> {
        info!("Setting up HTTP server fixture: {} on port {}", self.name, self.port);
        
        // Simulate server startup
        self.server_running = true;
        
        let server_url = self.server_url();
        self.data.insert("server_url".to_string(), TestValue::String(server_url));
        self.data.insert("port".to_string(), TestValue::Integer(self.port as i64));
        self.data.insert("status".to_string(), TestValue::String("running".to_string()));
        
        Ok(())
    }

    fn teardown(&mut self) -> TestResult<()> {
        info!("Tearing down HTTP server fixture: {}", self.name);
        
        if self.server_running {
            debug!("Stopping HTTP server");
            self.server_running = false;
        }
        
        self.data.clear();
        Ok(())
    }

    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    }

    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl MemoryFixture {
    /// Create new memory fixture
    pub fn new(name: String) -> Self {
        Self {
            name,
            data_store: Arc::new(Mutex::new(HashMap::new())),
            data: HashMap::new(),
        }
    }

    /// Store a value in memory
    pub fn store(&self, key: String, value: TestValue) -> TestResult<()> {
        if let Ok(mut store) = self.data_store.lock() {
            store.insert(key, value);
            Ok(())
        } else {
            Err(TestError::Framework("Failed to acquire memory store lock".to_string()))
        }
    }

    /// Retrieve a value from memory
    pub fn retrieve(&self, key: &str) -> TestResult<Option<TestValue>> {
        if let Ok(store) = self.data_store.lock() {
            Ok(store.get(key).cloned())
        } else {
            Err(TestError::Framework("Failed to acquire memory store lock".to_string()))
        }
    }
}

impl TestFixture for MemoryFixture {
    fn setup(&mut self) -> TestResult<()> {
        info!("Setting up memory fixture: {}", self.name);
        
        self.data.insert("status".to_string(), TestValue::String("initialized".to_string()));
        Ok(())
    }

    fn teardown(&mut self) -> TestResult<()> {
        info!("Tearing down memory fixture: {}", self.name);
        
        if let Ok(mut store) = self.data_store.lock() {
            store.clear();
        }
        
        self.data.clear();
        Ok(())
    }

    fn get_data(&self) -> HashMap<String, TestValue> {
        self.data.clone()
    }

    fn set_data(&mut self, key: String, value: TestValue) {
        self.data.insert(key, value);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_data_creation() {
        let mut data = TestData::new();
        data.set("key1".to_string(), TestValue::String("value1".to_string()));
        data.add_tag("test".to_string());
        
        assert!(data.get("key1").is_some());
        assert!(data.metadata.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_fixture_manager() {
        let mut manager = FixtureManager::new();
        
        let fixture = Box::new(MemoryFixture::new("test_memory".to_string()));
        assert!(manager.register_fixture("test_memory".to_string(), fixture).is_ok());
        
        assert!(manager.setup_fixture("test_memory").is_ok());
        assert!(manager.teardown_fixture("test_memory").is_ok());
    }

    #[test]
    fn test_filesystem_fixture() {
        let mut fixture = FileSystemFixture::new("test_fs".to_string());
        
        assert!(fixture.setup().is_ok());
        assert!(fixture.temp_path().is_some());
        
        let file_path = fixture.create_file("test.txt", "test content");
        assert!(file_path.is_ok());
        
        assert!(fixture.teardown().is_ok());
    }

    #[test]
    fn test_dependency_resolution() {
        let mut manager = FixtureManager::new();
        
        // Register fixtures
        let fixture1 = Box::new(MemoryFixture::new("fixture1".to_string()));
        let fixture2 = Box::new(MemoryFixture::new("fixture2".to_string()));
        
        manager.register_fixture("fixture1".to_string(), fixture1).unwrap();
        manager.register_fixture("fixture2".to_string(), fixture2).unwrap();
        
        // Register dependencies: fixture2 depends on fixture1
        manager.register_dependencies("fixture2".to_string(), vec!["fixture1".to_string()]).unwrap();
        
        let order = manager.calculate_setup_order().unwrap();
        
        // fixture1 should come before fixture2 in setup order
        let pos1 = order.iter().position(|x| x == "fixture1").unwrap();
        let pos2 = order.iter().position(|x| x == "fixture2").unwrap();
        assert!(pos1 < pos2);
    }
}
