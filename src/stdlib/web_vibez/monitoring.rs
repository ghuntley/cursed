use crate::error::CursedError;
/// Performance metrics collection and monitoring utilities
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Metrics collector for tracking performance and usage
pub struct MetricsCollector {
    request_metrics: HashMap<String, RequestMetrics>,
    global_metrics: GlobalMetrics,
    start_time: SystemTime,
}

#[derive(Debug, Clone)]
pub struct RequestMetrics {
    pub count: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub status_codes: HashMap<u16, u64>,
    pub response_sizes: Vec<usize>,
    pub last_request: SystemTime,
}

#[derive(Debug, Clone)]
pub struct GlobalMetrics {
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub concurrent_connections: u64,
    pub peak_concurrent_connections: u64,
    pub uptime: Duration,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            request_metrics: HashMap::new(),
            global_metrics: GlobalMetrics {
                total_requests: 0,
                total_errors: 0,
                total_bytes_sent: 0,
                total_bytes_received: 0,
                concurrent_connections: 0,
                peak_concurrent_connections: 0,
                uptime: Duration::new(0, 0),
            },
            start_time: SystemTime::now(),
        }
    }

    /// Record a request
    pub fn record_request(
        &mut self,
        path: &str,
        method: &str,
        status: u16,
        duration: Duration,
        response_size: usize,
    ) {
        let key = format!("{} {}", method, path);
        
        // Update global metrics
        self.global_metrics.total_requests += 1;
        self.global_metrics.total_bytes_sent += response_size as u64;
        
        if status >= 400 {
            self.global_metrics.total_errors += 1;
        }

        // Update route-specific metrics
        let metrics = self.request_metrics.entry(key).or_insert_with(|| RequestMetrics {
            count: 0,
            total_duration: Duration::new(0, 0),
            min_duration: duration,
            max_duration: duration,
            status_codes: HashMap::new(),
            response_sizes: Vec::new(),
            last_request: SystemTime::now(),
        });

        metrics.count += 1;
        metrics.total_duration += duration;
        metrics.min_duration = metrics.min_duration.min(duration);
        metrics.max_duration = metrics.max_duration.max(duration);
        metrics.last_request = SystemTime::now();

        // Track status codes
        *metrics.status_codes.entry(status).or_insert(0) += 1;

        // Track response sizes (keep last 100)
        metrics.response_sizes.push(response_size);
        if metrics.response_sizes.len() > 100 {
            metrics.response_sizes.remove(0);
        }
    }

    /// Record bytes received
    pub fn record_bytes_received(&mut self, bytes: u64) {
        self.global_metrics.total_bytes_received += bytes;
    }

    /// Update concurrent connections
    pub fn update_concurrent_connections(&mut self, count: u64) {
        self.global_metrics.concurrent_connections = count;
        if count > self.global_metrics.peak_concurrent_connections {
            self.global_metrics.peak_concurrent_connections = count;
        }
    }

    /// Get metrics for specific route
    pub fn get_route_metrics(&self, path: &str, method: &str) -> Option<&RequestMetrics> {
        let key = format!("{} {}", method, path);
        self.request_metrics.get(&key)
    }

    /// Get global metrics
    pub fn get_global_metrics(&mut self) -> GlobalMetrics {
        // Update uptime
        self.global_metrics.uptime = SystemTime::now()
            .duration_since(self.start_time)
            .unwrap_or_default();
        
        self.global_metrics.clone()
    }

    /// Get all route metrics
    pub fn get_all_route_metrics(&self) -> &HashMap<String, RequestMetrics> {
        &self.request_metrics
    }

    /// Get top slowest routes
    pub fn get_slowest_routes(&self, limit: usize) -> Vec<(String, Duration)> {
        let mut routes: Vec<_> = self.request_metrics
            .iter()
            .map(|(route, metrics)| {
                let avg_duration = if metrics.count > 0 {
                    metrics.total_duration / metrics.count as u32
                } else {
                    Duration::new(0, 0)
                };
                (route.clone(), avg_duration)
            })
            .collect();

        routes.sort_by(|a, b| b.1.cmp(&a.1));
        routes.into_iter().take(limit).collect()
    }

    /// Get routes with most errors
    pub fn get_error_prone_routes(&self, limit: usize) -> Vec<(String, u64)> {
        let mut routes: Vec<_> = self.request_metrics
            .iter()
            .map(|(route, metrics)| {
                let error_count = metrics.status_codes
                    .iter()
                    .filter(|(status, _)| **status >= 400)
                    .map(|(_, count)| *count)
                    .sum();
                (route.clone(), error_count)
            })
            .filter(|(_, errors)| *errors > 0)
            .collect();

        routes.sort_by(|a, b| b.1.cmp(&a.1));
        routes.into_iter().take(limit).collect()
    }

    /// Export metrics in Prometheus format
    pub fn export_prometheus(&mut self) -> String {
        let mut output = String::new();
        let global = self.get_global_metrics();

        // Global metrics
        output.push_str(&format!("# HELP cursed_http_requests_total Total number of HTTP requests\n"));
        output.push_str(&format!("# TYPE cursed_http_requests_total counter\n"));
        output.push_str(&format!("cursed_http_requests_total {}\n", global.total_requests));

        output.push_str(&format!("# HELP cursed_http_errors_total Total number of HTTP errors\n"));
        output.push_str(&format!("# TYPE cursed_http_errors_total counter\n"));
        output.push_str(&format!("cursed_http_errors_total {}\n", global.total_errors));

        output.push_str(&format!("# HELP cursed_http_bytes_sent_total Total bytes sent\n"));
        output.push_str(&format!("# TYPE cursed_http_bytes_sent_total counter\n"));
        output.push_str(&format!("cursed_http_bytes_sent_total {}\n", global.total_bytes_sent));

        output.push_str(&format!("# HELP cursed_http_concurrent_connections Current concurrent connections\n"));
        output.push_str(&format!("# TYPE cursed_http_concurrent_connections gauge\n"));
        output.push_str(&format!("cursed_http_concurrent_connections {}\n", global.concurrent_connections));

        // Route-specific metrics
        output.push_str(&format!("# HELP cursed_http_request_duration_seconds Request duration in seconds\n"));
        output.push_str(&format!("# TYPE cursed_http_request_duration_seconds histogram\n"));

        for (route, metrics) in &self.request_metrics {
            let parts: Vec<&str> = route.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let method = parts[0];
                let path = parts[1];
                let avg_duration = if metrics.count > 0 {
                    metrics.total_duration.as_secs_f64() / metrics.count as f64
                } else {
                    0.0
                };

                output.push_str(&format!(
                    "cursed_http_request_duration_seconds{{method=\"{}\",path=\"{}\"}} {}\n",
                    method, path, avg_duration
                ));
            }
        }

        output
    }

    /// Export metrics as JSON
    pub fn export_json(&mut self) -> String {
        let global = self.get_global_metrics();
        
        let mut json = String::new();
        json.push_str("{\n");
        
        // Global metrics
        json.push_str("  \"global\": {\n");
        json.push_str(&format!("    \"total_requests\": {},\n", global.total_requests));
        json.push_str(&format!("    \"total_errors\": {},\n", global.total_errors));
        json.push_str(&format!("    \"total_bytes_sent\": {},\n", global.total_bytes_sent));
        json.push_str(&format!("    \"total_bytes_received\": {},\n", global.total_bytes_received));
        json.push_str(&format!("    \"concurrent_connections\": {},\n", global.concurrent_connections));
        json.push_str(&format!("    \"peak_concurrent_connections\": {},\n", global.peak_concurrent_connections));
        json.push_str(&format!("    \"uptime_seconds\": {}\n", global.uptime.as_secs()));
        json.push_str("  },\n");
        
        // Route metrics
        json.push_str("  \"routes\": {\n");
        let mut first = true;
        for (route, metrics) in &self.request_metrics {
            if !first {
                json.push_str(",\n");
            }
            first = false;
            
            let avg_duration = if metrics.count > 0 {
                metrics.total_duration.as_secs_f64() / metrics.count as f64
            } else {
                0.0
            };
            
            json.push_str(&format!("    \"{}\": {{\n", route));
            json.push_str(&format!("      \"count\": {},\n", metrics.count));
            json.push_str(&format!("      \"avg_duration_seconds\": {},\n", avg_duration));
            json.push_str(&format!("      \"min_duration_seconds\": {},\n", metrics.min_duration.as_secs_f64()));
            json.push_str(&format!("      \"max_duration_seconds\": {},\n", metrics.max_duration.as_secs_f64()));
            json.push_str("      \"status_codes\": {");
            
            let mut status_first = true;
            for (status, count) in &metrics.status_codes {
                if !status_first {
                    json.push_str(", ");
                }
                status_first = false;
                json.push_str(&format!("\"{}\": {}", status, count));
            }
            json.push_str("}\n");
            json.push_str("    }");
        }
        json.push_str("\n  }\n");
        json.push_str("}\n");
        
        json
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        self.request_metrics.clear();
        self.global_metrics = GlobalMetrics {
            total_requests: 0,
            total_errors: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            concurrent_connections: 0,
            peak_concurrent_connections: 0,
            uptime: Duration::new(0, 0),
        };
        self.start_time = SystemTime::now();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestMetrics {
    /// Get average response time
    pub fn average_duration(&self) -> Duration {
        if self.count > 0 {
            self.total_duration / self.count as u32
        } else {
            Duration::new(0, 0)
        }
    }

    /// Get error rate as percentage
    pub fn error_rate(&self) -> f64 {
        if self.count == 0 {
            return 0.0;
        }

        let error_count: u64 = self.status_codes
            .iter()
            .filter(|(status, _)| **status >= 400)
            .map(|(_, count)| *count)
            .sum();

        (error_count as f64 / self.count as f64) * 100.0
    }

    /// Get average response size
    pub fn average_response_size(&self) -> f64 {
        if self.response_sizes.is_empty() {
            return 0.0;
        }

        let total: usize = self.response_sizes.iter().sum();
        total as f64 / self.response_sizes.len() as f64
    }

    /// Get requests per second (based on last minute)
    pub fn requests_per_second(&self) -> f64 {
        let now = SystemTime::now();
        let time_diff = now.duration_since(self.last_request).unwrap_or_default();
        
        if time_diff.as_secs() == 0 {
            return self.count as f64;
        }

        self.count as f64 / time_diff.as_secs() as f64
    }
}

/// Performance monitor for tracking system resources
pub struct PerformanceMonitor {
    cpu_usage_history: Vec<f64>,
    memory_usage_history: Vec<u64>,
    max_history_size: usize,
    last_measurement: SystemTime,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new() -> Self {
        Self {
            cpu_usage_history: Vec::new(),
            memory_usage_history: Vec::new(),
            max_history_size: 100,
            last_measurement: SystemTime::now(),
        }
    }

    /// Set maximum history size
    pub fn with_max_history(mut self, size: usize) -> Self {
        self.max_history_size = size;
        self
    }

    /// Record CPU usage
    pub fn record_cpu_usage(&mut self, usage_percent: f64) {
        self.cpu_usage_history.push(usage_percent);
        if self.cpu_usage_history.len() > self.max_history_size {
            self.cpu_usage_history.remove(0);
        }
        self.last_measurement = SystemTime::now();
    }

    /// Record memory usage
    pub fn record_memory_usage(&mut self, usage_bytes: u64) {
        self.memory_usage_history.push(usage_bytes);
        if self.memory_usage_history.len() > self.max_history_size {
            self.memory_usage_history.remove(0);
        }
    }

    /// Get current CPU usage (average of recent measurements)
    pub fn current_cpu_usage(&self) -> f64 {
        if self.cpu_usage_history.is_empty() {
            return 0.0;
        }

        let recent_count = (self.cpu_usage_history.len().min(10)).max(1);
        let recent_sum: f64 = self.cpu_usage_history
            .iter()
            .rev()
            .take(recent_count)
            .sum();

        recent_sum / recent_count as f64
    }

    /// Get current memory usage
    pub fn current_memory_usage(&self) -> u64 {
        self.memory_usage_history.last().copied().unwrap_or(0)
    }

    /// Get peak CPU usage
    pub fn peak_cpu_usage(&self) -> f64 {
        self.cpu_usage_history.iter().copied().fold(0.0, f64::max)
    }

    /// Get peak memory usage
    pub fn peak_memory_usage(&self) -> u64 {
        self.memory_usage_history.iter().copied().max().unwrap_or(0)
    }

    /// Get average CPU usage
    pub fn average_cpu_usage(&self) -> f64 {
        if self.cpu_usage_history.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.cpu_usage_history.iter().sum();
        sum / self.cpu_usage_history.len() as f64
    }

    /// Get average memory usage
    pub fn average_memory_usage(&self) -> u64 {
        if self.memory_usage_history.is_empty() {
            return 0;
        }

        let sum: u64 = self.memory_usage_history.iter().sum();
        sum / self.memory_usage_history.len() as u64
    }

    /// Check if system is under high load
    pub fn is_high_load(&self) -> bool {
        self.current_cpu_usage() > 80.0
    }

    /// Get performance summary
    pub fn get_summary(&self) -> PerformanceSummary {
        PerformanceSummary {
            current_cpu: self.current_cpu_usage(),
            peak_cpu: self.peak_cpu_usage(),
            average_cpu: self.average_cpu_usage(),
            current_memory: self.current_memory_usage(),
            peak_memory: self.peak_memory_usage(),
            average_memory: self.average_memory_usage(),
            measurements_count: self.cpu_usage_history.len(),
            last_measurement: self.last_measurement,
        }
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.cpu_usage_history.clear();
        self.memory_usage_history.clear();
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance summary
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub current_cpu: f64,
    pub peak_cpu: f64,
    pub average_cpu: f64,
    pub current_memory: u64,
    pub peak_memory: u64,
    pub average_memory: u64,
    pub measurements_count: usize,
    pub last_measurement: SystemTime,
}

/// Alert system for monitoring thresholds
pub struct AlertSystem {
    thresholds: AlertThresholds,
    alerts: Vec<Alert>,
    max_alerts: usize,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub cpu_threshold: f64,
    pub memory_threshold: u64,
    pub error_rate_threshold: f64,
    pub response_time_threshold: Duration,
    pub concurrent_connections_threshold: u64,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub alert_type: AlertType,
    pub message: String,
    pub timestamp: SystemTime,
    pub severity: AlertSeverity,
}

#[derive(Debug, Clone)]
pub enum AlertType {
    HighCpu,
    HighMemory,
    HighErrorRate,
    SlowResponse,
    TooManyConnections,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Info,
    Warning,
    CursedError,
    Critical,
}

impl AlertSystem {
    /// Create new alert system
    pub fn new() -> Self {
        Self {
            thresholds: AlertThresholds::default(),
            alerts: Vec::new(),
            max_alerts: 1000,
        }
    }

    /// Set alert thresholds
    pub fn with_thresholds(mut self, thresholds: AlertThresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// Check metrics and generate alerts
    pub fn check_metrics(
        &mut self,
        performance: &PerformanceSummary,
        global_metrics: &GlobalMetrics,
        route_metrics: &HashMap<String, RequestMetrics>,
    ) {
        // Check CPU usage
        if performance.current_cpu > self.thresholds.cpu_threshold {
            self.add_alert(Alert {
                alert_type: AlertType::HighCpu,
                message: format!("CPU usage is {:.1}% (threshold: {:.1}%)", 
                               performance.current_cpu, self.thresholds.cpu_threshold),
                timestamp: SystemTime::now(),
                severity: if performance.current_cpu > 90.0 {
                    AlertSeverity::Critical
                } else {
                    AlertSeverity::Warning
                },
            });
        }

        // Check memory usage
        if performance.current_memory > self.thresholds.memory_threshold {
            self.add_alert(Alert {
                alert_type: AlertType::HighMemory,
                message: format!("Memory usage is {} bytes (threshold: {} bytes)", 
                               performance.current_memory, self.thresholds.memory_threshold),
                timestamp: SystemTime::now(),
                severity: AlertSeverity::Warning,
            });
        }

        // Check concurrent connections
        if global_metrics.concurrent_connections > self.thresholds.concurrent_connections_threshold {
            self.add_alert(Alert {
                alert_type: AlertType::TooManyConnections,
                message: format!("Concurrent connections: {} (threshold: {})", 
                               global_metrics.concurrent_connections, 
                               self.thresholds.concurrent_connections_threshold),
                timestamp: SystemTime::now(),
                severity: AlertSeverity::CursedError,
            });
        }

        // Check route metrics for errors and slow responses
        for (route, metrics) in route_metrics {
            let error_rate = metrics.error_rate();
            if error_rate > self.thresholds.error_rate_threshold {
                self.add_alert(Alert {
                    alert_type: AlertType::HighErrorRate,
                    message: format!("High error rate on {}: {:.1}% (threshold: {:.1}%)", 
                                   route, error_rate, self.thresholds.error_rate_threshold),
                    timestamp: SystemTime::now(),
                    severity: AlertSeverity::CursedError,
                });
            }

            let avg_duration = metrics.average_duration();
            if avg_duration > self.thresholds.response_time_threshold {
                self.add_alert(Alert {
                    alert_type: AlertType::SlowResponse,
                    message: format!("Slow response on {}: {:.3}s (threshold: {:.3}s)", 
                                   route, avg_duration.as_secs_f64(), 
                                   self.thresholds.response_time_threshold.as_secs_f64()),
                    timestamp: SystemTime::now(),
                    severity: AlertSeverity::Warning,
                });
            }
        }
    }

    /// Add custom alert
    pub fn add_custom_alert(&mut self, message: String, severity: AlertSeverity) {
        self.add_alert(Alert {
            alert_type: AlertType::Custom("custom".to_string()),
            message,
            timestamp: SystemTime::now(),
            severity,
        });
    }

    /// Get recent alerts
    pub fn get_recent_alerts(&self, limit: usize) -> Vec<&Alert> {
        self.alerts.iter().rev().take(limit).collect()
    }

    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<&Alert> {
        self.alerts.iter()
            .filter(|alert| matches!(alert.severity, severity))
            .collect()
    }

    /// Clear old alerts
    pub fn clear_old_alerts(&mut self, older_than: Duration) {
        let cutoff = SystemTime::now() - older_than;
        self.alerts.retain(|alert| alert.timestamp > cutoff);
    }

    /// Add alert to system
    fn add_alert(&mut self, alert: Alert) {
        self.alerts.push(alert);
        if self.alerts.len() > self.max_alerts {
            self.alerts.remove(0);
        }
    }

    /// Get alert count by type
    pub fn get_alert_counts(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        
        for alert in &self.alerts {
            let type_name = match &alert.alert_type {
                AlertType::HighCpu => "high_cpu",
                AlertType::HighMemory => "high_memory",
                AlertType::HighErrorRate => "high_error_rate",
                AlertType::SlowResponse => "slow_response",
                AlertType::TooManyConnections => "too_many_connections",
                AlertType::Custom(name) => name,
            };
            
            *counts.entry(type_name.to_string()).or_insert(0) += 1;
        }
        
        counts
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 1024 * 1024 * 1024, // 1GB
            error_rate_threshold: 5.0, // 5%
            response_time_threshold: Duration::from_secs(5),
            concurrent_connections_threshold: 1000,
        }
    }
}


/// Configuration for monitoring dashboard
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub refresh_interval: std::time::Duration,
    pub max_data_points: usize,
    pub enable_real_time: bool,
    pub chart_types: Vec<ChartType>,
}

/// Types of charts available in dashboard
#[derive(Debug, Clone)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Gauge,
    Histogram,
}

impl MonitoringDashboard {
    /// Create new monitoring dashboard
    pub fn new(config: DashboardConfig) -> Self {
        Self {
            collector: MetricsCollector::new(),
            config,
        }
    }

    /// Generate HTML dashboard content
    pub fn generate_html(&self) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>CURSED Monitoring Dashboard</title>
                <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
            </head>
            <body>
                <h1>CURSED Application Metrics</h1>
                <div id="global-metrics">
                    <h2>Global Metrics</h2>
                    <p>Total Requests: {}</p>
                    <p>Total Errors: {}</p>
                    <p>Average Response Time: {:.2}ms</p>
                </div>
                <div id="charts">
                    <canvas id="request-chart"></canvas>
                    <canvas id="error-chart"></canvas>
                </div>
                <script>
                    // Real-time chart updates would go here
                    console.log('Dashboard loaded');
                </script>
            </body>
            </html>
            "#,
            self.collector.global_metrics.total_requests,
            self.collector.global_metrics.total_errors,
            self.collector.global_metrics.average_response_time
        )
    }

    /// Get dashboard metrics as JSON
    pub fn get_metrics_json(&self) -> String {
        self.collector.export_json()
    }

    /// Update dashboard with new request
    pub fn record_request(&mut self, path: &str, duration: std::time::Duration, status_code: u16) {
        self.collector.record_request(path, duration, status_code);
    }
}
