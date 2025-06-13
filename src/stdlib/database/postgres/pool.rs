/// PostgreSQL Connection Pool Implementation
/// 
/// Provides high-performance connection pooling for PostgreSQL using bb8 with
/// comprehensive monitoring, health checking, and configurable pool behavior.

use std::sync::Arc;
use std::time::{Duration, Instant};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Client};
use super::config::{PostgresConfig, SslMode};
use super::error::{PostgresError, PostgresErrorKind, PostgresResult};
use super::connection::PostgresConnection;

/// PostgreSQL connection pool configuration
#[derive(Debug, Clone)]
pub struct PostgresPoolConfig {
    /// Maximum number of connections in the pool
    pub max_size: u32,
    /// Minimum number of connections to maintain
    pub min_idle: Option<u32>,
    /// Maximum lifetime of a connection
    pub max_lifetime: Option<Duration>,
    /// Maximum idle time before connection is closed
    pub idle_timeout: Option<Duration>,
    /// Timeout for getting connection from pool
    pub connection_timeout: Duration,
    /// Test connections before use
    pub test_on_check_out: bool,
    /// Test connections while idle
    pub test_while_idle: bool,
    /// Interval for testing idle connections
    pub test_idle_interval: Duration,
    /// Number of connection retries
    pub retry_connection: u32,
    /// Delay between connection retries
    pub retry_delay: Duration,
}

impl Default for PostgresPoolConfig {
    fn default() -> Self {
        Self {
            max_size: 100,
            min_idle: Some(10),
            max_lifetime: Some(Duration::from_secs(3600)), // 1 hour
            idle_timeout: Some(Duration::from_secs(600)),   // 10 minutes
            connection_timeout: Duration::from_secs(30),
            test_on_check_out: true,
            test_while_idle: true,
            test_idle_interval: Duration::from_secs(300), // 5 minutes
            retry_connection: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

impl PostgresPoolConfig {
    /// Create pool configuration from PostgreSQL configuration
    pub fn from_postgres_config(config: &PostgresConfig) -> Self {
        Self {
            max_size: config.max_connections,
            min_idle: Some(config.min_connections),
            max_lifetime: config.max_lifetime,
            idle_timeout: config.idle_timeout,
            connection_timeout: config.connect_timeout,
            retry_connection: config.retry_attempts,
            retry_delay: config.retry_delay,
            ..Default::default()
        }
    }

    /// Convert to bb8 pool builder
    pub fn to_bb8_builder(&self) -> bb8::Builder<PostgresConnectionManager<NoTls>> {
        let mut builder = bb8::Pool::builder()
            .max_size(self.max_size)
            .connection_timeout(self.connection_timeout)
            .test_on_check_out(self.test_on_check_out);

        if let Some(min_idle) = self.min_idle {
            builder = builder.min_idle(Some(min_idle));
        }

        if let Some(max_lifetime) = self.max_lifetime {
            builder = builder.max_lifetime(Some(max_lifetime));
        }

        if let Some(idle_timeout) = self.idle_timeout {
            builder = builder.idle_timeout(Some(idle_timeout));
        }

        builder
    }
}

/// PostgreSQL connection pool with comprehensive monitoring
pub struct PostgresPool {
    /// Underlying bb8 connection pool
    pool: Pool<PostgresConnectionManager<NoTls>>,
    /// Pool configuration
    config: PostgresPoolConfig,
    /// PostgreSQL configuration
    pg_config: PostgresConfig,
    /// Pool creation time
    created_at: Instant,
    /// Pool statistics
    stats: Arc<std::sync::Mutex<PoolStatistics>>,
}

/// Pool statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct PoolStatistics {
    /// Total connections created
    pub total_connections_created: u64,
    /// Total connections closed
    pub total_connections_closed: u64,
    /// Total successful checkouts
    pub total_checkouts: u64,
    /// Total failed checkouts
    pub total_checkout_failures: u64,
    /// Total timeout errors
    pub total_timeouts: u64,
    /// Total connection errors
    pub total_connection_errors: u64,
    /// Average checkout time (milliseconds)
    pub avg_checkout_time_ms: f64,
    /// Peak concurrent connections
    pub peak_connections: u32,
    /// Current active connections
    pub current_active: u32,
    /// Current idle connections
    pub current_idle: u32,
}

impl PostgresPool {
    /// Create new PostgreSQL connection pool
    pub async fn new(config: PostgresConfig) -> PostgresResult<Self> {
        config.validate()?;
        
        let pool_config = PostgresPoolConfig::from_postgres_config(&config);
        let tokio_config = config.to_tokio_config();
        
        // Create connection manager
        let manager = match config.ssl_mode {
            SslMode::Disable => {
                PostgresConnectionManager::new(tokio_config, NoTls)
            }
            _ => {
                // For SSL modes, we would need rustls or native-tls
                // For now, use NoTls and log a warning
                log::warn!("SSL mode {:?} requested but SSL support not implemented, using plain connection", config.ssl_mode);
                PostgresConnectionManager::new(tokio_config, NoTls)
            }
        };

        // Build pool
        let pool = pool_config
            .to_bb8_builder()
            .build(manager)
            .await
            .map_err(|e| PostgresError::new(
                PostgresErrorKind::PoolError,
                &format!("Failed to create connection pool: {}", e),
            ))?;

        Ok(Self {
            pool,
            config: pool_config,
            pg_config: config,
            created_at: Instant::now(),
            stats: Arc::new(std::sync::Mutex::new(PoolStatistics::default())),
        })
    }

    /// Get connection from pool
    pub async fn get_connection(&self) -> PostgresResult<PooledPostgresConnection> {
        let start_time = Instant::now();
        
        let connection = self.pool.get().await.map_err(|e| {
            self.update_stats(|stats| {
                stats.total_checkout_failures += 1;
                if e.to_string().contains("timeout") {
                    stats.total_timeouts += 1;
                }
            });
            PostgresError::from_bb8_error(e)
        })?;

        let checkout_time = start_time.elapsed();
        self.update_stats(|stats| {
            stats.total_checkouts += 1;
            stats.avg_checkout_time_ms = (stats.avg_checkout_time_ms + checkout_time.as_millis() as f64) / 2.0;
        });

        Ok(PooledPostgresConnection {
            connection,
            pool_stats: Arc::clone(&self.stats),
        })
    }

    /// Get pool statistics
    pub fn get_statistics(&self) -> PoolStatistics {
        let stats = self.stats.lock().unwrap();
        let mut result = stats.clone();
        
        // Update current pool state
        let state = self.pool.state();
        result.current_active = state.connections - state.idle_connections;
        result.current_idle = state.idle_connections;
        
        if result.current_active > result.peak_connections {
            result.peak_connections = result.current_active;
        }
        
        result
    }

    /// Get pool health information
    pub fn get_health(&self) -> PoolHealth {
        let stats = self.get_statistics();
        let state = self.pool.state();
        let uptime = self.created_at.elapsed();
        
        let health_score = self.calculate_health_score(&stats, &state);
        
        PoolHealth {
            is_healthy: health_score > 0.7,
            health_score,
            uptime,
            total_connections: state.connections,
            active_connections: stats.current_active,
            idle_connections: stats.current_idle,
            max_connections: self.config.max_size,
            checkout_success_rate: if stats.total_checkouts > 0 {
                (stats.total_checkouts as f64 - stats.total_checkout_failures as f64) / stats.total_checkouts as f64
            } else {
                1.0
            },
            avg_checkout_time_ms: stats.avg_checkout_time_ms,
            connection_errors: stats.total_connection_errors,
            timeout_errors: stats.total_timeouts,
        }
    }

    /// Test pool connectivity
    pub async fn test_connectivity(&self) -> PostgresResult<()> {
        let connection = self.get_connection().await?;
        connection.ping().await?;
        Ok(())
    }

    /// Close all connections and shutdown pool
    pub async fn close(&self) {
        // bb8 doesn't provide explicit close method
        // Connections will be closed when pool is dropped
        log::info!("PostgreSQL connection pool shutting down");
    }

    /// Update pool statistics
    fn update_stats<F>(&self, updater: F) 
    where
        F: FnOnce(&mut PoolStatistics),
    {
        if let Ok(mut stats) = self.stats.lock() {
            updater(&mut stats);
        }
    }

    /// Calculate pool health score (0.0 to 1.0)
    fn calculate_health_score(&self, stats: &PoolStatistics, state: &bb8::State) -> f64 {
        let mut score = 1.0;
        
        // Penalize high failure rate
        if stats.total_checkouts > 0 {
            let failure_rate = stats.total_checkout_failures as f64 / stats.total_checkouts as f64;
            score -= failure_rate * 0.3;
        }
        
        // Penalize high timeout rate
        if stats.total_checkouts > 0 {
            let timeout_rate = stats.total_timeouts as f64 / stats.total_checkouts as f64;
            score -= timeout_rate * 0.2;
        }
        
        // Penalize high connection utilization
        if self.config.max_size > 0 {
            let utilization = state.connections as f64 / self.config.max_size as f64;
            if utilization > 0.9 {
                score -= (utilization - 0.9) * 0.5;
            }
        }
        
        // Penalize slow checkout times
        if stats.avg_checkout_time_ms > 1000.0 {
            score -= (stats.avg_checkout_time_ms - 1000.0) / 10000.0;
        }
        
        score.max(0.0).min(1.0)
    }
}

/// Pooled PostgreSQL connection wrapper
pub struct PooledPostgresConnection {
    connection: PooledConnection<'static, PostgresConnectionManager<NoTls>>,
    pool_stats: Arc<std::sync::Mutex<PoolStatistics>>,
}

impl PooledPostgresConnection {
    /// Get underlying client
    pub fn client(&self) -> &Client {
        &self.connection
    }

    /// Execute simple query
    pub async fn execute(&self, query: &str) -> PostgresResult<u64> {
        self.connection
            .execute(query, &[])
            .await
            .map_err(PostgresError::from)
    }

    /// Execute query with parameters
    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> PostgresResult<Vec<tokio_postgres::Row>> {
        self.connection
            .query(query, params)
            .await
            .map_err(PostgresError::from)
    }

    /// Ping the connection
    pub async fn ping(&self) -> PostgresResult<()> {
        self.connection
            .execute("SELECT 1", &[])
            .await
            .map(|_| ())
            .map_err(PostgresError::from)
    }

    /// Begin transaction
    pub async fn begin_transaction(&self) -> PostgresResult<tokio_postgres::Transaction<'_>> {
        self.connection
            .transaction()
            .await
            .map_err(PostgresError::from)
    }
}

impl Drop for PooledPostgresConnection {
    fn drop(&mut self) {
        // Connection automatically returned to pool by bb8
        if let Ok(mut stats) = self.pool_stats.lock() {
            stats.total_connections_closed += 1;
        }
    }
}

/// Pool health information
#[derive(Debug, Clone)]
pub struct PoolHealth {
    pub is_healthy: bool,
    pub health_score: f64,
    pub uptime: Duration,
    pub total_connections: u32,
    pub active_connections: u32,
    pub idle_connections: u32,
    pub max_connections: u32,
    pub checkout_success_rate: f64,
    pub avg_checkout_time_ms: f64,
    pub connection_errors: u64,
    pub timeout_errors: u64,
}

impl std::fmt::Display for PoolHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PostgreSQL Pool Health:")?;
        writeln!(f, "  Status: {}", if self.is_healthy { "Healthy" } else { "Unhealthy" })?;
        writeln!(f, "  Health Score: {:.1}%", self.health_score * 100.0)?;
        writeln!(f, "  Uptime: {:?}", self.uptime)?;
        writeln!(f, "  Connections: {}/{} (active: {}, idle: {})", 
                 self.total_connections, self.max_connections, 
                 self.active_connections, self.idle_connections)?;
        writeln!(f, "  Checkout Success Rate: {:.1}%", self.checkout_success_rate * 100.0)?;
        writeln!(f, "  Avg Checkout Time: {:.1}ms", self.avg_checkout_time_ms)?;
        writeln!(f, "  Errors: {} (timeouts: {})", self.connection_errors, self.timeout_errors)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_config_defaults() {
        let config = PostgresPoolConfig::default();
        assert_eq!(config.max_size, 100);
        assert_eq!(config.min_idle, Some(10));
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_pool_config_from_postgres_config() {
        let pg_config = PostgresConfig {
            max_connections: 50,
            min_connections: 5,
            ..Default::default()
        };
        
        let pool_config = PostgresPoolConfig::from_postgres_config(&pg_config);
        assert_eq!(pool_config.max_size, 50);
        assert_eq!(pool_config.min_idle, Some(5));
    }

    #[test]
    fn test_pool_statistics() {
        let stats = PoolStatistics::default();
        assert_eq!(stats.total_connections_created, 0);
        assert_eq!(stats.total_checkouts, 0);
        assert_eq!(stats.avg_checkout_time_ms, 0.0);
    }

    #[tokio::test]
    async fn test_pool_creation() {
        let config = PostgresConfig::default();
        
        // This will fail without a real PostgreSQL server, but tests the creation logic
        let result = PostgresPool::new(config).await;
        
        // Expect connection failure, not configuration error
        if let Err(err) = result {
            assert!(matches!(err.kind, PostgresErrorKind::ConnectionFailed | PostgresErrorKind::PoolError));
        }
    }
}
