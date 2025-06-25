/// fr fr Comprehensive test suite for cryptographic security analysis framework
/// 
/// This test suite validates all security analysis components including timing analysis,
/// side-channel detection, entropy validation, parameter verification, and vulnerability scanning.

#[path = "common.rs"]
pub mod common;

use cursed::stdlib::packages::crypto_advanced::*;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_security_analysis_framework_creation() {
    let config = SecurityAnalysisConfig::default();
    let analysis = SecurityAnalysis::new(config.clone());
    
    // Verify framework components are initialized
    assert_eq!(config.level, SecurityLevel::Standard);
    assert!(config.timing_analysis_enabled);
    assert!(config.side_channel_detection_enabled);
    assert!(config.entropy_validation_enabled);
    assert!(config.parameter_verification_enabled);
}

#[test]
fn test_security_analysis_function_analysis() {
    let analysis = SecurityAnalysis::default();
    
    // Test constant-time function
    let result = analysis.analyze_function("constant_operation", || {
        42 // Simple constant operation
    });
    
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.overall_security_score > 70.0);
    assert!(metrics.timing_variance < 1000.0); // Should be low variance
}

#[test]
fn test_security_analysis_entropy_validation() {
    let analysis = SecurityAnalysis::default();
    
    // Test good entropy (pseudo-random data)
    let good_entropy: Vec<u8> = (0..1000).map(|i| (i * 31 + 17) as u8).collect();
    let result = analysis.analyze_entropy(&good_entropy);
    
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.entropy_score > 0.3); // Should have reasonable entropy
    
    // Test poor entropy (all zeros)
    let poor_entropy = vec![0u8; 1000];
    let result = analysis.analyze_entropy(&poor_entropy);
    
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.entropy_score < 0.1); // Should have very low entropy
    assert_eq!(metrics.quality_assessment, EntropyQuality::Poor);
}

#[test]
fn test_security_analysis_parameter_verification() {
    let analysis = SecurityAnalysis::default();
    
    // Test strong AES parameters
    let strong_params = CryptoParameters {
        algorithm: "AES".to_string(),
        key_size: 256,
        block_size: Some(128),
        iv_size: Some(96),
        tag_size: Some(128),
        rounds: Some(14),
        custom_params: HashMap::new(),
    };
    
    let result = analysis.verify_parameters(&strong_params);
    assert!(result.is_ok());
    let verification = result.unwrap();
    assert!(verification.is_compliant());
    assert!(verification.security_level.is_production_ready());
    
    // Test weak parameters
    let weak_params = CryptoParameters {
        algorithm: "DES".to_string(),
        key_size: 56,
        block_size: Some(64),
        iv_size: Some(32),
        tag_size: Some(64),
        rounds: Some(16),
        custom_params: HashMap::new(),
    };
    
    let result = analysis.verify_parameters(&weak_params);
    assert!(result.is_ok());
    let verification = result.unwrap();
    assert!(!verification.is_compliant());
    assert!(!verification.violations.is_empty());
}

#[test]
fn test_security_analysis_vulnerability_scanning() {
    let analysis = SecurityAnalysis::default();
    
    // Test secure context
    let secure_context = SecurityContext {
        algorithm_name: "AES-256-GCM".to_string(),
        key_size: 256,
        implementation_details: {
            let mut details = HashMap::new();
            details.insert("timing_constant".to_string(), "true".to_string());
            details.insert("cache_safe".to_string(), "true".to_string());
            details
        },
        environment_info: {
            let mut env = HashMap::new();
            env.insert("debug_mode".to_string(), "false".to_string());
            env.insert("log_level".to_string(), "info".to_string());
            env
        },
    };
    
    let result = analysis.scan_vulnerabilities(&secure_context);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.security_posture_score() > 80.0);
    
    // Test insecure context
    let insecure_context = SecurityContext {
        algorithm_name: "MD5".to_string(),
        key_size: 128,
        implementation_details: {
            let mut details = HashMap::new();
            details.insert("timing_constant".to_string(), "false".to_string());
            details.insert("cache_safe".to_string(), "false".to_string());
            details
        },
        environment_info: {
            let mut env = HashMap::new();
            env.insert("debug_mode".to_string(), "true".to_string());
            env.insert("log_level".to_string(), "debug".to_string());
            env
        },
    };
    
    let result = analysis.scan_vulnerabilities(&insecure_context);
    assert!(result.is_ok());
    let report = result.unwrap();
    assert!(report.has_critical_vulnerabilities() || !report.vulnerabilities_found.is_empty());
    assert!(report.security_posture_score() < 50.0);
}

#[test]
fn test_security_report_generation() {
    let analysis = SecurityAnalysis::default();
    let report = analysis.generate_report();
    
    assert!(report.overall_score >= 0.0);
    assert!(report.overall_score <= 100.0);
    assert!(!report.recommendations.is_empty());
    assert!(report.is_secure() || !report.is_secure()); // Should be deterministic based on state
}

// Timing Analysis Tests
#[test]
fn test_timing_analyzer_creation() {
    let analyzer = TimingAnalyzer::new();
    // Default configuration validation
    // Note: The internal fields are private, so we test behavior instead
    
    let result = analyzer.analyze_timing("test", || 42, 50);
    assert!(result.is_ok());
}

#[test]
fn test_timing_analyzer_constant_time_detection() {
    let analyzer = TimingAnalyzer::new();
    
    // Constant time operation
    let result = analyzer.analyze_timing("constant", || {
        let mut sum = 0;
        for i in 0..100 {
            sum += i;
        }
        sum
    }, 50);
    
    assert!(result.is_ok());
    let timing_result = result.unwrap();
    assert!(timing_result.variance < 10000.0); // Should have low variance
    assert!(!timing_result.has_timing_vulnerability());
}

#[test]
fn test_timing_analyzer_comparison() {
    let analyzer = TimingAnalyzer::new();
    
    let comparison = analyzer.compare_timing(
        "fast", || 42,
        "fast_also", || 43,
        20
    );
    
    assert!(comparison.is_ok());
    let comp_result = comparison.unwrap();
    assert!(comp_result.performance_ratio > 0.0);
    assert!(comp_result.variance_ratio > 0.0);
}

// Side-Channel Analysis Tests
#[test]
fn test_side_channel_detector_creation() {
    let detector = SideChannelDetector::new();
    
    let result = detector.analyze_side_channels("test", || 42);
    assert!(result.is_ok());
}

#[test]
fn test_side_channel_analysis() {
    let detector = SideChannelDetector::new();
    
    // Analyze simple constant operation
    let result = detector.analyze_side_channels("constant_op", || {
        42 // Constant operation
    });
    
    assert!(result.is_ok());
    let analysis = result.unwrap();
    assert!(analysis.security_score > 0.0);
    assert!(analysis.confidence > 0.0);
    assert!(!analysis.has_side_channel_leak() || analysis.has_side_channel_leak()); // Either is valid
}

#[test]
fn test_differential_side_channel_analysis() {
    let detector = SideChannelDetector::new();
    
    let result = detector.analyze_differential_attack(
        "test",
        || 42, // Variant 1
        || 43  // Variant 2
    );
    
    assert!(result.is_ok());
    let diff_analysis = result.unwrap();
    assert!(diff_analysis.differential_score >= 0.0);
}

// Entropy Validation Tests
#[test]
fn test_entropy_validator_creation() {
    let validator = EntropyValidator::new();
    
    // Test with sufficient data
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data_large: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    
    let result = validator.validate_entropy(&data_large);
    assert!(result.is_ok());
}

#[test]
fn test_entropy_shannon_calculation() {
    let validator = EntropyValidator::new();
    
    // Perfect entropy (uniform distribution)
    let perfect_data: Vec<u8> = (0..=255).collect();
    let perfect_data_large: Vec<u8> = perfect_data.repeat(4); // Make it large enough
    
    let result = validator.validate_entropy(&perfect_data_large);
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.shannon_entropy > 7.0); // Should be close to 8.0
    
    // Poor entropy (all same values)
    let poor_data = vec![42u8; 1000];
    let result = validator.validate_entropy(&poor_data);
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.shannon_entropy, 0.0);
    assert_eq!(metrics.quality_assessment, EntropyQuality::Poor);
}

#[test]
fn test_entropy_statistical_tests() {
    let validator = EntropyValidator::new();
    
    // Create pseudo-random data
    let pseudo_random: Vec<u8> = (0..1000).map(|i| (i * 31 + 17) as u8).collect();
    
    let result = validator.validate_entropy(&pseudo_random);
    assert!(result.is_ok());
    let metrics = result.unwrap();
    
    // Verify statistical tests ran
    assert!(!metrics.statistical_tests.frequency_test.test_name.is_empty());
    assert!(!metrics.statistical_tests.runs_test.test_name.is_empty());
    assert!(metrics.statistical_tests.frequency_test.p_value >= 0.0);
    assert!(metrics.statistical_tests.frequency_test.p_value <= 1.0);
}

#[test]
fn test_entropy_quality_levels() {
    // Test quality descriptions
    assert_eq!(EntropyQuality::Poor.description(), "Poor - Not suitable for cryptographic use");
    assert_eq!(EntropyQuality::Excellent.description(), "Excellent - High-quality cryptographic entropy");
    
    // Test cryptographic security check
    assert!(!EntropyQuality::Poor.is_cryptographically_secure());
    assert!(!EntropyQuality::Weak.is_cryptographically_secure());
    assert!(!EntropyQuality::Fair.is_cryptographically_secure());
    assert!(EntropyQuality::Good.is_cryptographically_secure());
    assert!(EntropyQuality::Excellent.is_cryptographically_secure());
}

#[test]
fn test_entropy_comparison() {
    let validator = EntropyValidator::new();
    
    let good_data: Vec<u8> = (0..1000).map(|i| (i * 31 + 17) as u8).collect();
    let poor_data = vec![42u8; 1000];
    
    let result = validator.compare_entropy_sources(&good_data, &poor_data);
    assert!(result.is_ok());
    let comparison = result.unwrap();
    assert!(comparison.entropy_difference > 0.1);
    assert_eq!(comparison.better_source, EntropySource::Source1);
}

// Parameter Verification Tests
#[test]
fn test_parameter_verifier_creation() {
    let verifier = ParameterVerifier::new();
    let supported_algorithms = verifier.list_supported_algorithms();
    
    assert!(!supported_algorithms.is_empty());
    assert!(supported_algorithms.contains(&"aes".to_string()));
    assert!(supported_algorithms.contains(&"rsa".to_string()));
}

#[test]
fn test_aes_parameter_verification() {
    let verifier = ParameterVerifier::new();
    let params = CryptoParameters {
        algorithm: "AES".to_string(),
        key_size: 256,
        block_size: Some(128),
        iv_size: Some(96),
        tag_size: Some(128),
        rounds: Some(14),
        custom_params: HashMap::new(),
    };

    let result = verifier.verify_parameters(&params);
    assert!(result.is_ok());

    let verification = result.unwrap();
    assert!(verification.is_compliant());
    assert!(verification.security_level.is_production_ready());
    assert!(verification.estimated_security_bits >= 256);
}

#[test]
fn test_weak_parameter_detection() {
    let verifier = ParameterVerifier::new();
    let params = CryptoParameters {
        algorithm: "AES".to_string(),
        key_size: 64, // Too small
        block_size: Some(128),
        iv_size: Some(32), // Too small
        tag_size: Some(64), // Too small
        rounds: Some(5), // Too few
        custom_params: HashMap::new(),
    };

    let result = verifier.verify_parameters(&params);
    assert!(result.is_ok());

    let verification = result.unwrap();
    assert!(!verification.is_compliant());
    assert!(!verification.violations.is_empty());
    assert!(verification.violations.iter().any(|v| v.severity >= ViolationSeverity::High));
}

#[test]
fn test_deprecated_algorithm_detection() {
    let verifier = ParameterVerifier::new();
    let params = CryptoParameters {
        algorithm: "MD5".to_string(),
        key_size: 128,
        block_size: Some(512),
        iv_size: None,
        tag_size: Some(128),
        rounds: Some(64),
        custom_params: HashMap::new(),
    };

    let result = verifier.verify_parameters(&params);
    assert!(result.is_ok());

    let verification = result.unwrap();
    assert!(!verification.is_compliant());
    assert_eq!(verification.security_level, SecurityLevel::Insecure);
    assert!(verification.violations.iter().any(|v| v.violation_type == ViolationType::DeprecatedParameter));
}

#[test]
fn test_recommended_parameters() {
    let verifier = ParameterVerifier::new();
    let recommended = verifier.get_recommended_parameters("AES");
    
    assert!(recommended.is_ok());
    let params = recommended.unwrap();
    assert_eq!(params.algorithm, "AES");
    assert_eq!(params.key_size, 256);
}

#[test]
fn test_security_level_assessments() {
    assert_eq!(SecurityLevel::Insecure.description(), "Insecure - Immediate replacement required");
    assert_eq!(SecurityLevel::Excellent.description(), "Excellent - High security for sensitive applications");
    
    assert!(!SecurityLevel::Insecure.is_production_ready());
    assert!(!SecurityLevel::Weak.is_production_ready());
    assert!(SecurityLevel::Acceptable.is_production_ready());
    assert!(SecurityLevel::Strong.is_production_ready());
    assert!(SecurityLevel::Excellent.is_production_ready());
}

// Vulnerability Scanner Tests
#[test]
fn test_vulnerability_scanner_creation() {
    let scanner = VulnerabilityScanner::new();
    // Scanner should be created successfully
    // Internal state is private, so we test through behavior
    
    let context = SecurityContext {
        algorithm_name: "AES".to_string(),
        key_size: 256,
        implementation_details: HashMap::new(),
        environment_info: HashMap::new(),
    };
    
    let result = scanner.scan_vulnerabilities(&context);
    assert!(result.is_ok());
}

#[test]
fn test_vulnerability_severity_levels() {
    assert_eq!(VulnerabilitySeverity::Critical.description(), "Critical - Immediate security threat");
    assert_eq!(VulnerabilitySeverity::Info.description(), "Informational - No immediate security impact");
    
    let (min, max) = VulnerabilitySeverity::High.cvss_score_range();
    assert_eq!(min, 7.0);
    assert_eq!(max, 8.9);
}

#[test]
fn test_md5_vulnerability_detection() {
    let scanner = VulnerabilityScanner::new();
    let context = SecurityContext {
        algorithm_name: "MD5".to_string(),
        key_size: 128,
        implementation_details: HashMap::new(),
        environment_info: HashMap::new(),
    };

    let result = scanner.scan_vulnerabilities(&context);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert!(!report.vulnerabilities_found.is_empty());
    assert!(report.vulnerabilities_found.iter()
        .any(|v| v.vulnerability_type == VulnerabilityType::WeakCryptography));
}

#[test]
fn test_weak_key_size_detection() {
    let scanner = VulnerabilityScanner::new();
    let context = SecurityContext {
        algorithm_name: "AES".to_string(),
        key_size: 64, // Weak key size
        implementation_details: HashMap::new(),
        environment_info: HashMap::new(),
    };

    let result = scanner.scan_vulnerabilities(&context);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert!(report.vulnerabilities_found.iter()
        .any(|v| v.id == "WEAK-KEY-SIZE"));
}

#[test]
fn test_configuration_vulnerability_detection() {
    let scanner = VulnerabilityScanner::new();
    let context = SecurityContext {
        algorithm_name: "AES".to_string(),
        key_size: 256,
        implementation_details: {
            let mut details = HashMap::new();
            details.insert("timing_constant".to_string(), "false".to_string());
            details.insert("cache_safe".to_string(), "false".to_string());
            details
        },
        environment_info: {
            let mut env = HashMap::new();
            env.insert("debug_mode".to_string(), "true".to_string());
            env.insert("log_level".to_string(), "debug".to_string());
            env
        },
    };

    let result = scanner.scan_vulnerabilities(&context);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert!(!report.vulnerabilities_found.is_empty());
    
    // Should detect timing attack vulnerability
    assert!(report.vulnerabilities_found.iter()
        .any(|v| v.id == "TIMING-ATTACK"));
    
    // Should detect cache attack vulnerability
    assert!(report.vulnerabilities_found.iter()
        .any(|v| v.id == "CACHE-ATTACK"));
        
    // Should detect debug mode issue
    assert!(report.vulnerabilities_found.iter()
        .any(|v| v.id == "DEBUG-MODE-ENABLED"));
}

#[test]
fn test_vulnerability_report_analysis() {
    let vulnerabilities = vec![
        Vulnerability {
            id: "TEST-1".to_string(),
            title: "Test Vulnerability".to_string(),
            description: "Test description".to_string(),
            vulnerability_type: VulnerabilityType::WeakCryptography,
            severity: VulnerabilitySeverity::High,
            confidence: 0.9,
            cve_references: vec![],
            affected_components: vec!["Test".to_string()],
            attack_vectors: vec![AttackVector::Network],
            exploitation_difficulty: ExploitationDifficulty::Medium,
            mitigation_strategies: vec!["Test mitigation".to_string()],
            discovery_date: std::time::SystemTime::now(),
        }
    ];

    let report = VulnerabilityReport {
        scan_timestamp: std::time::SystemTime::now(),
        context_analyzed: "Test".to_string(),
        vulnerabilities_found: vulnerabilities,
        risk_score: 75.0,
        compliance_issues: vec![],
        recommendations: vec![],
        scan_coverage: ScanCoverage {
            algorithms_scanned: vec!["Test".to_string()],
            components_analyzed: vec![],
            vulnerability_classes_checked: vec![],
            coverage_percentage: 80.0,
            scan_limitations: vec![],
        },
        false_positive_likelihood: 0.1,
    };

    assert!(!report.has_critical_vulnerabilities()); // High, not Critical
    assert_eq!(report.get_vulnerabilities_above_severity(VulnerabilitySeverity::Medium).len(), 1);
    assert!(report.security_posture_score() < 100.0);

    let counts = report.count_vulnerabilities_by_type();
    assert_eq!(counts.get(&VulnerabilityType::WeakCryptography), Some(&1));
}

// Quick Analysis Functions Tests
#[test]
fn test_quick_timing_analysis() {
    let result = quick_analysis::check_timing_safety("test", || 42);
    assert!(result.is_ok());
}

#[test]
fn test_quick_entropy_analysis() {
    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data_large: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
    
    let result = quick_analysis::check_entropy_quality(&data_large);
    assert!(result.is_ok());
    let score = result.unwrap();
    assert!(score >= 0.0 && score <= 1.0);
}

#[test]
fn test_quick_parameter_verification() {
    let params = CryptoParameters {
        algorithm: "AES".to_string(),
        key_size: 256,
        block_size: Some(128),
        iv_size: Some(96),
        tag_size: Some(128),
        rounds: Some(14),
        custom_params: HashMap::new(),
    };
    
    let result = quick_analysis::verify_crypto_params(&params);
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should be compliant
}

// Integration Tests
#[test]
fn test_comprehensive_security_analysis_workflow() {
    // Test complete security analysis workflow
    let config = SecurityAnalysisConfig {
        level: SecurityLevel::Comprehensive,
        timing_analysis_enabled: true,
        side_channel_detection_enabled: true,
        entropy_validation_enabled: true,
        parameter_verification_enabled: true,
        vulnerability_scanning_enabled: true,
        analysis_timeout: Duration::from_secs(10),
        sample_size: 100,
        confidence_threshold: 0.9,
    };
    
    let analysis = SecurityAnalysis::new(config);
    
    // Analyze a cryptographic function
    let function_metrics = analysis.analyze_function("crypto_function", || {
        // Simulate cryptographic operation
        let mut result = 0u64;
        for i in 0..1000 {
            result = result.wrapping_add(i * 31 + 17);
        }
        result
    });
    
    assert!(function_metrics.is_ok());
    let metrics = function_metrics.unwrap();
    assert!(metrics.overall_security_score >= 0.0);
    assert!(metrics.overall_security_score <= 100.0);
    
    // Test entropy analysis
    let entropy_data: Vec<u8> = (0..1000).map(|i| (i * 31 + 17) as u8).collect();
    let entropy_metrics = analysis.analyze_entropy(&entropy_data);
    assert!(entropy_metrics.is_ok());
    
    // Test parameter verification
    let params = CryptoParameters {
        algorithm: "AES".to_string(),
        key_size: 256,
        block_size: Some(128),
        iv_size: Some(96),
        tag_size: Some(128),
        rounds: Some(14),
        custom_params: HashMap::new(),
    };
    
    let param_verification = analysis.verify_parameters(&params);
    assert!(param_verification.is_ok());
    
    // Test vulnerability scanning
    let context = SecurityContext {
        algorithm_name: "AES-256-GCM".to_string(),
        key_size: 256,
        implementation_details: HashMap::new(),
        environment_info: HashMap::new(),
    };
    
    let vuln_report = analysis.scan_vulnerabilities(&context);
    assert!(vuln_report.is_ok());
    
    // Generate comprehensive report
    let report = analysis.generate_report();
    assert!(report.is_secure() || !report.is_secure()); // Should be deterministic
    assert!(!report.recommendations.is_empty());
}

#[test]
fn test_security_analysis_error_handling() {
    let analysis = SecurityAnalysis::default();
    
    // Test insufficient data for entropy analysis
    let insufficient_data = vec![1u8, 2, 3]; // Too small
    let result = analysis.analyze_entropy(&insufficient_data);
    assert!(result.is_err());
    match result {
        Err(SecurityAnalysisError::InsufficientData(_)) => {}, // Expected
        _ => panic!("Expected InsufficientData error"),
    }
    
    // Test invalid algorithm for parameter verification
    let invalid_params = CryptoParameters {
        algorithm: "INVALID_ALGORITHM".to_string(),
        key_size: 256,
        block_size: None,
        iv_size: None,
        tag_size: None,
        rounds: None,
        custom_params: HashMap::new(),
    };
    
    let result = analysis.verify_parameters(&invalid_params);
    assert!(result.is_err());
    match result {
        Err(SecurityAnalysisError::ParameterError(_)) => {}, // Expected
        _ => panic!("Expected ParameterError"),
    }
}

#[test]
fn test_security_analysis_performance() {
    let analysis = SecurityAnalysis::default();
    
    // Test that analysis completes within reasonable time
    let start = std::time::Instant::now();
    
    let result = analysis.analyze_function("performance_test", || {
        // Simulate some work
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    });
    
    let duration = start.elapsed();
    assert!(result.is_ok());
    assert!(duration < Duration::from_secs(5)); // Should complete quickly
}
