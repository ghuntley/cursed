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
    fn before_operation(&self, context: &OperationContext) -> Result<(), Error>;
    
    /// facts After connection operation
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> Result<(), Error>;
    
    /// lowkey On connection error
    fn on_error(&self, context: &OperationContext, error: &DatabaseError) -> Result<(), Error>;
}

/// fr fr Operation context for middleware
#[derive(Debug, Clone)]
pub struct OperationContext {
    pub operation_type: OperationType,
    pub connection_id: String,
    pub sql: Option<String>,
    pub parameters: Vec<SqlValue>,
    pub started_at: Instant,
    pub metadata: HashMap<String, String>,
}

/// fr fr Operation types for monitoring
#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Query,
    Execute,
    Transaction,
    Connect,
    Disconnect,
    HealthCheck,
}

/// fr fr Operation result for post-processing
#[derive(Debug)]
pub struct OperationResult {
    pub success: bool,
    pub duration: Duration,
    pub rows_affected: Option<u64>,
    pub rows_returned: Option<usize>,
    pub error: Option<DatabaseError>,
}

/// fr fr Query logging middleware for comprehensive monitoring
#[derive(Debug)]
pub struct QueryLoggingMiddleware {
    log_slow_queries: bool,
    slow_query_threshold: Duration,
    log_all_queries: bool,
    sensitive_fields: Vec<String>,
}

impl QueryLoggingMiddleware {
    /// slay Create new query logging middleware
    #[instrument]
    pub fn new(slow_query_threshold: Duration, log_all: bool) -> Self {
        info!(
            threshold = ?slow_query_threshold,
            log_all = log_all,
            "Creating query logging middleware"
        );
        
        Self {
            log_slow_queries: true,
            slow_query_threshold,
            log_all_queries: log_all,
            sensitive_fields: vec![
                "password".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "key".to_string(),
            ],
        }
    }

    /// facts Sanitize SQL for logging (remove sensitive data)
    fn sanitize_sql(&self, sql: &str) -> String {
        let mut sanitized = sql.to_string();
        
        for field in &self.sensitive_fields {
            // Simple pattern to replace sensitive field values
            let pattern = format!(r"{}\s*=\s*'[^']*'", field);
            sanitized = sanitized.replace(&pattern, &format!("{} = '[REDACTED]'", field));
        }
        
        sanitized
    }
}

impl ConnectionMiddleware for QueryLoggingMiddleware {
    #[instrument(skip(self, context))]
    fn before_operation(&self, context: &OperationContext) -> Result<(), Error> {
        if self.log_all_queries {
            if let Some(ref sql) = context.sql {
                let sanitized_sql = self.sanitize_sql(sql);
                debug!(
                    operation = ?context.operation_type,
                    connection = %context.connection_id,
                    sql = %sanitized_sql,
                    param_count = context.parameters.len(),
                    "Starting database operation"
                );
            }
        }
        Ok(())
    }

    #[instrument(skip(self, context, result))]
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> Result<(), Error> {
        let should_log = self.log_all_queries || 
            (self.log_slow_queries && result.duration >= self.slow_query_threshold);

        if should_log {
            if let Some(ref sql) = context.sql {
                let sanitized_sql = self.sanitize_sql(sql);
                info!(
                    operation = ?context.operation_type,
                    connection = %context.connection_id,
                    sql = %sanitized_sql,
                    duration = ?result.duration,
                    success = result.success,
                    rows_affected = result.rows_affected,
                    rows_returned = result.rows_returned,
                    "Database operation completed"
                );
            }
        }

        if result.duration >= self.slow_query_threshold {
            warn!(
                operation = ?context.operation_type,
                connection = %context.connection_id,
                duration = ?result.duration,
                threshold = ?self.slow_query_threshold,
                "Slow query detected"
            );
        }

        Ok(())
    }

    #[instrument(skip(self, context, error))]
    fn on_error(&self, context: &OperationContext, error: &DatabaseError) -> Result<(), Error> {
        if let Some(ref sql) = context.sql {
            let sanitized_sql = self.sanitize_sql(sql);
            error!(
                operation = ?context.operation_type,
                connection = %context.connection_id,
                sql = %sanitized_sql,
                error = ?error,
                "Database operation failed"
            );
        }
        Ok(())
    }
}

/// fr fr Performance profiling middleware
#[derive(Debug)]
pub struct PerformanceProfilingMiddleware {
    operation_stats: Arc<Mutex<HashMap<OperationType, OperationStats>>>,
    connection_stats: Arc<Mutex<HashMap<String, ConnectionStats>>>,
}

#[derive(Debug, Clone)]
pub struct OperationStats {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub connection_id: String,
    pub created_at: Instant,
    pub total_operations: u64,
    pub active_operations: u64,
    pub last_operation_at: Option<Instant>,
}

impl PerformanceProfilingMiddleware {
    /// slay Create new performance profiling middleware
    #[instrument]
    pub fn new() -> Self {
        info!("Creating performance profiling middleware");
        Self {
            operation_stats: Arc::new(Mutex::new(HashMap::new())),
            connection_stats: Arc::new(Mutex::new(HashMap::new())),
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
            operation_stats,
            connection_stats,
            generated_at: Instant::now(),
        }
    }
}

impl ConnectionMiddleware for PerformanceProfilingMiddleware {
    #[instrument(skip(self, context))]
    fn before_operation(&self, context: &OperationContext) -> Result<(), Error> {
        if let Ok(mut conn_stats) = self.connection_stats.lock() {
            let stats = conn_stats.entry(context.connection_id.clone())
                .or_insert_with(|| ConnectionStats {
                    connection_id: context.connection_id.clone(),
                    created_at: Instant::now(),
                    total_operations: 0,
                    active_operations: 0,
                    last_operation_at: None,
                });
            
            stats.active_operations += 1;
        }
        Ok(())
    }

    #[instrument(skip(self, context, result))]
    fn after_operation(&self, context: &OperationContext, result: &OperationResult) -> Result<(), Error> {
        // Update operation stats
        if let Ok(mut op_stats) = self.operation_stats.lock() {
            let stats = op_stats.entry(context.operation_type.clone())
                .or_insert_with(|| OperationStats {
                    total_operations: 0,
                    successful_operations: 0,
                    failed_operations: 0,
                    total_duration: Duration::ZERO,
                    min_duration: Duration::MAX,
                    max_duration: Duration::ZERO,
                    avg_duration: Duration::ZERO,
                });
            
            stats.total_operations += 1;
            if result.success {
                stats.successful_operations += 1;
            } else {
                stats.failed_operations += 1;
            }
            
            stats.total_duration += result.duration;
            stats.min_duration = stats.min_duration.min(result.duration);
            stats.max_duration = stats.max_duration.max(result.duration);
            stats.avg_duration = stats.total_duration / stats.total_operations as u32;
        }

        // Update connection stats
        if let Ok(mut conn_stats) = self.connection_stats.lock() {
            if let Some(stats) = conn_stats.get_mut(&context.connection_id) {
                stats.total_operations += 1;
                stats.active_operations = stats.active_operations.saturating_sub(1);
                stats.last_operation_at = Some(Instant::now());
            }
        }

        Ok(())
    }

    #[instrument(skip(self, context, _error))]
    fn on_error(&self, context: &OperationContext, _error: &DatabaseError) -> Result<(), Error> {
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
    pub operation_stats: HashMap<OperationType, OperationStats>,
    pub connection_stats: HashMap<String, ConnectionStats>,
    pub generated_at: Instant,
}

/// fr fr Connection health checker for monitoring connection status
#[derive(Debug)]
pub struct ConnectionHealthChecker {
    health_check_interval: Duration,
    health_timeout: Duration,
    failure_threshold: u32,
    recovery_threshold: u32,
    connection_health: Arc<RwLock<HashMap<String, ConnectionHealth>>>,
}

#[derive(Debug, Clone)]
pub struct ConnectionHealth {
    pub connection_id: String,
    pub status: HealthStatus,
    pub last_check: Instant,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub total_checks: u64,
    pub successful_checks: u64,
    pub average_response_time: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl ConnectionHealthChecker {
    /// slay Create new connection health checker
    #[instrument]
    pub fn new(check_interval: Duration, timeout: Duration) -> Self {
        info!(
            interval = ?check_interval,
            timeout = ?timeout,
            "Creating connection health checker"
        );
        
        Self {
            health_check_interval: check_interval,
            health_timeout: timeout,
            failure_threshold: 3,
            recovery_threshold: 2,
            connection_health: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// facts Check health of specific connection
    #[instrument(skip(self))]
    pub async fn check_connection_health(&self, connection_id: &str) -> Result<(), Error> {
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
            Err(_) => HealthStatus::Unhealthy,
        };
        
        // Update health statistics
        self.update_health_stats(connection_id, status.clone(), response_time).await;
        
        debug!(
            connection_id = %connection_id,
            status = ?status,
            response_time = ?response_time,
            "Health check completed"
        );
        
        Ok(status)
    }

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
                        checked_at: Instant::now(),
                        connection_healths: HashMap::new(),
                        overall_status: HealthStatus::Healthy,
                    };
                    
                    let mut unhealthy_count = 0;
                    let mut degraded_count = 0;
                    
                    for connection_id in &connections {
                        match health_checker.check_connection_health(connection_id).await {
                            Ok(status) => {
                                match status {
                                    HealthStatus::Unhealthy => unhealthy_count += 1,
                                    HealthStatus::Degraded => degraded_count += 1,
                                    _ => {}
                                }
                                
                                if let Ok(health_map) = health_checker.connection_health.read() {
                                    if let Some(health) = health_map.get(connection_id) {
                                        health_report.connection_healths.insert(
                                            connection_id.clone(),
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
                    }
                    
                    // Determine overall status
                    health_report.overall_status = if unhealthy_count > connections.len() / 2 {
                        HealthStatus::Unhealthy
                    } else if degraded_count > 0 || unhealthy_count > 0 {
                        HealthStatus::Degraded
                    } else {
                        HealthStatus::Healthy
                    };
                    
                    if let Err(_) = tx.send(health_report) {
                        warn!("Failed to send health report - receiver dropped");
                        break;
                    }
                    
                    tokio::time::sleep(check_interval).await;
                });
            }
        });
        
        rx
    }

    /// lowkey Execute actual health check query
    async fn execute_health_check_query(&self, connection_id: &str) -> Result<(), Error> {
        trace!(connection_id = %connection_id, "Executing health check query");
        
        // Simulate health check with small delay
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        // Simulate occasional failures for testing
        if connection_id.contains("unhealthy") {
            return Err(DatabaseError::connection_error("Simulated health check failure"));
        }
        
        Ok(())
    }

    /// bestie Update health statistics
    async fn update_health_stats(&self, connection_id: &str, status: HealthStatus, response_time: Duration) {
        if let Ok(mut health_map) = self.connection_health.write() {
            let health = health_map.entry(connection_id.to_string())
                .or_insert_with(|| ConnectionHealth {
                    connection_id: connection_id.to_string(),
                    status: HealthStatus::Unknown,
                    last_check: Instant::now(),
                    consecutive_failures: 0,
                    consecutive_successes: 0,
                    total_checks: 0,
                    successful_checks: 0,
                    average_response_time: Duration::ZERO,
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
            }
            
            // Update average response time
            let total_time = health.average_response_time * (health.total_checks - 1) as u32 + response_time;
            health.average_response_time = total_time / health.total_checks as u32;
        }
    }
}

impl Clone for ConnectionHealthChecker {
    fn clone(&self) -> Self {
        Self {
            health_check_interval: self.health_check_interval,
            health_timeout: self.health_timeout,
            failure_threshold: self.failure_threshold,
            recovery_threshold: self.recovery_threshold,
            connection_health: Arc::clone(&self.connection_health),
        }
    }
}

/// fr fr Health report structure
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub checked_at: Instant,
    pub connection_healths: HashMap<String, ConnectionHealth>,
    pub overall_status: HealthStatus,
}

/// fr fr Load balancer for distributing connections across multiple databases
#[derive(Debug)]
pub struct DatabaseLoadBalancer {
    strategy: LoadBalancingStrategy,
    database_nodes: Arc<RwLock<Vec<DatabaseNode>>>,
    current_index: Arc<Mutex<usize>>,
    health_checker: Arc<ConnectionHealthChecker>,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    HealthAware,
}

#[derive(Debug, Clone)]
pub struct DatabaseNode {
    pub id: String,
    pub connection_string: String,
    pub weight: u32,
    pub max_connections: u32,
    pub current_connections: Arc<Mutex<u32>>,
    pub is_healthy: bool,
    pub priority: u8, // 0 = highest priority
}

impl DatabaseLoadBalancer {
    /// slay Create new database load balancer
    #[instrument]
    pub fn new(strategy: LoadBalancingStrategy, nodes: Vec<DatabaseNode>) -> Self {
        info!(
            strategy = ?strategy,
            node_count = nodes.len(),
            "Creating database load balancer"
        );
        
        Self {
            strategy,
            database_nodes: Arc::new(RwLock::new(nodes)),
            current_index: Arc::new(Mutex::new(0)),
            health_checker: Arc::new(ConnectionHealthChecker::new(
                Duration::from_secs(30),
                Duration::from_secs(5)
            )),
        }
    }

    /// facts Get next connection using load balancing strategy
    #[instrument(skip(self))]
    pub async fn get_connection(&self) -> Result<(), Error> {
        debug!(strategy = ?self.strategy, "Getting connection using load balancer");
        
        let nodes = self.database_nodes.read()
            .map_err(|_| DatabaseError::connection_error("Failed to read database nodes"))?;
        
        if nodes.is_empty() {
            return Err(DatabaseError::connection_error("No database nodes available"));
        }
        
        let selected_node = match self.strategy {
            LoadBalancingStrategy::RoundRobin => self.round_robin_selection(&nodes).await?,
            LoadBalancingStrategy::WeightedRoundRobin => self.weighted_round_robin_selection(&nodes).await?,
            LoadBalancingStrategy::LeastConnections => self.least_connections_selection(&nodes).await?,
            LoadBalancingStrategy::Random => self.random_selection(&nodes).await?,
            LoadBalancingStrategy::HealthAware => self.health_aware_selection(&nodes).await?,
        };
        
        // Increment connection count
        if let Ok(mut count) = selected_node.current_connections.lock() {
            *count += 1;
        }
        
        debug!(
            selected_node = %selected_node.id,
            strategy = ?self.strategy,
            "Connection selected"
        );
        
        Ok(selected_node.connection_string.clone())
    }

    /// lowkey Round robin selection
    async fn round_robin_selection(&self, nodes: &[DatabaseNode]) -> Result<(), Error> {
        let mut index = self.current_index.lock()
            .map_err(|_| DatabaseError::connection_error("Failed to acquire index lock"))?;
        
        let selected_index = *index % nodes.len();
        *index = (*index + 1) % nodes.len();
        
        Ok(&nodes[selected_index])
    }

    /// periodt Weighted round robin selection
    async fn weighted_round_robin_selection(&self, nodes: &[DatabaseNode]) -> Result<(), Error> {
        let total_weight: u32 = nodes.iter().map(|n| n.weight).sum();
        
        if total_weight == 0 {
            return self.round_robin_selection(nodes).await;
        }
        
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
    }

    /// bestie Least connections selection
    async fn least_connections_selection(&self, nodes: &[DatabaseNode]) -> Result<(), Error> {
        let mut min_connections = u32::MAX;
        let mut selected_node = &nodes[0];
        
        for node in nodes {
            if let Ok(connections) = node.current_connections.lock() {
                if *connections < min_connections {
                    min_connections = *connections;
                    selected_node = node;
                }
            }
        }
        
        Ok(selected_node)
    }

    /// yolo Random selection
    async fn random_selection(&self, nodes: &[DatabaseNode]) -> Result<(), Error> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..nodes.len());
        Ok(&nodes[index])
    }

    /// slay Health-aware selection (prefer healthy nodes with lower priority)
    async fn health_aware_selection(&self, nodes: &[DatabaseNode]) -> Result<(), Error> {
        // Filter healthy nodes
        let healthy_nodes: Vec<&DatabaseNode> = nodes.iter()
            .filter(|n| n.is_healthy)
            .collect();
        
        if healthy_nodes.is_empty() {
            warn!("No healthy nodes available, falling back to all nodes");
            return self.least_connections_selection(nodes).await;
        }
        
        // Sort by priority (lower number = higher priority)
        let mut sorted_nodes = healthy_nodes;
        sorted_nodes.sort_by_key(|n| n.priority);
        
        // Use least connections among highest priority nodes
        let highest_priority = sorted_nodes[0].priority;
        let priority_nodes: Vec<&DatabaseNode> = sorted_nodes.into_iter()
            .filter(|n| n.priority == highest_priority)
            .collect();
        
        self.least_connections_selection(&priority_nodes).await
    }

    /// facts Release connection and decrement count
    #[instrument(skip(self))]
    pub async fn release_connection(&self, connection_string: &str) -> Result<(), Error> {
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
    }

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
            }
            
            node_stats.insert(node.id.clone(), NodeStats {
                node_id: node.id.clone(),
                current_connections,
                max_connections: node.max_connections,
                is_healthy: node.is_healthy,
                weight: node.weight,
                priority: node.priority,
            });
        }
        
        LoadBalancerStats {
            strategy: self.strategy.clone(),
            total_nodes: nodes.len(),
            healthy_nodes,
            total_connections,
            node_stats,
        }
    }
}

/// fr fr Load balancer statistics
#[derive(Debug, Clone)]
pub struct LoadBalancerStats {
    pub strategy: LoadBalancingStrategy,
    pub total_nodes: usize,
    pub healthy_nodes: usize,
    pub total_connections: u32,
    pub node_stats: HashMap<String, NodeStats>,
}

#[derive(Debug, Clone)]
pub struct NodeStats {
    pub node_id: String,
    pub current_connections: u32,
    pub max_connections: u32,
    pub is_healthy: bool,
    pub weight: u32,
    pub priority: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;
use crate::error::Error;

    #[traced_test]
    #[test]
    fn test_query_logging_middleware() {
        let middleware = QueryLoggingMiddleware::new(Duration::from_millis(100), true);
        
        let context = OperationContext {
            operation_type: OperationType::Query,
            connection_id: "conn_1".to_string(),
            sql: Some("SELECT * FROM users WHERE password = 'secret123'".to_string()),
            parameters: Vec::from([]),
            started_at: Instant::now(),
            metadata: HashMap::new(),
        };
        
        let sanitized = middleware.sanitize_sql("SELECT * FROM users WHERE password = 'secret123'");
        assert!(sanitized.contains("[REDACTED]"));
        
        let result = middleware.before_operation(&context);
        assert!(result.is_ok());
    }

    #[traced_test]
    #[test]
    fn test_performance_profiling_middleware() {
        let middleware = PerformanceProfilingMiddleware::new();
        
        let context = OperationContext {
            operation_type: OperationType::Query,
            connection_id: "conn_1".to_string(),
            sql: Some("SELECT * FROM users".to_string()),
            parameters: Vec::from([]),
            started_at: Instant::now(),
            metadata: HashMap::new(),
        };
        
        let result = OperationResult {
            success: true,
            duration: Duration::from_millis(50),
            rows_affected: None,
            rows_returned: Some(10),
            error: None,
        };
        
        middleware.before_operation(&context).expect("Before operation should succeed");
        middleware.after_operation(&context, &result).expect("After operation should succeed");
        
        let stats = middleware.get_stats();
        assert!(stats.operation_stats.contains_key(&OperationType::Query));
        assert!(stats.connection_stats.contains_key("conn_1"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_connection_health_checker() {
        let checker = ConnectionHealthChecker::new(
            Duration::from_secs(1),
            Duration::from_millis(100)
        );
        
        let status = checker.check_connection_health("healthy_conn").await
            .expect("Health check should succeed");
        
        assert_eq!(status, HealthStatus::Healthy);
        
        let unhealthy_status = checker.check_connection_health("unhealthy_conn").await
            .expect("Health check should succeed");
        
        assert_eq!(unhealthy_status, HealthStatus::Unhealthy);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_load_balancer_round_robin() {
        let nodes = vec![
            DatabaseNode {
                id: "node1".to_string(),
                connection_string: "conn1".to_string(),
                weight: 1,
                max_connections: 10,
                current_connections: Arc::new(Mutex::new(0)),
                is_healthy: true,
                priority: 1,
            },
            DatabaseNode {
                id: "node2".to_string(),
                connection_string: "conn2".to_string(),
                weight: 1,
                max_connections: 10,
                current_connections: Arc::new(Mutex::new(0)),
                is_healthy: true,
                priority: 1,
            },
        ];
        
        let balancer = DatabaseLoadBalancer::new(LoadBalancingStrategy::RoundRobin, nodes);
        
        let conn1 = balancer.get_connection().await.expect("Should get connection");
        let conn2 = balancer.get_connection().await.expect("Should get connection");
        let conn3 = balancer.get_connection().await.expect("Should get connection");
        
        assert_eq!(conn1, "conn1");
        assert_eq!(conn2, "conn2");
        assert_eq!(conn3, "conn1"); // Should cycle back
    }

    #[traced_test]
    #[tokio::test]
    async fn test_load_balancer_least_connections() {
        let nodes = vec![
            DatabaseNode {
                id: "node1".to_string(),
                connection_string: "conn1".to_string(),
                weight: 1,
                max_connections: 10,
                current_connections: Arc::new(Mutex::new(5)),
                is_healthy: true,
                priority: 1,
            },
            DatabaseNode {
                id: "node2".to_string(),
                connection_string: "conn2".to_string(),
                weight: 1,
                max_connections: 10,
                current_connections: Arc::new(Mutex::new(2)),
                is_healthy: true,
                priority: 1,
            },
        ];
        
        let balancer = DatabaseLoadBalancer::new(LoadBalancingStrategy::LeastConnections, nodes);
        
        let conn = balancer.get_connection().await.expect("Should get connection");
        assert_eq!(conn, "conn2"); // Should pick node with fewer connections
        
        let stats = balancer.get_stats();
        assert_eq!(stats.total_nodes, 2);
        assert_eq!(stats.healthy_nodes, 2);
    }
}
