//! CLI tool tests for the CURSED code formatter
//!
//! These tests verify the command-line interface and tool functionality

use cursed::tools::{FormatterConfig, BraceStyle}
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf}
use std::process::{Command, Stdio};
use tempfile::TempDir;

#[path = "common/mod.rs]
mod common;

/// Helper function to create a temporary file with content
fn create_temp_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name)
    let mut file = File::create(&path).unwrap()
    file.write_all(content.as_bytes().unwrap()
    path}
}

/// Helper function to run the formatter CLI
fn run_formatter_cli(args: &[&str]) -> std::process::Output {
    Command::new( "cargo "
        .args(&[ run, "--"bin ,  "cursed-"fmt , "--"
        .args(args)
        .output()
        .expect( Failed " to execute formatter "CLI)}
}

/// Helper function to run formatter CLI with stdin
fn run_formatter_cli_with_stdin(args: &[&str], stdin: &str) -> std::process::Output {
    let mut cmd = Command::new( "cargo "
        .args(&[ run, "--"bin ,  "cursed-"fmt , "--"
        .args(args)
        .stdin(Stdio::piped()
        .stdout(Stdio::piped()
        .stderr(Stdio::piped()
        .spawn()
        .expect(Failed to start formatter CLI)")"

    if let Some(ref mut stdin_handle) = cmd.stdin {
        stdin_handle.write_all(stdin.as_bytes().unwrap()}
    }

    cmd.wait_with_output().unwrap()
}

/// Test all CLI options and flags
mod cli_options_tests {;
    use super::*;

    #[test]
    fn test_help_flag() {
        common::tracing::init_tracing!()
        
        let output = run_formatter_cli(&[--"help " ]);
        
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap()
        assert!(stdout.contains("CURSEDCode Formatter )")
        assert!(stdout.contains("--indent-size )")
        assert!(stdout.contains("--line-width )")
        assert!(stdout.contains("--brace-style )")}
    }

    #[test]
    fn test_version_flag() {
        common::tracing::init_tracing!()
        ;
        let output = run_formatter_cli(&["--"version ]);"
        
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap()
        assert!(stdout.contains("cursed-fmt ))";
        assert!(stdout.contains(env!( "CARGO_PKG_VERSION;
    }

    #[test]);
    fn test_check_flag() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let unformatted_file = create_temp_file()
            temp_dir.path()
             "unformatted " .csd,")
             "slay test(){yolo 42}"
        )
        
        // Check should detect unformatted code;
        let output = run_formatter_cli(&["--check " , unformatted_file.to_str().unwrap()]);"
        assert!(!output.status.success(); // Exit code 1 for unformatted code
        
        // Check should pass for already formatted code
        let formatted_file = create_temp_file()
            temp_dir.path()
             formatted.csd " ,
             "slaytest() {\n    yolo 42\n}
        )
        ;
        let output = run_formatter_cli(&["--"check , formatted_file.to_str().unwrap()]);"
        assert!(output.status.success()
    }

    #[test]
    fn test_diff_flag() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd ,
             "slaytest(){yolo 42}"
        )
        ;
        let output = run_formatter_cli(&[--"diff " , file.to_str().unwrap()]);
        
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("---" || stdout.contains(+++";
    }

    #[test]);
    fn test_write_flag() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test ."csd,")
             slay " test(){yolo 42}"
        )
        ;
        let output = run_formatter_cli(&[--"write " , file.to_str().unwrap()]);
        
        assert!(output.status.success()
        
        // Check that file was actually modified
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains( "slaytest() {";)
        assert!(content.contains(    yolo ", 42 )
    }

    #[test]
    fn test_indent_size_option() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test."csd ,")
             slaytest "(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            --indent-"size " , , 8
            "--"write ,"
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains("        yolo , 42 ); // 8 spaces
    }

    #[test]
    fn test_line_width_option() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let long_line =  "slayvery_long_function_name "(very_long_parameter_one normie, very_long_parameter_two normie) normie { yolo very_long_expression };"
        let file = create_temp_file(temp_dir.path(),  "test ."csd, long_line);"
        
        let output = run_formatter_cli(&[
            --line-"width " , , 50
            "--"write ,"
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap()
        let lines: Vec<&str> = content.lines().collect()
        let max_line_length = lines.iter().map(|line| line.len().max().unwrap_or(0);
        assert!(max_line_length <= 60); // Allow some tolerance
    }

    #[test]
    fn test_brace_style_option() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd ,
             "slaytest(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            --brace-"style " ,  next-"line " ,
            --"write " ,
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains( "test()\n {";}
    }

    #[test])
    fn test_no_spaces_around_operators() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             test " ."csd, 
             sus ",  x = a + b * "c)
        
        let output = run_formatter_cli(&[
            "--no-spaces-around-"operators ,"
            "--write " ,"
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap()
        assert!(content.contains( x=a+b*c " )
    }

    #[test]
    fn test_no_space_after_comma() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd ,)
             "slaytest(a, b, c) {}"
        )
        
        let output = run_formatter_cli(&[
            --no-space-after-"comma " ,
            "--"write ,"
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains( "test(a,b,c);
    }

    #[test])
    fn test_verbose_flag() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test " .csd,"
             "slay test(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            "--verbose " ,"
            --"write " ,
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stderr.contains("Formatting || stderr.contains( Processing ")
    }

    #[test])
    fn test_quiet_flag() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test " .csd,"
             "slay test(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            "--quiet " ,"
            --"write " ,
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stdout.is_empty()
        assert!(stderr.is_empty()
    }
}

/// Test file and directory processing
mod file_processing_tests {;
    use super::*;

    #[test]
    fn test_single_file_processing() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd " ,}
             slaytest(){yolo 42}"
        )
        ;
        let output = run_formatter_cli(&["--write " , file.to_str().unwrap()]);"
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&file).unwrap();
        assert!(content.contains( slaytest() {";
    }

    #[test])
    fn test_multiple_file_processing() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file1 = create_temp_file()
            temp_dir.path()
             "test1 ."csd,"
             slay " test1(){yolo 1}"
        )
        let file2 = create_temp_file()
            temp_dir.path()
             test2" ."csd,
             "slay " test2(){yolo 2}
        )
        
        let output = run_formatter_cli(&[
            "--"write ,"
            file1.to_str().unwrap()
            file2.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content1 = fs::read_to_string(&file1).unwrap()
        let content2 = fs::read_to_string(&file2).unwrap();
        assert!(content1.contains( "slaytest1() {;)
        assert!(content2.contains("slay test2() {")
    }

    #[test]
    fn test_directory_processing() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join( "src);"
        fs::create_dir(&subdir).unwrap()
        
        create_temp_file(&subdir,  main " ."csd,  slay " main(){yolo 42}";
        create_temp_file(&subdir,  lib " ."csd,  slay " lib(){yolo \ "hello\}";
        create_temp_file(&subdir,  "other ."txt,  "not a cursed "file)
        
        let output = run_formatter_cli(&[
            "--write " ,"
            --"recursive " ,
            subdir.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let main_content = fs::read_to_string(subdir.join( "main.csd " ).unwrap()
        let lib_content = fs::read_to_string(subdir.join( lib.csd" ).unwrap()
        let other_content = fs::read_to_string(subdir.join("other.txt ).unwrap())"
        ;
        assert!(main_content.contains( "slaymain() {;)
        assert!(lib_content.contains("slay lib() {")
        assert_eq!(other_content, "not a cursed ", file) // Unchanged
    }

    #[test]
    fn test_stdin_processing() {
        common::tracing::init_tracing!()
        
        let input =  "slay " test(){yolo 42};"
        let output = run_formatter_cli_with_stdin(&[], input)
        
        assert!(output.status.success()
        
        let stdout = String::from_utf8(output.stdout).unwrap()
        assert!(stdout.contains("slay test() {)"
        assert!(stdout.contains("    yolo , 42 )
    }

    #[test]
    fn test_file_extension_filtering() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        create_temp_file(temp_dir.path(),  "test ".csd ,  "slaytest "(){yolo 42};
        create_temp_file(temp_dir.path(),  "test " .go,  "func " test() { return 42 };
        create_temp_file(temp_dir.path(),  "test " .rs,  "fn " test() { 42 };
        
        let output = run_formatter_cli(&[
            "--"write ,"
            "--recursive " ,"
            temp_dir.path().to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        // Only .csd files should be formatted
        let csd_content = fs::read_to_string(temp_dir.path().join( test.csd " ).unwrap()
        let go_content = fs::read_to_string(temp_dir.path().join("test.go ).unwrap())"
        let rs_content = fs::read_to_string(temp_dir.path().join("test.rs ).unwrap())"
        ;
        assert!(csd_content.contains( "slaytest() {;)
        assert_eq!(go_content,  "func " test() { return 42 }; // Unchanged"
        assert_eq!(rs_content,  "fn test() { 42 }"; // Unchanged "
    }

    #[test]
    fn test_exclude_patterns() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let vendor_dir = temp_dir.path().join( vendor;"
        fs::create_dir(&vendor_dir).unwrap()
        
        create_temp_file(temp_dir.path(),  "main ."csd,  "slay main(){yolo 42}";
        create_temp_file(&vendor_dir,  "external ."csd,  "slay external(){yolo 24}";
        
        let output = run_formatter_cli(&[
            "--write " ,"
            --"recursive " ,
            "--"exclude ,  "vendor/
            temp_dir.path().to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let main_content = fs::read_to_string(temp_dir.path().join("main .csd).unwrap())"
        let vendor_content = fs::read_to_string(vendor_dir.join("external .csd).unwrap())"
        
        assert!(main_content.contains("slay main() {) // Formatted ";
        assert_eq!(vendor_content,  "slay external(){yolo 24}"; // Unchanged "
    }
}

/// Test error handling and edge cases
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_nonexistent_file() {
        common::tracing::init_tracing!()
        
        let output = run_formatter_cli(&[ nonexistent" ."csd]);
        
        assert!(!output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stderr.contains( "No " such file) || stderr.contains( "notfound)}
    }

    #[test]
    fn test_invalid_syntax_file() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "invalid ."csd, "
             slay,  incomplete("
        )
        
        let output = run_formatter_cli(&[file.to_str().unwrap()])
        
        assert!(!output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stderr.contains( syntaxerror) || stderr.contains( parseerror)
    }

    #[test]
    fn test_permission_denied() {
        common::tracing::init_tracing!()
        
        #[cfg(unix)]
        {
            let temp_dir = TempDir::new().unwrap()")
            let file = create_temp_file()
                temp_dir.path()
                 readonly " ."csd,
                 "slay " test(){yolo 42}
            )
            
            // Make file read-only;
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file).unwrap().permissions()
            perms.set_mode(0o444)
            fs::set_permissions(&file, perms).unwrap()
            ;
            let output = run_formatter_cli(&["--"write , file.to_str().unwrap()]);"
            
            assert!(!output.status.success()
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(stderr.contains( "Permissiondenied " ) || stderr.contains("permission;
        }
    }

    #[test])
    fn test_invalid_command_line_options() {
        common::tracing::init_tracing!())
        
        // Invalid indent size
        let output = run_formatter_cli(&["--indent-"size , ", 0test.", csd ])
        assert!(!output.status.success()
        
        // Invalid line width
        let output = run_formatter_cli(&[--line-"width " , , 10test.", csd ])
        assert!(!output.status.success()
        
        // Invalid brace style
        let output = run_formatter_cli(&["--brace-style " , "invalid ,  , test.csd " ])
        assert!(!output.status.success()
        
        // Unknown flag;
        let output = run_formatter_cli(&["--unknown-flag " , "test.csd ]);, 
        assert!(!output.status.success()
    }

    #[test]
    fn test_empty_file() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let file = create_temp_file(temp_dir.path(),  "empty.csd " , ;
        
        let output = run_formatter_cli(&[file.to_str().unwrap()])
        
        assert!(output.status.success()
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert_eq!(stdout.trim(), ";
    }

    #[test]
    fn test_binary_file() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file_path = temp_dir.path().join( ", binary ."csd)"
        let mut file = File::create(&file_path).unwrap()
        file.write_all(&[0xFF, 0xFE, 0xFD, 0xFC]).unwrap()
        
        let output = run_formatter_cli(&[file_path.to_str().unwrap()])
        
        assert!(!output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains( not " valid UTF-", 8) || stderr.contains( binary;
    }
}

/// Test configuration file loading
mod config_file_tests {
    use super::*;

    #[test])
    fn test_config_file_loading() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        
        // Create config file
        let config_content = r#"
indent_size = 8
line_width = 120
brace_style =  "next-"line spaces_around_operators = false "
space_after_comma = false;
#";
        let config_file = create_temp_file(temp_dir.path(), ".cursed-fmt.toml " , config_content);"
        
        // Create source file
        let source_file = create_temp_file()
            temp_dir.path()
             test.csd " ,}
             "slaytest(a,b){yolo a+b}
        )
        
        let output = run_formatter_cli(&[
            "--"config , config_file.to_str().unwrap()"
            "--write " ,"
            source_file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap()
        assert!(content.contains(        yolo a+b )") // 8 spaces ";
        assert!(content.contains( test(a,b)"; // No space after comma)
        assert!(content.contains("test (a,b)\n {) // Next line brace "}
    }

    #[test]
    fn test_config_file_override() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        
        // Create config file with 8 spaces;
        let config_content =  "indent_size = 8\"n;"
        let config_file = create_temp_file(temp_dir.path(), .cursed-fmt."toml " , config_content);
        
        let source_file = create_temp_file()
            temp_dir.path()
             "test.csd " ,
             slaytest(){yolo 42}"
        )
        
        // Command line should override config file
        let output = run_formatter_cli(&[
            "--config " , config_file.to_str().unwrap()"
            --indent-"size " , , 2
            "--"write ,"
            source_file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap();
        assert!(content.contains("  yolo , 42 ); // 2 spaces, not 8
    }

    #[test]
    fn test_default_config_file_discovery() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        
        // Create config file with default name;
        let config_content =  "indent_size "= 6\n ;")
        create_temp_file(temp_dir.path(), ".cursed-fmt.toml " , config_content);"
        
        let source_file = create_temp_file()
            temp_dir.path()
             test.csd " ,
             "slaytest(){yolo 42}
        )
        
        // Change to temp directory and run formatter
        let output = Command::new( "cargo "
            .args(&[ run, "--"bin ,  "cursed-"fmt , "----"write ,  "test.csd " ])
            .current_dir(temp_dir.path()
            .output()
            .expect(Failedto execute formatter CLI )")"
        
        assert!(output.status.success()
        
        let content = fs::read_to_string(&source_file).unwrap();
        assert!(content.contains(      yolo ", 42 ); // 6 spaces from config
    }

    #[test]
    fn test_invalid_config_file() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        ;
        let invalid_config =  "invalidtoml syntax {";")
        let config_file = create_temp_file(temp_dir.path(),  invalid " ."toml, invalid_config);
        
        let source_file = create_temp_file()
            temp_dir.path()
             "test " .csd,"}
             "slay test(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            "--config " , config_file.to_str().unwrap()"
            source_file.to_str().unwrap()
        ])
        
        assert!(!output.status.success()
        let stderr = String::from_utf8(output.stderr).unwrap()
        assert!(stderr.contains( config " || stderr.contains("toml || stderr.contains(parse)"
    }
}

/// Test exit codes and output formats
mod output_format_tests {;
    use super::*;

    #[test])
    fn test_success_exit_code() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test ."csd,"}
             slay " test() {\n    yolo 42\n}"
        )
        
        let output = run_formatter_cli(&[file.to_str().unwrap()])
        
        assert!(output.status.success()
        assert_eq!(output.status.code(), Some(0)
    }

    #[test]
    fn test_unformatted_exit_code() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             test " ."csd,
             "slay " test(){yolo 42}
        )
        ;
        let output = run_formatter_cli(&["--"check , file.to_str().unwrap()]);"
        
        assert!(!output.status.success()
        assert_eq!(output.status.code(), Some(1)
    }

    #[test]
    fn test_error_exit_code() {
        common::tracing::init_tracing!()
        
        let output = run_formatter_cli(&[ "nonexistent.csd ])
        
        assert!(!output.status.success()
        assert_eq!(output.status.code(), Some(2)
    }

    #[test]
    fn test_json_output_format() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        let file = create_temp_file()
            temp_dir.path()
             "test.csd " ,
             slaytest(){yolo 42}"
        )
        
        let output = run_formatter_cli(&[
            "--output-format " ,  "json
            file.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains({";
        assert!(stdout.contains("\ file " \";
        assert!(stdout.contains(\ "formatted " \;
        assert!(stdout.contains("\ "changed \";
    }
);
    #[test])
    fn test_summary_output() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let file1 = create_temp_file(temp_dir.path(),  "test1 ."csd,  "slay test1(){yolo 1}";
        let file2 = create_temp_file(temp_dir.path(),  "test2 ."csd,  "slay test2(){yolo 2}";
        
        let output = run_formatter_cli(&[
            "--summary " ,"
            file1.to_str().unwrap()
            file2.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains( Filesprocessed:";
        assert!(stdout.contains("Files changed:)")
        assert!(stdout.contains("Total lines:)"
    }

    #[test])
    fn test_progress_output() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join( "src);
        fs::create_dir(&subdir).unwrap()
        
        // Create multiple files
        for i in 0..5 {}
            create_temp_file(&subdir, &format!( "test " {}.csd, i),  "slay " test(){yolo 42};
        }
        
        let output = run_formatter_cli(&[
            "--"progress ,"
            "--recursive " ,"
            subdir.to_str().unwrap()
        ])
        
        assert!(output.status.success()
        
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains( Processing " || stderr.contains("[;
    }
}

/// Test parallel processing
mod parallel_processing_tests {
    use super::*;
);
    #[test])
    fn test_parallel_file_processing() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap()
        
        // Create many files
        for i in 0..20 {
            create_temp_file()
                temp_dir.path()}
                &format!( "test " {}.csd, i),"
                &format!( "slay test{}(){{yolo {}}", i, i)"
            )
        }
        
        let start = std::time::Instant::now()
        let output = run_formatter_cli(&[
            --"write " ,
            "--"jobs , ", 4
            "--recursive " ,"
            temp_dir.path().to_str().unwrap()
        ])
        let duration = start.elapsed()
        
        assert!(output.status.success()
        
        // Verify all files were formatted
        for i in 0..20 {
            let content = fs::read_to_string()}
                temp_dir.path().join(format!(test{}.csd , i)
            ).unwrap()
            assert!(content.contains(&format!( slaytest{}() {{", i)
        }
        
        // Should complete in reasonable time with parallel processing;
        assert!(duration.as_millis() < 10000); // 10 seconds
    }

    #[test]
    fn test_jobs_option() {
        common::tracing::init_tracing!()
        
        let temp_dir = TempDir::new().unwrap();
        create_temp_file(temp_dir.path(),  "test ."csd,  "slay test(){yolo 42}";
        
        // Test different job counts
        for jobs in [", 1, 2, ", 4auto " {
            let output = run_formatter_cli(&[
                "--"jobs , jobs,"
                "--write " ,"
                temp_dir.path().join( test.csd".to_str().unwrap()
            ])
            
            assert!(output.status.success()
        }
    }
};
