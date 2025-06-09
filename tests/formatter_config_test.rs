//! Configuration tests for the CURSED code formatter
//!
//! These tests verify configuration file loading, validation, and option handling

use cursed::tools::{FormatterConfig, BraceStyle};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

#[path = "common/mod.rs"]
mod common;

/// Helper function to create a temporary config file
fn create_config_file(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    let mut file = File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path
}

/// Test different formatting configuration options
mod config_option_tests {
    use super::*;

    #[test]
    fn test_default_configuration() {
        init_tracing!();
        
        let config = FormatterConfig::default();
        
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
        assert!(config.format_comments);
        assert!(config.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, 2);
        assert!(!config.use_tabs);
        assert_eq!(config.tab_width, 4);
    }

    #[test]
    fn test_custom_indent_configuration() {
        init_tracing!();
        
        for indent_size in [2, 4, 6, 8] {
            let config = FormatterConfig {
                indent_size,
                ..FormatterConfig::default()
            };
            
            assert_eq!(config.indent_size, indent_size);
            
            // Validate that the config is actually used
            assert!(FormatterConfig::validate(&config).is_ok());
        }
    }

    #[test]
    fn test_brace_style_configuration() {
        init_tracing!();
        
        let styles = [
            BraceStyle::SameLine,
            BraceStyle::NextLine,
            BraceStyle::NextLineUnindented,
        ];
        
        for style in &styles {
            let config = FormatterConfig {
                brace_style: *style,
                ..FormatterConfig::default()
            };
            
            assert_eq!(config.brace_style, *style);
            assert!(FormatterConfig::validate(&config).is_ok());
        }
    }

    #[test]
    fn test_line_width_configuration() {
        init_tracing!();
        
        let valid_widths = [50, 80, 100, 120, 150, 200];
        
        for width in &valid_widths {
            let config = FormatterConfig {
                line_width: *width,
                ..FormatterConfig::default()
            };
            
            assert_eq!(config.line_width, *width);
            assert!(FormatterConfig::validate(&config).is_ok());
        }
    }

    #[test]
    fn test_spacing_configuration() {
        init_tracing!();
        
        let combinations = [
            (true, true),   // spaces around operators and after commas
            (true, false),  // spaces around operators, no spaces after commas
            (false, true),  // no spaces around operators, spaces after commas
            (false, false), // no spaces anywhere
        ];
        
        for (operators, commas) in &combinations {
            let config = FormatterConfig {
                spaces_around_operators: *operators,
                space_after_comma: *commas,
                ..FormatterConfig::default()
            };
            
            assert_eq!(config.spaces_around_operators, *operators);
            assert_eq!(config.space_after_comma, *commas);
            assert!(FormatterConfig::validate(&config).is_ok());
        }
    }

    #[test]
    fn test_comment_configuration() {
        init_tracing!();
        
        let config_format_comments = FormatterConfig {
            format_comments: true,
            ..FormatterConfig::default()
        };
        
        let config_preserve_comments = FormatterConfig {
            format_comments: false,
            ..FormatterConfig::default()
        };
        
        assert!(config_format_comments.format_comments);
        assert!(!config_preserve_comments.format_comments);
        assert!(FormatterConfig::validate(&config_format_comments).is_ok());
        assert!(FormatterConfig::validate(&config_preserve_comments).is_ok());
    }

    #[test]
    fn test_empty_line_configuration() {
        init_tracing!();
        
        let test_cases = [
            (true, 0),   // preserve empty lines, no limit
            (true, 1),   // preserve empty lines, max 1
            (true, 2),   // preserve empty lines, max 2
            (true, 5),   // preserve empty lines, max 5
            (false, 0),  // don't preserve empty lines
        ];
        
        for (preserve, max_lines) in &test_cases {
            let config = FormatterConfig {
                preserve_empty_lines: *preserve,
                max_empty_lines: *max_lines,
                ..FormatterConfig::default()
            };
            
            assert_eq!(config.preserve_empty_lines, *preserve);
            assert_eq!(config.max_empty_lines, *max_lines);
            assert!(FormatterConfig::validate(&config).is_ok());
        }
    }

    #[test]
    fn test_tab_configuration() {
        init_tracing!();
        
        let config_tabs = FormatterConfig {
            use_tabs: true,
            tab_width: 8,
            ..FormatterConfig::default()
        };
        
        let config_spaces = FormatterConfig {
            use_tabs: false,
            indent_size: 4,
            ..FormatterConfig::default()
        };
        
        assert!(config_tabs.use_tabs);
        assert_eq!(config_tabs.tab_width, 8);
        assert!(!config_spaces.use_tabs);
        assert_eq!(config_spaces.indent_size, 4);
        
        assert!(FormatterConfig::validate(&config_tabs).is_ok());
        assert!(FormatterConfig::validate(&config_spaces).is_ok());
    }
}

/// Test configuration file loading and validation
mod config_file_tests {
    use super::*;

    #[test]
    fn test_toml_config_loading() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"
indent_size = 8
line_width = 120
brace_style = "next-line"
spaces_around_operators = false
space_after_comma = true
format_comments = false
preserve_empty_lines = true
max_empty_lines = 3
use_tabs = true
tab_width = 8
"#;
        
        let config_path = create_config_file(temp_dir.path(), "config.toml", config_content);
        let config = FormatterConfig::from_file(&config_path).unwrap();
        
        assert_eq!(config.indent_size, 8);
        assert_eq!(config.line_width, 120);
        assert_eq!(config.brace_style, BraceStyle::NextLine);
        assert!(!config.spaces_around_operators);
        assert!(config.space_after_comma);
        assert!(!config.format_comments);
        assert!(config.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, 3);
        assert!(config.use_tabs);
        assert_eq!(config.tab_width, 8);
    }

    #[test]
    fn test_partial_config_loading() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"
indent_size = 6
brace_style = "next-line-unindented"
"#;
        
        let config_path = create_config_file(temp_dir.path(), "partial.toml", config_content);
        let config = FormatterConfig::from_file(&config_path).unwrap();
        
        // Specified values should be loaded
        assert_eq!(config.indent_size, 6);
        assert_eq!(config.brace_style, BraceStyle::NextLineUnindented);
        
        // Unspecified values should use defaults
        assert_eq!(config.line_width, 100);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
    }

    #[test]
    fn test_json_config_loading() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"{
    "indent_size": 2,
    "line_width": 80,
    "brace_style": "same-line",
    "spaces_around_operators": true,
    "space_after_comma": false,
    "format_comments": true,
    "preserve_empty_lines": false,
    "max_empty_lines": 1,
    "use_tabs": false,
    "tab_width": 4
}"#;
        
        let config_path = create_config_file(temp_dir.path(), "config.json", config_content);
        let config = FormatterConfig::from_file(&config_path).unwrap();
        
        assert_eq!(config.indent_size, 2);
        assert_eq!(config.line_width, 80);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(!config.space_after_comma);
        assert!(config.format_comments);
        assert!(!config.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, 1);
        assert!(!config.use_tabs);
        assert_eq!(config.tab_width, 4);
    }

    #[test]
    fn test_yaml_config_loading() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"
indent_size: 4
line_width: 100
brace_style: same-line
spaces_around_operators: true
space_after_comma: true
format_comments: true
preserve_empty_lines: true
max_empty_lines: 2
use_tabs: false
tab_width: 4
"#;
        
        let config_path = create_config_file(temp_dir.path(), "config.yaml", config_content);
        let config = FormatterConfig::from_file(&config_path).unwrap();
        
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
        assert!(config.format_comments);
        assert!(config.preserve_empty_lines);
        assert_eq!(config.max_empty_lines, 2);
        assert!(!config.use_tabs);
        assert_eq!(config.tab_width, 4);
    }

    #[test]
    fn test_config_discovery() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        
        // Test various config file names
        let config_names = [
            ".cursed-fmt.toml",
            ".cursed-format.toml", 
            "cursed-fmt.toml",
            ".cursed.toml",
        ];
        
        for name in &config_names {
            let config_content = "indent_size = 6\n";
            let config_path = create_config_file(temp_dir.path(), name, config_content);
            
            let discovered = FormatterConfig::discover_config(temp_dir.path()).unwrap();
            assert_eq!(discovered.indent_size, 6);
            
            // Clean up for next iteration
            fs::remove_file(config_path).unwrap();
        }
    }

    #[test]
    fn test_config_precedence() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple config files
        create_config_file(temp_dir.path(), ".cursed.toml", "indent_size = 2\n");
        create_config_file(temp_dir.path(), "cursed-fmt.toml", "indent_size = 4\n");
        create_config_file(temp_dir.path(), ".cursed-fmt.toml", "indent_size = 8\n");
        
        let config = FormatterConfig::discover_config(temp_dir.path()).unwrap();
        
        // Should use highest precedence file (.cursed-fmt.toml)
        assert_eq!(config.indent_size, 8);
    }
}

/// Test conflicting configuration scenarios
mod config_conflict_tests {
    use super::*;

    #[test]
    fn test_invalid_config_validation() {
        init_tracing!();
        
        // Test invalid indent size
        let invalid_indent = FormatterConfig {
            indent_size: 0,
            ..FormatterConfig::default()
        };
        assert!(FormatterConfig::validate(&invalid_indent).is_err());
        
        // Test invalid line width
        let invalid_width = FormatterConfig {
            line_width: 10, // Too narrow
            ..FormatterConfig::default()
        };
        assert!(FormatterConfig::validate(&invalid_width).is_err());
        
        // Test invalid tab width
        let invalid_tab = FormatterConfig {
            use_tabs: true,
            tab_width: 0,
            ..FormatterConfig::default()
        };
        assert!(FormatterConfig::validate(&invalid_tab).is_err());
        
        // Test invalid max empty lines
        let invalid_empty = FormatterConfig {
            max_empty_lines: 1000, // Unreasonably high
            ..FormatterConfig::default()
        };
        assert!(FormatterConfig::validate(&invalid_empty).is_err());
    }

    #[test]
    fn test_conflicting_tab_space_config() {
        init_tracing!();
        
        // Using tabs but also specifying indent_size should work
        // (indent_size used for continuation lines, etc.)
        let config = FormatterConfig {
            use_tabs: true,
            indent_size: 4,
            tab_width: 8,
            ..FormatterConfig::default()
        };
        
        assert!(FormatterConfig::validate(&config).is_ok());
    }

    #[test]
    fn test_empty_line_conflict_resolution() {
        init_tracing!();
        
        // Don't preserve empty lines but set max > 0
        let config = FormatterConfig {
            preserve_empty_lines: false,
            max_empty_lines: 3,
            ..FormatterConfig::default()
        };
        
        // Should be valid - max_empty_lines ignored when preserve_empty_lines is false
        assert!(FormatterConfig::validate(&config).is_ok());
    }

    #[test]
    fn test_malformed_config_files() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        
        // Invalid TOML
        let invalid_toml = "indent_size = [invalid syntax";
        let toml_path = create_config_file(temp_dir.path(), "invalid.toml", invalid_toml);
        assert!(FormatterConfig::from_file(&toml_path).is_err());
        
        // Invalid JSON
        let invalid_json = r#"{"indent_size": }"#;
        let json_path = create_config_file(temp_dir.path(), "invalid.json", invalid_json);
        assert!(FormatterConfig::from_file(&json_path).is_err());
        
        // Invalid YAML
        let invalid_yaml = "indent_size:\n  - invalid_structure";
        let yaml_path = create_config_file(temp_dir.path(), "invalid.yaml", invalid_yaml);
        assert!(FormatterConfig::from_file(&yaml_path).is_err());
    }

    #[test]
    fn test_unknown_config_options() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"
indent_size = 4
unknown_option = "should be ignored"
another_unknown = 42
brace_style = "same-line"
"#;
        
        let config_path = create_config_file(temp_dir.path(), "unknown.toml", config_content);
        let config = FormatterConfig::from_file(&config_path).unwrap();
        
        // Known options should be loaded
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        
        // Unknown options should be ignored (no error)
        assert!(FormatterConfig::validate(&config).is_ok());
    }

    #[test]
    fn test_type_mismatch_in_config() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let config_content = r#"
indent_size = "should be number"
brace_style = 42
spaces_around_operators = "true"
"#;
        
        let config_path = create_config_file(temp_dir.path(), "type_mismatch.toml", config_content);
        let result = FormatterConfig::from_file(&config_path);
        
        // Should fail to parse due to type mismatches
        assert!(result.is_err());
    }
}

/// Test configuration serialization and round-trip
mod config_serialization_tests {
    use super::*;

    #[test]
    fn test_toml_round_trip() {
        init_tracing!();
        
        let original_config = FormatterConfig {
            indent_size: 6,
            line_width: 80,
            brace_style: BraceStyle::NextLine,
            spaces_around_operators: false,
            space_after_comma: true,
            format_comments: false,
            preserve_empty_lines: true,
            max_empty_lines: 1,
            use_tabs: true,
            tab_width: 8,
        };
        
        // Serialize to TOML
        let toml_string = original_config.to_toml().unwrap();
        
        // Deserialize back
        let roundtrip_config = FormatterConfig::from_toml(&toml_string).unwrap();
        
        // Should be identical
        assert_eq!(original_config.indent_size, roundtrip_config.indent_size);
        assert_eq!(original_config.line_width, roundtrip_config.line_width);
        assert_eq!(original_config.brace_style, roundtrip_config.brace_style);
        assert_eq!(original_config.spaces_around_operators, roundtrip_config.spaces_around_operators);
        assert_eq!(original_config.space_after_comma, roundtrip_config.space_after_comma);
        assert_eq!(original_config.format_comments, roundtrip_config.format_comments);
        assert_eq!(original_config.preserve_empty_lines, roundtrip_config.preserve_empty_lines);
        assert_eq!(original_config.max_empty_lines, roundtrip_config.max_empty_lines);
        assert_eq!(original_config.use_tabs, roundtrip_config.use_tabs);
        assert_eq!(original_config.tab_width, roundtrip_config.tab_width);
    }

    #[test]
    fn test_json_round_trip() {
        init_tracing!();
        
        let original_config = FormatterConfig {
            indent_size: 2,
            line_width: 120,
            brace_style: BraceStyle::NextLineUnindented,
            spaces_around_operators: true,
            space_after_comma: false,
            format_comments: true,
            preserve_empty_lines: false,
            max_empty_lines: 0,
            use_tabs: false,
            tab_width: 4,
        };
        
        // Serialize to JSON
        let json_string = original_config.to_json().unwrap();
        
        // Deserialize back
        let roundtrip_config = FormatterConfig::from_json(&json_string).unwrap();
        
        // Should be identical
        assert_eq!(original_config.indent_size, roundtrip_config.indent_size);
        assert_eq!(original_config.line_width, roundtrip_config.line_width);
        assert_eq!(original_config.brace_style, roundtrip_config.brace_style);
        assert_eq!(original_config.spaces_around_operators, roundtrip_config.spaces_around_operators);
        assert_eq!(original_config.space_after_comma, roundtrip_config.space_after_comma);
        assert_eq!(original_config.format_comments, roundtrip_config.format_comments);
        assert_eq!(original_config.preserve_empty_lines, roundtrip_config.preserve_empty_lines);
        assert_eq!(original_config.max_empty_lines, roundtrip_config.max_empty_lines);
        assert_eq!(original_config.use_tabs, roundtrip_config.use_tabs);
        assert_eq!(original_config.tab_width, roundtrip_config.tab_width);
    }

    #[test]
    fn test_config_cloning() {
        init_tracing!();
        
        let original_config = FormatterConfig {
            indent_size: 8,
            line_width: 100,
            brace_style: BraceStyle::SameLine,
            spaces_around_operators: true,
            space_after_comma: true,
            format_comments: true,
            preserve_empty_lines: true,
            max_empty_lines: 2,
            use_tabs: false,
            tab_width: 4,
        };
        
        let cloned_config = original_config.clone();
        
        // Should be identical
        assert_eq!(original_config.indent_size, cloned_config.indent_size);
        assert_eq!(original_config.line_width, cloned_config.line_width);
        assert_eq!(original_config.brace_style, cloned_config.brace_style);
        assert_eq!(original_config.spaces_around_operators, cloned_config.spaces_around_operators);
        assert_eq!(original_config.space_after_comma, cloned_config.space_after_comma);
        assert_eq!(original_config.format_comments, cloned_config.format_comments);
        assert_eq!(original_config.preserve_empty_lines, cloned_config.preserve_empty_lines);
        assert_eq!(original_config.max_empty_lines, cloned_config.max_empty_lines);
        assert_eq!(original_config.use_tabs, cloned_config.use_tabs);
        assert_eq!(original_config.tab_width, cloned_config.tab_width);
    }

    #[test]
    fn test_config_debug_display() {
        init_tracing!();
        
        let config = FormatterConfig::default();
        
        let debug_string = format!("{:?}", config);
        let display_string = format!("{}", config);
        
        // Debug should contain field names
        assert!(debug_string.contains("indent_size"));
        assert!(debug_string.contains("brace_style"));
        
        // Display should be human-readable
        assert!(display_string.contains("Indent size"));
        assert!(display_string.contains("Line width"));
        assert!(display_string.contains("Brace style"));
    }
}

/// Test environment variable configuration
mod environment_config_tests {
    use super::*;
    use std::env;

    #[test]
    fn test_environment_variable_override() {
        init_tracing!();
        
        // Set environment variables
        env::set_var("CURSED_INDENT_SIZE", "8");
        env::set_var("CURSED_LINE_WIDTH", "120");
        env::set_var("CURSED_BRACE_STYLE", "next-line");
        
        let config = FormatterConfig::from_environment().unwrap();
        
        assert_eq!(config.indent_size, 8);
        assert_eq!(config.line_width, 120);
        assert_eq!(config.brace_style, BraceStyle::NextLine);
        
        // Clean up
        env::remove_var("CURSED_INDENT_SIZE");
        env::remove_var("CURSED_LINE_WIDTH");
        env::remove_var("CURSED_BRACE_STYLE");
    }

    #[test]
    fn test_config_precedence_order() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create config file
        create_config_file(temp_dir.path(), ".cursed-fmt.toml", "indent_size = 4\n");
        
        // Set environment variable
        env::set_var("CURSED_INDENT_SIZE", "8");
        
        // Command line override
        let cli_config = FormatterConfig {
            indent_size: 2,
            ..FormatterConfig::default()
        };
        
        // Test precedence: CLI > Environment > Config File > Default
        let final_config = FormatterConfig::with_precedence(
            Some(cli_config),
            Some(temp_dir.path()),
            true // use environment
        ).unwrap();
        
        // CLI should win
        assert_eq!(final_config.indent_size, 2);
        
        // Test without CLI override
        let env_config = FormatterConfig::with_precedence(
            None,
            Some(temp_dir.path()),
            true // use environment
        ).unwrap();
        
        // Environment should win over file
        assert_eq!(env_config.indent_size, 8);
        
        // Clean up
        env::remove_var("CURSED_INDENT_SIZE");
    }
}
