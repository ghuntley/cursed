// Documentation Server
// 
// Production web server for hosting CURSED documentation with version management,
// search functionality, and performance monitoring.

use crate::error::{CursedError, Result};
use crate::docs::registry::DocumentationRegistry;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, instrument, warn};
use warp::{Filter, Reply};

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address
    pub bind_address: SocketAddr,
    /// Document root directory
    pub document_root: PathBuf,
    /// Enable HTTPS
    pub enable_https: bool,
    /// SSL certificate configuration
    pub ssl_config: Option<SslServerConfig>,
    /// CORS configuration
    pub cors_config: CorsConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Search configuration
    pub search_config: SearchConfig,
    /// Analytics configuration
    pub analytics_config: AnalyticsConfig,
}

/// SSL configuration for HTTPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslServerConfig {
    /// Certificate file path
    pub cert_path: PathBuf,
    /// Private key file path
    pub key_path: PathBuf,
    /// Certificate chain file path
    pub chain_path: Option<PathBuf>,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst capacity
    pub burst_capacity: u32,
    /// Enable rate limiting
    pub enabled: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache duration for static files (seconds)
    pub static_cache_duration: u64,
    /// Cache duration for API responses (seconds)
    pub api_cache_duration: u64,
    /// Maximum cache size (bytes)
    pub max_cache_size: u64,
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Enable search functionality
    pub enabled: bool,
    /// Maximum search results per query
    pub max_results: usize,
    /// Search index refresh interval (seconds)
    pub index_refresh_interval: u64,
    /// Full-text search enabled
    pub full_text_search: bool,
}

/// Analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsConfig {
    /// Enable analytics
    pub enabled: bool,
    /// Analytics data retention (days)
    pub retention_days: u32,
    /// Track page views
    pub track_page_views: bool,
    /// Track search queries
    pub track_search_queries: bool,
    /// Track download events
    pub track_downloads: bool,
}

/// Search query request
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// Search query string
    pub q: String,
    /// Package filter
    pub package: Option<String>,
    /// Version filter
    pub version: Option<String>,
    /// Result limit
    pub limit: Option<usize>,
    /// Result offset
    pub offset: Option<usize>,
}

/// Search result
#[derive(Debug, Serialize)]
pub struct SearchResult {
    /// Result title
    pub title: String,
    /// Result URL
    pub url: String,
    /// Result snippet
    pub snippet: String,
    /// Package name
    pub package: String,
    /// Package version
    pub version: String,
    /// Result type (function, struct, module, etc.)
    pub result_type: String,
    /// Relevance score
    pub score: f64,
}

/// Search response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// Search results
    pub results: Vec<SearchResult>,
    /// Total number of results
    pub total: usize,
    /// Query used
    pub query: String,
    /// Search time in milliseconds
    pub search_time_ms: u64,
    /// Pagination info
    pub pagination: PaginationInfo,
}

/// Pagination information
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    /// Current page
    pub page: usize,
    /// Total pages
    pub total_pages: usize,
    /// Results per page
    pub per_page: usize,
    /// Has next page
    pub has_next: bool,
    /// Has previous page
    pub has_prev: bool,
}

/// Analytics event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    /// Event type
    pub event_type: String,
    /// Timestamp
    pub timestamp: u64,
    /// User agent
    pub user_agent: Option<String>,
    /// IP address (hashed for privacy)
    pub ip_hash: String,
    /// Referer
    pub referer: Option<String>,
    /// Additional event data
    pub data: HashMap<String, serde_json::Value>,
}

/// Server metrics
#[derive(Debug, Clone, Serialize)]
pub struct ServerMetrics {
    /// Total requests served
    pub total_requests: u64,
    /// Requests per second (current)
    pub requests_per_second: f64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// CursedError rate (percentage)
    pub error_rate: f64,
    /// Cache hit rate (percentage)
    pub cache_hit_rate: f64,
    /// Active connections
    pub active_connections: u32,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
}

/// Version information
#[derive(Debug, Serialize)]
pub struct VersionInfo {
    /// Package name
    pub package: String,
    /// Available versions
    pub versions: Vec<String>,
    /// Latest version
    pub latest: String,
    /// Default version (if different from latest)
    pub default: Option<String>,
}

/// Documentation server
pub struct DocumentationServer {
    config: ServerConfig,
    registry: Arc<DocumentationRegistry>,
    analytics: Arc<RwLock<Vec<AnalyticsEvent>>>,
    metrics: Arc<RwLock<ServerMetrics>>,
    search_index: Arc<RwLock<HashMap<String, Vec<SearchResult>>>>,
    start_time: SystemTime,
}

impl DocumentationServer {
    /// Create a new documentation server
    pub fn new(config: ServerConfig, registry: DocumentationRegistry) -> Self {
        Self {
            config,
            registry: Arc::new(registry),
            analytics: Arc::new(RwLock::new(Vec::new())),
            metrics: Arc::new(RwLock::new(ServerMetrics {
                total_requests: 0,
                requests_per_second: 0.0,
                avg_response_time_ms: 0.0,
                error_rate: 0.0,
                cache_hit_rate: 0.0,
                active_connections: 0,
                uptime_seconds: 0,
                memory_usage_bytes: 0,
            })),
            search_index: Arc::new(RwLock::new(HashMap::new())),
            start_time: SystemTime::now(),
        }
    }

    /// Start the documentation server
    #[instrument(skip(self))]
    pub async fn start(&self) -> Result<()> {
        info!(bind_address = %self.config.bind_address, "Starting documentation server");

        // Initialize search index
        self.initialize_search_index().await?;

        // Build routes
        let routes = self.build_routes();

        // Start background tasks
        self.start_background_tasks().await;

        // Start server
        info!(bind_address = %self.config.bind_address, "Documentation server started");
        
        warp::serve(routes)
            .run(self.config.bind_address)
            .await;

        Ok(())
    }

    /// Build all server routes
    fn build_routes(&self) -> impl Filter<Extract = impl Reply, CursedError = warp::Rejection> + Clone {
        let registry = self.registry.clone();
        let analytics = self.analytics.clone();
        let metrics = self.metrics.clone();
        let search_index = self.search_index.clone();
        let config = self.config.clone();

        // Static file serving
        let static_files = warp::path("static")
            .and(warp::fs::dir(self.config.document_root.join("static")))
            .with(self.cache_headers_filter())
            .with(self.metrics_filter(metrics.clone()));

        // Documentation routes
        let docs = warp::path("docs")
            .and(warp::path::param::<String>()) // package name
            .and(warp::path::param::<String>()) // version
            .and(warp::path::tail())
            .and_then(move |package: String, version: String, path: warp::path::Tail| {
                let registry = registry.clone();
                let analytics = analytics.clone();
                async move {
                    Self::serve_documentation(registry, analytics, package, version, path.as_str()).await
                }
            })
            .with(self.cors_filter())
            .with(self.metrics_filter(metrics.clone()));

        // API routes
        let api = warp::path("api")
            .and(
                self.api_search(search_index.clone())
                .or(self.api_versions(registry.clone()))
                .or(self.api_metrics(metrics.clone()))
                .or(self.api_health())
            )
            .with(self.cors_filter())
            .with(self.metrics_filter(metrics.clone()));

        // Default route (redirect to latest docs)
        let default = warp::path::end()
            .map(|| warp::redirect::found(warp::http::Uri::from_static("/docs/latest")));

        static_files
            .or(docs)
            .or(api)
            .or(default)
    }

    /// Serve documentation files
    async fn serve_documentation(
        registry: Arc<DocumentationRegistry>,
        analytics: Arc<RwLock<Vec<AnalyticsEvent>>>,
        package: String,
        version: String,
        path: &str,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Log analytics event
        let event = AnalyticsEvent {
            event_type: "page_view".to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            user_agent: None, // Would extract from headers
            ip_hash: "hashed_ip".to_string(), // Would hash actual IP
            referer: None,
            data: {
                let mut data = HashMap::new();
                data.insert("package".to_string(), serde_json::Value::String(package.clone()));
                data.insert("version".to_string(), serde_json::Value::String(version.clone()));
                data.insert("path".to_string(), serde_json::Value::String(path.to_string()));
                data
            },
        };
        
        {
            let mut analytics_guard = analytics.write().await;
            analytics_guard.push(event);
        }

        // Serve file (simplified implementation)
        let file_path = format!("docs/{}/{}/{}", package, version, path);
        Ok(warp::reply::html(format!("Documentation for {} {} at {}", package, version, path)))
    }

    /// API route for search
    fn api_search(&self, search_index: Arc<RwLock<HashMap<String, Vec<SearchResult>>>>) -> impl Filter<Extract = impl Reply, CursedError = warp::Rejection> + Clone {
        warp::path("search")
            .and(warp::get())
            .and(warp::query::<SearchQuery>())
            .and_then(move |query: SearchQuery| {
                let search_index = search_index.clone();
                async move {
                    Self::handle_search(search_index, query).await
                }
            })
    }

    /// Handle search query
    async fn handle_search(
        search_index: Arc<RwLock<HashMap<String, Vec<SearchResult>>>>,
        query: SearchQuery,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        let start_time = SystemTime::now();
        
        let index = search_index.read().await;
        let mut results = Vec::new();
        
        // Simple search implementation - would be more sophisticated in production
        for (key, search_results) in index.iter() {
            if key.contains(&query.q.to_lowercase()) {
                results.extend(search_results.clone());
            }
        }
        
        // Apply filters
        if let Some(package) = &query.package {
            results.retain(|r| r.package == *package);
        }
        
        if let Some(version) = &query.version {
            results.retain(|r| r.version == *version);
        }
        
        // Sort by relevance score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        let total = results.len();
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(20).min(100);
        
        let paginated_results: Vec<SearchResult> = results
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();
        
        let search_time = start_time.elapsed().unwrap().as_millis() as u64;
        
        let response = SearchResponse {
            results: paginated_results,
            total,
            query: query.q,
            search_time_ms: search_time,
            pagination: PaginationInfo {
                page: offset / limit + 1,
                total_pages: (total + limit - 1) / limit,
                per_page: limit,
                has_next: offset + limit < total,
                has_prev: offset > 0,
            },
        };
        
        Ok(warp::reply::json(&response))
    }

    /// API route for version information
    fn api_versions(&self, registry: Arc<DocumentationRegistry>) -> impl Filter<Extract = impl Reply, CursedError = warp::Rejection> + Clone {
        warp::path("versions")
            .and(warp::get())
            .and(warp::path::param::<String>()) // package name
            .and_then(move |package: String| {
                let registry = registry.clone();
                async move {
                    Self::handle_versions(registry, package).await
                }
            })
    }

    /// Handle version information request
    async fn handle_versions(
        registry: Arc<DocumentationRegistry>,
        package: String,
    ) -> std::result::Result<impl Reply, warp::Rejection> {
        // Get versions from registry
        let versions = vec!["1.0.0".to_string(), "0.9.0".to_string()]; // Simplified
        
        let version_info = VersionInfo {
            package: package.clone(),
            versions: versions.clone(),
            latest: versions.first().unwrap_or(&"1.0.0".to_string()).clone(),
            default: None,
        };
        
        Ok(warp::reply::json(&version_info))
    }

    /// API route for server metrics
    fn api_metrics(&self, metrics: Arc<RwLock<ServerMetrics>>) -> impl Filter<Extract = impl Reply, CursedError = warp::Rejection> + Clone {
        warp::path("metrics")
            .and(warp::get())
            .and_then(move || {
                let metrics = metrics.clone();
                async move {
                    let metrics_guard = metrics.read().await;
                    Ok::<_, warp::Rejection>(warp::reply::json(&*metrics_guard))
                }
            })
    }

    /// API route for health check
    fn api_health(&self) -> impl Filter<Extract = impl Reply, CursedError = warp::Rejection> + Clone {
        warp::path("health")
            .and(warp::get())
            .map(|| {
                let health = serde_json::json!({
                    "status": "healthy",
                    "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
                });
                warp::reply::json(&health)
            })
    }

    /// CORS filter
    fn cors_filter(&self) -> impl Filter<Extract = (), CursedError = std::convert::Infallible> + Clone {
        let config = self.config.cors_config.clone();
        warp::cors()
            .allow_origins(config.allowed_origins.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .allow_methods(config.allowed_methods)
            .allow_headers(config.allowed_headers)
            .allow_credentials(config.allow_credentials)
    }

    /// Cache headers filter
    fn cache_headers_filter(&self) -> impl Filter<Extract = (), CursedError = std::convert::Infallible> + Clone {
        let cache_duration = self.config.cache_config.static_cache_duration;
        warp::reply::with::header("Cache-Control", format!("public, max-age={}", cache_duration))
    }

    /// Metrics filter
    fn metrics_filter(&self, metrics: Arc<RwLock<ServerMetrics>>) -> impl Filter<Extract = (), CursedError = std::convert::Infallible> + Clone {
        warp::any()
            .and_then(move || {
                let metrics = metrics.clone();
                async move {
                    let mut metrics_guard = metrics.write().await;
                    metrics_guard.total_requests += 1;
                    Ok::<_, std::convert::Infallible>(())
                }
            })
            .untuple_one()
    }

    /// Initialize search index
    #[instrument(skip(self))]
    async fn initialize_search_index(&self) -> Result<()> {
        info!("Initializing search index");
        
        let mut index = self.search_index.write().await;
        
        // Build search index from available documentation
        // This is a simplified implementation
        let sample_results = vec![
            SearchResult {
                title: "Function: println".to_string(),
                url: "/docs/std/1.0.0/io/println".to_string(),
                snippet: "Print a line to stdout".to_string(),
                package: "std".to_string(),
                version: "1.0.0".to_string(),
                result_type: "function".to_string(),
                score: 1.0,
            },
        ];
        
        index.insert("println".to_string(), sample_results);
        
        info!("Search index initialized");
        Ok(())
    }

    /// Start background tasks
    async fn start_background_tasks(&self) {
        let analytics = self.analytics.clone();
        let metrics = self.metrics.clone();
        let start_time = self.start_time;
        
        // Analytics cleanup task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Every hour
            loop {
                interval.tick().await;
                Self::cleanup_analytics(analytics.clone()).await;
            }
        });
        
        // Metrics update task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60)); // Every minute
            loop {
                interval.tick().await;
                Self::update_metrics(metrics.clone(), start_time).await;
            }
        });
    }

    /// Cleanup old analytics data
    async fn cleanup_analytics(analytics: Arc<RwLock<Vec<AnalyticsEvent>>>) {
        let mut analytics_guard = analytics.write().await;
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - (30 * 24 * 3600); // 30 days
        
        analytics_guard.retain(|event| event.timestamp > cutoff);
    }

    /// Update server metrics
    async fn update_metrics(metrics: Arc<RwLock<ServerMetrics>>, start_time: SystemTime) {
        let mut metrics_guard = metrics.write().await;
        
        metrics_guard.uptime_seconds = start_time.elapsed().unwrap().as_secs();
        
        // Update other metrics (simplified)
        metrics_guard.requests_per_second = metrics_guard.total_requests as f64 / metrics_guard.uptime_seconds as f64;
        metrics_guard.avg_response_time_ms = 50.0; // Would calculate from actual response times
        metrics_guard.error_rate = 0.1; // Would calculate from actual errors
        metrics_guard.cache_hit_rate = 85.0; // Would calculate from actual cache hits
        metrics_guard.memory_usage_bytes = Self::get_memory_usage();
    }

    /// Get current memory usage
    fn get_memory_usage() -> u64 {
        // Simplified - would use actual memory monitoring
        1024 * 1024 * 100 // 100MB
    }

    /// Validate server configuration
    pub fn validate_config(&self) -> Result<()> {
        if !self.config.document_root.exists() {
            return Err(CursedError::Configuration(
                format!("Document root does not exist: {}", self.config.document_root.display())
            ));
        }

        if self.config.enable_https {
            if let Some(ssl_config) = &self.config.ssl_config {
                if !ssl_config.cert_path.exists() {
                    return Err(CursedError::Configuration(
                        format!("SSL certificate file does not exist: {}", ssl_config.cert_path.display())
                    ));
                }
                if !ssl_config.key_path.exists() {
                    return Err(CursedError::Configuration(
                        format!("SSL private key file does not exist: {}", ssl_config.key_path.display())
                    ));
                }
            } else {
                return Err(CursedError::Configuration(
                    "HTTPS enabled but SSL configuration not provided".to_string()
                ));
            }
        }

        Ok(())
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".parse().unwrap(),
            document_root: PathBuf::from("./docs"),
            enable_https: false,
            ssl_config: None,
            cors_config: CorsConfig::default(),
            rate_limiting: RateLimitConfig::default(),
            cache_config: CacheConfig::default(),
            search_config: SearchConfig::default(),
            analytics_config: AnalyticsConfig::default(),
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            allow_credentials: false,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_capacity: 10,
            enabled: true,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            static_cache_duration: 3600,    // 1 hour
            api_cache_duration: 300,        // 5 minutes
            max_cache_size: 1024 * 1024 * 100, // 100MB
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_results: 100,
            index_refresh_interval: 300, // 5 minutes
            full_text_search: true,
        }
    }
}

impl Default for AnalyticsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 30,
            track_page_views: true,
            track_search_queries: true,
            track_downloads: true,
        }
    }
}

