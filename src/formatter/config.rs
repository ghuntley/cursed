//! Formatter configuration for CURSED code formatting

use serde::{Deserialize, Serialize};

/// Configuration for the CURSED formatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Use tabs for indentation instead of spaces
    pub use_tabs: bool,
    
    /// Number of spaces per indentation level (ignored if use_tabs is true)
    pub indent_size: usize,
    
    /// Maximum line length before wrapping
    pub max_line_length: usize,
    
    /// Add spaces around operators
    pub spaces_around_operators: bool,
    
    /// Maximum number of array elements on a single line
    pub max_array_elements_single_line: usize,
    
    /// Maximum number of map elements on a single line
    pub max_map_elements_single_line: usize,
    
    /// Maximum number of struct fields on a single line
    pub max_struct_fields_single_line: usize,
    
    /// Number of blank lines between top-level statements
    pub blank_lines_between_statements: usize,
    
    /// Number of blank lines between functions
    pub blank_lines_between_functions: usize,
    
    /// Group import statements
    pub group_imports: bool,
    
    /// Sort imports alphabetically
    pub sort_imports: bool,
    
    /// Format comments
    pub format_comments: bool,
    
    /// Preserve empty lines
    pub preserve_empty_lines: bool,
    
    /// Maximum number of consecutive empty lines
    pub max_empty_lines: usize,
    
    /// Trailing comma in arrays and structs
    pub trailing_comma: bool,
    
    /// Break function parameters into multiple lines if too long
    pub break_function_parameters: bool,
    
    /// Break function call arguments into multiple lines if too long
    pub break_function_arguments: bool,
    
    /// Space before opening brace
    pub space_before_brace: bool,
    
    /// Space inside parentheses
    pub space_inside_parentheses: bool,
    
    /// Space inside square brackets
    pub space_inside_brackets: bool,
    
    /// Space inside curly braces
    pub space_inside_braces: bool,
    
    /// Align consecutive assignments
    pub align_assignments: bool,
    
    /// Align consecutive struct fields
    pub align_struct_fields: bool,
    
    /// Insert final newline
    pub insert_final_newline: bool,
    
    /// Trim trailing whitespace
    pub trim_trailing_whitespace: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            use_tabs: false,
            indent_size: 4,
            max_line_length: 100,
            spaces_around_operators: true,
            max_array_elements_single_line: 5,
            max_map_elements_single_line: 3,
            max_struct_fields_single_line: 3,
            blank_lines_between_statements: 0,
            blank_lines_between_functions: 1,
            group_imports: true,
            sort_imports: true,
            format_comments: true,
            preserve_empty_lines: true,
            max_empty_lines: 2,
            trailing_comma: true,
            break_function_parameters: true,
            break_function_arguments: true,
            space_before_brace: true,
            space_inside_parentheses: false,
            space_inside_brackets: false,
            space_inside_braces: false,
            align_assignments: false,
            align_struct_fields: false,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
        }
    }
}

impl FormatterConfig {
    /// Load configuration from TOML file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to TOML file
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a compact configuration (minimal spacing)
    pub fn compact() -> Self {
        Self {
            use_tabs: false,
            indent_size: 2,
            max_line_length: 120,
            spaces_around_operators: false,
            max_array_elements_single_line: 10,
            max_map_elements_single_line: 8,
            max_struct_fields_single_line: 8,
            blank_lines_between_statements: 0,
            blank_lines_between_functions: 0,
            group_imports: true,
            sort_imports: true,
            format_comments: false,
            preserve_empty_lines: false,
            max_empty_lines: 0,
            trailing_comma: false,
            break_function_parameters: false,
            break_function_arguments: false,
            space_before_brace: false,
            space_inside_parentheses: false,
            space_inside_brackets: false,
            space_inside_braces: false,
            align_assignments: false,
            align_struct_fields: false,
            insert_final_newline: false,
            trim_trailing_whitespace: true,
        }
    }
    
    /// Create a verbose configuration (maximum spacing)
    pub fn verbose() -> Self {
        Self {
            use_tabs: false,
            indent_size: 4,
            max_line_length: 80,
            spaces_around_operators: true,
            max_array_elements_single_line: 1,
            max_map_elements_single_line: 1,
            max_struct_fields_single_line: 1,
            blank_lines_between_statements: 1,
            blank_lines_between_functions: 2,
            group_imports: true,
            sort_imports: true,
            format_comments: true,
            preserve_empty_lines: true,
            max_empty_lines: 3,
            trailing_comma: true,
            break_function_parameters: true,
            break_function_arguments: true,
            space_before_brace: true,
            space_inside_parentheses: true,
            space_inside_brackets: true,
            space_inside_braces: true,
            align_assignments: true,
            align_struct_fields: true,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
        }
    }
    
    /// Create a Go-style configuration (similar to gofmt)
    pub fn go_style() -> Self {
        Self {
            use_tabs: true,
            indent_size: 1,
            max_line_length: 100,
            spaces_around_operators: true,
            max_array_elements_single_line: 5,
            max_map_elements_single_line: 3,
            max_struct_fields_single_line: 3,
            blank_lines_between_statements: 0,
            blank_lines_between_functions: 1,
            group_imports: true,
            sort_imports: true,
            format_comments: true,
            preserve_empty_lines: true,
            max_empty_lines: 1,
            trailing_comma: false,
            break_function_parameters: true,
            break_function_arguments: true,
            space_before_brace: true,
            space_inside_parentheses: false,
            space_inside_brackets: false,
            space_inside_braces: false,
            align_assignments: false,
            align_struct_fields: false,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
        }
    }
}

/// Configuration presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigPreset {
    Default,
    Compact,
    Verbose,
    GoStyle,
}

impl ConfigPreset {
    pub fn to_config(self) -> FormatterConfig {
        match self {
            ConfigPreset::Default => FormatterConfig::default(),
            ConfigPreset::Compact => FormatterConfig::compact(),
            ConfigPreset::Verbose => FormatterConfig::verbose(),
            ConfigPreset::GoStyle => FormatterConfig::go_style(),
        }
    }
}

impl std::str::FromStr for ConfigPreset {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(ConfigPreset::Default),
            "compact" => Ok(ConfigPreset::Compact),
            "verbose" => Ok(ConfigPreset::Verbose),
            "go" | "gostyle" | "go-style" => Ok(ConfigPreset::GoStyle),
            _ => Err(format!("Unknown preset: {}", s)),
        }
    }
}
