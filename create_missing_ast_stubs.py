#!/usr/bin/env python3

import os

def create_missing_ast_stubs():
    """Create minimal stubs for missing AST statement types"""
    
    # Create missing statement files
    missing_statements = [
        ("EnumStatement", "enum_statement.rs"),
        ("ConstantStatement", "constant_statement.rs"), 
        ("TypeAliasStatement", "type_alias_statement.rs"),
        ("ModuleStatement", "module_statement.rs"),
    ]
    
    for statement_name, filename in missing_statements:
        content = f"""// Minimal {statement_name} for CURSED minimal build

use crate::ast::traits::{{Node, Statement}};
use crate::error::{{Error, SourceLocation}};

#[derive(Debug, Clone)]
pub struct {statement_name} {{
    pub name: String,
    pub location: SourceLocation,
}}

impl {statement_name} {{
    pub fn new(name: String) -> Self {{
        {statement_name} {{
            name,
            location: SourceLocation::default(),
        }}
    }}
}}

impl Node for {statement_name} {{
    fn source_location(&self) -> &SourceLocation {{
        &self.location
    }}
    
    fn to_string(&self) -> String {{
        format!("{statement_name}({{}})", self.name)
    }}
}}

impl Statement for {statement_name} {{
    fn clone_box(&self) -> Box<dyn Statement> {{
        Box::new(self.clone())
    }}
}}

impl std::fmt::Display for {statement_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{statement_name}({{}})", self.name)
    }}
}}
"""
        
        filepath = f"src/ast/statements/{filename}"
        os.makedirs(os.path.dirname(filepath), exist_ok=True)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Created {filepath}")

def update_ast_statements_mod():
    """Update the statements mod.rs to include missing exports"""
    
    # Read existing statements mod.rs
    mod_path = "src/ast/statements/mod.rs"
    if os.path.exists(mod_path):
        with open(mod_path, 'r') as f:
            content = f.read()
    else:
        content = "// AST statements module\n\n"
    
    # Add missing statement modules
    missing_mods = [
        "pub mod enum_statement;",
        "pub mod constant_statement;", 
        "pub mod type_alias_statement;",
        "pub mod module_statement;",
        "",
        "pub use enum_statement::EnumStatement;",
        "pub use constant_statement::ConstantStatement;",
        "pub use type_alias_statement::TypeAliasStatement;", 
        "pub use module_statement::ModuleStatement;",
    ]
    
    for mod_line in missing_mods:
        if mod_line and mod_line not in content:
            content += mod_line + "\n"
    
    os.makedirs(os.path.dirname(mod_path), exist_ok=True)
    with open(mod_path, 'w') as f:
        f.write(content)
    
    print(f"Updated {mod_path}")

def create_minimal_lexer_fix():
    """Fix lexer to have basic TokenType enum"""
    
    content = """// Minimal lexer for CURSED minimal build

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Boolean,
    
    // Identifiers and Keywords
    Identifier,
    
    // CURSED Gen Z Keywords
    Facts,  // Variable declaration
    Sus,    // Mutable variable
    Slay,   // Function declaration
    Stan,   // Goroutine spawn
    Yolo,   // Yield/continue
    
    // Control Flow
    Lowkey, // if
    Highkey, // else
    Periodt, // end/close block
    Bestie,  // for
    Flex,    // while
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Assign,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    
    // Special
    Eof,
    Newline,
    Illegal,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
            line: 1,
            column: 1,
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\\0',
            line: 1,
            column: 1,
        };
        lexer.read_char();
        lexer
    }
    
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap_or('\\0');
        }
        self.position = self.read_position;
        self.read_position += 1;
        
        if self.ch == '\\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            '+' => Token::new(TokenType::Plus, "+"),
            '-' => Token::new(TokenType::Minus, "-"),
            '*' => Token::new(TokenType::Multiply, "*"),
            '/' => Token::new(TokenType::Divide, "/"),
            '=' => Token::new(TokenType::Assign, "="),
            '(' => Token::new(TokenType::LeftParen, "("),
            ')' => Token::new(TokenType::RightParen, ")"),
            '{' => Token::new(TokenType::LeftBrace, "{"),
            '}' => Token::new(TokenType::RightBrace, "}"),
            ';' => Token::new(TokenType::Semicolon, ";"),
            ',' => Token::new(TokenType::Comma, ","),
            '\\0' => Token::new(TokenType::Eof, ""),
            _ => {
                if self.ch.is_ascii_letter() || self.ch == '_' {
                    let literal = self.read_identifier();
                    let token_type = self.lookup_ident(&literal);
                    return Token {
                        token_type,
                        literal,
                        line: self.line,
                        column: self.column,
                    };
                } else if self.ch.is_ascii_digit() {
                    let literal = self.read_number();
                    return Token::new(TokenType::Integer, &literal);
                } else {
                    Token::new(TokenType::Illegal, &self.ch.to_string())
                }
            }
        };
        
        self.read_char();
        token
    }
    
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\\t' || self.ch == '\\n' || self.ch == '\\r' {
            self.read_char();
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    
    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }
    
    fn lookup_ident(&self, ident: &str) -> TokenType {
        match ident {
            "facts" => TokenType::Facts,
            "sus" => TokenType::Sus,
            "slay" => TokenType::Slay,
            "stan" => TokenType::Stan,
            "yolo" => TokenType::Yolo,
            "lowkey" => TokenType::Lowkey,
            "highkey" => TokenType::Highkey,
            "periodt" => TokenType::Periodt,
            "bestie" => TokenType::Bestie,
            "flex" => TokenType::Flex,
            "true" | "false" => TokenType::Boolean,
            _ => TokenType::Identifier,
        }
    }
}
"""
    
    with open("src/lexer.rs", 'w') as f:
        f.write(content)
    print("Created minimal lexer.rs")

if __name__ == "__main__":
    create_missing_ast_stubs()
    update_ast_statements_mod()
    create_minimal_lexer_fix()
    print("✅ Missing AST stubs and lexer created successfully!")
