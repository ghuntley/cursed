/// fr fr Advanced connection features with middleware, logging, health checks, and load balancing
/// This module provides production-ready connection management with comprehensive monitoring

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use tracing::{instrument, debug, info, warn, error, trace};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};

/// fr fr Connection middleware trait for intercepting operations
pub trait ConnectionMiddleware: Send + Sync + std::fmt::Debug {
    /// sus Before connection operation
    fn before_operation(&self, context: &OperationContext) -> crate::error::Result<()>;
    
    /// facts After connection operation
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> crate::error::Result<()>;
    
    /// lowkey On connection error
    fn on_error(&self, context: &OperationContext, error: &DatabaseError) -> crate::error::Result<()>;
/// fr fr Operation context for middleware
#[derive(Debug, Clone)]
pub struct OperationContext {
/// fr fr Operation types for monitoring
#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
/// fr fr Operation result for post-processing
#[derive(Debug)]
pub struct OperationResult {
/// fr fr Query logging middleware for comprehensive monitoring
#[derive(Debug)]
pub struct QueryLoggingMiddleware {
impl QueryLoggingMiddleware {
    /// slay Create new query logging middleware
    #[instrument]
    pub fn new(slow_query_threshold: Duration, log_all: bool) -> Self {
        info!(
            "Creating query logging middleware"
        );
        
        Self {
            sensitive_fields: vec![
        }
    }

    /// facts Sanitize SQL for logging (remove sensitive data)
    fn sanitize_sql(&self, sql: &str) -> String {
        let mut sanitized = sql.to_string();
        
        for field in &self.sensitive_fields {
            // Simple pattern to replace sensitive field values
            let pattern = format!(r"{}\s*=\s*'[^']*'", field);
            sanitized = sanitized.replace(&pattern, &format!("{} = '[REDACTED]'", field));
        sanitized
    }
}

impl ConnectionMiddleware for QueryLoggingMiddleware {
    #[instrument(skip(self, context))]
    fn before_operation(&self, context: &OperationContext) -> crate::error::Result<()> {
        if self.log_all_queries {
            if let Some(ref sql) = context.sql {
                let sanitized_sql = self.sanitize_sql(sql);
                debug!(
                    "Starting database operation"
                );
            }
        }
        Ok(())
    #[instrument(skip(self, context, result))]
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> crate::error::Result<()> {
        let should_log = self.log_all_queries || 
            (self.log_slow_queries && result.duration >= self.slow_query_threshold);

        if should_log {
            if let Some(ref sql) = context.sql {
                let sanitized_sql = self.sanitize_sql(sql);
                info!(
                    "Database operation completed"
                );
            }
        }

        if result.duration >= self.slow_query_threshold {
            warn!(
                "Slow query detected"
            );
        Ok(())
    #[instrument(skip(self, context, error))]
    fn on_error(&self, context: &OperationContext, error: &DatabaseError) -> crate::error::Result<()> {
        if let Some(ref sql) = context.sql {
            let sanitized_sql = self.sanitize_sql(sql);
            error!(
                "Database operation failed"
            );
        }
        Ok(())
    }
}

/// fr fr Performance profiling middleware
#[derive(Debug)]
pub struct PerformanceProfilingMiddleware {
#[derive(Debug, Clone)]
pub struct OperationStats {
#[derive(Debug, Clone)]
pub struct ConnectionStats {
impl PerformanceProfilingMiddleware {
    /// slay Create new performance profiling middleware
    #[instrument]
    pub fn new() -> Self {
        info!("Creating performance profiling middleware");
        Self {
        }
    }

    /// facts Get performance statistics
    #[instrument(skip(self))]
    pub fn get_stats(&self) -> PerformanceReport {
        let operation_stats = self.operation_stats.lock()
            .map(|stats| stats.clone())
            .unwrap_or_default();
            
        let connection_stats = self.connection_stats.lock()
            .map(|stats| stats.clone())
            .unwrap_or_default();

        PerformanceReport {
        }
    }
impl ConnectionMiddleware for PerformanceProfilingMiddleware {
    #[instrument(skip(self, context))]
    fn before_operation(&self, context: &OperationContext) -> crate::error::Result<()> {
        if let Ok(mut conn_stats) = self.connection_stats.lock() {
            let stats = conn_stats.entry(context.connection_id.clone())
                .or_insert_with(|| ConnectionStats {
                });
            
            stats.active_operations += 1;
        }
        Ok(())
    #[instrument(skip(self, context, result))]
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> crate::error::Result<()> {
        // Update operation stats
        if let Ok(mut op_stats) = self.operation_stats.lock() {
            let stats = op_stats.entry(context.operation_type.clone())
                .or_insert_with(|| OperationStats {
                });
            
            stats.total_operations += 1;
            if result.success {
                stats.successful_operations += 1;
            } else {
                stats.failed_operations += 1;
            stats.total_duration += result.duration;
            stats.min_duration = stats.min_duration.min(result.duration);
            stats.max_duration = stats.max_duration.max(result.duration);
            stats.avg_duration = stats.total_duration / stats.total_operations as u32;
        // Update connection stats
        if let Ok(mut conn_stats) = self.connection_stats.lock() {
            if let Some(stats) = conn_stats.get_mut(&context.connection_id) {
                stats.total_operations += 1;
                stats.active_operations = stats.active_operations.saturating_sub(1);
                stats.last_operation_at = Some(Instant::now());
            }
        }

        Ok(())
    #[instrument(skip(self, context, _error))]
    fn on_error(&self, context: &OperationContext, _error: &DatabaseError) -> crate::error::Result<()> {
        if let Ok(mut conn_stats) = self.connection_stats.lock() {
            if let Some(stats) = conn_stats.get_mut(&context.connection_id) {
                stats.active_operations = stats.active_operations.saturating_sub(1);
            }
        }
        Ok(())
    }
}

/// fr fr Performance report structure
#[derive(Debug, Clone)]
pub struct PerformanceReport {
/// fr fr Connection health checker for monitoring connection status
#[derive(Debug)]
pub struct ConnectionHealthChecker {
#[derive(Debug, Clone)]
pub struct ConnectionHealth {
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
impl ConnectionHealthChecker {
    /// slay Create new connection health checker
    #[instrument]
    pub fn new(check_interval: Duration, timeout: Duration) -> Self {
        info!(
            "Creating connection health checker"
        );
        
        Self {
        }
    }

    /// facts Check health of specific connection
    #[instrument(skip(self))]
    pub async fn check_connection_health(&self, connection_id: &str) -> crate::error::Result<()> {
        debug!(connection_id = %connection_id, "Checking connection health");
        
        let start_time = Instant::now();
        
        // Simulate health check query
        let health_check_result = self.execute_health_check_query(connection_id).await;
        let response_time = start_time.elapsed();
        
        let status = match health_check_result {
            Ok(_) => {
                if response_time > self.health_timeout {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                }
            }
        
        // Update health statistics
        self.update_health_stats(connection_id, status.clone(), response_time).await;
        
        debug!(
            "Health check completed"
        );
        
        Ok(status)
    /// periodt Start background health monitoring
    #[instrument(skip(self))]
    pub fn start_background_monitoring(&self, connections: Vec<String>) -> mpsc::Receiver<HealthReport> {
        info!(connection_count = connections.len(), "Starting background health monitoring");
        
        let (tx, rx) = mpsc::channel();
        let health_checker = Arc::new(self.clone());
        let check_interval = self.health_check_interval;
        
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create async runtime");
            
            loop {
                rt.block_on(async {
                    let mut health_report = HealthReport {
                    
                    let mut unhealthy_count = 0;
                    let mut degraded_count = 0;
                    
                    for connection_id in &connections {
                        match health_checker.check_connection_health(connection_id).await {
                            Ok(status) => {
                                match status {
                                    _ => {}
                                }
                                
                                if let Ok(health_map) = health_checker.connection_health.read() {
                                    if let Some(health) = health_map.get(connection_id) {
                                        health_report.connection_healths.insert(
                                            health.clone()
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                error!(connection_id = %connection_id, error = ?e, "Health check failed");
                                unhealthy_count += 1;
                            }
                        }
                    // Determine overall status
                    health_report.overall_status = if unhealthy_count > connections.len() / 2 {
                        HealthStatus::Unhealthy
                    } else if degraded_count > 0 || unhealthy_count > 0 {
                        HealthStatus::Degraded
                    } else {
                        HealthStatus::Healthy
                    
                    if let Err(_) = tx.send(health_report) {
                        warn!("Failed to send health report - receiver dropped");
                        break;
                    tokio::time::sleep(check_interval).await;
                });
            }
        });
        
        rx
    /// lowkey Execute actual health check query
    async fn execute_health_check_query(&self, connection_id: &str) -> crate::error::Result<()> {
        trace!(connection_id = %connection_id, "Executing health check query");
        
        // Simulate health check with small delay
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        // Simulate occasional failures for testing
        if connection_id.contains("unhealthy") {
            return Err(DatabaseError::connection_error("Simulated health check failure"));
        Ok(())
    /// bestie Update health statistics
    async fn update_health_stats(&self, connection_id: &str, status: HealthStatus, response_time: Duration) {
        if let Ok(mut health_map) = self.connection_health.write() {
            let health = health_map.entry(connection_id.to_string())
                .or_insert_with(|| ConnectionHealth {
                });
            
            health.status = status.clone();
            health.last_check = Instant::now();
            health.total_checks += 1;
            
            match status {
                HealthStatus::Healthy => {
                    health.consecutive_successes += 1;
                    health.consecutive_failures = 0;
                    health.successful_checks += 1;
                }
                HealthStatus::Degraded => {
                    health.consecutive_successes += 1;
                    health.consecutive_failures = 0;
                    health.successful_checks += 1;
                }
                HealthStatus::Unhealthy => {
                    health.consecutive_failures += 1;
                    health.consecutive_successes = 0;
                }
                HealthStatus::Unknown => {}
            // Update average response time
            let total_time = health.average_response_time * (health.total_checks - 1) as u32 + response_time;
            health.average_response_time = total_time / health.total_checks as u32;
        }
    }
impl Clone for ConnectionHealthChecker {
    fn clone(&self) -> Self {
        Self {
        }
    }
/// fr fr Health report structure
#[derive(Debug, Clone)]
pub struct HealthReport {
/// fr fr Load balancer for distributing connections across multiple databases
#[derive(Debug)]
pub struct DatabaseLoadBalancer {
#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
#[derive(Debug, Clone)]
pub struct DatabaseNode {
    pub priority: u8, // 0 = highest priority
impl DatabaseLoadBalancer {
    /// slay Create new database load balancer
    #[instrument]
    pub fn new(strategy: LoadBalancingStrategy, nodes: Vec<DatabaseNode>) -> Self {
        info!(
            "Creating database load balancer"
        );
        
        Self {
            health_checker: Arc::new(ConnectionHealthChecker::new(
                Duration::from_secs(5)
        }
    }

    /// facts Get next connection using load balancing strategy
    #[instrument(skip(self))]
    pub async fn get_connection(&self) -> crate::error::Result<()> {
        debug!(strategy = ?self.strategy, "Getting connection using load balancer");
        
        let nodes = self.database_nodes.read()
            .map_err(|_| DatabaseError::connection_error("Failed to read database nodes"))?;
        
        if nodes.is_empty() {
            return Err(DatabaseError::connection_error("No database nodes available"));
        let selected_node = match self.strategy {
        
        // Increment connection count
        if let Ok(mut count) = selected_node.current_connections.lock() {
            *count += 1;
        debug!(
            "Connection selected"
        );
        
        Ok(selected_node.connection_string.clone())
    /// lowkey Round robin selection
    async fn round_robin_selection(&self, nodes: &[DatabaseNode]) -> crate::error::Result<()> {
        let mut index = self.current_index.lock()
            .map_err(|_| DatabaseError::connection_error("Failed to acquire index lock"))?;
        
        let selected_index = *index % nodes.len();
        *index = (*index + 1) % nodes.len();
        
        Ok(&nodes[selected_index])
    /// periodt Weighted round robin selection
    async fn weighted_round_robin_selection(&self, nodes: &[DatabaseNode]) -> crate::error::Result<()> {
        let total_weight: u32 = nodes.iter().map(|n| n.weight).sum();
        
        if total_weight == 0 {
            return self.round_robin_selection(nodes).await;
        let mut index = self.current_index.lock()
            .map_err(|_| DatabaseError::connection_error("Failed to acquire index lock"))?;
        
        *index = (*index + 1) % total_weight as usize;
        let target = *index as u32;
        
        let mut cumulative_weight = 0;
        for node in nodes {
            cumulative_weight += node.weight;
            if target < cumulative_weight {
                return Ok(node);
            }
        }
        
        Ok(&nodes[0]) // Fallback
    /// bestie Least connections selection
    async fn least_connections_selection(&self, nodes: &[DatabaseNode]) -> crate::error::Result<()> {
        let mut min_connections = u32::MAX;
        let mut selected_node = &nodes[0];
        
        for node in nodes {
            if let Ok(connections) = node.current_connections.lock() {
                if *connections < min_connections {
                    min_connections = *connections;
                    selected_node = node;
                }
            }
        Ok(selected_node)
    /// yolo Random selection
    async fn random_selection(&self, nodes: &[DatabaseNode]) -> crate::error::Result<()> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..nodes.len());
        Ok(&nodes[index])
    /// slay Health-aware selection (prefer healthy nodes with lower priority)
    async fn health_aware_selection(&self, nodes: &[DatabaseNode]) -> crate::error::Result<()> {
        // Filter healthy nodes
        let healthy_nodes: Vec<&DatabaseNode> = nodes.iter()
            .filter(|n| n.is_healthy)
            .collect();
        
        if healthy_nodes.is_empty() {
            warn!("No healthy nodes available, falling back to all nodes");
            return self.least_connections_selection(nodes).await;
        // Sort by priority (lower number = higher priority)
        let mut sorted_nodes = healthy_nodes;
        sorted_nodes.sort_by_key(|n| n.priority);
        
        // Use least connections among highest priority nodes
        let highest_priority = sorted_nodes[0].priority;
        let priority_nodes: Vec<&DatabaseNode> = sorted_nodes.into_iter()
            .filter(|n| n.priority == highest_priority)
            .collect();
        
        self.least_connections_selection(&priority_nodes).await
    /// facts Release connection and decrement count
    #[instrument(skip(self))]
    pub async fn release_connection(&self, connection_string: &str) -> crate::error::Result<()> {
        let nodes = self.database_nodes.read()
            .map_err(|_| DatabaseError::connection_error("Failed to read database nodes"))?;
        
        for node in nodes.iter() {
            if node.connection_string == connection_string {
                if let Ok(mut count) = node.current_connections.lock() {
                    *count = count.saturating_sub(1);
                }
                break;
            }
        }
        
        Ok(())
    /// highkey Get load balancer statistics
    #[instrument(skip(self))]
    pub fn get_stats(&self) -> LoadBalancerStats {
        let nodes = self.database_nodes.read()
            .map(|nodes| nodes.clone())
            .unwrap_or_default();
        
        let mut node_stats = HashMap::new();
        let mut total_connections = 0;
        let mut healthy_nodes = 0;
        
        for node in &nodes {
            let current_connections = node.current_connections.lock()
                .map(|c| *c)
                .unwrap_or(0);
            
            total_connections += current_connections;
            
            if node.is_healthy {
                healthy_nodes += 1;
            node_stats.insert(node.id.clone(), NodeStats {
            });
        LoadBalancerStats {
        }
    }
/// fr fr Load balancer statistics
#[derive(Debug, Clone)]
pub struct LoadBalancerStats {
#[derive(Debug, Clone)]
pub struct NodeStats {
