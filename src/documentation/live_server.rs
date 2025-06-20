//! # Live Documentation Server
//!
//! Real-time documentation server with hot reload capabilities for the CURSED programming language.
//! This module provides a WebSocket-enabled HTTP server that automatically regenerates and refreshes
//! documentation when source files change.
//!
//! ## Features
//!
//! - **Hot Reload**: Automatic regeneration on file changes with instant browser refresh
//! - **WebSocket Integration**: Real-time communication between server and browser
//! - **Interactive Features**: Code playground, executable examples, API explorer
//! - **Performance Monitoring**: Real-time metrics and generation statistics
//! - **Error Recovery**: Graceful handling of generation failures with user feedback
//! - **Multi-Format Support**: Serves multiple documentation formats simultaneously
//!
//! ## Example Usage
//!
//! ```rust
//! use cursed::documentation::{LiveDocumentationServer, LiveServerConfig};
//! use std::time::Duration;
//!
//! // Create server configuration
//! let config = LiveServerConfig {
//!     port: 8080,
//!     watch_debounce: Duration::from_millis(300),
//!     enable_playground: true,
//!     enable_api_explorer: true,
//!     ..Default::default()
//! };
//!
//! // Start the live documentation server
//! let mut server = LiveDocumentationServer::new(config)?;
//! server.start_serving(&["./src"], "./docs").await?;
//! ```

use crate::build_system::file_watcher::{FileWatcher, WatchConfig, FileWatchEvent, FileWatcherBuilder};
use crate::documentation::{DocumentationGenerator, DocumentationConfig, DocGeneratorConfig, DocFormat};
use crate::error::Error as CursedError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{broadcast, mpsc};
use tokio::time::timeout;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::{Filter, Rejection, Reply};

pub type LiveServerResult<T> = Result<T, CursedError>;

/// Configuration for the live documentation server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveServerConfig {
    /// HTTP server port
    pub port: u16,
    
    /// Host address to bind to
    pub host: String,
    
    /// Watch debounce duration to prevent rapid regeneration
    pub watch_debounce: Duration,
    
    /// Maximum time to wait for documentation generation
    pub generation_timeout: Duration,
    
    /// Enable interactive code playground
    pub enable_playground: bool,
    
    /// Enable API explorer functionality
    pub enable_api_explorer: bool,
    
    /// Enable syntax highlighting in code blocks
    pub enable_syntax_highlighting: bool,
    
    /// Enable code folding in documentation
    pub enable_code_folding: bool,
    
    /// Maximum number of concurrent WebSocket connections
    pub max_websocket_connections: usize,
    
    /// Auto-open browser on startup
    pub auto_open_browser: bool,
    
    /// Custom CSS file path
    pub custom_css_path: Option<PathBuf>,
    
    /// Custom JavaScript file path
    pub custom_js_path: Option<PathBuf>,
    
    /// Documentation formats to generate
    pub output_formats: Vec<DocFormat>,
    
    /// Include private items in documentation
    pub include_private: bool,
    
    /// Include source code in documentation
    pub include_source: bool,
    
    /// Generate cross-references between items
    pub generate_cross_refs: bool,
    
    /// Include executable examples
    pub include_examples: bool,
    
    /// CORS origins to allow
    pub cors_origins: Vec<String>,
}

impl Default for LiveServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            watch_debounce: Duration::from_millis(500),
            generation_timeout: Duration::from_secs(30),
            enable_playground: true,
            enable_api_explorer: true,
            enable_syntax_highlighting: true,
            enable_code_folding: true,
            max_websocket_connections: 100,
            auto_open_browser: true,
            custom_css_path: None,
            custom_js_path: None,
            output_formats: vec![DocFormat::Html],
            include_private: false,
            include_source: true,
            generate_cross_refs: true,
            include_examples: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// WebSocket message types for client-server communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Documentation has been regenerated
    DocumentationUpdated {
        timestamp: SystemTime,
        files_changed: Vec<String>,
        generation_time_ms: u64,
    },
    
    /// Generation started
    GenerationStarted {
        timestamp: SystemTime,
        trigger: String,
    },
    
    /// Generation failed
    GenerationFailed {
        timestamp: SystemTime,
        error: String,
        files_affected: Vec<String>,
    },
    
    /// Server statistics update
    ServerStats {
        timestamp: SystemTime,
        connected_clients: usize,
        total_regenerations: u64,
        average_generation_time_ms: u64,
        uptime_seconds: u64,
    },
    
    /// Code execution request (for playground)
    ExecuteCode {
        code: String,
        language: String,
        session_id: String,
    },
    
    /// Code execution result
    ExecutionResult {
        session_id: String,
        success: bool,
        output: String,
        error: Option<String>,
        execution_time_ms: u64,
    },
    
    /// API method call (for API explorer)
    ApiCall {
        method_name: String,
        parameters: HashMap<String, serde_json::Value>,
        session_id: String,
    },
    
    /// API call result
    ApiResult {
        session_id: String,
        success: bool,
        result: serde_json::Value,
        error: Option<String>,
    },
    
    /// Client ping
    Ping {
        timestamp: SystemTime,
    },
    
    /// Server pong
    Pong {
        timestamp: SystemTime,
    },
    
    /// Connection established
    Connected {
        server_version: String,
        features: Vec<String>,
    },
}

/// Real-time server statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatistics {
    /// Server start time
    pub started_at: SystemTime,
    
    /// Number of currently connected WebSocket clients
    pub connected_clients: usize,
    
    /// Total number of documentation regenerations
    pub total_regenerations: u64,
    
    /// Average generation time in milliseconds
    pub average_generation_time_ms: u64,
    
    /// Total generation time in milliseconds
    pub total_generation_time_ms: u64,
    
    /// Number of successful generations
    pub successful_generations: u64,
    
    /// Number of failed generations
    pub failed_generations: u64,
    
    /// Total files processed
    pub total_files_processed: u64,
    
    /// Last generation timestamp
    pub last_generation: Option<SystemTime>,
    
    /// Current generation in progress
    pub generation_in_progress: bool,
    
    /// Files currently being watched
    pub watched_files: usize,
    
    /// Recent error messages
    pub recent_errors: Vec<String>,
}

impl Default for ServerStatistics {
    fn default() -> Self {
        Self {
            started_at: SystemTime::now(),
            connected_clients: 0,
            total_regenerations: 0,
            average_generation_time_ms: 0,
            total_generation_time_ms: 0,
            successful_generations: 0,
            failed_generations: 0,
            total_files_processed: 0,
            last_generation: None,
            generation_in_progress: false,
            watched_files: 0,
            recent_errors: Vec::new(),
        }
    }
}

/// WebSocket client connection info
#[derive(Debug, Clone)]
pub struct ClientConnection {
    /// Unique client ID
    pub id: Uuid,
    
    /// Connection timestamp
    pub connected_at: SystemTime,
    
    /// Last ping timestamp
    pub last_ping: Option<SystemTime>,
    
    /// Client user agent
    pub user_agent: Option<String>,
    
    /// Features enabled for this client
    pub features: HashSet<String>,
}

/// Live documentation server with hot reload capabilities
pub struct LiveDocumentationServer {
    /// Server configuration
    config: LiveServerConfig,
    
    /// File watcher for detecting changes
    file_watcher: Option<FileWatcher>,
    
    /// Documentation generator
    doc_generator: Option<DocumentationGenerator>,
    
    /// WebSocket broadcast channel for sending updates
    websocket_tx: broadcast::Sender<WebSocketMessage>,
    
    /// Connected WebSocket clients
    clients: Arc<RwLock<HashMap<Uuid, ClientConnection>>>,
    
    /// Server statistics
    statistics: Arc<Mutex<ServerStatistics>>,
    
    /// Currently watched source paths
    watched_paths: Arc<RwLock<Vec<PathBuf>>>,
    
    /// Output documentation directory
    output_dir: Arc<RwLock<Option<PathBuf>>>,
    
    /// Generation task queue to prevent concurrent generations
    generation_queue: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    
    /// Server state
    is_running: Arc<Mutex<bool>>,
}

impl LiveDocumentationServer {
    /// Create a new live documentation server
    #[instrument(skip(config))]
    pub fn new(config: LiveServerConfig) -> LiveServerResult<Self> {
        info!("Creating live documentation server on {}:{}", config.host, config.port);
        
        let (websocket_tx, _) = broadcast::channel(1000);
        
        Ok(Self {
            config,
            file_watcher: None,
            doc_generator: None,
            websocket_tx,
            clients: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(ServerStatistics::default())),
            watched_paths: Arc::new(RwLock::new(Vec::new())),
            output_dir: Arc::new(RwLock::new(None)),
            generation_queue: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Start serving documentation with hot reload
    #[instrument(skip(self, source_paths))]
    pub async fn start_serving<P: AsRef<Path>>(
        &mut self,
        source_paths: &[P],
        output_dir: P,
    ) -> LiveServerResult<()> {
        let output_path = output_dir.as_ref().to_path_buf();
        
        info!("Starting live documentation server");
        info!("Source paths: {:?}", source_paths.iter().map(|p| p.as_ref()).collect::<Vec<_>>());
        info!("Output directory: {}", output_path.display());
        
        // Update server state
        {
            let mut is_running = self.is_running.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire running lock"))?;
            *is_running = true;
        }
        
        {
            let mut output_dir_guard = self.output_dir.write()
                .map_err(|_| CursedError::system_error("Failed to acquire output dir lock"))?;
            *output_dir_guard = Some(output_path.clone());
        }
        
        {
            let mut watched_paths = self.watched_paths.write()
                .map_err(|_| CursedError::system_error("Failed to acquire watched paths lock"))?;
            watched_paths.clear();
            for path in source_paths {
                watched_paths.push(path.as_ref().to_path_buf());
            }
        }
        
        // Initialize documentation generator
        self.initialize_documentation_generator(&output_path)?;
        
        // Setup file watcher
        self.setup_file_watcher(source_paths).await?;
        
        // Generate initial documentation
        self.generate_initial_documentation().await?;
        
        // Start HTTP and WebSocket server
        self.start_http_server(&output_path).await?;
        
        Ok(())
    }
    
    /// Stop the live documentation server
    #[instrument(skip(self))]
    pub async fn stop_serving(&mut self) -> LiveServerResult<()> {
        info!("Stopping live documentation server");
        
        // Update running state
        {
            let mut is_running = self.is_running.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire running lock"))?;
            *is_running = false;
        }
        
        // Stop file watcher
        if let Some(mut watcher) = self.file_watcher.take() {
            watcher.stop_watching()
                .map_err(|e| CursedError::system_error(&format!("Failed to stop file watcher: {}", e)))?;
        }
        
        // Cancel any pending generation
        {
            let mut generation_queue = self.generation_queue.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire generation queue lock"))?;
            if let Some(handle) = generation_queue.take() {
                handle.abort();
            }
        }
        
        // Notify all connected clients
        let _ = self.websocket_tx.send(WebSocketMessage::Ping { 
            timestamp: SystemTime::now() 
        });
        
        // Clear client connections
        {
            let mut clients = self.clients.write()
                .map_err(|_| CursedError::system_error("Failed to acquire clients lock"))?;
            clients.clear();
        }
        
        info!("Live documentation server stopped");
        Ok(())
    }
    
    /// Initialize the documentation generator
    #[instrument(skip(self, output_dir))]
    fn initialize_documentation_generator(&mut self, output_dir: &Path) -> LiveServerResult<()> {
        let doc_config = DocumentationConfig {
            input_dirs: Vec::new(), // Will be set when generating
            output_dir: output_dir.to_path_buf(),
            formats: self.config.output_formats.clone(),
            options: crate::documentation::DocOptions {
                include_private: self.config.include_private,
                include_source: self.config.include_source,
                generate_cross_refs: self.config.generate_cross_refs,
                include_examples: self.config.include_examples,
                max_type_depth: 10,
                custom_css: self.config.custom_css_path.clone(),
                template_dir: None,
            },
            metadata: crate::documentation::DocMetadata {
                title: "CURSED Documentation".to_string(),
                description: Some("Live documentation generated by CURSED".to_string()),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                authors: vec!["CURSED Live Server".to_string()],
                base_url: Some(format!("http://{}:{}", self.config.host, self.config.port)),
            },
        };
        
        self.doc_generator = Some(DocumentationGenerator::new(doc_config)?);
        
        info!("Documentation generator initialized");
        Ok(())
    }
    
    /// Setup file watcher for source files
    #[instrument(skip(self, source_paths))]
    async fn setup_file_watcher<P: AsRef<Path>>(&mut self, source_paths: &[P]) -> LiveServerResult<()> {
        let watch_config = WatchConfig {
            watch_patterns: vec![
                "*.csd".to_string(),
                "*.toml".to_string(),
                "*.md".to_string(),
                "Makefile".to_string(),
            ],
            ignore_patterns: vec![
                "*.tmp".to_string(),
                "*.bak".to_string(),
                "target/*".to_string(),
                ".git/*".to_string(),
                ".devenv/*".to_string(),
                "coverage/*".to_string(),
                "docs/*".to_string(), // Ignore output directory
            ],
            debounce_duration: self.config.watch_debounce,
            max_batch_size: 50,
            recursive: true,
            follow_symlinks: false,
            event_buffer_size: 1000,
        };
        
        let mut file_watcher = FileWatcher::new(watch_config)
            .map_err(|e| CursedError::system_error(&format!("Failed to create file watcher: {}", e)))?;
        
        // Setup event callback
        let websocket_tx = self.websocket_tx.clone();
        let generation_queue = Arc::clone(&self.generation_queue);
        let statistics = Arc::clone(&self.statistics);
        let output_dir = Arc::clone(&self.output_dir);
        let watched_paths_clone = Arc::clone(&self.watched_paths);
        let config = self.config.clone();
        
        file_watcher.set_event_callback(move |event| {
            let websocket_tx = websocket_tx.clone();
            let generation_queue = Arc::clone(&generation_queue);
            let statistics = Arc::clone(&statistics);
            let output_dir = Arc::clone(&output_dir);
            let watched_paths = Arc::clone(&watched_paths_clone);
            let config = config.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_file_change_event(
                    event,
                    websocket_tx,
                    generation_queue,
                    statistics,
                    output_dir,
                    watched_paths,
                    config,
                ).await {
                    error!("Failed to handle file change event: {}", e);
                }
            });
        }).map_err(|e| CursedError::system_error(&format!("Failed to set file watcher callback: {}", e)))?;
        
        // Start watching
        file_watcher.start_watching(source_paths)
            .map_err(|e| CursedError::system_error(&format!("Failed to start file watching: {}", e)))?;
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire statistics lock"))?;
            stats.watched_files = file_watcher.get_watched_paths().len();
        }
        
        self.file_watcher = Some(file_watcher);
        
        info!("File watcher setup complete");
        Ok(())
    }
    
    /// Handle file change events
    #[instrument(skip(websocket_tx, generation_queue, statistics, output_dir, watched_paths, config))]
    async fn handle_file_change_event(
        event: FileWatchEvent,
        websocket_tx: broadcast::Sender<WebSocketMessage>,
        generation_queue: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
        statistics: Arc<Mutex<ServerStatistics>>,
        output_dir: Arc<RwLock<Option<PathBuf>>>,
        watched_paths: Arc<RwLock<Vec<PathBuf>>>,
        config: LiveServerConfig,
    ) -> LiveServerResult<()> {
        // Check if this event should trigger regeneration
        if !event.should_trigger_rebuild() {
            return Ok(());
        }
        
        let changed_files = match &event {
            FileWatchEvent::Batch { events, .. } => {
                events.iter().map(|e| e.path().to_string_lossy().to_string()).collect()
            }
            _ => vec![event.path().to_string_lossy().to_string()],
        };
        
        info!("File change detected: {:?}", changed_files);
        
        // Notify clients that generation is starting
        let _ = websocket_tx.send(WebSocketMessage::GenerationStarted {
            timestamp: SystemTime::now(),
            trigger: format!("File change: {}", changed_files.join(", ")),
        });
        
        // Update statistics
        {
            let mut stats = statistics.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire statistics lock"))?;
            stats.generation_in_progress = true;
        }
        
        // Cancel any existing generation
        {
            let mut queue = generation_queue.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire generation queue lock"))?;
            if let Some(handle) = queue.take() {
                handle.abort();
            }
        }
        
        // Start new generation task
        let generation_task = {
            let websocket_tx = websocket_tx.clone();
            let statistics = Arc::clone(&statistics);
            let output_dir = Arc::clone(&output_dir);
            let watched_paths = Arc::clone(&watched_paths);
            let changed_files = changed_files.clone();
            let config = config.clone();
            
            tokio::spawn(async move {
                let start_time = Instant::now();
                
                // Perform the documentation generation
                let result = Self::regenerate_documentation(
                    &output_dir,
                    &watched_paths,
                    &config,
                ).await;
                
                let generation_time = start_time.elapsed();
                
                // Update statistics and notify clients
                {
                    let mut stats = statistics.lock().unwrap();
                    stats.generation_in_progress = false;
                    stats.total_regenerations += 1;
                    stats.total_generation_time_ms += generation_time.as_millis() as u64;
                    stats.average_generation_time_ms = 
                        stats.total_generation_time_ms / stats.total_regenerations;
                    stats.last_generation = Some(SystemTime::now());
                    
                    match &result {
                        Ok(_) => {
                            stats.successful_generations += 1;
                            
                            // Notify clients of successful generation
                            let _ = websocket_tx.send(WebSocketMessage::DocumentationUpdated {
                                timestamp: SystemTime::now(),
                                files_changed: changed_files,
                                generation_time_ms: generation_time.as_millis() as u64,
                            });
                        }
                        Err(error) => {
                            stats.failed_generations += 1;
                            let error_msg = error.to_string();
                            
                            // Keep only last 10 errors
                            stats.recent_errors.push(error_msg.clone());
                            if stats.recent_errors.len() > 10 {
                                stats.recent_errors.remove(0);
                            }
                            
                            // Notify clients of failed generation
                            let _ = websocket_tx.send(WebSocketMessage::GenerationFailed {
                                timestamp: SystemTime::now(),
                                error: error_msg,
                                files_affected: changed_files,
                            });
                        }
                    }
                }
                
                result
            })
        };
        
        // Store the generation task
        {
            let mut queue = generation_queue.lock()
                .map_err(|_| CursedError::system_error("Failed to acquire generation queue lock"))?;
            *queue = Some(generation_task);
        }
        
        Ok(())
    }
    
    /// Regenerate documentation
    #[instrument(skip(output_dir, watched_paths, config))]
    async fn regenerate_documentation(
        output_dir: &Arc<RwLock<Option<PathBuf>>>,
        watched_paths: &Arc<RwLock<Vec<PathBuf>>>,
        config: &LiveServerConfig,
    ) -> LiveServerResult<()> {
        let output_path = {
            let output_guard = output_dir.read()
                .map_err(|_| CursedError::system_error("Failed to acquire output dir lock"))?;
            output_guard.clone()
                .ok_or_else(|| CursedError::system_error("Output directory not set"))?
        };
        
        let source_paths = {
            let watched_guard = watched_paths.read()
                .map_err(|_| CursedError::system_error("Failed to acquire watched paths lock"))?;
            watched_guard.clone()
        };
        
        // Create new documentation generator for this generation
        let doc_config = DocumentationConfig {
            source_dirs: source_paths,
            output_dir: output_path,
            output_formats: config.output_formats.clone(),
            options: crate::documentation::DocOptions {
                generate_search_index: true,
                include_dependencies: false,
                generate_cross_refs: true,
                include_examples: true,
                include_private: false,
                max_depth: 10,
                theme: "default".to_string(),
            },
            project: crate::documentation::ProjectMetadata {
                name: "CURSED Documentation".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: Some("Live documentation generated by CURSED".to_string()),
                authors: vec!["CURSED Live Server".to_string()],
                homepage: Some(format!("http://{}:{}", config.host, config.port)),
                repository: None,
                license: None,
            },
            styling: crate::documentation::StylingConfig {
                custom_css: config.custom_css_path.clone().map(|p| vec![p]).unwrap_or_default(),
                template_dir: None,
                theme: "light".to_string(),
                // Note: These fields are not available in current StylingConfig
            },
        };
        
        let mut generator = DocumentationGenerator::new(doc_config)?;
        
        // Generate with timeout
        timeout(
            config.generation_timeout,
            async {
                let extracted_docs = Vec::new(); // TODO: Extract docs from source_dirs
                generator.generate_output(&extracted_docs).await?;
                Ok::<(), CursedError>(())
            }
        ).await
        .map_err(|_| CursedError::system_error("Documentation generation timed out"))?
        .map_err(|e| CursedError::system_error(&format!("Documentation generation failed: {}", e)))?;
        
        info!("Documentation regenerated successfully");
        Ok(())
    }
    
    /// Generate initial documentation
    #[instrument(skip(self))]
    async fn generate_initial_documentation(&mut self) -> LiveServerResult<()> {
        info!("Generating initial documentation");
        
        let output_dir = Arc::clone(&self.output_dir);
        let watched_paths = Arc::clone(&self.watched_paths);
        let config = self.config.clone();
        
        Self::regenerate_documentation(&output_dir, &watched_paths, &config).await?;
        
        info!("Initial documentation generated");
        Ok(())
    }
    
    /// Start HTTP and WebSocket server
    #[instrument(skip(self, docs_dir))]
    async fn start_http_server(&self, docs_dir: &Path) -> LiveServerResult<()> {
        if !docs_dir.exists() {
            return Err(CursedError::system_error(&format!(
                "Documentation directory does not exist: {}", 
                docs_dir.display()
            )));
        }
        
        let websocket_tx = self.websocket_tx.clone();
        let clients = Arc::clone(&self.clients);
        let statistics = Arc::clone(&self.statistics);
        let config = self.config.clone();
        
        // WebSocket route
        let websocket_route = warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map(move || websocket_tx.clone()))
            .and(warp::any().map(move || Arc::clone(&clients)))
            .and(warp::any().map(move || Arc::clone(&statistics)))
            .and(warp::any().map(move || config.clone()))
            .map(|ws: warp::ws::Ws, tx, clients, stats, config| {
                ws.on_upgrade(move |socket| {
                    Self::handle_websocket_connection(socket, tx, clients, stats, config)
                })
            });
        
        // API routes for interactive features
        let api_routes = self.create_api_routes();
        
        // Static file serving
        let docs_path = docs_dir.to_path_buf();
        let static_files = warp::fs::dir(docs_path.clone());
        
        // Index route
        let index_route = warp::path::end()
            .and(warp::fs::file(docs_path.join("index.html")));
        
        // Combined routes
        let routes = websocket_route
            .or(api_routes)
            .or(static_files)
            .or(index_route)
            .with(self.create_cors_filter());
        
        let addr = (
            self.config.host.parse::<std::net::IpAddr>()
                .map_err(|e| CursedError::system_error(&format!("Invalid host address: {}", e)))?,
            self.config.port,
        );
        
        info!("🌐 Live documentation server starting at http://{}:{}", self.config.host, self.config.port);
        info!("📖 Open your browser to view live documentation");
        info!("🔌 WebSocket endpoint: ws://{}:{}/ws", self.config.host, self.config.port);
        
        // Open browser if configured
        if self.config.auto_open_browser {
            self.open_browser()?;
        }
        
        // Start server statistics broadcaster
        self.start_statistics_broadcaster().await;
        
        // Start the server
        warp::serve(routes)
            .run(addr)
            .await;
        
        Ok(())
    }
    
    /// Handle WebSocket connection
    #[instrument(skip(ws, websocket_tx, clients, statistics, config))]
    async fn handle_websocket_connection(
        ws: WebSocket,
        websocket_tx: broadcast::Sender<WebSocketMessage>,
        clients: Arc<RwLock<HashMap<Uuid, ClientConnection>>>,
        statistics: Arc<Mutex<ServerStatistics>>,
        config: LiveServerConfig,
    ) {
        let client_id = Uuid::new_v4();
        let (ws_tx, mut ws_rx) = ws.split();
        let mut websocket_rx = websocket_tx.subscribe();
        
        // Register client
        {
            let mut clients_guard = clients.write().unwrap();
            let connection = ClientConnection {
                id: client_id,
                connected_at: SystemTime::now(),
                last_ping: None,
                user_agent: None, // Would extract from headers
                features: {
                    let mut features = HashSet::new();
                    if config.enable_playground {
                        features.insert("playground".to_string());
                    }
                    if config.enable_api_explorer {
                        features.insert("api_explorer".to_string());
                    }
                    if config.enable_syntax_highlighting {
                        features.insert("syntax_highlighting".to_string());
                    }
                    if config.enable_code_folding {
                        features.insert("code_folding".to_string());
                    }
                    features
                },
            };
            clients_guard.insert(client_id, connection.clone());
            
            info!("WebSocket client connected: {}", client_id);
            
            // Update statistics
            {
                let mut stats = statistics.lock().unwrap();
                stats.connected_clients = clients_guard.len();
            }
        }
        
        // Send connection confirmation
        let connected_message = WebSocketMessage::Connected {
            server_version: env!("CARGO_PKG_VERSION").to_string(),
            features: {
                let clients_guard = clients.read().unwrap();
                clients_guard.get(&client_id)
                    .map(|conn| conn.features.iter().cloned().collect())
                    .unwrap_or_default()
            },
        };
        
        let ws_tx = Arc::new(Mutex::new(ws_tx));
        let ws_tx_clone = Arc::clone(&ws_tx);
        
        // Send initial message
        if let Ok(message) = serde_json::to_string(&connected_message) {
            if let Ok(mut sender) = ws_tx.lock() {
                let _ = sender.send(Message::text(message)).await;
            }
        }
        
        // Handle incoming messages
        let clients_for_incoming = Arc::clone(&clients);
        let statistics_for_incoming = Arc::clone(&statistics);
        let ws_tx_for_incoming = Arc::clone(&ws_tx);
        let incoming_task = tokio::spawn(async move {
            while let Some(result) = ws_rx.next().await {
                match result {
                    Ok(message) => {
                        if let Err(e) = Self::handle_incoming_websocket_message(
                            client_id,
                            message,
                            &clients_for_incoming,
                            &statistics_for_incoming,
                            &ws_tx_for_incoming,
                        ).await {
                            warn!("Error handling WebSocket message from {}: {}", client_id, e);
                        }
                    }
                    Err(e) => {
                        warn!("WebSocket error for client {}: {}", client_id, e);
                        break;
                    }
                }
            }
        });
        
        // Handle outgoing messages
        let outgoing_task = tokio::spawn(async move {
            while let Ok(message) = websocket_rx.recv().await {
                if let Ok(message_text) = serde_json::to_string(&message) {
                    if let Ok(mut sender) = ws_tx_clone.lock() {
                        if sender.send(Message::text(message_text)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });
        
        // Wait for either task to complete
        tokio::select! {
            _ = incoming_task => {},
            _ = outgoing_task => {},
        }
        
        // Cleanup client
        {
            let mut clients_guard = clients.write().unwrap();
            clients_guard.remove(&client_id);
            
            info!("WebSocket client disconnected: {}", client_id);
            
            // Update statistics
            {
                let mut stats = statistics.lock().unwrap();
                stats.connected_clients = clients_guard.len();
            }
        }
    }
    
    /// Handle incoming WebSocket message from client
    #[instrument(skip(message, clients, statistics, ws_tx))]
    async fn handle_incoming_websocket_message(
        client_id: Uuid,
        message: Message,
        clients: &Arc<RwLock<HashMap<Uuid, ClientConnection>>>,
        statistics: &Arc<Mutex<ServerStatistics>>,
        ws_tx: &Arc<Mutex<warp::ws::SplitSink<WebSocket, Message>>>,
    ) -> LiveServerResult<()> {
        if message.is_text() {
            let text = message.to_str()
                .map_err(|_| CursedError::system_error("Invalid UTF-8 in WebSocket message"))?;
            
            let parsed_message: WebSocketMessage = serde_json::from_str(text)
                .map_err(|e| CursedError::system_error(&format!("Failed to parse WebSocket message: {}", e)))?;
            
            match parsed_message {
                WebSocketMessage::Ping { timestamp } => {
                    // Update client last ping
                    {
                        let mut clients_guard = clients.write().unwrap();
                        if let Some(client) = clients_guard.get_mut(&client_id) {
                            client.last_ping = Some(timestamp);
                        }
                    }
                    
                    // Send pong response
                    let pong = WebSocketMessage::Pong { timestamp: SystemTime::now() };
                    if let Ok(pong_text) = serde_json::to_string(&pong) {
                        if let Ok(mut sender) = ws_tx.lock() {
                            let _ = sender.send(Message::text(pong_text)).await;
                        }
                    }
                }
                
                WebSocketMessage::ExecuteCode { code, language, session_id } => {
                    // Handle code execution for playground
                    let result = Self::execute_code_playground(&code, &language).await;
                    let response = WebSocketMessage::ExecutionResult {
                        session_id,
                        success: result.is_ok(),
                        output: result.as_ref().map(|r| r.clone()).unwrap_or_default(),
                        error: result.err().map(|e| e.to_string()),
                        execution_time_ms: 0, // Would measure actual execution time
                    };
                    
                    if let Ok(response_text) = serde_json::to_string(&response) {
                        if let Ok(mut sender) = ws_tx.lock() {
                            let _ = sender.send(Message::text(response_text)).await;
                        }
                    }
                }
                
                WebSocketMessage::ApiCall { method_name, parameters, session_id } => {
                    // Handle API method call for API explorer
                    let result = Self::execute_api_method(&method_name, &parameters).await;
                    let response = WebSocketMessage::ApiResult {
                        session_id,
                        success: result.is_ok(),
                        result: result.as_ref()
                            .map(|r| r.clone())
                            .unwrap_or(serde_json::Value::Null),
                        error: result.err().map(|e| e.to_string()),
                    };
                    
                    if let Ok(response_text) = serde_json::to_string(&response) {
                        if let Ok(mut sender) = ws_tx.lock() {
                            let _ = sender.send(Message::text(response_text)).await;
                        }
                    }
                }
                
                _ => {
                    debug!("Unhandled WebSocket message type from client {}", client_id);
                }
            }
        }
        
        Ok(())
    }
    
    /// Execute code in the playground
    #[instrument(skip(code))]
    async fn execute_code_playground(code: &str, language: &str) -> Result<String, CursedError> {
        if language != "cursed" && language != "csd" {
            return Err(CursedError::system_error("Only CURSED language is supported"));
        }
        
        // This would integrate with the CURSED interpreter/compiler
        // For now, return a mock response
        info!("Executing code in playground: {} lines of {}", code.split("\n").count(), language);
        
        // Mock execution result
        Ok(format!("// Code execution result\n// {} lines of {} code processed\nslay \"Hello from CURSED playground!\"", 
            code.split("\n").count(), language))
    }
    
    /// Execute API method for API explorer
    #[instrument(skip(parameters))]
    async fn execute_api_method(
        method_name: &str, 
        parameters: &HashMap<String, serde_json::Value>
    ) -> Result<serde_json::Value, CursedError> {
        info!("Executing API method: {} with {} parameters", method_name, parameters.len());
        
        // This would integrate with the CURSED runtime to call actual methods
        // For now, return a mock response
        let result = serde_json::json!({
            "method": method_name,
            "parameters": parameters,
            "result": "Mock API execution result",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        
        Ok(result)
    }
    
    /// Create API routes for interactive features
    fn create_api_routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Statistics endpoint
        let statistics = Arc::clone(&self.statistics);
        let stats_route = warp::path!("api" / "stats")
            .and(warp::get())
            .and(warp::any().map(move || Arc::clone(&statistics)))
            .and_then(Self::handle_stats_request);
        
        // Health check endpoint
        let health_route = warp::path!("api" / "health")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "status": "healthy",
                    "server": "CURSED Live Documentation Server",
                    "version": env!("CARGO_PKG_VERSION"),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }))
            });
        
        stats_route.or(health_route)
    }
    
    /// Handle statistics API request
    async fn handle_stats_request(
        statistics: Arc<Mutex<ServerStatistics>>,
    ) -> std::result::Result<impl warp::Reply, warp::Rejection> {
        let stats = statistics.lock().unwrap().clone();
        Ok(warp::reply::json(&stats))
    }
    
    /// Create CORS filter
    fn create_cors_filter(&self) -> warp::cors::Builder {
        let origins: Vec<&str> = self.config.cors_origins.iter().map(|s| s.as_str()).collect();
        
        warp::cors()
            .allow_origins(origins)
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    }
    
    /// Start statistics broadcaster
    async fn start_statistics_broadcaster(&self) {
        let websocket_tx = self.websocket_tx.clone();
        let statistics = Arc::clone(&self.statistics);
        let clients = Arc::clone(&self.clients);
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                let stats_message = {
                    let stats = statistics.lock().unwrap();
                    let clients_count = clients.read().unwrap().len();
                    
                    WebSocketMessage::ServerStats {
                        timestamp: SystemTime::now(),
                        connected_clients: clients_count,
                        total_regenerations: stats.total_regenerations,
                        average_generation_time_ms: stats.average_generation_time_ms,
                        uptime_seconds: stats.started_at.elapsed()
                            .unwrap_or(Duration::from_secs(0))
                            .as_secs(),
                    }
                };
                
                let _ = websocket_tx.send(stats_message);
            }
        });
    }
    
    /// Open browser to view documentation
    fn open_browser(&self) -> LiveServerResult<()> {
        let url = format!("http://{}:{}", self.config.host, self.config.port);
        
        info!("🌐 Opening documentation in browser: {}", url);
        
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/c", "start", &url])
                .spawn()
                .map_err(|e| CursedError::system_error(&format!("Failed to open browser: {}", e)))?;
        }
        
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(&url)
                .spawn()
                .map_err(|e| CursedError::system_error(&format!("Failed to open browser: {}", e)))?;
        }
        
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(&url)
                .spawn()
                .map_err(|e| CursedError::system_error(&format!("Failed to open browser: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Get current server statistics
    pub fn get_statistics(&self) -> ServerStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_default()
    }
    
    /// Get connected client count
    pub fn get_connected_clients(&self) -> usize {
        self.clients.read()
            .map(|clients| clients.len())
            .unwrap_or(0)
    }
    
    /// Check if server is running
    pub fn is_running(&self) -> bool {
        self.is_running.lock()
            .map(|running| *running)
            .unwrap_or(false)
    }
}

impl Drop for LiveDocumentationServer {
    fn drop(&mut self) {
        let _ = futures::executor::block_on(self.stop_serving());
    }
}

/// Builder for creating live documentation server instances
pub struct LiveDocumentationServerBuilder {
    config: LiveServerConfig,
}

impl LiveDocumentationServerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: LiveServerConfig::default(),
        }
    }
    
    /// Set server port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }
    
    /// Set server host
    pub fn host(mut self, host: String) -> Self {
        self.config.host = host;
        self
    }
    
    /// Set watch debounce duration
    pub fn watch_debounce(mut self, duration: Duration) -> Self {
        self.config.watch_debounce = duration;
        self
    }
    
    /// Enable/disable playground
    pub fn enable_playground(mut self, enable: bool) -> Self {
        self.config.enable_playground = enable;
        self
    }
    
    /// Enable/disable API explorer
    pub fn enable_api_explorer(mut self, enable: bool) -> Self {
        self.config.enable_api_explorer = enable;
        self
    }
    
    /// Set output formats
    pub fn output_formats(mut self, formats: Vec<DocFormat>) -> Self {
        self.config.output_formats = formats;
        self
    }
    
    /// Enable/disable auto-open browser
    pub fn auto_open_browser(mut self, enable: bool) -> Self {
        self.config.auto_open_browser = enable;
        self
    }
    
    /// Build the live documentation server
    pub fn build(self) -> LiveServerResult<LiveDocumentationServer> {
        LiveDocumentationServer::new(self.config)
    }
}

impl Default for LiveDocumentationServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
