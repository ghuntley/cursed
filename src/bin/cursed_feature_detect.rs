//! # CURSED Feature Detection CLI Tool
//!
//! Command-line interface for the CURSED compiler feature detection system.
//! Provides comprehensive capability reporting, compatibility analysis, and
//! diagnostic tools for the bootstrap compiler.

use std::collections::HashMap;
use std::path::PathBuf;
use clap::{Arg, Command, ArgMatches};
use tracing::{info, error, Level};
use tracing_subscriber::{fmt, EnvFilter};

use cursed::bootstrap::feature_detection::{
    BootstrapStage, CompilerVersion, CompilerFeature, FeatureDetectionSystem,
    init_feature_detection, get_feature_system
};
use cursed::bootstrap::diagnostic_tools::{
    DiagnosticTool, ReportFormat, quick_diagnostic, export_diagnostic_to_file
};
use cursed::bootstrap::version_negotiation::{
    VersionNegotiator, create_negotiation_request
};
use cursed::bootstrap::conditional_compilation::ConditionalCompiler;

fn main() {
    let matches = build_cli().get_matches();
    
    // Initialize tracing
    init_tracing();
    
    // Parse global options
    let stage = parse_bootstrap_stage(matches.get_one::<String>("stage"));
    let version = parse_compiler_version(
        matches.get_one::<String>("version"),
        stage
    );
    
    // Initialize feature detection system
    init_feature_detection(stage, version.clone());
    
    // Execute subcommand
    match matches.subcommand() {
        Some(("detect", sub_matches)) => cmd_detect(sub_matches),
        Some(("diagnostic", sub_matches)) => cmd_diagnostic(sub_matches),
        Some(("compatibility", sub_matches)) => cmd_compatibility(sub_matches),
        Some(("negotiate", sub_matches)) => cmd_negotiate(sub_matches),
        Some(("preprocess", sub_matches)) => cmd_preprocess(sub_matches),
        Some(("list-features", sub_matches)) => cmd_list_features(sub_matches),
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

fn build_cli() -> Command {
    Command::new("cursed-feature-detect")
        .version(env!("CARGO_PKG_VERSION"))
        .author("CURSED Language Team")
        .about("CURSED Compiler Feature Detection and Diagnostic Tool")
        .arg(
            Arg::new("stage")
                .long("stage")
                .value_name("STAGE")
                .help("Bootstrap stage (stage0, stage1, stage2, development)")
                .default_value("development")
        )
        .arg(
            Arg::new("version")
                .long("version-string")
                .value_name("VERSION")
                .help("Compiler version string (e.g., 0.1.0)")
                .default_value("0.1.0")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::Count)
                .help("Increase verbosity level")
        )
        .subcommand(
            Command::new("detect")
                .about("Detect available compiler features")
                .arg(
                    Arg::new("feature")
                        .long("feature")
                        .value_name("FEATURE")
                        .help("Specific feature to detect (optional)")
                )
                .arg(
                    Arg::new("runtime")
                        .long("runtime")
                        .action(clap::ArgAction::SetTrue)
                        .help("Perform runtime feature detection")
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(clap::ArgAction::SetTrue)
                        .help("Output results in JSON format")
                )
        )
        .subcommand(
            Command::new("diagnostic")
                .about("Run comprehensive system diagnostic")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file path")
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .help("Output format (json, yaml, text, html)")
                        .default_value("text")
                )
                .arg(
                    Arg::new("include-benchmarks")
                        .long("include-benchmarks")
                        .action(clap::ArgAction::SetTrue)
                        .help("Include performance benchmarks")
                )
        )
        .subcommand(
            Command::new("compatibility")
                .about("Check compatibility between compiler versions")
                .arg(
                    Arg::new("target-version")
                        .long("target")
                        .value_name("VERSION")
                        .help("Target compiler version to check against")
                        .required(true)
                )
                .arg(
                    Arg::new("target-stage")
                        .long("target-stage")
                        .value_name("STAGE")
                        .help("Target bootstrap stage")
                        .default_value("stage1")
                )
        )
        .subcommand(
            Command::new("negotiate")
                .about("Negotiate features with another compiler version")
                .arg(
                    Arg::new("peer-version")
                        .long("peer")
                        .value_name("VERSION")
                        .help("Peer compiler version")
                        .required(true)
                )
                .arg(
                    Arg::new("required-features")
                        .long("required")
                        .value_name("FEATURES")
                        .help("Comma-separated list of required features")
                )
                .arg(
                    Arg::new("preferred-features")
                        .long("preferred")
                        .value_name("FEATURES")
                        .help("Comma-separated list of preferred features")
                )
        )
        .subcommand(
            Command::new("preprocess")
                .about("Preprocess source code with conditional compilation")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Input source file")
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Output file (stdout if not specified)")
                )
                .arg(
                    Arg::new("feature-flags")
                        .long("flags")
                        .value_name("FLAGS")
                        .help("Comma-separated list of feature flags to enable")
                )
        )
        .subcommand(
            Command::new("list-features")
                .about("List all available compiler features")
                .arg(
                    Arg::new("stage")
                        .long("for-stage")
                        .value_name("STAGE")
                        .help("List features for specific bootstrap stage")
                )
                .arg(
                    Arg::new("support-level")
                        .long("support-level")
                        .value_name("LEVEL")
                        .help("Filter by support level (stable, limited, experimental, unsupported)")
                )
                .arg(
                    Arg::new("category")
                        .long("category")
                        .value_name("CATEGORY")
                        .help("Filter by feature category")
                )
        )
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

fn parse_bootstrap_stage(stage_str: Option<&String>) -> BootstrapStage {
    match stage_str.map(|s| s.as_str()).unwrap_or("development") {
        "stage0" => BootstrapStage::Stage0,
        "stage1" => BootstrapStage::Stage1,
        "stage2" => BootstrapStage::Stage2,
        "development" | "dev" => BootstrapStage::Development,
        _ => {
            eprintln!("Invalid bootstrap stage. Using development.");
            BootstrapStage::Development
        }
    }
}

fn parse_compiler_version(version_str: Option<&String>, stage: BootstrapStage) -> CompilerVersion {
    let version_str = version_str.map(|s| s.as_str()).unwrap_or("0.1.0");
    let parts: Vec<&str> = version_str.split('.').collect();
    
    let (major, minor, patch) = if parts.len() >= 3 {
        (
            parts[0].parse().unwrap_or(0),
            parts[1].parse().unwrap_or(1),
            parts[2].parse().unwrap_or(0),
        )
    } else {
        (0, 1, 0)
    };
    
    CompilerVersion {
        major,
        minor,
        patch,
        stage,
        commit_hash: std::env::var("GIT_HASH").ok(),
        build_timestamp: std::env::var("BUILD_TIMESTAMP").ok(),
    }
}

fn cmd_detect(matches: &ArgMatches) {
    info!("Running feature detection");
    
    let system = get_feature_system();
    if system.is_none() {
        error!("Feature detection system not initialized");
        std::process::exit(1);
    }
    let system = system.unwrap();
    
    let runtime_detection = matches.get_flag("runtime");
    let json_output = matches.get_flag("json");
    
    if let Some(feature_name) = matches.get_one::<String>("feature") {
        // Detect specific feature
        if let Ok(feature) = parse_feature_name(feature_name) {
            if runtime_detection {
                let result = system.detect_feature_runtime(&feature);
                if json_output {
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                } else {
                    println!("Feature: {}", feature);
                    println!("Supported: {}", result.supported);
                    println!("Support Level: {}", result.support_level);
                    println!("Detection Method: {}", result.detection_method);
                    println!("Fallback Available: {}", result.fallback_available);
                }
            } else {
                let supported = system.is_feature_supported(&feature);
                let support_level = system.get_feature_support(&feature);
                
                if json_output {
                    let result = serde_json::json!({
                        "feature": feature,
                        "supported": supported,
                        "support_level": support_level
                    });
                    println!("{}", serde_json::to_string_pretty(&result).unwrap());
                } else {
                    println!("Feature: {}", feature);
                    println!("Supported: {}", supported);
                    println!("Support Level: {}", support_level);
                }
            }
        } else {
            eprintln!("Unknown feature: {}", feature_name);
            std::process::exit(1);
        }
    } else {
        // Detect all features
        let supported_features = system.get_supported_features();
        
        if json_output {
            let mut results = HashMap::new();
            for feature in &supported_features {
                let support_level = system.get_feature_support(feature);
                if runtime_detection {
                    let runtime_result = system.detect_feature_runtime(feature);
                    results.insert(feature, runtime_result);
                } else {
                    results.insert(feature, serde_json::json!({
                        "supported": true,
                        "support_level": support_level
                    }));
                }
            }
            println!("{}", serde_json::to_string_pretty(&results).unwrap());
        } else {
            println!("Supported Features ({}):", supported_features.len());
            println!("{:-<50}", "");
            
            for feature in supported_features {
                let support_level = system.get_feature_support(&feature);
                if runtime_detection {
                    let result = system.detect_feature_runtime(&feature);
                    println!("{:<30} {} [{:>12}] ({})", 
                        format!("{}", feature),
                        if result.supported { "✓" } else { "✗" },
                        format!("{}", support_level),
                        result.detection_method
                    );
                } else {
                    println!("{:<30} ✓ [{:>12}]", 
                        format!("{}", feature),
                        format!("{}", support_level)
                    );
                }
            }
        }
    }
}

fn cmd_diagnostic(matches: &ArgMatches) {
    info!("Running comprehensive diagnostic");
    
    let tool = DiagnosticTool::new();
    let diagnostic = tool.run_full_diagnostic();
    
    let format = match matches.get_one::<String>("format").map(|s| s.as_str()) {
        Some("json") => ReportFormat::Json,
        Some("yaml") => ReportFormat::Yaml,
        Some("html") => ReportFormat::Html,
        _ => ReportFormat::Text,
    };
    
    let report_content = tool.export_report(&diagnostic, format).unwrap_or_else(|e| {
        eprintln!("Failed to generate report: {}", e);
        std::process::exit(1);
    });
    
    if let Some(output_file) = matches.get_one::<String>("output") {
        if let Err(e) = export_diagnostic_to_file(&diagnostic, output_file, format) {
            eprintln!("Failed to write report to file: {}", e);
            std::process::exit(1);
        }
        println!("Diagnostic report written to: {}", output_file);
    } else {
        println!("{}", report_content);
    }
}

fn cmd_compatibility(matches: &ArgMatches) {
    info!("Checking compatibility");
    
    let system = get_feature_system();
    if system.is_none() {
        error!("Feature detection system not initialized");
        std::process::exit(1);
    }
    let system = system.unwrap();
    
    let target_version_str = matches.get_one::<String>("target-version").unwrap();
    let target_stage = parse_bootstrap_stage(matches.get_one::<String>("target-stage"));
    let target_version = parse_compiler_version(Some(target_version_str), target_stage);
    
    let compatibility_report = system.check_compatibility(&target_version);
    
    println!("Compatibility Analysis");
    println!("=====================");
    println!("Current Version: {}", system.current_version);
    println!("Target Version: {}", target_version);
    println!("Compatible: {}", compatibility_report.compatible);
    
    if !compatibility_report.major_issues.is_empty() {
        println!("\nMajor Issues:");
        for issue in &compatibility_report.major_issues {
            println!("  ❌ {}", issue);
        }
    }
    
    if !compatibility_report.minor_issues.is_empty() {
        println!("\nMinor Issues:");
        for issue in &compatibility_report.minor_issues {
            println!("  ⚠️  {}", issue);
        }
    }
    
    if !compatibility_report.feature_differences.is_empty() {
        println!("\nFeature Differences:");
        for diff in &compatibility_report.feature_differences {
            println!("  📝 {}", diff);
        }
    }
    
    if compatibility_report.compatible {
        println!("\n✅ Versions are compatible");
    } else {
        println!("\n❌ Versions are incompatible");
        std::process::exit(1);
    }
}

fn cmd_negotiate(matches: &ArgMatches) {
    info!("Negotiating with peer compiler");
    
    let system = get_feature_system();
    if system.is_none() {
        error!("Feature detection system not initialized");
        std::process::exit(1);
    }
    let system = system.unwrap();
    
    let peer_version_str = matches.get_one::<String>("peer-version").unwrap();
    let peer_version = parse_compiler_version(Some(peer_version_str), BootstrapStage::Stage1);
    
    let required_features = matches.get_one::<String>("required-features")
        .map(|s| parse_feature_list(s))
        .unwrap_or_default();
    
    let preferred_features = matches.get_one::<String>("preferred-features")
        .map(|s| parse_feature_list(s))
        .unwrap_or_default();
    
    let request = create_negotiation_request(
        system.current_version.clone(),
        required_features,
        preferred_features,
    );
    
    // Create mock peer capabilities
    let mut peer_features = HashMap::new();
    peer_features.insert(CompilerFeature::BasicTypes, cursed::bootstrap::feature_detection::FeatureSupport::Stable);
    peer_features.insert(CompilerFeature::LlvmCodegen, cursed::bootstrap::feature_detection::FeatureSupport::Stable);
    
    let peer_capabilities = cursed::bootstrap::version_negotiation::CapabilityAdvertisement {
        compiler_version: peer_version,
        protocol_version: cursed::bootstrap::version_negotiation::ProtocolVersion::V1_1,
        supported_features: peer_features,
        custom_capabilities: HashMap::new(),
        compatibility_matrix: Vec::new(),
    };
    
    let mut negotiator = VersionNegotiator::new(system.current_version.clone());
    let response = negotiator.negotiate(request, peer_capabilities);
    
    println!("Version Negotiation Result");
    println!("=========================");
    println!("Result: {:?}", response.negotiation_result);
    println!("Protocol: {}", response.protocol_version);
    println!("Agreed Features: {}", response.agreed_features.len());
    println!("Unsupported Features: {}", response.unsupported_features.len());
    println!("Fallback Options: {}", response.fallback_options.len());
    
    if !response.agreed_features.is_empty() {
        println!("\nAgreed Features:");
        for (feature, support) in &response.agreed_features {
            println!("  ✓ {} [{}]", feature, support);
        }
    }
    
    if !response.unsupported_features.is_empty() {
        println!("\nUnsupported Features:");
        for feature in &response.unsupported_features {
            println!("  ❌ {}", feature);
        }
    }
    
    if !response.fallback_options.is_empty() {
        println!("\nFallback Options:");
        for fallback in &response.fallback_options {
            println!("  🔄 {} -> {}", fallback.original_feature, fallback.description);
        }
    }
}

fn cmd_preprocess(matches: &ArgMatches) {
    info!("Preprocessing source code");
    
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    
    let source = std::fs::read_to_string(input_file).unwrap_or_else(|e| {
        eprintln!("Failed to read input file {}: {}", input_file, e);
        std::process::exit(1);
    });
    
    let system = get_feature_system();
    let mut compiler = ConditionalCompiler::new(system.cloned());
    
    // Add feature flags if specified
    if let Some(flags_str) = matches.get_one::<String>("feature-flags") {
        for flag in flags_str.split(',') {
            compiler.add_feature_flag(flag.trim().to_string());
        }
    }
    
    let processed_source = compiler.process_source(&source).unwrap_or_else(|e| {
        eprintln!("Failed to process source code: {}", e);
        std::process::exit(1);
    });
    
    if let Some(output_file) = output_file {
        std::fs::write(output_file, processed_source).unwrap_or_else(|e| {
            eprintln!("Failed to write output file {}: {}", output_file, e);
            std::process::exit(1);
        });
        println!("Processed source written to: {}", output_file);
    } else {
        println!("{}", processed_source);
    }
}

fn cmd_list_features(matches: &ArgMatches) {
    info!("Listing compiler features");
    
    let system = get_feature_system();
    let supported_features = if let Some(system) = system {
        system.get_supported_features()
    } else {
        get_all_features()
    };
    
    // Apply filters
    let mut filtered_features = supported_features;
    
    if let Some(support_level_str) = matches.get_one::<String>("support-level") {
        if let Some(system) = system {
            let target_level = parse_support_level(support_level_str);
            filtered_features.retain(|f| system.get_feature_support(f) == target_level);
        }
    }
    
    println!("Available Features ({}):", filtered_features.len());
    println!("{:-<60}", "");
    
    let categories = categorize_features(&filtered_features);
    
    for (category, features) in categories {
        println!("\n{}:", category);
        for feature in features {
            if let Some(system) = system {
                let support_level = system.get_feature_support(&feature);
                println!("  {:<30} [{}]", format!("{}", feature), support_level);
            } else {
                println!("  {}", feature);
            }
        }
    }
}

fn parse_feature_name(name: &str) -> Result<CompilerFeature, String> {
    use CompilerFeature::*;
    
    match name.to_lowercase().replace("_", "").replace("-", "").as_str() {
        "basictypes" => Ok(BasicTypes),
        "advancedtypes" => Ok(AdvancedTypes),
        "generics" => Ok(Generics),
        "interfaces" => Ok(Interfaces),
        "typeassertion" => Ok(TypeAssertion),
        "errorhandling" => Ok(ErrorHandling),
        "garbagecollection" | "gc" => Ok(GarbageCollection),
        "memoryprofiler" => Ok(MemoryProfiler),
        "leakdetection" => Ok(LeakDetection),
        "goroutines" => Ok(Goroutines),
        "channels" => Ok(Channels),
        "channelbuffering" => Ok(ChannelBuffering),
        "selectstatement" => Ok(SelectStatement),
        "mutexsupport" => Ok(MutexSupport),
        "llvmcodegen" => Ok(LlvmCodegen),
        "jitcompilation" => Ok(JitCompilation),
        "optimizedcodegen" => Ok(OptimizedCodegen),
        "bitstreamoutput" => Ok(BitstreamOutput),
        "staticlinking" => Ok(StaticLinking),
        "reflection" => Ok(Reflection),
        "metaprogramming" => Ok(MetaProgramming),
        "compilerplugins" => Ok(CompilerPlugins),
        "crosscompilation" => Ok(CrossCompilation),
        "stdlibcore" => Ok(StdlibCore),
        "stdlibextended" => Ok(StdlibExtended),
        "stdlibexperimental" => Ok(StdlibExperimental),
        "debuginfo" => Ok(DebugInfo),
        "profiling" => Ok(Profiling),
        "tracegeneration" => Ok(TraceGeneration),
        "errorrecovery" => Ok(ErrorRecovery),
        "languageserver" => Ok(LanguageServer),
        "syntaxhighlighting" => Ok(SyntaxHighlighting),
        "autocomplete" => Ok(AutoComplete),
        "refactoring" => Ok(Refactoring),
        _ => Err(format!("Unknown feature: {}", name)),
    }
}

fn parse_feature_list(features_str: &str) -> Vec<CompilerFeature> {
    features_str
        .split(',')
        .filter_map(|name| parse_feature_name(name.trim()).ok())
        .collect()
}

fn parse_support_level(level_str: &str) -> cursed::bootstrap::feature_detection::FeatureSupport {
    use cursed::bootstrap::feature_detection::FeatureSupport::*;
    
    match level_str.to_lowercase().as_str() {
        "stable" => Stable,
        "limited" => Limited,
        "experimental" => Experimental,
        "unsupported" => Unsupported,
        _ => Stable,
    }
}

fn get_all_features() -> Vec<CompilerFeature> {
    use CompilerFeature::*;
    vec![
        BasicTypes, AdvancedTypes, Generics, Interfaces, TypeAssertion, ErrorHandling,
        GarbageCollection, MemoryProfiler, LeakDetection,
        Goroutines, Channels, ChannelBuffering, SelectStatement, MutexSupport,
        LlvmCodegen, JitCompilation, OptimizedCodegen, BitstreamOutput, StaticLinking,
        Reflection, MetaProgramming, CompilerPlugins, CrossCompilation,
        StdlibCore, StdlibExtended, StdlibExperimental,
        DebugInfo, Profiling, TraceGeneration, ErrorRecovery,
        LanguageServer, SyntaxHighlighting, AutoComplete, Refactoring,
    ]
}

fn categorize_features(features: &[CompilerFeature]) -> HashMap<String, Vec<CompilerFeature>> {
    let mut categories = HashMap::new();
    
    for feature in features {
        let category = match feature {
            CompilerFeature::BasicTypes | CompilerFeature::AdvancedTypes | 
            CompilerFeature::Generics | CompilerFeature::Interfaces | 
            CompilerFeature::TypeAssertion | CompilerFeature::ErrorHandling => "Core Language",
            
            CompilerFeature::GarbageCollection | CompilerFeature::MemoryProfiler | 
            CompilerFeature::LeakDetection => "Memory Management",
            
            CompilerFeature::Goroutines | CompilerFeature::Channels | 
            CompilerFeature::ChannelBuffering | CompilerFeature::SelectStatement | 
            CompilerFeature::MutexSupport => "Concurrency",
            
            CompilerFeature::LlvmCodegen | CompilerFeature::JitCompilation | 
            CompilerFeature::OptimizedCodegen | CompilerFeature::BitstreamOutput | 
            CompilerFeature::StaticLinking => "Code Generation",
            
            CompilerFeature::Reflection | CompilerFeature::MetaProgramming | 
            CompilerFeature::CompilerPlugins | CompilerFeature::CrossCompilation => "Advanced Features",
            
            CompilerFeature::StdlibCore | CompilerFeature::StdlibExtended | 
            CompilerFeature::StdlibExperimental => "Standard Library",
            
            CompilerFeature::DebugInfo | CompilerFeature::Profiling | 
            CompilerFeature::TraceGeneration | CompilerFeature::ErrorRecovery => "Debugging & Diagnostics",
            
            CompilerFeature::LanguageServer | CompilerFeature::SyntaxHighlighting | 
            CompilerFeature::AutoComplete | CompilerFeature::Refactoring => "Development Tools",
        };
        
        categories.entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(feature.clone());
    }
    
    categories
}
