// Advanced refactoring tools for CURSED language server
// 
// Provides comprehensive refactoring capabilities including rename symbol,
// extract function/variable, organize imports, and code transformation.

use std::collections::{HashMap, HashSet};
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument};

use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::Parser;
use crate::ast::*;

/// Refactoring provider for the LSP server
pub struct RefactoringProvider {
    /// Symbol cache for quick lookups
    symbol_cache: std::sync::RwLock<HashMap<String, Vec<SymbolInfo>>>,
    /// Import organization rules
    import_rules: ImportOrganizationRules,
}

/// Symbol information for refactoring
#[derive(Debug, Clone)]
struct SymbolInfo {
    name: String,
    kind: SymbolKind,
    location: Location,
    references: Vec<Location>,
    scope: ScopeInfo,
}

/// Scope information for symbol analysis
#[derive(Debug, Clone)]
struct ScopeInfo {
    scope_type: ScopeType,
    start_location: Location,
    end_location: Option<Location>,
    parent_scope: Option<Box<ScopeInfo>>,
    children: Vec<ScopeInfo>,
}

/// Types of scopes in CURSED
#[derive(Debug, Clone, PartialEq)]
enum ScopeType {
    Global,
    Function,
    Struct,
    Interface,
    Block,
    Loop,
    Conditional,
}

/// Import organization rules
#[derive(Debug, Clone)]
struct ImportOrganizationRules {
    /// Group standard library imports first
    group_stdlib_first: bool,
    /// Group external packages second
    group_external_second: bool,
    /// Group local imports last
    group_local_last: bool,
    /// Sort imports alphabetically within groups
    sort_alphabetically: bool,
    /// Remove unused imports
    remove_unused: bool,
    /// Add missing imports
    add_missing: bool,
}

impl Default for ImportOrganizationRules {
    fn default() -> Self {
        Self {
            group_stdlib_first: true,
            group_external_second: true,
            group_local_last: true,
            sort_alphabetically: true,
            remove_unused: true,
            add_missing: true,
        }
    }
}

/// Extract function options
#[derive(Debug, Clone)]
pub struct ExtractFunctionOptions {
    pub function_name: String,
    pub selection_range: Range,
    pub generate_return_statement: bool,
    pub extract_parameters: bool,
    pub extract_local_variables: bool,
}

/// Extract variable options
#[derive(Debug, Clone)]
pub struct ExtractVariableOptions {
    pub variable_name: String,
    pub selection_range: Range,
    pub is_constant: bool,
    pub replace_all_occurrences: bool,
}

/// Rename symbol options
#[derive(Debug, Clone)]
pub struct RenameSymbolOptions {
    pub old_name: String,
    pub new_name: String,
    pub include_comments: bool,
    pub include_strings: bool,
}

/// Code block analysis result
#[derive(Debug, Clone)]
struct CodeBlockAnalysis {
    uses_async: bool,
    has_return_value: bool,
    local_variables: Vec<String>,
    dependencies: Vec<String>,
    complexity_score: usize,
    variables_used: Vec<String>,
    variables_declared: Vec<String>,
    functions_called: Vec<String>,
    has_return: bool,
}

/// Import statement information
#[derive(Debug, Clone)]
struct ImportStatement {
    path: String,
    line: usize,
    is_used: bool,
}

/// Function parameter for extracted functions
#[derive(Debug, Clone)]
struct FunctionParameter {
    name: String,
    type_name: String,
}

impl RefactoringProvider {
    /// Create a new refactoring provider
    pub fn new() -> Self {
        Self {
            symbol_cache: std::sync::RwLock::new(HashMap::new()),
            import_rules: ImportOrganizationRules::default(),
        }
    }

    /// Get symbol at position
    fn get_symbol_at_position(&self, content: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = content.split("\n").collect();
        if position.line as usize >= lines.len() {
            return None;
        }
        
        let line = lines[position.line as usize];
        if position.character as usize >= line.len() {
            return None;
        }
        
        // Simple word extraction - would be more sophisticated in real implementation
        let start = line.char_indices()
            .take_while(|(i, c)| *i < position.character as usize || c.is_alphanumeric() || *c == '_')
            .last()
            .map(|(i, _)| i)
            .unwrap_or(position.character as usize);
            
        let end = line.char_indices()
            .skip_while(|(i, _)| *i <= position.character as usize)
            .find(|(_, c)| !c.is_alphanumeric() && *c != '_')
            .map(|(i, _)| i)
            .unwrap_or(line.len());
            
        Some(line[start..end].to_string())
    }

    /// Get symbol range
    fn get_symbol_range(&self, content: &str, position: Position, symbol: &str) -> Option<Range> {
        let lines: Vec<&str> = content.split("\n").collect();
        if position.line as usize >= lines.len() {
            return None;
        }
        
        let line = lines[position.line as usize];
        if let Some(start_char) = line.find(symbol) {
            let start = Position::new(position.line, start_char as u32);
            let end = Position::new(position.line, (start_char + symbol.len()) as u32);
            Some(Range::new(start, end))
        } else {
            None
        }
    }

    /// Check if symbol is renameable
    fn is_renameable_symbol(&self, symbol: &str, content: &str) -> bool {
        // Simple check - exclude keywords and built-in types
        !matches!(symbol, "slay" | "sus" | "facts" | "lowkey" | "highkey" | "periodt" | "squad" | "collab")
    }

    /// Find all symbol references
    fn find_all_symbol_references(&self, content: &str, symbol: &str, uri: &Url) -> Vec<Location> {
        let mut references = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let mut start = 0;
            while let Some(pos) = line[start..].find(symbol) {
                let actual_pos = start + pos;
                let location = Location::new(
                    uri.clone(),
                    Range::new(
                        Position::new(line_num as u32, actual_pos as u32),
                        Position::new(line_num as u32, (actual_pos + symbol.len()) as u32),
                    ),
                );
                references.push(location);
                start = actual_pos + symbol.len();
            }
        }
        
        references
    }

    /// Get text in range
    fn get_text_in_range(&self, content: &str, range: &Range) -> Option<String> {
        let lines: Vec<&str> = content.split("\n").collect();
        if range.start.line == range.end.line {
            // Single line
            if let Some(line) = lines.get(range.start.line as usize) {
                let start = range.start.character as usize;
                let end = range.end.character as usize;
                if start <= line.len() && end <= line.len() && start <= end {
                    return Some(line[start..end].to_string());
                }
            }
        }
        // Multi-line would be more complex
        None
    }

    /// Analyze code block
    fn analyze_code_block(&self, code: &str) -> CodeBlockAnalysis {
        CodeBlockAnalysis {
            uses_async: code.contains("async") || code.contains("await"),
            has_return_value: code.contains("yolo "),
            local_variables: Vec::new(),
            dependencies: Vec::new(),
            complexity_score: code.split("\n").count(),
            variables_used: self.extract_variables_from_code(code),
            variables_declared: self.extract_declared_variables_from_code(code),
            functions_called: self.extract_function_calls_from_code(code),
            has_return: code.contains("vibez") || code.contains("bounce"),
        }
    }

    /// Generate function parameters
    fn generate_function_parameters(&self, analysis: &CodeBlockAnalysis) -> Vec<String> {
        // Simplified - would analyze variable usage
        Vec::new()
    }

    /// Infer return type
    fn infer_return_type(&self, analysis: &CodeBlockAnalysis) -> Option<String> {
        if analysis.has_return_value {
            Some("void".to_string())
        } else {
            None
        }
    }

    /// Generate function code
    fn generate_function_code(&self, name: &str, params: &[String], body: &str, return_type: Option<&str>) -> String {
        let param_str = params.join(", ");
        match return_type {
            Some(ret) => format!("slay {}({}) -> {} {{\n{}\n}}", name, param_str, ret, body),
            None => format!("slay {}({}) {{\n{}\n}}", name, param_str, body),
        }
    }

    /// Generate function call
    fn generate_function_call(&self, name: &str, args: &[String]) -> String {
        let args_str = args.join(", ");
        format!("{}({})", name, args_str)
    }

    /// Find function insertion point
    fn find_function_insertion_point(&self, content: &str, current_position: Position) -> Position {
        let lines: Vec<&str> = content.split("\n").collect();
        
        // Find the end of the current function or block
        for (i, line) in lines.iter().enumerate().skip(current_position.line as usize) {
            if line.trim().is_empty() || line.starts_with("slay ") {
                return Position::new(i as u32, 0);
            }
        }
        
        Position::new(lines.len() as u32, 0)
    }

    /// Find identical expressions
    fn find_identical_expressions(&self, content: &str, expression: &str) -> Vec<Range> {
        let mut ranges = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let mut start = 0;
            while let Some(pos) = line[start..].find(expression) {
                let actual_pos = start + pos;
                let range = Range::new(
                    Position::new(line_num as u32, actual_pos as u32),
                    Position::new(line_num as u32, (actual_pos + expression.len()) as u32),
                );
                ranges.push(range);
                start = actual_pos + expression.len();
            }
        }
        
        ranges
    }

    /// Find variable insertion point
    fn find_variable_insertion_point(&self, content: &str, current_position: Position) -> Position {
        let lines: Vec<&str> = content.split("\n").collect();
        
        // Find the start of the current function or block
        for i in (0..=current_position.line as usize).rev() {
            if let Some(line) = lines.get(i) {
                if line.trim().starts_with("slay ") || line.trim() == "{" {
                    return Position::new((i + 1) as u32, 0);
                }
            }
        }
        
        current_position
    }

    /// Analyze imports
    fn analyze_imports(&self, content: &str) -> Vec<ImportStatement> {
        let lines: Vec<&str> = content.split("\n").collect();
        let mut imports = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("yeet ") {
                imports.push(ImportStatement {
                    path: line.trim().trim_start_matches("yeet ").trim_matches('"').to_string(),
                    line: line_num,
                    is_used: true, // Simplified
                });
            }
        }
        
        imports
    }

    /// Organize import statements
    fn organize_import_statements(&self, imports: &[ImportStatement]) -> Vec<ImportStatement> {
        let mut organized = imports.to_vec();
        organized.sort_by(|a, b| a.path.cmp(&b.path));
        organized
    }

    /// Check if can extract function
    fn can_extract_function(&self, content: &str, range: &Range) -> bool {
        if let Some(text) = self.get_text_in_range(content, range) {
            !text.trim().is_empty() && text.split("\n").count() > 1
        } else {
            false
        }
    }

    /// Create extract function action
    fn create_extract_function_action(&self, content: &str, range: Range, function_name: &str) -> CodeAction {
        let title = format!("Extract function '{}'", function_name);
        CodeAction {
            title,
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None, // Would contain the actual edit
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        }
    }

    /// Check if can extract variable
    fn can_extract_variable(&self, content: &str, range: &Range) -> bool {
        if let Some(text) = self.get_text_in_range(content, range) {
            !text.trim().is_empty() && !text.contains('\n')
        } else {
            false
        }
    }

    /// Create extract variable action
    fn create_extract_variable_action(&self, content: &str, range: Range, variable_name: &str) -> CodeAction {
        let title = format!("Extract variable '{}'", variable_name);
        CodeAction {
            title,
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None, // Would contain the actual edit
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        }
    }

    /// Check if has imports
    fn has_imports(&self, content: &str) -> bool {
        content.split("\n").any(|line| line.trim().starts_with("yeet "))
    }

    /// Create organize imports action
    fn create_organize_imports_action(&self, content: &str) -> CodeAction {
        CodeAction {
            title: "Organize imports".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            diagnostics: None,
            edit: None, // Would contain the actual edit
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        }
    }

    /// Generate quick fix
    fn generate_quick_fix(&self, diagnostic: &Diagnostic, content: &str) -> Option<CodeAction> {
        // Simplified quick fix generation
        match diagnostic.message.as_str() {
            msg if msg.contains("undefined variable") => {
                Some(CodeAction {
                    title: "Declare variable".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: None,
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                })
            }
            _ => None,
        }
    }

    /// Prepare for rename symbol
    #[instrument(skip(self, content))]
    pub async fn prepare_rename(
        &self,
        content: &str,
        position: Position,
        uri: &Url,
    ) -> Option<PrepareRenameResponse> {
        debug!("Preparing rename at {:?}", position);
        
        if let Some(symbol) = self.get_symbol_at_position(content, position) {
            let range = self.get_symbol_range(content, position, &symbol)?;
            
            // Check if symbol can be renamed
            if self.is_renameable_symbol(&symbol, content) {
                Some(PrepareRenameResponse::Range(range))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Perform rename symbol refactoring
    #[instrument(skip(self, content))]
    pub async fn rename_symbol(
        &self,
        content: &str,
        position: Position,
        new_name: &str,
        uri: &Url,
    ) -> Option<WorkspaceEdit> {
        debug!("Renaming symbol at {:?} to {}", position, new_name);
        
        let symbol = self.get_symbol_at_position(content, position)?;
        
        // Find all references to the symbol
        let references = self.find_all_symbol_references(content, &symbol, uri);
        
        // Create text edits for all references
        let mut edits = HashMap::new();
        let mut text_edits = Vec::new();
        
        for reference in references {
            let range = reference.range;
            text_edits.push(TextEdit {
                range,
                new_text: new_name.to_string(),
            });
        }
        
        if !text_edits.is_empty() {
            edits.insert(uri.clone(), text_edits);
            
            Some(WorkspaceEdit {
                changes: Some(edits),
                document_changes: None,
                change_annotations: None,
            })
        } else {
            None
        }
    }

    /// Extract function refactoring
    #[instrument(skip(self, content))]
    pub async fn extract_function(
        &self,
        content: &str,
        options: ExtractFunctionOptions,
        uri: &Url,
    ) -> Option<WorkspaceEdit> {
        debug!("Extracting function: {}", options.function_name);
        
        let selected_text = self.get_text_in_range(content, &options.selection_range)?;
        
        // Analyze the selected code
        let analysis = self.analyze_code_block(&selected_text);
        
        // Generate function parameters from analysis
        let parameters = if options.extract_parameters {
            self.generate_function_parameters(&analysis)
        } else {
            Vec::new()
        };
        
        // Generate function return type
        let return_type = if options.generate_return_statement {
            self.infer_return_type(&analysis)
        } else {
            None
        };
        
        // Generate the new function
        let function_code = self.generate_function_code(
            &options.function_name,
            &parameters,
            &selected_text,
            return_type.as_deref(),
        );
        
        // Generate function call to replace selected code
        let function_call = self.generate_function_call(
            &options.function_name,
            &parameters,
        );
        
        // Create text edits
        let mut text_edits = Vec::new();
        
        // Replace selected code with function call
        text_edits.push(TextEdit {
            range: options.selection_range,
            new_text: function_call,
        });
        
        // Insert new function (find appropriate location)
        let insertion_point = self.find_function_insertion_point(content, options.selection_range.start);
        text_edits.push(TextEdit {
            range: Range {
                start: insertion_point,
                end: insertion_point,
            },
            new_text: format!("\n{}\n", function_code),
        });
        
        let mut changes = HashMap::new();
        changes.insert(uri.clone(), text_edits);
        
        Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        })
    }

    /// Extract variable refactoring
    #[instrument(skip(self, content))]
    pub async fn extract_variable(
        &self,
        content: &str,
        options: ExtractVariableOptions,
        uri: &Url,
    ) -> Option<WorkspaceEdit> {
        debug!("Extracting variable: {}", options.variable_name);
        
        let selected_text = self.get_text_in_range(content, &options.selection_range)?;
        
        // Find all occurrences if replace_all_occurrences is true
        let ranges_to_replace = if options.replace_all_occurrences {
            self.find_identical_expressions(content, &selected_text)
        } else {
            vec![options.selection_range]
        };
        
        // Generate variable declaration
        let variable_keyword = if options.is_constant { "facts" } else { "sus" };
        let variable_declaration = format!("{} {} = {}", variable_keyword, options.variable_name, selected_text.trim());
        
        // Create text edits
        let mut text_edits = Vec::new();
        
        // Insert variable declaration at appropriate location
        let insertion_point = self.find_variable_insertion_point(content, options.selection_range.start);
        text_edits.push(TextEdit {
            range: Range {
                start: insertion_point,
                end: insertion_point,
            },
            new_text: format!("{}\n", variable_declaration),
        });
        
        // Replace all occurrences with variable name
        for range in ranges_to_replace {
            text_edits.push(TextEdit {
                range,
                new_text: options.variable_name.clone(),
            });
        }
        
        let mut changes = HashMap::new();
        changes.insert(uri.clone(), text_edits);
        
        Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        })
    }

    /// Organize imports
    #[instrument(skip(self, content))]
    pub async fn organize_imports(&self, content: &str, uri: &Url) -> Option<WorkspaceEdit> {
        debug!("Organizing imports");
        
        let import_analysis = self.analyze_imports(content);
        let organized_imports = self.organize_import_statements(&import_analysis);
        
        // For now, just return None as this is a placeholder
        None
    }

    /// Generate code actions for the given range
    #[instrument(skip(self, content))]
    pub async fn get_code_actions(
        &self,
        content: &str,
        range: Range,
        context: &CodeActionContext,
        uri: &Url,
    ) -> Vec<CodeActionOrCommand> {
        debug!("Getting code actions for range {:?}", range);
        
        let mut actions = Vec::new();
        
        // Extract function action
        if self.can_extract_function(content, &range) {
            actions.push(CodeActionOrCommand::CodeAction(
                self.create_extract_function_action(content, range.clone(), "extracted_function")
            ));
        }
        
        // Extract variable action
        if self.can_extract_variable(content, &range) {
            actions.push(CodeActionOrCommand::CodeAction(
                self.create_extract_variable_action(content, range.clone(), "extracted_variable")
            ));
        }
        
        // Organize imports action
        if self.has_imports(content) {
            actions.push(CodeActionOrCommand::CodeAction(
                self.create_organize_imports_action(content)
            ));
        }
        
        // Quick fixes for diagnostics
        for diagnostic in &context.diagnostics {
            if let Some(quick_fix) = self.generate_quick_fix(diagnostic, content) {
                actions.push(CodeActionOrCommand::CodeAction(quick_fix));
            }
        }
        
        actions
    }

    // Helper methods











    fn extract_variables_from_code(&self, code: &str) -> Vec<String> {
        // Simple regex-based extraction - could be improved with AST analysis
        let mut variables = Vec::new();
        
        // This is a simplified implementation
        for word in code.split_whitespace() {
            if word.chars().all(|c| c.is_alphanumeric() || c == '_') 
                && word.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') {
                variables.push(word.to_string());
            }
        }
        
        variables.sort();
        variables.dedup();
        variables
    }

    fn extract_declared_variables_from_code(&self, code: &str) -> Vec<String> {
        let mut declared = Vec::new();
        
        for line in code.split("\n") {
            if line.contains("sus ") || line.contains("facts ") {
                // Extract variable name after the keyword
                if let Some(equals_pos) = line.find('=') {
                    let before_equals = &line[..equals_pos];
                    if let Some(colon_pos) = before_equals.find(':') {
                        let var_part = &before_equals[..colon_pos];
                        if let Some(var_name) = var_part.split_whitespace().last() {
                            declared.push(var_name.to_string());
                        }
                    } else if let Some(var_name) = before_equals.split_whitespace().last() {
                        declared.push(var_name.to_string());
                    }
                }
            }
        }
        
        declared
    }

    fn extract_function_calls_from_code(&self, code: &str) -> Vec<String> {
        let mut calls = Vec::new();
        
        // Simple pattern matching for function calls
        for line in code.split("\n") {
            for word in line.split_whitespace() {
                if word.contains('(') && !word.starts_with('(') {
                    if let Some(paren_pos) = word.find('(') {
                        let func_name = &word[..paren_pos];
                        if !func_name.is_empty() {
                            calls.push(func_name.to_string());
                        }
                    }
                }
            }
        }
        
        calls.sort();
        calls.dedup();
        calls
    }

    fn calculate_complexity(&self, code: &str) -> u32 {
        let mut complexity = 1; // Base complexity
        
        // Count control flow statements
        for line in code.split("\n") {
            if line.contains("lowkey") || line.contains("highkey") {
                complexity += 1;
            }
            if line.contains("periodt") || line.contains("bestie") || line.contains("flex") {
                complexity += 1;
            }
            if line.contains("vibe_check") {
                complexity += 1;
            }
        }
        
        complexity
    }

    fn generate_function_parameters(&self, analysis: &CodeBlockAnalysis) -> Vec<FunctionParameter> {
        let mut parameters = Vec::new();
        
        for var in &analysis.variables_used {
            // Skip variables that are declared within the block
            if !analysis.variables_declared.contains(var) {
                parameters.push(FunctionParameter {
                    name: var.clone(),
                    type_name: "any".to_string(), // Could be improved with type inference
                });
            }
        }
        
        parameters
    }

    fn infer_return_type(&self, analysis: &CodeBlockAnalysis) -> Option<String> {
        if analysis.has_return {
            Some("any".to_string()) // Could be improved with actual type inference
        } else {
            None
        }
    }

    fn generate_function_code(
        &self,
        name: &str,
        parameters: &[FunctionParameter],
        return_type: Option<&str>,
        body: &str,
    ) -> String {
        let params_str = parameters
            .iter()
            .map(|p| format!("{}: {}", p.name, p.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_annotation = return_type
            .map(|rt| format!(" -> {}", rt))
            .unwrap_or_default();
        
        format!(
            "slay {}({}){} {{\n{}\n}}",
            name, params_str, return_annotation, body
        )
    }

    fn generate_function_call(&self, name: &str, parameters: &[FunctionParameter], has_return: bool) -> String {
        let args_str = parameters
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        
        let call = format!("{}({})", name, args_str);
        
        if has_return {
            format!("facts result = {}", call)
        } else {
            call
        }
    }

    fn find_function_insertion_point(&self, content: &str) -> Option<Position> {
        let lines: Vec<&str> = content.split("\n").collect();
        
        // Find the end of the current function or at the end of file
        for (i, line) in lines.iter().enumerate().rev() {
            if line.trim().ends_with('}') && (line.contains("slay") || line.contains("yolo")) {
                return Some(Position {
                    line: i as u32 + 1,
                    character: 0,
                });
            }
        }
        
        // If no function found, insert at the end
        Some(Position {
            line: lines.len() as u32,
            character: 0,
        })
    }

    fn find_variable_insertion_point(&self, content: &str, _selection_range: &Range) -> Option<Position> {
        // Insert at the beginning of the current scope
        // For simplicity, insert at the beginning of the function or file
        Some(Position { line: 0, character: 0 })
    }

    fn find_identical_expressions(&self, content: &str, expression: &str) -> Vec<Range> {
        let mut ranges = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();

        for (line_num, line) in lines.iter().enumerate() {
            let mut search_pos = 0;
            while let Some(pos) = line[search_pos..].find(expression.trim()) {
                let actual_pos = search_pos + pos;
                
                ranges.push(Range {
                    start: Position {
                        line: line_num as u32,
                        character: actual_pos as u32,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: (actual_pos + expression.trim().len()) as u32,
                    },
                });

                search_pos = actual_pos + 1;
            }
        }

        ranges
    }

    fn analyze_imports(&self, content: &str) -> ImportAnalysis {
        let mut imports = Vec::new();
        let lines: Vec<&str> = content.split("\n").collect();
        let mut import_range: Option<Range> = None;
        let mut first_import_line = None;
        let mut last_import_line = None;

        for (line_num, line) in lines.iter().enumerate() {
            if line.trim().starts_with("use ") || line.trim().starts_with("import ") {
                if first_import_line.is_none() {
                    first_import_line = Some(line_num);
                }
                last_import_line = Some(line_num);
                
                let import_info = self.parse_import_statement(line);
                imports.push(import_info);
            }
        }

        if let (Some(first), Some(last)) = (first_import_line, last_import_line) {
            import_range = Some(Range {
                start: Position {
                    line: first as u32,
                    character: 0,
                },
                end: Position {
                    line: last as u32 + 1,
                    character: 0,
                },
            });
        }

        ImportAnalysis {
            imports,
            import_range,
        }
    }

    fn parse_import_statement(&self, line: &str) -> ImportInfo {
        // Simple parsing - could be improved
        let trimmed = line.trim();
        let is_use_statement = trimmed.starts_with("use ");
        
        let path = if is_use_statement {
            trimmed.strip_prefix("use ").unwrap_or("")
        } else {
            trimmed.strip_prefix("import ").unwrap_or("")
        };
        
        let path = path.trim_end_matches(';').trim_matches('"').trim_matches('\'');
        
        let import_type = if path.starts_with("std/") || path.starts_with("stdlib/") {
            ImportType::StandardLibrary
        } else if path.starts_with("./") || path.starts_with("../") {
            ImportType::Local
        } else {
            ImportType::External
        };

        ImportInfo {
            path: path.to_string(),
            import_type,
            is_used: true, // Simplified - would need usage analysis
        }
    }

    fn organize_import_statements(&self, analysis: &ImportAnalysis) -> Option<(Range, String)> {
        if analysis.imports.is_empty() {
            return None;
        }

        let mut organized_imports = analysis.imports.clone();
        
        // Filter out unused imports if configured
        if self.import_rules.remove_unused {
            organized_imports.retain(|import| import.is_used);
        }
        
        // Sort imports by type and then alphabetically
        organized_imports.sort_by(|a, b| {
            let type_order = match (&a.import_type, &b.import_type) {
                (ImportType::StandardLibrary, ImportType::StandardLibrary) => std::cmp::Ordering::Equal,
                (ImportType::StandardLibrary, _) => std::cmp::Ordering::Less,
                (_, ImportType::StandardLibrary) => std::cmp::Ordering::Greater,
                (ImportType::External, ImportType::External) => std::cmp::Ordering::Equal,
                (ImportType::External, ImportType::Local) => std::cmp::Ordering::Less,
                (ImportType::Local, ImportType::External) => std::cmp::Ordering::Greater,
                (ImportType::Local, ImportType::Local) => std::cmp::Ordering::Equal,
            };
            
            if type_order == std::cmp::Ordering::Equal && self.import_rules.sort_alphabetically {
                a.path.cmp(&b.path)
            } else {
                type_order
            }
        });

        // Generate organized import statements
        let mut result = String::new();
        let mut current_type = None;
        
        for import in &organized_imports {
            // Add blank line between import groups
            if current_type.is_some() && current_type != Some(&import.import_type) {
                result.push('\n');
            }
            current_type = Some(&import.import_type);
            
            result.push_str(&format!("use \"{}\";\n", import.path));
        }

        if let Some(range) = analysis.import_range {
            Some((range, result))
        } else {
            None
        }
    }

    // Code action creation methods

    fn can_extract_function(&self, content: &str, range: &Range) -> bool {
        if let Some(selected_text) = self.get_text_in_range(content, range) {
            let analysis = self.analyze_code_block(&selected_text);
            analysis.complexity_score >= 2 && selected_text.trim().lines().count() >= 3
        } else {
            false
        }
    }

    fn can_extract_variable(&self, content: &str, range: &Range) -> bool {
        if let Some(selected_text) = self.get_text_in_range(content, range) {
            // Check if selection is a single expression
            let trimmed = selected_text.trim();
            !trimmed.is_empty() && 
            !trimmed.contains('\n') && 
            !trimmed.contains("=") &&
            !trimmed.starts_with("sus ") &&
            !trimmed.starts_with("facts ")
        } else {
            false
        }
    }

    fn has_imports(&self, content: &str) -> bool {
        content.split("\n").any(|line| {
            line.trim().starts_with("use ") || line.trim().starts_with("import ")
        })
    }

    fn create_extract_function_action(&self, range: &Range) -> CodeActionOrCommand {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "Extract Function".to_string(),
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None, // Would be filled in by actual refactoring
            command: Some(Command {
                title: "Extract Function".to_string(),
                command: "cursed.refactor.extractFunction".to_string(),
                arguments: Some(vec![serde_json::to_value(range).unwrap()]),
            }),
            is_preferred: Some(true),
            disabled: None,
            data: None,
        })
    }

    fn create_extract_variable_action(&self, range: &Range) -> CodeActionOrCommand {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "Extract Variable".to_string(),
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None, // Would be filled in by actual refactoring
            command: Some(Command {
                title: "Extract Variable".to_string(),
                command: "cursed.refactor.extractVariable".to_string(),
                arguments: Some(vec![serde_json::to_value(range).unwrap()]),
            }),
            is_preferred: Some(false),
            disabled: None,
            data: None,
        })
    }

    fn create_organize_imports_action(&self) -> CodeActionOrCommand {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "Organize Imports".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            diagnostics: None,
            edit: None, // Would be filled in by actual refactoring
            command: Some(Command {
                title: "Organize Imports".to_string(),
                command: "cursed.refactor.organizeImports".to_string(),
                arguments: None,
            }),
            is_preferred: Some(false),
            disabled: None,
            data: None,
        })
    }

    fn generate_quick_fix(&self, _content: &str, diagnostic: &Diagnostic, _uri: &Url) -> Option<CodeActionOrCommand> {
        // Generate quick fixes based on diagnostic messages
        match diagnostic.message.as_str() {
            msg if msg.contains("unused variable") => {
                Some(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Remove unused variable".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: None, // Would generate actual edit
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                }))
            }
            msg if msg.contains("missing import") => {
                Some(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Add missing import".to_string(),
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: None, // Would generate actual edit
                    command: None,
                    is_preferred: Some(true),
                    disabled: None,
                    data: None,
                }))
            }
            _ => None,
        }
    }

    // Missing methods implementation

    fn get_symbol_at_position(&self, content: &str, position: Position) -> Option<SymbolInfo> {
        // Placeholder implementation - would parse content and find symbol at position
        None
    }

    fn get_symbol_range(&self, content: &str, position: Position, symbol: &SymbolInfo) -> Option<Range> {
        // Placeholder implementation
        None
    }

    fn is_renameable_symbol(&self, symbol: &SymbolInfo, content: &str) -> bool {
        // Most symbols can be renamed in CURSED
        true
    }

    fn find_all_symbol_references(&self, content: &str, symbol: &SymbolInfo, uri: &Url) -> Vec<Location> {
        // Placeholder implementation - would find all references
        Vec::new()
    }

    fn get_text_in_range(&self, content: &str, range: &Range) -> Option<String> {
        // Placeholder implementation - would extract text in range
        let lines: Vec<&str> = content.lines().collect();
        if let Some(line) = lines.get(range.start.line as usize) {
            Some(line[range.start.character as usize..].to_string())
        } else {
            None
        }
    }

    fn analyze_code_block(&self, selected_text: &str) -> CodeBlockAnalysisAdvanced {
        // Placeholder implementation
        CodeBlockAnalysisAdvanced {
            variables_used: Vec::new(),
            variables_declared: Vec::new(),
            functions_called: Vec::new(),
            has_return: false,
            complexity_score: 1,
        }
    }

    fn generate_function_parameters(&self, analysis: &CodeBlockAnalysisAdvanced) -> Vec<FunctionParameter> {
        // Generate parameters based on variables used
        analysis.variables_used.iter().map(|var| FunctionParameter {
            name: var.clone(),
            type_name: "auto".to_string(),
        }).collect()
    }

    fn infer_return_type(&self, analysis: &CodeBlockAnalysisAdvanced) -> String {
        if analysis.has_return {
            "auto".to_string()
        } else {
            "void".to_string()
        }
    }

    fn generate_function_code(
        &self,
        function_name: &str,
        parameters: &[FunctionParameter],
        return_type: &str,
        selected_text: &str,
    ) -> String {
        let param_list = parameters.iter()
            .map(|p| format!("{}: {}", p.name, p.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        
        format!(
            "slay {}({}) -> {} {{\n    {}\n}}",
            function_name, param_list, return_type, selected_text
        )
    }

    fn generate_function_call(&self, function_name: &str, parameters: &[FunctionParameter]) -> String {
        let args = parameters.iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        
        format!("{}({})", function_name, args)
    }

    fn find_function_insertion_point(&self, content: &str) -> Option<Position> {
        // Find a good place to insert the function - typically at the end
        let lines: Vec<&str> = content.lines().collect();
        Some(Position {
            line: lines.len() as u32,
            character: 0,
        })
    }

    fn find_identical_expressions(&self, content: &str, selected_text: &str) -> Vec<Range> {
        // Placeholder implementation - would find all identical expressions
        Vec::new()
    }

    fn find_variable_insertion_point(&self, content: &str, selection_range: &Range) -> Option<Position> {
        // Find a good place to insert the variable declaration
        Some(Position {
            line: selection_range.start.line.saturating_sub(1),
            character: 0,
        })
    }

    fn analyze_imports(&self, content: &str) -> ImportAnalysis {
        // Placeholder implementation
        ImportAnalysis {
            imports: Vec::new(),
            import_range: None,
        }
    }

    fn organize_import_statements(&self, analysis: &ImportAnalysis) -> Vec<TextEdit> {
        // Placeholder implementation
        Vec::new()
    }

    fn can_extract_function(&self, content: &str, range: &Range) -> bool {
        // Check if the selection is suitable for function extraction
        true
    }

    fn create_extract_function_action(&self, range: &Range) -> CodeAction {
        CodeAction {
            title: "Extract Function".to_string(),
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        }
    }

    fn can_extract_variable(&self, content: &str, range: &Range) -> bool {
        // Check if the selection is suitable for variable extraction
        true
    }

    fn create_extract_variable_action(&self, range: &Range) -> CodeAction {
        CodeAction {
            title: "Extract Variable".to_string(),
            kind: Some(CodeActionKind::REFACTOR_EXTRACT),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(true),
            disabled: None,
            data: None,
        }
    }

    fn has_imports(&self, content: &str) -> bool {
        content.contains("import ")
    }

    fn create_organize_imports_action(&self) -> CodeAction {
        CodeAction {
            title: "Organize Imports".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        }
    }

    fn generate_quick_fix(&self, content: &str, diagnostic: &Diagnostic, uri: &Url) -> Option<CodeAction> {
        // Placeholder implementation for quick fixes
        None
    }
}

// Supporting structures

#[derive(Debug, Clone)]
struct CodeBlockAnalysisAdvanced {
    variables_used: Vec<String>,
    variables_declared: Vec<String>,
    functions_called: Vec<String>,
    has_return: bool,
    complexity_score: u32,
}

#[derive(Debug, Clone)]
struct ImportAnalysis {
    imports: Vec<ImportInfo>,
    import_range: Option<Range>,
}

#[derive(Debug, Clone)]
struct ImportInfo {
    path: String,
    import_type: ImportType,
    is_used: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ImportType {
    StandardLibrary,
    External,
    Local,
}

impl Default for RefactoringProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract_function_analysis() {
        let provider = RefactoringProvider::new();
        let code = "sus x = 5\nsus y = 10\nfacts result = x + y\nprintln(result)";
        let analysis = provider.analyze_code_block(code);
        
        assert!(!analysis.variables_used.is_empty());
        assert!(!analysis.variables_declared.is_empty());
        assert!(!analysis.functions_called.is_empty());
    }

    #[tokio::test]
    async fn test_import_analysis() {
        let provider = RefactoringProvider::new();
        let content = "use \"std/fmt\"\nuse \"./local_module\"\nuse \"external_package\"";
        let analysis = provider.analyze_imports(content);
        
        assert_eq!(analysis.imports.len(), 3);
        assert!(analysis.import_range.is_some());
    }

    #[test]
    fn test_can_extract_function() {
        let provider = RefactoringProvider::new();
        let content = "sus x = 5\nsus y = 10\nfacts result = x + y\nprintln(result)";
        let range = Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 3, character: 15 },
        };
        
        assert!(provider.can_extract_function(content, &range));
    }

    #[test]
    fn test_can_extract_variable() {
        let provider = RefactoringProvider::new();
        let content = "sus result = x + y * 2";
        let range = Range {
            start: Position { line: 0, character: 13 },
            end: Position { line: 0, character: 22 },
        };
        
        assert!(provider.can_extract_variable(content, &range));
    }
}
