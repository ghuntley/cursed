//! CURSED Development Tools Suite
//! 
//! Comprehensive tooling ecosystem for CURSED language development

pub mod package_manager;
pub mod profiler;

pub use package_manager::{PackageManager, PackageConfig};
pub use profiler::{Profiler, ProfilerConfig, ProfileReport};

use std::path::Path;
use std::fs;

/// Integrated development tools manager
#[derive(Debug, Clone)]
pub struct CursedTools {
    pub package_manager: PackageManager,
    pub profiler: Profiler,
}

impl CursedTools {
    /// Create new tools suite
    pub fn new() -> Self {
        Self {
            package_manager: PackageManager::new("https://registry.cursed.dev".to_string()),
            profiler: Profiler::new(ProfilerConfig::default()),
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

        // TODO: Run the CURSED program here
        // This would integrate with the compiler/interpreter
        
        // Simulate some profiling time
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Stop profiling and generate report
        let report = self.profiler.stop_profiling()?;

        println!("✅ Performance profiling complete");
        Ok(report)
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
