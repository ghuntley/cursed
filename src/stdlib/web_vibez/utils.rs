/// General utilities for HTTP server development
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Connection pool manager for efficient connection reuse
pub struct ConnectionPool {
    pools: HashMap<String, Vec<PooledConnection>>,
    max_connections_per_host: usize,
    connection_timeout: Duration,
    idle_timeout: Duration,
    total_connections: usize,
    max_total_connections: usize,
}

#[derive(Debug, Clone)]
pub struct PooledConnection {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub created_at: SystemTime,
    pub last_used: SystemTime,
    pub is_active: bool,
    pub connection_count: u64,
}

impl ConnectionPool {
    /// Create new connection pool
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            max_connections_per_host: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300), // 5 minutes
            total_connections: 0,
            max_total_connections: 100,
        }
    }

    /// Configure connection pool limits
    pub fn with_limits(
        mut self,
        max_per_host: usize,
        max_total: usize,
        idle_timeout: Duration,
    ) -> Self {
        self.max_connections_per_host = max_per_host;
        self.max_total_connections = max_total;
        self.idle_timeout = idle_timeout;
        self
    }

    /// Get or create connection for host
    pub fn get_connection(&mut self, host: &str, port: u16) -> Option<PooledConnection> {
        self.cleanup_idle_connections();

        let host_key = format!("{}:{}", host, port);
        
        // Try to get existing connection from pool
        if let Some(connections) = self.pools.get_mut(&host_key) {
            if let Some(mut connection) = connections.pop() {
                connection.last_used = SystemTime::now();
                connection.is_active = true;
                connection.connection_count += 1;
                return Some(connection);
            }
        }

        // Create new connection if under limits
        if self.total_connections < self.max_total_connections {
            let connection = PooledConnection {
                id: format!("conn_{}_{}", host_key, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()),
                host: host.to_string(),
                port,
                created_at: SystemTime::now(),
                last_used: SystemTime::now(),
                is_active: true,
                connection_count: 1,
            };
            
            self.total_connections += 1;
            Some(connection)
        } else {
            None
        }
    }

    /// Return connection to pool
    pub fn return_connection(&mut self, mut connection: PooledConnection) {
        connection.last_used = SystemTime::now();
        connection.is_active = false;

        let host_key = format!("{}:{}", connection.host, connection.port);
        let host_connections = self.pools.entry(host_key).or_insert_with(Vec::new);

        if host_connections.len() < self.max_connections_per_host {
            host_connections.push(connection);
        } else {
            // Pool is full, drop the connection
            self.total_connections = self.total_connections.saturating_sub(1);
        }
    }

    /// Close and remove connection
    pub fn close_connection(&mut self, connection: &PooledConnection) {
        let host_key = format!("{}:{}", connection.host, connection.port);
        
        if let Some(connections) = self.pools.get_mut(&host_key) {
            connections.retain(|conn| conn.id != connection.id);
        }
        
        self.total_connections = self.total_connections.saturating_sub(1);
    }

    /// Cleanup idle connections
    fn cleanup_idle_connections(&mut self) {
        let now = SystemTime::now();
        let mut connections_removed = 0;

        for connections in self.pools.values_mut() {
            let initial_len = connections.len();
            connections.retain(|conn| {
                now.duration_since(conn.last_used).unwrap_or_default() < self.idle_timeout
            });
            connections_removed += initial_len - connections.len();
        }

        self.pools.retain(|_, connections| !connections.is_empty());
        self.total_connections = self.total_connections.saturating_sub(connections_removed);
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let mut host_counts = HashMap::new();
        let mut total_idle = 0;

        for (host, connections) in &self.pools {
            host_counts.insert(host.clone(), connections.len());
            total_idle += connections.len();
        }

        PoolStats {
            total_connections: self.total_connections,
            idle_connections: total_idle,
            active_connections: self.total_connections - total_idle,
            host_counts,
            max_connections_per_host: self.max_connections_per_host,
            max_total_connections: self.max_total_connections,
        }
    }

    /// Force cleanup all connections
    pub fn clear(&mut self) {
        self.pools.clear();
        self.total_connections = 0;
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Pool statistics
#[derive(Debug)]
pub struct PoolStats {
    pub total_connections: usize,
    pub idle_connections: usize,
    pub active_connections: usize,
    pub host_counts: HashMap<String, usize>,
    pub max_connections_per_host: usize,
    pub max_total_connections: usize,
}

/// Request timeout manager
pub struct RequestTimeoutManager {
    default_timeout: Duration,
    route_timeouts: HashMap<String, Duration>,
    active_requests: HashMap<String, RequestTimeout>,
}

#[derive(Debug, Clone)]
pub struct RequestTimeout {
    pub request_id: String,
    pub started_at: SystemTime,
    pub timeout: Duration,
    pub route: String,
}

impl RequestTimeoutManager {
    /// Create new timeout manager
    pub fn new(default_timeout: Duration) -> Self {
        Self {
            default_timeout,
            route_timeouts: HashMap::new(),
            active_requests: HashMap::new(),
        }
    }

    /// Set timeout for specific route pattern
    pub fn set_route_timeout(&mut self, route_pattern: String, timeout: Duration) {
        self.route_timeouts.insert(route_pattern, timeout);
    }

    /// Start tracking request timeout
    pub fn start_request(&mut self, request_id: String, route: String) -> Duration {
        let timeout = self.route_timeouts.get(&route)
            .copied()
            .unwrap_or(self.default_timeout);

        let request_timeout = RequestTimeout {
            request_id: request_id.clone(),
            started_at: SystemTime::now(),
            timeout,
            route,
        };

        self.active_requests.insert(request_id, request_timeout);
        timeout
    }

    /// Check if request has timed out
    pub fn is_timed_out(&self, request_id: &str) -> bool {
        if let Some(request) = self.active_requests.get(request_id) {
            let elapsed = SystemTime::now()
                .duration_since(request.started_at)
                .unwrap_or_default();
            elapsed > request.timeout
        } else {
            false
        }
    }

    /// Complete request (remove from tracking)
    pub fn complete_request(&mut self, request_id: &str) -> Option<Duration> {
        if let Some(request) = self.active_requests.remove(request_id) {
            Some(SystemTime::now()
                .duration_since(request.started_at)
                .unwrap_or_default())
        } else {
            None
        }
    }

    /// Get all timed out requests
    pub fn get_timed_out_requests(&self) -> Vec<&RequestTimeout> {
        let now = SystemTime::now();
        self.active_requests.values()
            .filter(|request| {
                let elapsed = now.duration_since(request.started_at).unwrap_or_default();
                elapsed > request.timeout
            })
            .collect()
    }

    /// Cleanup old requests
    pub fn cleanup_old_requests(&mut self) {
        let cutoff = SystemTime::now() - Duration::from_secs(3600); // 1 hour
        self.active_requests.retain(|_, request| request.started_at > cutoff);
    }

    /// Get timeout statistics
    pub fn get_stats(&self) -> TimeoutStats {
        let now = SystemTime::now();
        let mut route_counts = HashMap::new();
        let mut total_active = 0;
        let mut total_timed_out = 0;

        for request in self.active_requests.values() {
            total_active += 1;
            *route_counts.entry(request.route.clone()).or_insert(0) += 1;

            let elapsed = now.duration_since(request.started_at).unwrap_or_default();
            if elapsed > request.timeout {
                total_timed_out += 1;
            }
        }

        TimeoutStats {
            active_requests: total_active,
            timed_out_requests: total_timed_out,
            route_counts,
            default_timeout: self.default_timeout,
        }
    }
}

/// Timeout statistics
#[derive(Debug)]
pub struct TimeoutStats {
    pub active_requests: usize,
    pub timed_out_requests: usize,
    pub route_counts: HashMap<String, usize>,
    pub default_timeout: Duration,
}

/// Keep-alive connection manager
pub struct KeepAliveManager {
    keep_alive_timeout: Duration,
    max_keep_alive_requests: u32,
    connections: HashMap<String, KeepAliveConnection>,
}

#[derive(Debug, Clone)]
pub struct KeepAliveConnection {
    pub connection_id: String,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub request_count: u32,
    pub client_ip: String,
}

impl KeepAliveManager {
    /// Create new keep-alive manager
    pub fn new(keep_alive_timeout: Duration, max_requests: u32) -> Self {
        Self {
            keep_alive_timeout,
            max_keep_alive_requests: max_requests,
            connections: HashMap::new(),
        }
    }

    /// Register new keep-alive connection
    pub fn register_connection(&mut self, connection_id: String, client_ip: String) {
        let connection = KeepAliveConnection {
            connection_id: connection_id.clone(),
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            request_count: 0,
            client_ip,
        };

        self.connections.insert(connection_id, connection);
    }

    /// Update connection activity
    pub fn update_activity(&mut self, connection_id: &str) -> bool {
        if let Some(connection) = self.connections.get_mut(connection_id) {
            connection.last_activity = SystemTime::now();
            connection.request_count += 1;

            // Check if connection should be closed
            connection.request_count < self.max_keep_alive_requests
        } else {
            false
        }
    }

    /// Check if connection should be kept alive
    pub fn should_keep_alive(&self, connection_id: &str) -> bool {
        if let Some(connection) = self.connections.get(connection_id) {
            let elapsed = SystemTime::now()
                .duration_since(connection.last_activity)
                .unwrap_or_default();

            elapsed < self.keep_alive_timeout 
                && connection.request_count < self.max_keep_alive_requests
        } else {
            false
        }
    }

    /// Close connection
    pub fn close_connection(&mut self, connection_id: &str) {
        self.connections.remove(connection_id);
    }

    /// Cleanup expired connections
    pub fn cleanup_expired(&mut self) -> usize {
        let now = SystemTime::now();
        let initial_count = self.connections.len();

        self.connections.retain(|_, connection| {
            let elapsed = now.duration_since(connection.last_activity).unwrap_or_default();
            elapsed < self.keep_alive_timeout
        });

        initial_count - self.connections.len()
    }

    /// Get keep-alive statistics
    pub fn get_stats(&self) -> KeepAliveStats {
        let now = SystemTime::now();
        let mut active_connections = 0;
        let mut expired_connections = 0;
        let mut total_requests = 0;

        for connection in self.connections.values() {
            total_requests += connection.request_count;
            
            let elapsed = now.duration_since(connection.last_activity).unwrap_or_default();
            if elapsed < self.keep_alive_timeout {
                active_connections += 1;
            } else {
                expired_connections += 1;
            }
        }

        KeepAliveStats {
            total_connections: self.connections.len(),
            active_connections,
            expired_connections,
            total_requests,
            keep_alive_timeout: self.keep_alive_timeout,
            max_keep_alive_requests: self.max_keep_alive_requests,
        }
    }
}

/// Keep-alive statistics
#[derive(Debug)]
pub struct KeepAliveStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub expired_connections: usize,
    pub total_requests: u32,
    pub keep_alive_timeout: Duration,
    pub max_keep_alive_requests: u32,
}

/// Rate limiter for controlling request rates
pub struct RateLimiter {
    buckets: HashMap<String, TokenBucket>,
    default_capacity: u32,
    default_refill_rate: u32, // tokens per second
    cleanup_interval: Duration,
    last_cleanup: SystemTime,
}

#[derive(Debug, Clone)]
pub struct TokenBucket {
    capacity: u32,
    tokens: f64,
    last_refill: SystemTime,
    refill_rate: u32, // tokens per second
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        Self {
            buckets: HashMap::new(),
            default_capacity: capacity,
            default_refill_rate: refill_rate,
            cleanup_interval: Duration::from_secs(300), // 5 minutes
            last_cleanup: SystemTime::now(),
        }
    }

    /// Check if request is allowed for client
    pub fn is_allowed(&mut self, client_id: &str) -> bool {
        self.maybe_cleanup();

        let bucket = self.buckets.entry(client_id.to_string()).or_insert_with(|| {
            TokenBucket {
                capacity: self.default_capacity,
                tokens: self.default_capacity as f64,
                last_refill: SystemTime::now(),
                refill_rate: self.default_refill_rate,
            }
        });

        bucket.refill();
        
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Set custom rate limit for specific client
    pub fn set_client_limit(&mut self, client_id: String, capacity: u32, refill_rate: u32) {
        let bucket = TokenBucket {
            capacity,
            tokens: capacity as f64,
            last_refill: SystemTime::now(),
            refill_rate,
        };
        self.buckets.insert(client_id, bucket);
    }

    /// Get remaining tokens for client
    pub fn get_remaining_tokens(&mut self, client_id: &str) -> u32 {
        if let Some(bucket) = self.buckets.get_mut(client_id) {
            bucket.refill();
            bucket.tokens.floor() as u32
        } else {
            self.default_capacity
        }
    }

    /// Get time until next token for client
    pub fn time_until_next_token(&self, client_id: &str) -> Duration {
        if let Some(bucket) = self.buckets.get(client_id) {
            if bucket.tokens >= 1.0 {
                Duration::from_secs(0)
            } else {
                Duration::from_secs_f64(1.0 / bucket.refill_rate as f64)
            }
        } else {
            Duration::from_secs(0)
        }
    }

    /// Cleanup old buckets
    fn maybe_cleanup(&mut self) {
        let now = SystemTime::now();
        if now.duration_since(self.last_cleanup).unwrap_or_default() > self.cleanup_interval {
            let cutoff = now - Duration::from_secs(3600); // 1 hour
            self.buckets.retain(|_, bucket| bucket.last_refill > cutoff);
            self.last_cleanup = now;
        }
    }

    /// Get rate limiter statistics
    pub fn get_stats(&self) -> RateLimiterStats {
        let total_clients = self.buckets.len();
        let mut blocked_clients = 0;
        let mut total_tokens = 0.0;

        for bucket in self.buckets.values() {
            if bucket.tokens < 1.0 {
                blocked_clients += 1;
            }
            total_tokens += bucket.tokens;
        }

        RateLimiterStats {
            total_clients,
            blocked_clients,
            average_tokens: if total_clients > 0 {
                total_tokens / total_clients as f64
            } else {
                0.0
            },
            default_capacity: self.default_capacity,
            default_refill_rate: self.default_refill_rate,
        }
    }
}

impl TokenBucket {
    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.last_refill).unwrap_or_default();
        let tokens_to_add = elapsed.as_secs_f64() * self.refill_rate as f64;
        
        self.tokens = (self.tokens + tokens_to_add).min(self.capacity as f64);
        self.last_refill = now;
    }
}

/// Rate limiter statistics
#[derive(Debug)]
pub struct RateLimiterStats {
    pub total_clients: usize,
    pub blocked_clients: usize,
    pub average_tokens: f64,
    pub default_capacity: u32,
    pub default_refill_rate: u32,
}

/// Request ID generator for tracing
pub struct RequestIdGenerator {
    counter: u64,
    prefix: String,
}

impl RequestIdGenerator {
    /// Create new request ID generator
    pub fn new(prefix: String) -> Self {
        Self {
            counter: 0,
            prefix,
        }
    }

    /// Generate next request ID
    pub fn next_id(&mut self) -> String {
        self.counter += 1;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        format!("{}-{}-{}", self.prefix, timestamp, self.counter)
    }

    /// Generate request ID with custom suffix
    pub fn next_id_with_suffix(&mut self, suffix: &str) -> String {
        let base_id = self.next_id();
        format!("{}-{}", base_id, suffix)
    }
}

impl Default for RequestIdGenerator {
    fn default() -> Self {
        Self::new("req".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new().with_limits(2, 10, Duration::from_secs(1));

        // Get connections
        let conn1 = pool.get_connection("example.com", 80).unwrap();
        let conn2 = pool.get_connection("example.com", 80).unwrap();
        
        assert_eq!(pool.stats().total_connections, 2);
        assert_eq!(pool.stats().active_connections, 2);

        // Return connections
        pool.return_connection(conn1);
        pool.return_connection(conn2);
        
        assert_eq!(pool.stats().idle_connections, 2);
    }

    #[test]
    fn test_timeout_manager() {
        let mut manager = RequestTimeoutManager::new(Duration::from_millis(100));
        
        let timeout = manager.start_request("req1".to_string(), "/api/test".to_string());
        assert_eq!(timeout, Duration::from_millis(100));
        
        assert!(!manager.is_timed_out("req1"));
        
        // Wait for timeout
        thread::sleep(Duration::from_millis(150));
        assert!(manager.is_timed_out("req1"));
    }

    #[test]
    fn test_keep_alive_manager() {
        let mut manager = KeepAliveManager::new(Duration::from_secs(30), 100);
        
        manager.register_connection("conn1".to_string(), "127.0.0.1".to_string());
        assert!(manager.should_keep_alive("conn1"));
        
        // Update activity
        assert!(manager.update_activity("conn1"));
        assert!(manager.should_keep_alive("conn1"));
        
        // Close connection
        manager.close_connection("conn1");
        assert!(!manager.should_keep_alive("conn1"));
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(5, 1); // 5 tokens, 1 per second
        
        // Use all tokens
        for _ in 0..5 {
            assert!(limiter.is_allowed("client1"));
        }
        
        // Should be blocked now
        assert!(!limiter.is_allowed("client1"));
        
        // Check remaining tokens
        assert_eq!(limiter.get_remaining_tokens("client1"), 0);
        
        // Different client should have full capacity
        assert!(limiter.is_allowed("client2"));
    }

    #[test]
    fn test_request_id_generator() {
        let mut generator = RequestIdGenerator::new("test".to_string());
        
        let id1 = generator.next_id();
        let id2 = generator.next_id();
        
        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
        assert_ne!(id1, id2);
        
        let id_with_suffix = generator.next_id_with_suffix("special");
        assert!(id_with_suffix.ends_with("-special"));
    }

    #[test]
    fn test_token_bucket_refill() {
        let mut bucket = TokenBucket {
            capacity: 10,
            tokens: 0.0,
            last_refill: SystemTime::now() - Duration::from_secs(1),
            refill_rate: 5, // 5 tokens per second
        };
        
        bucket.refill();
        assert_eq!(bucket.tokens, 5.0); // Should have refilled 5 tokens
        
        // Test capacity limit
        bucket.last_refill = SystemTime::now() - Duration::from_secs(10);
        bucket.refill();
        assert_eq!(bucket.tokens, 10.0); // Should be capped at capacity
    }
}
