use crate::web::StatusCode;
/// Comprehensive timeout middleware for HTTP request processing
/// 
/// Provides timeout mechanisms for requests, connections, sessions, and database operations
/// using parking_lot crate's timeout support and async-aware implementations

use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareResult};
use crate::stdlib::web_vibez::config::{ServerConfig, SessionConfig};
use crate::stdlib::web_vibez::error_handling::MiddlewareError;
use crate::stdlib::web_vibez::StatusCode;
use crate::error::Error;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use parking_lot::{RwLock, Condvar};
use tracing::{debug, info, warn, error, instrument};
use tokio::time::timeout;

/// Timeout middleware that enforces various timeout mechanisms
#[derive(Debug)]
pub struct TimeoutMiddleware {
    /// Server configuration with timeout values
    server_config: ServerConfig,
    /// Session configuration with timeout values
    session_config: SessionConfig,
    /// Active request tracking
    active_requests: Arc<RwLock<HashMap<String, RequestInfo>>>,
    /// Active connection tracking
    active_connections: Arc<RwLock<HashMap<String, ConnectionInfo>>>,
    /// Session timeout tracking
    session_tracker: Arc<RwLock<HashMap<String, SessionTimeout>>>,
    /// Database operation timeout tracking
    database_tracker: Arc<RwLock<HashMap<String, DatabaseTimeout>>>,
    /// Timeout configuration
    config: TimeoutConfig,
    /// Condition variable for timeout notifications
    timeout_notify: Arc<(Mutex<bool>, Condvar)>,
}

/// Timeout configuration options
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Enable request timeouts
    pub enable_request_timeout: bool,
    /// Enable connection timeouts
    pub enable_connection_timeout: bool,
    /// Enable session timeouts
    pub enable_session_timeout: bool,
    /// Enable database timeouts
    pub enable_database_timeout: bool,
    /// Graceful shutdown timeout
    pub graceful_shutdown_timeout: Duration,
    /// Cleanup interval for expired timeouts
    pub cleanup_interval: Duration,
    /// Log timeout events
    pub log_timeout_events: bool,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            enable_request_timeout: true,
            enable_connection_timeout: true,
            enable_session_timeout: true,
            enable_database_timeout: true,
            graceful_shutdown_timeout: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(60),
            log_timeout_events: true,
        }
    }
}

/// Information about active requests
#[derive(Debug, Clone)]
struct RequestInfo {
    request_id: String,
    start_time: Instant,
    timeout_duration: Duration,
    client_ip: Option<String>,
    path: String,
    method: String,
}

/// Information about active connections
#[derive(Debug, Clone)]
struct ConnectionInfo {
    connection_id: String,
    established_at: Instant,
    timeout_duration: Duration,
    client_ip: Option<String>,
    last_activity: Instant,
}

/// Session timeout tracking
#[derive(Debug, Clone)]
struct SessionTimeout {
    session_id: String,
    last_accessed: Instant,
    timeout_duration: Duration,
    is_active: bool,
}

/// Database operation timeout tracking
#[derive(Debug, Clone)]
struct DatabaseTimeout {
    operation_id: String,
    started_at: Instant,
    timeout_duration: Duration,
    operation_type: String,
}

/// Timeout result for async operations
pub type TimeoutResult<T> = std::result::Result<T, TimeoutError>;

/// Timeout error types
#[derive(Debug, Clone)]
pub enum TimeoutError {
    RequestTimeout { elapsed: Duration, timeout: Duration },
    ConnectionTimeout { elapsed: Duration, timeout: Duration },
    SessionTimeout { elapsed: Duration, timeout: Duration },
    DatabaseTimeout { elapsed: Duration, timeout: Duration, operation: String },
    GracefulShutdownTimeout,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeoutError::RequestTimeout { elapsed, timeout } => {
                write!(f, "Request timeout: {}ms elapsed, {}ms timeout", 
                       elapsed.as_millis(), timeout.as_millis())
            }
            TimeoutError::ConnectionTimeout { elapsed, timeout } => {
                write!(f, "Connection timeout: {}ms elapsed, {}ms timeout", 
                       elapsed.as_millis(), timeout.as_millis())
            }
            TimeoutError::SessionTimeout { elapsed, timeout } => {
                write!(f, "Session timeout: {}ms elapsed, {}ms timeout", 
                       elapsed.as_millis(), timeout.as_millis())
            }
            TimeoutError::DatabaseTimeout { elapsed, timeout, operation } => {
                write!(f, "Database operation '{}' timeout: {}ms elapsed, {}ms timeout", 
                       operation, elapsed.as_millis(), timeout.as_millis())
            }
            TimeoutError::GracefulShutdownTimeout => {
                write!(f, "Graceful shutdown timeout exceeded")
            }
        }
    }
}

impl std::error::Error for TimeoutError {}

impl TimeoutMiddleware {
    /// Create new timeout middleware with configurations
    pub fn new(server_config: ServerConfig, session_config: SessionConfig) -> Self {
        Self {
            server_config,
            session_config,
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
            session_tracker: Arc::new(RwLock::new(HashMap::new())),
            database_tracker: Arc::new(RwLock::new(HashMap::new())),
            config: TimeoutConfig::default(),
            timeout_notify: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// Configure timeout settings
    pub fn with_config(mut self, config: TimeoutConfig) -> Self {
        self.config = config;
        self
    }

    /// Start request timeout tracking
    fn start_request_timeout(&self, context: &RequestContext) {
        if !self.config.enable_request_timeout {
            return;
        }

        let request_info = RequestInfo {
            request_id: context.request_id.clone(),
            start_time: Instant::now(),
            timeout_duration: self.server_config.request_timeout,
            client_ip: context.client_ip.clone(),
            path: context.path.clone(),
            method: context.method.to_string(),
        };

        let mut requests = self.active_requests.write();
        requests.insert(context.request_id.clone(), request_info);

        if self.config.log_timeout_events {
            debug!(
                request_id = %context.request_id,
                timeout_ms = self.server_config.request_timeout.as_millis(),
                "Started request timeout tracking"
            );
        }
    }

    /// Stop request timeout tracking
    fn stop_request_timeout(&self, request_id: &str) {
        if !self.config.enable_request_timeout {
            return;
        }

        let mut requests = self.active_requests.write();
        if let Some(request_info) = requests.remove(request_id) {
            let elapsed = request_info.start_time.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    request_id = %request_id,
                    elapsed_ms = elapsed.as_millis(),
                    "Stopped request timeout tracking"
                );
            }
        }
    }

    /// Check for timed out requests
    fn check_request_timeouts(&self) -> Vec<String> {
        if !self.config.enable_request_timeout {
            return Vec::new();
        }

        let requests = self.active_requests.read();
        let now = Instant::now();
        let mut timed_out = Vec::new();

        for (request_id, request_info) in requests.iter() {
            let elapsed = now.duration_since(request_info.start_time);
            if elapsed > request_info.timeout_duration {
                timed_out.push(request_id.clone());
                
                if self.config.log_timeout_events {
                    warn!(
                        request_id = %request_id,
                        elapsed_ms = elapsed.as_millis(),
                        timeout_ms = request_info.timeout_duration.as_millis(),
                        path = %request_info.path,
                        method = %request_info.method,
                        "Request timeout detected"
                    );
                }
            }
        }

        timed_out
    }

    /// Start connection timeout tracking
    pub fn start_connection_timeout(&self, connection_id: String, client_ip: Option<String>) {
        if !self.config.enable_connection_timeout {
            return;
        }

        let connection_info = ConnectionInfo {
            connection_id: connection_id.clone(),
            established_at: Instant::now(),
            timeout_duration: self.server_config.connection_timeout,
            client_ip: client_ip.clone(),
            last_activity: Instant::now(),
        };

        let mut connections = self.active_connections.write();
        connections.insert(connection_id.clone(), connection_info);

        if self.config.log_timeout_events {
            debug!(
                connection_id = %connection_id,
                client_ip = ?client_ip,
                timeout_ms = self.server_config.connection_timeout.as_millis(),
                "Started connection timeout tracking"
            );
        }
    }

    /// Update connection activity
    pub fn update_connection_activity(&self, connection_id: &str) {
        if !self.config.enable_connection_timeout {
            return;
        }

        let mut connections = self.active_connections.write();
        if let Some(connection_info) = connections.get_mut(connection_id) {
            connection_info.last_activity = Instant::now();
        }
    }

    /// Stop connection timeout tracking
    pub fn stop_connection_timeout(&self, connection_id: &str) {
        if !self.config.enable_connection_timeout {
            return;
        }

        let mut connections = self.active_connections.write();
        if let Some(connection_info) = connections.remove(connection_id) {
            let elapsed = connection_info.established_at.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    connection_id = %connection_id,
                    elapsed_ms = elapsed.as_millis(),
                    "Stopped connection timeout tracking"
                );
            }
        }
    }

    /// Start session timeout tracking
    pub fn start_session_timeout(&self, session_id: String) {
        if !self.config.enable_session_timeout {
            return;
        }

        let session_timeout = SessionTimeout {
            session_id: session_id.clone(),
            last_accessed: Instant::now(),
            timeout_duration: self.session_config.session_timeout,
            is_active: true,
        };

        let mut sessions = self.session_tracker.write();
        sessions.insert(session_id.clone(), session_timeout);

        if self.config.log_timeout_events {
            debug!(
                session_id = %session_id,
                timeout_ms = self.session_config.session_timeout.as_millis(),
                "Started session timeout tracking"
            );
        }
    }

    /// Update session activity
    pub fn update_session_activity(&self, session_id: &str) {
        if !self.config.enable_session_timeout {
            return;
        }

        let mut sessions = self.session_tracker.write();
        if let Some(session_timeout) = sessions.get_mut(session_id) {
            session_timeout.last_accessed = Instant::now();
        }
    }

    /// Check session timeout
    pub fn is_session_timed_out(&self, session_id: &str) -> bool {
        if !self.config.enable_session_timeout {
            return false;
        }

        let sessions = self.session_tracker.read();
        if let Some(session_timeout) = sessions.get(session_id) {
            let elapsed = session_timeout.last_accessed.elapsed();
            elapsed > session_timeout.timeout_duration
        } else {
            false
        }
    }

    /// Start database operation timeout tracking
    pub fn start_database_timeout(&self, operation_id: String, operation_type: String) {
        if !self.config.enable_database_timeout {
            return;
        }

        let db_timeout = DatabaseTimeout {
            operation_id: operation_id.clone(),
            started_at: Instant::now(),
            timeout_duration: self.session_config.database_timeout,
            operation_type: operation_type.clone(),
        };

        let mut database_ops = self.database_tracker.write();
        database_ops.insert(operation_id.clone(), db_timeout);

        if self.config.log_timeout_events {
            debug!(
                operation_id = %operation_id,
                operation_type = %operation_type,
                timeout_ms = self.session_config.database_timeout.as_millis(),
                "Started database operation timeout tracking"
            );
        }
    }

    /// Stop database operation timeout tracking
    pub fn stop_database_timeout(&self, operation_id: &str) {
        if !self.config.enable_database_timeout {
            return;
        }

        let mut database_ops = self.database_tracker.write();
        if let Some(db_timeout) = database_ops.remove(operation_id) {
            let elapsed = db_timeout.started_at.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    operation_id = %operation_id,
                    operation_type = %db_timeout.operation_type,
                    elapsed_ms = elapsed.as_millis(),
                    "Stopped database operation timeout tracking"
                );
            }
        }
    }

    /// Async timeout wrapper for database operations
    pub async fn with_database_timeout<F, T>(&self, operation_id: String, operation_type: String, future: F) -> TimeoutResult<T>
    where
        F: Future<Output = T>,
    {
        if !self.config.enable_database_timeout {
            return Ok(future.await);
        }

        self.start_database_timeout(operation_id.clone(), operation_type.clone());
        
        let timeout_duration = self.session_config.database_timeout;
        let result = timeout(timeout_duration, future).await;
        
        self.stop_database_timeout(&operation_id);
        
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                let elapsed = timeout_duration; // Maximum elapsed time
                if self.config.log_timeout_events {
                    error!(
                        operation_id = %operation_id,
                        operation_type = %operation_type,
                        elapsed_ms = elapsed.as_millis(),
                        timeout_ms = timeout_duration.as_millis(),
                        "Database operation timed out"
                    );
                }
                Err(TimeoutError::DatabaseTimeout {
                    elapsed,
                    timeout: timeout_duration,
                    operation: operation_type,
                })
            }
        }
    }

    /// Async timeout wrapper for requests
    pub async fn with_request_timeout<F, T>(&self, request_id: String, future: F) -> TimeoutResult<T>
    where
        F: Future<Output = T>,
    {
        if !self.config.enable_request_timeout {
            return Ok(future.await);
        }

        let timeout_duration = self.server_config.request_timeout;
        let start_time = Instant::now();
        
        let result = timeout(timeout_duration, future).await;
        
        match result {
            Ok(value) => Ok(value),
            Err(_) => {
                let elapsed = start_time.elapsed();
                if self.config.log_timeout_events {
                    error!(
                        request_id = %request_id,
                        elapsed_ms = elapsed.as_millis(),
                        timeout_ms = timeout_duration.as_millis(),
                        "Request processing timed out"
                    );
                }
                Err(TimeoutError::RequestTimeout {
                    elapsed,
                    timeout: timeout_duration,
                })
            }
        }
    }

    /// Cleanup expired timeout tracking entries
    pub fn cleanup_expired_timeouts(&self) {
        let now = Instant::now();

        // Cleanup expired requests
        if self.config.enable_request_timeout {
            let mut requests = self.active_requests.write();
            let initial_count = requests.len();
            requests.retain(|_, request_info| {
                now.duration_since(request_info.start_time) <= request_info.timeout_duration
            });
            let removed = initial_count - requests.len();
            if removed > 0 && self.config.log_timeout_events {
                debug!(removed = removed, "Cleaned up expired request timeouts");
            }
        }

        // Cleanup expired connections
        if self.config.enable_connection_timeout {
            let mut connections = self.active_connections.write();
            let initial_count = connections.len();
            connections.retain(|_, connection_info| {
                now.duration_since(connection_info.last_activity) <= connection_info.timeout_duration
            });
            let removed = initial_count - connections.len();
            if removed > 0 && self.config.log_timeout_events {
                debug!(removed = removed, "Cleaned up expired connection timeouts");
            }
        }

        // Cleanup expired sessions
        if self.config.enable_session_timeout {
            let mut sessions = self.session_tracker.write();
            let initial_count = sessions.len();
            sessions.retain(|_, session_timeout| {
                session_timeout.is_active && 
                now.duration_since(session_timeout.last_accessed) <= session_timeout.timeout_duration
            });
            let removed = initial_count - sessions.len();
            if removed > 0 && self.config.log_timeout_events {
                debug!(removed = removed, "Cleaned up expired session timeouts");
            }
        }

        // Cleanup expired database operations
        if self.config.enable_database_timeout {
            let mut database_ops = self.database_tracker.write();
            let initial_count = database_ops.len();
            database_ops.retain(|_, db_timeout| {
                now.duration_since(db_timeout.started_at) <= db_timeout.timeout_duration
            });
            let removed = initial_count - database_ops.len();
            if removed > 0 && self.config.log_timeout_events {
                debug!(removed = removed, "Cleaned up expired database operation timeouts");
            }
        }
    }

    /// Get timeout statistics
    pub fn get_timeout_statistics(&self) -> TimeoutStatistics {
        let requests = self.active_requests.read();
        let connections = self.active_connections.read();
        let sessions = self.session_tracker.read();
        let database_ops = self.database_tracker.read();

        TimeoutStatistics {
            active_requests: requests.len(),
            active_connections: connections.len(),
            active_sessions: sessions.len(),
            active_database_operations: database_ops.len(),
            request_timeout_enabled: self.config.enable_request_timeout,
            connection_timeout_enabled: self.config.enable_connection_timeout,
            session_timeout_enabled: self.config.enable_session_timeout,
            database_timeout_enabled: self.config.enable_database_timeout,
        }
    }
}

/// Timeout statistics for monitoring
#[derive(Debug, Clone)]
pub struct TimeoutStatistics {
    pub active_requests: usize,
    pub active_connections: usize,
    pub active_sessions: usize,
    pub active_database_operations: usize,
    pub request_timeout_enabled: bool,
    pub connection_timeout_enabled: bool,
    pub session_timeout_enabled: bool,
    pub database_timeout_enabled: bool,
}

impl Middleware for TimeoutMiddleware {
    fn before_request(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
    ) -> MiddlewareResult {
        // Start request timeout tracking
        self.start_request_timeout(context);

        // Update connection activity if available
        if let Some(connection_data) = context.get_data("connection_id") {
            if let Some(connection_id) = connection_data.as_string() {
                self.update_connection_activity(&connection_id.to_string());
            }
        }

        // Check session timeout if session ID available
        if let Some(session_data) = context.get_data("session_id") {
            if let Some(session_id) = session_data.as_string() {
                let session_id_string = session_id.to_string();
                if self.is_session_timed_out(&session_id_string) {
                    response.set_status(StatusCode::UNAUTHORIZED);
                    response.set_text("Session has expired");
                    return Err(MiddlewareError::Authentication("Session timeout".to_string()));
                }
                self.update_session_activity(&session_id_string);
            }
        }

        Ok(())
    }

    fn after_response(
        &self,
        context: &RequestContext,
        _response: &mut ResponseContext,
    ) -> MiddlewareResult {
        // Stop request timeout tracking
        self.stop_request_timeout(&context.request_id);

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Timeout"
    }

    fn priority(&self) -> u32 {
        25 // Medium priority - after auth and rate limiting
    }
}

/// Timeout-aware future wrapper
pub struct TimeoutFuture<F> {
    future: Pin<Box<F>>,
    timeout_duration: Duration,
    start_time: Instant,
}

impl<F> TimeoutFuture<F>
where
    F: Future,
{
    pub fn new(future: F, timeout_duration: Duration) -> Self {
        Self {
            future: Box::pin(future),
            timeout_duration,
            start_time: Instant::now(),
        }
    }
}

impl<F> Future for TimeoutFuture<F>
where
    F: Future,
{
    type Output = TimeoutResult<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed = self.start_time.elapsed();
        
        if elapsed > self.timeout_duration {
            return Poll::Ready(Err(TimeoutError::RequestTimeout {
                elapsed,
                timeout: self.timeout_duration,
            }));
        }

        match self.future.as_mut().poll(cx) {
            Poll::Ready(value) => Poll::Ready(Ok(value)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::web_vibez::context::RequestContext;
    use std::thread;

    #[test]
    fn test_timeout_middleware_creation() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        assert_eq!(middleware.name(), "Timeout");
        assert_eq!(middleware.priority(), 25);
    }

    #[test]
    fn test_request_timeout_tracking() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        
        // Start tracking
        middleware.start_request_timeout(&context);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_requests, 1);
        
        // Stop tracking
        middleware.stop_request_timeout(&context.request_id);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_requests, 0);
    }

    #[test]
    fn test_session_timeout() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        let session_id = "test_session_123".to_string();
        
        // Start session tracking
        middleware.start_session_timeout(session_id.clone());
        
        // Should not be timed out immediately
        assert!(!middleware.is_session_timed_out(&session_id));
        
        // Update activity
        middleware.update_session_activity(&session_id);
        assert!(!middleware.is_session_timed_out(&session_id));
    }

    #[test]
    fn test_connection_timeout_tracking() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        let connection_id = "conn_123".to_string();
        let client_ip = Some("192.168.1.1".to_string());
        
        // Start connection tracking
        middleware.start_connection_timeout(connection_id.clone(), client_ip);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_connections, 1);
        
        // Update activity
        middleware.update_connection_activity(&connection_id);
        
        // Stop tracking
        middleware.stop_connection_timeout(&connection_id);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_connections, 0);
    }

    #[test]
    fn test_database_timeout_tracking() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        let operation_id = "db_op_123".to_string();
        let operation_type = "SELECT".to_string();
        
        // Start database operation tracking
        middleware.start_database_timeout(operation_id.clone(), operation_type);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_database_operations, 1);
        
        // Stop tracking
        middleware.stop_database_timeout(&operation_id);
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_database_operations, 0);
    }

    #[test]
    fn test_cleanup_expired_timeouts() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        // Add some timeout tracking
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        middleware.start_request_timeout(&context);
        
        let session_id = "test_session".to_string();
        middleware.start_session_timeout(session_id);
        
        // Initial state
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_requests, 1);
        assert_eq!(stats.active_sessions, 1);
        
        // Cleanup (should not remove anything yet)
        middleware.cleanup_expired_timeouts();
        
        let stats = middleware.get_timeout_statistics();
        assert_eq!(stats.active_requests, 1);
        assert_eq!(stats.active_sessions, 1);
    }

    #[tokio::test]
    async fn test_async_timeout_wrapper() {
        let server_config = ServerConfig::default();
        let session_config = SessionConfig::default();
        let middleware = TimeoutMiddleware::new(server_config, session_config);
        
        // Test successful operation
        let operation_id = "test_op".to_string();
        let operation_type = "TEST".to_string();
        
        let result = middleware.with_database_timeout(
            operation_id,
            operation_type,
            async { "success" }
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
}
