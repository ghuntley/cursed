//! Comprehensive example demonstrating secure Prometheus metrics export
//! 
//! This example shows how to use the CURSED metrics system with proper
//! label sanitization and security measures to prevent injection attacks.

use std::collections::HashMap;
use std::time::{SystemTime, Duration};

use cursed::metrics::{
    MetricsManager, MetricsConfig, AggregatedMetrics, SystemHealthScore, 
    HealthStatus, MetricAlert, AlertSeverity, AlertType
};
use cursed::metrics::prometheus_exporter::{
    PrometheusExporter, PrometheusExporterConfig, PrometheusMetricType
};
use cursed::error::CursedError;

fn main() -> Result<(), CursedError> {
    println!("🔒 CURSED Secure Metrics Export Demonstration");
    println!("=" .repeat(50));
    
    // Example 1: Basic secure metrics export
    demonstrate_basic_export()?;
    
    // Example 2: Label sanitization and injection prevention
    demonstrate_security_features()?;
    
    // Example 3: Custom metrics with validation
    demonstrate_custom_metrics()?;
    
    // Example 4: Production-ready configuration
    demonstrate_production_config()?;
    
    // Example 5: Performance monitoring integration
    demonstrate_performance_integration()?;
    
    Ok(())
}

/// Demonstrate basic secure metrics export
fn demonstrate_basic_export() -> Result<(), CursedError> {
    println!("\n📊 Basic Secure Metrics Export");
    println!("-".repeat(30));
    
    // Create secure Prometheus exporter configuration
    let config = PrometheusExporterConfig {
        namespace: "cursed_demo".to_string(),
        global_labels: {
            let mut labels = HashMap::new();
            labels.insert("environment".to_string(), "demo".to_string());
            labels.insert("service".to_string(), "cursed_compiler".to_string());
            labels
        },
        include_timestamps: true,
        max_metrics: 1000,
        include_metadata: true,
        strict_validation: true,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Create sample metrics
    let metrics = create_sample_metrics();
    
    // Export to Prometheus format
    let prometheus_output = exporter.export_metrics(&metrics)?;
    
    println!("✅ Exported {} lines of Prometheus metrics", prometheus_output.lines().count());
    println!("📄 Sample output (first 10 lines):");
    for (i, line) in prometheus_output.lines().take(10).enumerate() {
        println!("  {}: {}", i + 1, line);
    }
    
    Ok(())
}

/// Demonstrate security features and label sanitization
fn demonstrate_security_features() -> Result<(), CursedError> {
    println!("\n🛡️  Security Features & Label Sanitization");
    println!("-".repeat(40));
    
    let config = PrometheusExporterConfig {
        namespace: "security_test".to_string(),
        strict_validation: true,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Test various injection attempts and malformed labels
    let malicious_tests = vec![
        ("SQL injection attempt", "'; DROP TABLE metrics; --"),
        ("Newline injection", "value\n# MALICIOUS COMMENT\nfake_metric 999"),
        ("Quote injection", "value\"}, fake_metric{instance=\"injected\" 999"),
        ("Unicode control chars", "value\u{200B}\u{FEFF}with\u{2028}hidden\u{2029}chars"),
        ("Backslash injection", "value\\n\\t\\r\\\\attack"),
    ];
    
    for (test_name, malicious_value) in malicious_tests {
        println!("🔍 Testing: {}", test_name);
        
        let mut labels = HashMap::new();
        labels.insert("test_type".to_string(), test_name.to_string());
        labels.insert("malicious_label".to_string(), malicious_value.to_string());
        
        // This should safely sanitize the malicious input
        match exporter.add_metric(
            "security_test_metric",
            "Test metric for security validation",
            PrometheusMetricType::Gauge,
            1.0,
            labels,
        ) {
            Ok(_) => println!("  ✅ Safely handled malicious input"),
            Err(e) => println!("  ⚠️  Rejected with error: {}", e),
        }
    }
    
    // Export and verify sanitization
    let output = exporter.export_cached_metrics()?;
    
    // Verify no injection succeeded
    assert!(!output.contains("DROP TABLE"));
    assert!(!output.contains("fake_metric"));
    assert!(!output.lines().any(|line| line.contains("# MALICIOUS")));
    
    println!("✅ All injection attempts were safely sanitized");
    println!("📄 Sanitized output sample:");
    for line in output.lines().take(5) {
        println!("  {}", line);
    }
    
    Ok(())
}

/// Demonstrate custom metrics with strict validation
fn demonstrate_custom_metrics() -> Result<(), CursedError> {
    println!("\n⚙️  Custom Metrics with Validation");
    println!("-".repeat(35));
    
    let config = PrometheusExporterConfig {
        namespace: "cursed_custom".to_string(),
        strict_validation: true,
        max_metrics: 100,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Add various types of custom metrics
    let custom_metrics = vec![
        ("compilation_time_seconds", "Time taken for compilation", PrometheusMetricType::Histogram, 2.5),
        ("active_goroutines", "Number of active goroutines", PrometheusMetricType::Gauge, 42.0),
        ("memory_allocations_total", "Total memory allocations", PrometheusMetricType::Counter, 1000.0),
        ("error_rate", "Current error rate percentage", PrometheusMetricType::Gauge, 0.5),
        ("build_success_total", "Total successful builds", PrometheusMetricType::Counter, 150.0),
    ];
    
    for (name, description, metric_type, value) in custom_metrics {
        let mut labels = HashMap::new();
        labels.insert("component".to_string(), "compiler".to_string());
        labels.insert("version".to_string(), "1.0.0".to_string());
        
        exporter.add_metric(name, description, metric_type, value, labels)?;
        println!("✅ Added metric: {} ({})", name, metric_type.as_str());
    }
    
    println!("📊 Total metrics in cache: {}", exporter.metrics_count());
    
    let output = exporter.export_cached_metrics()?;
    println!("📄 Generated {} lines of output", output.lines().count());
    
    Ok(())
}

/// Demonstrate production-ready configuration
fn demonstrate_production_config() -> Result<(), CursedError> {
    println!("\n🏭 Production-Ready Configuration");
    println!("-".repeat(35));
    
    // Production configuration with security hardening
    let mut excluded_labels = std::collections::HashSet::new();
    excluded_labels.insert("internal_debug".to_string());
    excluded_labels.insert("temporary".to_string());
    
    let mut transformations = HashMap::new();
    transformations.insert("old_metric_name".to_string(), "new_metric_name".to_string());
    
    let config = PrometheusExporterConfig {
        namespace: "cursed_prod".to_string(),
        global_labels: {
            let mut labels = HashMap::new();
            labels.insert("datacenter".to_string(), "us-west-2".to_string());
            labels.insert("cluster".to_string(), "production".to_string());
            labels.insert("service".to_string(), "cursed-compiler".to_string());
            labels.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
            labels
        },
        include_timestamps: true,
        max_metrics: 5000,  // Higher limit for production
        include_metadata: true,
        metric_transformations: transformations,
        excluded_labels,
        strict_validation: true,
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Add production metrics
    let prod_metrics = create_production_metrics();
    let output = exporter.export_metrics(&prod_metrics)?;
    
    println!("✅ Production metrics exported successfully");
    println!("📊 Metrics lines: {}", output.lines().count());
    println!("🏷️  Global labels applied to all metrics");
    println!("🔒 Security validation enabled");
    println!("⚡ Performance optimized for {} max metrics", 5000);
    
    // Verify production features
    assert!(output.contains("datacenter=\"us-west-2\""));
    assert!(output.contains("cluster=\"production\""));
    assert!(!output.contains("internal_debug")); // Should be excluded
    
    println!("✅ All production features verified");
    
    Ok(())
}

/// Demonstrate integration with performance monitoring
fn demonstrate_performance_integration() -> Result<(), CursedError> {
    println!("\n⚡ Performance Monitoring Integration");
    println!("-".repeat(40));
    
    // Create metrics manager with Prometheus export
    let metrics_config = MetricsConfig {
        enable_compilation_metrics: true,
        enable_runtime_metrics: true,
        enable_gc_metrics: true,
        enable_memory_metrics: true,
        enable_performance_hooks: true,
        enable_export: true,
        monitoring_interval_ms: 1000,
        export_format: cursed::codegen::llvm::performance_monitor::MetricsExportFormat::Prometheus,
        export_path: Some("cursed_metrics.prom".to_string()),
        ..Default::default()
    };
    
    let metrics_manager = MetricsManager::new(metrics_config)?;
    
    // Simulate metrics collection
    println!("🔄 Starting metrics collection...");
    metrics_manager.start()?;
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));
    
    // Get current metrics and export
    let current_metrics = metrics_manager.get_current_metrics();
    
    let config = PrometheusExporterConfig {
        namespace: "cursed_perf".to_string(),
        global_labels: {
            let mut labels = HashMap::new();
            labels.insert("monitoring_mode".to_string(), "performance".to_string());
            labels
        },
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    let output = exporter.export_metrics(&current_metrics)?;
    
    println!("✅ Performance metrics integration complete");
    println!("📊 Exported metrics from live system");
    println!("🔄 Metrics collection active");
    
    // Stop metrics collection
    metrics_manager.stop()?;
    println!("⏹️  Metrics collection stopped");
    
    Ok(())
}

/// Create sample metrics for demonstration
fn create_sample_metrics() -> AggregatedMetrics {
    AggregatedMetrics {
        timestamp: SystemTime::now(),
        compilation_metrics: None,
        runtime_metrics: None,
        performance_metrics: None,
        gc_metrics: Some(serde_json::json!({
            "total_collections": 50,
            "total_pause_time_ms": 750.0,
            "heap_size_bytes": 1048576
        })),
        memory_metrics: Some(serde_json::json!({
            "used_bytes": 512000,
            "total_bytes": 1024000,
            "allocations_count": 2500
        })),
        system_health: SystemHealthScore {
            overall_score: 88.0,
            compilation_score: 92.0,
            runtime_score: 85.0,
            memory_score: 87.0,
            gc_score: 90.0,
            performance_score: 86.0,
            status: HealthStatus::Good,
            recommendations: vec!["Consider optimizing memory allocation patterns".to_string()],
        },
        alerts: vec![
            MetricAlert {
                timestamp: SystemTime::now(),
                alert_type: AlertType::MemoryUsage,
                severity: AlertSeverity::Warning,
                metric_name: "heap_usage_percent".to_string(),
                current_value: 75.0,
                threshold: 70.0,
                message: "Memory usage is approaching threshold".to_string(),
                suggested_action: "Consider garbage collection or memory optimization".to_string(),
                acknowledged: false,
            }
        ],
    }
}

/// Create production-like metrics for demonstration
fn create_production_metrics() -> AggregatedMetrics {
    AggregatedMetrics {
        timestamp: SystemTime::now(),
        compilation_metrics: None,
        runtime_metrics: None,
        performance_metrics: None,
        gc_metrics: Some(serde_json::json!({
            "total_collections": 200,
            "total_pause_time_ms": 1200.0,
            "heap_size_bytes": 4194304,
            "gc_frequency_hz": 0.5
        })),
        memory_metrics: Some(serde_json::json!({
            "used_bytes": 3145728,
            "total_bytes": 8388608,
            "allocations_count": 10000,
            "peak_usage_bytes": 3500000
        })),
        system_health: SystemHealthScore {
            overall_score: 94.0,
            compilation_score: 96.0,
            runtime_score: 92.0,
            memory_score: 93.0,
            gc_score: 95.0,
            performance_score: 94.0,
            status: HealthStatus::Excellent,
            recommendations: vec![],
        },
        alerts: vec![],
    }
}
