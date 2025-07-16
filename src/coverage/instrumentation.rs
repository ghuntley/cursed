/// Code instrumentation for coverage collection
/// 
/// Instruments CURSED source files with coverage tracking calls
/// to collect runtime execution data.

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use regex::Regex;

use crate::lexer::{Lexer, Token, TokenType, Position};
use crate::parser::{Parser, AstNode};

/// Instrument CURSED source files for coverage collection
pub fn instrument_cursed_files(source_dirs: &[PathBuf], output_dir: &Path) -> io::Result<()> {
    let instrumented_dir = output_dir.join("instrumented");
    fs::create_dir_all(&instrumented_dir)?;
    
    for source_dir in source_dirs {
        instrument_directory(source_dir, &instrumented_dir)?;
    }
    
    Ok(())
}

/// Instrument all CURSED files in a directory
fn instrument_directory(source_dir: &Path, output_dir: &Path) -> io::Result<()> {
    if !source_dir.exists() {
        return Ok(());
    }
    
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let relative_path = path.strip_prefix(source_dir).unwrap();
            let output_subdir = output_dir.join(relative_path);
            fs::create_dir_all(&output_subdir)?;
            instrument_directory(&path, &output_subdir)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("csd") {
            let relative_path = path.strip_prefix(source_dir).unwrap();
            let output_file = output_dir.join(relative_path);
            
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)?;
            }
            
            instrument_cursed_file(&path, &output_file)?;
        }
    }
    
    Ok(())
}

/// Instrument a single CURSED source file
fn instrument_cursed_file(input_file: &Path, output_file: &Path) -> io::Result<()> {
    let source_content = fs::read_to_string(input_file)?;
    let instrumented_content = instrument_cursed_source(&source_content, input_file)?;
    fs::write(output_file, instrumented_content)?;
    Ok(())
}

/// Instrument CURSED source code with coverage tracking
fn instrument_cursed_source(source: &str, file_path: &Path) -> io::Result<String> {
    let mut instrumenter = CursedInstrumenter::new(file_path);
    instrumenter.instrument(source)
}

/// CURSED source code instrumenter
struct CursedInstrumenter {
    file_path: PathBuf,
    line_instruments: Vec<LineInstrumentation>,
    function_instruments: Vec<FunctionInstrumentation>,
    branch_instruments: Vec<BranchInstrumentation>,
}

#[derive(Debug)]
struct LineInstrumentation {
    line_number: u32,
    insert_position: usize,
    is_executable: bool,
}

#[derive(Debug)]
struct FunctionInstrumentation {
    function_name: String,
    start_line: u32,
    insert_position: usize,
}

#[derive(Debug)]
struct BranchInstrumentation {
    line_number: u32,
    branch_id: String,
    condition_start: usize,
    condition_end: usize,
}

impl CursedInstrumenter {
    fn new(file_path: &Path) -> Self {
        Self {
            file_path: file_path.to_path_buf(),
            line_instruments: Vec::new(),
            function_instruments: Vec::new(),
            branch_instruments: Vec::new(),
        }
    }

    /// Instrument the source code with coverage tracking
    fn instrument(&mut self, source: &str) -> io::Result<String> {
        // Parse the source to find instrumentation points
        self.analyze_source(source)?;
        
        // Apply instrumentation
        let mut instrumented = String::new();
        
        // Add coverage import at the top
        instrumented.push_str("// CURSED Coverage Instrumentation\n");
        instrumented.push_str("yeet \"coverage_runtime\"\n\n");
        
        // Add original source with instrumentation
        let lines: Vec<&str> = source.lines().collect();
        for (line_idx, line) in lines.iter().enumerate() {
            let line_number = (line_idx + 1) as u32;
            
            // Add line instrumentation
            if self.should_instrument_line(line_number, line) {
                let file_path_str = self.file_path.to_string_lossy();
                instrumented.push_str(&format!(
                    "coverage_runtime.record_line({}, {}); ",
                    quote_string(&file_path_str),
                    line_number
                ));
            }
            
            // Add function instrumentation
            if let Some(func_name) = self.extract_function_name(line) {
                let file_path_str = self.file_path.to_string_lossy();
                instrumented.push_str(&format!(
                    "coverage_runtime.record_function({}, {}); ",
                    quote_string(&file_path_str),
                    quote_string(&func_name)
                ));
            }
            
            // Add branch instrumentation
            let instrumented_line = self.instrument_branches_in_line(line, line_number);
            instrumented.push_str(&instrumented_line);
            instrumented.push('\n');
        }
        
        Ok(instrumented)
    }

    /// Analyze source to find instrumentation points
    fn analyze_source(&mut self, source: &str) -> io::Result<()> {
        // Try to parse with lexer/parser for detailed analysis
        if let Ok(mut lexer) = Lexer::new(source).tokenize() {
            // Analyze tokens for instrumentation points
            self.analyze_tokens(&lexer);
        } else {
            // Fall back to line-by-line analysis
            self.analyze_lines(source);
        }
        
        Ok(())
    }

    fn analyze_tokens(&mut self, tokens: &[Token]) {
        for (i, token) in tokens.iter().enumerate() {
            match &token.token_type {
                TokenType::Identifier(name) if name == "slay" => {
                    // Function declaration
                    if let Some(func_token) = tokens.get(i + 1) {
                        if let TokenType::Identifier(func_name) = &func_token.token_type {
                            self.function_instruments.push(FunctionInstrumentation {
                                function_name: func_name.clone(),
                                start_line: token.position.line as u32,
                                insert_position: 0, // Will be calculated later
                            });
                        }
                    }
                }
                TokenType::Identifier(name) if matches!(name.as_str(), "lowkey" | "highkey" | "around" | "bestie") => {
                    // Branch point
                    self.branch_instruments.push(BranchInstrumentation {
                        line_number: token.position.line as u32,
                        branch_id: format!("{}:{}", token.position.line, name),
                        condition_start: 0,
                        condition_end: 0,
                    });
                }
                _ => {}
            }
        }
    }

    fn analyze_lines(&mut self, source: &str) {
        for (line_idx, line) in source.lines().enumerate() {
            let line_number = (line_idx + 1) as u32;
            
            // Check if line is executable
            if self.is_executable_line(line) {
                self.line_instruments.push(LineInstrumentation {
                    line_number,
                    insert_position: 0,
                    is_executable: true,
                });
            }
            
            // Check for function declarations
            if let Some(func_name) = self.extract_function_name(line) {
                self.function_instruments.push(FunctionInstrumentation {
                    function_name: func_name,
                    start_line: line_number,
                    insert_position: 0,
                });
            }
            
            // Check for branch points
            if self.has_branch_point(line) {
                self.branch_instruments.push(BranchInstrumentation {
                    line_number,
                    branch_id: format!("{}:branch", line_number),
                    condition_start: 0,
                    condition_end: 0,
                });
            }
        }
    }

    fn should_instrument_line(&self, line_number: u32, line: &str) -> bool {
        self.is_executable_line(line) && !line.trim_start().starts_with("coverage_runtime")
    }

    fn is_executable_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || 
           trimmed.starts_with("//") || 
           trimmed.starts_with("fr fr") || 
           trimmed.starts_with("no cap") {
            return false;
        }
        
        // Skip braces only
        if matches!(trimmed, "{" | "}") {
            return false;
        }
        
        // Skip imports/packages
        if trimmed.starts_with("yeet") || 
           trimmed.starts_with("vibe") || 
           trimmed.starts_with("fam") {
            return false;
        }
        
        // Skip variable declarations without initialization
        if trimmed.starts_with("sus") && !trimmed.contains('=') {
            return false;
        }
        
        true
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        // Match function declarations: slay function_name(
        let func_regex = Regex::new(r"slay\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        func_regex.captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn has_branch_point(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Check for conditional statements
        trimmed.contains("lowkey") ||   // if
        trimmed.contains("highkey") ||  // else if
        trimmed.contains("around") ||   // while
        trimmed.contains("bestie") ||   // for
        trimmed.contains("ready")       // select
    }

    fn instrument_branches_in_line(&self, line: &str, line_number: u32) -> String {
        let mut instrumented_line = line.to_string();
        
        // Instrument conditional expressions
        if line.contains("lowkey") {
            instrumented_line = self.instrument_conditional(&instrumented_line, line_number, "if");
        }
        
        if line.contains("highkey") {
            instrumented_line = self.instrument_conditional(&instrumented_line, line_number, "else_if");
        }
        
        if line.contains("around") {
            instrumented_line = self.instrument_conditional(&instrumented_line, line_number, "while");
        }
        
        instrumented_line
    }

    fn instrument_conditional(&self, line: &str, line_number: u32, branch_type: &str) -> String {
        // Extract condition from line
        if let Some(condition_start) = line.find('(') {
            if let Some(condition_end) = line.rfind(')') {
                let before_condition = &line[..condition_start + 1];
                let condition = &line[condition_start + 1..condition_end];
                let after_condition = &line[condition_end..];
                
                let file_path_str = self.file_path.to_string_lossy();
                let branch_id = format!("{}:{}", line_number, branch_type);
                
                // Wrap condition with branch tracking
                let instrumented_condition = format!(
                    "coverage_runtime.record_branch({}, {}, {})",
                    quote_string(&file_path_str),
                    quote_string(&branch_id),
                    condition
                );
                
                return format!("{}{}{}", before_condition, instrumented_condition, after_condition);
            }
        }
        
        line.to_string()
    }
}

/// Quote a string for use in generated code
fn quote_string(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\\\"").replace('\\', "\\\\"))
}

/// Create coverage runtime module for instrumented code
pub fn create_coverage_runtime_module(output_dir: &Path) -> io::Result<()> {
    let runtime_dir = output_dir.join("coverage_runtime");
    fs::create_dir_all(&runtime_dir)?;
    
    let runtime_content = r#"
// CURSED Coverage Runtime Module
// Provides runtime coverage tracking functions

sus global_coverage_collector = cringe

slay init_coverage_collector() {
    // Initialize global coverage collector
    // This would interface with the Rust coverage system
}

slay record_line(file_path tea, line_number normie) {
    // Record line execution
    // Interface with Rust collector
}

slay record_function(file_path tea, function_name tea) {
    // Record function execution
    // Interface with Rust collector
}

slay record_branch(file_path tea, branch_id tea, condition lit) lit {
    // Record branch execution and return condition result
    // Interface with Rust collector
    damn condition
}

slay finalize_coverage() {
    // Finalize coverage collection
    // Save coverage data
}
"#;
    
    fs::write(runtime_dir.join("mod.csd"), runtime_content)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_instrument_simple_function() {
        let source = r#"
slay test_function(param tea) {
    vibez.spill("Hello")
    damn based
}
"#;
        
        let mut instrumenter = CursedInstrumenter::new(&PathBuf::from("test.csd"));
        let result = instrumenter.instrument(source).unwrap();
        
        assert!(result.contains("coverage_runtime.record_line"));
        assert!(result.contains("coverage_runtime.record_function"));
    }

    #[test]
    fn test_instrument_conditional() {
        let source = r#"
lowkey (x > 5) {
    vibez.spill("Greater")
}
"#;
        
        let mut instrumenter = CursedInstrumenter::new(&PathBuf::from("test.csd"));
        let result = instrumenter.instrument(source).unwrap();
        
        assert!(result.contains("coverage_runtime.record_branch"));
    }

    #[test]
    fn test_skip_non_executable_lines() {
        let source = r#"
// This is a comment
fr fr This is also a comment

{
}

yeet "module"
"#;
        
        let mut instrumenter = CursedInstrumenter::new(&PathBuf::from("test.csd"));
        let result = instrumenter.instrument(source).unwrap();
        
        // Should not instrument comment lines, braces, or imports
        let coverage_calls = result.matches("coverage_runtime.record_line").count();
        assert_eq!(coverage_calls, 0);
    }
}
"#
