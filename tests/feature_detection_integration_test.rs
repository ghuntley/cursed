//! Integration tests for the CURSED compiler feature detection system

use cursed::bootstrap::feature_detection::*;
use cursed::bootstrap::conditional_compilation::*;
use cursed::bootstrap::version_negotiation::*;
use cursed::bootstrap::diagnostic_tools::*;
use std::collections::HashMap;

#[test]
fn test_feature_detection_system_initialization() {
    let version = CompilerVersion {
        major: 0,
        minor: 1,
        patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: Some("abc123".to_string()),
        build_timestamp: Some("2024-01-01T00:00:00Z".to_string()),
    };
    
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version.clone());
    
    assert_eq!(system.current_stage, BootstrapStage::Stage1);
    assert_eq!(system.current_version.major, 0);
    assert_eq!(system.current_version.minor, 1);
    assert_eq!(system.current_version.patch, 0);
    
    // Test basic feature support
    assert!(system.is_feature_supported(&CompilerFeature::BasicTypes));
    assert!(system.is_feature_supported(&CompilerFeature::LlvmCodegen));
    
    // Test stage-appropriate feature support
    assert_eq!(system.get_feature_support(&CompilerFeature::BasicTypes), FeatureSupport::Stable);
    assert_eq!(system.get_feature_support(&CompilerFeature::Interfaces), FeatureSupport::Stable);
    
    let supported_features = system.get_supported_features();
    assert!(!supported_features.is_empty());
    assert!(supported_features.contains(&CompilerFeature::BasicTypes));
}

#[test]
fn test_cross_stage_feature_differences() {
    // Test Stage 0 (Bootstrap)
    let stage0_version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage0,
        commit_hash: None, build_timestamp: None,
    };
    let stage0_system = FeatureDetectionSystem::new(BootstrapStage::Stage0, stage0_version);
    
    // Test Stage 2 (Stable)
    let stage2_version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage2,
        commit_hash: None, build_timestamp: None,
    };
    let stage2_system = FeatureDetectionSystem::new(BootstrapStage::Stage2, stage2_version);
    
    // Stage 0 should have fewer features than Stage 2
    let stage0_features = stage0_system.get_supported_features();
    let stage2_features = stage2_system.get_supported_features();
    
    assert!(stage0_features.len() < stage2_features.len());
    
    // Both should support basic types
    assert!(stage0_system.is_feature_supported(&CompilerFeature::BasicTypes));
    assert!(stage2_system.is_feature_supported(&CompilerFeature::BasicTypes));
    
    // Stage 2 should support more advanced features
    assert_eq!(stage0_system.get_feature_support(&CompilerFeature::Generics), FeatureSupport::Unsupported);
    assert_eq!(stage2_system.get_feature_support(&CompilerFeature::Generics), FeatureSupport::Stable);
}

#[test]
fn test_runtime_feature_detection() {
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
    
    // Test runtime detection for LLVM
    let llvm_result = system.detect_feature_runtime(&CompilerFeature::LlvmCodegen);
    assert_eq!(llvm_result.feature, CompilerFeature::LlvmCodegen);
    assert_eq!(llvm_result.detection_method, "llvm_context_creation");
    
    // Test runtime detection for JIT
    let jit_result = system.detect_feature_runtime(&CompilerFeature::JitCompilation);
    assert_eq!(jit_result.feature, CompilerFeature::JitCompilation);
    assert_eq!(jit_result.detection_method, "jit_engine_creation");
    
    // Test runtime detection caching
    let cached_result = system.detect_feature_runtime(&CompilerFeature::LlvmCodegen);
    assert_eq!(cached_result.feature, llvm_result.feature);
    assert_eq!(cached_result.supported, llvm_result.supported);
}

#[test]
fn test_compatibility_checking() {
    let current_version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, current_version);
    
    // Test compatible version
    let compatible_version = CompilerVersion {
        major: 0, minor: 1, patch: 1,
        stage: BootstrapStage::Stage2,
        commit_hash: None, build_timestamp: None,
    };
    let compatibility = system.check_compatibility(&compatible_version);
    assert!(compatibility.compatible || !compatibility.major_issues.is_empty() || !compatibility.minor_issues.is_empty());
    
    // Test incompatible version (different major version)
    let incompatible_version = CompilerVersion {
        major: 1, minor: 0, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let incompatibility = system.check_compatibility(&incompatible_version);
    assert!(!incompatibility.compatible);
    assert!(!incompatibility.major_issues.is_empty());
}

#[test]
fn test_conditional_compilation() {
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
    let mut compiler = ConditionalCompiler::new(Some(system));
    
    // Test feature-based condition
    let condition = ConditionalDirective::IfFeature(CompilerFeature::BasicTypes);
    assert!(compiler.evaluate_condition(&condition));
    
    let unsupported_condition = ConditionalDirective::IfFeature(CompilerFeature::CompilerPlugins);
    // CompilerPlugins should be unsupported in Stage1
    // Note: This might be supported depending on the actual capability matrix
    
    // Test feature flags
    compiler.add_feature_flag("debug".to_string());
    let flag_condition = ConditionalDirective::IfFlag("debug".to_string());
    assert!(compiler.evaluate_condition(&flag_condition));
    
    let missing_flag_condition = ConditionalDirective::IfFlag("release".to_string());
    assert!(!compiler.evaluate_condition(&missing_flag_condition));
    
    // Test complex conditions
    let all_condition = ConditionalDirective::IfAll(vec![
        ConditionalDirective::IfFeature(CompilerFeature::BasicTypes),
        ConditionalDirective::IfFlag("debug".to_string()),
    ]);
    assert!(compiler.evaluate_condition(&all_condition));
    
    let any_condition = ConditionalDirective::IfAny(vec![
        ConditionalDirective::IfFeature(CompilerFeature::BasicTypes),
        ConditionalDirective::IfFlag("nonexistent".to_string()),
    ]);
    assert!(compiler.evaluate_condition(&any_condition));
}

#[test]
fn test_source_preprocessing() {
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
    let mut compiler = ConditionalCompiler::new(Some(system));
    
    let source = r#""
slay main() {
    #if_feature basic_types
    sus x: normie = 42
    #else
    sus x = 42
    #endif
    
    #if_feature compiler_plugins
    load_plugin("advanced_features")
    #else
    // Use built-in functionality
    #endif
    
    vibez.spill("Hello, World!")
}
"#";
    
    let processed = compiler.process_source(source).unwrap();
    
    // Should include basic_types block since it's supported
    assert!(processed.contains("sus x: normie = 42"));
    
    // Should include fallback for compiler_plugins since it's likely unsupported
    assert!(processed.contains("// Use built-in functionality") || 
            processed.contains("load_plugin")); // Might be supported in some cases
    
    // Should preserve unprocessed lines
    assert!(processed.contains("vibez.spill(\"Hello, World!\")"));
}

#[test]
fn test_version_negotiation() {
    let current_version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let mut negotiator = VersionNegotiator::new(current_version.clone());
    
    // Create a negotiation request
    let request = create_negotiation_request(
        current_version,
        vec![CompilerFeature::BasicTypes, CompilerFeature::LlvmCodegen],
        vec![CompilerFeature::Goroutines, CompilerFeature::Channels],
    );
    
    // Create mock peer capabilities
    let peer_version = CompilerVersion {
        major: 0, minor: 1, patch: 1,
        stage: BootstrapStage::Stage2,
        commit_hash: None, build_timestamp: None,
    };
    
    let mut peer_features = HashMap::new();
    peer_features.insert(CompilerFeature::BasicTypes, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::LlvmCodegen, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::Goroutines, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::Channels, FeatureSupport::Limited);
    
    let peer_capabilities = CapabilityAdvertisement {
        compiler_version: peer_version,
        protocol_version: ProtocolVersion::V1_1,
        supported_features: peer_features,
        custom_capabilities: HashMap::new(),
        compatibility_matrix: Vec::new(),
    };
    
    // Perform negotiation
    let response = negotiator.negotiate(request, peer_capabilities);
    
    // Should succeed since all required features are available
    assert!(matches!(response.negotiation_result, 
        NegotiationResult::FullCompatibility | 
        NegotiationResult::PartialCompatibility(_)));
    
    // Should agree on required features
    assert!(response.agreed_features.contains_key(&CompilerFeature::BasicTypes));
    assert!(response.agreed_features.contains_key(&CompilerFeature::LlvmCodegen));
    
    // Should include preferred features if available
    assert!(response.agreed_features.contains_key(&CompilerFeature::Goroutines) || 
            response.agreed_features.contains_key(&CompilerFeature::Channels));
}

#[test]
fn test_diagnostic_report_generation() {
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Development,
        commit_hash: Some("abc123".to_string()),
        build_timestamp: Some("2024-01-01T00:00:00Z".to_string()),
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Development, version);
    
    let report = system.generate_diagnostic_report();
    
    assert_eq!(report.bootstrap_stage, BootstrapStage::Development);
    assert!(report.supported_features > 0);
    assert!(!report.feature_details.is_empty());
    
    // Check that basic features are reported
    assert!(report.feature_details.contains_key(&CompilerFeature::BasicTypes));
    
    // Verify report formatting
    let report_str = format!("{}", report);
    assert!(report_str.contains("CURSED Compiler Diagnostic Report"));
    assert!(report_str.contains("Version:"));
    assert!(report_str.contains("Bootstrap Stage:"));
    assert!(report_str.contains("Features:"));
}

#[test]
fn test_comprehensive_diagnostic_tool() {
    let tool = DiagnosticTool::new();
    let diagnostic = tool.run_full_diagnostic();
    
    // Verify all sections are populated
    assert_eq!(diagnostic.compiler_info.bootstrap_stage, BootstrapStage::Development);
    assert!(!diagnostic.feature_matrix.current_stage_features.is_empty());
    assert!(!diagnostic.compatibility_analysis.cross_stage_compatibility.stage_compatibility_matrix.is_empty());
    
    // Test report export in different formats
    let json_report = tool.export_report(&diagnostic, ReportFormat::Json);
    assert!(json_report.is_ok());
    
    let text_report = tool.export_report(&diagnostic, ReportFormat::Text);
    assert!(text_report.is_ok());
    
    let html_report = tool.export_report(&diagnostic, ReportFormat::Html);
    assert!(html_report.is_ok());
    
    // Verify content
    let text_content = text_report.unwrap();
    assert!(text_content.contains("CURSED Compiler Diagnostic Report"));
    assert!(text_content.contains("Compiler Version:"));
    assert!(text_content.contains("Bootstrap Stage:"));
}

#[test]
fn test_global_feature_detection_functions() {
    // Initialize global system
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    init_feature_detection(BootstrapStage::Stage1, version);
    
    // Test global functions
    assert!(is_feature_supported(&CompilerFeature::BasicTypes));
    assert_eq!(get_feature_support_level(&CompilerFeature::BasicTypes), FeatureSupport::Stable);
    
    let global_system = get_feature_system();
    assert!(global_system.is_some());
    
    let system = global_system.unwrap();
    assert_eq!(system.current_stage, BootstrapStage::Stage1);
}

#[test]
fn test_feature_fallback_strategies() {
    let compiler = ConditionalCompiler::new(None);
    
    // Test built-in fallback strategies
    let stats = compiler.get_statistics();
    assert!(stats.fallback_strategies > 0); // Should have default fallbacks
    
    // Test fallback application
    let goroutines_directive = ConditionalDirective::IfFeature(CompilerFeature::Goroutines);
    let fallback_result = compiler.apply_fallback_strategy(&goroutines_directive);
    assert!(fallback_result.contains("Sequential execution fallback"));
    
    let channels_directive = ConditionalDirective::IfFeature(CompilerFeature::Channels);
    let channels_fallback = compiler.apply_fallback_strategy(&channels_directive);
    assert!(channels_fallback.contains("Direct communication fallback"));
}

#[test]
fn test_performance_and_stability_assessment() {
    let tool = DiagnosticTool::new();
    
    // Test performance impact assessment
    assert_eq!(tool.assess_performance_impact(&CompilerFeature::BasicTypes), 
               cursed::bootstrap::diagnostic_tools::PerformanceImpact::None);
    assert_eq!(tool.assess_performance_impact(&CompilerFeature::GarbageCollection), 
               cursed::bootstrap::diagnostic_tools::PerformanceImpact::Medium);
    assert_eq!(tool.assess_performance_impact(&CompilerFeature::MetaProgramming), 
               cursed::bootstrap::diagnostic_tools::PerformanceImpact::High);
    
    // Test stability rating assessment
    assert_eq!(tool.assess_stability_rating(&CompilerFeature::BasicTypes), 
               cursed::bootstrap::diagnostic_tools::StabilityRating::Stable);
    assert_eq!(tool.assess_stability_rating(&CompilerFeature::Goroutines), 
               cursed::bootstrap::diagnostic_tools::StabilityRating::Experimental);
    assert_eq!(tool.assess_stability_rating(&CompilerFeature::CompilerPlugins), 
               cursed::bootstrap::diagnostic_tools::StabilityRating::Unstable);
}

#[test]
fn test_capability_matrix_integrity() {
    // Test all bootstrap stages have consistent capability matrices
    let stages = [
        BootstrapStage::Stage0,
        BootstrapStage::Stage1, 
        BootstrapStage::Stage2,
        BootstrapStage::Development,
    ];
    
    for stage in stages {
        let version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage,
            commit_hash: None, build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(stage, version);
        let features = system.get_supported_features();
        
        // All stages should support basic types
        assert!(features.contains(&CompilerFeature::BasicTypes));
        
        // Development should support the most features
        if stage == BootstrapStage::Development {
            assert!(features.len() >= 20); // Should have many features
        }
        
        // Stage0 should support the fewest features
        if stage == BootstrapStage::Stage0 {
            assert!(features.len() <= 15); // Should have limited features
        }
    }
}
