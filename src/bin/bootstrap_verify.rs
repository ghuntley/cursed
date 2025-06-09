//! Bootstrap Verification Tool
//!
//! A command-line tool for running comprehensive self-compilation verification
//! of the CURSED compiler bootstrap process.

use clap::{Arg, Command};
use cursed::bootstrap::self_compilation_verification::{
    SelfCompilationVerifier, VerificationConfig, VerificationReport, generate_verification_report
};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{error, info};

fn main() {
    // Initialize tracing
    cursed::init_tracing();

    let matches = Command::new("bootstrap-verify")
        .version(cursed::VERSION)
        .about("CURSED Bootstrap Verification Tool")
        .arg(
            Arg::new("work-dir")
                .long("work-dir")
                .value_name("DIR")
                .help("Working directory for verification")
                .default_value("./bootstrap_verification"),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("Compilation timeout in seconds")
                .default_value("300"),
        )
        .arg(
            Arg::new("cycles")
                .long("cycles")
                .value_name("N")
                .help("Number of bootstrap cycles to test")
                .default_value("3"),
        )
        .arg(
            Arg::new("keep-intermediates")
                .long("keep-intermediates")
                .help("Keep intermediate files for debugging")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("optimization-levels")
                .long("opt-levels")
                .value_name("LEVELS")
                .help("Comma-separated optimization levels to test")
                .default_value("-O0,-O1,-O2"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file for verification report")
                .default_value("bootstrap_verification_report.md"),
        )
        .arg(
            Arg::new("quick")
                .long("quick")
                .help("Run a quick verification (fewer cycles, basic tests)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Parse arguments
    let work_dir = PathBuf::from(matches.get_one::<String>("work-dir").unwrap());
    let timeout_secs: u64 = matches.get_one::<String>("timeout").unwrap().parse()
        .expect("Invalid timeout value");
    let cycles: usize = matches.get_one::<String>("cycles").unwrap().parse()
        .expect("Invalid cycles value");
    let keep_intermediates = matches.get_flag("keep-intermediates");
    let output_file = matches.get_one::<String>("output").unwrap();
    let quick = matches.get_flag("quick");
    let verbose = matches.get_flag("verbose");

    // Parse optimization levels
    let opt_levels: Vec<String> = matches.get_one::<String>("optimization-levels")
        .unwrap()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Configure verification
    let mut config = VerificationConfig {
        work_dir,
        compilation_timeout: Duration::from_secs(timeout_secs),
        execution_timeout: Duration::from_secs(60),
        keep_intermediates,
        optimization_levels: opt_levels,
        bootstrap_cycles: if quick { 2 } else { cycles },
    };

    if verbose {
        info!("Starting bootstrap verification with config: {:?}", config);
    }

    // Run verification
    let verifier = SelfCompilationVerifier::new(config);
    
    info!("🚀 Starting CURSED bootstrap verification...");
    info!("This may take several minutes depending on the configuration.");

    match verifier.run_verification() {
        Ok(report) => {
            // Generate and save report
            let report_text = generate_verification_report(&report);
            
            match std::fs::write(output_file, &report_text) {
                Ok(_) => {
                    info!("📄 Verification report saved to: {}", output_file);
                }
                Err(e) => {
                    error!("Failed to save report: {}", e);
                }
            }

            // Print summary
            print_verification_summary(&report);

            // Exit with appropriate code
            if report.overall_success {
                info!("✅ Bootstrap verification PASSED");
                std::process::exit(0);
            } else {
                error!("❌ Bootstrap verification FAILED");
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Fatal error during bootstrap verification: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_verification_summary(report: &VerificationReport) {
    println!("\n🔍 BOOTSTRAP VERIFICATION SUMMARY");
    println!("=====================================");
    
    println!("Overall Result: {}", 
        if report.overall_success { "✅ PASS" } else { "❌ FAIL" });
    
    println!("Verification Time: {:?}", report.verification_time);
    
    println!("\nCompilation Stages:");
    for result in &report.compilation_results {
        println!("  {} - {} ({})", 
            result.stage,
            if result.success { "✅" } else { "❌" },
            format_duration(result.compilation_time)
        );
    }

    if !report.bootstrap_cycle_results.is_empty() {
        println!("\nBootstrap Cycles:");
        for cycle in &report.bootstrap_cycle_results {
            println!("  Cycle {} - Convergence: {} | Binary Stable: {} | Performance Stable: {}",
                cycle.cycle_number,
                if cycle.convergence_achieved { "✅" } else { "❌" },
                if cycle.binary_stable { "✅" } else { "❌" },
                if cycle.performance_stable { "✅" } else { "❌" }
            );
        }
    }

    if !report.comparison_results.is_empty() {
        println!("\nStage Comparisons:");
        for comparison in &report.comparison_results {
            println!("  {} vs {} - Functional Equivalence: {}",
                comparison.stage1,
                comparison.stage2,
                if comparison.functional_equivalent { "✅" } else { "❌" }
            );
        }
    }

    if !report.issues_found.is_empty() {
        println!("\n⚠️  Issues Found:");
        for (i, issue) in report.issues_found.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
    }

    println!("\nPerformance Summary:");
    if !report.performance_metrics.compilation_times.is_empty() {
        println!("  Compilation Times:");
        for (stage, time) in &report.performance_metrics.compilation_times {
            println!("    {}: {}", stage, format_duration(*time));
        }
    }

    if !report.performance_metrics.binary_sizes.is_empty() {
        println!("  Binary Sizes:");
        for (stage, size) in &report.performance_metrics.binary_sizes {
            println!("    {}: {}", stage, format_bytes(*size));
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    if total_secs > 0 {
        format!("{}.{:03}s", total_secs, millis)
    } else {
        format!("{}ms", duration.as_millis())
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(2)), "2.000s");
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.500s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }
}
