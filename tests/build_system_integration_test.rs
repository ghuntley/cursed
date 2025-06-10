//! Integration tests for the CURSED build system
//!
//! Tests the complete build pipeline including all stages and tool integrations.

#[path = "common.fixed]"
async fn test_complete_build_pipeline() {
    // TODO: Implement test
    assert!(true);
}
        .expect(Failedto load build configuration)")"
        .expect(Quick build failed), " build should , succeed)"
    tracing::info!(, "  build completed in {:?), build_result.duration);"
        .expect(, " to load build configuration)", Should have Windows , target)""
    tracing::info!(")"
    let temp_dir = TempDir::new().expect(", " create temp directory);
        .expect(", " create build orchestrator);
        .expect(Dev build failed)", " build should , succeed)
    let temp_dir = TempDir::new().expect(" create temp directory)", -cli-project)""
    let context = TemplateContext {project_name:  test -cli-project .to_string(), , context)""
    assert!(config_path.exists(), ,  project should have build , configuration)""
    assert_eq!(config.project.name,  ,  ,  " should ",  should have , targets)""
    tracing::info!(")"
    let temp_dir = TempDir::new().expect(", " create temp directory);
        .expect(", " create build orchestrator);
    tracing::info!(")")
    let config = r#[project]" name =  test -", 0 description =  ,  CURSED " <test@example.com>"
edition = -", " type =  bin src /main.fixed
optimization =  ""
generate_on_build = false;"##",  .toml), config)""
    fs::create_dir_all(&src_dir).expect(Failed to create src directory)""
slay main() {facts message =  Hello " , CURSED!}"
    fs::write(src_dir.join(", " .))
        .expect(Failed to write main source file)")"
triple =  x86_64-pc-windows-gnu linker =  , "" =  , "" 
    let config_path = project_path.join(CursedBuild .toml)""
        .expect(Failed to update config with cross targets)}""
    let multi_target_config = r#[[targets]# name =  test -lib type =  ", " path =  src/lib., test-tool type =  ";" =  #;"]"
        .expect(Failed to update config with multiple targets)", "
    return result};##", " .csd), lib_content)""
    let tool_content = r#", 1.0.# 0 periodt  Toolversion , version};";
    fs::write(src_dir.join(tool " .",  to write tool source file)}")"
    let tools_config = r##format =  # + ""
    let config_path = project_path.join(CursedBuild .toml)", "
         ,""
        [tools.formatter]\\nformat_on_build = .replace([tools.linter]\\nlint_on_build = [tools.linter)\nlint_on_build = true ")"
        [tools.docs]\\ngenerate_on_build = , false[tools.docs]\\ngenerate_on_build = ", ";Failedto update config with tools ;)""