//! Completion Provider for CURSED Language Server
//! Provides intelligent code completion with context awareness

use tower_lsp::lsp_types::*;
use crate::ast::{Statement, Expression, Type};
use crate::lexer::{Token, TokenType};
use std::collections::HashMap;

/// CURSED completion provider
pub struct CursedCompletionProvider {
    /// Known symbols in the workspace
    symbols: HashMap<String, CompletionItem>,
    /// CURSED-specific keywords and their documentation
    keywords: Vec<(String, String, String)>, // (keyword, description, detail)
}

impl CursedCompletionProvider {
    pub fn new() -> Self {
        let keywords = vec![
            // Variable declarations
            ("sus".to_string(), "Variable declaration".to_string(), "sus variable_name type = value".to_string()),
            ("facts".to_string(), "Constant declaration".to_string(), "facts CONSTANT_NAME type = value".to_string()),
            
            // Function declarations
            ("slay".to_string(), "Function declaration".to_string(), "slay function_name(params) return_type { }".to_string()),
            ("damn".to_string(), "Return statement".to_string(), "damn value".to_string()),
            
            // Control flow
            ("lowkey".to_string(), "If statement".to_string(), "lowkey condition { }".to_string()),
            ("otherwise".to_string(), "Else statement".to_string(), "otherwise { }".to_string()),
            ("bestie".to_string(), "For loop".to_string(), "bestie init; condition; update { }".to_string()),
            ("ghosted".to_string(), "Break statement".to_string(), "ghosted".to_string()),
            ("simp".to_string(), "Continue statement".to_string(), "simp".to_string()),
            
            // Error handling
            ("yikes".to_string(), "Error declaration".to_string(), "yikes error_var := expression".to_string()),
            ("shook".to_string(), "Error check".to_string(), "shook error_var { }".to_string()),
            ("fam".to_string(), "Error propagation".to_string(), "fam error_var".to_string()),
            
            // Async/concurrency
            ("yolo".to_string(), "Goroutine spawn".to_string(), "yolo function_call()".to_string()),
            ("ready".to_string(), "Select statement".to_string(), "ready { case -> action }".to_string()),
            ("defer".to_string(), "Defer statement".to_string(), "defer function_call()".to_string()),
            
            // Types
            ("lit".to_string(), "Boolean type".to_string(), "Variable of boolean type".to_string()),
            ("tea".to_string(), "String type".to_string(), "Variable of string type".to_string()),
            ("drip".to_string(), "Float32 type".to_string(), "Variable of float32 type".to_string()),
            ("normie".to_string(), "Integer type".to_string(), "Variable of int32 type".to_string()),
            ("smol".to_string(), "Int8 type".to_string(), "Variable of int8 type".to_string()),
            ("mid".to_string(), "Int16 type".to_string(), "Variable of int16 type".to_string()),
            ("thicc".to_string(), "Int64 type".to_string(), "Variable of int64 type".to_string()),
            ("meal".to_string(), "Float64 type".to_string(), "Variable of float64 type".to_string()),
            ("byte".to_string(), "Byte type".to_string(), "Variable of byte type".to_string()),
            ("rune".to_string(), "Character type".to_string(), "Variable of character type".to_string()),
            ("sip".to_string(), "Single character type".to_string(), "Variable of character type".to_string()),
            
            // Literals
            ("based".to_string(), "True literal".to_string(), "Boolean true value".to_string()),
            ("cap".to_string(), "False literal".to_string(), "Boolean false value".to_string()),
            ("cringe".to_string(), "Nil literal".to_string(), "Null/nil value".to_string()),
            
            // Module system
            ("yeet".to_string(), "Import statement".to_string(), "yeet \"module_name\"".to_string()),
            ("vibes".to_string(), "Export statement".to_string(), "vibes function_name".to_string()),
            ("vibe".to_string(), "Package declaration".to_string(), "vibe package_name".to_string()),
            
            // Standard library
            ("vibez.spill".to_string(), "Print function".to_string(), "vibez.spill(value)".to_string()),
            ("vibez.slurp".to_string(), "Input function".to_string(), "vibez.slurp()".to_string()),
        ];

        Self {
            symbols: HashMap::new(),
            keywords,
        }
    }

    /// Get completion items for a given position and context
    pub fn get_completions(
        &self,
        text: &str,
        position: Position,
        trigger_character: Option<String>,
    ) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Add keywords
        for (keyword, description, detail) in &self.keywords {
            completions.push(CompletionItem {
                label: keyword.clone(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(detail.clone()),
                documentation: Some(Documentation::String(description.clone())),
                deprecated: Some(false),
                preselect: None,
                sort_text: Some(format!("0_{}", keyword)), // Prioritize keywords
                filter_text: Some(keyword.clone()),
                insert_text: Some(keyword.clone()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                insert_text_mode: None,
                text_edit: None,
                additional_text_edits: None,
                command: None,
                commit_characters: None,
                data: None,
                tags: None,
            });
        }

        // Add context-specific completions
        if let Some(trigger) = trigger_character {
            match trigger.as_str() {
                "." => {
                    // Member access completions
                    completions.extend(self.get_member_completions(text, position));
                }
                ":" => {
                    // Type annotation completions
                    completions.extend(self.get_type_completions());
                }
                _ => {}
            }
        }

        // Add symbols from workspace
        for symbol in self.symbols.values() {
            completions.push(symbol.clone());
        }

        // Add snippets for common patterns
        completions.extend(self.get_snippet_completions());

        completions
    }

    /// Get member access completions (e.g., vibez.spill, math.add)
    fn get_member_completions(&self, text: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();
        
        // Extract the object before the dot
        let line = text.lines().nth(position.line as usize).unwrap_or("");
        let before_cursor = &line[..position.character as usize.min(line.len())];
        
        if let Some(object_start) = before_cursor.rfind(|c: char| !c.is_alphanumeric() && c != '_') {
            let object = &before_cursor[object_start + 1..before_cursor.len() - 1]; // Remove the dot
            
            match object {
                "vibez" => {
                    completions.extend(vec![
                        CompletionItem {
                            label: "spill".to_string(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some("vibez.spill(value)".to_string()),
                            documentation: Some(Documentation::String("Print value to stdout".to_string())),
                            insert_text: Some("spill($0)".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                        CompletionItem {
                            label: "slurp".to_string(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some("vibez.slurp()".to_string()),
                            documentation: Some(Documentation::String("Read input from stdin".to_string())),
                            insert_text: Some("slurp()".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                    ]);
                }
                "math" => {
                    completions.extend(vec![
                        CompletionItem {
                            label: "add".to_string(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some("math.add(a, b)".to_string()),
                            documentation: Some(Documentation::String("Add two numbers".to_string())),
                            insert_text: Some("add($1, $2)".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                        CompletionItem {
                            label: "sqrt".to_string(),
                            kind: Some(CompletionItemKind::FUNCTION),
                            detail: Some("math.sqrt(x)".to_string()),
                            documentation: Some(Documentation::String("Square root of x".to_string())),
                            insert_text: Some("sqrt($1)".to_string()),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..Default::default()
                        },
                    ]);
                }
                _ => {}
            }
        }
        
        completions
    }

    /// Get type completions for type annotations
    fn get_type_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "lit".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Boolean type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "tea".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("String type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "normie".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Integer type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "drip".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Float type".to_string()),
                ..Default::default()
            },
        ]
    }

    /// Get snippet completions for common patterns
    fn get_snippet_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "function".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Function declaration".to_string()),
                documentation: Some(Documentation::String("Create a new function".to_string())),
                insert_text: Some("slay ${1:function_name}(${2:params}) ${3:return_type} {\n    ${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("If statement".to_string()),
                documentation: Some(Documentation::String("Conditional statement".to_string())),
                insert_text: Some("lowkey ${1:condition} {\n    ${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "for".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("For loop".to_string()),
                documentation: Some(Documentation::String("For loop statement".to_string())),
                insert_text: Some("bestie ${1:i} := ${2:0}; ${1:i} < ${3:10}; ${1:i}++ {\n    ${0}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "var".to_string(),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some("Variable declaration".to_string()),
                documentation: Some(Documentation::String("Declare a variable".to_string())),
                insert_text: Some("sus ${1:name} ${2:type} = ${0:value}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ]
    }

    /// Add a symbol to the completion provider
    pub fn add_symbol(&mut self, name: String, item: CompletionItem) {
        self.symbols.insert(name, item);
    }

    /// Remove a symbol from the completion provider
    pub fn remove_symbol(&mut self, name: &str) {
        self.symbols.remove(name);
    }

    /// Clear all symbols
    pub fn clear_symbols(&mut self) {
        self.symbols.clear();
    }
}
