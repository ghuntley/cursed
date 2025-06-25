use crate::error::CursedError;
// Enhanced Auto-completion provider for CURSED language server
// 
// Provides intelligent context-aware code completion using CURSED's type system,
// AST analysis, and semantic information for keywords, variables, functions, types, etc.

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, instrument, info};

use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;
use crate::type_system::TypeChecker, Type;
use crate::ast::Program;

/// Enhanced completion provider with semantic analysis
pub struct CompletionProvider {
    /// Cache for completion items to improve performance
    /// Type checker for semantic completions
    /// AST cache for context-aware completions
impl CompletionProvider {
    /// Create a new completion provider with semantic analysis
    pub fn new() -> Self {
        Self {
        }
    }

    /// Get context-aware completions using semantic analysis
    #[instrument(skip(self, content))]
    pub async fn get_completions(&self, content: &str, position: Position) -> Vec<CompletionItem> {
        debug!("Getting context-aware completions at {:?}", position);
        
        let mut completions = Vec::new();
        
        // Parse content for semantic context
        let semantic_context = self.get_semantic_context(content, position).await;
        
        // Get traditional context as fallback
        let context = self.get_completion_context(content, position);
        
        match context.completion_type {
            CompletionType::Keyword => {
                completions.extend(self.get_keyword_completions(&context));
            }
            CompletionType::Variable => {
                completions.extend(self.get_variable_completions_with_types(content, &context, &semantic_context).await);
            }
            CompletionType::Function => {
                completions.extend(self.get_function_completions_with_signatures(content, &context, &semantic_context).await);
            }
            CompletionType::Type => {
                completions.extend(self.get_type_completions_enhanced(&context, &semantic_context).await);
            }
            CompletionType::Member => {
                completions.extend(self.get_member_completions_typed(content, &context, &semantic_context).await);
            }
            CompletionType::Import => {
                completions.extend(self.get_import_completions(&context));
            }
            CompletionType::Snippet => {
                completions.extend(self.get_snippet_completions(&context));
            }
            CompletionType::Generic => {
                // Provide all types of completions with semantic information
                completions.extend(self.get_keyword_completions(&context));
                completions.extend(self.get_variable_completions_with_types(content, &context, &semantic_context).await);
                completions.extend(self.get_function_completions_with_signatures(content, &context, &semantic_context).await);
                completions.extend(self.get_type_completions_enhanced(&context, &semantic_context).await);
            }
        }

        // Sort completions by relevance
        completions.sort_by(|a, b| {
            // Prioritize by sort_text, then by label
            match (&a.sort_text, &b.sort_text) {
            }
        });

        completions
    /// Get semantic context using AST analysis
    async fn get_semantic_context(&self, content: &str, position: Position) -> SemanticContext {
        info!("Getting semantic context for position {:?}", position);
        
        // Try to parse content and get AST
        let content_hash = format!("{:x}", md5::compute(content));
        
        // Check cache first
        if let Ok(cache) = self.ast_cache.read() {
            if let Some(ast) = cache.get(&content_hash) {
                return self.analyze_semantic_context_from_ast(ast, position);
            }
        }
        
        // Parse if not cached
        match self.parse_content_for_context(content).await {
            Ok(ast) => {
                // Cache the AST
                if let Ok(mut cache) = self.ast_cache.write() {
                    cache.insert(content_hash, ast.clone());
                }
                self.analyze_semantic_context_from_ast(&ast, position)
            }
            Err(err) => {
                debug!("Failed to parse content for semantic context: {}", err);
                SemanticContext::default()
            }
        }
    /// Parse content to get AST for context analysis
    async fn parse_content_for_context(&self, content: &str) -> crate::error::Result<()> {
        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer)?;
        parser.parse_program()
    /// Get completion context at cursor position
    fn get_completion_context(&self, content: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = content.split("\n").collect();
        let line_index = position.line as usize;
        let char_index = position.character as usize;

        if line_index >= lines.len() {
            return CompletionContext::default();
        let line = lines[line_index];
        let before_cursor = if char_index <= line.len() {
            &line[..char_index]
        } else {
            line

        // Determine completion type based on context
        let completion_type = if before_cursor.trim_end().ends_with('.') {
            CompletionType::Member
        } else if before_cursor.contains("use ") || before_cursor.contains("import ") {
            CompletionType::Import
        } else if before_cursor.contains(": ") || before_cursor.contains("-> ") {
            CompletionType::Type
        } else if before_cursor.trim_end().ends_with('(') {
            CompletionType::Function
        } else if self.is_in_keyword_context(before_cursor) {
            CompletionType::Keyword
        } else if self.is_in_variable_context(before_cursor) {
            CompletionType::Variable
        } else {
            CompletionType::Generic

        let word_start = self.find_word_start(before_cursor);
        let prefix = if word_start < before_cursor.len() {
            before_cursor[word_start..].to_string()
        } else {
            String::new()

        CompletionContext {
        }
    }

    /// Get keyword completions
    fn get_keyword_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let keywords = vec![
            // Function keywords
            
            // Variable keywords
            
            // Control flow keywords
            
            // Type keywords
            
            // Control keywords
            ("yeet", "Throw/panic statement", CompletionItemKind::KEYWORD),
            
            // Switch statement keywords
            
            // Import keywords
            
            // Visibility keywords
            
            // Async keywords
            
            // Channel keywords
            
            // Generic keywords
        ];

        keywords
            .into_iter()
            .filter(|(keyword, _, _)| {
                context.prefix.is_empty() || keyword.starts_with(&context.prefix.to_lowercase())
            })
            .map(|(keyword, description, kind)| {
                CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        description
                    sort_text: Some(format!("0_{}", keyword)), // Prioritize keywords
                    ..CompletionItem::default()
                }
            })
            .collect()
    /// Get variable completions with enhanced type information
    async fn get_variable_completions_with_types(&self, content: &str, context: &CompletionContext, semantic_context: &SemanticContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Use semantic context variables first (more accurate)
        for (var_name, var_type) in &semantic_context.variables_in_scope {
            if context.prefix.is_empty() || var_name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        var_name, var_type
                    ..CompletionItem::default()
                });
            }
        }
        
        // Fallback to text-based parsing for variables not captured semantically
        let lines: Vec<&str> = content.split("\n").collect();
        for line in lines {
            if let Some(var_name) = self.extract_variable_name(line) {
                // Skip if already added from semantic context
                if semantic_context.variables_in_scope.contains_key(&var_name) {
                    continue;
                if context.prefix.is_empty() || var_name.starts_with(&context.prefix) {
                    let var_type = self.extract_variable_type(line).unwrap_or_else(|| "unknown".to_string());
                    
                    completions.push(CompletionItem {
                        documentation: Some(Documentation::String(format!(
                            var_name, var_type
                        ..CompletionItem::default()
                    });
                }
            }
        completions
    /// Get function completions with enhanced signature information
    async fn get_function_completions_with_signatures(&self, content: &str, context: &CompletionContext, semantic_context: &SemanticContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Enhanced built-in functions with more stdlib coverage
        let builtins = vec![
            // Core functions
            
            // Type conversion functions
            
            // Control functions
            
            // Collection functions
            
            // Concurrency functions
            
            // Math functions
            
            // String functions
            
            // CursedError handling
            
            // Channel functions
        ];

        for (name, signature, description) in builtins {
            if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        description
                    ..CompletionItem::default()
                });
            }
        }
        
        // Use semantic context functions first (more accurate)
        for (func_name, signature) in &semantic_context.functions_in_scope {
            if context.prefix.is_empty() || func_name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        signature
                    ..CompletionItem::default()
                });
            }
        }
        
        // Fallback to text-based parsing for functions not captured semantically
        let lines: Vec<&str> = content.split("\n").collect();
        for line in lines {
            if let Some((func_name, params, return_type)) = self.extract_function_signature(line) {
                // Skip if already added from semantic context
                if semantic_context.functions_in_scope.contains_key(&func_name) {
                    continue;
                if context.prefix.is_empty() || func_name.starts_with(&context.prefix) {
                    let signature = format!("{}({})", func_name, params);
                    let detail = if !return_type.is_empty() {
                        format!("{} -> {}", signature, return_type)
                    } else {
                        signature.clone()
                    
                    completions.push(CompletionItem {
                        documentation: Some(Documentation::String(format!(
                            signature
                        ..CompletionItem::default()
                    });
                }
            }
        completions
    /// Get type completions
    fn get_type_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let types = vec![
            // Primitive types
            
            // Collection types
            ("map", "Map/dictionary type"),
            
            // Special types
            
            // Generic types
        ];

        types
            .into_iter()
            .filter(|(type_name, _)| {
                context.prefix.is_empty() || type_name.starts_with(&context.prefix)
            })
            .map(|(type_name, description)| {
                CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        description
                    ..CompletionItem::default()
                }
            })
            .collect()
    /// Get member completions (for dot notation)
    fn get_member_completions(&self, _content: &str, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Generic member completions
        let members = vec![
        ];

        for (name, description, kind) in members {
            if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                completions.push(CompletionItem {
                    insert_text: Some(if kind == CompletionItemKind::METHOD {
                        format!("{}($0)", name)
                    } else {
                        name.to_string()
                    ..CompletionItem::default()
                });
            }
        }

        completions
    /// Get import completions
    fn get_import_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let modules = vec![
            ("io", "Input/output operations"),
            ("http", "HTTP client/server"),
            ("json", "JSON parsing/serialization"),
        ];

        modules
            .into_iter()
            .filter(|(module, _)| {
                context.prefix.is_empty() || module.starts_with(&context.prefix)
            })
            .map(|(module, description)| {
                CompletionItem {
                    documentation: Some(Documentation::String(format!(
                        description
                    ..CompletionItem::default()
                }
            })
            .collect()
    /// Get snippet completions
    fn get_snippet_completions(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let snippets = vec![
            (
            (
            (
            (
            (
            (
                "lowkey ${1:condition} {\n    ${2:// if body}\n} highkey {\n    ${3:// else body}\n    $0\n}",
            (
            (
            (
                "vibe_check ${1:value} {\n    mood ${2:case1}:\n        ${3:// case body}\n    basic:\n        ${4:// default}\n        $0\n}",
        ];

        snippets
            .into_iter()
            .filter(|(name, _, _)| {
                context.prefix.is_empty() || name.starts_with(&context.prefix)
            })
            .map(|(name, snippet, description)| {
                CompletionItem {
                    ..CompletionItem::default()
                }
            })
            .collect()
    /// Helper methods

    fn is_in_keyword_context(&self, text: &str) -> bool {
        let text = text.trim();
        text.is_empty() || text.ends_with('{') || text.ends_with(';') || text.ends_with('\n')
    fn is_in_variable_context(&self, text: &str) -> bool {
        // Check if we're in a context where variables are expected
        text.contains('=') || text.contains('(') || text.contains(',')
    fn find_word_start(&self, text: &str) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let mut pos = chars.len();
        
        while pos > 0 {
            let ch = chars[pos - 1];
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            pos -= 1;
        pos
    fn extract_variable_name(&self, line: &str) -> Option<String> {
        if let Some(facts_pos) = line.find("facts").or_else(|| line.find("sus")) {
            let keyword_len = if line[facts_pos..].starts_with("facts") { 5 } else { 3 };
            let after_keyword = &line[facts_pos + keyword_len..];
            if let Some(equals_pos) = after_keyword.find('=') {
                let var_part = after_keyword[..equals_pos].trim();
                // Handle type annotations
                if let Some(colon_pos) = var_part.find(':') {
                    return Some(var_part[..colon_pos].trim().to_string());
                } else {
                    return Some(var_part.to_string());
                }
            }
        }
        None
    fn extract_variable_type(&self, line: &str) -> Option<String> {
        if let Some(colon_pos) = line.find(':') {
            if let Some(equals_pos) = line.find('=') {
                if colon_pos < equals_pos {
                    let type_part = line[colon_pos + 1..equals_pos].trim();
                    return Some(type_part.to_string());
                }
            }
        }
        // Try to infer type from value
        if line.contains("= \"") {
            Some("string".to_string())
        } else if line.contains("= true") || line.contains("= false") {
            Some("bool".to_string())
        } else if line.contains("= ") && line.chars().any(|c| c.is_ascii_digit()) {
            Some("int".to_string())
        } else {
            None
        }
    }

    fn extract_function_signature(&self, line: &str) -> Option<(String, String, String)> {
        if line.contains("slay") || line.contains("yolo") {
            if let Some(paren_start) = line.find('(') {
                if let Some(paren_end) = line.find(')') {
                    let before_paren = &line[..paren_start];
                    let func_name = before_paren
                        .split_whitespace()
                        .last()?
                        .to_string();
                    
                    let params = line[paren_start + 1..paren_end].to_string();
                    
                    let return_type = if let Some(arrow_pos) = line.find("->") {
                        line[arrow_pos + 2..].split('{').next()?.trim().to_string()
                    } else {
                        String::new()
                    
                    return Some((func_name, params, return_type));
                }
            }
        }
        None
    /// Get type completions with enhanced context awareness
    async fn get_type_completions_enhanced(&self, context: &CompletionContext, semantic_context: &SemanticContext) -> Vec<CompletionItem> {
        let mut completions = self.get_type_completions(context);
        
        // Add types based on expected type context
        if let Some(expected_type) = &semantic_context.expected_type {
            // Prioritize the expected type
            for completion in &mut completions {
                if completion.label == *expected_type {
                    completion.sort_text = Some(format!("0_{}", completion.label));
                    completion.documentation = Some(Documentation::String(format!(
                        expected_type
                    )));
                }
            }
        // Add generic type parameters if we're in a generic context
        if let Some(construct) = &semantic_context.containing_construct {
            if construct.contains("generic") || construct.contains("template") {
                let generic_types = vec!["T", "U", "V", "K", "E"];
                for generic_type in generic_types {
                    if context.prefix.is_empty() || generic_type.starts_with(&context.prefix) {
                        completions.push(CompletionItem {
                            documentation: Some(Documentation::String(format!(
                                generic_type
                            ..CompletionItem::default()
                        });
                    }
                }
            }
        }
        
        completions
    /// Get member completions with type-aware suggestions
    async fn get_member_completions_typed(&self, content: &str, context: &CompletionContext, semantic_context: &SemanticContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Extract the object being accessed (before the dot)
        let before_dot = context.before_cursor.trim_end_matches('.');
        if let Some(last_word_start) = before_dot.rfind(|c: char| !c.is_alphanumeric() && c != '_') {
            let object_name = &before_dot[last_word_start + 1..];
            
            // Check if we know the type of this object
            if let Some(object_type) = semantic_context.variables_in_scope.get(object_name) {
                completions.extend(self.get_type_specific_members(object_type, context));
            }
        }
        
        // Fallback to generic member completions
        if completions.is_empty() {
            completions = self.get_member_completions(content, context);
        completions
    /// Get type-specific member completions
    fn get_type_specific_members(&self, type_name: &str, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        match type_name {
            "string" | "str" => {
                let string_members = vec![
                ];
                
                for (name, description, kind) in string_members {
                    if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                        completions.push(self.create_member_completion(name, description, kind));
                    }
                }
            }
            "array" | "slice" | "Vec" => {
                let array_members = vec![
                ];
                
                for (name, description, kind) in array_members {
                    if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                        completions.push(self.create_member_completion(name, description, kind));
                    }
                }
            }
            "map" | "HashMap" => {
                let map_members = vec![
                ];
                
                for (name, description, kind) in map_members {
                    if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                        completions.push(self.create_member_completion(name, description, kind));
                    }
                }
            }
            "chan" | "channel" => {
                let channel_members = vec![
                ];
                
                for (name, description, kind) in channel_members {
                    if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                        completions.push(self.create_member_completion(name, description, kind));
                    }
                }
            }
            _ => {
                // Generic object members for unknown types
                let generic_members = vec![
                ];
                
                for (name, description, kind) in generic_members {
                    if context.prefix.is_empty() || name.starts_with(&context.prefix) {
                        completions.push(self.create_member_completion(name, description, kind));
                    }
                }
            }
        }
        
        completions
    /// Helper to create member completion items
    fn create_member_completion(&self, name: &str, description: &str, kind: CompletionItemKind) -> CompletionItem {
        CompletionItem {
            insert_text: Some(if kind == CompletionItemKind::METHOD {
                format!("{}($0)", name)
            } else {
                name.to_string()
            ..CompletionItem::default()
        }
    }
    
    /// Analyze semantic context from AST
    fn analyze_semantic_context_from_ast(&self, ast: &Program, position: Position) -> SemanticContext {
        let mut context = SemanticContext::default();
        
        // This is a simplified analysis - in a real implementation,
        // you would traverse the AST to find the scope at the given position
        // and extract variables, functions, and type information
        
        // For now, we'll provide a basic implementation that could be enhanced
        // with proper AST traversal and scope analysis
        
        // TODO: Implement proper AST traversal to:
        // 1. Find the current scope (function, struct, etc.) at position
        // 2. Extract all variables in scope with their types
        // 3. Extract all functions in scope with their signatures
        // 4. Determine expected type based on context (assignments, parameters, etc.)
        
        context.containing_construct = Some("unknown".to_string());
        
        context
    }
}

/// Completion context information
#[derive(Debug, Clone)]
struct CompletionContext {
impl Default for CompletionContext {
    fn default() -> Self {
        Self {
        }
    }
/// Type of completion to provide
#[derive(Debug, Clone, PartialEq)]
enum CompletionType {
/// Semantic context for enhanced completions
#[derive(Debug, Clone)]
struct SemanticContext {
    /// Type of construct we're currently in (function, struct, etc.)
    /// Variables available in current scope with their types
    /// Functions available in current scope with their signatures
    /// Expected type based on context (e.g., in assignment)
impl Default for SemanticContext {
    fn default() -> Self {
        Self {
        }
    }
impl Default for CompletionProvider {
    fn default() -> Self {
        Self::new()
    }
}

