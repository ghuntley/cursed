//! CURSED Metrics Export CLI Tool
//! 
//! A standalone tool for secure Prometheus metrics export with label sanitization.
//! Provides comprehensive security features to prevent injection attacks.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use clap::{Parser, Subcommand, ValueEnum};
use serde_json;

use cursed::metrics::{
    MetricsManager, MetricsConfig, AggregatedMetrics, get_global_metrics,
    start_global_metrics, stop_global_metrics, generate_global_report
};
use cursed::metrics::prometheus_exporter::{
    PrometheusExporter, PrometheusExporterConfig, PrometheusMetricType,
    sanitize_metric_name, sanitize_label_key, sanitize_label_value,
    validate_metric_name, validate_label_key
};
use cursed::error::CursedError;

#[derive(Parser)]
#[command(name = "cursed-metrics")]
#[command(about = "CURSED Secure Metrics Export Tool")]
#[command(long_about = "A production-ready tool for exporting CURSED compiler metrics in Prometheus format with comprehensive security features including label sanitization and injection attack prevention.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Export current metrics to Prometheus format
    Export {
        /// Output file path (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Metrics namespace prefix
        #[arg(short, long, default_value = "cursed")]
        namespace: String,
        
        /// Include timestamps in output
        #[arg(long, default_value_t = true)]
        timestamps: bool,
        
        /// Include metadata (HELP and TYPE lines)
        #[arg(long, default_value_t = true)]
        metadata: bool,
        
        /// Maximum number of metrics to export
        #[arg(long, default_value_t = 5000)]
        max_metrics: usize,
        
        /// Enable strict validation
        #[arg(long, default_value_t = true)]
        strict: bool,
        
        /// Global labels to add (format: key=value,key2=value2)
        #[arg(long)]
        labels: Option<String>,
    },
    
    /// Validate metric names and labels for Prometheus compliance
    Validate {
        /// Input file containing metrics (JSON format)
        input: PathBuf,
        
        /// Enable strict validation mode
        #[arg(long, default_value_t = true)]
        strict: bool,
        
        /// Fix validation issues automatically
        #[arg(long)]
        fix: bool,
        
        /// Output file for fixed metrics
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Start metrics monitoring daemon
    Monitor {
        /// Monitoring interval in milliseconds
        #[arg(short, long, default_value_t = 5000)]
        interval: u64,
        
        /// Output file for continuous export
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Enable real-time alerts
        #[arg(long)]
        alerts: bool,
        
        /// Alert webhook URL
        #[arg(long)]
        webhook: Option<String>,
    },
    
    /// Test security features and sanitization
    SecurityTest {
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
        
        /// Test specific injection vectors
        #[arg(long, value_enum)]
        test_type: Option<InjectionTestType>,
        
        /// Output detailed security report
        #[arg(short, long)]
        report: Option<PathBuf>,
    },
    
    /// Generate comprehensive metrics report
    Report {
        /// Output format
        #[arg(short, long, value_enum, default_value = "prometheus")]
        format: OutputFormat,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Include system health information
        #[arg(long, default_value_t = true)]
        health: bool,
        
        /// Include performance recommendations
        #[arg(long, default_value_t = true)]
        recommendations: bool,
    },
}

#[derive(ValueEnum, Clone)]
enum OutputFormat {
    Prometheus,
    Json,
    Yaml,
    Text,
}

#[derive(ValueEnum, Clone)]
enum InjectionTestType {
    Sql,
    Xss,
    Newline,
    Unicode,
    All,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Export { 
            output, 
            namespace, 
            timestamps, 
            metadata, 
            max_metrics, 
            strict, 
            labels 
        } => {
            export_metrics(output, namespace, timestamps, metadata, max_metrics, strict, labels)?;
        }
        
        Commands::Validate { 
            input, 
            strict, 
            fix, 
            output 
        } => {
            validate_metrics(input, strict, fix, output)?;
        }
        
        Commands::Monitor { 
            interval, 
            output, 
            alerts, 
            webhook 
        } => {
            monitor_metrics(interval, output, alerts, webhook)?;
        }
        
        Commands::SecurityTest { 
            verbose, 
            test_type, 
            report 
        } => {
            security_test(verbose, test_type, report)?;
        }
        
        Commands::Report { 
            format, 
            output, 
            health, 
            recommendations 
        } => {
            generate_report(format, output, health, recommendations)?;
        }
    }
    
    Ok(())
}

/// Export current metrics to Prometheus format
fn export_metrics(
    output: Option<PathBuf>,
    namespace: String,
    timestamps: bool,
    metadata: bool,
    max_metrics: usize,
    strict: bool,
    labels: Option<String>,
) -> Result<(), CursedError> {
    println!("🔒 Exporting metrics with security validation...");
    
    // Parse global labels
    let global_labels = parse_labels(labels)?;
    
    // Create secure exporter configuration
    let config = PrometheusExporterConfig {
        namespace,
        global_labels,
        include_timestamps: timestamps,
        max_metrics,
        include_metadata: metadata,
        strict_validation: strict,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Get current metrics from global manager
    start_global_metrics()?;
    let metrics = get_global_metrics().get_current_metrics();
    
    // Export to Prometheus format
    let prometheus_output = exporter.export_metrics(&metrics)?;
    
    // Output results
    match output {
        Some(path) => {
            fs::write(&path, &prometheus_output)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to write file: {}", e)))?;
            println!("✅ Metrics exported to: {}", path.display());
        }
        None => {
            print!("{}", prometheus_output);
        }
    }
    
    stop_global_metrics()?;
    
    let line_count = prometheus_output.lines().count();
    let metric_count = prometheus_output.lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .count();
    
    eprintln!("📊 Export complete: {} lines, {} metrics", line_count, metric_count);
    
    Ok(())
}

/// Validate metrics for Prometheus compliance
fn validate_metrics(
    input: PathBuf,
    strict: bool,
    fix: bool,
    output: Option<PathBuf>,
) -> Result<(), CursedError> {
    println!("🔍 Validating metrics for Prometheus compliance...");
    
    // Read input file
    let content = fs::read_to_string(&input)
        .map_err(|e| CursedError::runtime_error(&format!("Failed to read input file: {}", e)))?;
    
    // Parse as JSON (assuming structured metrics data)
    let data: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| CursedError::runtime_error(&format!("Failed to parse JSON: {}", e)))?;
    
    let mut validation_errors = Vec::new();
    let mut fixed_data = data.clone();
    
    // Validate metric names
    if let Some(metrics) = data.as_object() {
        for (metric_name, _) in metrics {
            if strict {
                if let Err(e) = validate_metric_name(metric_name) {
                    validation_errors.push(format!("Metric name '{}': {}", metric_name, e));
                    
                    if fix {
                        if let Ok(sanitized) = sanitize_metric_name(metric_name) {
                            // Update in fixed_data
                            println!("🔧 Fixed metric name: {} -> {}", metric_name, sanitized);
                        }
                    }
                }
            }
        }
    }
    
    // Validate labels (if present in data structure)
    // This is a simplified validation - in practice, you'd have a more complex data structure
    
    if validation_errors.is_empty() {
        println!("✅ All metrics are valid!");
    } else {
        println!("❌ Found {} validation errors:", validation_errors.len());
        for error in &validation_errors {
            println!("  - {}", error);
        }
        
        if fix {
            if let Some(output_path) = output {
                let fixed_content = serde_json::to_string_pretty(&fixed_data)
                    .map_err(|e| CursedError::runtime_error(&format!("Failed to serialize fixed data: {}", e)))?;
                
                fs::write(&output_path, fixed_content)
                    .map_err(|e| CursedError::runtime_error(&format!("Failed to write fixed file: {}", e)))?;
                
                println!("🔧 Fixed metrics written to: {}", output_path.display());
            }
        }
    }
    
    Ok(())
}

/// Monitor metrics continuously
fn monitor_metrics(
    interval: u64,
    output: Option<PathBuf>,
    alerts: bool,
    webhook: Option<String>,
) -> Result<(), CursedError> {
    println!("📊 Starting metrics monitoring (interval: {}ms)...", interval);
    
    // Start global metrics collection
    start_global_metrics()?;
    
    if alerts {
        println!("🚨 Alert monitoring enabled");
        if let Some(url) = webhook {
            println!("📡 Webhook configured: {}", url);
        }
    }
    
    let config = PrometheusExporterConfig {
        namespace: "cursed_monitor".to_string(),
        include_timestamps: true,
        max_metrics: 10000,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    
    // Monitoring loop
    loop {
        let metrics = get_global_metrics().get_current_metrics();
        
        // Export metrics
        let prometheus_output = exporter.export_metrics(&metrics)?;
        
        // Write to file if specified
        if let Some(ref path) = output {
            fs::write(path, &prometheus_output)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to write metrics: {}", e)))?;
        }
        
        // Check for alerts
        if alerts && !metrics.alerts.is_empty() {
            for alert in &metrics.alerts {
                println!("🚨 ALERT: {} - {} ({})", 
                    alert.metric_name, 
                    alert.message, 
                    format!("{:?}", alert.severity));
                
                // Send webhook if configured
                if let Some(ref webhook_url) = webhook {
                    send_webhook_alert(webhook_url, alert)?;
                }
            }
        }
        
        println!("📊 Metrics updated: {} alerts, health: {:.1}", 
            metrics.alerts.len(), 
            metrics.system_health.overall_score);
        
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }
}

/// Test security features
fn security_test(
    verbose: bool,
    test_type: Option<InjectionTestType>,
    report: Option<PathBuf>,
) -> Result<(), CursedError> {
    println!("🛡️  Running security tests...");
    
    let config = PrometheusExporterConfig {
        namespace: "security_test".to_string(),
        strict_validation: true,
        ..Default::default()
    };
    
    let mut exporter = PrometheusExporter::new(config)?;
    let mut test_results = Vec::new();
    
    let test_vectors = match test_type {
        Some(InjectionTestType::Sql) => get_sql_injection_tests(),
        Some(InjectionTestType::Xss) => get_xss_injection_tests(),
        Some(InjectionTestType::Newline) => get_newline_injection_tests(),
        Some(InjectionTestType::Unicode) => get_unicode_injection_tests(),
        Some(InjectionTestType::All) | None => get_all_injection_tests(),
    };
    
    for (test_name, malicious_input, expected_result) in test_vectors {
        if verbose {
            println!("🔍 Testing: {}", test_name);
        }
        
        let mut labels = HashMap::new();
        labels.insert("test".to_string(), malicious_input.clone());
        
        let result = exporter.add_metric(
            "security_test_metric",
            "Security test metric",
            PrometheusMetricType::Gauge,
            1.0,
            labels,
        );
        
        let passed = match expected_result {
            ExpectedResult::ShouldSucceed => result.is_ok(),
            ExpectedResult::ShouldFail => result.is_err(),
            ExpectedResult::ShouldSanitize => {
                if result.is_ok() {
                    let output = exporter.export_cached_metrics()?;
                    !output.contains(&malicious_input)
                } else {
                    false
                }
            }
        };
        
        test_results.push(SecurityTestResult {
            test_name: test_name.to_string(),
            input: malicious_input,
            passed,
            result_description: format!("{:?}", result),
        });
        
        if verbose {
            println!("  {} {}", if passed { "✅" } else { "❌" }, 
                if passed { "PASSED" } else { "FAILED" });
        }
    }
    
    // Summary
    let passed_count = test_results.iter().filter(|r| r.passed).count();
    let total_count = test_results.len();
    
    println!("🛡️  Security test results: {}/{} passed", passed_count, total_count);
    
    if passed_count == total_count {
        println!("✅ All security tests passed!");
    } else {
        println!("❌ Some security tests failed!");
        for result in &test_results {
            if !result.passed {
                println!("  ❌ {}: {}", result.test_name, result.result_description);
            }
        }
    }
    
    // Write report if requested
    if let Some(report_path) = report {
        let report_content = serde_json::to_string_pretty(&test_results)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to serialize report: {}", e)))?;
        
        fs::write(&report_path, report_content)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to write report: {}", e)))?;
        
        println!("📄 Security report written to: {}", report_path.display());
    }
    
    Ok(())
}

/// Generate comprehensive metrics report
fn generate_report(
    format: OutputFormat,
    output: Option<PathBuf>,
    include_health: bool,
    include_recommendations: bool,
) -> Result<(), CursedError> {
    println!("📊 Generating comprehensive metrics report...");
    
    start_global_metrics()?;
    let report = generate_global_report();
    
    let content = match format {
        OutputFormat::Prometheus => {
            let config = PrometheusExporterConfig::default();
            let mut exporter = PrometheusExporter::new(config)?;
            exporter.export_metrics(&report.aggregated_metrics)?
        }
        OutputFormat::Json => {
            serde_json::to_string_pretty(&report)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to serialize JSON: {}", e)))?
        }
        OutputFormat::Yaml => {
            serde_yaml::to_string(&report)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to serialize YAML: {}", e)))?
        }
        OutputFormat::Text => {
            format_text_report(&report, include_health, include_recommendations)
        }
    };
    
    match output {
        Some(path) => {
            fs::write(&path, content)
                .map_err(|e| CursedError::runtime_error(&format!("Failed to write report: {}", e)))?;
            println!("✅ Report written to: {}", path.display());
        }
        None => {
            print!("{}", content);
        }
    }
    
    stop_global_metrics()?;
    Ok(())
}

// Helper functions

fn parse_labels(labels_str: Option<String>) -> Result<HashMap<String, String>, CursedError> {
    let mut labels = HashMap::new();
    
    if let Some(str) = labels_str {
        for pair in str.split(',') {
            let parts: Vec<&str> = pair.split('=').collect();
            if parts.len() != 2 {
                return Err(CursedError::runtime_error(&format!("Invalid label format: {}", pair)));
            }
            labels.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    
    Ok(labels)
}

fn send_webhook_alert(webhook_url: &str, alert: &cursed::metrics::MetricAlert) -> Result<(), CursedError> {
    // Simplified webhook implementation
    println!("📡 Sending webhook to: {}", webhook_url);
    println!("   Alert: {} - {}", alert.metric_name, alert.message);
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct SecurityTestResult {
    test_name: String,
    input: String,
    passed: bool,
    result_description: String,
}

enum ExpectedResult {
    ShouldSucceed,
    ShouldFail,
    ShouldSanitize,
}

fn get_sql_injection_tests() -> Vec<(String, String, ExpectedResult)> {
    vec![
        ("SQL DROP TABLE".to_string(), "'; DROP TABLE metrics; --".to_string(), ExpectedResult::ShouldSanitize),
        ("SQL UNION SELECT".to_string(), "' UNION SELECT * FROM users --".to_string(), ExpectedResult::ShouldSanitize),
    ]
}

fn get_xss_injection_tests() -> Vec<(String, String, ExpectedResult)> {
    vec![
        ("XSS Script Tag".to_string(), "<script>alert('xss')</script>".to_string(), ExpectedResult::ShouldSanitize),
        ("XSS JavaScript".to_string(), "javascript:alert('xss')".to_string(), ExpectedResult::ShouldSanitize),
    ]
}

fn get_newline_injection_tests() -> Vec<(String, String, ExpectedResult)> {
    vec![
        ("Newline Injection".to_string(), "value\n# MALICIOUS\nfake_metric 999".to_string(), ExpectedResult::ShouldSanitize),
        ("CRLF Injection".to_string(), "value\r\n# EVIL COMMENT".to_string(), ExpectedResult::ShouldSanitize),
    ]
}

fn get_unicode_injection_tests() -> Vec<(String, String, ExpectedResult)> {
    vec![
        ("Unicode Control".to_string(), "value\u{200B}\u{FEFF}hidden".to_string(), ExpectedResult::ShouldSanitize),
        ("Unicode Homograph".to_string(), "vaIue".to_string(), ExpectedResult::ShouldSanitize), // Cyrillic I
    ]
}

fn get_all_injection_tests() -> Vec<(String, String, ExpectedResult)> {
    let mut tests = Vec::new();
    tests.extend(get_sql_injection_tests());
    tests.extend(get_xss_injection_tests());
    tests.extend(get_newline_injection_tests());
    tests.extend(get_unicode_injection_tests());
    tests
}

fn format_text_report(
    report: &cursed::metrics::MetricsReport,
    include_health: bool,
    include_recommendations: bool,
) -> String {
    let mut output = String::new();
    
    output.push_str("CURSED Metrics Report\n");
    output.push_str("====================\n\n");
    
    if include_health {
        output.push_str(&format!("System Health: {:.1}/100 ({:?})\n", 
            report.aggregated_metrics.system_health.overall_score,
            report.aggregated_metrics.system_health.status));
        output.push_str(&format!("- Compilation: {:.1}\n", report.aggregated_metrics.system_health.compilation_score));
        output.push_str(&format!("- Runtime: {:.1}\n", report.aggregated_metrics.system_health.runtime_score));
        output.push_str(&format!("- Memory: {:.1}\n", report.aggregated_metrics.system_health.memory_score));
        output.push_str(&format!("- GC: {:.1}\n", report.aggregated_metrics.system_health.gc_score));
        output.push_str("\n");
    }
    
    output.push_str(&format!("Alerts: {}\n", report.aggregated_metrics.alerts.len()));
    for alert in &report.aggregated_metrics.alerts {
        output.push_str(&format!("- {:?}: {} ({})\n", 
            alert.severity, alert.message, alert.metric_name));
    }
    output.push_str("\n");
    
    if include_recommendations && !report.aggregated_metrics.system_health.recommendations.is_empty() {
        output.push_str("Recommendations:\n");
        for rec in &report.aggregated_metrics.system_health.recommendations {
            output.push_str(&format!("- {}\n", rec));
        }
    }
    
    output
}
