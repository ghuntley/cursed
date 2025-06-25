//! # Feature Detection Example
//!
//! This example demonstrates how to use the CURSED compiler feature detection
//! system to write programs that adapt to different compiler capabilities.

use cursed::bootstrap::feature_detection::*;
use cursed::bootstrap::conditional_compilation::*;
use cursed::bootstrap::version_negotiation::*;
use cursed::bootstrap::diagnostic_tools::*;
use std::collections::HashMap;

fn main() {
    // Initialize tracing for better output
    tracing_subscriber::fmt::init();
    
    println!("🔍 CURSED Compiler Feature Detection Example");
    println!("=============================================\n");
    
    // 1. Initialize feature detection system
    demonstrate_feature_detection();
    
    // 2. Show conditional compilation
    demonstrate_conditional_compilation();
    
    // 3. Show version negotiation
    demonstrate_version_negotiation();
    
    // 4. Show diagnostic capabilities
    demonstrate_diagnostic_tools();
    
    // 5. Show cross-stage compatibility
    demonstrate_cross_stage_compatibility();
    
    println!("\n✅ Feature detection example completed successfully!");
}

fn demonstrate_feature_detection() {
    println!("📊 1. Basic Feature Detection");
    println!("-----------------------------");
    
    // Create a feature detection system for Stage 1
    let version = CompilerVersion {
        major: 0,
        minor: 1,
        patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: Some("abc123def".to_string()),
        build_timestamp: Some("2024-01-15T10:30:00Z".to_string()),
    };
    
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version.clone());
    
    println!("Compiler Version: {}", version);
    println!("Bootstrap Stage: {}", version.stage);
    
    // Check specific features
    let features_to_check = [
        CompilerFeature::BasicTypes,
        CompilerFeature::Goroutines,
        CompilerFeature::Channels,
        CompilerFeature::TypeAssertion,
        CompilerFeature::MetaProgramming,
        CompilerFeature::CompilerPlugins,
    ];
    
    println!("\nFeature Support Status:");
    for feature in &features_to_check {
        let supported = system.is_feature_supported(feature);
        let support_level = system.get_feature_support(feature);
        let icon = if supported { "✅" } else { "❌" };
        
        println!("  {} {:<20} [{:>12}]", icon, format!("{}", feature), format!("{}", support_level));
    }
    
    // Runtime detection
    println!("\nRuntime Feature Detection:");
    let runtime_result = system.detect_feature_runtime(&CompilerFeature::LlvmCodegen);
    println!("  LLVM Codegen: {} (method: {})", 
        if runtime_result.supported { "✅" } else { "❌" }, 
        runtime_result.detection_method);
    
    let jit_result = system.detect_feature_runtime(&CompilerFeature::JitCompilation);
    println!("  JIT Compilation: {} (method: {})", 
        if jit_result.supported { "✅" } else { "❌" }, 
        jit_result.detection_method);
    
    println!();
}

fn demonstrate_conditional_compilation() {
    println!("🔧 2. Conditional Compilation");
    println!("------------------------------");
    
    // Create feature system and conditional compiler
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
    let mut compiler = ConditionalCompiler::new(Some(system));
    
    // Add some feature flags
    compiler.add_feature_flag("debug".to_string());
    compiler.add_feature_flag("optimization".to_string());
    
    // Example source code with conditional compilation directives
    let source_code = r#"
vibe main

slay main() {
    #if_feature basic_types
    sus number: normie = 42
    sus text: tea = "Hello, World!"
    #else
    sus number = 42
    sus text = "Hello, World!"
    #endif
    
    #if_feature goroutines
    spn {
        vibez.spill("Running in goroutine!")
    }
    #else
    vibez.spill("Sequential execution")
    #endif
    
    #ifdef debug
    vibez.spill("Debug mode enabled")
    #endif
    
    #if_feature channels
    sus ch = make([]normie)
    ch <- number
    sus result = <-ch
    #else
    sus result = number
    #endif
    
    vibez.spill(text)
    vibez.spill("Result:", result)
}
"#;
    
    println!("Original source code:");
    println!("{}", source_code);
    
    // Process the source code
    match compiler.process_source(source_code) {
        Ok(processed) => {
            println!("Processed source code:");
            println!("{}", processed);
        },
        Err(e) => {
            println!("❌ Failed to process source: {}", e);
        }
    }
    
    let stats = compiler.get_statistics();
    println!("Processing statistics:");
    println!("  Feature flags: {}", stats.enabled_flags);
    println!("  Fallback strategies: {}", stats.fallback_strategies);
    
    println!();
}

fn demonstrate_version_negotiation() {
    println!("🤝 3. Version Negotiation");
    println!("-------------------------");
    
    // Create two different compiler versions
    let current_version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Stage1,
        commit_hash: None, build_timestamp: None,
    };
    
    let peer_version = CompilerVersion {
        major: 0, minor: 1, patch: 2,
        stage: BootstrapStage::Stage2,
        commit_hash: None, build_timestamp: None,
    };
    
    println!("Current compiler: {}", current_version);
    println!("Peer compiler: {}", peer_version);
    
    // Create negotiator
    let mut negotiator = VersionNegotiator::new(current_version.clone());
    
    // Create negotiation request
    let request = create_negotiation_request(
        current_version,
        vec![
            CompilerFeature::BasicTypes,
            CompilerFeature::LlvmCodegen,
            CompilerFeature::ErrorHandling,
        ],
        vec![
            CompilerFeature::Goroutines,
            CompilerFeature::Channels,
            CompilerFeature::TypeAssertion,
        ],
    );
    
    // Create peer capabilities
    let mut peer_features = HashMap::new();
    peer_features.insert(CompilerFeature::BasicTypes, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::AdvancedTypes, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::LlvmCodegen, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::ErrorHandling, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::Goroutines, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::Channels, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::TypeAssertion, FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::OptimizedCodegen, FeatureSupport::Limited);
    
    let peer_capabilities = CapabilityAdvertisement {
        compiler_version: peer_version,
        protocol_version: ProtocolVersion::V1_1,
        supported_features: peer_features,
        custom_capabilities: HashMap::new(),
        compatibility_matrix: Vec::new(),
    };
    
    // Perform negotiation
    let response = negotiator.negotiate(request, peer_capabilities);
    
    println!("\nNegotiation Result: {:?}", response.negotiation_result);
    println!("Protocol Version: {}", response.protocol_version);
    
    println!("\nAgreed Features ({}):", response.agreed_features.len());
    for (feature, support) in &response.agreed_features {
        println!("  ✅ {} [{}]", feature, support);
    }
    
    if !response.unsupported_features.is_empty() {
        println!("\nUnsupported Features ({}):", response.unsupported_features.len());
        for feature in &response.unsupported_features {
            println!("  ❌ {}", feature);
        }
    }
    
    if !response.fallback_options.is_empty() {
        println!("\nFallback Options ({}):", response.fallback_options.len());
        for fallback in &response.fallback_options {
            println!("  🔄 {} -> {}", fallback.original_feature, fallback.description);
        }
    }
    
    // Show negotiation statistics
    let stats = negotiator.get_negotiation_stats();
    println!("\nNegotiation Statistics:");
    println!("{}", stats);
    
    println!();
}

fn demonstrate_diagnostic_tools() {
    println!("🩺 4. Diagnostic Tools");
    println!("----------------------");
    
    // Create diagnostic tool
    let version = CompilerVersion {
        major: 0, minor: 1, patch: 0,
        stage: BootstrapStage::Development,
        commit_hash: Some("latest".to_string()),
        build_timestamp: Some("2024-01-15T15:45:00Z".to_string()),
    };
    let system = FeatureDetectionSystem::new(BootstrapStage::Development, version);
    
    let tool = DiagnosticTool::new()
        .with_feature_system(system);
    
    // Run comprehensive diagnostic
    let diagnostic = tool.run_full_diagnostic();
    
    println!("System Diagnostic Summary:");
    println!("  Compiler: {}", diagnostic.compiler_info.version);
    println!("  Stage: {:?}", diagnostic.compiler_info.bootstrap_stage);
    println!("  Supported Features: {}/{}", 
        diagnostic.feature_matrix.current_stage_features.len(),
        get_total_feature_count());
    println!("  Recommendations: {}", diagnostic.recommendations.len());
    
    // Show feature breakdown by category
    println!("\nFeature Breakdown:");
    let categories = categorize_features(&diagnostic.feature_matrix.current_stage_features);
    for (category, features) in categories {
        println!("  {}: {} features", category, features.len());
    }
    
    // Show top recommendations
    if !diagnostic.recommendations.is_empty() {
        println!("\nTop Recommendations:");
        for (i, rec) in diagnostic.recommendations.iter().take(3).enumerate() {
            println!("  {}. [{:?}] {}", i + 1, rec.priority, rec.title);
            println!("     {}", rec.description);
        }
    }
    
    // Export example (in real usage, you'd write to file)
    match tool.export_report(&diagnostic, ReportFormat::Text) {
        Ok(report) => {
            println!("\n📄 Sample diagnostic report (first 500 chars):");
            let preview = if report.len() > 500 {
                format!("{}...", &report[..500])
            } else {
                report
            };
            println!("{}", preview);
        },
        Err(e) => println!("❌ Failed to export report: {}", e),
    }
    
    println!();
}

fn demonstrate_cross_stage_compatibility() {
    println!("🔄 5. Cross-Stage Compatibility");
    println!("--------------------------------");
    
    let stages = [
        BootstrapStage::Stage0,
        BootstrapStage::Stage1,
        BootstrapStage::Stage2,
        BootstrapStage::Development,
    ];
    
    println!("Compatibility Matrix:");
    println!("{:<12} | Features | Basic | Advanced | Experimental", "Stage");
    println!("{:-<60}", "");
    
    for stage in stages {
        let version = CompilerVersion {
            major: 0, minor: 1, patch: 0,
            stage,
            commit_hash: None, build_timestamp: None,
        };
        
        let system = FeatureDetectionSystem::new(stage, version);
        let features = system.get_supported_features();
        
        let (basic, advanced, experimental) = categorize_features_by_maturity(&system, &features);
        
        println!("{:<12} | {:>8} | {:>5} | {:>8} | {:>12}", 
            format!("{:?}", stage),
            features.len(),
            basic,
            advanced,
            experimental
        );
    }
    
    // Show upgrade path
    println!("\nRecommended Bootstrap Upgrade Path:");
    println!("  Stage0 → Stage1 → Stage2 → Development");
    
    // Show feature evolution
    println!("\nFeature Evolution:");
    let key_features = [
        CompilerFeature::BasicTypes,
        CompilerFeature::Goroutines,
        CompilerFeature::TypeAssertion,
        CompilerFeature::MetaProgramming,
    ];
    
    for feature in key_features {
        print!("  {:<20} ", format!("{}", feature));
        for stage in stages {
            let version = CompilerVersion {
                major: 0, minor: 1, patch: 0,
                stage, commit_hash: None, build_timestamp: None,
            };
            let system = FeatureDetectionSystem::new(stage, version);
            let support = system.get_feature_support(&feature);
            
            let symbol = match support {
                FeatureSupport::Stable => "🟢",
                FeatureSupport::Limited => "🟡",
                FeatureSupport::Experimental => "🟠",
                FeatureSupport::Unsupported => "🔴",
            };
            print!(" {}", symbol);
        }
        println!();
    }
    
    println!("\nLegend: 🟢 Stable  🟡 Limited  🟠 Experimental  🔴 Unsupported");
    println!();
}

// Helper functions

fn get_total_feature_count() -> usize {
    // This would return the total number of possible features
    30 // Placeholder
}

fn categorize_features(features: &std::collections::HashMap<CompilerFeature, cursed::bootstrap::feature_detection::FeatureDetectionResult>) -> std::collections::HashMap<String, Vec<CompilerFeature>> {
    let mut categories = std::collections::HashMap::new();
    
    for (feature, _) in features {
        let category = match feature {
            CompilerFeature::BasicTypes | CompilerFeature::AdvancedTypes | 
            CompilerFeature::Generics | CompilerFeature::Interfaces => "Core Language",
            CompilerFeature::Goroutines | CompilerFeature::Channels => "Concurrency",
            CompilerFeature::LlvmCodegen | CompilerFeature::JitCompilation => "Code Generation",
            _ => "Other",
        };
        
        categories.entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(feature.clone());
    }
    
    categories
}

fn categorize_features_by_maturity(system: &FeatureDetectionSystem, features: &[CompilerFeature]) -> (usize, usize, usize) {
    let mut basic = 0;
    let mut advanced = 0;
    let mut experimental = 0;
    
    for feature in features {
        match system.get_feature_support(feature) {
            FeatureSupport::Stable => basic += 1,
            FeatureSupport::Limited => advanced += 1,
            FeatureSupport::Experimental => experimental += 1,
            FeatureSupport::Unsupported => {},
        }
    }
    
    (basic, advanced, experimental)
}
