#!/usr/bin/env cursed

/// fr fr Comprehensive Security Analysis Framework Demo
/// 
/// This example demonstrates the complete cryptographic security analysis
/// framework including timing analysis, side-channel detection, entropy
/// validation, parameter verification, and vulnerability scanning.

import "stdlib::io";
import "stdlib::crypto_advanced";

// Demo function for timing analysis
def crypto_operation(input: i32) -> i32 {
    // Simulate cryptographic operation with consistent timing
    facts result = 0;
    lowkey (sus i = 0; i < 1000; i++) {
        result = result + input * i;
    }
    return result;
}

// Demo function with potential timing vulnerability
def insecure_comparison(secret: String, input: String) -> Bool {
    lowkey (sus i = 0; i < secret.length(); i++) {
        lowkey (secret.char_at(i) != input.char_at(i)) {
            return false; // Early return creates timing vulnerability
        }
    }
    return true;
}

def main() -> Nil {
    println("🔒 CURSED Cryptographic Security Analysis Framework Demo")?;
    println("=========================================================")?;
    
    // 1. Basic Security Analysis Framework Setup
    println("\n1. Setting up Security Analysis Framework...")?;
    
    sus config = SecurityAnalysisConfig {
        level: SecurityLevel::Comprehensive,
        timing_analysis_enabled: true,
        side_channel_detection_enabled: true,
        entropy_validation_enabled: true,
        parameter_verification_enabled: true,
        vulnerability_scanning_enabled: true,
        analysis_timeout: Duration::from_secs(30),
        sample_size: 500,
        confidence_threshold: 0.95
    };
    
    sus analysis = SecurityAnalysis::new(config);
    println("✅ Security analysis framework initialized")?;
    
    // 2. Function Timing Analysis
    println("\n2. Analyzing Function Timing Characteristics...")?;
    
    // Analyze secure function
    sus secure_metrics = analysis.analyze_function("crypto_operation", || {
        crypto_operation(42)
    })?;
    
    printf("Secure Function Analysis:")?;
    printf("  - Overall Security Score: {:.2}")?;
    printf("  - Timing Variance: {:.2}")?;
    printf("  - Issues Detected: {}")?;
    
    // Analyze insecure function  
    sus insecure_metrics = analysis.analyze_function("insecure_comparison", || {
        insecure_comparison("secret123", "secret124")
    })?;
    
    printf("Insecure Function Analysis:")?;
    printf("  - Overall Security Score: {:.2}")?;
    printf("  - Timing Variance: {:.2}")?;
    printf("  - Issues Detected: {}")?;
    
    // 3. Entropy Quality Assessment
    println("\n3. Entropy Quality Assessment...")?;
    
    // Test high-quality entropy
    sus good_entropy: Array<u8> = [];
    lowkey (sus i = 0; i < 1000; i++) {
        good_entropy.push((i * 31 + 17) % 256);
    }
    
    sus entropy_metrics = analysis.analyze_entropy(good_entropy)?;
    printf("Good Entropy Analysis:")?;
    printf("  - Shannon Entropy: {:.3} bits")?;
    printf("  - Entropy Score: {:.3}")?;
    printf("  - Quality: {}")?;
    printf("  - Cryptographically Secure: {}")?;
    
    // Test poor entropy
    sus poor_entropy: Array<u8> = [];
    lowkey (sus i = 0; i < 1000; i++) {
        poor_entropy.push(42); // All same value
    }
    
    sus poor_entropy_metrics = analysis.analyze_entropy(poor_entropy)?;
    printf("Poor Entropy Analysis:")?;
    printf("  - Shannon Entropy: {:.3} bits")?;
    printf("  - Entropy Score: {:.3}")?;
    printf("  - Quality: {}")?;
    printf("  - Recommendations: {}")?;
    
    // 4. Cryptographic Parameter Verification
    println("\n4. Cryptographic Parameter Verification...")?;
    
    // Test strong parameters
    sus strong_params = CryptoParameters {
        algorithm: "AES",
        key_size: 256,
        block_size: Some(128),
        iv_size: Some(96),
        tag_size: Some(128),
        rounds: Some(14),
        custom_params: HashMap::new()
    };
    
    sus param_result = analysis.verify_parameters(strong_params)?;
    printf("Strong Parameters (AES-256):")?;
    printf("  - Compliance Score: {:.2}")?;
    printf("  - Security Level: {}")?;
    printf("  - Estimated Security Bits: {}")?;
    printf("  - FIPS Compliant: {}")?;
    printf("  - Production Ready: {}")?;
    
    // Test weak parameters
    sus weak_params = CryptoParameters {
        algorithm: "DES",
        key_size: 56,
        block_size: Some(64),
        iv_size: Some(32),
        tag_size: Some(64),
        rounds: Some(16),
        custom_params: HashMap::new()
    };
    
    sus weak_result = analysis.verify_parameters(weak_params)?;
    printf("Weak Parameters (DES):")?;
    printf("  - Compliance Score: {:.2}")?;
    printf("  - Security Level: {}")?;
    printf("  - Violations: {}")?;
    printf("  - Critical Issues: {}")?;
    
    // 5. Vulnerability Scanning
    println("\n5. Comprehensive Vulnerability Scanning...")?;
    
    // Test secure configuration
    sus secure_context = SecurityContext {
        algorithm_name: "AES-256-GCM",
        key_size: 256,
        implementation_details: hashmap! {
            "timing_constant" => "true",
            "cache_safe" => "true",
            "side_channel_resistant" => "true"
        },
        environment_info: hashmap! {
            "debug_mode" => "false",
            "log_level" => "info",
            "production" => "true"
        }
    };
    
    sus secure_scan = analysis.scan_vulnerabilities(secure_context)?;
    printf("Secure Configuration Scan:")?;
    printf("  - Risk Score: {:.1}")?;
    printf("  - Security Posture: {:.1}%")?;
    printf("  - Critical Vulnerabilities: {}")?;
    printf("  - Total Vulnerabilities: {}")?;
    
    // Test insecure configuration
    sus insecure_context = SecurityContext {
        algorithm_name: "MD5",
        key_size: 128,
        implementation_details: hashmap! {
            "timing_constant" => "false",
            "cache_safe" => "false",
            "debug_symbols" => "true"
        },
        environment_info: hashmap! {
            "debug_mode" => "true",
            "log_level" => "debug",
            "production" => "false"
        }
    };
    
    sus insecure_scan = analysis.scan_vulnerabilities(insecure_context)?;
    printf("Insecure Configuration Scan:")?;
    printf("  - Risk Score: {:.1}")?;
    printf("  - Security Posture: {:.1}%")?;
    printf("  - Critical Vulnerabilities: {}")?;
    printf("  - Total Vulnerabilities: {}")?;
    
    lowkey (insecure_scan.vulnerabilities_found.length() > 0) {
        println("  - Detected Vulnerabilities:")?;
        lowkey (sus vuln in insecure_scan.vulnerabilities_found) {
            printf("    * {} ({}): {}")?;
        }
    }
    
    // 6. Advanced Timing Analysis
    println("\n6. Advanced Timing Analysis...")?;
    
    sus timing_analyzer = TimingAnalyzer::new();
    
    // Compare constant-time vs variable-time operations
    sus timing_comparison = timing_analyzer.compare_timing(
        "constant_time", || crypto_operation(42),
        "variable_time", || insecure_comparison("test", "test"),
        100
    )?;
    
    printf("Timing Comparison Results:")?;
    printf("  - Performance Ratio: {:.3}")?;
    printf("  - Variance Ratio: {:.3}")?;
    printf("  - Significant Difference: {}")?;
    
    // 7. Side-Channel Analysis
    println("\n7. Side-Channel Vulnerability Analysis...")?;
    
    sus side_channel_detector = SideChannelDetector::new();
    
    sus side_channel_result = side_channel_detector.analyze_side_channels("crypto_op", || {
        crypto_operation(42)
    })?;
    
    printf("Side-Channel Analysis:")?;
    printf("  - Security Score: {:.3}")?;
    printf("  - Confidence: {:.3}")?;
    printf("  - Side-Channel Leak Detected: {}")?;
    printf("  - Power Analysis Score: {:.3}")?;
    printf("  - Cache Analysis Score: {:.3}")?;
    
    // 8. Quick Analysis Functions
    println("\n8. Quick Security Analysis Functions...")?;
    
    // Quick timing safety check
    sus timing_safe = quick_analysis::check_timing_safety("quick_test", || {
        crypto_operation(123)
    })?;
    printf("Quick Timing Safety: {}")?;
    
    // Quick entropy check
    sus quick_entropy_score = quick_analysis::check_entropy_quality(good_entropy)?;
    printf("Quick Entropy Score: {:.3}")?;
    
    // Quick parameter verification
    sus params_ok = quick_analysis::verify_crypto_params(strong_params)?;
    printf("Quick Parameter Check: {}")?;
    
    // 9. Comprehensive Security Report
    println("\n9. Generating Comprehensive Security Report...")?;
    
    sus security_report = analysis.generate_report();
    
    printf("=== SECURITY ANALYSIS REPORT ===")?;
    printf("Overall Security Score: {:.1}/100")?;
    printf("Analysis Timestamp: {}")?;
    printf("Secure Status: {}")?;
    printf("Critical Issues: {}")?;
    
    println("\nRecommendations:")?;
    lowkey (sus rec in security_report.recommendations) {
        printf("  • {}")?;
    }
    
    lowkey (security_report.is_secure()) {
        println("✅ SECURITY ANALYSIS PASSED - System meets security requirements")?;
    } else {
        println("❌ SECURITY ANALYSIS FAILED - Critical issues require attention")?;
        
        sus critical_issues = security_report.get_critical_issues();
        lowkey (critical_issues.length() > 0) {
            println("\nCRITICAL ISSUES REQUIRING IMMEDIATE ATTENTION:")?;
            lowkey (sus issue in critical_issues) {
                printf("  🚨 {}: {}")?;
            }
        }
    }
    
    // 10. Real-world Application Examples
    println("\n10. Real-world Security Analysis Examples...")?;
    
    // Example: Web application crypto analysis
    println("\nWeb Application Crypto Configuration:")?;
    sus web_config = SecurityContext {
        algorithm_name: "ChaCha20-Poly1305",
        key_size: 256,
        implementation_details: hashmap! {
            "protocol" => "TLS1.3",
            "perfect_forward_secrecy" => "true",
            "hsts_enabled" => "true"
        },
        environment_info: hashmap! {
            "environment" => "production",
            "load_balancer" => "nginx",
            "cert_transparency" => "enabled"
        }
    };
    
    sus web_scan = analysis.scan_vulnerabilities(web_config)?;
    printf("Web App Security Score: {:.1}%")?;
    
    // Example: Database encryption analysis
    println("\nDatabase Encryption Configuration:")?;
    sus db_config = SecurityContext {
        algorithm_name: "AES-256-GCM",
        key_size: 256,
        implementation_details: hashmap! {
            "key_rotation" => "enabled",
            "field_level_encryption" => "true",
            "key_management" => "hsm"
        },
        environment_info: hashmap! {
            "database" => "postgresql",
            "encryption_at_rest" => "true",
            "backup_encryption" => "true"
        }
    };
    
    sus db_scan = analysis.scan_vulnerabilities(db_config)?;
    printf("Database Security Score: {:.1}%")?;
    
    // Example: IoT device crypto analysis
    println("\nIoT Device Crypto Configuration:")?;
    sus iot_config = SecurityContext {
        algorithm_name: "Ed25519",
        key_size: 256,
        implementation_details: hashmap! {
            "hardware_security" => "true",
            "secure_boot" => "enabled",
            "firmware_signing" => "true"
        },
        environment_info: hashmap! {
            "device_type" => "iot_sensor",
            "update_mechanism" => "secure_ota",
            "tamper_detection" => "enabled"
        }
    };
    
    sus iot_scan = analysis.scan_vulnerabilities(iot_config)?;
    printf("IoT Device Security Score: {:.1}%")?;
    
    println("\n🎉 Cryptographic Security Analysis Demo Completed!")?;
    println("This framework provides enterprise-grade security analysis for:")?;
    println("  • Timing attack detection and prevention")?;
    println("  • Side-channel vulnerability assessment")?;
    println("  • Entropy quality validation")?;
    println("  • Cryptographic parameter verification")?;
    println("  • Comprehensive vulnerability scanning")?;
    println("  • Real-time security monitoring and reporting")?;
    
    printf("\nFramework ready for production use with {} security checks!", 
           "comprehensive")?;
}

// Utility function to demonstrate secure vs insecure implementations
def demonstrate_security_comparison() -> Nil {
    println("Security Implementation Comparison:")?;
    
    // Secure constant-time string comparison
    def secure_compare(a: String, b: String) -> Bool {
        lowkey (a.length() != b.length()) {
            return false;
        }
        
        sus result = true;
        lowkey (sus i = 0; i < a.length(); i++) {
            result = result && (a.char_at(i) == b.char_at(i));
        }
        return result;
    }
    
    // Timing analysis would show this is more secure
    sus analysis = SecurityAnalysis::default();
    
    sus secure_result = analysis.analyze_function("secure_compare", || {
        secure_compare("secret123", "secret124")
    })?;
    
    sus insecure_result = analysis.analyze_function("insecure_compare", || {
        insecure_comparison("secret123", "secret124")
    })?;
    
    printf("Secure Implementation Score: {:.1}")?;
    printf("Insecure Implementation Score: {:.1}")?;
    printf("Security Improvement: {:.1}%")?;
}

// Helper function for creating test entropy data
def generate_test_entropy(size: usize, quality: String) -> Array<u8> {
    sus data: Array<u8> = [];
    
    vibe_check (quality) {
        mood "good" => {
            // Pseudo-random data with good entropy
            lowkey (sus i = 0; i < size; i++) {
                data.push((i * 31 + 17) % 256);
            }
        }
        mood "poor" => {
            // Low entropy data
            lowkey (sus i = 0; i < size; i++) {
                data.push(42); // All same value
            }
        }
        mood "medium" => {
            // Medium entropy with some patterns
            lowkey (sus i = 0; i < size; i++) {
                data.push((i % 16) * 16 + (i % 16));
            }
        }
        basic => {
            // Default to good entropy
            lowkey (sus i = 0; i < size; i++) {
                data.push((i * 31 + 17) % 256);
            }
        }
    }
    
    return data;
}

// Function to demonstrate parameter verification across algorithms
def demonstrate_algorithm_analysis() -> Nil {
    println("\nDemonstrating Multi-Algorithm Security Analysis:")?;
    
    sus verifier = ParameterVerifier::new();
    sus algorithms = ["AES", "ChaCha20", "RSA", "ECDSA", "Ed25519"];
    
    lowkey (sus alg in algorithms) {
        sus recommended = verifier.get_recommended_parameters(alg)?;
        sus verification = verifier.verify_parameters(recommended)?;
        
        printf("Algorithm: {} - Security Level: {} - Compliant: {}")?;
    }
}
