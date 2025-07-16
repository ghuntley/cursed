//! Semantic Highlighting Provider for CURSED Language Server
//! Provides context-aware syntax highlighting

use tower_lsp::lsp_types::*;
use crate::ast::{Statement, Expression, Type, Program};
use crate::lexer::{Token, TokenType};
use std::collections::HashMap;

/// CURSED semantic highlighting provider
pub struct CursedSemanticHighlighter {
    /// Token type mappings
    token_types: Vec<SemanticTokenType>,
    /// Token modifier mappings
    token_modifiers: Vec<SemanticTokenModifier>,
}

impl CursedSemanticHighlighter {
    pub fn new() -> Self {
        let token_types = vec![
            SemanticTokenType::KEYWORD,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::TYPE,
            SemanticTokenType::COMMENT,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::PROPERTY,
            // CURSED-specific token types
            SemanticTokenType::new("cursed_keyword"),
            SemanticTokenType::new("cursed_literal"),
            SemanticTokenType::new("cursed_builtin"),
        ];

        let token_modifiers = vec![
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::DEPRECATED,
            // CURSED-specific modifiers
            SemanticTokenModifier::new("cursed_slang"),
            SemanticTokenModifier::new("cursed_vibe"),
        ];

        Self {
            token_types,
            token_modifiers,
        }
    }

    /// Get semantic tokens legend
    pub fn get_legend(&self) -> SemanticTokensLegend {
        SemanticTokensLegend {
            token_types: self.token_types.clone(),
            token_modifiers: self.token_modifiers.clone(),
        }
    }

    /// Generate semantic tokens for a document
    pub fn get_semantic_tokens(&self, text: &str, ast: Option<&Program>) -> SemanticTokens {
        let mut tokens = Vec::new();
        
        if let Some(program) = ast {
            self.highlight_ast(program, &mut tokens);
        } else {
            // Fallback to lexical highlighting
            self.highlight_lexical(text, &mut tokens);
        }

        SemanticTokens {
            result_id: None,
            data: tokens,
        }
    }

    /// Generate semantic tokens from AST
    fn highlight_ast(&self, program: &Program, tokens: &mut Vec<SemanticToken>) {
        for statement in &program.statements {
            self.highlight_statement(statement, tokens);
        }
    }

    /// Highlight a statement
    fn highlight_statement(&self, statement: &Statement, tokens: &mut Vec<SemanticToken>) {
        match statement {
            Statement::FunctionDeclaration(func) => {
                // Highlight 'slay' keyword
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 4, // "slay"
                    token_type: self.get_token_type_index(&SemanticTokenType::KEYWORD),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_slang")]),
                });

                // Highlight function name
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 5, // after "slay "
                    length: func.name.len() as u32,
                    token_type: self.get_token_type_index(&SemanticTokenType::FUNCTION),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::DEFINITION]),
                });

                // Highlight parameters
                for param in &func.parameters {
                    tokens.push(SemanticToken {
                        delta_line: 0,
                        delta_start: 0, // Will be calculated based on position
                        length: param.name.len() as u32,
                        token_type: self.get_token_type_index(&SemanticTokenType::PARAMETER),
                        token_modifiers_bitset: 0,
                    });
                }

                // Highlight body
                for stmt in &func.body {
                    self.highlight_statement(stmt, tokens);
                }
            }
            Statement::VariableDeclaration(var) => {
                // Highlight 'sus' keyword
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 3, // "sus"
                    token_type: self.get_token_type_index(&SemanticTokenType::KEYWORD),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_slang")]),
                });

                // Highlight variable name
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 4, // after "sus "
                    length: var.name.len() as u32,
                    token_type: self.get_token_type_index(&SemanticTokenType::VARIABLE),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::DEFINITION]),
                });

                // Highlight type
                if let Some(type_annotation) = &var.type_annotation {
                    self.highlight_type(type_annotation, tokens);
                }

                // Highlight initializer
                if let Some(initializer) = &var.initializer {
                    self.highlight_expression(initializer, tokens);
                }
            }
            Statement::ExpressionStatement(expr) => {
                self.highlight_expression(expr, tokens);
            }
            Statement::ReturnStatement(ret) => {
                // Highlight 'damn' keyword
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 4, // "damn"
                    token_type: self.get_token_type_index(&SemanticTokenType::KEYWORD),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_slang")]),
                });

                if let Some(value) = &ret.value {
                    self.highlight_expression(value, tokens);
                }
            }
            Statement::IfStatement(if_stmt) => {
                // Highlight 'lowkey' keyword
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 6, // "lowkey"
                    token_type: self.get_token_type_index(&SemanticTokenType::KEYWORD),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_slang")]),
                });

                self.highlight_expression(&if_stmt.condition, tokens);
                
                for stmt in &if_stmt.then_branch {
                    self.highlight_statement(stmt, tokens);
                }

                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.highlight_statement(stmt, tokens);
                    }
                }
            }
            _ => {
                // Handle other statement types
            }
        }
    }

    /// Highlight an expression
    fn highlight_expression(&self, expression: &Expression, tokens: &mut Vec<SemanticToken>) {
        match expression {
            Expression::StringLiteral(s) => {
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: s.len() as u32 + 2, // Include quotes
                    token_type: self.get_token_type_index(&SemanticTokenType::STRING),
                    token_modifiers_bitset: 0,
                });
            }
            Expression::IntegerLiteral(_) => {
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: 5, // Estimate
                    token_type: self.get_token_type_index(&SemanticTokenType::NUMBER),
                    token_modifiers_bitset: 0,
                });
            }
            Expression::BooleanLiteral(value) => {
                let keyword = if *value { "based" } else { "cap" };
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: keyword.len() as u32,
                    token_type: self.get_token_type_index(&SemanticTokenType::new("cursed_literal")),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_vibe")]),
                });
            }
            Expression::Identifier(name) => {
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: name.len() as u32,
                    token_type: self.get_token_type_index(&SemanticTokenType::VARIABLE),
                    token_modifiers_bitset: 0,
                });
            }
            Expression::FunctionCall(call) => {
                // Highlight function name
                self.highlight_expression(&call.function, tokens);
                
                // Highlight arguments
                for arg in &call.arguments {
                    self.highlight_expression(arg, tokens);
                }
            }
            Expression::MemberAccess(member) => {
                self.highlight_expression(&member.object, tokens);
                
                // Highlight member name
                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length: member.member.len() as u32,
                    token_type: self.get_token_type_index(&SemanticTokenType::PROPERTY),
                    token_modifiers_bitset: 0,
                });
            }
            _ => {
                // Handle other expression types
            }
        }
    }

    /// Highlight a type annotation
    fn highlight_type(&self, type_annotation: &Type, tokens: &mut Vec<SemanticToken>) {
        match type_annotation {
            Type::Primitive(name) => {
                let length = match name.as_str() {
                    "lit" => 3,
                    "tea" => 3,
                    "normie" => 6,
                    "drip" => 4,
                    "thicc" => 5,
                    _ => name.len() as u32,
                };

                tokens.push(SemanticToken {
                    delta_line: 0,
                    delta_start: 0,
                    length,
                    token_type: self.get_token_type_index(&SemanticTokenType::TYPE),
                    token_modifiers_bitset: self.get_modifier_bitset(&[SemanticTokenModifier::new("cursed_slang")]),
                });
            }
            _ => {
                // Handle other type forms
            }
        }
    }

    /// Fallback lexical highlighting
    fn highlight_lexical(&self, text: &str, tokens: &mut Vec<SemanticToken>) {
        let lines: Vec<&str> = text.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let mut char_pos = 0;
            let words: Vec<&str> = line.split_whitespace().collect();
            
            for word in words {
                let token_type = self.classify_word(word);
                let length = word.len() as u32;
                
                tokens.push(SemanticToken {
                    delta_line: if tokens.is_empty() { line_num as u32 } else { 0 },
                    delta_start: char_pos,
                    length,
                    token_type: self.get_token_type_index(&token_type),
                    token_modifiers_bitset: self.get_word_modifiers(word),
                });
                
                char_pos += length + 1; // +1 for space
            }
        }
    }

    /// Classify a word for highlighting
    fn classify_word(&self, word: &str) -> SemanticTokenType {
        match word {
            // CURSED keywords
            "sus" | "slay" | "damn" | "lowkey" | "otherwise" | "bestie" | "ghosted" | "simp" |
            "yikes" | "shook" | "fam" | "yolo" | "ready" | "defer" | "yeet" | "vibes" | "vibe" => {
                SemanticTokenType::KEYWORD
            }
            // CURSED types
            "lit" | "tea" | "normie" | "drip" | "thicc" | "smol" | "mid" | "meal" | "byte" | "rune" | "sip" => {
                SemanticTokenType::TYPE
            }
            // CURSED literals
            "based" | "cap" | "cringe" => {
                SemanticTokenType::new("cursed_literal")
            }
            // CURSED builtins
            "vibez" => {
                SemanticTokenType::new("cursed_builtin")
            }
            _ => {
                if word.starts_with('"') && word.ends_with('"') {
                    SemanticTokenType::STRING
                } else if word.parse::<i32>().is_ok() || word.parse::<f64>().is_ok() {
                    SemanticTokenType::NUMBER
                } else if word.starts_with("//") {
                    SemanticTokenType::COMMENT
                } else {
                    SemanticTokenType::VARIABLE
                }
            }
        }
    }

    /// Get modifiers for a word
    fn get_word_modifiers(&self, word: &str) -> u32 {
        let mut modifiers = 0;
        
        if matches!(word, "sus" | "slay" | "damn" | "lowkey" | "bestie" | "yolo" | "based" | "cap") {
            modifiers |= 1 << self.get_modifier_index(&SemanticTokenModifier::new("cursed_slang"));
        }
        
        if matches!(word, "vibez" | "based" | "cap" | "cringe") {
            modifiers |= 1 << self.get_modifier_index(&SemanticTokenModifier::new("cursed_vibe"));
        }
        
        modifiers
    }

    /// Get token type index
    fn get_token_type_index(&self, token_type: &SemanticTokenType) -> u32 {
        self.token_types.iter()
            .position(|t| t == token_type)
            .unwrap_or(0) as u32
    }

    /// Get modifier index
    fn get_modifier_index(&self, modifier: &SemanticTokenModifier) -> usize {
        self.token_modifiers.iter()
            .position(|m| m == modifier)
            .unwrap_or(0)
    }

    /// Get modifier bitset
    fn get_modifier_bitset(&self, modifiers: &[SemanticTokenModifier]) -> u32 {
        let mut bitset = 0;
        for modifier in modifiers {
            let index = self.get_modifier_index(modifier);
            bitset |= 1 << index;
        }
        bitset
    }
}
