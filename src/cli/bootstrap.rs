//! Bootstrap CLI Commands
//!
//! This module provides CLI commands for the CURSED bootstrap compiler system,
//! enabling self-hosting capability through multi-stage compilation.

use crate::bootstrap::{
    SelfCompilationVerifier, VerificationConfig, VerificationResult
};
use crate::error::{Error, Result as CursedResult};
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Bootstrap CLI command configuration
pub struct BootstrapCliConfig {
    pub work_dir: PathBuf,
    pub bootstrap_cycles: usize,
    pub timeout_minutes: u64,
    pub keep_intermediates: bool,
    pub optimization_levels: Vec<String>,
    pub verbose: bool,
    pub force: bool,
}

impl Default for BootstrapCliConfig {
    fn default() -> Self {
        Self {
            work_dir: PathBuf::from("bootstrap_verification"),
            bootstrap_cycles: 3,
            timeout_minutes: 10,
            keep_intermediates: false,
            optimization_levels: vec!["-O2".to_string()],
            verbose: false,
            force: false,
        }
    }
}

/// Create the bootstrap CLI command
pub fn bootstrap_command() -> Command {
    Command::new("bootstrap")
        .about("Bootstrap compiler self-hosting verification")
        .long_about("Verify that the CURSED compiler can compile itself through multiple stages,\nensuring self-hosting capability and compiler correctness.")
        .subcommand(
            Command::new("verify")
                .about("Run full bootstrap verification")
                .arg(
                    Arg::new("cycles")
                        .long("cycles")
                        .short('c')
                        .value_name("NUMBER")
                        .help("Number of bootstrap cycles to perform")
                        .default_value("3")
                )
                .arg(
                    Arg::new("timeout")
                        .long("timeout")
                        .short('t')
                        .value_name("MINUTES")
                        .help("Timeout for each stage in minutes")
                        .default_value("10")
                )
                .arg(
                    Arg::new("work-dir")
                        .long("work-dir")
                        .short('w')
                        .value_name("PATH")
                        .help("Working directory for bootstrap verification")
                        .default_value("bootstrap_verification")
                )
                .arg(
                    Arg::new("keep")
                        .long("keep")
                        .short('k')
                        .help("Keep intermediate files after verification")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("optimization")
                        .long("optimization")
                        .short('O')
                        .value_name("LEVEL")
                        .help("Optimization levels to test")
                        .action(clap::ArgAction::Append)
                        .default_values(["O2"])
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .short('v')
                        .help("Enable verbose output")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .help("Force verification even if previous run was recent")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("stages")
                .about("Show bootstrap stage information")
                .arg(
                    Arg::new("detail")
                        .long("detail")
                        .short('d')
                        .help("Show detailed stage information")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("clean")
                .about("Clean bootstrap verification artifacts")
                .arg(
                    Arg::new("work-dir")
                        .long("work-dir")
                        .short('w')
                        .value_name("PATH")
                        .help("Working directory to clean")
                        .default_value("bootstrap_verification")
                )
        )
        .subcommand(
            Command::new("status")
                .about("Show bootstrap system status")
        )
}

/// Handle bootstrap CLI commands
pub async fn handle_bootstrap_command(matches: &ArgMatches) -> CursedResult<()> {
    match matches.subcommand() {
        Some(("verify", sub_matches)) => {
            let config = parse_verify_config(sub_matches)?;
            run_bootstrap_verification(config).await
        }
        Some(("stages", sub_matches)) => {
            let detail = sub_matches.get_flag("detail");
            show_bootstrap_stages(detail).await
        }
        Some(("clean", sub_matches)) => {
            let work_dir = sub_matches.get_one::<String>("work-dir")
                .unwrap_or(&"bootstrap_verification".to_string());
            clean_bootstrap_artifacts(PathBuf::from(work_dir)).await
        }
        Some(("status", _)) => {
            show_bootstrap_status().await
        }
        _ => {
            println!("Use 'cursed bootstrap --help' for usage information");
            Ok(())
        }
    }
}

/// Parse verification configuration from CLI arguments
fn parse_verify_config(matches: &ArgMatches) -> CursedResult<BootstrapCliConfig> {
    let cycles = matches.get_one::<String>("cycles")
        .unwrap_or(&"3".to_string())
        .parse::<usize>()
        .map_err(|_| Error::invalid_input("Invalid number of cycles"))?;

    let timeout_minutes = matches.get_one::<String>("timeout")
        .unwrap_or(&"10".to_string())
        .parse::<u64>()
        .map_err(|_| Error::invalid_input("Invalid timeout value"))?;

    let work_dir = PathBuf::from(
        matches.get_one::<String>("work-dir")
            .unwrap_or(&"bootstrap_verification".to_string())
    );

    let keep_intermediates = matches.get_flag("keep");
    let verbose = matches.get_flag("verbose");
    let force = matches.get_flag("force");

    let optimization_levels = matches.get_many::<String>("optimization")
        .map(|vals| vals.map(|s| format!("-{}", s)).collect())
        .unwrap_or_else(|| vec!["-O2".to_string()]);

    Ok(BootstrapCliConfig {
        work_dir,
        bootstrap_cycles: cycles,
        timeout_minutes,
        keep_intermediates,
        optimization_levels,
        verbose,
        force,
    })
}

/// Run bootstrap verification with the given configuration
async fn run_bootstrap_verification(config: BootstrapCliConfig) -> CursedResult<()> {
    info!("Starting bootstrap verification with {} cycles", config.bootstrap_cycles);

    // Create verification configuration
    let verification_config = VerificationConfig {
        work_dir: config.work_dir.clone(),
        compilation_timeout: Duration::from_secs(config.timeout_minutes * 60),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates: config.keep_intermediates,
        optimization_levels: config.optimization_levels.clone(),
        bootstrap_cycles: config.bootstrap_cycles,
    };

    // Create and run verifier
    let mut verifier = SelfCompilationVerifier::new(verification_config);
    
    println!("🔄 Running bootstrap verification...");
    println!("   Cycles: {}", config.bootstrap_cycles);
    println!("   Work directory: {}", config.work_dir.display());
    println!("   Optimization levels: {:?}", config.optimization_levels);
    
    if config.verbose {
        println!("   Timeout per stage: {} minutes", config.timeout_minutes);
        println!("   Keep intermediates: {}", config.keep_intermediates);
    }

    match verifier.run_verification().await {
        Ok(result) => {
            display_verification_result(&result, config.verbose)?;
            
            if result.success {
                println!("✅ Bootstrap verification completed successfully!");
                println!("   Stages completed: {}/{}", result.stages_completed, config.bootstrap_cycles + 1);
                
                if result.convergence_analysis.binary_stability {
                    println!("🎯 Compiler convergence achieved!");
                }
                
                Ok(())
            } else {
                println!("❌ Bootstrap verification failed!");
                println!("   Issues encountered: {}", result.issues.len());
                
                if config.verbose {
                    for issue in &result.issues {
                        println!("   - {}", issue);
                    }
                }
                
                Err(Error::general_error("Bootstrap verification failed"))
            }
        }
        Err(e) => {
            error!("Bootstrap verification error: {}", e);
            println!("❌ Bootstrap verification failed with error: {}", e);
            Err(e)
        }
    }
}

/// Display detailed verification results
fn display_verification_result(result: &VerificationResult, verbose: bool) -> CursedResult<()> {
    println!("\n📊 Bootstrap Verification Results:");
    println!("   Total time: {:.2}s", result.total_time.as_secs_f64());
    println!("   Stages completed: {}", result.stages_completed);
    
    if verbose {
        println!("\n🔧 Stage Details:");
        for stage_result in &result.stage_results {
            println!("   Stage {}: {}", 
                stage_result.stage, 
                if stage_result.success { "✅ SUCCESS" } else { "❌ FAILED" }
            );
            println!("     Compilation time: {:.2}s", stage_result.compilation_time.as_secs_f64());
            println!("     Binary checksum: {}", stage_result.binary_checksum);
            
            if !stage_result.errors.is_empty() {
                println!("     Errors:");
                for error in &stage_result.errors {
                    println!("       - {}", error);
                }
            }
        }
        
        println!("\n📈 Performance Metrics:");
        if !result.performance_metrics.compilation_times.is_empty() {
            let avg_time = result.performance_metrics.compilation_times.iter()
                .map(|d| d.as_secs_f64())
                .sum::<f64>() / result.performance_metrics.compilation_times.len() as f64;
            println!("   Average compilation time: {:.2}s", avg_time);
        }
        
        if !result.performance_metrics.binary_sizes.is_empty() {
            let avg_size = result.performance_metrics.binary_sizes.iter().sum::<u64>() 
                / result.performance_metrics.binary_sizes.len() as u64;
            println!("   Average binary size: {} bytes", avg_size);
        }
        
        println!("\n🎯 Convergence Analysis:");
        println!("   Binary stability: {}", 
            if result.convergence_analysis.binary_stability { "✅ YES" } else { "❌ NO" }
        );
        println!("   Performance stability: {}", 
            if result.convergence_analysis.performance_stability { "✅ YES" } else { "❌ NO" }
        );
        
        if let Some(cycle) = result.convergence_analysis.convergence_cycle {
            println!("   Convergence achieved at cycle: {}", cycle);
        }
        
        println!("   Stability threshold: {:.1}%", 
            result.convergence_analysis.stability_threshold * 100.0);
    }
    
    if !result.issues.is_empty() && !verbose {
        println!("\n⚠️  Issues (use --verbose for details): {}", result.issues.len());
    }
    
    Ok(())
}

/// Show bootstrap stage information
async fn show_bootstrap_stages(detail: bool) -> CursedResult<()> {
    println!("🏗️  Bootstrap Stages:");
    println!("   Stage 0: Rust-based CURSED compiler (bootstrap compiler)");
    println!("   Stage 1: CURSED-based compiler (compiled by Stage 0)");
    println!("   Stage 2: Self-compiled CURSED compiler (compiled by Stage 1)");
    println!("   Stage 3+: Convergence verification (Stage N compiled by Stage N-1)");
    
    if detail {
        println!("\n📝 Stage Details:");
        println!("   Stage 0 (Bootstrap):");
        println!("     - Rust implementation in src/");
        println!("     - Provides initial CURSED compilation capability");
        println!("     - Used to compile Stage 1 compiler");
        
        println!("   Stage 1 (Self-Hosting):");
        println!("     - CURSED implementation in src/bootstrap/stage2/");
        println!("     - Complete compiler written in CURSED syntax");
        println!("     - Compiled by Stage 0 (Rust) compiler");
        
        println!("   Stage 2+ (Convergence):");
        println!("     - Each stage compiled by previous stage");
        println!("     - Binary comparison for stability verification");
        println!("     - Performance analysis for optimization validation");
        println!("     - Convergence detection when stages produce identical results");
    }
    
    Ok(())
}

/// Clean bootstrap verification artifacts
async fn clean_bootstrap_artifacts(work_dir: PathBuf) -> CursedResult<()> {
    info!("Cleaning bootstrap artifacts in {}", work_dir.display());
    
    if !work_dir.exists() {
        println!("No bootstrap artifacts found in {}", work_dir.display());
        return Ok(());
    }
    
    println!("🧹 Cleaning bootstrap artifacts...");
    println!("   Directory: {}", work_dir.display());
    
    match std::fs::remove_dir_all(&work_dir) {
        Ok(()) => {
            println!("✅ Bootstrap artifacts cleaned successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to clean bootstrap artifacts: {}", e);
            println!("❌ Failed to clean bootstrap artifacts: {}", e);
            Err(Error::io_error(format!("Failed to clean {}: {}", work_dir.display(), e)))
        }
    }
}

/// Show bootstrap system status
async fn show_bootstrap_status() -> CursedResult<()> {
    println!("🔍 Bootstrap System Status:");
    
    // Check if stage2 compiler exists
    let stage2_dir = PathBuf::from("src/bootstrap/stage2");
    let stage2_exists = stage2_dir.exists();
    
    println!("   Stage 2 compiler: {}", 
        if stage2_exists { "✅ Available" } else { "❌ Not found" }
    );
    
    if stage2_exists {
        let stage2_files = [
            "main.csd", "lexer.csd", "parser.csd", 
            "type_checker.csd", "codegen.csd", "error.csd"
        ];
        
        let mut found_files = 0;
        for file in &stage2_files {
            let file_path = stage2_dir.join(file);
            if file_path.exists() {
                found_files += 1;
            }
        }
        
        println!("   Stage 2 files: {}/{} present", found_files, stage2_files.len());
        
        if found_files == stage2_files.len() {
            println!("   Status: ✅ Ready for bootstrap verification");
        } else {
            println!("   Status: ⚠️  Incomplete stage 2 implementation");
        }
    }
    
    // Check for verification artifacts
    let work_dir = PathBuf::from("bootstrap_verification");
    if work_dir.exists() {
        println!("   Verification artifacts: ✅ Present");
        
        // Count stage directories
        if let Ok(entries) = std::fs::read_dir(&work_dir) {
            let stage_count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with("stage"))
                .count();
            
            if stage_count > 0 {
                println!("   Previous stages: {} found", stage_count);
            }
        }
    } else {
        println!("   Verification artifacts: ❌ None");
    }
    
    // Bootstrap readiness check
    let bootstrap_ready = stage2_exists;
    
    println!("\n🎯 Bootstrap Readiness:");
    if bootstrap_ready {
        println!("   Status: ✅ Ready to run bootstrap verification");
        println!("   Command: cursed bootstrap verify");
    } else {
        println!("   Status: ❌ Bootstrap verification not available");
        println!("   Reason: Stage 2 compiler implementation missing");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_bootstrap_cli_config_default() {
        let config = BootstrapCliConfig::default();
        assert_eq!(config.bootstrap_cycles, 3);
        assert_eq!(config.timeout_minutes, 10);
        assert!(!config.keep_intermediates);
        assert!(!config.verbose);
        assert!(!config.force);
    }
    
    #[test]
    fn test_bootstrap_command_creation() {
        let cmd = bootstrap_command();
        assert_eq!(cmd.get_name(), "bootstrap");
        
        // Check subcommands exist
        let subcommands: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();
        assert!(subcommands.contains(&"verify"));
        assert!(subcommands.contains(&"stages"));
        assert!(subcommands.contains(&"clean"));
        assert!(subcommands.contains(&"status"));
    }
    
    #[tokio::test]
    async fn test_clean_bootstrap_artifacts_nonexistent() {
        let temp_dir = tempdir().unwrap();
        let work_dir = temp_dir.path().join("nonexistent");
        
        let result = clean_bootstrap_artifacts(work_dir).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_show_bootstrap_status() {
        let result = show_bootstrap_status().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_show_bootstrap_stages() {
        let result = show_bootstrap_stages(false).await;
        assert!(result.is_ok());
        
        let result_detailed = show_bootstrap_stages(true).await;
        assert!(result_detailed.is_ok());
    }
}
