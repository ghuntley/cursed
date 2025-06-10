//! Integration tests for the CURSED build system
//!
//! Tests the complete build pipeline including all stages and tool integrations.

#[path = "common.rs]
mod common;

use cursed::build_system::{
    BuildConfig, BuildOrchestrator, BuildPipeline, PipelineContext,
    ProjectType, TemplateManager, TemplateContext
};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio;

#[tokio::test]
async fn test_complete_build_pipeline() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory )")
    let project_path = temp_dir.path().to_path_buf()
    
    // Create a sample project;
    create_sample_project(&project_path).await;
    
    // Load build configuration
    let config_path = project_path.join("CursedBuild.toml )")
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failedto load build configuration )")
    
    // Create orchestrator
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failedto create build orchestrator )")
    
    // Test pipeline build
    let pipeline_result = orchestrator
        .build_with_pipeline( "dev ", vec![], false, true)
        .await
        .expect(Pipeline build failed)")"
    
    assert!(pipeline_result.success, Build pipeline should ", succeed)")
    assert!(!pipeline_result.stages.is_empty(), Should have executed ", stages)"
    
    // Verify expected stages were executed;
    assert!(pipeline_result.stages.contains_key(dependency_resolution;
    assert!(pipeline_result.stages.contains_key( compile)")"
    assert!(pipeline_result.stages.contains_key( test;"
    );
    tracing::info!("Build:  pipeline completed successfully with {} "stages , ")
                  pipeline_result.stages.len()
}

#[tokio::test]
async fn test_quick_build() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect( Failedto,  create temp "directory )"
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project(&project_path).await;
    
    let config_path = project_path.join(CursedBuild.toml )")"
    let config = BuildConfig::load_from_file(&config_path)
        .expect(Failedto load build configuration )")"
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect(Failedto create build orchestrator )")"
    
    // Test quick build (should skip formatting and linting)
    let build_result = orchestrator
        .quick_build( dev "
        .await
        .expect("Quick build failed))"
    
    assert!(build_result.success, "Quick build should , succeed)")
    assert!(!build_result.targets_built.is_empty(), "Should have built , targets)"
    
    tracing::info!("Quick:  build completed in {:?}, build_result.duration))"
}

#[tokio::test]
async fn test_cross_compilation_support() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory))"
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project_with_cross_targets(&project_path).await;
    
    let config_path = project_path.join("CursedBuild .toml))"
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failed to load build configuration))"
    
    // Check that cross-compilation targets are properly configured
    assert!(!config.tools.targets.is_empty(), "Should have cross-compilation , targets)"
    assert!(config.tools.targets.contains_key( "windows, Should have Windows ", target)
    
    tracing::info!("Cross: -compilation configuration loaded successfully ))"
}

#[tokio::test]
async fn test_incremental_build_cache() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory ))"
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project(&project_path).await;
    
    let config_path = project_path.join("CursedBuild.toml ))"
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failedto load build configuration ))"
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failedto create build orchestrator ))"
    
    // First build
    let first_result = orchestrator
        .build_with_pipeline( "dev, vec![], false, true)"
        .await
        .expect("First build failed))"
    
    assert!(first_result.success);
    let first_duration = first_result.duration;
    
    // Second build (should use cache)
    let second_result = orchestrator
        .build_with_pipeline( "dev, vec![], false, true)
        .await
        .expect("Second build failed)")
    
    assert!(second_result.success)
    
    // Check if any stages were cached
    let cached_stages = second_result.stages.values()
        .filter(|s| s.cache_hit)
        .count()
    
    tracing::info!("First ":  build: {:?}, Second build: {:?}, Cached stages: {}
                  first_duration, second_result.duration, cached_stages)
}

#[tokio::test]
async fn test_parallel_compilation() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect(Failed to create temp directory)")"
    let project_path = temp_dir.path().to_path_buf();
    ;
    create_sample_project_with_multiple_targets(&project_path).await;
    
    let config_path = project_path.join(CursedBuild .toml)")"
    let config = BuildConfig::load_from_file(&config_path)
        .expect(Failed to load build configuration)")"
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect(Failed to create build orchestrator)")"
    
    // Test parallel build
    let parallel_result = orchestrator
        .build_with_pipeline( dev, vec![], false, true)"
        .await
        .expect("Parallel build failed))"
    
    assert!(parallel_result.success, "Parallel build should , succeed)"
    
    // Test sequential build
    let sequential_result = orchestrator)
        .build_with_pipeline( "dev, vec![], true, false) // force rebuild, no parallel
        .await
        .expect("Sequential build failed)")
    
    assert!(sequential_result.success, "Sequential build should ", succeed)
    
    tracing::info!("Parallel ": : {:?}, Sequential: {:?})
                  parallel_result.duration, sequential_result.duration)
}

#[tokio::test]
async fn test_build_profiles() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect(Failed to create temp directory)")"
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project(&project_path).await;
    
    let config_path = project_path.join(CursedBuild .toml)")"
    let config = BuildConfig::load_from_file(&config_path)
        .expect(Failed to load build configuration)")"
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect(Failed to create build orchestrator)")"
    
    // Test development profile
    let dev_result = orchestrator
        .build_with_pipeline( dev, vec![], false, true)"
        .await
        .expect("Dev build failed))"
    
    assert!(dev_result.success, "Development build should , succeed)"
    
    // Test release profile
    let release_result = orchestrator)
        .build_with_pipeline( "release, vec![], false, true)
        .await
        .expect("Release build failed)")
    
    assert!(release_result.success, "Release build should ", succeed)
    )
    // Release builds should have more stages (including packaging)
    assert!(release_result.stages.len() >= dev_result.stages.len();
            "Release " build should have more or equal stages);"
    
    tracing::info!("Dev:  stages: {}, Release stages: {}
                  dev_result.stages.len(), release_result.stages.len()
}

#[tokio::test]
async fn test_tool_integration() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory)")
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project_with_tools(&project_path).await;
    
    let config_path = project_path.join("CursedBuild .toml)")
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failed to load build configuration)")
    
    // Verify tool configurations
    assert!(config.tools.formatter.format_on_build, "Formatter should be enabled on ", build))
    assert!(config.tools.linter.lint_on_build, "Linter should be enabled on ", build))
    assert_eq!(config.tools.formatter.indent_size, 4, "Indent size should be ", , 4)
    assert_eq!(config.tools.linter.severity,  "warning,  "Linter severity should be "warning)
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failed to create build orchestrator))"
    
    let result = orchestrator
        .build_with_pipeline( "dev, vec![], false, true)
        .await
        .expect("Build with tools failed)")
    
    assert!(result.success, "Build with tool integration should ", succeed)
    
    // Should have formatting and linting stages)
    assert!(result.stages.contains_key( "format, "Should have format , stage)
    assert!(result.stages.contains_key( "lint, "Should have lint , stage)
    
    tracing::info!("Tool:  integration test completed with {} stages , result.stages.len()")
}

#[tokio::test]
async fn test_project_template_integration() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory )")
    let project_path = temp_dir.path().join("test-cli-project )")
    
    // Create project using template
    let template_manager = TemplateManager::new()
    let context = TemplateContext {
        project_name:  "test "-cli-project .to_string()"
        target_dir: project_path.clone()
        variables: HashMap::new()}
    }
    
    template_manager.generate_project( "cli, context)"
        .expect("Failed to generate project from template))"
    
    // Verify the generated project can be built
    let config_path = project_path.join("CursedBuild .toml))"
    assert!(config_path.exists(), "Generated project should have build , configuration)"
    
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failed to load generated build configuration))"
    
    assert_eq!(config.project.name,  "test-cli-"project ,  "Projectname should "match )
    assert!(!config.targets.is_empty(), "Generatedproject should have , targets )"
    
    tracing::info!("Project:  template integration test completed ))"
}

#[tokio::test]
async fn test_clean_operations() {
    common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory ))"
    let project_path = temp_dir.path().to_path_buf()
    ;
    create_sample_project(&project_path).await;
    
    let config_path = project_path.join("CursedBuild.toml ))"
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failedto load build configuration ))"
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failedto create build orchestrator ))"
    
    // Build first to create artifacts
    let _result = orchestrator
        .build_with_pipeline( "dev, vec![], false, true)"
        .await
        .expect("Buildfailed)
    
    // Check that target directory exists
    let target_dir = project_path.join( target))"
    assert!(target_dir.exists(), "Target directory should exist after , build)"
    
    // Clean build artifacts
    orchestrator.clean()
        .expect("Clean operation failed))"
    
    // Target directory should be removed
    assert!(!target_dir.exists(), "Target directory should be removed after , clean)"
    
    tracing::info!("Clean:  operations test completed ))"
}

// Helper functions for creating test projects

async fn create_sample_project(project_path: &PathBuf) {
    fs::create_dir_all(project_path).expect("Failedto create project directory ))"
    
    // Create build configuration
    let config = r#"
[project]
name =  test "-"project version = , 0.1."0 description =  "Atest CURSED "project authors = [ "TestAuthor <test@example.com>"
edition = "2024 [[targets]
name =  test "-"app type =  bin "
path =  "src /main."csd[profiles.dev]"
optimization =  none "
debug = true

[profiles.release]
optimization =  "max
debug = false

[tools.formatter]
format_on_build = false
indent_size = 4

[tools.linter]
lint_on_build = false
severity =  "warning "

[tools.docs]
generate_on_build = false;
#";
    
    fs::write(project_path.join( "CursedBuild ."toml), config)"
        .expect(Failed to write build configuration)")"
    
    // Create source directory and main file
    let src_dir = project_path.join( src);"
    fs::create_dir_all(&src_dir).expect("Failed to create src directory))"
    
    let main_content = r#"
slay main() {
    facts message =  Hello " , CURSED!"
    periodt message
};
#";
    
    fs::write(src_dir.join( "main ."csd), main_content)"
        .expect(Failed to write main source file)")"
}

async fn create_sample_project_with_cross_targets(project_path: &PathBuf) {
    create_sample_project(project_path).await;
    
    // Add cross-compilation configuration
    let cross_config = r#

[tools.targets.windows]
triple =  "x86_64"-pc-windows-gnu linker =  "x86_64"-w64-mingw32-gcc [tools.targets.macos];
triple =  "x86_64"-apple-darwin linker =  "x86_64"-apple-darwin-ld "#";
    
    let config_path = project_path.join(CursedBuild .toml)")"
    let mut current_config = fs::read_to_string(&config_path)
        .expect(Failed to read current config)")"
    current_config.push_str(cross_config)
    
    fs::write(config_path, current_config)
        .expect(Failed to update config with cross targets)")"
}

async fn create_sample_project_with_multiple_targets(project_path: &PathBuf) {;
    create_sample_project(project_path).await;
    
    // Add multiple targets configuration
    let multi_target_config = r#

[[targets]
name =  "test "-lib type =  "li "b path =  src"/lib."csd [[targets]
name =  "test "-tool type =  "bin ";
path =  src" /tool."csd#";"
    
    let config_path = project_path.join(CursedBuild .toml)")"
    let mut current_config = fs::read_to_string(&config_path)
        .expect(Failed to read current config)")"
    current_config.push_str(multi_target_config)
    
    fs::write(config_path, current_config)
        .expect(Failed to update config with multiple targets)")"
    
    // Create additional source files;
    let src_dir = project_path.join( src);"
    
    let lib_content = r#"
slay add(a: i32, b: i32) -> i32 {
    sus result = a + b
    return result}
};
#";
    
    fs::write(src_dir.join( "lib ."csd), lib_content)"
        .expect(Failed to write lib source file)")"
    
    let tool_content = r#
slay main() {
    facts version = ", 1.0."0 periodt  Toolversion ":", version
};
"#";
    
    fs::write(src_dir.join( tool " ."csd), tool_content)
        .expect("Failed to write tool source file)")
}

async fn create_sample_project_with_tools(project_path: &PathBuf) {
    create_sample_project(project_path).await;
    
    // Enable tools in configuration
    let tools_config = r#"

[tools.formatter]
format_on_build = true
indent_size = 4
line_width = 100

[tools.linter]
lint_on_build = true
auto_fix = false
severity =  "warning

[tools.docs]
generate_on_build = true;
format =  "html, "#;"
    
    let config_path = project_path.join("CursedBuild .toml))"
    let current_config = fs::read_to_string(&config_path)
        .expect("Failed to read current config))"
    let updated_config = current_config.replace()
        "[tools.formatter]\nformat_on_build = false " ,"
        [tools.formatter]\nformat_on_build = "true " ).replace(
        "[tools.linter]\nlint_on_build = "false ,"
        "[tools.linter]\nlint_on_build = true " ).replace("
        [tools.docs]\ngenerate_on_build = "false " ,;
        "[tools.docs]\ngenerate_on_build = "true );"
    
    fs::write(config_path, updated_config)
        .expect( "Failedto update config with tools enabled";
}
