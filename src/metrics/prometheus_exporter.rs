//! Production-ready Prometheus metrics exporter with label sanitization and security
//! 
//! This module provides a comprehensive Prometheus metrics export system with:
//! - Label sanitization to prevent injection attacks
//! - Compliance with Prometheus naming conventions
//! - Proper metric type handling
//! - Label value sanitization and validation
//! - Metric deduplication and validation
//! - Performance optimizations for large metric sets

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

use crate::metrics::{AggregatedMetrics, MetricAlert, AlertSeverity, AlertType, HealthStatus};
use crate::error::CursedError;

/// Prometheus metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrometheusMetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Untyped,
}

impl PrometheusMetricType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge", 
            Self::Histogram => "histogram",
            Self::Summary => "summary",
            Self::Untyped => "untyped",
        }
    }
}

/// A single Prometheus metric with metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrometheusMetric {
    pub name: String,
    pub help: String,
    pub metric_type: PrometheusMetricType,
    pub value: f64,
    pub labels: BTreeMap<String, String>,
    pub timestamp: Option<i64>,
}

impl PrometheusMetric {
    /// Create new metric with sanitized name and labels
    pub fn new(
        name: &str,
        help: &str,
        metric_type: PrometheusMetricType,
        value: f64,
        labels: HashMap<String, String>,
    ) -> Result<Self, CursedError> {
        let sanitized_name = sanitize_metric_name(name)?;
        let sanitized_labels = sanitize_labels(labels)?;
        
        Ok(Self {
            name: sanitized_name,
            help: help.to_string(),
            metric_type,
            value,
            labels: sanitized_labels,
            timestamp: None,
        })
    }

    /// Set timestamp (Unix timestamp in milliseconds)
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Format metric as Prometheus exposition format line
    pub fn to_prometheus_line(&self) -> String {
        let mut line = self.name.clone();
        
        if !self.labels.is_empty() {
            line.push('{');
            let label_pairs: Vec<String> = self.labels
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, escape_label_value(v)))
                .collect();
            line.push_str(&label_pairs.join(","));
            line.push('}');
        }
        
        line.push(' ');
        line.push_str(&format_metric_value(self.value));
        
        if let Some(timestamp) = self.timestamp {
            line.push(' ');
            line.push_str(&timestamp.to_string());
        }
        
        line
    }
}

/// Prometheus exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusExporterConfig {
    /// Namespace prefix for all metrics
    pub namespace: String,
    /// Additional global labels to add to all metrics
    pub global_labels: HashMap<String, String>,
    /// Whether to include timestamps in output
    pub include_timestamps: bool,
    /// Maximum number of metrics to export (prevents memory exhaustion)
    pub max_metrics: usize,
    /// Whether to include help text and type information
    pub include_metadata: bool,
    /// Custom metric name transformations
    pub metric_transformations: HashMap<String, String>,
    /// Label keys to exclude from export
    pub excluded_labels: HashSet<String>,
    /// Whether to validate metric names strictly
    pub strict_validation: bool,
}

impl Default for PrometheusExporterConfig {
    fn default() -> Self {
        Self {
            namespace: "cursed".to_string(),
            global_labels: HashMap::new(),
            include_timestamps: true,
            max_metrics: 10000,
            include_metadata: true,
            metric_transformations: HashMap::new(),
            excluded_labels: HashSet::new(),
            strict_validation: true,
        }
    }
}

/// Main Prometheus metrics exporter
pub struct PrometheusExporter {
    config: PrometheusExporterConfig,
    metrics_cache: HashMap<String, PrometheusMetric>,
}

impl PrometheusExporter {
    /// Create new Prometheus exporter
    pub fn new(config: PrometheusExporterConfig) -> Result<Self, CursedError> {
        // Validate configuration
        if config.namespace.is_empty() {
            return Err(CursedError::runtime_error("Namespace cannot be empty"));
        }
        
        let sanitized_namespace = sanitize_metric_name(&config.namespace)?;
        let mut validated_config = config;
        validated_config.namespace = sanitized_namespace;
        
        // Sanitize global labels
        validated_config.global_labels = sanitize_labels(validated_config.global_labels)?;
        
        Ok(Self {
            config: validated_config,
            metrics_cache: HashMap::new(),
        })
    }

    /// Export aggregated metrics to Prometheus format
    pub fn export_metrics(&mut self, metrics: &AggregatedMetrics) -> Result<String, CursedError> {
        let mut prometheus_metrics = Vec::new();
        
        // Convert aggregated metrics to Prometheus metrics
        self.collect_system_metrics(metrics, &mut prometheus_metrics)?;
        self.collect_performance_metrics(metrics, &mut prometheus_metrics)?;
        self.collect_gc_metrics(metrics, &mut prometheus_metrics)?;
        self.collect_memory_metrics(metrics, &mut prometheus_metrics)?;
        self.collect_alert_metrics(metrics, &mut prometheus_metrics)?;
        self.collect_health_metrics(metrics, &mut prometheus_metrics)?;
        
        // Limit metrics count for safety
        if prometheus_metrics.len() > self.config.max_metrics {
            prometheus_metrics.truncate(self.config.max_metrics);
        }
        
        // Generate Prometheus exposition format
        self.format_prometheus_output(&prometheus_metrics)
    }

    /// Export metrics from cache (for incremental updates)
    pub fn export_cached_metrics(&self) -> Result<String, CursedError> {
        let metrics: Vec<PrometheusMetric> = self.metrics_cache.values().cloned().collect();
        self.format_prometheus_output(&metrics)
    }

    /// Add custom metric to exporter
    pub fn add_metric(
        &mut self,
        name: &str,
        help: &str,
        metric_type: PrometheusMetricType,
        value: f64,
        labels: HashMap<String, String>,
    ) -> Result<(), CursedError> {
        let metric = PrometheusMetric::new(name, help, metric_type, value, labels)?;
        let metric_key = format!("{}_{}", self.config.namespace, metric.name);
        self.metrics_cache.insert(metric_key, metric);
        Ok(())
    }

    /// Clear metrics cache
    pub fn clear_cache(&mut self) {
        self.metrics_cache.clear();
    }

    /// Get metrics count in cache
    pub fn metrics_count(&self) -> usize {
        self.metrics_cache.len()
    }

    // Private helper methods for metric collection
    fn collect_system_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        let timestamp = metrics.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        // System health score
        let health_metric = PrometheusMetric::new(
            &format!("{}_system_health_score", self.config.namespace),
            "Overall system health score (0-100)",
            PrometheusMetricType::Gauge,
            metrics.system_health.overall_score,
            self.get_base_labels(),
        )?.with_timestamp(timestamp);
        output.push(health_metric);

        // Component health scores
        let components = [
            ("compilation", metrics.system_health.compilation_score),
            ("runtime", metrics.system_health.runtime_score),
            ("memory", metrics.system_health.memory_score),
            ("gc", metrics.system_health.gc_score),
            ("performance", metrics.system_health.performance_score),
        ];

        for (component, score) in &components {
            let mut labels = self.get_base_labels();
            labels.insert("component".to_string(), component.to_string());
            
            let metric = PrometheusMetric::new(
                &format!("{}_component_health_score", self.config.namespace),
                "Component health score (0-100)",
                PrometheusMetricType::Gauge,
                *score,
                labels,
            )?.with_timestamp(timestamp);
            output.push(metric);
        }

        // Health status as enum
        let status_value = match metrics.system_health.status {
            HealthStatus::Excellent => 5.0,
            HealthStatus::Good => 4.0,
            HealthStatus::Fair => 3.0,
            HealthStatus::Poor => 2.0,
            HealthStatus::Critical => 1.0,
        };

        let status_metric = PrometheusMetric::new(
            &format!("{}_system_health_status", self.config.namespace),
            "System health status (1=Critical, 2=Poor, 3=Fair, 4=Good, 5=Excellent)",
            PrometheusMetricType::Gauge,
            status_value,
            self.get_base_labels(),
        )?.with_timestamp(timestamp);
        output.push(status_metric);

        Ok(())
    }

    fn collect_performance_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        if let Some(ref perf_metrics) = metrics.performance_metrics {
            let timestamp = metrics.timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64;

            // Future performance metrics
            if let Some(ref future_stats) = perf_metrics.future_stats {
                let mut labels = self.get_base_labels();
                labels.insert("subsystem".to_string(), "futures".to_string());

                let active_metric = PrometheusMetric::new(
                    &format!("{}_active_futures_total", self.config.namespace),
                    "Number of currently active futures",
                    PrometheusMetricType::Gauge,
                    future_stats.active_futures as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(active_metric);

                let completed_metric = PrometheusMetric::new(
                    &format!("{}_completed_futures_total", self.config.namespace),
                    "Total number of completed futures",
                    PrometheusMetricType::Counter,
                    future_stats.completed_futures as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(completed_metric);

                if let Some(avg_duration) = future_stats.average_duration_ms {
                    let duration_metric = PrometheusMetric::new(
                        &format!("{}_future_duration_ms", self.config.namespace),
                        "Average future duration in milliseconds",
                        PrometheusMetricType::Gauge,
                        avg_duration,
                        labels,
                    )?.with_timestamp(timestamp);
                    output.push(duration_metric);
                }
            }

            // Memory performance metrics
            if let Some(ref memory_stats) = perf_metrics.memory_stats {
                let mut labels = self.get_base_labels();
                labels.insert("subsystem".to_string(), "memory".to_string());

                let allocations_metric = PrometheusMetric::new(
                    &format!("{}_memory_allocations_total", self.config.namespace),
                    "Total number of memory allocations",
                    PrometheusMetricType::Counter,
                    memory_stats.total_allocations as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(allocations_metric);

                let bytes_metric = PrometheusMetric::new(
                    &format!("{}_allocated_bytes_total", self.config.namespace),
                    "Total bytes allocated",
                    PrometheusMetricType::Counter,
                    memory_stats.total_allocated_bytes as f64,
                    labels,
                )?.with_timestamp(timestamp);
                output.push(bytes_metric);
            }

            // Thread performance metrics
            if let Some(ref thread_stats) = perf_metrics.thread_stats {
                let mut labels = self.get_base_labels();
                labels.insert("subsystem".to_string(), "threads".to_string());

                let active_metric = PrometheusMetric::new(
                    &format!("{}_active_threads_total", self.config.namespace),
                    "Number of currently active threads",
                    PrometheusMetricType::Gauge,
                    thread_stats.active_threads as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(active_metric);

                let spawned_metric = PrometheusMetric::new(
                    &format!("{}_spawned_threads_total", self.config.namespace),
                    "Total number of spawned threads",
                    PrometheusMetricType::Counter,
                    thread_stats.total_spawned as f64,
                    labels,
                )?.with_timestamp(timestamp);
                output.push(spawned_metric);
            }
        }

        Ok(())
    }

    fn collect_gc_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        if let Some(ref gc_data) = metrics.gc_metrics {
            let timestamp = metrics.timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64;

            let mut labels = self.get_base_labels();
            labels.insert("subsystem".to_string(), "gc".to_string());

            // Try to extract common GC metrics from JSON
            if let Some(collections) = gc_data.get("total_collections").and_then(|v| v.as_u64()) {
                let collections_metric = PrometheusMetric::new(
                    &format!("{}_gc_collections_total", self.config.namespace),
                    "Total number of garbage collections",
                    PrometheusMetricType::Counter,
                    collections as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(collections_metric);
            }

            if let Some(pause_time) = gc_data.get("total_pause_time_ms").and_then(|v| v.as_f64()) {
                let pause_metric = PrometheusMetric::new(
                    &format!("{}_gc_pause_time_ms_total", self.config.namespace),
                    "Total GC pause time in milliseconds",
                    PrometheusMetricType::Counter,
                    pause_time,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(pause_metric);
            }

            if let Some(heap_size) = gc_data.get("heap_size_bytes").and_then(|v| v.as_u64()) {
                let heap_metric = PrometheusMetric::new(
                    &format!("{}_gc_heap_size_bytes", self.config.namespace),
                    "Current GC heap size in bytes",
                    PrometheusMetricType::Gauge,
                    heap_size as f64,
                    labels,
                )?.with_timestamp(timestamp);
                output.push(heap_metric);
            }
        }

        Ok(())
    }

    fn collect_memory_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        if let Some(ref memory_data) = metrics.memory_metrics {
            let timestamp = metrics.timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64;

            let mut labels = self.get_base_labels();
            labels.insert("subsystem".to_string(), "memory".to_string());

            // Extract common memory metrics
            if let Some(used_bytes) = memory_data.get("used_bytes").and_then(|v| v.as_u64()) {
                let used_metric = PrometheusMetric::new(
                    &format!("{}_memory_used_bytes", self.config.namespace),
                    "Currently used memory in bytes",
                    PrometheusMetricType::Gauge,
                    used_bytes as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(used_metric);
            }

            if let Some(total_bytes) = memory_data.get("total_bytes").and_then(|v| v.as_u64()) {
                let total_metric = PrometheusMetric::new(
                    &format!("{}_memory_total_bytes", self.config.namespace),
                    "Total available memory in bytes",
                    PrometheusMetricType::Gauge,
                    total_bytes as f64,
                    labels.clone(),
                )?.with_timestamp(timestamp);
                output.push(total_metric);
            }

            if let Some(allocations) = memory_data.get("allocations_count").and_then(|v| v.as_u64()) {
                let alloc_metric = PrometheusMetric::new(
                    &format!("{}_memory_allocations_total", self.config.namespace),
                    "Total number of memory allocations",
                    PrometheusMetricType::Counter,
                    allocations as f64,
                    labels,
                )?.with_timestamp(timestamp);
                output.push(alloc_metric);
            }
        }

        Ok(())
    }

    fn collect_alert_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        let timestamp = metrics.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        // Count alerts by severity
        let mut severity_counts = HashMap::new();
        let mut type_counts = HashMap::new();

        for alert in &metrics.alerts {
            let severity_key = format!("{:?}", alert.severity).to_lowercase();
            *severity_counts.entry(severity_key).or_insert(0u64) += 1;

            let type_key = format!("{:?}", alert.alert_type).to_lowercase();
            *type_counts.entry(type_key).or_insert(0u64) += 1;
        }

        // Export alert counts by severity
        for (severity, count) in severity_counts {
            let mut labels = self.get_base_labels();
            labels.insert("severity".to_string(), severity);

            let alert_metric = PrometheusMetric::new(
                &format!("{}_alerts_total", self.config.namespace),
                "Total number of alerts by severity",
                PrometheusMetricType::Gauge,
                count as f64,
                labels,
            )?.with_timestamp(timestamp);
            output.push(alert_metric);
        }

        // Export alert counts by type
        for (alert_type, count) in type_counts {
            let mut labels = self.get_base_labels();
            labels.insert("alert_type".to_string(), alert_type);

            let type_metric = PrometheusMetric::new(
                &format!("{}_alerts_by_type_total", self.config.namespace),
                "Total number of alerts by type",
                PrometheusMetricType::Gauge,
                count as f64,
                labels,
            )?.with_timestamp(timestamp);
            output.push(type_metric);
        }

        // Export individual critical alerts as separate metrics
        for alert in &metrics.alerts {
            if alert.severity == AlertSeverity::Critical {
                let mut labels = self.get_base_labels();
                labels.insert("alert_type".to_string(), format!("{:?}", alert.alert_type).to_lowercase());
                labels.insert("metric_name".to_string(), sanitize_label_value(&alert.metric_name));

                let critical_metric = PrometheusMetric::new(
                    &format!("{}_critical_alert", self.config.namespace),
                    "Critical alert indicator (1 = active)",
                    PrometheusMetricType::Gauge,
                    if alert.acknowledged { 0.0 } else { 1.0 },
                    labels,
                )?.with_timestamp(timestamp);
                output.push(critical_metric);
            }
        }

        Ok(())
    }

    fn collect_health_metrics(
        &self,
        metrics: &AggregatedMetrics,
        output: &mut Vec<PrometheusMetric>,
    ) -> Result<(), CursedError> {
        let timestamp = metrics.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        // Export number of recommendations
        let recommendations_metric = PrometheusMetric::new(
            &format!("{}_health_recommendations_total", self.config.namespace),
            "Number of health recommendations",
            PrometheusMetricType::Gauge,
            metrics.system_health.recommendations.len() as f64,
            self.get_base_labels(),
        )?.with_timestamp(timestamp);
        output.push(recommendations_metric);

        Ok(())
    }

    fn get_base_labels(&self) -> HashMap<String, String> {
        let mut labels = self.config.global_labels.clone();
        
        // Remove excluded labels
        for excluded in &self.config.excluded_labels {
            labels.remove(excluded);
        }
        
        labels
    }

    fn format_prometheus_output(&self, metrics: &[PrometheusMetric]) -> Result<String, CursedError> {
        let mut output = String::new();
        let mut seen_metrics = HashSet::new();

        // Group metrics by name for metadata output
        let mut metric_groups: BTreeMap<String, Vec<&PrometheusMetric>> = BTreeMap::new();
        for metric in metrics {
            metric_groups.entry(metric.name.clone()).or_default().push(metric);
        }

        for (metric_name, metric_group) in metric_groups {
            if seen_metrics.contains(&metric_name) {
                continue;
            }
            seen_metrics.insert(metric_name.clone());

            // Get the first metric for metadata
            let first_metric = metric_group.first().unwrap();

            // Write metadata if enabled
            if self.config.include_metadata {
                writeln!(output, "# HELP {} {}", metric_name, first_metric.help)
                    .map_err(|e| CursedError::runtime_error(&format!("Failed to write help: {}", e)))?;
                writeln!(output, "# TYPE {} {}", metric_name, first_metric.metric_type.as_str())
                    .map_err(|e| CursedError::runtime_error(&format!("Failed to write type: {}", e)))?;
            }

            // Write all metric samples for this metric name
            for metric in metric_group {
                writeln!(output, "{}", metric.to_prometheus_line())
                    .map_err(|e| CursedError::runtime_error(&format!("Failed to write metric: {}", e)))?;
            }
        }

        Ok(output)
    }
}

// Label and metric name sanitization functions

/// Sanitize metric name according to Prometheus conventions
pub fn sanitize_metric_name(name: &str) -> Result<String, CursedError> {
    static METRIC_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap()
    });

    if name.is_empty() {
        return Err(CursedError::runtime_error("Metric name cannot be empty"));
    }

    // Replace invalid characters with underscores
    let mut sanitized = String::new();
    let chars: Vec<char> = name.chars().collect();
    
    // First character must be letter, underscore, or colon
    if let Some(first_char) = chars.first() {
        if first_char.is_ascii_alphabetic() || *first_char == '_' || *first_char == ':' {
            sanitized.push(*first_char);
        } else {
            sanitized.push('_');
        }
    }
    
    // Subsequent characters can be letters, digits, underscores, or colons
    for &ch in chars.iter().skip(1) {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == ':' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }

    // Validate final result
    if !METRIC_NAME_REGEX.is_match(&sanitized) {
        return Err(CursedError::runtime_error(&format!(
            "Sanitized metric name '{}' is still invalid", sanitized
        )));
    }

    Ok(sanitized)
}

/// Sanitize label key according to Prometheus conventions
pub fn sanitize_label_key(key: &str) -> Result<String, CursedError> {
    static LABEL_KEY_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap()
    });

    if key.is_empty() {
        return Err(CursedError::runtime_error("Label key cannot be empty"));
    }

    if key.starts_with("__") {
        return Err(CursedError::runtime_error(&format!(
            "Label key '{}' cannot start with '__' (reserved for internal use)", key
        )));
    }

    // Replace invalid characters with underscores
    let mut sanitized = String::new();
    let chars: Vec<char> = key.chars().collect();
    
    // First character must be letter or underscore
    if let Some(first_char) = chars.first() {
        if first_char.is_ascii_alphabetic() || *first_char == '_' {
            sanitized.push(*first_char);
        } else {
            sanitized.push('_');
        }
    }
    
    // Subsequent characters can be letters, digits, or underscores
    for &ch in chars.iter().skip(1) {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }

    // Validate final result
    if !LABEL_KEY_REGEX.is_match(&sanitized) {
        return Err(CursedError::runtime_error(&format!(
            "Sanitized label key '{}' is still invalid", sanitized
        )));
    }

    Ok(sanitized)
}

/// Sanitize label value by removing/escaping dangerous characters
pub fn sanitize_label_value(value: &str) -> String {
    // Remove or replace dangerous characters that could cause injection
    value
        .chars()
        .filter_map(|c| match c {
            // Keep printable ASCII and common Unicode characters
            c if c.is_ascii_graphic() || c.is_ascii_whitespace() => Some(c),
            // Replace other characters with underscore
            _ => Some('_'),
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Escape label value for Prometheus exposition format
pub fn escape_label_value(value: &str) -> String {
    value
        .replace('\\', "\\\\")  // Escape backslashes
        .replace('"', "\\\"")   // Escape quotes
        .replace('\n', "\\n")   // Escape newlines
        .replace('\t', "\\t")   // Escape tabs
        .replace('\r', "\\r")   // Escape carriage returns
}

/// Sanitize a map of labels
pub fn sanitize_labels(labels: HashMap<String, String>) -> Result<BTreeMap<String, String>, CursedError> {
    let mut sanitized = BTreeMap::new();
    
    for (key, value) in labels {
        let sanitized_key = sanitize_label_key(&key)?;
        let sanitized_value = sanitize_label_value(&value);
        
        // Skip empty values
        if !sanitized_value.is_empty() {
            sanitized.insert(sanitized_key, sanitized_value);
        }
    }
    
    Ok(sanitized)
}

/// Format metric value for Prometheus (handles special float values)
pub fn format_metric_value(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_string()
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            "+Inf".to_string()
        } else {
            "-Inf".to_string()
        }
    } else {
        value.to_string()
    }
}

/// Validate Prometheus metric name strictly
pub fn validate_metric_name(name: &str) -> Result<(), CursedError> {
    static METRIC_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z_:][a-zA-Z0-9_:]*$").unwrap()
    });

    if name.is_empty() {
        return Err(CursedError::runtime_error("Metric name cannot be empty"));
    }

    if name.len() > 200 {
        return Err(CursedError::runtime_error("Metric name too long (max 200 characters)"));
    }

    if !METRIC_NAME_REGEX.is_match(name) {
        return Err(CursedError::runtime_error(&format!(
            "Invalid metric name '{}': must match [a-zA-Z_:][a-zA-Z0-9_:]*", name
        )));
    }

    Ok(())
}

/// Validate Prometheus label key strictly
pub fn validate_label_key(key: &str) -> Result<(), CursedError> {
    static LABEL_KEY_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap()
    });

    if key.is_empty() {
        return Err(CursedError::runtime_error("Label key cannot be empty"));
    }

    if key.len() > 100 {
        return Err(CursedError::runtime_error("Label key too long (max 100 characters)"));
    }

    if key.starts_with("__") {
        return Err(CursedError::runtime_error(&format!(
            "Label key '{}' cannot start with '__' (reserved for internal use)", key
        )));
    }

    if !LABEL_KEY_REGEX.is_match(key) {
        return Err(CursedError::runtime_error(&format!(
            "Invalid label key '{}': must match [a-zA-Z_][a-zA-Z0-9_]*", key
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_metric_name() {
        assert_eq!(sanitize_metric_name("valid_metric").unwrap(), "valid_metric");
        assert_eq!(sanitize_metric_name("metric-with-dashes").unwrap(), "metric_with_dashes");
        assert_eq!(sanitize_metric_name("123invalid").unwrap(), "_23invalid");
        assert_eq!(sanitize_metric_name("valid:metric").unwrap(), "valid:metric");
    }

    #[test]
    fn test_sanitize_label_key() {
        assert_eq!(sanitize_label_key("valid_label").unwrap(), "valid_label");
        assert_eq!(sanitize_label_key("label-with-dashes").unwrap(), "label_with_dashes");
        assert!(sanitize_label_key("__reserved").is_err());
        assert_eq!(sanitize_label_key("123invalid").unwrap(), "_23invalid");
    }

    #[test]
    fn test_sanitize_label_value() {
        assert_eq!(sanitize_label_value("normal value"), "normal value");
        assert_eq!(sanitize_label_value("value\nwith\nlines"), "value_with_lines");
        assert_eq!(sanitize_label_value("unicode\u{200B}chars"), "unicode_chars");
    }

    #[test]
    fn test_escape_label_value() {
        assert_eq!(escape_label_value("normal value"), "normal value");
        assert_eq!(escape_label_value("value with \"quotes\""), "value with \\\"quotes\\\"");
        assert_eq!(escape_label_value("value\nwith\nlines"), "value\\nwith\\nlines");
        assert_eq!(escape_label_value("value\\with\\backslashes"), "value\\\\with\\\\backslashes");
    }

    #[test]
    fn test_format_metric_value() {
        assert_eq!(format_metric_value(42.0), "42");
        assert_eq!(format_metric_value(f64::NAN), "NaN");
        assert_eq!(format_metric_value(f64::INFINITY), "+Inf");
        assert_eq!(format_metric_value(f64::NEG_INFINITY), "-Inf");
    }
}
