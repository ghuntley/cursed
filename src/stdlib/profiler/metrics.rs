/// Performance metrics collection and analysis
use crate::stdlib::profiler::error::{ProfilerError, ProfilerResult, metrics_error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicI64, AtomicBool, Ordering}};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Global metrics state
static METRICS_COLLECTOR_STATE: Mutex<Option<Arc<MetricsCollector>>> = Mutex::new(None);
static METRICS_COUNT: AtomicU64 = AtomicU64::new(0);

/// Type of metric
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricType {
    /// Monotonically increasing counter
    Counter,
    /// Instantaneous measurement
    Gauge,
    /// Distribution of values over time
    Histogram,
    /// Time-based measurements
    Timer,
    /// Rate measurements (events per time unit)
    Rate,
    /// Summary statistics
    Summary,
}

/// Metric value representation
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Integer counter or gauge value
    Integer(i64),
    /// Floating point gauge value
    Float(f64),
    /// Duration value for timers
    Duration(Duration),
    /// Distribution of values
    Distribution(Vec<f64>),
    /// Statistical summary
    Summary(MetricSummary),
}

impl MetricValue {
    /// Convert to f64 for calculations
    pub fn as_f64(&self) -> f64 {
        match self {
            MetricValue::Integer(i) => *i as f64,
            MetricValue::Float(f) => *f,
            MetricValue::Duration(d) => d.as_nanos() as f64,
            MetricValue::Distribution(v) => v.iter().sum::<f64>() / v.len() as f64,
            MetricValue::Summary(s) => s.mean,
        }
    }

    /// Check if metric value is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, MetricValue::Integer(_) | MetricValue::Float(_))
    }
}

/// Statistical summary of metric values
#[derive(Debug, Clone)]
pub struct MetricSummary {
    pub count: u64,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub std_dev: f64,
    pub percentiles: HashMap<u8, f64>, // percentile -> value
}

impl MetricSummary {
    /// Create from a series of values
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self::default();
        }

        let count = values.len() as u64;
        let sum = values.iter().sum();
        let mean = sum / values.len() as f64;
        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // Calculate standard deviation
        let variance = values.iter()
            .map(|x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();

        // Calculate percentiles
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut percentiles = HashMap::new();
        for p in [50, 75, 90, 95, 99].iter() {
            let index = ((*p as f64 / 100.0) * (sorted_values.len() - 1) as f64) as usize;
            percentiles.insert(*p, sorted_values[index.min(sorted_values.len() - 1)]);
        }

        Self {
            count,
            sum,
            mean,
            min,
            max,
            std_dev,
            percentiles,
        }
    }
}

impl Default for MetricSummary {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            mean: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            std_dev: 0.0,
            percentiles: HashMap::new(),
        }
    }
}

/// Individual metric definition
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub labels: HashMap<String, String>,
    pub timestamp: SystemTime,
    pub value: MetricValue,
}

impl Metric {
    /// Create a new metric
    pub fn new(name: &str, metric_type: MetricType, value: MetricValue) -> Self {
        Self {
            name: name.to_string(),
            metric_type,
            description: String::new(),
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
            value,
        }
    }

    /// Create metric with description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Add a label to the metric
    pub fn with_label(mut self, key: &str, value: &str) -> Self {
        self.labels.insert(key.to_string(), value.to_string());
        self
    }

    /// Get metric identifier (name + labels)
    pub fn identifier(&self) -> String {
        if self.labels.is_empty() {
            self.name.clone()
        } else {
            let mut id = self.name.clone();
            id.push('{');
            let mut first = true;
            for (k, v) in &self.labels {
                if !first {
                    id.push(',');
                }
                id.push_str(&format!("{}=\"{}\"", k, v));
                first = false;
            }
            id.push('}');
            id
        }
    }
}

/// Counter metric for monotonically increasing values
pub struct CounterMetric {
    name: String,
    value: AtomicU64,
    labels: HashMap<String, String>,
}

impl CounterMetric {
    /// Create a new counter
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: AtomicU64::new(0),
            labels: HashMap::new(),
        }
    }

    /// Increment counter by 1
    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Add a value to the counter
    pub fn add(&self, value: u64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    /// Get current value
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Reset counter to zero
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }

    /// Convert to metric
    pub fn to_metric(&self) -> Metric {
        Metric {
            name: self.name.clone(),
            metric_type: MetricType::Counter,
            description: String::new(),
            labels: self.labels.clone(),
            timestamp: SystemTime::now(),
            value: MetricValue::Integer(self.get() as i64),
        }
    }
}

/// Gauge metric for instantaneous values
pub struct GaugeMetric {
    name: String,
    value: AtomicI64, // Using i64 to support negative values
    labels: HashMap<String, String>,
}

impl GaugeMetric {
    /// Create a new gauge
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: AtomicI64::new(0),
            labels: HashMap::new(),
        }
    }

    /// Set gauge value
    pub fn set(&self, value: i64) {
        self.value.store(value, Ordering::Relaxed);
    }

    /// Increment gauge
    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement gauge
    pub fn decrement(&self) {
        self.value.fetch_sub(1, Ordering::Relaxed);
    }

    /// Add to gauge
    pub fn add(&self, value: i64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    /// Get current value
    pub fn get(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Convert to metric
    pub fn to_metric(&self) -> Metric {
        Metric {
            name: self.name.clone(),
            metric_type: MetricType::Gauge,
            description: String::new(),
            labels: self.labels.clone(),
            timestamp: SystemTime::now(),
            value: MetricValue::Integer(self.get()),
        }
    }
}

/// Histogram metric for distribution of values
pub struct HistogramMetric {
    name: String,
    values: Arc<Mutex<Vec<f64>>>,
    buckets: Vec<f64>,
    labels: HashMap<String, String>,
}

impl HistogramMetric {
    /// Create a new histogram with default buckets
    pub fn new(name: &str) -> Self {
        let buckets = vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];
        Self::with_buckets(name, buckets)
    }

    /// Create histogram with custom buckets
    pub fn with_buckets(name: &str, buckets: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values: Arc::new(Mutex::new(Vec::new())),
            buckets,
            labels: HashMap::new(),
        }
    }

    /// Observe a value
    pub fn observe(&self, value: f64) {
        if let Ok(mut values) = self.values.lock() {
            values.push(value);
        }
    }

    /// Get histogram summary
    pub fn get_summary(&self) -> MetricSummary {
        if let Ok(values) = self.values.lock() {
            MetricSummary::from_values(&values)
        } else {
            MetricSummary::default()
        }
    }

    /// Convert to metric
    pub fn to_metric(&self) -> Metric {
        let summary = self.get_summary();
        Metric {
            name: self.name.clone(),
            metric_type: MetricType::Histogram,
            description: String::new(),
            labels: self.labels.clone(),
            timestamp: SystemTime::now(),
            value: MetricValue::Summary(summary),
        }
    }
}

/// Timer metric for measuring durations
pub struct TimerMetric {
    name: String,
    durations: Arc<Mutex<Vec<Duration>>>,
    labels: HashMap<String, String>,
}

impl TimerMetric {
    /// Create a new timer
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            durations: Arc::new(Mutex::new(Vec::new())),
            labels: HashMap::new(),
        }
    }

    /// Record a duration
    pub fn record(&self, duration: Duration) {
        if let Ok(mut durations) = self.durations.lock() {
            durations.push(duration);
        }
    }

    /// Time a function and record the duration
    pub fn time<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        self.record(start.elapsed());
        result
    }

    /// Get timer summary
    pub fn get_summary(&self) -> MetricSummary {
        if let Ok(durations) = self.durations.lock() {
            let values: Vec<f64> = durations.iter().map(|d| d.as_nanos() as f64).collect();
            MetricSummary::from_values(&values)
        } else {
            MetricSummary::default()
        }
    }

    /// Convert to metric
    pub fn to_metric(&self) -> Metric {
        let summary = self.get_summary();
        Metric {
            name: self.name.clone(),
            metric_type: MetricType::Timer,
            description: String::new(),
            labels: self.labels.clone(),
            timestamp: SystemTime::now(),
            value: MetricValue::Summary(summary),
        }
    }
}

/// Performance metrics collection
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub metrics: HashMap<String, Metric>,
    pub collection_time: SystemTime,
    pub collection_duration: Duration,
    pub total_metrics: usize,
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            collection_time: SystemTime::now(),
            collection_duration: Duration::new(0, 0),
            total_metrics: 0,
        }
    }

    /// Add a metric
    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.insert(metric.identifier(), metric);
        self.total_metrics = self.metrics.len();
    }

    /// Get metric by name
    pub fn get_metric(&self, name: &str) -> Option<&Metric> {
        self.metrics.get(name)
    }

    /// Get all metrics of a specific type
    pub fn get_metrics_by_type(&self, metric_type: MetricType) -> Vec<&Metric> {
        self.metrics
            .values()
            .filter(|m| m.metric_type == metric_type)
            .collect()
    }

    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        
        for metric in self.metrics.values() {
            // Add HELP line
            if !metric.description.is_empty() {
                output.push_str(&format!("# HELP {} {}\n", metric.name, metric.description));
            }
            
            // Add TYPE line
            let type_str = match metric.metric_type {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Histogram => "histogram",
                MetricType::Timer => "histogram",
                MetricType::Rate => "gauge",
                MetricType::Summary => "summary",
            };
            output.push_str(&format!("# TYPE {} {}\n", metric.name, type_str));
            
            // Add metric value
            let value = metric.value.as_f64();
            if metric.labels.is_empty() {
                output.push_str(&format!("{} {}\n", metric.name, value));
            } else {
                let labels = metric.labels
                    .iter()
                    .map(|(k, v)| format!("{}=\"{}\"", k, v))
                    .collect::<Vec<_>>()
                    .join(",");
                output.push_str(&format!("{}{{{}}} {}\n", metric.name, labels, value));
            }
        }
        
        output
    }

    /// Export metrics in JSON format
    pub fn export_json(&self) -> ProfilerResult<String> {
        let json_metrics: HashMap<String, serde_json::Value> = self.metrics
            .iter()
            .map(|(name, metric)| {
                let mut map = serde_json::Map::new();
                map.insert("type".to_string(), serde_json::Value::String(format!("{:?}", metric.metric_type)));
                map.insert("value".to_string(), serde_json::Value::Number(
                    serde_json::Number::from_f64(metric.value.as_f64()).unwrap_or(serde_json::Number::from(0))
                ));
                map.insert("timestamp".to_string(), serde_json::Value::Number(
                    serde_json::Number::from(
                        metric.timestamp.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
                    )
                ));
                map.insert("labels".to_string(), serde_json::Value::Object(
                    metric.labels.iter().map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone()))).collect()
                ));
                (name.clone(), serde_json::Value::Object(map))
            })
            .collect();

        serde_json::to_string_pretty(&json_metrics)
            .map_err(|e| metrics_error(&format!("JSON serialization failed: {}", e)))
    }
}

/// Metrics collector for gathering performance data
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
    is_collecting: AtomicBool,
    collection_interval: Duration,
    start_time: Option<Instant>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            is_collecting: AtomicBool::new(false),
            collection_interval: Duration::from_secs(1),
            start_time: None,
        }
    }

    /// Create with custom collection interval
    pub fn with_interval(interval: Duration) -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            is_collecting: AtomicBool::new(false),
            collection_interval: interval,
            start_time: None,
        }
    }

    /// Start metrics collection
    pub fn start(&mut self) -> ProfilerResult<()> {
        if self.is_collecting.load(Ordering::Relaxed) {
            return Err(ProfilerError::AlreadyRunning);
        }

        self.is_collecting.store(true, Ordering::Relaxed);
        self.start_time = Some(Instant::now());

        // Clear previous metrics
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.clear();
        }

        Ok(())
    }

    /// Stop metrics collection
    pub fn stop(&mut self) -> ProfilerResult<PerformanceMetrics> {
        if !self.is_collecting.load(Ordering::Relaxed) {
            return Err(ProfilerError::NotRunning);
        }

        self.is_collecting.store(false, Ordering::Relaxed);
        let collection_duration = if let Some(start) = self.start_time {
            start.elapsed()
        } else {
            Duration::new(0, 0)
        };

        let metrics = self.metrics.lock()
            .map_err(|_| ProfilerError::General("Failed to lock metrics".to_string()))?
            .clone();

        let mut performance_metrics = PerformanceMetrics {
            metrics,
            collection_time: SystemTime::now(),
            collection_duration,
            total_metrics: 0,
        };
        performance_metrics.total_metrics = performance_metrics.metrics.len();

        METRICS_COUNT.fetch_add(performance_metrics.total_metrics as u64, Ordering::Relaxed);

        Ok(performance_metrics)
    }

    /// Record a metric
    pub fn record_metric(&self, metric: Metric) -> ProfilerResult<()> {
        if !self.is_collecting.load(Ordering::Relaxed) {
            return Ok(()); // Silently ignore when not collecting
        }

        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.insert(metric.identifier(), metric);
        }

        Ok(())
    }

    /// Check if collecting metrics
    pub fn is_collecting(&self) -> bool {
        self.is_collecting.load(Ordering::Relaxed)
    }

    /// Get current metrics snapshot
    pub fn get_current_metrics(&self) -> ProfilerResult<PerformanceMetrics> {
        let metrics = self.metrics.lock()
            .map_err(|_| ProfilerError::General("Failed to lock metrics".to_string()))?
            .clone();

        let collection_duration = if let Some(start) = self.start_time {
            start.elapsed()
        } else {
            Duration::new(0, 0)
        };

        let mut performance_metrics = PerformanceMetrics {
            metrics,
            collection_time: SystemTime::now(),
            collection_duration,
            total_metrics: 0,
        };
        performance_metrics.total_metrics = performance_metrics.metrics.len();

        Ok(performance_metrics)
    }
}

/// Collect current system metrics
pub fn collect_metrics() -> ProfilerResult<PerformanceMetrics> {
    let mut metrics = PerformanceMetrics::new();
    let collection_start = Instant::now();

    // CPU metrics
    let cpu_metric = Metric::new("cpu_usage_percent", MetricType::Gauge, 
        MetricValue::Float(get_cpu_usage()));
    metrics.add_metric(cpu_metric);

    // Memory metrics
    let memory_metric = Metric::new("memory_usage_bytes", MetricType::Gauge,
        MetricValue::Integer(get_memory_usage() as i64));
    metrics.add_metric(memory_metric);

    // Goroutine metrics (if available)
    if let Ok(goroutine_count) = get_goroutine_count() {
        let goroutine_metric = Metric::new("goroutines_active", MetricType::Gauge,
            MetricValue::Integer(goroutine_count as i64));
        metrics.add_metric(goroutine_metric);
    }

    // GC metrics (if available)
    if let Ok(gc_stats) = get_gc_stats() {
        let gc_collections_metric = Metric::new("gc_collections_total", MetricType::Counter,
            MetricValue::Integer(gc_stats.collections as i64));
        metrics.add_metric(gc_collections_metric);

        let gc_duration_metric = Metric::new("gc_duration_seconds", MetricType::Histogram,
            MetricValue::Duration(gc_stats.total_duration));
        metrics.add_metric(gc_duration_metric);
    }

    metrics.collection_duration = collection_start.elapsed();
    METRICS_COUNT.fetch_add(metrics.total_metrics as u64, Ordering::Relaxed);

    Ok(metrics)
}

/// Start metrics collection
pub fn start_metrics_collection() -> ProfilerResult<()> {
    let mut state = METRICS_COLLECTOR_STATE.lock()
        .map_err(|_| ProfilerError::General("Failed to lock metrics collector state".to_string()))?;

    if state.is_some() {
        return Err(ProfilerError::AlreadyRunning);
    }

    let mut collector = MetricsCollector::new();
    collector.start()?;
    *state = Some(Arc::new(collector));

    Ok(())
}

/// Stop metrics collection
pub fn stop_metrics_collection() -> ProfilerResult<PerformanceMetrics> {
    let mut state = METRICS_COLLECTOR_STATE.lock()
        .map_err(|_| ProfilerError::General("Failed to lock metrics collector state".to_string()))?;

    if let Some(_collector_arc) = state.take() {
        // Return dummy metrics for now
        // In real implementation, we'd need a different approach to get mutable access
        let metrics = collect_metrics()?;
        Ok(metrics)
    } else {
        Err(ProfilerError::NotRunning)
    }
}

/// Get current metrics
pub fn get_current_metrics() -> ProfilerResult<PerformanceMetrics> {
    collect_metrics()
}

/// Export metrics in specified format
pub fn export_metrics(format: &str) -> ProfilerResult<String> {
    let metrics = collect_metrics()?;
    
    match format.to_lowercase().as_str() {
        "prometheus" => Ok(metrics.export_prometheus()),
        "json" => metrics.export_json(),
        _ => Err(metrics_error(&format!("Unsupported export format: {}", format))),
    }
}

/// Get number of metrics collected
pub fn get_metrics_count() -> u64 {
    METRICS_COUNT.load(Ordering::Relaxed)
}

// Helper functions for system metrics (simplified implementations)

fn get_cpu_usage() -> f64 {
    // Simplified CPU usage calculation
    // In real implementation, would use system calls or /proc/stat
    std::thread::sleep(Duration::from_millis(1));
    42.5 // Dummy value
}

fn get_memory_usage() -> u64 {
    // Simplified memory usage
    // In real implementation, would use system calls or /proc/meminfo
    1024 * 1024 * 512 // 512 MB
}

fn get_goroutine_count() -> ProfilerResult<usize> {
    // Would integrate with runtime goroutine system
    Ok(8) // Dummy value
}

struct GcStats {
    collections: u64,
    total_duration: Duration,
}

fn get_gc_stats() -> ProfilerResult<GcStats> {
    // Would integrate with GC system
    Ok(GcStats {
        collections: 42,
        total_duration: Duration::from_millis(100),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_creation() {
        let metric = Metric::new("test_metric", MetricType::Counter, MetricValue::Integer(42))
            .with_description("Test metric")
            .with_label("component", "test");

        assert_eq!(metric.name, "test_metric");
        assert_eq!(metric.metric_type, MetricType::Counter);
        assert_eq!(metric.description, "Test metric");
        assert_eq!(metric.labels.get("component"), Some(&"test".to_string()));
    }

    #[test]
    fn test_counter_metric() {
        let counter = CounterMetric::new("requests_total");
        
        assert_eq!(counter.get(), 0);
        
        counter.increment();
        assert_eq!(counter.get(), 1);
        
        counter.add(5);
        assert_eq!(counter.get(), 6);
        
        counter.reset();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_gauge_metric() {
        let gauge = GaugeMetric::new("temperature");
        
        gauge.set(25);
        assert_eq!(gauge.get(), 25);
        
        gauge.increment();
        assert_eq!(gauge.get(), 26);
        
        gauge.decrement();
        assert_eq!(gauge.get(), 25);
        
        gauge.add(-5);
        assert_eq!(gauge.get(), 20);
    }

    #[test]
    fn test_histogram_metric() {
        let histogram = HistogramMetric::new("request_duration");
        
        histogram.observe(0.1);
        histogram.observe(0.2);
        histogram.observe(0.15);
        
        let summary = histogram.get_summary();
        assert_eq!(summary.count, 3);
        assert_eq!(summary.mean, 0.15);
    }

    #[test]
    fn test_timer_metric() {
        let timer = TimerMetric::new("operation_duration");
        
        let result = timer.time(|| {
            std::thread::sleep(Duration::from_millis(1));
            42
        });
        
        assert_eq!(result, 42);
        
        let summary = timer.get_summary();
        assert!(summary.count > 0);
        assert!(summary.mean > 0.0);
    }

    #[test]
    fn test_metric_summary() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let summary = MetricSummary::from_values(&values);
        
        assert_eq!(summary.count, 5);
        assert_eq!(summary.mean, 3.0);
        assert_eq!(summary.min, 1.0);
        assert_eq!(summary.max, 5.0);
        assert!(summary.std_dev > 0.0);
    }

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        
        let metric = Metric::new("test", MetricType::Counter, MetricValue::Integer(1));
        metrics.add_metric(metric);
        
        assert_eq!(metrics.total_metrics, 1);
        assert!(metrics.get_metric("test").is_some());
        
        let counters = metrics.get_metrics_by_type(MetricType::Counter);
        assert_eq!(counters.len(), 1);
    }

    #[test]
    fn test_metrics_collector_lifecycle() {
        let mut collector = MetricsCollector::new();
        
        // Start collection
        collector.start().unwrap();
        assert!(collector.is_collecting());
        
        // Record a metric
        let metric = Metric::new("test", MetricType::Counter, MetricValue::Integer(1));
        collector.record_metric(metric).unwrap();
        
        // Stop collection
        let metrics = collector.stop().unwrap();
        assert!(!collector.is_collecting());
        assert_eq!(metrics.total_metrics, 1);
    }

    #[test]
    fn test_prometheus_export() {
        let mut metrics = PerformanceMetrics::new();
        
        let metric = Metric::new("test_counter", MetricType::Counter, MetricValue::Integer(42))
            .with_description("A test counter")
            .with_label("service", "test");
        
        metrics.add_metric(metric);
        
        let prometheus_output = metrics.export_prometheus();
        assert!(prometheus_output.contains("# HELP test_counter A test counter"));
        assert!(prometheus_output.contains("# TYPE test_counter counter"));
        assert!(prometheus_output.contains("test_counter{service=\"test\"} 42"));
    }

    #[test]
    fn test_json_export() {
        let mut metrics = PerformanceMetrics::new();
        
        let metric = Metric::new("test_gauge", MetricType::Gauge, MetricValue::Float(3.14));
        metrics.add_metric(metric);
        
        let json_output = metrics.export_json().unwrap();
        assert!(json_output.contains("test_gauge"));
        assert!(json_output.contains("3.14"));
    }

    #[test]
    fn test_global_metrics_functions() {
        let metrics = collect_metrics().unwrap();
        assert!(metrics.total_metrics > 0);
        
        let prometheus_export = export_metrics("prometheus").unwrap();
        assert!(prometheus_export.contains("cpu_usage_percent"));
        
        let json_export = export_metrics("json").unwrap();
        assert!(json_export.contains("memory_usage_bytes"));
        
        let count = get_metrics_count();
        assert!(count > 0);
    }
}
