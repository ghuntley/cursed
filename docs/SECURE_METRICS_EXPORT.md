# CURSED Secure Metrics Export System

## Overview

The CURSED compiler includes a production-ready metrics export system with comprehensive security features designed to prevent injection attacks and ensure compliance with Prometheus format standards. This system provides secure, sanitized metrics export with proper label validation and format compliance.

## 🔒 Security Features

### Label Sanitization
- **Injection Prevention**: Automatically sanitizes metric names and label values to prevent injection attacks
- **Character Filtering**: Removes dangerous characters that could cause format violations
- **Escape Handling**: Properly escapes special characters in label values
- **Reserved Label Protection**: Prevents use of reserved label prefixes (`__`)

### Validation System
- **Metric Name Validation**: Ensures compliance with Prometheus naming conventions
- **Label Key Validation**: Validates label keys according to format standards
- **Length Limits**: Enforces reasonable limits to prevent resource exhaustion
- **Strict Mode**: Optional strict validation for production environments

### Attack Prevention
- **SQL Injection**: Sanitizes input to prevent SQL injection attempts
- **XSS Prevention**: Removes script tags and dangerous HTML entities
- **Newline Injection**: Prevents metric format corruption via newline characters
- **Unicode Attacks**: Handles Unicode control characters and homograph attacks

## 📊 Usage Examples

### Basic Metrics Export

```rust
use cursed::metrics::prometheus_exporter::{PrometheusExporter, PrometheusExporterConfig};

// Create secure exporter configuration
let config = PrometheusExporterConfig {
    namespace: "my_app".to_string(),
    global_labels: {
        let mut labels = HashMap::new();
        labels.insert("environment".to_string(), "production".to_string());
        labels.insert("version".to_string(), "1.0.0".to_string());
        labels
    },
    strict_validation: true,
    max_metrics: 5000,
    ..Default::default()
};

let mut exporter = PrometheusExporter::new(config)?;
let metrics = get_current_metrics();
let prometheus_output = exporter.export_metrics(&metrics)?;

// Output is now safely sanitized and format-compliant
println!("{}", prometheus_output);
```

### Security Testing

```rust
use cursed::metrics::prometheus_exporter::*;

// Test injection attempts
let malicious_tests = vec![
    ("SQL injection", "'; DROP TABLE metrics; --"),
    ("XSS attempt", "<script>alert('xss')</script>"),
    ("Newline injection", "value\n# MALICIOUS\nfake_metric 999"),
];

for (test_name, malicious_input) in malicious_tests {
    let mut labels = HashMap::new();
    labels.insert("test_input".to_string(), malicious_input.to_string());
    
    // This will safely sanitize the malicious input
    let result = exporter.add_metric(
        "security_test",
        "Security test metric",
        PrometheusMetricType::Gauge,
        1.0,
        labels,
    );
    
    assert!(result.is_ok()); // Should handle safely
    
    let output = exporter.export_cached_metrics()?;
    assert!(!output.contains(malicious_input)); // Should be sanitized
}
```

### CLI Tool Usage

```bash
# Export current metrics with security validation
cursed-metrics export --namespace my_app --strict --output metrics.prom

# Validate existing metrics for compliance
cursed-metrics validate input.json --strict --fix --output fixed.json

# Run security tests
cursed-metrics security-test --verbose --report security_report.json

# Monitor metrics continuously
cursed-metrics monitor --interval 5000 --output live_metrics.prom --alerts
```

## 🛡️ Security Implementation Details

### Metric Name Sanitization

```rust
pub fn sanitize_metric_name(name: &str) -> Result<String, CursedError> {
    // Ensures metric names match: [a-zA-Z_:][a-zA-Z0-9_:]*
    // - First character: letter, underscore, or colon
    // - Subsequent characters: letters, digits, underscores, colons
    // - Invalid characters replaced with underscores
}
```

### Label Key Sanitization

```rust
pub fn sanitize_label_key(key: &str) -> Result<String, CursedError> {
    // Ensures label keys match: [a-zA-Z_][a-zA-Z0-9_]*
    // - First character: letter or underscore
    // - Subsequent characters: letters, digits, underscores
    // - Rejects reserved prefixes (__*)
}
```

### Label Value Sanitization

```rust
pub fn sanitize_label_value(value: &str) -> String {
    // Removes dangerous characters:
    // - Non-printable ASCII characters
    // - Unicode control characters
    // - Potential injection vectors
    // - Preserves printable ASCII and whitespace
}
```

### Escape Handling

```rust
pub fn escape_label_value(value: &str) -> String {
    // Properly escapes for Prometheus format:
    // - Backslashes: \ → \\
    // - Quotes: " → \"
    // - Newlines: \n → \\n
    // - Tabs: \t → \\t
    // - Carriage returns: \r → \\r
}
```

## 📋 Configuration Options

### PrometheusExporterConfig

```rust
pub struct PrometheusExporterConfig {
    /// Namespace prefix for all metrics
    pub namespace: String,
    
    /// Global labels added to all metrics
    pub global_labels: HashMap<String, String>,
    
    /// Include timestamps in output
    pub include_timestamps: bool,
    
    /// Maximum metrics to prevent memory exhaustion
    pub max_metrics: usize,
    
    /// Include HELP and TYPE metadata
    pub include_metadata: bool,
    
    /// Custom metric name transformations
    pub metric_transformations: HashMap<String, String>,
    
    /// Label keys to exclude from export
    pub excluded_labels: HashSet<String>,
    
    /// Enable strict validation mode
    pub strict_validation: bool,
}
```

### Security Defaults

```rust
impl Default for PrometheusExporterConfig {
    fn default() -> Self {
        Self {
            namespace: "cursed".to_string(),
            global_labels: HashMap::new(),
            include_timestamps: true,
            max_metrics: 10000,        // Prevent resource exhaustion
            include_metadata: true,
            metric_transformations: HashMap::new(),
            excluded_labels: HashSet::new(),
            strict_validation: true,   // Security-first approach
        }
    }
}
```

## 🔍 Validation Functions

### Strict Validation

```rust
// Validate metric names
validate_metric_name("valid_metric")?;          // ✅ OK
validate_metric_name("123invalid")?;            // ❌ Error
validate_metric_name("")?;                      // ❌ Error
validate_metric_name("too_long_name_...")?;     // ❌ Error (>200 chars)

// Validate label keys
validate_label_key("valid_label")?;             // ✅ OK
validate_label_key("__reserved")?;              // ❌ Error
validate_label_key("123invalid")?;              // ❌ Error
validate_label_key("")?;                        // ❌ Error
```

### Sanitization Results

```rust
// Metric name sanitization
assert_eq!(sanitize_metric_name("metric-with-dashes")?, "metric_with_dashes");
assert_eq!(sanitize_metric_name("123invalid")?, "_23invalid");
assert_eq!(sanitize_metric_name("metric.dots")?, "metric_dots");

// Label value sanitization
assert_eq!(sanitize_label_value("normal value"), "normal value");
assert_eq!(sanitize_label_value("value\nwith\nlines"), "value_with_lines");
assert_eq!(sanitize_label_value("unicode\u{200B}hidden"), "unicode_hidden");

// Label value escaping
assert_eq!(escape_label_value("value with \"quotes\""), "value with \\\"quotes\\\"");
assert_eq!(escape_label_value("value\nwith\nlines"), "value\\nwith\\nlines");
```

## 🚨 Security Testing

### Automated Security Tests

The system includes comprehensive security tests that validate protection against:

1. **SQL Injection Attacks**
   ```
   '; DROP TABLE metrics; --
   ' UNION SELECT * FROM secrets --
   ```

2. **XSS Attacks**
   ```
   <script>alert('xss')</script>
   javascript:malicious_code()
   ```

3. **Format Injection**
   ```
   value\n# HELP fake_metric Injected metric
   value\nfake_metric{} 999
   ```

4. **Unicode Attacks**
   ```
   value\u{200B}\u{FEFF}hidden_content
   Сyrillic_lookalike_characters
   ```

### Running Security Tests

```bash
# Run all security tests
cursed-metrics security-test --verbose

# Test specific attack vectors
cursed-metrics security-test --test-type sql --verbose
cursed-metrics security-test --test-type xss --verbose
cursed-metrics security-test --test-type newline --verbose
cursed-metrics security-test --test-type unicode --verbose

# Generate security report
cursed-metrics security-test --report security_audit.json
```

## 📈 Performance Considerations

### Memory Management
- **Metric Limits**: Configurable maximum metric counts prevent memory exhaustion
- **Efficient Sanitization**: Regex compilation is cached using `once_cell::sync::Lazy`
- **Streaming Export**: Large metric sets are processed efficiently

### Processing Optimization
- **Batch Processing**: Metrics are grouped by name for efficient metadata output
- **Deduplication**: Automatic deduplication of metric metadata
- **Caching**: Sanitization results can be cached for repeated exports

### Production Scaling
```rust
// Production configuration
let config = PrometheusExporterConfig {
    max_metrics: 50000,           // Higher limits for production
    include_timestamps: true,     // Full timestamp precision
    strict_validation: true,      // Maximum security
    global_labels: production_labels(),
    ..Default::default()
};
```

## 🔧 Integration Examples

### With CURSED Metrics Manager

```rust
use cursed::metrics::{MetricsManager, MetricsConfig};
use cursed::metrics::prometheus_exporter::PrometheusExporterConfig;

// Configure metrics collection
let metrics_config = MetricsConfig {
    enable_compilation_metrics: true,
    enable_runtime_metrics: true,
    enable_gc_metrics: true,
    export_format: MetricsExportFormat::Prometheus,
    ..Default::default()
};

let metrics_manager = MetricsManager::new(metrics_config)?;
metrics_manager.start()?;

// Export with security
let prometheus_config = PrometheusExporterConfig {
    namespace: "cursed_compiler".to_string(),
    strict_validation: true,
    ..Default::default()
};

let mut exporter = PrometheusExporter::new(prometheus_config)?;
let current_metrics = metrics_manager.get_current_metrics();
let secure_output = exporter.export_metrics(&current_metrics)?;
```

### With Custom Applications

```rust
// Add custom application metrics
exporter.add_metric(
    "application_requests_total",
    "Total number of application requests",
    PrometheusMetricType::Counter,
    request_count as f64,
    labels,
)?;

exporter.add_metric(
    "application_response_time_seconds",
    "Application response time in seconds",
    PrometheusMetricType::Histogram,
    response_time,
    labels,
)?;
```

## 🚀 Production Deployment

### Docker Integration

```dockerfile
FROM rust:1.70 AS builder
COPY . /app
WORKDIR /app
RUN cargo build --release --bin cursed-metrics

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/cursed-metrics /usr/local/bin/
EXPOSE 9090
CMD ["cursed-metrics", "monitor", "--interval", "5000", "--alerts"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cursed-metrics-exporter
spec:
  replicas: 1
  selector:
    matchLabels:
      app: cursed-metrics
  template:
    metadata:
      labels:
        app: cursed-metrics
    spec:
      containers:
      - name: metrics-exporter
        image: cursed-metrics:latest
        ports:
        - containerPort: 9090
        args:
          - "monitor"
          - "--interval=5000"
          - "--alerts"
          - "--webhook=http://alertmanager:9093/api/v1/alerts"
        env:
        - name: CURSED_METRICS_NAMESPACE
          value: "cursed_production"
        - name: CURSED_METRICS_STRICT
          value: "true"
```

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'cursed-metrics'
    static_configs:
      - targets: ['cursed-metrics-exporter:9090']
    scrape_interval: 10s
    metrics_path: /metrics
    scheme: http
```

## 📚 API Reference

### Core Functions

```rust
// Sanitization functions
pub fn sanitize_metric_name(name: &str) -> Result<String, CursedError>
pub fn sanitize_label_key(key: &str) -> Result<String, CursedError>
pub fn sanitize_label_value(value: &str) -> String
pub fn escape_label_value(value: &str) -> String
pub fn sanitize_labels(labels: HashMap<String, String>) -> Result<BTreeMap<String, String>, CursedError>

// Validation functions
pub fn validate_metric_name(name: &str) -> Result<(), CursedError>
pub fn validate_label_key(key: &str) -> Result<(), CursedError>

// Formatting functions
pub fn format_metric_value(value: f64) -> String
```

### PrometheusExporter Methods

```rust
impl PrometheusExporter {
    pub fn new(config: PrometheusExporterConfig) -> Result<Self, CursedError>
    pub fn export_metrics(&mut self, metrics: &AggregatedMetrics) -> Result<String, CursedError>
    pub fn add_metric(&mut self, name: &str, help: &str, metric_type: PrometheusMetricType, value: f64, labels: HashMap<String, String>) -> Result<(), CursedError>
    pub fn clear_cache(&mut self)
    pub fn metrics_count(&self) -> usize
    pub fn export_cached_metrics(&self) -> Result<String, CursedError>
}
```

## 🏆 Compliance & Standards

### Prometheus Exposition Format
- Full compliance with [Prometheus exposition format](https://prometheus.io/docs/instrumenting/exposition_formats/)
- Proper metric naming conventions
- Correct label formatting and escaping
- Standard metric types (counter, gauge, histogram, summary)

### Security Standards
- OWASP injection prevention guidelines
- Input validation and sanitization best practices
- Secure by default configuration
- Comprehensive security testing

### Performance Standards
- Memory usage limits and monitoring
- Efficient processing algorithms
- Configurable resource constraints
- Production-ready scalability

This secure metrics export system provides enterprise-grade security while maintaining full Prometheus compatibility, making it suitable for production deployments where security is paramount.
