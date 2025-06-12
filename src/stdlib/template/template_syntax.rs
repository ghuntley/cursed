/// Template Syntax Parser - Lexer, Parser, and AST for CURSED templates
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, error, instrument, warn};

use crate::error::{Error as CursedError, SourceLocation as ErrorSourceLocation};
use crate::object::Object as CursedObject;
use super::template_core::TemplateDelimiters;

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, position: usize) -> Self {
        Self { line, column, position }
    }
}

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
        expression: TemplateExpression,
        filters: Vec<FilterCall>,
        location: Option<SourceLocation>,
    },
    /// Block statement: {% if condition %}
    Block {
        block: BlockNode,
        location: Option<SourceLocation>,
    },
    /// Comment: {# comment #}
    Comment {
        content: String,
        location: Option<SourceLocation>,
    },
    /// Template inclusion: {% include "template" %}
    Include {
        template_name: String,
        context: Option<HashMap<String, TemplateExpression>>,
        location: Option<SourceLocation>,
    },
    /// Layout definition: {% extends "base" %}
    Extends {
        name: String,
        location: Option<SourceLocation>,
    },
    /// Block definition: {% block "content" %}
    BlockDef {
        name: String,
        content: Vec<TemplateNode>,
        location: Option<SourceLocation>,
    },
    /// Raw content: {% raw %}...{% endraw %}
    Raw {
        content: String,
        location: Option<SourceLocation>,
    },
    /// Set variable: {% set name = value %}
    Set {
        name: String,
        value: TemplateExpression,
        location: Option<SourceLocation>,
    },
    /// CURSED-style conditional: {% lowkey condition %}
    LowkeyIf {
        condition: TemplateExpression,
        then_branch: Vec<TemplateNode>,
        else_branch: Option<Vec<TemplateNode>>,
        location: Option<SourceLocation>,
    },
    /// CURSED-style loop: {% stan items %}
    StanLoop {
        variable: String,
        iterator: TemplateExpression,
        body: Vec<TemplateNode>,
        location: Option<SourceLocation>,
    },
}

/// Block statement types
#[derive(Debug, Clone)]
pub enum BlockNode {
    /// Conditional: {% if condition %}
    If {
        condition: TemplateExpression,
        then_branch: Vec<TemplateNode>,
        elsif_branches: Vec<(TemplateExpression, Vec<TemplateNode>)>,
        else_branch: Option<Vec<TemplateNode>>,
    },
    /// Loop: {% for item in items %}
    For {
        variable: String,
        iterator: TemplateExpression,
        body: Vec<TemplateNode>,
        else_body: Option<Vec<TemplateNode>>,
    },
    /// While loop: {% while condition %}
    While {
        condition: TemplateExpression,
        body: Vec<TemplateNode>,
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
    /// Match statement: {% match value %}
    Match {
        value: TemplateExpression,
        cases: Vec<MatchCase>,
        default_case: Option<Vec<TemplateNode>>,
    },
    /// With statement: {% with context %}
    With {
        context: HashMap<String, TemplateExpression>,
        body: Vec<TemplateNode>,
    },
}

/// Match case for pattern matching
#[derive(Debug, Clone)]
pub struct MatchCase {
    pub pattern: TemplateExpression,
    pub body: Vec<TemplateNode>,
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
    /// Null literal
    Null,
    /// Array literal: [1, 2, 3]
    Array(Vec<TemplateExpression>),
    /// Object literal: {key: value}
    Object(HashMap<String, TemplateExpression>),
    /// Function call: add(5, 10)
    FunctionCall {
        name: String,
        args: Vec<TemplateExpression>,
    },
    /// Method call: object.method(args)
    MethodCall {
        object: Box<TemplateExpression>,
        method: String,
        args: Vec<TemplateExpression>,
    },
    /// Property access: .User.Name
    PropertyAccess {
        object: Box<TemplateExpression>,
        property: String,
    },
    /// Index access: array[0]
    IndexAccess {
        object: Box<TemplateExpression>,
        index: Box<TemplateExpression>,
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
    /// Conditional expression: condition ? then : else
    Conditional {
        condition: Box<TemplateExpression>,
        then_expr: Box<TemplateExpression>,
        else_expr: Box<TemplateExpression>,
    },
    /// CURSED-style expressions
    /// Truthiness check: sus value
    Sus(Box<TemplateExpression>),
    /// Falsy check: cap value
    Cap(Box<TemplateExpression>),
    /// Type check: facts value
    Facts(Box<TemplateExpression>),
}

/// Binary operators
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    // CURSED-style operators
    Vibe,      // "vibe" - loose equality
    NoVibe,    // "no_vibe" - loose inequality
    Slay,      // "slay" - contains/in
    NoSlay,    // "no_slay" - not contains
}

/// Unary operators
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not, Minus, Plus,
    // CURSED-style operators
    Sus,       // truthiness check
    Cap,       // falsy check
    Facts,     // type check
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
    Null,               // null
    Pipe,               // |
    Dot,                // .
    Comma,              // ,
    Colon,              // :
    Semicolon,          // ;
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
    LeftParen,          // (
    RightParen,         // )
    LeftBracket,        // [
    RightBracket,       // ]
    LeftBrace,          // {
    RightBrace,         // }
    Question,           // ?
    And,                // and
    Or,                 // or
    Not,                // not
    If,                 // if
    Else,               // else
    ElseIf,             // elif
    End,                // end
    For,                // for
    In,                 // in
    While,              // while
    When,               // when
    Each,               // each
    Loop,               // loop
    To,                 // to
    Step,               // step
    Include,            // include
    Extends,            // extends
    Block,              // block
    Raw,                // raw
    EndRaw,             // endraw
    Set,                // set
    Match,              // match
    Case,               // case
    Default,            // default
    With,               // with
    // CURSED-style keywords
    Lowkey,             // lowkey (if)
    Highkey,            // highkey (else)
    Stan,               // stan (for/loop)
    Sus,                // sus (truthiness)
    Cap,                // cap (falsy)
    Facts,              // facts (type check)
    Vibe,               // vibe (loose equality)
    NoVibe,             // no_vibe
    Slay,               // slay (contains)
    NoSlay,             // no_slay
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
    line: usize,
    column: usize,
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
            line: 1,
            column: 1,
            delimiters,
            mode: LexerMode::Text,
        }
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column, self.position)
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
            ':' => {
                self.advance();
                tokens.push(TemplateToken::Colon);
            }
            ';' => {
                self.advance();
                tokens.push(TemplateToken::Semicolon);
            }
            '(' => {
                self.advance();
                tokens.push(TemplateToken::LeftParen);
            }
            ')' => {
                self.advance();
                tokens.push(TemplateToken::RightParen);
            }
            '[' => {
                self.advance();
                tokens.push(TemplateToken::LeftBracket);
            }
            ']' => {
                self.advance();
                tokens.push(TemplateToken::RightBracket);
            }
            '{' => {
                self.advance();
                tokens.push(TemplateToken::LeftBrace);
            }
            '}' => {
                self.advance();
                tokens.push(TemplateToken::RightBrace);
            }
            '?' => {
                self.advance();
                tokens.push(TemplateToken::Question);
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
                    "null" => tokens.push(TemplateToken::Null),
                    // CURSED-style keywords
                    "sus" => tokens.push(TemplateToken::Sus),
                    "cap" => tokens.push(TemplateToken::Cap),
                    "facts" => tokens.push(TemplateToken::Facts),
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
                    "while" => tokens.push(TemplateToken::While),
                    "when" => tokens.push(TemplateToken::When),
                    "each" => tokens.push(TemplateToken::Each),
                    "loop" => tokens.push(TemplateToken::Loop),
                    "to" => tokens.push(TemplateToken::To),
                    "step" => tokens.push(TemplateToken::Step),
                    "include" => tokens.push(TemplateToken::Include),
                    "extends" => tokens.push(TemplateToken::Extends),
                    "block" => tokens.push(TemplateToken::Block),
                    "raw" => tokens.push(TemplateToken::Raw),
                    "endraw" => tokens.push(TemplateToken::EndRaw),
                    "set" => tokens.push(TemplateToken::Set),
                    "match" => tokens.push(TemplateToken::Match),
                    "case" => tokens.push(TemplateToken::Case),
                    "default" => tokens.push(TemplateToken::Default),
                    "with" => tokens.push(TemplateToken::With),
                    "and" => tokens.push(TemplateToken::And),
                    "or" => tokens.push(TemplateToken::Or),
                    "not" => tokens.push(TemplateToken::Not),
                    "true" => tokens.push(TemplateToken::Boolean(true)),
                    "false" => tokens.push(TemplateToken::Boolean(false)),
                    "null" => tokens.push(TemplateToken::Null),
                    // CURSED-style keywords
                    "lowkey" => tokens.push(TemplateToken::Lowkey),
                    "highkey" => tokens.push(TemplateToken::Highkey),
                    "stan" => tokens.push(TemplateToken::Stan),
                    "sus" => tokens.push(TemplateToken::Sus),
                    "cap" => tokens.push(TemplateToken::Cap),
                    "facts" => tokens.push(TemplateToken::Facts),
                    "vibe" => tokens.push(TemplateToken::Vibe),
                    "no_vibe" => tokens.push(TemplateToken::NoVibe),
                    "slay" => tokens.push(TemplateToken::Slay),
                    "no_slay" => tokens.push(TemplateToken::NoSlay),
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
            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
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
        let location = self.current_location();
        let expression = self.parse_expression()?;

        let mut filters = Vec::new();
        while matches!(self.tokens[self.current], TemplateToken::Pipe) {
            self.advance(); // Skip |
            filters.push(self.parse_filter()?);
        }

        Ok(TemplateNode::Variable { 
            expression, 
            filters, 
            location: Some(location) 
        })
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(1, 1, self.current) // Simplified for now
    }

    fn current_error_location(&self) -> ErrorSourceLocation {
        ErrorSourceLocation::new(1, 1) // Convert to error module's SourceLocation
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
        let location = self.current_location();
        
        match &self.tokens[self.current] {
            TemplateToken::If => {
                self.advance();
                let condition = self.parse_expression()?;
                self.advance(); // Skip block end
                
                let then_branch = self.parse_block_body()?;
                let mut elsif_branches = Vec::new();
                let mut else_branch = None;

                // Handle elsif and else branches
                while matches!(self.tokens[self.current], TemplateToken::BlockStart) {
                    let next_pos = self.current + 1;
                    if next_pos < self.tokens.len() {
                        match &self.tokens[next_pos] {
                            TemplateToken::ElseIf => {
                                self.advance(); // Skip {%
                                self.advance(); // Skip elif
                                let elsif_condition = self.parse_expression()?;
                                self.advance(); // Skip %}
                                let elsif_body = self.parse_block_body()?;
                                elsif_branches.push((elsif_condition, elsif_body));
                            },
                            TemplateToken::Else => {
                                self.advance(); // Skip {%
                                self.advance(); // Skip else
                                self.advance(); // Skip %}
                                else_branch = Some(self.parse_block_body()?);
                                break;
                            },
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }

                Ok(TemplateNode::Block {
                    block: BlockNode::If {
                        condition,
                        then_branch,
                        elsif_branches,
                        else_branch,
                    },
                    location: Some(location),
                })
            }
            TemplateToken::For => {
                self.advance();
                let variable = match &self.tokens[self.current] {
                    TemplateToken::Identifier(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected variable name in for loop".to_string(),
                        source_location: Some(self.current_error_location()),
                    }),
                };
                self.advance();
                self.expect(&TemplateToken::In)?;
                let iterator = self.parse_expression()?;
                self.advance(); // Skip block end

                let body = self.parse_block_body()?;

                Ok(TemplateNode::Block {
                    block: BlockNode::For {
                        variable,
                        iterator,
                        body,
                        else_body: None,
                    },
                    location: Some(location),
                })
            }
            TemplateToken::Lowkey => {
                // CURSED-style conditional: {% lowkey condition %}
                self.advance();
                let condition = self.parse_expression()?;
                self.advance(); // Skip block end
                
                let then_branch = self.parse_block_body()?;
                let else_branch = if matches!(self.tokens[self.current], TemplateToken::BlockStart) {
                    let next_pos = self.current + 1;
                    if next_pos < self.tokens.len() && 
                       matches!(self.tokens[next_pos], TemplateToken::Highkey) {
                        self.advance(); // Skip {%
                        self.advance(); // Skip highkey
                        self.advance(); // Skip %}
                        Some(self.parse_block_body()?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                Ok(TemplateNode::LowkeyIf {
                    condition,
                    then_branch,
                    else_branch,
                    location: Some(location),
                })
            }
            TemplateToken::Stan => {
                // CURSED-style loop: {% stan item in items %}
                self.advance();
                let variable = match &self.tokens[self.current] {
                    TemplateToken::Identifier(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected variable name in stan loop".to_string(),
                        source_location: Some(self.current_error_location()),
                    }),
                };
                self.advance();
                self.expect(&TemplateToken::In)?;
                let iterator = self.parse_expression()?;
                self.advance(); // Skip block end

                let body = self.parse_block_body()?;

                Ok(TemplateNode::StanLoop {
                    variable,
                    iterator,
                    body,
                    location: Some(location),
                })
            }
            TemplateToken::Include => {
                self.advance();
                let template_name = match &self.tokens[self.current] {
                    TemplateToken::String(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected template name for include".to_string(),
                        source_location: Some(self.current_error_location()),
                    }),
                };
                self.advance();

                Ok(TemplateNode::Include {
                    template_name,
                    context: None,
                    location: Some(location),
                })
            }
            TemplateToken::Set => {
                self.advance();
                let name = match &self.tokens[self.current] {
                    TemplateToken::Identifier(name) => name.clone(),
                    _ => return Err(CursedError::TemplateError {
                        message: "Expected variable name for set".to_string(),
                        source_location: Some(self.current_error_location()),
                    }),
                };
                self.advance();
                self.expect(&TemplateToken::Equal)?;
                let value = self.parse_expression()?;

                Ok(TemplateNode::Set {
                    name,
                    value,
                    location: Some(location),
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
        self.parse_conditional_expression()
    }

    fn parse_conditional_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut expr = self.parse_logical_or_expression()?;

        if matches!(self.tokens[self.current], TemplateToken::Question) {
            self.advance(); // Skip ?
            let then_expr = Box::new(self.parse_expression()?);
            self.expect(&TemplateToken::Colon)?;
            let else_expr = Box::new(self.parse_expression()?);
            expr = TemplateExpression::Conditional {
                condition: Box::new(expr),
                then_expr,
                else_expr,
            };
        }

        Ok(expr)
    }

    fn parse_logical_or_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_logical_and_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::Or) {
            self.advance();
            let right = self.parse_logical_and_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_equality_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::And) {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_relational_expression()?;

        while matches!(self.tokens[self.current], 
                      TemplateToken::EqualEqual | TemplateToken::NotEqual | 
                      TemplateToken::Vibe | TemplateToken::NoVibe) {
            let operator = match self.tokens[self.current] {
                TemplateToken::EqualEqual => BinaryOperator::Eq,
                TemplateToken::NotEqual => BinaryOperator::Ne,
                TemplateToken::Vibe => BinaryOperator::Vibe,
                TemplateToken::NoVibe => BinaryOperator::NoVibe,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_relational_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_relational_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_additive_expression()?;

        while matches!(self.tokens[self.current], 
                      TemplateToken::LessThan | TemplateToken::LessEqual |
                      TemplateToken::GreaterThan | TemplateToken::GreaterEqual |
                      TemplateToken::Slay | TemplateToken::NoSlay) {
            let operator = match self.tokens[self.current] {
                TemplateToken::LessThan => BinaryOperator::Lt,
                TemplateToken::LessEqual => BinaryOperator::Le,
                TemplateToken::GreaterThan => BinaryOperator::Gt,
                TemplateToken::GreaterEqual => BinaryOperator::Ge,
                TemplateToken::Slay => BinaryOperator::Slay,
                TemplateToken::NoSlay => BinaryOperator::NoSlay,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_additive_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_multiplicative_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::Plus | TemplateToken::Minus) {
            let operator = match self.tokens[self.current] {
                TemplateToken::Plus => BinaryOperator::Add,
                TemplateToken::Minus => BinaryOperator::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplicative_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut left = self.parse_unary_expression()?;

        while matches!(self.tokens[self.current], 
                      TemplateToken::Star | TemplateToken::Slash | TemplateToken::Percent) {
            let operator = match self.tokens[self.current] {
                TemplateToken::Star => BinaryOperator::Mul,
                TemplateToken::Slash => BinaryOperator::Div,
                TemplateToken::Percent => BinaryOperator::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary_expression()?;
            left = TemplateExpression::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        match &self.tokens[self.current] {
            TemplateToken::Not => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::UnaryOp {
                    operator: UnaryOperator::Not,
                    operand,
                })
            }
            TemplateToken::Minus => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::UnaryOp {
                    operator: UnaryOperator::Minus,
                    operand,
                })
            }
            TemplateToken::Sus => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::Sus(operand))
            }
            TemplateToken::Cap => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::Cap(operand))
            }
            TemplateToken::Facts => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::Facts(operand))
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Result<TemplateExpression, CursedError> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            match &self.tokens[self.current] {
                TemplateToken::Dot => {
                    self.advance(); // Skip .
                    let property = match &self.tokens[self.current] {
                        TemplateToken::Identifier(name) => name.clone(),
                        _ => return Err(CursedError::TemplateError {
                            message: "Expected property name after '.'".to_string(),
                            source_location: Some(self.current_error_location()),
                        }),
                    };
                    self.advance();
                    expr = TemplateExpression::PropertyAccess {
                        object: Box::new(expr),
                        property,
                    };
                }
                TemplateToken::LeftBracket => {
                    self.advance(); // Skip [
                    let index = Box::new(self.parse_expression()?);
                    self.expect(&TemplateToken::RightBracket)?;
                    expr = TemplateExpression::IndexAccess {
                        object: Box::new(expr),
                        index,
                    };
                }
                TemplateToken::LeftParen => {
                    // Function call
                    self.advance(); // Skip (
                    let mut args = Vec::new();
                    while !matches!(self.tokens[self.current], TemplateToken::RightParen) {
                        args.push(self.parse_expression()?);
                        if matches!(self.tokens[self.current], TemplateToken::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(&TemplateToken::RightParen)?;
                    
                    if let TemplateExpression::Variable(name) = expr {
                        expr = TemplateExpression::FunctionCall { name, args };
                    } else {
                        return Err(CursedError::TemplateError {
                            message: "Invalid function call".to_string(),
                            source_location: Some(self.current_error_location()),
                        });
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary_expression(&mut self) -> Result<TemplateExpression, CursedError> {
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
            TemplateToken::Null => {
                self.advance();
                Ok(TemplateExpression::Null)
            }
            TemplateToken::LeftParen => {
                self.advance(); // Skip (
                let expr = self.parse_expression()?;
                self.expect(&TemplateToken::RightParen)?;
                Ok(expr)
            }
            TemplateToken::LeftBracket => {
                // Array literal
                self.advance(); // Skip [
                let mut elements = Vec::new();
                while !matches!(self.tokens[self.current], TemplateToken::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if matches!(self.tokens[self.current], TemplateToken::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TemplateToken::RightBracket)?;
                Ok(TemplateExpression::Array(elements))
            }
            TemplateToken::LeftBrace => {
                // Object literal
                self.advance(); // Skip {
                let mut object = HashMap::new();
                while !matches!(self.tokens[self.current], TemplateToken::RightBrace) {
                    let key = match &self.tokens[self.current] {
                        TemplateToken::Identifier(key) | TemplateToken::String(key) => key.clone(),
                        _ => return Err(CursedError::TemplateError {
                            message: "Expected object key".to_string(),
                            source_location: Some(self.current_error_location()),
                        }),
                    };
                    self.advance();
                    self.expect(&TemplateToken::Colon)?;
                    let value = self.parse_expression()?;
                    object.insert(key, value);
                    
                    if matches!(self.tokens[self.current], TemplateToken::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TemplateToken::RightBrace)?;
                Ok(TemplateExpression::Object(object))
            }
            _ => Err(CursedError::TemplateError {
                message: format!("Unexpected expression token: {:?}", self.tokens[self.current]),
                source_location: Some(self.current_error_location()),
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

    #[test]
    fn test_cursed_style_conditional() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("{% lowkey user.is_active %}Active{% highkey %}Inactive{% end %}", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.nodes.len(), 1);
        if let TemplateNode::LowkeyIf { condition, then_branch, else_branch, .. } = &ast.nodes[0] {
            assert!(matches!(condition, TemplateExpression::PropertyAccess { .. }));
            assert_eq!(then_branch.len(), 1);
            assert!(else_branch.is_some());
        } else {
            panic!("Expected LowkeyIf node");
        }
    }

    #[test]
    fn test_cursed_style_loop() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("{% stan item in items %}{{ item }}{% end %}", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = TemplateParser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.nodes.len(), 1);
        if let TemplateNode::StanLoop { variable, iterator, body, .. } = &ast.nodes[0] {
            assert_eq!(variable, "item");
            assert!(matches!(iterator, TemplateExpression::Variable(_)));
            assert_eq!(body.len(), 1);
        } else {
            panic!("Expected StanLoop node");
        }
    }

    #[test]
    fn test_expression_parsing() {
        let tokens = vec![
            TemplateToken::Identifier("user".to_string()),
            TemplateToken::Dot,
            TemplateToken::Identifier("name".to_string()),
            TemplateToken::EOF,
        ];
        
        let mut parser = TemplateParser::new(tokens);
        let expr = parser.parse_expression().unwrap();
        
        if let TemplateExpression::PropertyAccess { object, property } = expr {
            assert!(matches!(*object.as_ref(), TemplateExpression::Variable(_)));
            assert_eq!(property, "name");
        } else {
            panic!("Expected PropertyAccess expression");
        }
    }

    #[test]
    fn test_cursed_operators() {
        let delimiters = create_test_delimiters();
        let mut lexer = TemplateLexer::new("{{ sus value vibe other }}", &delimiters);
        let tokens = lexer.tokenize().unwrap();
        
        // Check that CURSED keywords are tokenized correctly
        assert!(tokens.contains(&TemplateToken::Sus));
        assert!(tokens.contains(&TemplateToken::Vibe));
    }
}
