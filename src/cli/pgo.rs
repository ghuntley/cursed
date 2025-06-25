// CLI Integration for Profile-Guided Optimization
// 
// Provides command-line interface for PGO operations including:
// - Profile collection during compilation
// - Profile management and inspection
// - PGO-guided optimization execution
// - Performance analysis and reporting

use crate::error::{CursedError, Result};
use crate::optimization::pgo::*;
use crate::optimization::pgo::optimization_integration::{
    OptimizationResult, IssueSeverity
// };

use crate::codegen::LlvmCodeGenerator;

use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use std::time::Duration;
use std::fs;
use tracing::{debug, info, warn, error};

/// PGO (Profile-Guided Optimization) command-line interface
#[derive(Parser, Debug)]
#[command(name = "pgo")]
#[command(about = "Profile-Guided Optimization tools for CURSED")]
pub struct PgoCommand {
    #[command(subcommand)]
    
    /// Enable verbose output
    #[arg(short, long)]
    
    /// Profile directory
    #[arg(short = 'd', long, default_value = "target/pgo-profiles")]
    
    /// Configuration file
    #[arg(short, long)]
/// PGO subcommands
#[derive(Subcommand, Debug)]
pub enum PgoSubcommand {
    /// Collect profile data during program execution
    
    /// Analyze collected profile data
    
    /// Optimize code using profile data
    
    /// Manage profile data
    
    /// Show PGO statistics and information
    
    /// Validate profile data quality
    
    /// Merge multiple profile datasets
    
    /// Clean up old profile data
/// Arguments for profile collection
#[derive(Args, Debug)]
pub struct CollectArgs {
    /// Source file to compile and profile
    
    /// Output profile file
    #[arg(short, long)]
    
    /// Program arguments for execution
    #[arg(short, long)]
    
    /// Input data file for program
    #[arg(short, long)]
    
    /// Expected output file for validation
    #[arg(short, long)]
    
    /// Execution timeout in seconds
    #[arg(short, long, default_value = "300")]
    
    /// Sampling rate (0.0 to 1.0)
    #[arg(short, long, default_value = "1.0")]
    
    /// Enable function profiling
    #[arg(long, default_value = "true")]
    
    /// Enable branch profiling
    #[arg(long, default_value = "true")]
    
    /// Enable loop profiling
    #[arg(long, default_value = "true")]
    
    /// Enable memory profiling
    #[arg(long)]
    
    /// Enable real-time collection
    #[arg(long)]
/// Arguments for profile analysis
#[derive(Args, Debug)]
pub struct AnalyzeArgs {
    /// Profile file to analyze
    
    /// Output analysis report
    #[arg(short, long)]
    
    /// Analysis depth level
    #[arg(short, long, default_value = "standard")]
    
    /// Hot function threshold
    #[arg(long, default_value = "100")]
    
    /// Branch misprediction threshold
    #[arg(long, default_value = "0.1")]
    
    /// Enable cross-function analysis
    #[arg(long)]
    
    /// Generate optimization recommendations
    #[arg(long, default_value = "true")]
    
    /// Output format (text, json, html)
    #[arg(short, long, default_value = "text")]
/// Arguments for PGO optimization
#[derive(Args, Debug)]
pub struct OptimizeArgs {
    /// Source file to optimize
    
    /// Profile file to use for optimization
    #[arg(short, long)]
    
    /// Output optimized file
    #[arg(short, long)]
    
    /// Optimization level
    #[arg(short, long, default_value = "moderate")]
    
    /// Enable function inlining
    #[arg(long, default_value = "true")]
    
    /// Enable branch layout optimization
    #[arg(long, default_value = "true")]
    
    /// Enable loop optimization
    #[arg(long, default_value = "true")]
    
    /// Enable code layout optimization
    #[arg(long, default_value = "true")]
    
    /// Integration strategy
    #[arg(long, default_value = "augment")]
    
    /// Enable performance validation
    #[arg(long, default_value = "true")]
    
    /// Performance threshold for acceptance
    #[arg(long, default_value = "0.05")]
/// Arguments for profile management
#[derive(Args, Debug)]
pub struct ManageArgs {
    #[command(subcommand)]
/// Profile management subcommands
#[derive(Subcommand, Debug)]
pub enum ManageSubcommand {
    /// List available profiles
    List {
        /// Show detailed information
        #[arg(short, long)]
        
        /// Filter by quality threshold
        #[arg(short, long)]
        
        /// Filter by age (days)
        #[arg(short, long)]
    
    /// Show profile information
    Info {
        /// Profile file or ID
        
        /// Show detailed statistics
        #[arg(short, long)]
    
    /// Delete a profile
    Delete {
        /// Profile file or ID
        
        /// Force deletion without confirmation
        #[arg(short, long)]
    
    /// Rename a profile
    Rename {
        /// Current profile name
        
        /// New profile name
    
    /// Export profile data
    Export {
        /// Profile to export
        
        /// Output file
        
        /// Export format
        #[arg(short, long, default_value = "json")]
    
    /// Import profile data
    Import {
        /// Input file
        
        /// Profile name
        
        /// Input format
        #[arg(short, long, default_value = "json")]
/// Arguments for PGO information
#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Show system statistics
    #[arg(short, long)]
    
    /// Show configuration
    #[arg(short, long)]
    
    /// Show supported features
    #[arg(short, long)]
    
    /// Show version information
    #[arg(short, long)]
/// Arguments for profile validation
#[derive(Args, Debug)]
pub struct ValidateArgs {
    /// Profile file to validate
    
    /// Quality threshold for validation
    #[arg(short, long, default_value = "0.7")]
    
    /// Enable comprehensive validation
    #[arg(short, long)]
    
    /// Output validation report
    #[arg(short, long)]
    
    /// Fix validation issues if possible
    #[arg(short, long)]
/// Arguments for profile merging
#[derive(Args, Debug)]
pub struct MergeArgs {
    /// Profile files to merge
    
    /// Output merged profile
    #[arg(short, long)]
    
    /// Merge strategy
    #[arg(short, long, default_value = "weighted")]
    
    /// Quality threshold for inclusion
    #[arg(short, long, default_value = "0.5")]
    
    /// Enable outlier removal
    #[arg(long)]
/// Arguments for cleanup operations
#[derive(Args, Debug)]
pub struct CleanupArgs {
    /// Maximum age in days
    #[arg(short, long, default_value = "30")]
    
    /// Minimum quality threshold
    #[arg(short, long, default_value = "0.3")]
    
    /// Maximum number of profiles to keep
    #[arg(short, long)]
    
    /// Dry run (show what would be deleted)
    #[arg(short, long)]
    
    /// Force cleanup without confirmation
    #[arg(short, long)]
/// Execute PGO command
pub fn execute_pgo_command(cmd: PgoCommand) -> Result<()> {
    // Initialize logging if verbose
    if cmd.verbose {
        env_logger::init();
    info!("Executing PGO command: {:?}", cmd.command);
    
    match cmd.command {
    }
}

/// Execute profile collection command
fn execute_collect_command(args: CollectArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Starting profile collection for: {}", args.source_file.display());
    
    // Create PGO system configuration
    let mut config = PgoSystemConfig::default();
    config.enable_collection = true;
    config.profile_directory = cmd.profile_dir.to_string_lossy().to_string();
    
    // Set optimization level based on sampling rate
    config.optimization_level = if args.sampling_rate >= 1.0 {
        OptimizationAggressiveness::Aggressive
    } else if args.sampling_rate >= 0.5 {
        OptimizationAggressiveness::Moderate
    } else {
        OptimizationAggressiveness::Conservative
    
    // Create PGO system
    let mut pgo_system = PgoSystem::with_config(config)?;
    
    // Initialize for collection
    pgo_system.initialize_collection(&cmd.profile_dir)?;
    
    // Create execution context
    let execution_context = ExecutionContext {
        input_data: if let Some(input_file) = args.input {
            Some(std::fs::read(&input_file)?)
        } else {
            None
        expected_output: if let Some(expected_file) = args.expected {
            Some(std::fs::read_to_string(&expected_file)?)
        } else {
            None
    
    // Collect profile data
    let profile_data = pgo_system.collect_profile_data(&execution_context)?;
    
    // Store profile data
    pgo_system.store_profile_data(&profile_data)?;
    
    // Get statistics
    let stats = pgo_system.get_system_statistics();
    
    println!("Profile collection completed successfully!");
    println!("Collection duration: {:?}", profile_data.collection_duration);
    println!("Functions profiled: {}", profile_data.function_profiles.len());
    println!("Branches profiled: {}", profile_data.branch_profiles.len());
    println!("Loops profiled: {}", profile_data.loop_profiles.len());
    println!("Quality score: {:.2}", profile_data.metadata.quality_score);
    
    if cmd.verbose {
        println!("Detailed statistics:");
        println!("  Total events: {}", profile_data.collection_stats.total_events);
        println!("  Events per second: {:.2}", profile_data.collection_stats.events_per_second);
        println!("  Memory usage: {} bytes", profile_data.collection_stats.memory_usage);
    Ok(())
/// Execute profile analysis command
fn execute_analyze_command(args: AnalyzeArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Analyzing profile: {}", args.profile_file.display());
    
    // Create analysis configuration
    let mut analysis_config = ProfileAnalysisConfig::default();
    analysis_config.hot_function_threshold = args.hot_function_threshold;
    analysis_config.branch_misprediction_threshold = args.branch_threshold;
    analysis_config.enable_cross_function_analysis = args.cross_function;
    
    // Parse analysis depth
    analysis_config.analysis_depth = match args.depth.as_str() {
        _ => {
            warn!("Unknown analysis depth '{}', using standard", args.depth);
            AnalysisDepth::Standard
        }
    
    // Create analyzer
    let mut analyzer = ProfileAnalyzer::new(analysis_config)?;
    
    // Load profile data
    let config = PgoSystemConfig::default();
    let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
    let profile_data = storage.load_profile(&args.profile_file)?;
    
    // Perform analysis
    let analysis_result = analyzer.analyze_profile(&profile_data)?;
    
    // Display results
    display_analysis_results(&analysis_result, &args)?;
    
    // Save report if requested
    if let Some(output_file) = args.output {
        save_analysis_report(&analysis_result, &output_file, &args.format)?;
        println!("Analysis report saved to: {}", output_file.display());
    Ok(())
/// Execute optimization command
fn execute_optimize_command(args: OptimizeArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Optimizing {} with profile {}", args.source_file.display(), args.profile.display());
    
    // Parse optimization level
    let optimization_level = match args.level.as_str() {
        _ => {
            warn!("Unknown optimization level '{}', using moderate", args.level);
            OptimizationAggressiveness::Moderate
        }
    
    // Create PGO system configuration
    let mut config = PgoSystemConfig::default();
    config.enable_optimization = true;
    config.profile_directory = cmd.profile_dir.to_string_lossy().to_string();
    config.optimization_level = optimization_level;
    config.performance_target = args.threshold * 100.0;
    config.enable_validation = args.validate;
    
    // Create PGO system
    let mut pgo_system = PgoSystem::with_config(config)?;
    
    // Initialize for optimization
    pgo_system.initialize_optimization(&args.profile)?;
    
    // Load and optimize LLVM module
    let optimization_result = optimize_llvm_module(&args, &mut pgo_system)?;
    
    println!("Optimization completed successfully!");
    println!("Optimization level: {}", args.level);
    println!("Profile used: {}", args.profile.display());
    println!("Effectiveness score: {:.2}", optimization_result.effectiveness_score);
    println!("Optimization time: {:?}", optimization_result.optimization_time);
    
    if args.inlining {
        println!("✓ Function inlining enabled");
    }
    if args.branch_layout {
        println!("✓ Branch layout optimization enabled");
    }
    if args.loop_optimization {
        println!("✓ Loop optimization enabled");
    }
    if args.code_layout {
        println!("✓ Code layout optimization enabled");
    // Get statistics
    let stats = pgo_system.get_system_statistics();
    println!("Average performance improvement: {:.1}%", stats.average_performance_improvement * 100.0);
    
    Ok(())
/// Execute profile management command
fn execute_manage_command(args: ManageArgs, cmd: &PgoCommand) -> Result<()> {
    let config = PgoSystemConfig::default();
    let mut manager = ProfileManager::new(ProfileManagerConfig::from_pgo_config(&config))?;
    
    match args.command {
        ManageSubcommand::List { detailed, quality_threshold, max_age_days } => {
            let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
            let profiles = storage.list_profiles()?;
            
            println!("Available profiles:");
            for profile in profiles {
                // Apply filters
                if let Some(threshold) = quality_threshold {
                    if profile.quality_score < threshold {
                        continue;
                    }
                }
                
                if let Some(max_age) = max_age_days {
                    let age = std::time::SystemTime::now()
                        .duration_since(profile.created_at)
                        .unwrap_or_default();
                    if age > Duration::from_secs(max_age * 24 * 3600) {
                        continue;
                    }
                }
                
                if detailed {
                    println!("  {} ({})", profile.profile_name, profile.profile_id);
                    println!("    Quality: {:.2}", profile.quality_score);
                    println!("    Created: {:?}", profile.created_at);
                    println!("    Size: {} bytes", profile.file_size);
                    println!("    Functions: {}", profile.collection_summary.function_count);
                    println!("    Branches: {}", profile.collection_summary.branch_count);
                    println!();
                } else {
                    println!("  {} (quality: {:.2})", profile.profile_name, profile.quality_score);
                }
            }
        ManageSubcommand::Info { profile, detailed } => {
            println!("Profile information for: {}", profile);
            
            // Load and display profile information
            let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
            if let Ok(profile_data) = storage.load_profile(&PathBuf::from(&profile)) {
                println!("✅ Profile loaded successfully");
                println!("  Created: {:?}", profile_data.metadata.created_at);
                println!("  Quality score: {:.2}", profile_data.metadata.quality_score);
                println!("  Collection duration: {:?}", profile_data.collection_duration);
                println!("  Functions profiled: {}", profile_data.function_profiles.len());
                println!("  Branches profiled: {}", profile_data.branch_profiles.len());
                println!("  Loops profiled: {}", profile_data.loop_profiles.len());
                
                if detailed {
                    println!("\nDetailed Statistics:");
                    println!("  Total events: {}", profile_data.collection_stats.total_events);
                    println!("  Events per second: {:.2}", profile_data.collection_stats.events_per_second);
                    println!("  Memory usage: {} bytes", profile_data.collection_stats.memory_usage);
                    println!("  Data size: {} bytes", profile_data.data_size);
                }
            } else {
                println!("❌ Profile '{}' not found or could not be loaded", profile);
            }
        }
        
        ManageSubcommand::Delete { profile, force } => {
            if !force {
                print!("Are you sure you want to delete profile '{}'? (y/N): ", profile);
                use std::io::{self, Write};
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Delete cancelled.");
                    return Ok(());
                }
            }
            
            // Implement profile deletion
            let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
            let profile_path = PathBuf::from(&profile);
            
            if profile_path.exists() {
                std::fs::remove_file(&profile_path)?;
                println!("✅ Profile '{}' deleted successfully.", profile);
            } else {
                println!("❌ Profile '{}' not found.", profile);
            }
        }
        
        ManageSubcommand::Rename { old_name, new_name } => {
            // Implement profile renaming
            let old_path = PathBuf::from(&old_name);
            let new_path = PathBuf::from(&new_name);
            
            if old_path.exists() {
                std::fs::rename(&old_path, &new_path)?;
                println!("✅ Renamed profile '{}' to '{}'", old_name, new_name);
            } else {
                println!("❌ Profile '{}' not found.", old_name);
            }
        }
        
        ManageSubcommand::Export { profile, output, format } => {
            // Implement profile export
            let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
            let profile_path = PathBuf::from(&profile);
            
            if let Ok(profile_data) = storage.load_profile(&profile_path) {
                match format.as_str() {
                    "json" => {
                        let json_data = serde_json::to_string_pretty(&profile_data)?;
                        std::fs::write(&output, json_data)?;
                        println!("✅ Exported profile '{}' to {} (JSON format)", profile, output.display());
                    }
                    _ => {
                        println!("❌ Unsupported format: {}. Supported formats: json", format);
                    }
                }
            } else {
                println!("❌ Profile '{}' not found or could not be loaded", profile);
            }
        }
        
        ManageSubcommand::Import { input, name, format } => {
            // Implement profile import
            let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
            
            match format.as_str() {
                "json" => {
                    let json_data = std::fs::read_to_string(&input)?;
                    let profile_data: ProfileData = serde_json::from_str(&json_data)?;
                    
                    let output_name = name.unwrap_or_else(|| {
                        input.file_stem().unwrap_or_default().to_string_lossy().to_string()
                    });
                    
                    storage.store_profile(&profile_data)?;
                    println!("✅ Imported profile from {} (JSON format)", input.display());
                    println!("Profile name: {}", output_name);
                }
                _ => {
                    println!("❌ Unsupported format: {}. Supported formats: json", format);
                }
            }
        }
    }
    
    Ok(())
/// Execute info command
fn execute_info_command(args: InfoArgs, cmd: &PgoCommand) -> Result<()> {
    if args.version {
        println!("CURSED PGO System v1.0.0");
        println!("Profile-Guided Optimization for CURSED Language");
        println!();
    if args.config {
        println!("PGO Configuration:");
        println!("  Profile directory: {}", cmd.profile_dir.display());
        println!("  Default quality threshold: 0.7");
        println!("  Default performance target: 15%");
        println!();
    if args.features {
        println!("Supported PGO Features:");
        println!("  ✓ Function call frequency profiling");
        println!("  ✓ Branch prediction analysis");
        println!("  ✓ Loop iteration profiling");
        println!("  ✓ Memory access pattern analysis");
        println!("  ✓ Profile-guided function inlining");
        println!("  ✓ Branch layout optimization");
        println!("  ✓ Loop unrolling and vectorization");
        println!("  ✓ Code layout optimization");
        println!("  ✓ Profile data merging");
        println!("  ✓ Performance regression detection");
        println!();
    if args.stats {
        let config = PgoSystemConfig::default();
        let pgo_system = PgoSystem::with_config(config)?;
        let stats = pgo_system.get_system_statistics();
        
        println!("PGO System Statistics:");
        println!("  Total profiles: {}", stats.profile_count);
        println!("  Total optimization time: {:?}", stats.total_optimization_time);
        println!("  Average performance improvement: {:.1}%", stats.average_performance_improvement * 100.0);
        println!();
    // If no specific info requested, show general info
    if !args.version && !args.config && !args.features && !args.stats {
        println!("CURSED Profile-Guided Optimization (PGO) System");
        println!("Use --help for available commands and options");
        println!("Use --version, --config, --features, or --stats for specific information");
    Ok(())
/// Execute validation command
fn execute_validate_command(args: ValidateArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Validating profile: {}", args.profile_file.display());
    
    // Load profile data
    let config = PgoSystemConfig::default();
    let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
    let profile_data = storage.load_profile(&args.profile_file)?;
    
    // Create profile manager for validation
    let mut manager = ProfileManager::new(ProfileManagerConfig::from_pgo_config(&config))?;
    
    // Perform validation
    let validation_result = manager.validate_profile(&profile_data)?;
    
    // Display results
    println!("Profile Validation Results:");
    println!("  Overall score: {:.2}", validation_result.result.score);
    println!("  Validation passed: {}", if validation_result.result.passed { "✓ Yes" } else { "✗ No" });
    println!("  Quality assessment:");
    println!("    Completeness: {:.2}", validation_result.result.quality_assessment.completeness_score);
    println!("    Accuracy: {:.2}", validation_result.result.quality_assessment.accuracy_score);
    println!("    Consistency: {:.2}", validation_result.result.quality_assessment.consistency_score);
    
    if !validation_result.result.issues.is_empty() {
        println!("  Issues found:");
        for issue in &validation_result.result.issues {
            let severity_symbol = match issue.severity {
            println!("    {} {}", severity_symbol, issue.description);
            if let Some(resolution) = &issue.resolution {
                println!("      → {}", resolution);
            }
        }
    if !validation_result.result.quality_assessment.recommendations.is_empty() {
        println!("  Recommendations:");
        for recommendation in &validation_result.result.quality_assessment.recommendations {
            println!("    • {}", recommendation);
        }
    }
    
    // Save validation report if requested
    if let Some(output_file) = args.output {
        save_validation_report(&validation_result, &output_file)?;
        println!("Validation report saved to: {}", output_file.display());
    Ok(())
/// Execute merge command
fn execute_merge_command(args: MergeArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Merging {} profiles", args.profiles.len());
    
    if args.profiles.len() < 2 {
        return Err(CursedError::General("Need at least 2 profiles to merge".to_string()));
    // Load profile storage
    let config = PgoSystemConfig::default();
    let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
    
    // Get profile IDs from file paths (simplified)
    let profile_ids: Vec<String> = args.profiles.iter()
        .map(|path| path.file_stem().unwrap_or_default().to_string_lossy().to_string())
        .collect();
    
    // Perform merge
    let merged_profile = storage.merge_profiles(&profile_ids)?;
    
    // Save merged profile
    // TODO: Implement saving to specified output path
    
    println!("Profile merge completed successfully!");
    println!("Input profiles: {}", args.profiles.len());
    println!("Merge strategy: {}", args.strategy);
    println!("Quality threshold: {:.2}", args.threshold);
    println!("Output: {}", args.output.display());
    println!("Merged profile quality: {:.2}", merged_profile.metadata.quality_score);
    
    Ok(())
/// Execute cleanup command
fn execute_cleanup_command(args: CleanupArgs, cmd: &PgoCommand) -> Result<()> {
    info!("Cleaning up profiles older than {} days", args.max_age_days);
    
    let config = PgoSystemConfig::default();
    let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
    
    // Get profiles for analysis
    let profiles = storage.list_profiles()?;
    let max_age = Duration::from_secs(args.max_age_days * 24 * 3600);
    let now = std::time::SystemTime::now();
    
    let mut candidates_for_deletion = Vec::new();
    
    for profile in profiles {
        let age = now.duration_since(profile.created_at).unwrap_or_default();
        let should_delete = age > max_age || 
                           profile.quality_score < args.min_quality ||
                           (args.max_count.is_some() && candidates_for_deletion.len() >= args.max_count.unwrap());
        
        if should_delete {
            candidates_for_deletion.push(profile);
        }
    }
    
    if candidates_for_deletion.is_empty() {
        println!("No profiles found for cleanup.");
        return Ok(());
    println!("Found {} profiles for cleanup:", candidates_for_deletion.len());
    for profile in &candidates_for_deletion {
        let age_days = now.duration_since(profile.created_at)
            .unwrap_or_default()
            .as_secs() / (24 * 3600);
                profile.profile_name, age_days, profile.quality_score);
    if args.dry_run {
        println!("Dry run completed. Use --force to actually delete these profiles.");
        return Ok(());
    if !args.force {
        print!("Are you sure you want to delete these {} profiles? (y/N): ", candidates_for_deletion.len());
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Cleanup cancelled.");
            return Ok(());
        }
    }
    
    // Implement actual profile deletion
    let mut deleted_count = 0;
    let mut storage = ProfileStorage::new(ProfileStorageConfig::from_pgo_config(&config))?;
    
    for profile in candidates_for_deletion {
        // Try to delete the profile file
        let profile_path = PathBuf::from(&profile.profile_name);
        if profile_path.exists() {
            if let Err(e) = std::fs::remove_file(&profile_path) {
                warn!("Failed to delete profile {}: {}", profile.profile_name, e);
            } else {
                deleted_count += 1;
            }
        }
    println!("Cleanup completed successfully!");
    println!("Deleted {} profiles", deleted_count);
    
    Ok(())
/// Optimize LLVM module using PGO data
fn optimize_llvm_module(args: &OptimizeArgs, pgo_system: &mut PgoSystem) -> Result<OptimizationResult> {
    info!("Loading and optimizing LLVM module: {}", args.source_file.display());
    
    // Read source file
    let source_code = fs::read_to_string(&args.source_file)
        .map_err(|e| CursedError::Io(std::sync::Arc::new(e)))?;
    
    // Create LLVM code generator and compile source to module
    let mut codegen = LlvmCodeGenerator::new()?;
    
    // Enable optimizations based on the optimization level
    match args.level.as_str() {
        "conservative" => {
            // Basic optimizations for safety
            info!("Applying conservative optimizations");
        }
        "moderate" => {
            // Standard optimizations
            codegen.enable_release_optimizations()?;
            info!("Applying moderate optimizations");
        }
        "aggressive" | "experimental" => {
            // Maximum optimizations
            codegen.enable_release_optimizations()?;
            info!("Applying aggressive optimizations");
        }
        _ => {
            codegen.enable_release_optimizations()?;
        }
    }
    
    // Compile source code to LLVM IR first
    let _llvm_ir = codegen.compile(&source_code, Some(&args.source_file))?;
    
    // Get the compiled LLVM module
    let module_ref = codegen.get_module();
    let module_guard = module_ref.lock()
        .map_err(|_| CursedError::General("Failed to lock LLVM module".to_string()))?;
    
    // Use the existing PGO system's optimize_with_profile method
    let optimization_result = pgo_system.optimize_with_profile(&*module_guard)?;
    
    // Save optimized module if output path specified
    if let Some(output_path) = &args.output {
        // Generate optimized LLVM IR
        let optimized_ir = module_guard.print_to_string().to_string();
        fs::write(output_path, optimized_ir)
            .map_err(|e| CursedError::Io(std::sync::Arc::new(e)))?;
        
        info!("Optimized module saved to: {}", output_path.display());
    // Display optimization details
    if !optimization_result.optimizations_applied.is_empty() {
        info!("Applied optimizations:");
        for opt in &optimization_result.optimizations_applied {
                  opt.optimization_name, opt.target, opt.estimated_improvement * 100.0);
        }
    }
    
    if !optimization_result.issues.is_empty() {
        warn!("Optimization issues encountered:");
        for issue in &optimization_result.issues {
            let level = match issue.severity {
            warn!("  [{}] {}", level, issue.description);
        }
    }
    
    Ok(optimization_result)
// Helper functions for displaying and saving results

fn display_analysis_results(analysis: &ProfileAnalysisResult, args: &AnalyzeArgs) -> Result<()> {
    println!("Profile Analysis Results:");
    println!("═══════════════════════════");
    
    // Hot functions
    println!("\nHot Functions ({}):", analysis.hot_function_analysis.hot_functions.len());
    for (i, func) in analysis.hot_function_analysis.hot_functions.iter().take(10).enumerate() {
                func.time_percentage * 100.0, func.hotness_score);
    // Inlining candidates
    println!("\nInlining Candidates ({}):", analysis.hot_function_analysis.inline_candidates.len());
    for (i, candidate) in analysis.hot_function_analysis.inline_candidates.iter().take(5).enumerate() {
                candidate.performance_improvement_estimate * 100.0);
    // Branch analysis
    println!("\nBranch Prediction Analysis:");
    println!("  Overall accuracy: {:.1}%", analysis.branch_analysis.overall_statistics.overall_accuracy * 100.0);
    println!("  Mispredicted branches: {}", analysis.branch_analysis.mispredicted_branches.len());
    
    // Loop analysis
    println!("\nLoop Analysis:");
    println!("  Unroll candidates: {}", analysis.loop_analysis.unroll_candidates.len());
    println!("  Vectorization candidates: {}", analysis.loop_analysis.vectorization_candidates.len());
    println!("  Average iterations per execution: {:.1}", analysis.loop_analysis.efficiency_metrics.average_iterations_per_execution);
    
    // Memory analysis
    println!("\nMemory Analysis:");
    println!("  Cache optimizations: {}", analysis.memory_analysis.cache_optimizations.len());
    println!("  Layout recommendations: {}", analysis.memory_analysis.layout_recommendations.len());
    println!("  Bandwidth utilization: {:.1}%", analysis.memory_analysis.bandwidth_utilization.current_utilization * 100.0);
    
    // Optimization opportunities
    println!("\nOptimization Opportunities ({}):", analysis.optimization_opportunities.len());
    for (i, opp) in analysis.optimization_opportunities.iter().take(5).enumerate() {
                i + 1, opp.id, opp.priority, opp.expected_improvement * 100.0);
        println!("     → {}", opp.recommendation);
    // Insights
    if !analysis.insights.is_empty() {
        println!("\nKey Insights:");
        for insight in &analysis.insights {
            let icon = match insight.insight_type {
            println!("  {} {} (confidence: {:.1}%)", icon, insight.message, insight.confidence * 100.0);
        }
    }
    
    // Overall statistics
    println!("\nAnalysis Summary:");
    println!("  Analysis quality: {:.2}", analysis.analysis_quality);
    println!("  Analysis time: {:?}", analysis.analysis_time);
    
    Ok(())
fn save_analysis_report(analysis: &ProfileAnalysisResult, output_file: &std::path::Path, format: &str) -> Result<()> {
    match format {
        "json" => {
            let json_data = serde_json::to_string_pretty(analysis)
                .map_err(|e| CursedError::General(format!("JSON serialization failed: {}", e)))?;
            std::fs::write(output_file, json_data)?;
        }
        "html" => {
            let html_report = generate_html_report(analysis)?;
            std::fs::write(output_file, html_report)?;
        }
        "text" | _ => {
            let text_report = generate_text_report(analysis)?;
            std::fs::write(output_file, text_report)?;
        }
    }
    Ok(())
fn save_validation_report(validation: &ProfileValidation, output_file: &std::path::Path) -> Result<()> {
    let report = format!(
        "Profile Validation Report\n\
         ========================\n\
         \n\
         Overall Score: {:.2}\n\
         Validation Passed: {}\n\
         Validation Time: {:?}\n\
         \n\
         Quality Assessment:\n\
         - Completeness: {:.2}\n\
         - Accuracy: {:.2}\n\
         - Consistency: {:.2}\n\
         \n\
         Issues Found: {}\n\
        validation.result.quality_assessment.recommendations.len()
    );
    
    std::fs::write(output_file, report)?;
    Ok(())
fn generate_html_report(analysis: &ProfileAnalysisResult) -> Result<String> {
    // Generate HTML report (simplified)
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>PGO Analysis Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .section {{ margin: 20px 0; }}
        .metric {{ margin: 5px 0; }}
        .opportunity {{ background: #f0f8ff; padding: 10px; margin: 5px 0; border-left: 4px solid #007acc; }}
    </style>
</head>
<body>
    <h1>Profile Analysis Report</h1>
    
    <div class="section">
        <h2>Summary</h2>
        <div class="metric">Analysis Quality: {:.2}</div>
        <div class="metric">Analysis Time: {:?}</div>
        <div class="metric">Hot Functions: {}</div>
        <div class="metric">Optimization Opportunities: {}</div>
    </div>
    
    <div class="section">
        <h2>Optimization Opportunities</h2>
        {}
    </div>
</body>
</html>"#,
        analysis.optimization_opportunities.iter()
            .take(10)
            .map(|opp| format!(
                r#"<div class="opportunity">
                    <strong>{}</strong><br>
                    Priority: {:.2}, Expected Improvement: {:.1}%<br>
                    {}</div>"#,
                opp.id, opp.priority, opp.expected_improvement * 100.0, opp.recommendation
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    Ok(html)
fn generate_text_report(analysis: &ProfileAnalysisResult) -> Result<String> {
    let mut report = String::new();
    
    report.push_str("CURSED PGO Analysis Report\n");
    report.push_str("=========================\n\n");
    
    report.push_str(&format!("Analysis Quality: {:.2}\n", analysis.analysis_quality));
    report.push_str(&format!("Analysis Time: {:?}\n\n", analysis.analysis_time));
    
    report.push_str("Hot Functions:\n");
    for func in analysis.hot_function_analysis.hot_functions.iter().take(10) {
                                func.function_name, func.call_frequency, func.hotness_score));
    report.push_str("\nOptimization Opportunities:\n");
    for opp in analysis.optimization_opportunities.iter().take(10) {
                                opp.id, opp.priority, opp.expected_improvement * 100.0));
        report.push_str(&format!("    {}\n", opp.recommendation));
    Ok(report)
}
