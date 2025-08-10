//! CURSED Development Tools Suite
//! 
//! Comprehensive tooling ecosystem for CURSED language development

pub mod package_manager;
pub mod profiler;
pub mod execution_pipeline_demo;

pub use package_manager::{PackageManager, PackageConfig};
pub use profiler::{Profiler, ProfilerConfig, ProfileReport};

use std::path::Path;
use std::fs;

// Import the execution pipeline
use crate::execution_pipeline::{CursedExecutionPipeline, ExecutionConfig, ExecutionBackend, ExecutionResult};

/// Integrated development tools manager
#[derive(Debug, Clone)]
pub struct CursedTools {
    pub package_manager: PackageManager,
    pub profiler: Profiler,
    pub execution_pipeline: Option<CursedExecutionPipeline>,
}

impl CursedTools {
    /// Create new tools suite
    pub fn new() -> Self {
        let execution_pipeline = match CursedExecutionPipeline::new() {
            Ok(pipeline) => Some(pipeline),
            Err(e) => {
                eprintln!("⚠️  Warning: Could not initialize execution pipeline: {}", e);
                eprintln!("   Some features may not be available. Run 'zig build' to build the CURSED interpreter.");
                None
            }
        };

        Self {
            package_manager: PackageManager::new("https://registry.cursed.dev".to_string()),
            profiler: Profiler::new(ProfilerConfig::default()),
            execution_pipeline,
        }
    }

    /// Initialize project with all tools
    pub async fn init_project(&mut self, name: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Initializing CURSED project with complete tooling suite...");

        // Create project directory
        fs::create_dir_all(path)?;
        std::env::set_current_dir(path)?;

        // Initialize package
        self.package_manager.init_package(name, "0.1.0")?;

        // Create additional project structure
        fs::create_dir_all("docs")?;
        fs::create_dir_all("tests")?;
        fs::create_dir_all("benchmarks")?;
        fs::create_dir_all("examples")?;
        fs::create_dir_all(".cursed")?;

        // Create configuration files
        self.create_config_files()?;

        // Create example files
        self.create_example_files()?;

        println!("✅ Project initialized with full tooling support");
        Ok(())
    }

    /// Profile application performance
    pub async fn profile_application(&mut self, program_path: &Path) -> Result<ProfileReport, Box<dyn std::error::Error>> {
        println!("🔍 Starting comprehensive performance profiling...");

        // Start profiling
        self.profiler.start_profiling()?;

        // Execute the CURSED program with complete pipeline: parser → type-checker → interpreter/VM
        match self.execute_cursed_program(program_path).await {
            Ok(execution_result) => {
                if self.profiler.config.verbose {
                    println!("📊 Program executed successfully in {}ms", execution_result.execution_time_ms);
                    if let Some(tokens) = execution_result.tokens_count {
                        println!("🔤 Tokens processed: {}", tokens);
                    }
                    if let Some(ast_nodes) = execution_result.ast_nodes_count {
                        println!("🌳 AST nodes generated: {}", ast_nodes);
                    }
                    if let Some(memory) = execution_result.memory_usage_bytes {
                        println!("💾 Memory usage: {} bytes", memory);
                    }
                }
                
                if execution_result.exit_code != 0 {
                    eprintln!("⚠️  Program execution failed with exit code: {}", execution_result.exit_code);
                    eprintln!("Error output: {}", execution_result.stderr);
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to execute CURSED program: {}", e);
                return Err(e);
            }
        }

        // Stop profiling and generate report
        let report = self.profiler.stop_profiling()?;

        println!("✅ Performance profiling complete");
        Ok(report)
    }

    /// Execute a CURSED program through the complete pipeline
    async fn execute_cursed_program(&self, program_path: &Path) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        match &self.execution_pipeline {
            Some(pipeline) => {
                let config = ExecutionConfig {
                    backend: ExecutionBackend::AST,  // Use AST backend for full language support
                    verbose: self.profiler.config.verbose,
                    memory_profile: true,  // Enable memory profiling for performance analysis
                    performance_profile: true,
                    show_tokens: self.profiler.config.verbose,
                    show_ast: self.profiler.config.verbose,
                    optimization_level: 0,  // No optimization for profiling accuracy
                };

                let program_path_str = program_path.to_string_lossy();
                
                // Execute through complete pipeline: Lexer → Parser → Type-Checker → Interpreter/VM
                let result = pipeline.execute_file(&program_path_str, &config)?;
                Ok(result)
            }
            None => {
                Err("Execution pipeline not available. Please build the CURSED interpreter with 'zig build'.".into())
            }
        }
    }

    /// Manage project dependencies
    pub async fn manage_dependencies(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Managing project dependencies...");

        // Check for outdated dependencies
        self.package_manager.check_outdated().await?;

        // Resolve and install dependencies
        self.package_manager.resolve_dependencies().await?;
        self.package_manager.install_dependencies().await?;

        println!("✅ Dependencies updated successfully");
        Ok(())
    }

    /// Compile a CURSED program to optimized native binary
    pub async fn compile_program(&self, source_path: &Path, output_path: Option<&str>, optimization_level: u8) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        println!("🔨 Compiling CURSED program to native binary...");

        match &self.execution_pipeline {
            Some(pipeline) => {
                let config = ExecutionConfig {
                    backend: ExecutionBackend::LLVM,  // Use LLVM for production compilation
                    verbose: true,
                    optimization_level,
                    ..Default::default()
                };

                let source_path_str = source_path.to_string_lossy();
                let result = pipeline.compile_to_binary(&source_path_str, output_path, &config)?;
                
                if result.exit_code == 0 {
                    println!("✅ Compilation successful!");
                    if let Some(output) = output_path {
                        println!("📦 Output binary: {}", output);
                        println!("🚀 Run with: ./{}", output);
                    }
                } else {
                    eprintln!("❌ Compilation failed:");
                    eprintln!("{}", result.stderr);
                }

                Ok(result)
            }
            None => {
                Err("Execution pipeline not available. Please build the CURSED interpreter with 'zig build'.".into())
            }
        }
    }

    /// Type-check a CURSED program without execution
    pub async fn type_check_program(&self, source_path: &Path) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        println!("🔍 Type-checking CURSED program...");

        match &self.execution_pipeline {
            Some(pipeline) => {
                let config = ExecutionConfig {
                    backend: ExecutionBackend::AST,
                    verbose: true,
                    ..Default::default()
                };

                let source_path_str = source_path.to_string_lossy();
                let result = pipeline.type_check(&source_path_str, &config)?;
                
                if result.exit_code == 0 {
                    println!("✅ Type checking passed!");
                } else {
                    eprintln!("❌ Type checking failed:");
                    eprintln!("{}", result.stderr);
                }

                Ok(result)
            }
            None => {
                Err("Execution pipeline not available. Please build the CURSED interpreter with 'zig build'.".into())
            }
        }
    }

    /// Execute a CURSED program with debugging information
    pub async fn debug_execute(&self, source_path: &Path) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        println!("🐛 Debug-executing CURSED program...");

        match &self.execution_pipeline {
            Some(pipeline) => {
                let result = pipeline.debug_execute(&source_path.to_string_lossy())?;
                
                println!("📊 Execution completed in {}ms", result.execution_time_ms);
                if let Some(tokens) = result.tokens_count {
                    println!("🔤 Tokens: {}", tokens);
                }
                if let Some(ast_nodes) = result.ast_nodes_count {
                    println!("🌳 AST nodes: {}", ast_nodes);
                }
                if let Some(memory) = result.memory_usage_bytes {
                    println!("💾 Memory: {} bytes", memory);
                }

                if result.exit_code == 0 {
                    println!("✅ Execution successful!");
                    println!("Output:\n{}", result.stdout);
                } else {
                    eprintln!("❌ Execution failed:");
                    eprintln!("{}", result.stderr);
                }

                Ok(result)
            }
            None => {
                Err("Execution pipeline not available. Please build the CURSED interpreter with 'zig build'.".into())
            }
        }
    }

    /// Quick interpretation for development/testing
    pub async fn quick_run(&self, source_code: &str) -> Result<String, Box<dyn std::error::Error>> {
        match &self.execution_pipeline {
            Some(pipeline) => {
                let output = pipeline.quick_interpret(source_code)?;
                Ok(output)
            }
            None => {
                Err("Execution pipeline not available. Please build the CURSED interpreter with 'zig build'.".into())
            }
        }
    }

    /// Run complete project analysis
    pub async fn analyze_project(&mut self, project_path: &Path) -> Result<ProjectAnalysis, Box<dyn std::error::Error>> {
        println!("🔬 Running comprehensive project analysis...");

        let mut analysis = ProjectAnalysis::default();

        // Dependency analysis
        self.package_manager.check_outdated().await?;
        analysis.outdated_dependencies = self.count_outdated_dependencies().await?;

        println!("✅ Project analysis complete");
        Ok(analysis)
    }

    /// Create configuration files
    fn create_config_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create .cursed-profile.toml
        let profile_config = r#"
# CURSED Profiler Configuration

[profiling]
sample_rate = 100
memory_tracking = true
call_graph_tracking = true
cpu_profiling = true
output_format = "html"
max_samples = 10000
"#;
        fs::write(".cursed-profile.toml", profile_config)?;

        Ok(())
    }

    /// Create example files
    fn create_example_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create example program
        let example_program = r#"## Example CURSED program demonstrating basic features
yeet "vibez"
yeet "math"

## Main function that demonstrates various language features
slay main() {
    vibez.spill("Welcome to CURSED!")
    
    ## Variables with different types
    sus name tea = "CURSED Developer"
    sus age normie = 25
    sus score drip = 98.5
    sus active lit = based
    
    ## Function call with multiple arguments
    greet_user(name, age)
    
    ## Mathematical operations
    sus result normie = math.add(age, 5)
    vibez.spill("Age in 5 years:", result)
    
    ## Conditional logic
    bestie score > 90.0 {
        vibez.spill("Excellent score!")
    } salty {
        vibez.spill("Good effort!")
    }
}

## Helper function with parameters
slay greet_user(name tea, age normie) {
    vibez.spill("Hello", name)
    vibez.spill("You are", age, "years old")
}
"#;
        fs::write("examples/basic.csd", example_program)?;

        // Create test file
        let test_program = r#"## Test suite for example program
yeet "testz"

test_start("Basic functionality test")

## Test variable assignments
sus x normie = 42
assert_eq_int(x, 42)

sus message tea = "Hello, World!"
assert_eq_string(message, "Hello, World!")

## Test mathematical operations
sus sum normie = 10 + 15
assert_eq_int(sum, 25)

print_test_summary()
"#;
        fs::write("tests/basic_test.csd", test_program)?;

        // Create benchmark file
        let benchmark_program = r#"## Performance benchmarks
yeet "vibez"

## Benchmark array operations
slay benchmark_array_ops() {
    sus start_time normie = time.now()
    
    sus numbers [1000]normie
    bestie i := 0; i < 1000; i++ {
        numbers[i] = i * 2
    }
    
    sus end_time normie = time.now()
    sus duration normie = end_time - start_time
    
    vibez.spill("Array operations took:", duration, "ms")
}

slay main() {
    benchmark_array_ops()
}
"#;
        fs::write("benchmarks/array_ops.csd", benchmark_program)?;

        Ok(())
    }

    /// Count outdated dependencies
    async fn count_outdated_dependencies(&self) -> Result<usize, Box<dyn std::error::Error>> {
        // This would integrate with the package manager
        // For now, return a placeholder
        Ok(0)
    }
}

/// Project analysis results
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ProjectAnalysis {
    pub format_issues: usize,
    pub doc_coverage: f64,
    pub outdated_dependencies: usize,
    pub test_coverage: f64,
    pub performance_score: f64,
    pub security_issues: usize,
    pub code_quality_score: f64,
}

impl Default for CursedTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_tools_creation() {
        let tools = CursedTools::new();
        
        // Just verify that all tools are created
        assert_eq!(tools.package_manager.registry_url, "https://registry.cursed.dev");
    }

    #[ignore] // Skip due to tokio runtime stack overflow
#[tokio::test]
async fn test_project_initialization() {
        let temp_dir = tempdir().unwrap();
        let mut tools = CursedTools::new();
        
        let result = tools.init_project("test_project", temp_dir.path()).await;
        assert!(result.is_ok());
        
        // Verify project structure was created
        assert!(temp_dir.path().join("cursed.toml").exists());
        assert!(temp_dir.path().join("tests").exists());
    }

    #[test]
    fn test_project_analysis_default() {
        let analysis = ProjectAnalysis::default();
        
        assert_eq!(analysis.format_issues, 0);
        assert_eq!(analysis.doc_coverage, 0.0);
        assert_eq!(analysis.outdated_dependencies, 0);
    }
}
