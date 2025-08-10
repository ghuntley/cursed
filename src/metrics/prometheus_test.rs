//! Comprehensive test suite for Prometheus metrics exporter with security validation

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::metrics::prometheus_exporter::*;
use crate::metrics::{AggregatedMetrics, SystemHealthScore, HealthStatus, MetricAlert, AlertSeverity, AlertType};
use crate::error::CursedError;

/// Test metrics export with label injection attempts
#[test]
fn test_label_injection_prevention() {
    let mut labels = HashMap::new();
    labels.insert("normal_label".to_string(), "normal_value".to_string());
    labels.insert("injection\"attempt".to_string(), "value\"with\\quotes\nand\nlines".to_string());
    labels.insert("__reserved_label".to_string(), "should_be_rejected".to_string());
    labels.insert("123invalid_start".to_string(), "value".to_string());
    
    let sanitized = sanitize_labels(labels).unwrap();
    
    // Check that reserved labels are rejected during key sanitization
    assert!(!sanitized.contains_key("__reserved_label"));
    
    // Check that invalid starting characters are fixed
    assert!(sanitized.contains_key("_23invalid_start"));
    
    // Check that injection characters are properly sanitized in keys
    assert!(sanitized.contains_key("injection_attempt"));
    
    // Check that values are sanitized
    let value = sanitized.get("injection_attempt").unwrap();
    assert!(!value.contains('"'));
    assert!(!value.contains('\n'));
}

/// Test metric name sanitization
#[test]
fn test_metric_name_sanitization() {
    let test_cases = vec![
        ("valid_metric", "valid_metric"),
        ("metric-with-dashes", "metric_with_dashes"),
        ("123starts_with_number", "_23starts_with_number"),
        ("metric.with.dots", "metric_with_dots"),
        ("metric@with#special$chars", "metric_with_special_chars"),
        ("metric:with:colons", "metric:with:colons"), // Colons are valid
    ];
    
    for (input, expected) in test_cases {
        let result = sanitize_metric_name(input).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

/// Test label value escaping for Prometheus format
#[test]
fn test_label_value_escaping() {
    let test_cases = vec![
        ("normal value", "normal value"),
        ("value with \"quotes\"", "value with \\\"quotes\\\""),
        ("value\nwith\nlines", "value\\nwith\\nlines"),
        ("value\\with\\backslashes", "value\\\\with\\\\backslashes"),
        ("value\twith\ttabs", "value\\twith\\ttabs"),
        ("value\rwith\rcarriage", "value\\rwith\\rcarriage"),
    ];
    
    for (input, expected) in test_cases {
        let result = escape_label_value(input);
        assert_eq!(result, expected, "Failed for input: {:?}", input);
    }
}

/// Test Prometheus metric format generation
#[test]
fn test_prometheus_metric_format() {
    let mut labels = HashMap::new();
    labels.insert("instance".to_string(), "test_instance".to_string());
    labels.insert("severity".to_string(), "critical".to_string());
    
    let metric = PrometheusMetric::new(
        "test_metric",
        "Test metric description",
        PrometheusMetricType::Gauge,
        42.5,
        labels,
    ).unwrap().with_timestamp(1609459200000); // Jan 1, 2021
    
    let line = metric.to_prometheus_line();
    
    // Verify the format
    assert!(line.contains("test_metric{"));
    assert!(line.contains("instance=\"test_instance\""));
    assert!(line.contains("severity=\"critical\""));
    assert!(line.contains("} 42.5 1609459200000"));
}

/// Test full metrics export with security considerations
#[test]
fn test_full_metrics_export() {
    let config = PrometheusExporterConfig {
        namespace: "test".to_string(),
        global_labels: {
            let mut labels = HashMap::new();
            labels.insert("environment".to_string(), "testing".to_string());
            labels
        },
        include_timestamps: true,
        max_metrics: 1000,
        include_metadata: true,
        strict_validation: true,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config).unwrap();
    
    // Create test metrics with potential security issues
    let metrics = create_test_metrics_with_security_issues();
    
    let result = exporter.export_metrics(&metrics).unwrap();
    
    // Verify security measures
    assert!(!result.contains("__reserved"));  // Reserved labels should be filtered
    assert!(!result.contains("\"malicious"));  // Injection attempts should be escaped
    assert!(!result.contains("\n# MALICIOUS"));  // Newline injection should be prevented
    assert!(result.contains("# HELP"));  // Metadata should be included
    assert!(result.contains("# TYPE"));  // Type information should be included
    
    // Verify metric limits are respected
    let metric_count = result.lines().filter(|line| !line.starts_with('#') && !line.trim().is_empty()).count();
    assert!(metric_count <= 1000, "Too many metrics exported: {}", metric_count);
}

/// Test special float value handling
#[test]
fn test_special_float_values() {
    assert_eq!(format_metric_value(f64::NAN), "NaN");
    assert_eq!(format_metric_value(f64::INFINITY), "+Inf");
    assert_eq!(format_metric_value(f64::NEG_INFINITY), "-Inf");
    assert_eq!(format_metric_value(42.0), "42");
    assert_eq!(format_metric_value(-42.5), "-42.5");
}

/// Test strict validation mode
#[test]
fn test_strict_validation() {
    // Test metric name validation
    assert!(validate_metric_name("valid_metric").is_ok());
    assert!(validate_metric_name("").is_err());
    assert!(validate_metric_name("123invalid").is_err());
    assert!(validate_metric_name("a".repeat(201).as_str()).is_err());
    
    // Test label key validation
    assert!(validate_label_key("valid_label").is_ok());
    assert!(validate_label_key("").is_err());
    assert!(validate_label_key("__reserved").is_err());
    assert!(validate_label_key("123invalid").is_err());
    assert!(validate_label_key("a".repeat(101).as_str()).is_err());
}

/// Test metric deduplication and grouping
#[test]
fn test_metric_grouping() {
    let config = PrometheusExporterConfig::default();
    let mut exporter = PrometheusExporter::new(config).unwrap();
    
    // Add multiple metrics with same name but different labels
    let mut labels1 = HashMap::new();
    labels1.insert("instance".to_string(), "server1".to_string());
    
    let mut labels2 = HashMap::new();
    labels2.insert("instance".to_string(), "server2".to_string());
    
    exporter.add_metric(
        "cpu_usage",
        "CPU usage percentage",
        PrometheusMetricType::Gauge,
        75.0,
        labels1,
    ).unwrap();
    
    exporter.add_metric(
        "cpu_usage",
        "CPU usage percentage",
        PrometheusMetricType::Gauge,
        80.0,
        labels2,
    ).unwrap();
    
    let result = exporter.export_cached_metrics().unwrap();
    
    // Should have only one HELP and TYPE line per metric name
    let help_count = result.lines().filter(|line| line.contains("# HELP cpu_usage")).count();
    let type_count = result.lines().filter(|line| line.contains("# TYPE cpu_usage")).count();
    
    assert_eq!(help_count, 1, "Should have exactly one HELP line");
    assert_eq!(type_count, 1, "Should have exactly one TYPE line");
    
    // Should have two data lines
    let data_count = result.lines().filter(|line| line.starts_with("cursed_cpu_usage{")).count();
    assert_eq!(data_count, 2, "Should have two data lines");
}

/// Test performance with large number of metrics
#[test]
fn test_performance_with_large_metrics() {
    let config = PrometheusExporterConfig {
        max_metrics: 5000,
        ..Default::default()
    };
    let mut exporter = PrometheusExporter::new(config).unwrap();
    
    // Generate large number of metrics
    for i in 0..10000 {
        let mut labels = HashMap::new();
        labels.insert("instance".to_string(), format!("server_{}", i % 100));
        labels.insert("shard".to_string(), format!("shard_{}", i % 10));
        
        exporter.add_metric(
            &format!("metric_{}", i % 50),
            "Test metric",
            PrometheusMetricType::Counter,
            i as f64,
            labels,
        ).unwrap();
    }
    
    let result = exporter.export_cached_metrics().unwrap();
    
    // Should respect max_metrics limit
    let data_lines = result.lines().filter(|line| !line.starts_with('#') && !line.trim().is_empty()).count();
    assert!(data_lines <= 5000, "Exported too many metrics: {}", data_lines);
}

/// Helper function to create test metrics with security issues
fn create_test_metrics_with_security_issues() -> AggregatedMetrics {
    let alerts = vec![
        MetricAlert {
            timestamp: SystemTime::now(),
            alert_type: AlertType::MemoryUsage,
            severity: AlertSeverity::Critical,
            metric_name: "memory\"injection\nattack".to_string(),
            current_value: 95.0,
            threshold: 90.0,
            message: "High memory usage\n# MALICIOUS INJECTION".to_string(),
            suggested_action: "Restart service".to_string(),
            acknowledged: false,
        },
    ];
    
    AggregatedMetrics {
        timestamp: SystemTime::now(),
        compilation_metrics: None,
        runtime_metrics: None,
        performance_metrics: None,
        gc_metrics: Some(serde_json::json!({
            "total_collections": 100,
            "total_pause_time_ms": 1500.0,
            "heap_size_bytes": 1048576
        })),
        memory_metrics: Some(serde_json::json!({
            "used_bytes": 1024000,
            "total_bytes": 2048000,
            "allocations_count": 5000
        })),
        system_health: SystemHealthScore {
            overall_score: 85.0,
            compilation_score: 90.0,
            runtime_score: 80.0,
            memory_score: 85.0,
            gc_score: 88.0,
            performance_score: 82.0,
            status: HealthStatus::Good,
            recommendations: vec!["Optimize memory usage".to_string()],
        },
        alerts,
    }
}

/// Test configuration validation
#[test]
fn test_config_validation() {
    // Test empty namespace
    let config = PrometheusExporterConfig {
        namespace: "".to_string(),
        ..Default::default()
    };
    assert!(PrometheusExporter::new(config).is_err());
    
    // Test valid configuration
    let config = PrometheusExporterConfig {
        namespace: "valid_namespace".to_string(),
        ..Default::default()
    };
    assert!(PrometheusExporter::new(config).is_ok());
}

/// Integration test with realistic scenarios
#[test]
fn test_realistic_integration() {
    let config = PrometheusExporterConfig {
        namespace: "cursed_compiler".to_string(),
        global_labels: {
            let mut labels = HashMap::new();
            labels.insert("version".to_string(), "1.0.0".to_string());
            labels.insert("environment".to_string(), "production".to_string());
            labels
        },
        include_timestamps: true,
        max_metrics: 1000,
        include_metadata: true,
        strict_validation: true,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config).unwrap();
    let metrics = create_realistic_test_metrics();
    
    let result = exporter.export_metrics(&metrics).unwrap();
    
    // Verify key components are present
    assert!(result.contains("cursed_compiler_system_health_score"));
    assert!(result.contains("cursed_compiler_alerts_total"));
    assert!(result.contains("cursed_compiler_gc_collections_total"));
    assert!(result.contains("cursed_compiler_memory_used_bytes"));
    
    // Verify global labels are applied
    assert!(result.contains("version=\"1.0.0\""));
    assert!(result.contains("environment=\"production\""));
    
    // Verify format compliance
    for line in result.lines() {
        if line.starts_with('#') {
            // Metadata lines
            assert!(line.starts_with("# HELP") || line.starts_with("# TYPE"));
        } else if !line.trim().is_empty() {
            // Data lines should have proper format
            assert!(line.contains(' '));  // Must have space between metric and value
        }
    }
}

fn create_realistic_test_metrics() -> AggregatedMetrics {
    AggregatedMetrics {
        timestamp: SystemTime::now(),
        compilation_metrics: None,
        runtime_metrics: None,
        performance_metrics: None,
        gc_metrics: Some(serde_json::json!({
            "total_collections": 150,
            "total_pause_time_ms": 2250.0,
            "heap_size_bytes": 2097152
        })),
        memory_metrics: Some(serde_json::json!({
            "used_bytes": 1572864,
            "total_bytes": 4194304,
            "allocations_count": 7500
        })),
        system_health: SystemHealthScore {
            overall_score: 92.0,
            compilation_score: 95.0,
            runtime_score: 88.0,
            memory_score: 90.0,
            gc_score: 94.0,
            performance_score: 89.0,
            status: HealthStatus::Excellent,
            recommendations: vec![],
        },
        alerts: vec![],
    }
}
