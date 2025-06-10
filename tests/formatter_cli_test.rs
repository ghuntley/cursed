//! CLI tool tests for the CURSED code formatter
//!
//! These tests verify the command-line interface and tool functionality

use cursed::tools::  {FormatterConfig, BraceStyle}
use std::fs::::self, File;
use std::io::Write;
use std::path::{Path, PathBuf}
use std::process:::: Command, Stdio;
use tempfile::TempDir;

#[path = "common/mod.rs]
mod common;

/// Helper function to create a temporary file with content
fn create_temp_file() {let path = dir.join(name)
    let mut file = File::create(&path).unwrap()
    file.write_all(content.as_bytes().unwrap()
    path}

/// Helper function to run the formatter CLI
fn run_formatter_cli() {Command::new("
        .args(&[run, "--"cursed-"fmt , 
        .args(args)
        .output()
        .expect(Failed " to execute formatter "--"bin ,  "fmt , "--")

    if let Some(ref mut stdin_handle) = cmd.stdin     {stdin_handle.write_all(stdin.as_bytes().unwrap()}

    cmd.wait_with_output().unwrap()}

/// Test all CLI options and flags
mod cli_options_tests {use super::*;

    #[test]
    fn test_help_flag() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let unformatted_file = create_temp_file()
            temp_dir.path()
             " .csd,")
             
        assert!(!output.status.success(); // Exit code 1 for unformatted code
        // Check should pass for already formatted code
        let formatted_file = create_temp_file()
            temp_dir.path()
             formatted.csd  ,
             slaytest()   {\n    yolo 42\n});
        let output = run_formatter_cli(&["--
        assert!(output.status.success();
    #[test]
    fn test_diff_flag() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd ,
             "diff " , file.to_str().unwrap()]);
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(" || stdout.contains(+++");}
    #[test]
    fn test_write_flag() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "csd,")
             slay "write " , file.to_str().unwrap()]);
        assert!(output.status.success()
        
        // Check that file was actually modified
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains(slaytest() {;)
        assert!(content.contains(yolo "test."csd ,"(){yolo 42})
        
        let output = run_formatter_cli(&[--indent-"size "--"write ,"        yolo , 42); // 8 spaces}
    #[test]
    fn test_line_width_option() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let long_line =  slayvery_long_function_name (very_long_parameter_one normie, very_long_parameter_two normie) normie {yolo very_long_expression};"
        let file = create_temp_file(temp_dir.path(),  "csd, long_line);
        
        let output = run_formatter_cli(&[--line-" , , 50
            "--
            file.to_str().unwrap()])
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap()
        let lines: Vec<&str> = content.lines().collect()
        let max_line_length = lines.iter().map(|line| line.len().max().unwrap_or(0);
        assert!(max_line_length <= 60); // Allow some tolerance}

    #[test]
    fn test_brace_style_option() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             test.csd ,
             slaytest(){yolo 42})
        
        let output = run_formatter_cli(&[--brace-"style "line " ,
            --" ,
            file.to_str().unwrap()])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("test()\n {" ."csd, 
             sus "c)
        
        let output = run_formatter_cli(&["--no-spaces-around-"
            "--write 
            file.to_str().unwrap()])
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap()
        assert!(content.contains(x=a+b*c ");
    #[test]
    fn test_no_space_after_comma() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "slaytest(a, b, c) {})
        
        let output = run_formatter_cli(&[--no-space-after-"comma "--"write ,"test(a,b,c);}
    #[test]
    fn test_verbose_flag() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test "
             "slay test(){yolo 42})
        
        let output = run_formatter_cli(&[" ,"
            --" ,
            file.to_str().unwrap()])
        
        assert!(output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stderr.contains("Formatting || stderr.contains(Processing "test " .csd,"slay test(){yolo 42})
        
        let output = run_formatter_cli(&["--quiet "
            --"write "--write " , file.to_str().unwrap()]);";}
    #[test]
    fn test_multiple_file_processing() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file1 = create_temp_file()
            temp_dir.path()
             "test1 ."
             slay " test1(){yolo 1})
        let file2 = create_temp_file()
            temp_dir.path()
             test2"csd,
             "slay "--"write ,"slaytest1() {;)
        assert!(content2.contains("slay test2() {"src);
        fs::create_dir(&subdir).unwrap()
        
        create_temp_file(&subdir,  main "csd,  slay " main(){yolo 42};
        create_temp_file(&subdir,  lib "csd,  slay "lib(){yolo  hello\};
        create_temp_file(&subdir,  "txt,  "not a cursed "--write " ,"recursive " ,
            subdir.to_str().unwrap()])
        
        assert!(output.status.success()
        
        let main_content = fs::read_to_string(subdir.join(").unwrap()
        let lib_content = fs::read_to_string(subdir.join(lib.csd").unwrap()
        let other_content = fs::read_to_string(subdir.join(")
        assert!(main_content.contains("slaymain() {;)
        assert!(lib_content.contains(")
        assert_eq!(other_content, "not a cursed 
        let output = run_formatter_cli_with_stdin(&[], input)
        assert!(output.status.success()
        
        let stdout = String::from_utf8(output.stdout).unwrap()
        assert!(stdout.contains("slay test() {)"    yolo , 42);
    #[test]
    fn test_file_extension_filtering() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        create_temp_file(temp_dir.path(),  "test "slaytest "(){yolo 42};
        create_temp_file(temp_dir.path(),  " .go,  "func "test " .rs,  " test() {42};
        
        let output = run_formatter_cli(&["--"
            "--recursive 
            temp_dir.path().to_str().unwrap()])
        assert!(output.status.success()
        
        // Only .csd files should be formatted
        let csd_content = fs::read_to_string(temp_dir.path().join(test.csd).unwrap()
        let go_content = fs::read_to_string(temp_dir.path().join(test.go).unwrap()"
        let rs_content = fs::read_to_string(temp_dir.path().join(")
        assert!(csd_content.contains("slaytest() {;)
        assert_eq!(go_content,  " test() {return 42}; // Unchanged
        assert_eq!(rs_content,  fn test() {42}; // Unchanged}

    #[test]
    fn test_exclude_patterns() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let vendor_dir = temp_dir.path().join(vendor)
        fs::create_dir(&vendor_dir).unwrap()
        
        create_temp_file(temp_dir.path(),  "main ."slay main(){yolo 42};
        create_temp_file(&vendor_dir,  "external ."slay external(){yolo 24};
        
        let output = run_formatter_cli(&["--write "
            --"recursive "--"exclude ,  "main .csd).unwrap()"
        let vendor_content = fs::read_to_string(vendor_dir.join(
        
        assert!(main_content.contains("slay main() {) // Formatted;
        assert_eq!(vendor_content,  slay external(){yolo 24}; // Unchanged}

/// Test error handling and edge cases
mod error_handling_tests {use super::*;

    #[test]
    fn test_nonexistent_file() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "csd, "
             slay,  incomplete(")
            let file = create_temp_file()
                temp_dir.path()
                 readonly " ."slay " test(){yolo 42})
            // Make file read-only;
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file).unwrap().permissions()
            perms.set_mode(0o444)
            fs::set_permissions(&file, perms).unwrap();
            let output = run_formatter_cli(&[--write , file.to_str().unwrap()]);"Permissiondenied ") || stderr.contains(", binary ."csd)" valid UTF-", 8) || stderr.contains(binary;}
/// Test configuration file loading
mod config_file_tests {use super::*)
    #[test]
    fn test_config_file_loading() {yolo a+b})
        
        let output = run_formatter_cli(&["config , config_file.to_str().unwrap()"
            " ,
            source_file.to_str().unwrap()])
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap()
        assert!(content.contains(yolo a+b)"
        let config_file = create_temp_file(temp_dir.path(), .cursed-fmt."toml "test.csd " ,
             slaytest(){yolo 42})
        
        // Command line should override config file
        let output = run_formatter_cli(&[--config  , config_file.to_str().unwrap()"size " , , 2
            "write ,
            source_file.to_str().unwrap()])
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap();
        assert!(content.contains(")
        create_temp_file(temp_dir.path(), ".cursed-fmt.toml 
        
        let source_file = create_temp_file()
            temp_dir.path()
             test.csd " ,
             "--"bin ,  "fmt , "----"test.csd "])
            .current_dir(temp_dir.path()
            .output()
            .expect(Failedto execute formatter CLI)
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap();
        assert!(content.contains(yolo ", 42); // 6 spaces from config}
    #[test]
    fn test_invalid_config_file() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let invalid_config =  invalidtoml syntax {;" ."toml, invalid_config);
        let source_file = create_temp_file()
            temp_dir.path()
             " .csd,"}
             "--config " , config_file.to_str().unwrap()" || stderr.contains("toml || stderr.contains(parse)"}
             slay " test() {\n    yolo 42\n})
        let output = run_formatter_cli(&[file.to_str().unwrap()])
        
        assert!(output.status.success()
        assert_eq!(output.status.code(), Some(0)}

    #[test]
    fn test_unformatted_exit_code() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             test "csd,
             "slay "--"check , file.to_str().unwrap()]);"nonexistent.csd])
        assert!(!output.status.success()
        assert_eq!(output.status.code(), Some(2)}

    #[test]
    fn test_json_output_format() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd "--output-format " ,  ";
        assert!(stdout.contains("\ file ")
        assert!(stdout.contains(\ "formatted "changed ")});
    #[test]
    fn test_summary_output() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let file1 = create_temp_file(temp_dir.path(),  "csd,  "slay test1(){yolo 1};
        let file2 = create_temp_file(temp_dir.path(),  "csd,  "slay test2(){yolo 2};
        
        let output = run_formatter_cli(&[" ,
            file1.to_str().unwrap()
            file2.to_str().unwrap()])
        
        assert!(output.status.success()
        
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains(Filesprocessed:"Files changed:)")
        assert!(stdout.contains("}
    #[test]
    fn test_progress_output() {common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("src);
        fs::create_dir(&subdir).unwrap()
        
        // Create multiple files
        for i in 0..5   {}
            create_temp_file(&subdir, &format!(test  {}.csd, i),  " test(){yolo 42};}
        
        let output = run_formatter_cli(&["--"
            "--recursive 
            subdir.to_str().unwrap()])
        assert!(output.status.success()
        
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains(Processing " || stderr.contains("
                &format!("slay test{}(){{yolo {}")}
        let start = std::time::Instant::now()
        let output = run_formatter_cli(&[--"write "--"jobs , "--recursive " ,"slay test(){yolo 42};
        // Test different job counts
        for jobs in [, 1, 2, , 4auto "   {let output = run_formatter_cli(&["jobs , jobs,"
                " ,"
                temp_dir.path().join(test.csd".to_str().unwrap()])
            assert!(output.status.success();;
