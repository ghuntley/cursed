//! Debug Integration for LLVM Codegen
//! 
//! This module provides integration between the LLVM code generator and
//! the enhanced debug information system, including:
//! - Source location tracking throughout compilation
//! - Debug symbol generation during codegen
//! - DWARF information integration
//! - Stack trace support for runtime errors

use crate::error::{CursedError, SourceLocation};
use crate::debug::enhanced_debug::{
    EnhancedDebugManager, DebugSymbol, SymbolType, RuntimeState,
    create_debug_symbol, create_stack_frame, create_variable_debug_info
};
use crate::ast::{Statement, Expression, Program};
use std::collections::HashMap;

/// Debug-aware LLVM code generator wrapper
pub struct DebugIntegratedCodegen {
    pub debug_manager: EnhancedDebugManager,
    pub current_location: Option<SourceLocation>,
    pub function_stack: Vec<String>,
    pub variable_locations: HashMap<String, SourceLocation>,
    pub debug_symbols_enabled: bool,
    pub source_map_enabled: bool,
}

impl DebugIntegratedCodegen {
    /// Create new debug-integrated codegen
    pub fn new() -> Self {
        Self {
            debug_manager: EnhancedDebugManager::new(),
            current_location: None,
            function_stack: Vec::new(),
            variable_locations: HashMap::new(),
            debug_symbols_enabled: true,
            source_map_enabled: true,
        }
    }

    /// Enable debug symbols generation
    pub fn enable_debug_symbols(&mut self) {
        self.debug_symbols_enabled = true;
        self.debug_manager.enable_debug();
    }

    /// Enable source map generation
    pub fn enable_source_maps(&mut self) {
        self.source_map_enabled = true;
    }

    /// Add source file to debug manager
    pub fn add_source_file(&mut self, file_path: &str) -> Result<(), CursedError> {
        self.debug_manager.add_source_file(file_path)
    }

    /// Set current source location for debug info
    pub fn set_current_location(&mut self, location: SourceLocation) {
        self.current_location = Some(location);
    }

    /// Enter function for debug tracking
    pub fn enter_function(&mut self, function_name: &str, location: SourceLocation) -> Result<(), CursedError> {
        self.function_stack.push(function_name.to_string());
        self.set_current_location(location.clone());
        
        // Add debug symbol for function
        if self.debug_symbols_enabled {
            let symbol = create_debug_symbol(function_name, SymbolType::Function, location);
            self.debug_manager.add_debug_symbol(symbol);
        }
        
        Ok(())
    }

    /// Exit function for debug tracking
    pub fn exit_function(&mut self) {
        self.function_stack.pop();
    }

    /// Add variable debug information
    pub fn add_variable(&mut self, name: &str, var_type: &str, location: SourceLocation) -> Result<(), CursedError> {
        self.variable_locations.insert(name.to_string(), location.clone());
        
        // Add debug symbol for variable
        if self.debug_symbols_enabled {
            let symbol = create_debug_symbol(name, SymbolType::Variable, location);
            self.debug_manager.add_debug_symbol(symbol);
        }
        
        Ok(())
    }

    /// Generate LLVM IR with debug information
    pub fn generate_ir_with_debug(&mut self, program: &Program) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Add debug information header
        ir.push_str("; CURSED Language Debug Information\n");
        ir.push_str("; Generated with enhanced debug support\n\n");
        
        // Add debug metadata
        ir.push_str("!llvm.dbg.cu = !{!0}\n");
        ir.push_str("!llvm.module.flags = !{!1, !2, !3}\n\n");
        
        // Generate compilation unit debug info
        ir.push_str("!0 = distinct !DICompileUnit(");
        ir.push_str("language: DW_LANG_C99, ");
        ir.push_str("file: !4, ");
        ir.push_str("producer: \"CURSED Compiler\", ");
        ir.push_str("isOptimized: false, ");
        ir.push_str("runtimeVersion: 0, ");
        ir.push_str("emissionKind: FullDebug, ");
        ir.push_str("enums: !5, ");
        ir.push_str("retainedTypes: !5, ");
        ir.push_str("subprograms: !6, ");
        ir.push_str("globals: !5, ");
        ir.push_str("imports: !5)\n");
        
        // Add metadata flags
        ir.push_str("!1 = !{i32 2, !\"Dwarf Version\", i32 4}\n");
        ir.push_str("!2 = !{i32 2, !\"Debug Info Version\", i32 3}\n");
        ir.push_str("!3 = !{i32 1, !\"wchar_size\", i32 4}\n");
        ir.push_str("!4 = !DIFile(filename: \"main.csd\", directory: \".\")\n");
        ir.push_str("!5 = !{}\n");
        ir.push_str("!6 = !{}\n\n");
        
        // Generate main program IR
        for (i, statement) in program.statements.iter().enumerate() {
            let statement_ir = self.generate_statement_with_debug(statement, i)?;
            ir.push_str(&statement_ir);
        }
        
        // Generate debug symbol table
        if self.debug_symbols_enabled {
            ir.push_str(&self.generate_debug_symbol_table()?);
        }
        
        Ok(ir)
    }

    /// Generate statement with debug information
    fn generate_statement_with_debug(&mut self, statement: &Statement, statement_index: usize) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        // Add debug location metadata
        ir.push_str(&format!("; Statement {} debug location\n", statement_index));
        
        match statement {
            Statement::Function(func_stmt) => {
                let location = SourceLocation {
                    file: "main.csd".to_string(),
                    line: statement_index + 1,
                    column: 1,
                    offset: 0,
                };
                
                self.enter_function(&func_stmt.name, location.clone())?;
                
                // Generate function with debug info
                ir.push_str(&format!("define i32 @{}() ", func_stmt.name));
                ir.push_str("!dbg !7 {\n");
                
                // Add function debug metadata
                ir.push_str(&format!("!7 = distinct !DISubprogram("));
                ir.push_str(&format!("name: \"{}\", ", func_stmt.name));
                ir.push_str("scope: !4, ");
                ir.push_str(&format!("file: !4, line: {}, ", location.line));
                ir.push_str("type: !8, ");
                ir.push_str("scopeLine: 1, ");
                ir.push_str("spFlags: DISPFlagDefinition, ");
                ir.push_str("unit: !0, ");
                ir.push_str("retainedNodes: !5)\n");
                
                ir.push_str("!8 = !DISubroutineType(types: !9)\n");
                ir.push_str("!9 = !{!10}\n");
                ir.push_str("!10 = !DIBasicType(name: \"int\", size: 32, encoding: DW_ATE_signed)\n");
                
                // Generate function body
                for (body_index, body_stmt) in func_stmt.body.iter().enumerate() {
                    let body_ir = self.generate_statement_with_debug(body_stmt, body_index)?;
                    ir.push_str(&body_ir);
                }
                
                ir.push_str("  ret i32 0\n");
                ir.push_str("}\n\n");
                
                self.exit_function();
            },
            Statement::Let(let_stmt) => {
                if let crate::ast::LetTarget::Single(var_name) = &let_stmt.target {
                    let location = SourceLocation {
                        file: "main.csd".to_string(),
                        line: statement_index + 1,
                        column: 1,
                        offset: 0,
                    };
                    
                    self.add_variable(var_name, "i32", location.clone())?;
                    
                    // Generate variable declaration with debug info
                    ir.push_str(&format!("  %{} = alloca i32, align 4, !dbg !{}\n", var_name, 11 + statement_index));
                    ir.push_str(&format!("  call void @llvm.dbg.declare(metadata i32* %{}, metadata !{}, metadata !DIExpression()), !dbg !{}\n", 
                                        var_name, 12 + statement_index, 13 + statement_index));
                    
                    // Add variable debug metadata
                    ir.push_str(&format!("!{} = !DILocation(line: {}, column: 1, scope: !7)\n", 
                                        11 + statement_index, location.line));
                    ir.push_str(&format!("!{} = !DILocalVariable(name: \"{}\", scope: !7, file: !4, line: {}, type: !10)\n", 
                                        12 + statement_index, var_name, location.line));
                    ir.push_str(&format!("!{} = !DILocation(line: {}, column: 1, scope: !7)\n", 
                                        13 + statement_index, location.line));
                }
            },
            Statement::Expression(expr) => {
                let location = SourceLocation {
                    file: "main.csd".to_string(),
                    line: statement_index + 1,
                    column: 1,
                    offset: 0,
                };
                
                self.set_current_location(location.clone());
                
                // Generate expression with debug info
                let expr_ir = self.generate_expression_with_debug(expr, statement_index)?;
                ir.push_str(&expr_ir);
            },
            _ => {
                // Handle other statement types
                ir.push_str(&format!("  ; Other statement type at line {}\n", statement_index + 1));
            }
        }
        
        Ok(ir)
    }

    /// Generate expression with debug information
    fn generate_expression_with_debug(&mut self, expression: &Expression, expr_index: usize) -> Result<String, CursedError> {
        let mut ir = String::new();
        
        match expression {
            Expression::Call(func_call) => {
                // Generate debug-aware function call
                ir.push_str(&format!("  ; Function call at line {}\n", expr_index + 1));
            },
            Expression::Integer(value) => {
                ir.push_str(&format!("  ; Integer literal: {} at line {}\n", value, expr_index + 1));
            },
            Expression::String(value) => {
                ir.push_str(&format!("  ; String literal: \"{}\" at line {}\n", value, expr_index + 1));
            },
            _ => {
                ir.push_str(&format!("  ; Expression at line {}\n", expr_index + 1));
            }
        }
        
        Ok(ir)
    }

    /// Generate debug symbol table
    fn generate_debug_symbol_table(&self) -> Result<String, CursedError> {
        let mut symbols = String::new();
        
        symbols.push_str("; Debug Symbol Table\n");
        symbols.push_str("; Generated by CURSED enhanced debug system\n\n");
        
        symbols.push_str(".section .debug_info,\"\",@progbits\n");
        symbols.push_str(".Ldebug_info0:\n");
        symbols.push_str("  .long 0x1234  ; Length of Compilation Unit Info\n");
        symbols.push_str("  .value 0x4    ; DWARF version number\n");
        symbols.push_str("  .long .Ldebug_abbrev0  ; Offset into Abbrev. Section\n");
        symbols.push_str("  .byte 0x8     ; Pointer Size (in bytes)\n");
        
        // Add compilation unit DIE
        symbols.push_str("  .uleb128 0x1  ; (DIE (0xb) DW_TAG_compile_unit)\n");
        symbols.push_str("  .ascii \"CURSED Compiler\\0\"  ; DW_AT_producer\n");
        symbols.push_str("  .byte 0xc     ; DW_AT_language\n");
        symbols.push_str("  .ascii \"main.csd\\0\"  ; DW_AT_name\n");
        symbols.push_str("  .ascii \".\\0\"  ; DW_AT_comp_dir\n");
        
        // Add debug symbols from debug manager
        for (name, symbol) in self.debug_manager.symbol_table.iter() {
            match symbol.symbol_type {
                SymbolType::Function => {
                    symbols.push_str(&format!("  .uleb128 0x2  ; Function symbol\n"));
                    symbols.push_str(&format!("  .ascii \"{}\\0\"  ; Function name\n", name));
                    symbols.push_str(&format!("  .long {}  ; Line number\n", symbol.source_location.line));
                    symbols.push_str(&format!("  .long {}  ; Column number\n", symbol.source_location.column));
                },
                SymbolType::Variable => {
                    symbols.push_str(&format!("  .uleb128 0x3  ; Variable symbol\n"));
                    symbols.push_str(&format!("  .ascii \"{}\\0\"  ; Variable name\n", name));
                    symbols.push_str(&format!("  .long {}  ; Line number\n", symbol.source_location.line));
                    symbols.push_str(&format!("  .long {}  ; Column number\n", symbol.source_location.column));
                },
                _ => {
                    symbols.push_str(&format!("  .uleb128 0x4  ; Other symbol\n"));
                    symbols.push_str(&format!("  .ascii \"{}\\0\"  ; Symbol name\n", name));
                }
            }
        }
        
        symbols.push_str("  .byte 0  ; End of symbols\n");
        symbols.push_str(".Ldebug_info_end:\n\n");
        
        // Add abbreviation table
        symbols.push_str(".section .debug_abbrev,\"\",@progbits\n");
        symbols.push_str(".Ldebug_abbrev0:\n");
        symbols.push_str("  .uleb128 0x1  ; Abbreviation Code\n");
        symbols.push_str("  .uleb128 0x11 ; DW_TAG_compile_unit\n");
        symbols.push_str("  .byte 0x1     ; DW_CHILDREN_yes\n");
        symbols.push_str("  .uleb128 0x25 ; DW_AT_producer\n");
        symbols.push_str("  .uleb128 0x8  ; DW_FORM_string\n");
        symbols.push_str("  .uleb128 0x13 ; DW_AT_language\n");
        symbols.push_str("  .uleb128 0xb  ; DW_FORM_data1\n");
        symbols.push_str("  .byte 0\n");
        symbols.push_str("  .byte 0\n");
        
        symbols.push_str("  .uleb128 0x2  ; Abbreviation Code\n");
        symbols.push_str("  .uleb128 0x2e ; DW_TAG_subprogram\n");
        symbols.push_str("  .byte 0x0     ; DW_CHILDREN_no\n");
        symbols.push_str("  .uleb128 0x3  ; DW_AT_name\n");
        symbols.push_str("  .uleb128 0x8  ; DW_FORM_string\n");
        symbols.push_str("  .uleb128 0x3a ; DW_AT_decl_line\n");
        symbols.push_str("  .uleb128 0x5  ; DW_FORM_data2\n");
        symbols.push_str("  .byte 0\n");
        symbols.push_str("  .byte 0\n");
        
        symbols.push_str("  .byte 0  ; End of abbreviations\n\n");
        
        Ok(symbols)
    }

    /// Get enhanced error message with debug context
    pub fn get_enhanced_error_message(&self, error: &CursedError) -> Result<String, CursedError> {
        if let Some(location) = &self.current_location {
            self.debug_manager.format_error_message(error, location)
        } else {
            Ok(format!("Error: {}", error))
        }
    }

    /// Generate runtime debug support
    pub fn generate_runtime_debug_support(&self) -> Result<String, CursedError> {
        let mut runtime_support = String::new();
        
        // Add runtime debug function declarations
        runtime_support.push_str("; Runtime Debug Support Functions\n");
        runtime_support.push_str("declare void @llvm.dbg.declare(metadata, metadata, metadata)\n");
        runtime_support.push_str("declare void @llvm.dbg.value(metadata, metadata, metadata)\n");
        runtime_support.push_str("declare i8* @cursed_capture_stack_trace()\n");
        runtime_support.push_str("declare void @cursed_print_stack_trace(i8*)\n");
        runtime_support.push_str("declare void @cursed_debug_print(i8*, i32, i32, i8*)\n");
        runtime_support.push_str("declare i64 @cursed_get_current_goroutine_id()\n");
        runtime_support.push_str("declare i8* @cursed_get_runtime_state()\n\n");
        
        // Add debug helper functions
        runtime_support.push_str("define void @cursed_debug_location(i8* %file, i32 %line, i32 %column, i8* %function) {\n");
        runtime_support.push_str("  call void @cursed_debug_print(i8* %file, i32 %line, i32 %column, i8* %function)\n");
        runtime_support.push_str("  ret void\n");
        runtime_support.push_str("}\n\n");
        
        runtime_support.push_str("define void @cursed_debug_variable(i8* %name, i8* %type, i8* %value) {\n");
        runtime_support.push_str("  ; Debug variable information\n");
        runtime_support.push_str("  call i32 @printf(i8* getelementptr inbounds ([32 x i8], [32 x i8]* @debug_var_format, i32 0, i32 0), i8* %name, i8* %type, i8* %value)\n");
        runtime_support.push_str("  ret void\n");
        runtime_support.push_str("}\n\n");
        
        // Add debug format strings
        runtime_support.push_str("@debug_var_format = private unnamed_addr constant [32 x i8] c\"DEBUG: %s (%s) = %s\\0A\\00\", align 1\n");
        runtime_support.push_str("@debug_loc_format = private unnamed_addr constant [32 x i8] c\"DEBUG: %s:%d:%d in %s\\0A\\00\", align 1\n");
        
        Ok(runtime_support)
    }
}

impl Default for DebugIntegratedCodegen {
    fn default() -> Self {
        Self::new()
    }
}

/// Stack trace utilities
pub struct StackTraceUtils;

impl StackTraceUtils {
    /// Capture current stack trace
    pub fn capture_stack_trace() -> Result<Vec<String>, CursedError> {
        // Platform-specific stack trace capture
        // This is a simplified implementation
        Ok(vec![
            "main+0x10".to_string(),
            "function_call+0x20".to_string(),
            "expression_eval+0x30".to_string(),
        ])
    }

    /// Format stack trace for display
    pub fn format_stack_trace(stack_trace: &[String]) -> String {
        let mut formatted = String::new();
        formatted.push_str("Stack trace:\n");
        
        for (i, frame) in stack_trace.iter().enumerate() {
            formatted.push_str(&format!("  {}: {}\n", i, frame));
        }
        
        formatted
    }
}
