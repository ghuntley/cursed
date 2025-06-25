use crate::web::StatusCode;
/// Comprehensive timeout middleware for HTTP request processing
/// 
/// Provides timeout mechanisms for requests, connections, sessions, and database operations
/// using parking_lot crate's timeout support and async-aware implementations

// use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
// use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareResult};
// use crate::stdlib::web_vibez::config::{ServerConfig, SessionConfig};
use crate::error::CursedError;
// use crate::stdlib::web_vibez::StatusCode;

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
    /// Session configuration with timeout values
    /// Active request tracking
    /// Active connection tracking
    /// Session timeout tracking
    /// Database operation timeout tracking
    /// Timeout configuration
    /// Condition variable for timeout notifications
/// Timeout configuration options
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Enable request timeouts
    /// Enable connection timeouts
    /// Enable session timeouts
    /// Enable database timeouts
    /// Graceful shutdown timeout
    /// Cleanup interval for expired timeouts
    /// Log timeout events
impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Information about active requests
#[derive(Debug, Clone)]
struct RequestInfo {
/// Information about active connections
#[derive(Debug, Clone)]
struct ConnectionInfo {
/// Session timeout tracking
#[derive(Debug, Clone)]
struct SessionTimeout {
/// Database operation timeout tracking
#[derive(Debug, Clone)]
struct DatabaseTimeout {
/// Timeout result for async operations
pub type TimeoutResult<T> = std::result::Result<T, TimeoutError>;

/// Timeout error types
#[derive(Debug, Clone)]
pub enum TimeoutError {
// impl std::fmt::Display for TimeoutError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             TimeoutError::RequestTimeout { elapsed, timeout } => {
//                 write!(f, "Request timeout: {}ms elapsed, {}ms timeout", 
//                        elapsed.as_millis(), timeout.as_millis())
//             }
//             TimeoutError::ConnectionTimeout { elapsed, timeout } => {
//                 write!(f, "Connection timeout: {}ms elapsed, {}ms timeout", 
//                        elapsed.as_millis(), timeout.as_millis())
//             }
//             TimeoutError::SessionTimeout { elapsed, timeout } => {
//                 write!(f, "Session timeout: {}ms elapsed, {}ms timeout", 
//                        elapsed.as_millis(), timeout.as_millis())
//             }
//             TimeoutError::DatabaseTimeout { elapsed, timeout, operation } => {
//                 write!(f, "Database operation '{}' timeout: {}ms elapsed, {}ms timeout", 
//                        operation, elapsed.as_millis(), timeout.as_millis())
//             }
//             TimeoutError::GracefulShutdownTimeout => {
//                 write!(f, "Graceful shutdown timeout exceeded")
//             }
//         }
//     }
// }

// impl std::error::CursedError for TimeoutError {}
// 
impl TimeoutMiddleware {
    /// Create new timeout middleware with configurations
    pub fn new(server_config: ServerConfig, session_config: SessionConfig) -> Self {
        Self {
        }
    }

    /// Configure timeout settings
    pub fn with_config(mut self, config: TimeoutConfig) -> Self {
        self.config = config;
        self
    /// Start request timeout tracking
    fn start_request_timeout(&self, context: &RequestContext) {
        if !self.config.enable_request_timeout {
            return;
        let request_info = RequestInfo {

        let mut requests = self.active_requests.write();
        requests.insert(context.request_id.clone(), request_info);

        if self.config.log_timeout_events {
            debug!(
                "Started request timeout tracking"
            );
        }
    }

    /// Stop request timeout tracking
    fn stop_request_timeout(&self, request_id: &str) {
        if !self.config.enable_request_timeout {
            return;
        let mut requests = self.active_requests.write();
        if let Some(request_info) = requests.remove(request_id) {
            let elapsed = request_info.start_time.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    "Stopped request timeout tracking"
                );
            }
        }
    /// Check for timed out requests
    fn check_request_timeouts(&self) -> Vec<String> {
        if !self.config.enable_request_timeout {
            return Vec::new();
        let requests = self.active_requests.read();
        let now = Instant::now();
        let mut timed_out = Vec::new();

        for (request_id, request_info) in requests.iter() {
            let elapsed = now.duration_since(request_info.start_time);
            if elapsed > request_info.timeout_duration {
                timed_out.push(request_id.clone());
                
                if self.config.log_timeout_events {
                    warn!(
                        "Request timeout detected"
                    );
                }
            }
        timed_out
    /// Start connection timeout tracking
    pub fn start_connection_timeout(&self, connection_id: String, client_ip: Option<String>) {
        if !self.config.enable_connection_timeout {
            return;
        let connection_info = ConnectionInfo {

        let mut connections = self.active_connections.write();
        connections.insert(connection_id.clone(), connection_info);

        if self.config.log_timeout_events {
            debug!(
                "Started connection timeout tracking"
            );
        }
    }

    /// Update connection activity
    pub fn update_connection_activity(&self, connection_id: &str) {
        if !self.config.enable_connection_timeout {
            return;
        let mut connections = self.active_connections.write();
        if let Some(connection_info) = connections.get_mut(connection_id) {
            connection_info.last_activity = Instant::now();
        }
    }

    /// Stop connection timeout tracking
    pub fn stop_connection_timeout(&self, connection_id: &str) {
        if !self.config.enable_connection_timeout {
            return;
        let mut connections = self.active_connections.write();
        if let Some(connection_info) = connections.remove(connection_id) {
            let elapsed = connection_info.established_at.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    "Stopped connection timeout tracking"
                );
            }
        }
    /// Start session timeout tracking
    pub fn start_session_timeout(&self, session_id: String) {
        if !self.config.enable_session_timeout {
            return;
        let session_timeout = SessionTimeout {

        let mut sessions = self.session_tracker.write();
        sessions.insert(session_id.clone(), session_timeout);

        if self.config.log_timeout_events {
            debug!(
                "Started session timeout tracking"
            );
        }
    }

    /// Update session activity
    pub fn update_session_activity(&self, session_id: &str) {
        if !self.config.enable_session_timeout {
            return;
        let mut sessions = self.session_tracker.write();
        if let Some(session_timeout) = sessions.get_mut(session_id) {
            session_timeout.last_accessed = Instant::now();
        }
    }

    /// Check session timeout
    pub fn is_session_timed_out(&self, session_id: &str) -> bool {
        if !self.config.enable_session_timeout {
            return false;
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
        let db_timeout = DatabaseTimeout {

        let mut database_ops = self.database_tracker.write();
        database_ops.insert(operation_id.clone(), db_timeout);

        if self.config.log_timeout_events {
            debug!(
                "Started database operation timeout tracking"
            );
        }
    }

    /// Stop database operation timeout tracking
    pub fn stop_database_timeout(&self, operation_id: &str) {
        if !self.config.enable_database_timeout {
            return;
        let mut database_ops = self.database_tracker.write();
        if let Some(db_timeout) = database_ops.remove(operation_id) {
            let elapsed = db_timeout.started_at.elapsed();
            
            if self.config.log_timeout_events {
                debug!(
                    "Stopped database operation timeout tracking"
                );
            }
        }
    /// Async timeout wrapper for database operations
    pub async fn with_database_timeout<F, T>(&self, operation_id: String, operation_type: String, future: F) -> TimeoutResult<T>
    where
    {
        if !self.config.enable_database_timeout {
            return Ok(future.await);
        self.start_database_timeout(operation_id.clone(), operation_type.clone());
        
        let timeout_duration = self.session_config.database_timeout;
        let result = timeout(timeout_duration, future).await;
        
        self.stop_database_timeout(&operation_id);
        
        match result {
            Err(_) => {
                let elapsed = timeout_duration; // Maximum elapsed time
                if self.config.log_timeout_events {
                    error!(
                        "Database operation timed out"
                    );
                }
                Err(TimeoutError::DatabaseTimeout {
                })
            }
        }
    /// Async timeout wrapper for requests
    pub async fn with_request_timeout<F, T>(&self, request_id: String, future: F) -> TimeoutResult<T>
    where
    {
        if !self.config.enable_request_timeout {
            return Ok(future.await);
        let timeout_duration = self.server_config.request_timeout;
        let start_time = Instant::now();
        
        let result = timeout(timeout_duration, future).await;
        
        match result {
            Err(_) => {
                let elapsed = start_time.elapsed();
                if self.config.log_timeout_events {
                    error!(
                        "Request processing timed out"
                    );
                }
                Err(TimeoutError::RequestTimeout {
                })
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
    /// Get timeout statistics
    pub fn get_timeout_statistics(&self) -> TimeoutStatistics {
        let requests = self.active_requests.read();
        let connections = self.active_connections.read();
        let sessions = self.session_tracker.read();
        let database_ops = self.database_tracker.read();

        TimeoutStatistics {
        }
    }
/// Timeout statistics for monitoring
#[derive(Debug, Clone)]
pub struct TimeoutStatistics {
impl Middleware for TimeoutMiddleware {
    fn before_request(
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
                    response.set_status(StatusCode::Unauthorized);
                    response.set_text("Session has expired");
                    return Err(MiddlewareError::Authentication("Session timeout".to_string()));
                }
                self.update_session_activity(&session_id_string);
            }
        }

        Ok(())
    fn after_response(
    ) -> MiddlewareResult {
        // Stop request timeout tracking
        self.stop_request_timeout(&context.request_id);

        Ok(())
    fn name(&self) -> &'static str {
        "Timeout"
    fn priority(&self) -> u32 {
        25 // Medium priority - after auth and rate limiting
    }
}

/// Timeout-aware future wrapper
pub struct TimeoutFuture<F> {
impl<F> TimeoutFuture<F>
where
{
    pub fn new(future: F, timeout_duration: Duration) -> Self {
        Self {
        }
    }
impl<F> Future for TimeoutFuture<F>
where
{
    type Output = TimeoutResult<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed = self.start_time.elapsed();
        
        if elapsed > self.timeout_duration {
            return Poll::Ready(Err(TimeoutError::RequestTimeout {
            }));
        match self.future.as_mut().poll(cx) {
        }
    }
