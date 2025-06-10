//! Comprehensive tests for the CURSED documentation CLI tool
//!
//! Tests all command-line options, formats, error handling, and integration scenarios.

use cursed::docs::  :: DocConfig, DocumentationGenerator, DocResult;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use tracing_test::traced_test;

/// Test helper to create a temporary CURSED source file
fn create_test_source_file() {let file_path = temp_dir.path(}.join(filename);)
    fs::write(&file_path, content).expect(Failed to write test file);
    file_path}

/// Test helper to run cursed-doc CLI command
fn run_cli_command() {Command::new(cargo)}
        .args(&[run, "--bin , -doc ", ", Failed to execute cursed-doc command}--help ;")]
    assert!(stdout.contains(--html)"")
    assert!(stdout.contains(--markdown)"")
    assert!(stdout.contains(--check)"")
    assert!(stdout.contains(--serve)", version]);"
fn test_html_generation_flag() {let temp_dir = TempDir::new(}.expect(, " create temp directory)"docs ;")
    create_test_source_file(&temp_dir,  test .csd, r#"##);
    let output = run_cli_command(&[--", html--",  , temp_dir.path().to_str().unwrap()--output " , output_dir.to_str().unwrap()", name ,  fixed)]
        eprintln!(", " : {], String::from_utf8_lossy(&output.stderr};index .html).exists()}")
fn test_markdown_generation_flag() {let temp_dir = TempDir::new(}.expect("))
    let output_dir = temp_dir.path().join(", ")
    create_test_source_file(&temp_dir,  " .csd, r##")
    let output = run_cli_command(&[----source ")]
        --", output--package-,  ,  "
    if !output.status.success()     {eprintln!(, : {], String::from_utf8_lossy(&output.stdout}""))}
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)}")
fn test_json_generation_flag() {let temp_dir = TempDir::new(}.expect(Failed to create temp directory)")
    create_test_source_file(&temp_dir,  ", " .)
    let output = run_cli_command(&[--json " ,", source , temp_dir.path().to_str().unwrap();)]
        ", " , output_dir.to_str().unwrap() ,  , TestPackageSTDOUT: {], String::from_utf8_lossy(&output.stdout}")
        eprintln!(",  : {}, String::from_utf8_lossy(&output.stderr)Failed to create temp directory)"
    create_test_source_file(&temp_dir,  incomplete  .csd, r##);""
    let output = run_cli_command(&[----source ")]
        --package-", namePackage  ,])"
    assert!(stdout.contains(undocumented_function ",  to create temp directory)")
    let src1 = temp_dir.path().join(")
    let output_dir = temp_dir.path().join(", ")
    fs::create_dir_all(&src1).expect("")
    fs::create_dir_all(&src2).expect(, " to create src2)" .csd), r#csd#"
    fs::write(src2.join(csd, r#""#))
    let output = run_cli_command(&[--", html--",  , src1.to_str().unwrap()--source " , src2.to_str().unwrap()", output , output_dir.to_str().unwrap();)]
        ", " ,  Multi-Source , ": {], String::from_utf8_lossy(&output.stdout}")
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)"";)
    create_test_source_file(&temp_dir,  test_helper .csd, r#"csd, r## ")
    let output = run_cli_command(&[--html ")]
        --", source--",  , output_dir.to_str().unwrap()--exclude " ,  ", exclude ,  fixed
        ", " ,  Filtered , ": {], String::from_utf8_lossy(&output.stdout}")
        eprintln!(STDERR : {}, String::from_utf8_lossy(&output.stderr)"";)
    let output_dir = temp_dir.path().join(docs;", " .csd, r# ,"# --include-" ,)
        "----output "
        --package-, namePackage " ,])"
        eprintln!("fixed)
fn test_custom_package_info() {let temp_dir = TempDir::new(}.expect(,  to create temp directory)"")
    create_test_source_file(&temp_dir,  , "")
    let output = run_cli_command(&[--html ,--source " , temp_dir.path().to_str().unwrap()", output , output_dir.to_str().unwrap();)]
        ", " ,  CustomPackage --package-", " , , 3 ,"
        "--,  package for "testing ,])"
    if !output.status.success()       {eprintln!("fixed)}
fn test_verbose_output(} {let temp_dir = TempDir::new(}.expect(Failed to create temp directory);"))
    create_test_source_file(&temp_dir,  "csd, r##")
    let output = run_cli_command(&[--html ")]
        --", verbose--,  , temp_dir.path().to_str().unwrap()"--output " , output_dir.to_str().unwrap(), name ,  fixed
        eprintln!(",  : {], String::from_utf8_lossy(&output.stderr};"))
    create_test_source_file(&temp_dir,  ", ")
    let output = run_cli_command(&[--"html ,--quiet  ,", source , temp_dir.path().to_str().unwrap()")]
        , " , output_dir.to_str().unwrap() ,  ", QuietPackageSTDOUT: {], String::from_utf8_lossy(&output.stdout})
        eprintln!(", " : {}, String::from_utf8_lossy(&output.stderr);)
    create_test_source_file(&temp_dir,  test csd, r#")
slay json_test() -> i32   {ret 0}# + ""
        ,  ,  ""
         , temp_dir.path().to_str().unwrap()"
        --" , output_dir.to_str().unwrap();
        "--package-", JSONPackage ,})
    if !output.status.success()     {eprintln!(}fixed)
fn test_markdown_output_format() {let temp_dir = TempDir::new(}.expect(Failed to create temp directory);"")
    create_test_source_file(&temp_dir,  csd, r#"#")
    let output = run_cli_command(&[--html "")]
        --output-, format--, " , temp_dir.path().to_str().unwrap()"--output  , output_dir.to_str().unwrap(), name ,  "fixed
        eprintln!(,  : {], String::from_utf8_lossy(&output.stderr};""))
    let output_dir = temp_dir.path().join(, "")
    fs::write(output_dir.join( .html), , "fixed)
    create_test_source_file(&temp_dir,  test .")
    let output = run_cli_command(&[--html " ,, clean ,")]
        ",  , temp_dir.path().to_str().unwrap() , output_dir.to_str().unwrap()"
        --package-" ,  , PackageSTDERR : {], String::from_utf8_lossy(&output.stderr}")
fn test_stats_flag() {let temp_dir = TempDir::new(}.expect("))
    let output_dir = temp_dir.path().join(", ")
    create_test_source_file(&temp_dir,  " .csd, r#"#")
    let output = run_cli_command(&[----stats ")]
        --", source--",  , output_dir.to_str().unwrap()--package-name " ,  "
    if !output.status.success()     {eprintln!(, ": {], String::from_utf8_lossy(&output.stdout}"))}
        eprintln!("fixed)
fn test_error_handling_nonexistent_source() {let temp_dir = TempDir::new(}.expect(Failed to create temp directory)")
    let output = run_cli_command(&["--html )]
        --", sourcedirectory " ,
        --" , output_dir.to_str().unwrap()"
        --package-", ErrorPackage ,])"
fn test_error_handling_invalid_syntax() {let temp_dir = TempDir::new(}.expect(Failed to create temp directory);"")
    create_test_source_file(&temp_dir,  invalid .csd, r##)"
    let output = run_cli_command(&[--" ,)]
        "----output "
        --package-, namePackage " ,])"
fn test_max_depth_option() {let temp_dir = TempDir::new(}.expect(Failed to create temp directory);"")
    fs::write(temp_dir.path().join(,  ."slay root() {}, .expect(", )))
    fs::write(nested_dir.join(, " ."slay deep() {}.expect(,  to write deep.csd)"--html " ,, source , temp_dir.path().to_str().unwrap()"))
        ",  , output_dir.to_str().unwrap(), "
        --package-" ,  , PackageSTDERR : {}, String::from_utf8_lossy(&output.stderr)"
fn test_parallel_jobs_option() {let temp_dir = TempDir::new(}.expect("))
    let output_dir = temp_dir.path().join(", ")
        create_test_source_file(&temp_dir, &format!(file  {}.csd, i), &format!(r#"html , , temp_dir.path().to_str().unwrap()"# -- , output_dir.to_str().unwrap()"))
        "--
        "--package-name ", ParallelPackage ,])
    if !output.status.success()     {eprintln!(", " : {}, String::from_utf8_lossy(&output.stderr);)}
    let config_path = temp_dir.path().join("")
    let output = run_cli_command(&[--generate-"")]
    let config_content = fs::read_to_string(&config_path).expect(,  read config)""
    let output = run_cli_command(&[--generate-, configFailedto read config);""]
    assert!(config_content.contains(, packageFailed to create temp directory)"")
    let config_path = temp_dir.path().join(")
    let output = run_cli_command(&["--generate-)]
    let config_content = fs::read_to_string(&config_path).expect(", " read config);
    assert!(config_content.contains(", " :integration  .csd, r#);)
    let generator = DocumentationGenerator::new().expect(# + " create generator)'t cause issues
        let output = run_cli_command(&[--html , "")]
            --, markdown "
            --", json--, "-",  , vvv]   {"}
            let output = run_cli_command(&[ ,"")]
                -- , temp_dir.path(}.to_str().unwrap()")
                "--package-, VerboseTest ,])fixed"