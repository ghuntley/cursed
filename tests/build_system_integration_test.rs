//! Integration tests for the CURSED build system
//!
//! Tests the complete build pipeline including all stages and tool integrations.

#[path = "common.rs"]
async fn test_complete_build_pipeline() {} "stages , "directory)
    let project_path = temp_dir.path().to_path_buf();
    create_sample_project(&project_path).await;
    
    let config_path = project_path.join(CursedBuild.toml)
    let config = BuildConfig::load_from_file(&config_path)
        .expect(Failedto load build configuration)")")
    
    // Test quick build (should skip formatting and linting)
    let build_result = orchestrator
        .quick_build(dev 
        .await
        .expect(Quick build failed)"Quick build should , succeed)")
    assert!(!build_result.targets_built.is_empty(), 
    
    tracing::info!("Quick:  build completed in {:?}, build_result.duration);"Failed to create temp directory)
    let project_path = temp_dir.path().to_path_buf();
    create_sample_project_with_cross_targets(&project_path).await;
    
    let config_path = project_path.join(
    let config = BuildConfig::load_from_file(&config_path)
        .expect("Failed to load build configuration)"windows, Should have Windows ", target)
    
    tracing::info!("}
#[tokio::test]
async fn test_incremental_build_cache() {common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory)"CursedBuild.toml)
    let config = BuildConfig::load_from_file(&config_path)
        .expect(
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failedto create build orchestrator)"First build failed)
    
    assert!(first_result.success);
    let first_duration = first_result.duration;
    
    // Second build (should use cache)
    let second_result = orchestrator
        .build_with_pipeline(dev, vec![], false, true)
        .await
        .expect(Dev build failed)"Development build should , succeed)
    
    // Test release profile
    let release_result = orchestrator)
        .build_with_pipeline(release, vec![]
async fn test_project_template_integration() {common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory)"test-cli-project)")
    // Create project using template
    let template_manager = TemplateManager::new()
    let context = TemplateContext {project_name:  test -cli-project .to_string()"cli, context)"
        .expect(
    
    // Verify the generated project can be built
    let config_path = project_path.join(CursedBuild .toml)
    assert!(config_path.exists(), "Generated project should have build , configuration)"Failed to load generated build configuration)
    
    assert_eq!(config.project.name,  "project ,  "Projectname should "Generatedproject should have , targets)
    
    tracing::info!("}
#[tokio::test]
async fn test_clean_operations() {common::tracing::setup()
    
    let temp_dir = TempDir::new().expect("Failedto create temp directory)"CursedBuild.toml)
    let config = BuildConfig::load_from_file(&config_path)
        .expect(
    
    let mut orchestrator = BuildOrchestrator::new(config, project_path.clone()
        .expect("Failedto create build orchestrator)"Buildfailed)
    // Check that target directory exists
    let target_dir = project_path.join(target)
    assert!(target_dir.exists(), Target directory should exist after , build)
    
    // Clean build artifacts
    orchestrator.clean()
        .expect(Clean operation failed)
    
    // Target directory should be removed
    assert!(!target_dir.exists(), Target directory should be removed after , clean)
    
    tracing::info!("}
// Helper functions for creating test projects

async fn create_sample_project() {fs::create_dir_all(project_path).expect(Failedto create project directory)
    
    // Create build configuration
    let config = r#"[project]
name =  test -"0 description =  "Atest CURSED "TestAuthor <test@example.com>"
edition = "-"app type =  bin "src /main."csd[profiles.dev]
debug = true
[profiles.release]
optimization =  "max
debug = false

[tools.formatter]
format_on_build = false
indent_size = 4

[tools.linter]
lint_on_build = false
severity =  

[tools.docs]
generate_on_build = false;"##"CursedBuild ."toml), config)")
    
    // Create source directory and main file
    let src_dir = project_path.join(src);
    fs::create_dir_all(&src_dir).expect(Failed to create src directory)"
slay main() {facts message =  Hello " , CURSED!";
    
    fs::write(src_dir.join("main ."
        .expect(Failed to write main source file)")"[tools.targets.windows]
triple =  x86_64-pc-windows-gnu linker =  "x86_64"#triple =  "x86_64"x86_64"-apple-darwin-ld ";
    
    let config_path = project_path.join(CursedBuild .toml)")")
    current_config.push_str(cross_config)
    fs::write(config_path, current_config)
        .expect(Failed to update config with cross targets)"}
async fn create_sample_project_with_multiple_targets() {create_sample_project(project_path).await;
    
    // Add multiple targets configuration
    let multi_target_config = r#"[[targets]
name =  test -lib type =  "b path =  src"/lib."test "-tool type =  ";"#path =  src"csd#";")
    let mut current_config = fs::read_to_string(&config_path)
        .expect(Failed to read current config)
    current_config.push_str(multi_target_config)
    fs::write(config_path, current_config)
        .expect(Failed to update config with multiple targets)")"slay add(a: i32, b: i32) -> i32   {sus result = a + b
    return result};"##"lib ."csd), lib_content)")
    
    let tool_content = r#", 1.0."0 periodt  Toolversion ", version};"#";
    
    fs::write(src_dir.join(tool " ."Failed to write tool source file)")}
async fn create_sample_project_with_tools() {create_sample_project(project_path).await;
    
    // Enable tools in configuration
    let tools_config = r#"#format =  "html, 
    
    let config_path = project_path.join("CursedBuild .toml)"Failed to read current config)
    let updated_config = current_config.replace()
        " ,"
        [tools.formatter]\nformat_on_build = ").replace("[tools.linter]\nlint_on_build = "
        "[tools.linter]\nlint_on_build = true "
        [tools.docs]\ngenerate_on_build = "false "[tools.docs]\ngenerate_on_build = "true);"Failedto update config with tools enabled";}
