use crate::error::CursedError;
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
    /// Minimum number of connections to maintain
    /// Maximum lifetime of a connection
    /// Maximum idle time before connection is closed
    /// Timeout for getting connection from pool
    /// Test connections before use
    /// Test connections while idle
    /// Interval for testing idle connections
    /// Number of connection retries
    /// Delay between connection retries
impl Default for PostgresPoolConfig {
    fn default() -> Self {
        Self {
            max_lifetime: Some(Duration::from_secs(3600)), // 1 hour
            idle_timeout: Some(Duration::from_secs(600)),   // 10 minutes
            test_idle_interval: Duration::from_secs(300), // 5 minutes
        }
    }
impl PostgresPoolConfig {
    /// Create pool configuration from PostgreSQL configuration
    pub fn from_postgres_config(config: &PostgresConfig) -> Self {
        Self {
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
        if let Some(max_lifetime) = self.max_lifetime {
            builder = builder.max_lifetime(Some(max_lifetime));
        if let Some(idle_timeout) = self.idle_timeout {
            builder = builder.idle_timeout(Some(idle_timeout));
        builder
    }
}

/// PostgreSQL connection pool with comprehensive monitoring
pub struct PostgresPool {
    /// Underlying bb8 connection pool
    /// Pool configuration
    /// PostgreSQL configuration
    /// Pool creation time
    /// Pool statistics
/// Pool statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct PoolStatistics {
    /// Total connections created
    /// Total connections closed
    /// Total successful checkouts
    /// Total failed checkouts
    /// Total timeout errors
    /// Total connection errors
    /// Average checkout time (milliseconds)
    /// Peak concurrent connections
    /// Current active connections
    /// Current idle connections
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

        // Build pool
        let pool = pool_config
            .to_bb8_builder()
            .build(manager)
            .await
            .map_err(|e| PostgresError::new(
            ))?;

        Ok(Self {
        })
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
        })
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
        result
    /// Get pool health information
    pub fn get_health(&self) -> PoolHealth {
        let stats = self.get_statistics();
        let state = self.pool.state();
        let uptime = self.created_at.elapsed();
        
        let health_score = self.calculate_health_score(&stats, &state);
        
        PoolHealth {
            checkout_success_rate: if stats.total_checkouts > 0 {
                (stats.total_checkouts as f64 - stats.total_checkout_failures as f64) / stats.total_checkouts as f64
            } else {
                1.0
        }
    }

    /// Test pool connectivity
    pub async fn test_connectivity(&self) -> PostgresResult<()> {
        let connection = self.get_connection().await?;
        connection.ping().await?;
        Ok(())
    /// Close all connections and shutdown pool
    pub async fn close(&self) {
        // bb8 doesn't provide explicit close method
        // Connections will be closed when pool is dropped
        log::info!("PostgreSQL connection pool shutting down");
    /// Update pool statistics
    fn update_stats<F>(&self, updater: F) 
    where
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
        // Penalize high timeout rate
        if stats.total_checkouts > 0 {
            let timeout_rate = stats.total_timeouts as f64 / stats.total_checkouts as f64;
            score -= timeout_rate * 0.2;
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
        score.max(0.0).min(1.0)
    }
}

/// Pooled PostgreSQL connection wrapper
pub struct PooledPostgresConnection {
impl PooledPostgresConnection {
    /// Get underlying client
    pub fn client(&self) -> &Client {
        &self.connection
    /// Execute simple query
    pub async fn execute(&self, query: &str) -> PostgresResult<u64> {
        self.connection
            .execute(query, &[])
            .await
            .map_err(PostgresError::from)
    /// Execute query with parameters
    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> PostgresResult<Vec<tokio_postgres::Row>> {
        self.connection
            .query(query, params)
            .await
            .map_err(PostgresError::from)
    /// Ping the connection
    pub async fn ping(&self) -> PostgresResult<()> {
        self.connection
            .execute("SELECT 1", &[])
            .await
            .map(|_| ())
            .map_err(PostgresError::from)
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
/// Pool health information
#[derive(Debug, Clone)]
pub struct PoolHealth {
impl std::fmt::Display for PoolHealth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PostgreSQL Pool Health:")?;
        writeln!(f, "  Status: {}", if self.is_healthy { "Healthy" } else { "Unhealthy" })?;
        writeln!(f, "  Health Score: {:.1}%", self.health_score * 100.0)?;
        writeln!(f, "  Uptime: {:?}", self.uptime)?;
        writeln!(f, "  Connections: {}/{} (active: {}, idle: {})", 
                 self.active_connections, self.idle_connections)?;
        writeln!(f, "  Checkout Success Rate: {:.1}%", self.checkout_success_rate * 100.0)?;
        writeln!(f, "  Avg Checkout Time: {:.1}ms", self.avg_checkout_time_ms)?;
        writeln!(f, "  Errors: {} (timeouts: {})", self.connection_errors, self.timeout_errors)?;
        Ok(())
    }
}

