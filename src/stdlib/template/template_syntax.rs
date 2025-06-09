/// Template Syntax Parser - Lexer, Parser, and AST for CURSED templates
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, error, instrument, warn};

use crate::error::Error as CursedError;
use crate::object::Object as CursedObject;
use super::template_core::TemplateDelimiters;

/// Template AST representing a parsed template
#[derive(Debug, Clone)]
pub struct TemplateAst {
    pub nodes: Vec<TemplateNode>,
}

/// Individual nodes in the template AST
#[derive(Debug, Clone)]
pub enum TemplateNode {
    /// Plain text content
    Text(String),
    /// Variable interpolation: {{ variable }}
    Variable {
        name: String,
        filters: Vec<FilterCall>,
    },
    /// Block statement: {% if condition %}
    Block(BlockNode),
    /// Comment: {# comment #}
    Comment(String),
    /// Template inclusion: {% include "template" %}
    Include {
        template_name: String,
        context: Option<HashMap<String, TemplateExpression>>,
    },
    /// Layout definition: {% layout "base" %}
    Layout {
        name: String,
        blocks: HashMap<String, Vec<TemplateNode>>,
    },
    /// Block definition: {% block "content" %}
    BlockDef {
        name: String,
        content: Vec<TemplateNode>,
    },
}

/// Block statement types
#[derive(Debug, Clone)]
pub enum BlockNode {
    /// Conditional: {% if condition %}
    If {
        condition: TemplateExpression,
        then_branch: Vec<TemplateNode>,
        else_branch: Option<Vec<TemplateNode>>,
    },
    /// Loop: {% for item in items %}
    For {
        variable: String,
        iterator: TemplateExpression,
        body: Vec<TemplateNode>,
        else_body: Option<Vec<TemplateNode>>,
    },
    /// Enhanced conditional: {% when condition %}
    When {
        condition: TemplateExpression,
        body: Vec<TemplateNode>,
    },
    /// Enhanced iteration: {% each items %}
    Each {
        iterator: TemplateExpression,
        body: Vec<TemplateNode>,
    },
    /// Range loop: {% loop count %}
    Loop {
        count: TemplateExpression,
        body: Vec<TemplateNode>,
    },
    /// Range with params: {% for i=0 to=10 step=2 %}
    RangeFor {
        variable: String,
        start: TemplateExpression,
        end: TemplateExpression,
        step: Option<TemplateExpression>,
        body: Vec<TemplateNode>,
    },
}

/// Template expressions for conditions and values
#[derive(Debug, Clone)]
pub enum TemplateExpression {
    /// Variable reference: .Name or .User.Email
    Variable(String),
    /// String literal: "hello"
    String(String),
    /// Number literal: 42 or 3.14
    Number(f64),
    /// Boolean literal: true or false
    Boolean(bool),
    /// Function call: add 5 10
    FunctionCall {
        name: String,
        args: Vec<TemplateExpression>,
    },
    /// Property access: .User.Name
    PropertyAccess {
        object: Box<TemplateExpression>,
        property: String,
    },
    /// Binary operation: a + b
    BinaryOp {
        left: Box<TemplateExpression>,
        operator: BinaryOperator,
        right: Box<TemplateExpression>,
    },
    /// Unary operation: not a
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<TemplateExpression>,
    },
}

/// Binary operators
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

/// Unary operators
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not, Minus,
}

/// Filter call in templates
#[derive(Debug, Clone)]
pub struct FilterCall {
    pub name: String,
    pub args: Vec<TemplateExpression>,
}

/// Template tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateToken {
    Text(String),
    VariableStart,      // {{
    VariableEnd,        // }}
    BlockStart,         // {%
    BlockEnd,           // %}
    CommentStart,       // {#
    CommentEnd,         // #}
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),
    Pipe,               // |
    Dot,                // .
    Comma,              // ,
    Colon,              // :
    Equal,              // =
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    Percent,            // %
    EqualEqual,         // ==
    NotEqual,           // !=
    LessThan,           // <
    LessEqual,          // <=
    GreaterThan,        // >
    GreaterEqual,       // >=
    And,                // and
    Or,                 // or
    Not,                // not
    If,                 // if
    Else,               // else
    ElseIf,             // elif
    End,                // end
    For,                // for
    In,                 // in
    When,               // when
    Each,               // each
    Loop,               // loop
    To,                 // to
    Step,               // step
    Include,            // include
    Layout,             // layout
    Block,              // block
    EOF,
}

impl fmt::Display for TemplateToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateToken::Text(s) => write!(f, "Text({})", s),
            TemplateToken::Identifier(s) => write!(f, "Identifier({})", s),
            TemplateToken::String(s) => write!(f, "String(\"{}\")", s),
            TemplateToken::Number(n) => write!(f, "Number({})", n),
            TemplateToken::Boolean(b) => write!(f, "Boolean({})", b),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Template lexer
pub struct TemplateLexer<'a> {
    input: &'a str,
    position: usize,
    delimiters: &'a TemplateDelimiters,
    mode: LexerMode,
}

#[derive(Debug, Clone, PartialEq)]
enum LexerMode {
    Text,
    Variable,
    Block,
    Comment,
}

impl<'a> TemplateLexer<'a> {
    pub fn new(input: &'a str, delimiters: &'a TemplateDelimiters) -> Self {
        Self {
            input,
            position: 0,
            delimiters,
            mode: LexerMode::Text,
        }
    }

    #[instrument(skip(self))]
    pub fn tokenize(&mut self) -> Result<Vec<TemplateToken>, CursedError> {
        debug!(input_length = self.input.len(), "Starting template tokenization");
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            match self.mode {
                LexerMode::Text => self.tokenize_text(&mut tokens)?,
                LexerMode::Variable => self.tokenize_variable(&mut tokens)?,
                LexerMode::Block => self.tokenize_block(&mut tokens)?,
                LexerMode::Comment => self.tokenize_comment(&mut tokens)?,
            }
        }

        tokens.push(TemplateToken::EOF);
        debug!(token_count = tokens.len(), "Template tokenization completed");
        Ok(tokens)
    }

    fn tokenize_text(&mut self, tokens: &mut Vec<TemplateToken>) -> Result<(), CursedError> {
        let start = self.position;

        while !self.is_at_end() {
            if self.check_delimiter(&self.delimiters.variable.0) {
                break;
            } else if self.check_delimiter(&self.delimiters.block.0) {
                break;
            } else if self.check_delimiter(&self.delimiters.comment.0) {
                break;
            }
            self.advance();
        }

        if self.position > start {
            let text = self.input[start..self.position].to_string();
            tokens.push(TemplateToken::Text(text));
        }

        // Check for delimiter transitions
        if self.check_delimiter(&self.delimiters.variable.0) {
            self.advance_by(self.delimiters.variable.0.len());
            tokens.push(TemplateToken::VariableStart);
            self.mode = LexerMode::Variable;
        } else if self.check_delimiter(&self.delimiters.block.0) {
            self.advance_by(self.delimiters.block.0.len());
            tokens.push(TemplateToken::BlockStart);
            self.mode = LexerMode::Block;
        } else if self.check_delimiter(&self.delimiters.comment.0) {
            self.advance_by(self.delimiters.comment.0.len());
            tokens.push(TemplateToken::CommentStart);
            self.mode = LexerMode::Comment;
        }

        Ok(())
    }

    fn tokenize_variable(&mut self, tokens: &mut Vec<TemplateToken>) -> Result<(), CursedError> {
        self.skip_whitespace();

        if self.check_delimiter(&self.delimiters.variable.1) {
            self.advance_by(self.delimiters.variable.1.len());
            tokens.push(TemplateToken::VariableEnd);
            self.mode = LexerMode::Text;
            return Ok(());
        }

        match self.current_char() {
            '|' => {
                self.advance();
                tokens.push(TemplateToken::Pipe);
            }
            '.' => {
                self.advance();
                tokens.push(TemplateToken::Dot);
            }
            ',' => {
                self.advance();
                tokens.push(TemplateToken::Comma);
            }
            '"' | '\'' => {
                let string_val = self.read_string()?;
                tokens.push(TemplateToken::String(string_val));
            }
            _ if self.current_char().is_ascii_digit() => {
                let number = self.read_number()?;
                tokens.push(TemplateToken::Number(number));
            }
            _ if self.current_char().is_ascii_alphabetic() || self.current_char() == '_' => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "true" => tokens.push(TemplateToken::Boolean(true)),
                    "false" => tokens.push(TemplateToken::Boolean(false)),
                    _ => tokens.push(TemplateToken::Identifier(identifier)),
                }
            }
            _ => {
                self.advance();
            }
        }

        Ok(())
    }

    fn tokenize_block(&mut self, tokens: &mut Vec<TemplateToken>) -> Result<(), CursedError> {
        self.skip_whitespace();

        if self.check_delimiter(&self.delimiters.block.1) {
            self.advance_by(self.delimiters.block.1.len());
            tokens.push(TemplateToken::BlockEnd);
            self.mode = LexerMode::Text;
            return Ok(());
        }

        match self.current_char() {
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    tokens.push(TemplateToken::EqualEqual);
                } else {
                    tokens.push(TemplateToken::Equal);
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    tokens.push(TemplateToken::NotEqual);
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    tokens.push(TemplateToken::LessEqual);
                } else {
                    tokens.push(TemplateToken::LessThan);
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    tokens.push(TemplateToken::GreaterEqual);
                } else {
                    tokens.push(TemplateToken::GreaterThan);
                }
            }
            '+' => {
                self.advance();
                tokens.push(TemplateToken::Plus);
            }
            '-' => {
                self.advance();
                tokens.push(TemplateToken::Minus);
            }
            '*' => {
                self.advance();
                tokens.push(TemplateToken::Star);
            }
            '/' => {
                self.advance();
                tokens.push(TemplateToken::Slash);
            }
            '%' => {
                self.advance();
                tokens.push(TemplateToken::Percent);
            }
            '"' | '\'' => {
                let string_val = self.read_string()?;
                tokens.push(TemplateToken::String(string_val));
            }
            _ if self.current_char().is_ascii_digit() => {
                let number = self.read_number()?;
                tokens.push(TemplateToken::Number(number));
            }
            _ if self.current_char().is_ascii_alphabetic() || self.current_char() == '_' => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "if" => tokens.push(TemplateToken::If),
                    "else" => tokens.push(TemplateToken::Else),
                    "elif" => tokens.push(TemplateToken::ElseIf),
                    "end" => tokens.push(TemplateToken::End),
                    "for" => tokens.push(TemplateToken::For),
                    "in" => tokens.push(TemplateToken::In),
                    "when" => tokens.push(TemplateToken::When),
                    "each" => tokens.push(TemplateToken::Each),
                    "loop" => tokens.push(TemplateToken::Loop),
                    "to" => tokens.push(TemplateToken::To),
                    "step" => tokens.push(TemplateToken::Step),
                    "include" => tokens.push(TemplateToken::Include),
                    "layout" => tokens.push(TemplateToken::Layout),
                    "block" => tokens.push(TemplateToken::Block),
                    "and" => tokens.push(TemplateToken::And),
                    "or" => tokens.push(TemplateToken::Or),
                    "not" => tokens.push(TemplateToken::Not),
                    "true" => tokens.push(TemplateToken::Boolean(true)),
                    "false" => tokens.push(TemplateToken::Boolean(false)),
                    _ => tokens.push(TemplateToken::Identifier(identifier)),
                }
            }
            _ => {
                self.advance();
            }
        }

        Ok(())
    }

    fn tokenize_comment(&mut self, tokens: &mut Vec<TemplateToken>) -> Result<(), CursedError> {
        let start = self.position;

        while !self.is_at_end() && !self.check_delimiter(&self.delimiters.comment.1) {
            self.advance();
        }

        if self.position > start {
            let comment = self.input[start..self.position].to_string();
            tokens.push(TemplateToken::Text(comment)); // Comments are treated as text internally
        }

        if self.check_delimiter(&self.delimiters.comment.1) {
            self.advance_by(self.delimiters.comment.1.len());
            tokens.push(TemplateToken::CommentEnd);
            self.mode = LexerMode::Text;
        }

        Ok(())
    }

    fn read_string(&mut self) -> Result<String, CursedError> {
        let quote_char = self.current_char();
        self.advance(); // Skip opening quote

        let mut value = String::new();
        while !self.is_at_end() && self.current_char() != quote_char {
            if self.current_char() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    match self.current_char() {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '\\' => value.push('\\'),
                        '"' => value.push('"'),
                        '\'' => value.push('\''),
                        _ => {
                            value.push('\\');
                            value.push(self.current_char());
                        }
                    }
                }
            } else {
                value.push(self.current_char());
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(CursedError::TemplateError {
                message: "Unterminated string literal".to_string(),
                source_location: None,
            });
        }

        self.advance(); // Skip closing quote
        Ok(value)
    }

    fn read_number(&mut self) -> Result<f64, CursedError> {
        let start = self.position;
        while !self.is_at_end() && (self.current_char().is_ascii_digit() || self.current_char() == '.') {
            self.advance();
        }

        let number_str = &self.input[start..self.position];
        number_str.parse::<f64>()
            .map_err(|_| CursedError::TemplateError {
                message: format!("Invalid number: {}", number_str),
                source_location: None,
            })
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while !self.is_at_end() && 
              (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        self.input[start..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn check_delimiter(&self, delimiter: &str) -> bool {
        if self.position + delimiter.len() > self.input.len() {
            return false;
        }
        &self.input[self.position..self.position + delimiter.len()] == delimiter
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
        }
    }

    fn advance_by(&mut self, count: usize) {
        for _ in 0..count {
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

/// Template parser
pub struct TemplateParser {
    tokens: Vec<TemplateToken>,
    current: usize,
}

impl TemplateParser {
    pub fn new(tokens: Vec<TemplateToken>) -> Self {
        Self { tokens, current: 0 }
    }

    #[instrument(skip(self))]
    pub fn parse(&mut self) -> Result<TemplateAst, CursedError> {
        debug!(token_count = self.tokens.len(), "Starting template parsing");
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            nodes.push(self.parse_node()?);
        }

        debug!(node_count = nodes.len(), "Template parsing completed");
        Ok(TemplateAst { nodes })
    }

    fn parse_node(&mut self) -> Result<TemplateNode, CursedError> {
        match &self.tokens[self.current] {
            TemplateToken::Text(text) => {
                let text = text.clone();
                self.advance();
                Ok(TemplateNode::Text(text))
            }
            TemplateToken::VariableStart => {
                self.advance(); // Skip {{
                let node = self.parse_variable()?;
                self.expect(&TemplateToken::VariableEnd)?;
                Ok(node)
            }
            TemplateToken::BlockStart => {
                self.advance(); // Skip {%
                let node = self.parse_block()?;
                self.expect(&TemplateToken::BlockEnd)?;
                Ok(node)
            }
            TemplateToken::CommentStart => {
                self.advance(); // Skip {#
                // Skip until comment end
                while !self.is_at_end() && !matches!(self.tokens[self.current], TemplateToken::CommentEnd) {
                    self.advance();
                }
                if matches!(self.tokens[self.current], TemplateToken::CommentEnd) {
                    self.advance(); // Skip #}
                }
                // Comments are ignored in the AST
                self.parse_node()
            }
            _ => {
                self.advance();
                self.parse_node()
            }
        }
    }

    fn parse_variable(&mut self) -> Result<TemplateNode, CursedError> {
        let name = match &self.tokens[self.current] {
            TemplateToken::Identifier(name) => name.clone(),
            _ => return Err(CursedError::TemplateError {
                message: "Expected variable name".to_string(),
                source_location: None,
            }),
        };
        self.advance();

        let mut filters = Vec::new();
        while matches!(self.tokens[self.current], TemplateToken::Pipe) {
            self.advance(); // Skip |
            filters.push(self.parse_filter()?);
        }

        Ok(TemplateNode::Variable { name, filters })
    }

    fn parse_filter(&mut self) -> Result<FilterCall, CursedError> {
        let name = match &self.tokens[self.current] {
            TemplateToken::Identifier(name) => name.clone(),
            _ => return Err(CursedError::TemplateError {
                message: "Expected filter name".to_string(),
                source_location: None,
            }),
        };
        self.advance();

        let mut args = Vec::new();
        // Parse filter arguments (simplified for now)
        while !matches!(self.tokens[self.current], TemplateToken::Pipe | TemplateToken::VariableEnd) {
            args.push(self.parse_expression()?);
        }

        Ok(FilterCall { name, args })
    }

    fn parse_block(&mut self) -> Result<TemplateNode, CursedError> {
        match &self.tokens[self.current] {
            TemplateToken::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.advance(); // Skip block end
                
                let then_branch = self.parse_block_body()?;
                let else_branch = if matches!(self.tokens[self.current], TemplateToken::BlockStart) {
                    self.advance(); // Skip {%
                    if matches!(self.tokens[self.current], TemplateToken::Else) {
                        self.advance(); // Skip else
                        self.advance(); // Skip %}
                        Some(self.parse_block_body()?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                Ok(TemplateNode::Block(BlockNode::If {
                    condition,
                    then_branch,
                    else_branch,
                }))
            }
            TemplateToken::For => {
                self.advance();
                let variable = match &self.tokens[self.current] {
                    TemplateToken::Identifier(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected variable name in for loop".to_string(),
                        source_location: None,
                    }),
                };
                self.advance();
                self.expect(&TemplateToken::In)?;
                let iterator = self.parse_expression()?;
                self.advance(); // Skip block end

                let body = self.parse_block_body()?;

                Ok(TemplateNode::Block(BlockNode::For {
                    variable,
                    iterator,
                    body,
                    else_body: None,
                }))
            }
            TemplateToken::Include => {
                self.advance();
                let template_name = match &self.tokens[self.current] {
                    TemplateToken::String(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected template name for include".to_string(),
                        source_location: None,
                    }),
                };
                self.advance();

                Ok(TemplateNode::Include {
                    template_name,
                    context: None,
                })
            }
            _ => Err(CursedError::TemplateError {
                message: format!("Unexpected block token: {:?}", self.tokens[self.current]),
                source_location: None,
            }),
        }
    }

    fn parse_block_body(&mut self) -> Result<Vec<TemplateNode>, CursedError> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            // Check for end of block
            if matches!(self.tokens[self.current], TemplateToken::BlockStart) {
                let next_pos = self.current + 1;
                if next_pos < self.tokens.len() && 
                   matches!(self.tokens[next_pos], TemplateToken::End | TemplateToken::Else) {
                    break;
                }
            }
            body.push(self.parse_node()?);
        }

        // Skip the end block
        if matches!(self.tokens[self.current], TemplateToken::BlockStart) {
            self.advance(); // Skip {%
            self.advance(); // Skip end/else
            self.advance(); // Skip %}
        }

        Ok(body)
    }

    fn parse_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        match &self.tokens[self.current] {
            TemplateToken::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(TemplateExpression::Variable(name))
            }
            TemplateToken::String(value) => {
                let value = value.clone();
                self.advance();
                Ok(TemplateExpression::String(value))
            }
            TemplateToken::Number(value) => {
                let value = *value;
                self.advance();
                Ok(TemplateExpression::Number(value))
            }
            TemplateToken::Boolean(value) => {
                let value = *value;
                self.advance();
                Ok(TemplateExpression::Boolean(value))
            }
            _ => Err(CursedError::TemplateError {
                message: format!("Unexpected expression token: {:?}", self.tokens[self.current]),
                source_location: None,
            }),
        }
    }

    fn expect(&mut self, expected: &TemplateToken) -> Result<(), CursedError> {
        if std::mem::discriminant(&self.tokens[self.current]) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(CursedError::TemplateError {
                message: format!("Expected {:?}, found {:?}", expected, self.tokens[self.current]),
                source_location: None,
            })
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.tokens[self.current], TemplateToken::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_delimiters() -> TemplateDelimiters {
        TemplateDelimiters {
            variable: ("{{".to_string(), "}}".to_string()),
            block: ("{%".to_string(), "%}".to_string()),
            comment: ("{#".to_string(), "#}".to_string()),
        }
    }

    #[test]
    fn test_lexer_simple_text() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("Hello World", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 2); // Text + EOF
        assert!(matches!(tokens[0], TemplateToken::Text(_)));
        assert!(matches!(tokens[1], TemplateToken::EOF));
    }

    #[test]
    fn test_lexer_variable() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("{{ name }}", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0], TemplateToken::VariableStart));
        assert!(matches!(tokens[1], TemplateToken::Identifier(_)));
        assert!(matches!(tokens[2], TemplateToken::VariableEnd));
    }

    #[test]
    fn test_lexer_block() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("{% if condition %}", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0], TemplateToken::BlockStart));
        assert!(matches!(tokens[1], TemplateToken::If));
        assert!(matches!(tokens[2], TemplateToken::Identifier(_)));
        assert!(matches!(tokens[3], TemplateToken::BlockEnd));
    }

    #[test]
    fn test_parser_simple() {
        let tokens = vec![
            TemplateToken::Text("Hello ".to_string()),
            TemplateToken::VariableStart,
            TemplateToken::Identifier("name".to_string()),
            TemplateToken::VariableEnd,
            TemplateToken::EOF,
        ];
        
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.nodes.len(), 2);
        assert!(matches!(ast.nodes[0], TemplateNode::Text(_)));
        assert!(matches!(ast.nodes[1], TemplateNode::Variable { .. }));
    }
}
