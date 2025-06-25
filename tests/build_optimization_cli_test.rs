//! Comprehensive tests for the Build Optimization CLI
//! 
//! Tests all functionality of the enhanced build optimization system including
//! dependency analysis, caching, distributed compilation, analytics, memory optimization,
//! performance tuning, and optimized builds.

use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use tempfile::TempDir;

use cursed::cli::build_optimization::{
    BuildOptimizationCli, BuildOptimizationCommand, AnalyzeArgs, CacheArgs, CacheAction,
    DistributedArgs, DistributedAction, AnalyticsArgs, AnalyticsAction, MemoryArgs, MemoryAction,
    TuneArgs, OptimizedBuildArgs, run_build_optimization
};
use cursed::build_system::{CompilationUnit, DependencyOptimizer, DependencyOptimizerConfig};
use cursed::error::Result;

/// Create a temporary project with sample CURSED files for testing
fn create_test_project() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();
    
    // Create main source file
    let main_content = r#"
import "utils"
import "models"

fn main() {
    sus result = utils::calculate(42)
    facts data = models::create_user("test")
    println("Hello, CURSED!")
}
"#;
    fs::write(project_path.join("main.csd"), main_content)?;
    
    // Create utils module
    let utils_content = r#"
export fn calculate(x: i32) -> i32 {
    lowkey (x > 0) {
        return x * 2
    } highkey {
        return 0
    }
}

export fn helper() -> String {
    return "utility function"
}
"#;
    fs::write(project_path.join("utils.csd"), utils_content)?;
    
    // Create models module
    let models_content = r#"
export squad User {
    sus name: String
    sus age: i32
}

export fn create_user(name: String) -> User {
    return User {
        name: name,
        age: 25
    }
}
"#;
    fs::write(project_path.join("models.csd"), models_content)?;
    
    // Create a complex file for testing
    let complex_content = r#"
import "utils"

squad ComplexStruct {
    sus field1: String
    sus field2: i32
    sus field3: Vec<String>
}

fn complex_function() {
    sus data = Vec::new()
    
    // Complex loop with nested conditions
    lowkey (sus i = 0; i < 100; i++) {
        lowkey (i % 2 == 0) {
            data.push(format!("even: {}", i))
        } highkey (i % 3 == 0) {
            data.push(format!("multiple of 3: {}", i))
        } flex {
            data.push(format!("other: {}", i))
        }
        
        yolo // yield point
    }
    
    // Generic function with complex logic
    fn inner_generic<T: Clone + Debug>(value: T) -> T {
        lowkey (some_condition()) {
            return value.clone()
        }
        return value
    }
}

fn some_condition() -> bool {
    return true
}
"#;
    fs::write(project_path.join("complex.csd"), complex_content)?;
    
    // Create subdirectory with more files
    let subdir = project_path.join("submodule");
    fs::create_dir(&subdir)?;
    
    let sub_content = r#"
export fn sub_function() -> i32 {
    return 42
}
"#;
    fs::write(subdir.join("sub.csd"), sub_content)?;
    
    Ok(temp_dir)
}

#[test]
fn test_dependency_analysis_basic() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 4,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "json".to_string(),
            output_file: None,
            suggestions: true,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    // Should complete without errors
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Dependency analysis should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_dependency_analysis_with_report() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    let report_file = temp_dir.path().join("analysis_report.md");
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 8,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "report".to_string(),
            output_file: Some(report_file.clone()),
            suggestions: true,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Report generation should succeed: {:?}", result.err());
    
    // Check that report file was created
    assert!(report_file.exists(), "Report file should be created");
    
    let report_content = fs::read_to_string(&report_file)?;
    assert!(report_content.contains("CURSED Project Dependency Analysis Report"));
    assert!(report_content.contains("Project Overview"));
    assert!(report_content.contains("Complexity Analysis"));
    
    Ok(())
}

#[test]
fn test_cache_management() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Test cache statistics
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Cache(CacheArgs {
            action: CacheAction::Stats,
        }),
        verbose: false,
        config: None,
        project_dir: project_path.clone(),
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Cache stats should succeed: {:?}", result.err());
    
    // Test cache warming
    let files = vec![
        temp_dir.path().join("main.csd"),
        temp_dir.path().join("utils.csd"),
    ];
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Cache(CacheArgs {
            action: CacheAction::Warm { files },
        }),
        verbose: false,
        config: None,
        project_dir: project_path.clone(),
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Cache warming should succeed: {:?}", result.err());
    
    // Test cache clearing
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Cache(CacheArgs {
            action: CacheAction::Clear {
                cache_type: "all".to_string(),
            },
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Cache clearing should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_distributed_compilation_status() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Distributed(DistributedArgs {
            action: DistributedAction::Status,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    // Should complete without errors even if no distributed system is running
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Distributed status should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_analytics_report_generation() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    let report_file = temp_dir.path().join("build_report.md");
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analytics(AnalyticsArgs {
            action: AnalyticsAction::Report {
                format: "markdown".to_string(),
                output: Some(report_file.clone()),
                trends: true,
                bottlenecks: true,
            },
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Analytics report should succeed: {:?}", result.err());
    
    // Check that report file was created
    assert!(report_file.exists(), "Analytics report file should be created");
    
    let report_content = fs::read_to_string(&report_file)?;
    assert!(report_content.contains("CURSED Build Performance Report"));
    assert!(report_content.contains("Executive Summary"));
    
    Ok(())
}

#[test]
fn test_analytics_html_report() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    let report_file = temp_dir.path().join("build_report.html");
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analytics(AnalyticsArgs {
            action: AnalyticsAction::Report {
                format: "html".to_string(),
                output: Some(report_file.clone()),
                trends: false,
                bottlenecks: true,
            },
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "HTML analytics report should succeed: {:?}", result.err());
    
    // Check that HTML report file was created
    assert!(report_file.exists(), "HTML analytics report file should be created");
    
    let report_content = fs::read_to_string(&report_file)?;
    assert!(report_content.contains("<!DOCTYPE html>"));
    assert!(report_content.contains("CURSED Build Performance Report"));
    assert!(report_content.contains("<style>"));
    
    Ok(())
}

#[test]
fn test_memory_optimization() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Test memory statistics
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Memory(MemoryArgs {
            action: MemoryAction::Stats,
        }),
        verbose: false,
        config: None,
        project_dir: project_path.clone(),
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Memory stats should succeed: {:?}", result.err());
    
    // Test memory configuration
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Memory(MemoryArgs {
            action: MemoryAction::Configure {
                max_memory: Some(2048.0),
                strategy: Some("balanced".to_string()),
                streaming: true,
                chunk_size: Some(64.0),
            },
        }),
        verbose: false,
        config: None,
        project_dir: project_path.clone(),
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Memory configuration should succeed: {:?}", result.err());
    
    // Test garbage collection trigger
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Memory(MemoryArgs {
            action: MemoryAction::Gc,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Memory GC should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_performance_tuning_wizard() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Tune(TuneArgs {
            wizard: true,
            benchmark: false,
            apply_recommendations: false,
            test_config: None,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Performance tuning wizard should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_performance_benchmark() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Tune(TuneArgs {
            wizard: false,
            benchmark: true,
            apply_recommendations: false,
            test_config: None,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Performance benchmark should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_optimized_build_basic() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::OptimizedBuild(OptimizedBuildArgs {
            target: None,
            all_optimizations: false,
            dependency_optimization: true,
            advanced_caching: true,
            distributed: false,
            memory_optimization: true,
            analytics: true,
            release: false,
            jobs: Some(4),
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Optimized build should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_optimized_build_all_optimizations() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::OptimizedBuild(OptimizedBuildArgs {
            target: Some("test_target".to_string()),
            all_optimizations: true,
            dependency_optimization: false,
            advanced_caching: false,
            distributed: false,
            memory_optimization: false,
            analytics: false,
            release: true,
            jobs: Some(8),
        }),
        verbose: true,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "All-optimizations build should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_compilation_unit_collection() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Use internal function through dependency optimizer
    let config = DependencyOptimizerConfig::default();
    let optimizer = DependencyOptimizer::new(config);
    
    // This would internally call collect_compilation_units
    // We test it indirectly through the analyze command
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 2,
            smart_ordering: false,
            dependency_pruning: false,
            output_format: "text".to_string(),
            output_file: None,
            suggestions: false,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Compilation unit collection should work: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_complex_project_analysis() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Add more complex files to test edge cases
    let nested_dir = project_path.join("deeply").join("nested").join("structure");
    fs::create_dir_all(&nested_dir)?;
    
    let deeply_nested_content = r#"
import "../../utils"

export fn deeply_nested_function() -> String {
    sus result = utils::calculate(100)
    return format!("Result: {}", result)
}
"#;
    fs::write(nested_dir.join("deep.csd"), deeply_nested_content)?;
    
    // Test comprehensive analysis
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 16,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "text".to_string(),
            output_file: None,
            suggestions: true,
        }),
        verbose: true,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Complex project analysis should succeed: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_error_handling_empty_project() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let empty_project_path = temp_dir.path().to_path_buf();
    
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 4,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "json".to_string(),
            output_file: None,
            suggestions: true,
        }),
        verbose: false,
        config: None,
        project_dir: empty_project_path,
    };
    
    // Should handle empty project gracefully
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Empty project should be handled gracefully: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_output_formats() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Test JSON output
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 4,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "json".to_string(),
            output_file: None,
            suggestions: false,
        }),
        verbose: false,
        config: None,
        project_dir: project_path.clone(),
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "JSON output should work: {:?}", result.err());
    
    // Test text output
    let cli = BuildOptimizationCli {
        command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
            max_jobs: 4,
            smart_ordering: true,
            dependency_pruning: true,
            output_format: "text".to_string(),
            output_file: None,
            suggestions: true,
        }),
        verbose: false,
        config: None,
        project_dir: project_path,
    };
    
    let result = run_build_optimization(cli);
    assert!(result.is_ok(), "Text output should work: {:?}", result.err());
    
    Ok(())
}

#[test]
fn test_configuration_options() -> Result<()> {
    let temp_dir = create_test_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Test various configuration combinations
    let test_cases = vec![
        (1, false, false),   // Single threaded, no optimizations
        (4, true, false),    // Multi-threaded, smart ordering only
        (8, true, true),     // Multi-threaded, all optimizations
        (16, false, true),   // High parallelism, pruning only
    ];
    
    for (jobs, smart_ordering, dependency_pruning) in test_cases {
        let cli = BuildOptimizationCli {
            command: BuildOptimizationCommand::Analyze(AnalyzeArgs {
                max_jobs: jobs,
                smart_ordering,
                dependency_pruning,
                output_format: "text".to_string(),
                output_file: None,
                suggestions: false,
            }),
            verbose: false,
            config: None,
            project_dir: project_path.clone(),
        };
        
        let result = run_build_optimization(cli);
        assert!(result.is_ok(), 
            "Configuration jobs={}, smart={}, pruning={} should work: {:?}", 
            jobs, smart_ordering, dependency_pruning, result.err()
        );
    }
    
    Ok(())
}
