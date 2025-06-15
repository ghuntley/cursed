//! Optimization CLI Commands Test Suite
//! 
//! Comprehensive tests for the optimization CLI commands including
//! command parsing, configuration management, and integration testing.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use serde_json;

use cursed::cli::optimization_commands::{
    OptimizationCliConfig, BenchmarkConfig, ProfilingConfig,
    add_optimization_commands, handle_optimization_command,
};
use cursed::optimization::OptimizationLevel;

/// Test helper to create a temporary CURSED source file
fn create_test_source_file(temp_dir: &TempDir, content: &str) -> PathBuf {
    let file_path = temp_dir.path().join("test.csd");
    fs::write(&file_path, content).unwrap();
    file_path
}

/// Test helper to create a temporary configuration file
fn create_test_config(temp_dir: &TempDir, config: &OptimizationCliConfig) -> PathBuf {
    let config_path = temp_dir.path().join("optimization.json");
    let json = serde_json::to_string_pretty(config).unwrap();
    fs::write(&config_path, json).unwrap();
    config_path
}

#[test]
fn test_optimization_cli_config_default() {
    let config = OptimizationCliConfig::default();
    
    assert_eq!(config.default_level, OptimizationLevel::O2);
    assert!(config.enabled_passes.contains(&"inline".to_string()));
    assert!(config.enabled_passes.contains(&"dce".to_string()));
    assert!(config.enabled_passes.contains(&"mem2reg".to_string()));
    assert!(config.enabled_passes.contains(&"gvn".to_string()));
    assert!(config.disabled_passes.is_empty());
    assert!(config.custom_params.is_empty());
    
    // Test benchmark config defaults
    assert_eq!(config.benchmark_config.iterations, 5);
    assert_eq!(config.benchmark_config.timeout_seconds, 300);
    assert_eq!(config.benchmark_config.warmup_iterations, 2);
    assert!(config.benchmark_config.test_files.is_empty());
    
    // Test profiling config defaults
    assert!(config.profiling_config.detailed_timing);
    assert!(!config.profiling_config.memory_tracking);
    assert_eq!(config.profiling_config.sample_rate, 1000);
    assert_eq!(config.profiling_config.output_format, "markdown");
}

#[test]
fn test_optimization_cli_config_serialization() {
    let config = OptimizationCliConfig::default();
    
    // Test JSON serialization
    let json = serde_json::to_string_pretty(&config).unwrap();
    assert!(json.contains("default_level"));
    assert!(json.contains("enabled_passes"));
    assert!(json.contains("benchmark_config"));
    assert!(json.contains("profiling_config"));
    
    // Test JSON deserialization
    let deserialized: OptimizationCliConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.default_level, config.default_level);
    assert_eq!(deserialized.enabled_passes, config.enabled_passes);
    assert_eq!(deserialized.disabled_passes, config.disabled_passes);
}

#[test]
fn test_benchmark_config_validation() {
    let mut config = BenchmarkConfig {
        iterations: 10,
        timeout_seconds: 60,
        warmup_iterations: 3,
        test_files: vec![
            PathBuf::from("test1.csd"),
            PathBuf::from("test2.csd"),
        ],
    };
    
    assert_eq!(config.iterations, 10);
    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.warmup_iterations, 3);
    assert_eq!(config.test_files.len(), 2);
    
    // Test configuration modification
    config.iterations = 20;
    config.timeout_seconds = 120;
    assert_eq!(config.iterations, 20);
    assert_eq!(config.timeout_seconds, 120);
}

#[test]
fn test_profiling_config_validation() {
    let mut config = ProfilingConfig {
        detailed_timing: true,
        memory_tracking: true,
        sample_rate: 2000,
        output_format: "json".to_string(),
    };
    
    assert!(config.detailed_timing);
    assert!(config.memory_tracking);
    assert_eq!(config.sample_rate, 2000);
    assert_eq!(config.output_format, "json");
    
    // Test configuration modification
    config.detailed_timing = false;
    config.sample_rate = 500;
    assert!(!config.detailed_timing);
    assert_eq!(config.sample_rate, 500);
}

#[test]
fn test_optimization_level_parsing() {
    // Test valid optimization levels
    assert_eq!("0".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O0);
    assert_eq!("1".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O1);
    assert_eq!("2".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O2);
    assert_eq!("3".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::O3);
    assert_eq!("s".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Os);
    assert_eq!("z".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Oz);
    
    // Test invalid optimization levels
    assert!("4".parse::<OptimizationLevel>().is_err());
    assert!("invalid".parse::<OptimizationLevel>().is_err());
    assert!("".parse::<OptimizationLevel>().is_err());
}

#[test]
fn test_add_optimization_commands_structure() {
    use clap::Command;
    
    // Create base command and add optimization subcommands
    let cmd = add_optimization_commands(Command::new("optimize"));
    
    // Test that all expected subcommands are present
    let subcommands: Vec<&str> = cmd.get_subcommands()
        .map(|sc| sc.get_name())
        .collect();
    
    assert!(subcommands.contains(&"analyze"));
    assert!(subcommands.contains(&"benchmark"));
    assert!(subcommands.contains(&"profile"));
    assert!(subcommands.contains(&"enable"));
    assert!(subcommands.contains(&"disable"));
    assert!(subcommands.contains(&"config"));
    assert!(subcommands.contains(&"reset"));
}

#[test]
fn test_analyze_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    let analyze_cmd = cmd.find_subcommand("analyze").unwrap();
    
    // Test required arguments
    let file_arg = analyze_cmd.find_arg("file").unwrap();
    assert!(file_arg.is_required_set());
    assert_eq!(file_arg.get_value_names().unwrap()[0], "FILE");
    
    // Test optional arguments
    assert!(analyze_cmd.find_arg("output").is_some());
    assert!(analyze_cmd.find_arg("format").is_some());
    assert!(analyze_cmd.find_arg("detailed").is_some());
    assert!(analyze_cmd.find_arg("suggestions").is_some());
}

#[test]
fn test_benchmark_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    let benchmark_cmd = cmd.find_subcommand("benchmark").unwrap();
    
    // Test required arguments
    let file_arg = benchmark_cmd.find_arg("file").unwrap();
    assert!(file_arg.is_required_set());
    
    // Test optional arguments with defaults
    let levels_arg = benchmark_cmd.find_arg("levels").unwrap();
    assert_eq!(levels_arg.get_default_values()[0], "0,1,2,3,s,z");
    
    let iterations_arg = benchmark_cmd.find_arg("iterations").unwrap();
    assert_eq!(iterations_arg.get_default_values()[0], "5");
    
    let timeout_arg = benchmark_cmd.find_arg("timeout").unwrap();
    assert_eq!(timeout_arg.get_default_values()[0], "300");
    
    // Test flags
    assert!(benchmark_cmd.find_arg("parallel").is_some());
}

#[test]
fn test_profile_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    let profile_cmd = cmd.find_subcommand("profile").unwrap();
    
    // Test required arguments
    let file_arg = profile_cmd.find_arg("file").unwrap();
    assert!(file_arg.is_required_set());
    
    // Test optional arguments with defaults
    let opt_level_arg = profile_cmd.find_arg("opt-level").unwrap();
    assert_eq!(opt_level_arg.get_default_values()[0], "2");
    
    let sample_rate_arg = profile_cmd.find_arg("sample-rate").unwrap();
    assert_eq!(sample_rate_arg.get_default_values()[0], "1000");
    
    // Test flags
    assert!(profile_cmd.find_arg("phases").is_some());
    assert!(profile_cmd.find_arg("memory").is_some());
    assert!(profile_cmd.find_arg("flamegraph").is_some());
}

#[test]
fn test_enable_disable_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    
    // Test enable command
    let enable_cmd = cmd.find_subcommand("enable").unwrap();
    let passes_arg = enable_cmd.find_arg("passes").unwrap();
    assert!(passes_arg.is_required_set());
    assert!(enable_cmd.find_arg("global").is_some());
    assert!(enable_cmd.find_arg("project").is_some());
    
    // Test disable command
    let disable_cmd = cmd.find_subcommand("disable").unwrap();
    let passes_arg = disable_cmd.find_arg("passes").unwrap();
    assert!(passes_arg.is_required_set());
    assert!(disable_cmd.find_arg("global").is_some());
    assert!(disable_cmd.find_arg("project").is_some());
}

#[test]
fn test_config_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    let config_cmd = cmd.find_subcommand("config").unwrap();
    
    // Test all config arguments
    assert!(config_cmd.find_arg("show").is_some());
    assert!(config_cmd.find_arg("set").is_some());
    assert!(config_cmd.find_arg("unset").is_some());
    assert!(config_cmd.find_arg("default-level").is_some());
    assert!(config_cmd.find_arg("global").is_some());
    assert!(config_cmd.find_arg("export").is_some());
    assert!(config_cmd.find_arg("import").is_some());
}

#[test]
fn test_reset_command_arguments() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    let reset_cmd = cmd.find_subcommand("reset").unwrap();
    
    // Test reset command arguments
    assert!(reset_cmd.find_arg("global").is_some());
    assert!(reset_cmd.find_arg("project").is_some());
    assert!(reset_cmd.find_arg("confirm").is_some());
}

#[test]
fn test_config_modification() {
    let mut config = OptimizationCliConfig::default();
    
    // Test enabling new passes
    let new_pass = "vectorize".to_string();
    assert!(!config.enabled_passes.contains(&new_pass));
    config.enabled_passes.push(new_pass.clone());
    assert!(config.enabled_passes.contains(&new_pass));
    
    // Test disabling passes
    let pass_to_disable = config.enabled_passes[0].clone();
    config.enabled_passes.retain(|p| p != &pass_to_disable);
    config.disabled_passes.push(pass_to_disable.clone());
    assert!(!config.enabled_passes.contains(&pass_to_disable));
    assert!(config.disabled_passes.contains(&pass_to_disable));
    
    // Test custom parameter modification
    config.custom_params.insert("llvm_opt_level".to_string(), "3".to_string());
    assert_eq!(config.custom_params.get("llvm_opt_level"), Some(&"3".to_string()));
    
    // Test default level modification
    config.default_level = OptimizationLevel::O3;
    assert_eq!(config.default_level, OptimizationLevel::O3);
}

#[test]
fn test_benchmark_config_modification() {
    let mut config = BenchmarkConfig {
        iterations: 5,
        timeout_seconds: 300,
        warmup_iterations: 2,
        test_files: vec![],
    };
    
    // Test adding test files
    config.test_files.push(PathBuf::from("test1.csd"));
    config.test_files.push(PathBuf::from("test2.csd"));
    assert_eq!(config.test_files.len(), 2);
    
    // Test modifying parameters
    config.iterations = 10;
    config.timeout_seconds = 600;
    config.warmup_iterations = 5;
    
    assert_eq!(config.iterations, 10);
    assert_eq!(config.timeout_seconds, 600);
    assert_eq!(config.warmup_iterations, 5);
}

#[test]
fn test_profiling_config_modification() {
    let mut config = ProfilingConfig {
        detailed_timing: true,
        memory_tracking: false,
        sample_rate: 1000,
        output_format: "markdown".to_string(),
    };
    
    // Test toggling boolean flags
    config.memory_tracking = true;
    config.detailed_timing = false;
    assert!(config.memory_tracking);
    assert!(!config.detailed_timing);
    
    // Test changing numeric values
    config.sample_rate = 2000;
    assert_eq!(config.sample_rate, 2000);
    
    // Test changing output format
    config.output_format = "json".to_string();
    assert_eq!(config.output_format, "json");
}

#[test] 
fn test_configuration_edge_cases() {
    let mut config = OptimizationCliConfig::default();
    
    // Test duplicate pass handling
    let duplicate_pass = "inline".to_string();
    let initial_len = config.enabled_passes.len();
    config.enabled_passes.push(duplicate_pass.clone());
    assert_eq!(config.enabled_passes.len(), initial_len + 1);
    
    // Test removing non-existent pass
    let initial_disabled_len = config.disabled_passes.len();
    config.disabled_passes.retain(|p| p != "non_existent_pass");
    assert_eq!(config.disabled_passes.len(), initial_disabled_len);
    
    // Test empty custom parameters
    assert!(config.custom_params.is_empty());
    config.custom_params.insert("test".to_string(), "value".to_string());
    assert!(!config.custom_params.is_empty());
    config.custom_params.clear();
    assert!(config.custom_params.is_empty());
}

#[test]
fn test_optimization_level_edge_cases() {
    // Test all valid optimization levels
    let valid_levels = vec!["0", "1", "2", "3", "s", "z"];
    for level_str in valid_levels {
        assert!(level_str.parse::<OptimizationLevel>().is_ok());
    }
    
    // Test invalid optimization levels
    let invalid_levels = vec!["4", "10", "a", "fast", ""];
    for level_str in invalid_levels {
        assert!(level_str.parse::<OptimizationLevel>().is_err());
    }
    
    // Test case sensitivity
    assert!("S".parse::<OptimizationLevel>().is_err()); // Should be lowercase 's'
    assert!("Z".parse::<OptimizationLevel>().is_err()); // Should be lowercase 'z'
}

#[test]
fn test_help_text_generation() {
    use clap::Command;
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    
    // Test that help can be generated without panicking
    let help_output = cmd.render_help();
    let help_str = help_output.to_string();
    
    // Verify key sections are present in help
    assert!(help_str.contains("analyze"));
    assert!(help_str.contains("benchmark"));
    assert!(help_str.contains("profile"));
    assert!(help_str.contains("enable"));
    assert!(help_str.contains("disable"));
    assert!(help_str.contains("config"));
    assert!(help_str.contains("reset"));
}

#[test]
fn test_argument_validation() {
    use clap::{Command, ArgMatches};
    
    let cmd = add_optimization_commands(Command::new("optimize"));
    
    // Test valid analyze command
    let analyze_args = vec!["optimize", "analyze", "test.csd", "--format", "json"];
    let matches = cmd.clone().try_get_matches_from(analyze_args);
    assert!(matches.is_ok());
    
    // Test invalid analyze command (missing required file)
    let invalid_analyze_args = vec!["optimize", "analyze", "--format", "json"];
    let invalid_matches = cmd.clone().try_get_matches_from(invalid_analyze_args);
    assert!(invalid_matches.is_err());
    
    // Test valid benchmark command
    let benchmark_args = vec!["optimize", "benchmark", "test.csd", "--levels", "1,2,3"];
    let matches = cmd.clone().try_get_matches_from(benchmark_args);
    assert!(matches.is_ok());
    
    // Test valid config command
    let config_args = vec!["optimize", "config", "--show"];
    let matches = cmd.clone().try_get_matches_from(config_args);
    assert!(matches.is_ok());
}

#[test]
fn test_command_aliases() {
    use clap::Command;
    
    // Test that the optimization command can be accessed via alias
    let main_cmd = Command::new("cursed")
        .subcommand(
            add_optimization_commands(Command::new("optimize"))
                .alias("opt")
        );
    
    // Test full command name
    let full_args = vec!["cursed", "optimize", "analyze", "test.csd"];
    let matches = main_cmd.clone().try_get_matches_from(full_args);
    assert!(matches.is_ok());
    
    // Test alias
    let alias_args = vec!["cursed", "opt", "analyze", "test.csd"];
    let matches = main_cmd.clone().try_get_matches_from(alias_args);
    assert!(matches.is_ok());
}

/// Integration test for configuration file handling
#[test]
fn test_configuration_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test configuration
    let mut config = OptimizationCliConfig::default();
    config.default_level = OptimizationLevel::O3;
    config.enabled_passes.push("vectorize".to_string());
    config.custom_params.insert("test_param".to_string(), "test_value".to_string());
    
    // Save configuration to file
    let config_path = create_test_config(&temp_dir, &config);
    assert!(config_path.exists());
    
    // Load configuration from file
    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: OptimizationCliConfig = serde_json::from_str(&loaded_content).unwrap();
    
    // Verify configuration was loaded correctly
    assert_eq!(loaded_config.default_level, OptimizationLevel::O3);
    assert!(loaded_config.enabled_passes.contains(&"vectorize".to_string()));
    assert_eq!(loaded_config.custom_params.get("test_param"), Some(&"test_value".to_string()));
}

/// Test that demonstrates the expected workflow
#[test]
fn test_optimization_workflow() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test CURSED source file
    let cursed_source = r#"
        func main() {
            sus x = 42;
            sus y = x * 2;
            println("Result: " + y.toString());
        }
    "#;
    let source_file = create_test_source_file(&temp_dir, cursed_source);
    
    // Create initial configuration
    let mut config = OptimizationCliConfig::default();
    
    // Simulate enabling optimization passes
    config.enabled_passes.push("loop-unroll".to_string());
    config.enabled_passes.push("vectorize".to_string());
    
    // Simulate configuring benchmark settings
    config.benchmark_config.iterations = 10;
    config.benchmark_config.timeout_seconds = 120;
    
    // Simulate configuring profiling settings
    config.profiling_config.detailed_timing = true;
    config.profiling_config.memory_tracking = true;
    config.profiling_config.sample_rate = 2000;
    
    // Save configuration
    let config_path = create_test_config(&temp_dir, &config);
    
    // Verify the complete workflow setup
    assert!(source_file.exists());
    assert!(config_path.exists());
    assert_eq!(config.enabled_passes.len(), 6); // 4 default + 2 added
    assert!(config.enabled_passes.contains(&"loop-unroll".to_string()));
    assert!(config.enabled_passes.contains(&"vectorize".to_string()));
}
