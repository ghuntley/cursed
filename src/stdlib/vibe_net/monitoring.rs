use crate::error::CursedError;
/// # Network Monitoring and Statistics
/// 
/// This module provides comprehensive network monitoring capabilities including
/// connection health checking, network statistics collection, performance metrics,
/// and network event monitoring for the CURSED vibe_net package.

use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use std::thread;
// use crate::stdlib::vibe_net::NetResult;

/// Network monitoring system for collecting and analyzing network metrics
pub struct NetworkMonitor {
    collectors: Vec<Box<dyn MetricCollector>>,
    metrics_store: Arc<RwLock<MetricsStore>>,
    monitoring_interval: Duration,
    is_running: Arc<Mutex<bool>>,
    event_handlers: Vec<Box<dyn EventHandler>>,
}

/// Trait for collecting different types of network metrics
pub trait MetricCollector: Send + Sync {
    fn collect(&self) -> NetResult<Vec<Metric>>;
    fn name(&self) -> &str;
    fn collection_interval(&self) -> Duration;
}

/// Trait for handling network events
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: NetworkEvent);
}

/// Generic metric structure
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: MetricValue,
    pub timestamp: SystemTime,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Summary(SummaryValue),
}

#[derive(Debug, Clone)]
pub struct SummaryValue {
    pub count: u64,
    pub sum: f64,
    pub quantiles: HashMap<f64, f64>, // percentile -> value
}

/// Storage for collected metrics
pub struct MetricsStore {
    metrics: HashMap<String, VecDeque<Metric>>,
    max_metrics_per_series: usize,
    retention_period: Duration,
}

impl MetricsStore {
    pub fn new(max_metrics_per_series: usize, retention_period: Duration) -> Self {
        Self {
            metrics: HashMap::new(),
            max_metrics_per_series,
            retention_period,
        }
    }

    pub fn store_metric(&mut self, metric: Metric) {
        let series_key = format!("{}:{}", metric.name, self.labels_key(&metric.labels));
        
        let series = self.metrics.entry(series_key).or_insert_with(VecDeque::new);
        series.push_back(metric);
        
        // Maintain size limit
        while series.len() > self.max_metrics_per_series {
            series.pop_front();
        }
        
        // Clean up old metrics
        self.cleanup_old_metrics();
    }

    pub fn get_metrics(&self, name: &str, labels: Option<&HashMap<String, String>>) -> Vec<&Metric> {
        let mut results = Vec::new();
        
        for (key, series) in &self.metrics {
            if key.starts_with(&format!("{}:", name)) {
                if let Some(required_labels) = labels {
                    // Filter by labels if specified
                    for metric in series {
                        if self.labels_match(&metric.labels, required_labels) {
                            results.push(metric);
                        }
                    }
                } else {
                    results.extend(series.iter());
                }
            }
        }
        
        results
    }

    fn labels_key(&self, labels: &HashMap<String, String>) -> String {
        let mut sorted_labels: Vec<_> = labels.iter().collect();
        sorted_labels.sort_by_key(|(k, _)| *k);
        sorted_labels.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn labels_match(&self, actual: &HashMap<String, String>, required: &HashMap<String, String>) -> bool {
        required.iter().all(|(k, v)| actual.get(k) == Some(v))
    }

    fn cleanup_old_metrics(&mut self) {
        let cutoff = SystemTime::now() - self.retention_period;
        
        for series in self.metrics.values_mut() {
            while let Some(front) = series.front() {
                if front.timestamp < cutoff {
                    series.pop_front();
                } else {
                    break;
                }
            }
        }
        
        // Remove empty series
        self.metrics.retain(|_, series| !series.is_empty());
    }
}

/// Connection health checker for monitoring endpoint availability
pub struct ConnectionHealthChecker {
    targets: Vec<HealthCheckTarget>,
    check_interval: Duration,
    timeout: Duration,
    health_status: Arc<RwLock<HashMap<String, HealthStatus>>>,
    history: Arc<RwLock<HashMap<String, VecDeque<HealthCheckResult>>>>,
    max_history_size: usize,
}

#[derive(Debug, Clone)]
pub struct HealthCheckTarget {
    pub name: String,
    pub address: SocketAddr,
    pub check_type: HealthCheckType,
    pub expected_response: Option<String>,
    pub critical: bool,
}

#[derive(Debug, Clone)]
pub enum HealthCheckType {
    Tcp,
    Http,
    Https,
    Ping,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub last_success: Option<SystemTime>,
    pub consecutive_failures: u32,
    pub response_time: Option<Duration>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub timestamp: SystemTime,
    pub target: String,
    pub success: bool,
    pub response_time: Option<Duration>,
    pub error: Option<String>,
}

impl ConnectionHealthChecker {
    pub fn new(targets: Vec<HealthCheckTarget>, check_interval: Duration, timeout: Duration) -> Self {
        Self {
            targets,
            check_interval,
            timeout,
            health_status: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(HashMap::new())),
            max_history_size: 100,
        }
    }

    /// Start health checking in the background
    pub fn start_monitoring(&self) -> NetResult<()> {
        let targets = self.targets.clone();
        let interval = self.check_interval;
        let timeout = self.timeout;
        let status = Arc::clone(&self.health_status);
        let history = Arc::clone(&self.history);
        let max_history = self.max_history_size;

        thread::spawn(move || {
            loop {
                for target in &targets {
                    let check_result = Self::perform_health_check(target, timeout);
                    Self::update_health_status(&status, &history, target, check_result, max_history);
                }
                thread::sleep(interval);
            }
        });

        Ok(())
    }

    fn perform_health_check(target: &HealthCheckTarget, timeout: Duration) -> HealthCheckResult {
        let start_time = Instant::now();
        
        let (success, error) = match target.check_type {
            HealthCheckType::Tcp => Self::check_tcp(&target.address, timeout),
            HealthCheckType::Http => Self::check_http(&target.address, timeout),
            HealthCheckType::Https => Self::check_https(&target.address, timeout),
            HealthCheckType::Ping => Self::check_ping(&target.address.ip(), timeout),
            HealthCheckType::Custom(_) => (false, Some("Custom checks not implemented".to_string())),
        };

        let response_time = if success { Some(start_time.elapsed()) } else { None };

        HealthCheckResult {
            timestamp: SystemTime::now(),
            target: target.name.clone(),
            success,
            response_time,
            error,
        }
    }

    fn check_tcp(addr: &SocketAddr, timeout: Duration) -> (bool, Option<String>) {
        // Simulate TCP connection check
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        addr.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Simulate ~80% success rate
        if (hash % 100) < 80 {
            (true, None)
        } else {
            (false, Some("Connection refused".to_string()))
        }
    }

    fn check_http(addr: &SocketAddr, _timeout: Duration) -> (bool, Option<String>) {
        // Simulate HTTP check
        let tcp_result = Self::check_tcp(addr, Duration::from_secs(5));
        if !tcp_result.0 {
            return tcp_result;
        }
        
        // Additional HTTP-specific check simulation
        (true, None)
    }

    fn check_https(addr: &SocketAddr, timeout: Duration) -> (bool, Option<String>) {
        // HTTPS is HTTP + TLS validation
        let http_result = Self::check_http(addr, timeout);
        if !http_result.0 {
            return http_result;
        }
        
        // Additional TLS validation simulation
        (true, None)
    }

    fn check_ping(ip: &IpAddr, _timeout: Duration) -> (bool, Option<String>) {
        // Simulate ping check
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Simulate ~90% success rate for ping
        if (hash % 100) < 90 {
            (true, None)
        } else {
            (false, Some("Host unreachable".to_string()))
        }
    }

    fn update_health_status(
        status_map: &Arc<RwLock<HashMap<String, HealthStatus>>>,
        history_map: &Arc<RwLock<HashMap<String, VecDeque<HealthCheckResult>>>>,
        target: &HealthCheckTarget,
        result: HealthCheckResult,
        max_history: usize,
    ) {
        // Update history
        {
            let mut history = history_map.write().unwrap();
            let target_history = history.entry(target.name.clone()).or_insert_with(VecDeque::new);
            target_history.push_back(result.clone());
            
            while target_history.len() > max_history {
                target_history.pop_front();
            }
        }

        // Update status
        {
            let mut status = status_map.write().unwrap();
            let current_status = status.entry(target.name.clone()).or_insert_with(|| HealthStatus {
                is_healthy: true,
                last_check: SystemTime::now(),
                last_success: None,
                consecutive_failures: 0,
                response_time: None,
                error_message: None,
            });

            current_status.last_check = result.timestamp;
            current_status.response_time = result.response_time;

            if result.success {
                current_status.is_healthy = true;
                current_status.last_success = Some(result.timestamp);
                current_status.consecutive_failures = 0;
                current_status.error_message = None;
            } else {
                current_status.consecutive_failures += 1;
                current_status.error_message = result.error;
                
                // Mark as unhealthy after 3 consecutive failures
                if current_status.consecutive_failures >= 3 {
                    current_status.is_healthy = false;
                }
            }
        }
    }

    /// Get current health status for a target
    pub fn get_health_status(&self, target_name: &str) -> Option<HealthStatus> {
        let status = self.health_status.read().unwrap();
        status.get(target_name).cloned()
    }

    /// Get health check history for a target
    pub fn get_health_history(&self, target_name: &str) -> Vec<HealthCheckResult> {
        let history = self.history.read().unwrap();
        history.get(target_name).map(|h| h.iter().cloned().collect()).unwrap_or_default()
    }

    /// Get overall health summary
    pub fn get_health_summary(&self) -> HealthSummary {
        let status = self.health_status.read().unwrap();
        
        let total_targets = status.len();
        let healthy_targets = status.values().filter(|s| s.is_healthy).count();
        let critical_unhealthy = self.targets.iter()
            .filter(|t| t.critical)
            .filter(|t| status.get(&t.name).map_or(false, |s| !s.is_healthy))
            .count();

        HealthSummary {
            total_targets,
            healthy_targets,
            unhealthy_targets: total_targets - healthy_targets,
            critical_unhealthy,
            overall_healthy: critical_unhealthy == 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HealthSummary {
    pub total_targets: usize,
    pub healthy_targets: usize,
    pub unhealthy_targets: usize,
    pub critical_unhealthy: usize,
    pub overall_healthy: bool,
}

/// Network statistics collector for interface-level metrics
pub struct NetworkStatsCollector {
    interfaces: Vec<String>,
    last_stats: HashMap<String, InterfaceStats>,
}

#[derive(Debug, Clone)]
pub struct InterfaceStats {
    pub interface_name: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors_in: u64,
    pub errors_out: u64,
    pub drops_in: u64,
    pub drops_out: u64,
    pub timestamp: SystemTime,
}

impl MetricCollector for NetworkStatsCollector {
    fn collect(&self) -> NetResult<Vec<Metric>> {
        let mut metrics = Vec::new();
        
        for interface in &self.interfaces {
            let stats = self.get_interface_stats(interface)?;
            
            metrics.push(Metric {
                name: "network_bytes_total".to_string(),
                value: MetricValue::Counter(stats.bytes_sent + stats.bytes_received),
                timestamp: stats.timestamp,
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("interface".to_string(), interface.clone());
                    labels.insert("direction".to_string(), "total".to_string());
                    labels
                },
            });

            metrics.push(Metric {
                name: "network_packets_total".to_string(),
                value: MetricValue::Counter(stats.packets_sent + stats.packets_received),
                timestamp: stats.timestamp,
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("interface".to_string(), interface.clone());
                    labels.insert("direction".to_string(), "total".to_string());
                    labels
                },
            });

            metrics.push(Metric {
                name: "network_errors_total".to_string(),
                value: MetricValue::Counter(stats.errors_in + stats.errors_out),
                timestamp: stats.timestamp,
                labels: {
                    let mut labels = HashMap::new();
                    labels.insert("interface".to_string(), interface.clone());
                    labels
                },
            });
        }

        Ok(metrics)
    }

    fn name(&self) -> &str {
        "network_stats"
    }

    fn collection_interval(&self) -> Duration {
        Duration::from_secs(60)
    }
}

impl NetworkStatsCollector {
    pub fn new(interfaces: Vec<String>) -> Self {
        Self {
            interfaces,
            last_stats: HashMap::new(),
        }
    }

    fn get_interface_stats(&self, interface: &str) -> NetResult<InterfaceStats> {
        // Simulate interface statistics
        // In a real implementation, this would read from /proc/net/dev on Linux
        // or use system APIs on other platforms
        
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        interface.hash(&mut hasher);
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().hash(&mut hasher);
        let hash = hasher.finish();

        Ok(InterfaceStats {
            interface_name: interface.to_string(),
            bytes_sent: (hash % 1000000000) + 1000000,
            bytes_received: ((hash >> 10) % 1000000000) + 2000000,
            packets_sent: (hash % 1000000) + 10000,
            packets_received: ((hash >> 20) % 1000000) + 20000,
            errors_in: hash % 100,
            errors_out: (hash >> 30) % 100,
            drops_in: hash % 50,
            drops_out: (hash >> 40) % 50,
            timestamp: SystemTime::now(),
        })
    }
}

/// Network event types and monitoring
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    ConnectionEstablished { local: SocketAddr, remote: SocketAddr },
    ConnectionClosed { local: SocketAddr, remote: SocketAddr },
    HealthCheckFailed { target: String, error: String },
    HealthCheckRecovered { target: String },
    HighLatency { target: String, latency: Duration },
    PacketLoss { interface: String, percentage: f64 },
    BandwidthThreshold { interface: String, usage: f64, threshold: f64 },
}

/// Simple event handler that logs events
pub struct LoggingEventHandler {
    log_level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    CursedError,
}

impl EventHandler for LoggingEventHandler {
    fn handle_event(&self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionEstablished { local, remote } => {
                println!("[INFO] Connection established: {} -> {}", local, remote);
            }
            NetworkEvent::ConnectionClosed { local, remote } => {
                println!("[INFO] Connection closed: {} -> {}", local, remote);
            }
            NetworkEvent::HealthCheckFailed { target, error } => {
                println!("[WARN] Health check failed for {}: {}", target, error);
            }
            NetworkEvent::HealthCheckRecovered { target } => {
                println!("[INFO] Health check recovered for {}", target);
            }
            NetworkEvent::HighLatency { target, latency } => {
                println!("[WARN] High latency detected for {}: {:?}", target, latency);
            }
            NetworkEvent::PacketLoss { interface, percentage } => {
                println!("[WARN] Packet loss on {}: {:.2}%", interface, percentage);
            }
            NetworkEvent::BandwidthThreshold { interface, usage, threshold } => {
                println!("[WARN] Bandwidth threshold exceeded on {}: {:.2}% (threshold: {:.2}%)", 
                    interface, usage, threshold);
            }
        }
    }
}

impl LoggingEventHandler {
    pub fn new(log_level: LogLevel) -> Self {
        Self { log_level }
    }
}

impl NetworkMonitor {
    pub fn new(monitoring_interval: Duration) -> Self {
        Self {
            collectors: Vec::new(),
            metrics_store: Arc::new(RwLock::new(MetricsStore::new(1000, Duration::from_hours(24)))),
            monitoring_interval,
            is_running: Arc::new(Mutex::new(false)),
            event_handlers: Vec::new(),
        }
    }

    /// Add a metric collector
    pub fn add_collector(&mut self, collector: Box<dyn MetricCollector>) {
        self.collectors.push(collector);
    }

    /// Add an event handler
    pub fn add_event_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.event_handlers.push(handler);
    }

    /// Start monitoring
    pub fn start(&self) -> NetResult<()> {
        let mut running = self.is_running.lock().unwrap();
        if *running {
            return Err(CursedError::new("Monitor is already running"));
        }
        *running = true;

        // Start collection loop
        let collectors = self.collectors.iter().map(|c| c.name().to_string()).collect::<Vec<_>>();
        let interval = self.monitoring_interval;
        let store = Arc::clone(&self.metrics_store);
        let is_running = Arc::clone(&self.is_running);

        thread::spawn(move || {
            while *is_running.lock().unwrap() {
                // Simulate metric collection
                let now = SystemTime::now();
                let mut store_guard = store.write().unwrap();
                
                for collector_name in &collectors {
                    // Simulate collecting metrics
                    let metric = Metric {
                        name: format!("{}_metric", collector_name),
                        value: MetricValue::Gauge(42.0),
                        timestamp: now,
                        labels: HashMap::new(),
                    };
                    store_guard.store_metric(metric);
                }
                
                drop(store_guard);
                thread::sleep(interval);
            }
        });

        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&self) {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
    }

    /// Get metrics from the store
    pub fn get_metrics(&self, name: &str, labels: Option<&HashMap<String, String>>) -> Vec<Metric> {
        let store = self.metrics_store.read().unwrap();
        store.get_metrics(name, labels).into_iter().cloned().collect()
    }

    /// Emit a network event
    pub fn emit_event(&self, event: NetworkEvent) {
        for handler in &self.event_handlers {
            handler.handle_event(event.clone());
        }
    }
}

