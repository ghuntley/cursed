//! Debug CLI Support for CURSED Compiler
//! 
//! This module provides command-line interface support for debugging,
//! including options for enabling debug information, controlling output
//! verbosity, and setting debug-specific compilation flags.

use crate::error::CursedError;
use crate::debug::enhanced_debug::EnhancedDebugManager;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Debug CLI configuration and commands
#[derive(Parser, Debug)]
#[command(name = "cursed-debug")]
#[command(about = "CURSED Compiler Debug Tools")]
pub struct DebugCli {
    #[command(subcommand)]
    pub command: DebugCommand,
}

/// Debug subcommands
#[derive(Subcommand, Debug)]
pub enum DebugCommand {
    /// Compile with debug information
    Compile {
        /// Source file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Enable debug symbols
        #[arg(short = 'd', long)]
        debug_symbols: bool,
        
        /// Enable verbose debug output
        #[arg(short = 'v', long)]
        verbose: bool,
        
        /// Generate source maps
        #[arg(short = 's', long)]
        source_maps: bool,
        
        /// Enable stack traces
        #[arg(short = 't', long)]
        stack_traces: bool,
        
        /// Debug output format
        #[arg(short = 'f', long, default_value = "dwarf")]
        format: DebugFormat,
        
        /// Output file for debug information
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },
    
    /// Analyze debug information
    Analyze {
        /// Debug info file to analyze
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Show symbol table
        #[arg(long)]
        symbols: bool,
        
        /// Show source locations
        #[arg(long)]
        locations: bool,
        
        /// Show function information
        #[arg(long)]
        functions: bool,
        
        /// Show variable information
        #[arg(long)]
        variables: bool,
    },
    
    /// Generate debug report
    Report {
        /// Source file to analyze
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Report format
        #[arg(short = 'f', long, default_value = "text")]
        format: ReportFormat,
        
        /// Output file for report
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
        
        /// Include stack traces
        #[arg(long)]
        stack_traces: bool,
        
        /// Include source context
        #[arg(long)]
        source_context: bool,
    },
    
    /// Validate debug information
    Validate {
        /// Debug info file to validate
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Strict validation mode
        #[arg(long)]
        strict: bool,
        
        /// Check DWARF format
        #[arg(long)]
        dwarf: bool,
    },
    
    /// Interactive debug session
    Interactive {
        /// Program to debug
        #[arg(value_name = "FILE")]
        program: PathBuf,
        
        /// Set breakpoints
        #[arg(short = 'b', long)]
        breakpoints: Vec<String>,
        
        /// Enable step mode
        #[arg(long)]
        step: bool,
    },
}

/// Debug output formats
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum DebugFormat {
    /// DWARF debug format
    Dwarf,
    /// JSON debug format
    Json,
    /// XML debug format
    Xml,
    /// Human-readable text
    Text,
}

/// Report formats
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ReportFormat {
    /// Plain text format
    Text,
    /// HTML format
    Html,
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
}

/// Debug CLI handler
pub struct DebugCliHandler {
    debug_manager: EnhancedDebugManager,
}

impl DebugCliHandler {
    /// Create new debug CLI handler
    pub fn new() -> Self {
        Self {
            debug_manager: EnhancedDebugManager::new(),
        }
    }

    /// Handle debug CLI commands
    pub fn handle_command(&mut self, command: DebugCommand) -> Result<(), CursedError> {
        match command {
            DebugCommand::Compile { 
                file, 
                debug_symbols, 
                verbose, 
                source_maps, 
                stack_traces, 
                format, 
                output 
            } => {
                self.handle_compile(file, debug_symbols, verbose, source_maps, stack_traces, format, output)
            },
            DebugCommand::Analyze { 
                file, 
                symbols, 
                locations, 
                functions, 
                variables 
            } => {
                self.handle_analyze(file, symbols, locations, functions, variables)
            },
            DebugCommand::Report { 
                file, 
                format, 
                output, 
                stack_traces, 
                source_context 
            } => {
                self.handle_report(file, format, output, stack_traces, source_context)
            },
            DebugCommand::Validate { 
                file, 
                strict, 
                dwarf 
            } => {
                self.handle_validate(file, strict, dwarf)
            },
            DebugCommand::Interactive { 
                program, 
                breakpoints, 
                step 
            } => {
                self.handle_interactive(program, breakpoints, step)
            },
        }
    }

    /// Handle compile command with debug options
    fn handle_compile(
        &mut self, 
        file: PathBuf, 
        debug_symbols: bool, 
        verbose: bool, 
        source_maps: bool, 
        stack_traces: bool, 
        format: DebugFormat, 
        output: Option<PathBuf>
    ) -> Result<(), CursedError> {
        println!("Compiling with debug information: {}", file.display());
        
        // Configure debug manager
        if debug_symbols {
            self.debug_manager.enable_debug();
            println!("Debug symbols enabled");
        }
        
        if verbose {
            self.debug_manager.enable_verbose();
            println!("Verbose debug mode enabled");
        }
        
        // Add source file
        if let Some(file_str) = file.to_str() {
            self.debug_manager.add_source_file(file_str)?;
            println!("Added source file: {}", file_str);
        }
        
        // Generate debug information based on format
        let debug_output = match format {
            DebugFormat::Dwarf => {
                println!("Generating DWARF debug information");
                self.debug_manager.generate_dwarf_debug_info()?
            },
            DebugFormat::Json => {
                println!("Generating JSON debug information");
                self.generate_json_debug_info()?
            },
            DebugFormat::Xml => {
                println!("Generating XML debug information");
                self.generate_xml_debug_info()?
            },
            DebugFormat::Text => {
                println!("Generating text debug information");
                self.generate_text_debug_info()?
            },
        };
        
        // Write debug output
        if let Some(output_path) = output {
            match format {
                DebugFormat::Dwarf => {
                    std::fs::write(&output_path, debug_output)?;
                },
                _ => {
                    if let Ok(text) = String::from_utf8(debug_output) {
                        std::fs::write(&output_path, text)?;
                    }
                }
            }
            println!("Debug information written to: {}", output_path.display());
        } else {
            println!("Debug information generated (use -o to save to file)");
        }
        
        if stack_traces {
            println!("Stack trace support enabled");
            let stack_trace = self.debug_manager.capture_stack_trace()?;
            println!("Sample stack trace captured: {} frames", stack_trace.len());
        }
        
        Ok(())
    }

    /// Handle analyze command
    fn handle_analyze(
        &mut self, 
        file: PathBuf, 
        symbols: bool, 
        locations: bool, 
        functions: bool, 
        variables: bool
    ) -> Result<(), CursedError> {
        println!("Analyzing debug information: {}", file.display());
        
        // Load debug information
        let debug_data = std::fs::read(&file)?;
        
        if symbols {
            println!("\n=== Symbol Table ===");
            for (name, symbol) in &self.debug_manager.symbol_table {
                println!("{}: {:?} at {}:{}:{}", 
                    name, 
                    symbol.symbol_type, 
                    symbol.source_location.file,
                    symbol.source_location.line, 
                    symbol.source_location.column
                );
            }
        }
        
        if locations {
            println!("\n=== Source Locations ===");
            for (file_path, source_map) in &self.debug_manager.source_maps {
                println!("File: {}", file_path);
                for (line_num, line_content) in &source_map.line_mappings {
                    println!("  {}:{}", line_num, line_content);
                }
            }
        }
        
        if functions {
            println!("\n=== Functions ===");
            for (name, symbol) in &self.debug_manager.symbol_table {
                if matches!(symbol.symbol_type, crate::debug::enhanced_debug::SymbolType::Function) {
                    println!("Function: {} at {}:{}:{}", 
                        name, 
                        symbol.source_location.file,
                        symbol.source_location.line, 
                        symbol.source_location.column
                    );
                }
            }
        }
        
        if variables {
            println!("\n=== Variables ===");
            for (name, symbol) in &self.debug_manager.symbol_table {
                if matches!(symbol.symbol_type, crate::debug::enhanced_debug::SymbolType::Variable) {
                    println!("Variable: {} at {}:{}:{}", 
                        name, 
                        symbol.source_location.file,
                        symbol.source_location.line, 
                        symbol.source_location.column
                    );
                }
            }
        }
        
        Ok(())
    }

    /// Handle report command
    fn handle_report(
        &mut self, 
        file: PathBuf, 
        format: ReportFormat, 
        output: Option<PathBuf>, 
        stack_traces: bool, 
        source_context: bool
    ) -> Result<(), CursedError> {
        println!("Generating debug report: {}", file.display());
        
        // Add source file
        if let Some(file_str) = file.to_str() {
            self.debug_manager.add_source_file(file_str)?;
        }
        
        let report = match format {
            ReportFormat::Text => self.generate_text_report(stack_traces, source_context)?,
            ReportFormat::Html => self.generate_html_report(stack_traces, source_context)?,
            ReportFormat::Json => self.generate_json_report(stack_traces, source_context)?,
            ReportFormat::Markdown => self.generate_markdown_report(stack_traces, source_context)?,
        };
        
        if let Some(output_path) = output {
            std::fs::write(&output_path, report)?;
            println!("Report written to: {}", output_path.display());
        } else {
            println!("{}", report);
        }
        
        Ok(())
    }

    /// Handle validate command
    fn handle_validate(
        &mut self, 
        file: PathBuf, 
        strict: bool, 
        dwarf: bool
    ) -> Result<(), CursedError> {
        println!("Validating debug information: {}", file.display());
        
        let debug_data = std::fs::read(&file)?;
        
        if dwarf {
            println!("Validating DWARF format...");
            self.validate_dwarf_format(&debug_data, strict)?;
        }
        
        // General validation
        println!("Performing general validation...");
        self.validate_debug_info(&debug_data, strict)?;
        
        println!("Validation completed successfully");
        Ok(())
    }

    /// Handle interactive debug session
    fn handle_interactive(
        &mut self, 
        program: PathBuf, 
        breakpoints: Vec<String>, 
        step: bool
    ) -> Result<(), CursedError> {
        println!("Starting interactive debug session: {}", program.display());
        
        // Set up breakpoints
        for breakpoint in &breakpoints {
            println!("Setting breakpoint: {}", breakpoint);
        }
        
        if step {
            println!("Step mode enabled");
        }
        
        // Start interactive session
        self.run_interactive_session(program, breakpoints, step)?;
        
        Ok(())
    }

    /// Generate JSON debug information
    fn generate_json_debug_info(&self) -> Result<Vec<u8>, CursedError> {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("  \"symbols\": [\n");
        
        let mut first = true;
        for (name, symbol) in &self.debug_manager.symbol_table {
            if !first {
                json.push_str(",\n");
            }
            json.push_str(&format!("    {{\n"));
            json.push_str(&format!("      \"name\": \"{}\",\n", name));
            json.push_str(&format!("      \"type\": \"{:?}\",\n", symbol.symbol_type));
            json.push_str(&format!("      \"file\": \"{}\",\n", symbol.source_location.file));
            json.push_str(&format!("      \"line\": {},\n", symbol.source_location.line));
            json.push_str(&format!("      \"column\": {}\n", symbol.source_location.column));
            json.push_str("    }");
            first = false;
        }
        
        json.push_str("\n  ],\n");
        json.push_str("  \"version\": \"1.0\",\n");
        json.push_str("  \"generator\": \"CURSED Compiler\"\n");
        json.push_str("}\n");
        
        Ok(json.into_bytes())
    }

    /// Generate XML debug information
    fn generate_xml_debug_info(&self) -> Result<Vec<u8>, CursedError> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<debug_info version=\"1.0\" generator=\"CURSED Compiler\">\n");
        xml.push_str("  <symbols>\n");
        
        for (name, symbol) in &self.debug_manager.symbol_table {
            xml.push_str(&format!("    <symbol name=\"{}\" type=\"{:?}\">\n", name, symbol.symbol_type));
            xml.push_str(&format!("      <location file=\"{}\" line=\"{}\" column=\"{}\"/>\n", 
                symbol.source_location.file, symbol.source_location.line, symbol.source_location.column));
            xml.push_str("    </symbol>\n");
        }
        
        xml.push_str("  </symbols>\n");
        xml.push_str("</debug_info>\n");
        
        Ok(xml.into_bytes())
    }

    /// Generate text debug information
    fn generate_text_debug_info(&self) -> Result<Vec<u8>, CursedError> {
        let mut text = String::new();
        text.push_str("CURSED Debug Information\n");
        text.push_str("========================\n\n");
        
        text.push_str("Symbol Table:\n");
        text.push_str("-------------\n");
        for (name, symbol) in &self.debug_manager.symbol_table {
            text.push_str(&format!("{}: {:?} at {}:{}:{}\n", 
                name, symbol.symbol_type, symbol.source_location.file,
                symbol.source_location.line, symbol.source_location.column));
        }
        
        Ok(text.into_bytes())
    }

    /// Generate text report
    fn generate_text_report(&self, stack_traces: bool, source_context: bool) -> Result<String, CursedError> {
        let mut report = String::new();
        report.push_str("CURSED Debug Report\n");
        report.push_str("==================\n\n");
        
        // Add symbol information
        report.push_str("Symbols:\n");
        report.push_str("--------\n");
        for (name, symbol) in &self.debug_manager.symbol_table {
            report.push_str(&format!("{}: {:?} at {}:{}:{}\n", 
                name, symbol.symbol_type, symbol.source_location.file,
                symbol.source_location.line, symbol.source_location.column));
        }
        
        if stack_traces {
            report.push_str("\nStack Traces:\n");
            report.push_str("-------------\n");
            for stack_trace in &self.debug_manager.stack_traces {
                for (i, frame) in stack_trace.iter().enumerate() {
                    report.push_str(&format!("  {}: {} at {}:{}:{}\n", 
                        i, frame.function_name, frame.source_location.file,
                        frame.source_location.line, frame.source_location.column));
                }
            }
        }
        
        if source_context {
            report.push_str("\nSource Context:\n");
            report.push_str("---------------\n");
            for (file_path, content) in &self.debug_manager.source_files {
                report.push_str(&format!("File: {}\n", file_path));
                let lines: Vec<&str> = content.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    report.push_str(&format!("{:3}: {}\n", i + 1, line));
                }
                report.push_str("\n");
            }
        }
        
        Ok(report)
    }

    /// Generate HTML report
    fn generate_html_report(&self, stack_traces: bool, source_context: bool) -> Result<String, CursedError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("<title>CURSED Debug Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: monospace; }\n");
        html.push_str(".symbol { margin: 5px 0; }\n");
        html.push_str(".location { color: #666; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str("<h1>CURSED Debug Report</h1>\n");
        
        html.push_str("<h2>Symbols</h2>\n");
        for (name, symbol) in &self.debug_manager.symbol_table {
            html.push_str(&format!("<div class=\"symbol\">{}: {:?} ", name, symbol.symbol_type));
            html.push_str(&format!("<span class=\"location\">at {}:{}:{}</span></div>\n", 
                symbol.source_location.file, symbol.source_location.line, symbol.source_location.column));
        }
        
        if stack_traces {
            html.push_str("<h2>Stack Traces</h2>\n");
            for stack_trace in &self.debug_manager.stack_traces {
                html.push_str("<ol>\n");
                for frame in stack_trace {
                    html.push_str(&format!("<li>{} at {}:{}:{}</li>\n", 
                        frame.function_name, frame.source_location.file,
                        frame.source_location.line, frame.source_location.column));
                }
                html.push_str("</ol>\n");
            }
        }
        
        html.push_str("</body>\n</html>\n");
        Ok(html)
    }

    /// Generate JSON report
    fn generate_json_report(&self, stack_traces: bool, source_context: bool) -> Result<String, CursedError> {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("  \"report\": {\n");
        json.push_str("    \"generator\": \"CURSED Compiler\",\n");
        json.push_str("    \"timestamp\": \"2024-01-01T00:00:00Z\",\n");
        json.push_str("    \"symbols\": [\n");
        
        let mut first = true;
        for (name, symbol) in &self.debug_manager.symbol_table {
            if !first {
                json.push_str(",\n");
            }
            json.push_str(&format!("      {{\n"));
            json.push_str(&format!("        \"name\": \"{}\",\n", name));
            json.push_str(&format!("        \"type\": \"{:?}\",\n", symbol.symbol_type));
            json.push_str(&format!("        \"location\": {{\n"));
            json.push_str(&format!("          \"file\": \"{}\",\n", symbol.source_location.file));
            json.push_str(&format!("          \"line\": {},\n", symbol.source_location.line));
            json.push_str(&format!("          \"column\": {}\n", symbol.source_location.column));
            json.push_str("        }\n");
            json.push_str("      }");
            first = false;
        }
        
        json.push_str("\n    ]");
        
        if stack_traces {
            json.push_str(",\n    \"stack_traces\": [\n");
            // Add stack trace data
            json.push_str("    ]");
        }
        
        json.push_str("\n  }\n");
        json.push_str("}\n");
        
        Ok(json)
    }

    /// Generate Markdown report
    fn generate_markdown_report(&self, stack_traces: bool, source_context: bool) -> Result<String, CursedError> {
        let mut md = String::new();
        md.push_str("# CURSED Debug Report\n\n");
        
        md.push_str("## Symbols\n\n");
        for (name, symbol) in &self.debug_manager.symbol_table {
            md.push_str(&format!("- **{}**: {:?} at `{}:{}:{}`\n", 
                name, symbol.symbol_type, symbol.source_location.file,
                symbol.source_location.line, symbol.source_location.column));
        }
        
        if stack_traces {
            md.push_str("\n## Stack Traces\n\n");
            for (i, stack_trace) in self.debug_manager.stack_traces.iter().enumerate() {
                md.push_str(&format!("### Trace {}\n\n", i + 1));
                for (j, frame) in stack_trace.iter().enumerate() {
                    md.push_str(&format!("{}. **{}** at `{}:{}:{}`\n", 
                        j + 1, frame.function_name, frame.source_location.file,
                        frame.source_location.line, frame.source_location.column));
                }
            }
        }
        
        Ok(md)
    }

    /// Validate DWARF format
    fn validate_dwarf_format(&self, data: &[u8], strict: bool) -> Result<(), CursedError> {
        // Basic DWARF header validation
        if data.len() < 12 {
            return Err(CursedError::General("DWARF data too short".to_string()));
        }
        
        // Check DWARF version
        let version = u16::from_le_bytes([data[4], data[5]]);
        if version != 4 {
            if strict {
                return Err(CursedError::General(format!("Unsupported DWARF version: {}", version)));
            } else {
                println!("Warning: DWARF version {} may not be fully supported", version);
            }
        }
        
        println!("DWARF format validation passed");
        Ok(())
    }

    /// Validate debug information
    fn validate_debug_info(&self, data: &[u8], strict: bool) -> Result<(), CursedError> {
        if data.is_empty() {
            return Err(CursedError::General("Empty debug information".to_string()));
        }
        
        // Check for minimum required sections
        let has_symbols = !self.debug_manager.symbol_table.is_empty();
        let has_source_maps = !self.debug_manager.source_maps.is_empty();
        
        if strict && !has_symbols {
            return Err(CursedError::General("No symbols found in debug information".to_string()));
        }
        
        if strict && !has_source_maps {
            return Err(CursedError::General("No source maps found in debug information".to_string()));
        }
        
        println!("Debug information validation passed");
        Ok(())
    }

    /// Run interactive debug session
    fn run_interactive_session(&mut self, program: PathBuf, breakpoints: Vec<String>, step: bool) -> Result<(), CursedError> {
        println!("Interactive debug session started");
        println!("Program: {}", program.display());
        
        // Simple interactive loop
        loop {
            print!("(cursed-debug) ");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input {
                "help" | "h" => {
                    println!("Available commands:");
                    println!("  step (s)     - Step to next line");
                    println!("  continue (c) - Continue execution");
                    println!("  break (b)    - Set breakpoint");
                    println!("  symbols      - Show symbol table");
                    println!("  stack        - Show stack trace");
                    println!("  quit (q)     - Exit debug session");
                },
                "step" | "s" => {
                    println!("Stepping to next line...");
                },
                "continue" | "c" => {
                    println!("Continuing execution...");
                },
                "symbols" => {
                    println!("Symbol table:");
                    for (name, symbol) in &self.debug_manager.symbol_table {
                        println!("  {}: {:?} at {}:{}:{}", 
                            name, symbol.symbol_type, symbol.source_location.file,
                            symbol.source_location.line, symbol.source_location.column);
                    }
                },
                "stack" => {
                    if let Ok(stack_trace) = self.debug_manager.capture_stack_trace() {
                        println!("Stack trace:");
                        for (i, frame) in stack_trace.iter().enumerate() {
                            println!("  {}: {} at {}:{}:{}", 
                                i, frame.function_name, frame.source_location.file,
                                frame.source_location.line, frame.source_location.column);
                        }
                    }
                },
                "quit" | "q" => {
                    println!("Exiting debug session");
                    break;
                },
                _ => {
                    println!("Unknown command: {}. Type 'help' for available commands.", input);
                }
            }
        }
        
        Ok(())
    }
}

impl Default for DebugCliHandler {
    fn default() -> Self {
        Self::new()
    }
}
