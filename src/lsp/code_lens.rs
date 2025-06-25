// Code lens implementation for CURSED language
// 
// Provides contextual information overlays including reference counts,
// test execution status, performance metrics, and memory usage hints.

use std::collections::HashMap;
use std::time::Duration;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};
use serde_json::Value;

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::*;

/// Types of code lenses available
#[derive(Debug, Clone, PartialEq)]
pub enum CodeLensType {
    /// Reference count for symbols
    ReferenceCount,
    /// Test execution status
    TestStatus,
    /// Performance metrics
    Performance,
    /// Memory usage hints
    MemoryUsage,
    /// Type information
    TypeInfo,
    /// Documentation link
    Documentation,
    /// Git blame information
    GitBlame,
    /// Coverage information
    Coverage,
}

/// Performance metrics for code lenses
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub avg_execution_time: Duration,
    pub max_execution_time: Duration,
    pub call_count: u64,
    pub memory_allocated: u64,
    pub cpu_usage: f32,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_execution_time: Duration::from_millis(0),
            max_execution_time: Duration::from_millis(0),
            call_count: 0,
            memory_allocated: 0,
            cpu_usage: 0.0,
        }
    }
}

/// Test execution status
#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    NotRun,
    Passed,
    Failed,
    Skipped,
    Running,
}

impl TestStatus {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            TestStatus::NotRun => "⚪",
            TestStatus::Passed => "✅",
            TestStatus::Failed => "❌",
            TestStatus::Skipped => "⏭️",
            TestStatus::Running => "🔄",
        }
    }
    
    pub fn to_color(&self) -> &'static str {
        match self {
            TestStatus::NotRun => "gray",
            TestStatus::Passed => "green",
            TestStatus::Failed => "red",
            TestStatus::Skipped => "yellow",
            TestStatus::Running => "blue",
        }
    }
}

/// Memory usage information
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub heap_allocated: u64,
    pub stack_size: u64,
    pub gc_collections: u32,
    pub leak_potential: bool,
}

impl Default for MemoryUsage {
    fn default() -> Self {
        Self {
            heap_allocated: 0,
            stack_size: 0,
            gc_collections: 0,
            leak_potential: false,
        }
    }
}

/// Symbol reference information
#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    pub definition_count: u32,
    pub reference_count: u32,
    pub write_count: u32,
    pub read_count: u32,
}

impl Default for ReferenceInfo {
    fn default() -> Self {
        Self {
            definition_count: 0,
            reference_count: 0,
            write_count: 0,
            read_count: 0,
        }
    }
}

/// Code lens data structure
#[derive(Debug, Clone)]
pub struct CursedCodeLens {
    pub range: Range,
    pub lens_type: CodeLensType,
    pub title: String,
    pub command: Option<Command>,
    pub tooltip: Option<String>,
    pub data: Option<Value>,
}

impl CursedCodeLens {
    /// Create a new code lens
    pub fn new(
        range: Range,
        lens_type: CodeLensType,
        title: String,
        command: Option<Command>,
        tooltip: Option<String>,
    ) -> Self {
        Self {
            range,
            lens_type,
            title,
            command,
            tooltip,
            data: None,
        }
    }
    
    /// Convert to LSP CodeLens
    pub fn to_lsp_code_lens(&self) -> CodeLens {
        CodeLens {
            range: self.range,
            command: self.command.clone(),
            data: self.data.clone(),
        }
    }
}

/// Code lens provider for CURSED language
pub struct CodeLensProvider {
    /// Symbol reference cache
    reference_cache: HashMap<String, ReferenceInfo>,
    /// Performance metrics cache
    performance_cache: HashMap<String, PerformanceMetrics>,
    /// Test status cache
    test_cache: HashMap<String, TestStatus>,
    /// Memory usage cache
    memory_cache: HashMap<String, MemoryUsage>,
    /// Enable various code lens types
    enable_references: bool,
    enable_tests: bool,
    enable_performance: bool,
    enable_memory: bool,
}

impl CodeLensProvider {
    /// Create a new code lens provider
    pub fn new() -> Self {
        Self {
            reference_cache: HashMap::new(),
            performance_cache: HashMap::new(),
            test_cache: HashMap::new(),
            memory_cache: HashMap::new(),
            enable_references: true,
            enable_tests: true,
            enable_performance: true,
            enable_memory: true,
        }
    }
    
    /// Configure which code lens types are enabled
    pub fn configure(
        &mut self,
        references: bool,
        tests: bool,
        performance: bool,
        memory: bool,
    ) {
        self.enable_references = references;
        self.enable_tests = tests;
        self.enable_performance = performance;
        self.enable_memory = memory;
    }
    
    /// Get code lenses for the given content
    #[instrument(skip(self, content))]
    pub async fn get_code_lenses(&self, content: &str, uri: &Url) -> Result<Vec<CursedCodeLens>, String> {
        debug!("Generating code lenses for document");
        
        let mut lenses = Vec::new();
        
        // TODO: Temporarily simplified - basic lens generation
        debug!("Generating basic code lenses for content analysis");
        
        // Create a simple lens for demonstration
        if !content.len() == 0 {
            let range = Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 10 },
            };
            
            lenses.push(CursedCodeLens::new(
                range,
                CodeLensType::TypeInfo,
                "CURSED".to_string(),
                None,
                Some("CURSED code detected".to_string()),
            ));
        }
        
        Ok(lenses)
    }
    
    /// Generate code lenses for function declarations
    async fn generate_function_lenses(
        &self,
        ast: &Program,
        lenses: &mut Vec<CursedCodeLens>,
    ) {
        for statement in &ast.statements {
            if let Some(func_decl) = statement.as_any().downcast_ref::<FunctionStatement>() {
                let range = self.get_function_range(func_decl);
                let func_name = &func_decl.name.value;
                
                // Reference count lens
                if self.enable_references {
                    let ref_info = self.get_reference_info(func_name);
                    let title = format!("{} references", ref_info.reference_count);
                    let command = Some(Command {
                        title: "Show References".to_string(),
                        command: "cursed.showReferences".to_string(),
                        arguments: Some(vec![
                            serde_json::json!(func_name),
                            serde_json::json!(range.start),
                        ]),
                    });
                    
                    lenses.push(CursedCodeLens::new(
                        range,
                        CodeLensType::ReferenceCount,
                        title,
                        command,
                        Some(format!("Function '{}' is referenced {} times", func_name, ref_info.reference_count)),
                    ));
                }
                
                // Performance metrics lens
                if self.enable_performance {
                    let perf_metrics = self.get_performance_metrics(func_name);
                    if perf_metrics.call_count > 0 {
                        let title = format!(
                            "⚡ {}ms avg ({}x called)",
                            perf_metrics.avg_execution_time.as_millis(),
                            perf_metrics.call_count
                        );
                        let command = Some(Command {
                            title: "Show Performance Details".to_string(),
                            command: "cursed.showPerformance".to_string(),
                            arguments: Some(vec![serde_json::json!(func_name)]),
                        });
                        
                        lenses.push(CursedCodeLens::new(
                            range,
                            CodeLensType::Performance,
                            title,
                            command,
                            Some(format!(
                                "Average: {}ms, Max: {}ms, Calls: {}, Memory: {} KB",
                                perf_metrics.avg_execution_time.as_millis(),
                                perf_metrics.max_execution_time.as_millis(),
                                perf_metrics.call_count,
                                perf_metrics.memory_allocated / 1024
                            )),
                        ));
                    }
                }
                
                // Memory usage lens
                if self.enable_memory {
                    let memory_usage = self.get_memory_usage(func_name);
                    if memory_usage.heap_allocated > 0 {
                        let title = format!("🧠 {} KB", memory_usage.heap_allocated / 1024);
                        let warning = if memory_usage.leak_potential { " ⚠️" } else { "" };
                        
                        lenses.push(CursedCodeLens::new(
                            range,
                            CodeLensType::MemoryUsage,
                            format!("{}{}", title, warning),
                            None,
                            Some(format!(
                                "Heap: {} KB, Stack: {} KB, GC Collections: {}{}",
                                memory_usage.heap_allocated / 1024,
                                memory_usage.stack_size / 1024,
                                memory_usage.gc_collections,
                                if memory_usage.leak_potential { ", Potential leak detected" } else { "" }
                            )),
                        ));
                    }
                }
                
                // Test status lens for test functions
                if self.enable_tests && self.is_test_function(func_name) {
                    let test_status = self.get_test_status(func_name);
                    let title = format!("{} {}", test_status.to_emoji(), func_name);
                    let command = Some(Command {
                        title: "Run Test".to_string(),
                        command: "cursed.runTest".to_string(),
                        arguments: Some(vec![serde_json::json!(func_name)]),
                    });
                    
                    lenses.push(CursedCodeLens::new(
                        range,
                        CodeLensType::TestStatus,
                        title,
                        command,
                        Some(format!("Test status: {:?}", test_status)),
                    ));
                }
            }
        }
    }
    
    /// Generate code lenses for type declarations
    async fn generate_type_lenses(
        &self,
        ast: &Program,
        lenses: &mut Vec<CursedCodeLens>,
    ) {
        for statement in &ast.statements {
            if let Some(struct_decl) = statement.as_any().downcast_ref::<SquadStatement>() {
                let range = self.get_struct_range(struct_decl);
                let struct_name = &struct_decl.name.value;
                    
                    if self.enable_references {
                        let ref_info = self.get_reference_info(struct_name);
                        let title = format!("{} implementations", ref_info.reference_count);
                        
                        lenses.push(CursedCodeLens::new(
                            range,
                            CodeLensType::ReferenceCount,
                            title,
                            None,
                            Some(format!("Struct '{}' is used {} times", struct_name, ref_info.reference_count)),
                        ));
                    }
            } else if let Some(interface_decl) = statement.as_any().downcast_ref::<CollabStatement>() {
                let range = self.get_interface_range(interface_decl);
                let interface_name = &interface_decl.name.value;
                    
                    if self.enable_references {
                        let ref_info = self.get_reference_info(interface_name);
                        let title = format!("{} implementations", ref_info.reference_count);
                        
                        lenses.push(CursedCodeLens::new(
                            range,
                            CodeLensType::ReferenceCount,
                            title,
                            None,
                            Some(format!("Interface '{}' has {} implementations", interface_name, ref_info.reference_count)),
                        ));
                    }
            }
        }
    }
    
    /// Generate code lenses for test functions
    async fn generate_test_lenses(
        &self,
        ast: &Program,
        lenses: &mut Vec<CursedCodeLens>,
    ) {
        // Look for test files and functions
        for statement in &ast.statements {
            if let Some(func_decl) = statement.as_any().downcast_ref::<FunctionStatement>() {
                let func_name = &func_decl.name.value;
                
                if self.is_test_function(func_name) {
                    let range = self.get_function_range(func_decl);
                    let test_status = self.get_test_status(func_name);
                    
                    let title = format!("Run Test {}", test_status.to_emoji());
                    let command = Some(Command {
                        title: "Run Test".to_string(),
                        command: "cursed.runTest".to_string(),
                        arguments: Some(vec![serde_json::json!(func_name)]),
                    });
                    
                    lenses.push(CursedCodeLens::new(
                        range,
                        CodeLensType::TestStatus,
                        title,
                        command,
                        Some(format!("Execute test: {}", func_name)),
                    ));
                }
            }
        }
    }
    
    /// Generate code lenses for variable declarations
    async fn generate_variable_lenses(
        &self,
        ast: &Program,
        lenses: &mut Vec<CursedCodeLens>,
    ) {
        for statement in &ast.statements {
            if let Some(var_decl) = statement.as_any().downcast_ref::<VariableStatement>() {
                let range = self.get_variable_range(var_decl);
                let var_name = &var_decl.name;
                
                if self.enable_references {
                    let ref_info = self.get_reference_info(var_name);
                    if ref_info.reference_count > 0 {
                        let title = format!("{} refs", ref_info.reference_count);
                        
                        lenses.push(CursedCodeLens::new(
                            range,
                            CodeLensType::ReferenceCount,
                            title,
                            None,
                            Some(format!(
                                "Variable '{}': {} reads, {} writes",
                                var_name, ref_info.read_count, ref_info.write_count
                            )),
                        ));
                    }
                }
            }
        }
    }
    
    /// Generate basic lenses from lexical analysis when parsing fails
    async fn generate_lexical_lenses(
        &self,
        content: &str,
        lenses: &mut Vec<CursedCodeLens>,
    ) {
        // Simple pattern-based detection for now
        let lines: Vec<&str> = content.split('\n').collect();
        
        for (line_index, line) in lines.iter().enumerate() {
            // Look for function-like patterns
            if line.contains("slay") {
                let range = Range {
                    start: Position {
                        line: line_index as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line_index as u32,
                        character: line.len() as u32,
                    },
                };
                
                lenses.push(CursedCodeLens::new(
                    range,
                    CodeLensType::TypeInfo,
                    "function".to_string(),
                    None,
                    Some("Function declaration".to_string()),
                ));
            }
        }
    }
    
    /// Get range for a function declaration
    fn get_function_range(&self, func_decl: &FunctionStatement) -> Range {
        Range {
            start: Position {
                line: (func_decl.name.line - 1) as u32,
                character: func_decl.name.column as u32,
            },
            end: Position {
                line: (func_decl.name.line - 1) as u32,
                character: (func_decl.name.column + func_decl.name.name.len()) as u32,
            },
        }
    }
    
    /// Get range for a struct declaration
    fn get_struct_range(&self, struct_decl: &SquadStatement) -> Range {
        Range {
            start: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: struct_decl.name.column as u32,
            },
            end: Position {
                line: (struct_decl.name.line - 1) as u32,
                character: (struct_decl.name.column + struct_decl.name.name.len()) as u32,
            },
        }
    }
    
    /// Get range for an interface declaration
    fn get_interface_range(&self, interface_decl: &CollabStatement) -> Range {
        Range {
            start: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: interface_decl.name.column as u32,
            },
            end: Position {
                line: (interface_decl.name.line - 1) as u32,
                character: (interface_decl.name.column + interface_decl.name.name.len()) as u32,
            },
        }
    }
    
    /// Get range for a variable declaration
    fn get_variable_range(&self, var_decl: &VariableStatement) -> Range {
        // Since VariableStatement doesn't have position info, return a default range
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: var_decl.name.len() as u32,
            },
        }
    }
    
    /// Get range for a token
    fn get_token_range(&self, token: &Token, lines: &[&str]) -> Option<Range> {
        if token.line == 0 || token.line > lines.len() {
            return None;
        }
        
        Some(Range {
            start: Position {
                line: (token.line - 1) as u32,
                character: token.column as u32,
            },
            end: Position {
                line: (token.line - 1) as u32,
                character: (token.column + token.lexeme.len()) as u32,
            },
        })
    }
    
    /// Get reference information for a symbol
    fn get_reference_info(&self, symbol: &str) -> ReferenceInfo {
        self.reference_cache
            .get(symbol)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get performance metrics for a symbol
    fn get_performance_metrics(&self, symbol: &str) -> PerformanceMetrics {
        self.performance_cache
            .get(symbol)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get test status for a symbol
    fn get_test_status(&self, symbol: &str) -> TestStatus {
        self.test_cache
            .get(symbol)
            .cloned()
            .unwrap_or(TestStatus::NotRun)
    }
    
    /// Get memory usage for a symbol
    fn get_memory_usage(&self, symbol: &str) -> MemoryUsage {
        self.memory_cache
            .get(symbol)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Check if a function is a test function
    fn is_test_function(&self, name: &str) -> bool {
        name.starts_with("test_") || 
        name.ends_with("_test") ||
        name.contains("_test_")
    }
    
    /// Update reference information
    pub fn update_reference_info(&mut self, symbol: String, info: ReferenceInfo) {
        self.reference_cache.insert(symbol, info);
    }
    
    /// Update performance metrics
    pub fn update_performance_metrics(&mut self, symbol: String, metrics: PerformanceMetrics) {
        self.performance_cache.insert(symbol, metrics);
    }
    
    /// Update test status
    pub fn update_test_status(&mut self, symbol: String, status: TestStatus) {
        self.test_cache.insert(symbol, status);
    }
    
    /// Update memory usage
    pub fn update_memory_usage(&mut self, symbol: String, usage: MemoryUsage) {
        self.memory_cache.insert(symbol, usage);
    }
    
    /// Resolve a code lens (for lazy resolution)
    pub async fn resolve_code_lens(&self, mut code_lens: CodeLens) -> Result<CodeLens, String> {
        // Add any additional data or commands that need to be resolved lazily
        if let Some(data) = &code_lens.data {
            if let Some(symbol) = data.get("symbol").and_then(|s| s.as_str()) {
                // Update command with current data
                match data.get("type").and_then(|t| t.as_str()) {
                    Some("references") => {
                        let ref_info = self.get_reference_info(symbol);
                        code_lens.command = Some(Command {
                            title: format!("{} references", ref_info.reference_count),
                            command: "cursed.showReferences".to_string(),
                            arguments: Some(vec![serde_json::json!(symbol)]),
                        });
                    }
                    Some("performance") => {
                        let perf_metrics = self.get_performance_metrics(symbol);
                        code_lens.command = Some(Command {
                            title: format!("⚡ {}ms avg", perf_metrics.avg_execution_time.as_millis()),
                            command: "cursed.showPerformance".to_string(),
                            arguments: Some(vec![serde_json::json!(symbol)]),
                        });
                    }
                    _ => {}
                }
            }
        }
        
        Ok(code_lens)
    }
}

impl Default for CodeLensProvider {
    fn default() -> Self {
        Self::new()
    }
}

