
/// Template Syntax Parser - Lexer, Parser, and AST for CURSED templates
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, error, instrument, warn};
use crate::error::CursedError;

use crate::object::Object as CursedObject;
use super::template_core::TemplateDelimiters;

/// Source location for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
impl SourceLocation {
    pub fn new(line: usize, column: usize, position: usize) -> Self {
        Self { line, column, position }
    }
/// Template AST representing a parsed template
#[derive(Debug, Clone)]
pub struct TemplateAst {
/// Individual nodes in the template AST
#[derive(Debug, Clone)]
pub enum TemplateNode {
    /// Plain text content
    /// Variable interpolation: {{ variable }}
    Variable {
    /// Block statement: {% if condition %}
    Block {
    /// Comment: {# comment #}
    Comment {
    /// Template inclusion: {% include "template" %}
    Include {
    /// Layout definition: {% extends "base" %}
    Extends {
    /// Block definition: {% block "content" %}
    BlockDef {
    /// Raw content: {% raw %}...{% endraw %}
    Raw {
    /// Set variable: {% set name = value %}
    Set {
    /// CURSED-style conditional: {% lowkey condition %}
    LowkeyIf {
    /// CURSED-style loop: {% stan items %}
    StanLoop {
/// Block statement types
#[derive(Debug, Clone)]
pub enum BlockNode {
    /// Conditional: {% if condition %}
    If {
    /// Loop: {% for item in items %}
    For {
    /// While loop: {% while condition %}
    While {
    /// Enhanced conditional: {% when condition %}
    When {
    /// Enhanced iteration: {% each items %}
    Each {
    /// Range loop: {% loop count %}
    Loop {
    /// Range with params: {% for i=0 to=10 step=2 %}
    RangeFor {
    /// Match statement: {% match value %}
    Match {
    /// With statement: {% with context %}
    With {
/// Match case for pattern matching
#[derive(Debug, Clone)]
pub struct MatchCase {
/// Template expressions for conditions and values
#[derive(Debug, Clone)]
pub enum TemplateExpression {
    /// Variable reference: .Name or .User.Email
    /// String literal: "hello"
    /// Number literal: 42 or 3.14
    /// Boolean literal: true or false
    /// Null literal
    /// Array literal: [1, 2, 3]
    /// Object literal: {key: value}
    /// Function call: add(5, 10)
    FunctionCall {
    /// Method call: object.method(args)
    MethodCall {
    /// Property access: .User.Name
    PropertyAccess {
    /// Index access: array[0]
    IndexAccess {
    /// Binary operation: a + b
    BinaryOp {
    /// Unary operation: not a
    UnaryOp {
    /// Conditional expression: condition ? then : else
    Conditional {
    /// CURSED-style expressions
    /// Truthiness check: sus value
    /// Falsy check: cap value
    /// Type check: facts value
/// Binary operators
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    // CURSED-style operators
    Vibe,      // "vibe" - loose equality
    NoVibe,    // "no_vibe" - loose inequality
    Slay,      // "slay" - contains/in
    NoSlay,    // "no_slay" - not contains
/// Unary operators
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    // CURSED-style operators
    Sus,       // truthiness check
    Cap,       // falsy check
    Facts,     // type check
/// Filter call in templates
#[derive(Debug, Clone)]
pub struct FilterCall {
/// Template tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateToken {
    VariableStart,      // {{
    VariableEnd,        // }}
    BlockStart,         // {%
    BlockEnd,           // %}
    CommentStart,       // {#
    CommentEnd,         // #}
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
impl fmt::Display for TemplateToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
/// Template lexer
pub struct TemplateLexer<'a> {
#[derive(Debug, Clone, PartialEq)]
enum LexerMode {
impl<'a> TemplateLexer<'a> {
    pub fn new(input: &'a str, delimiters: &'a TemplateDelimiters) -> Self {
        Self {
        }
    }

    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column, self.position)
    #[instrument(skip(self))]
    pub fn tokenize(&mut self) -> crate::error::Result<()> {
        debug!(input_length = self.input.len(), "Starting template tokenization");
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            match self.mode {
            }
        }

        tokens.push(TemplateToken::EOF);
        debug!(token_count = tokens.len(), "Template tokenization completed");
        Ok(tokens)
    fn tokenize_text(&mut self, tokens: &mut Vec<TemplateToken>) -> crate::error::Result<()> {
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
        if self.position > start {
            let text = self.input[start..self.position].to_string();
            tokens.push(TemplateToken::Text(text));
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
        Ok(())
    fn tokenize_variable(&mut self, tokens: &mut Vec<TemplateToken>) -> crate::error::Result<()> {
        self.skip_whitespace();

        if self.check_delimiter(&self.delimiters.variable.1) {
            self.advance_by(self.delimiters.variable.1.len());
            tokens.push(TemplateToken::VariableEnd);
            self.mode = LexerMode::Text;
            return Ok(());
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
                    // CURSED-style keywords
                }
            }
            _ => {
                self.advance();
            }
        }

        Ok(())
    fn tokenize_block(&mut self, tokens: &mut Vec<TemplateToken>) -> crate::error::Result<()> {
        self.skip_whitespace();

        if self.check_delimiter(&self.delimiters.block.1) {
            self.advance_by(self.delimiters.block.1.len());
            tokens.push(TemplateToken::BlockEnd);
            self.mode = LexerMode::Text;
            return Ok(());
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
                    // CURSED-style keywords
                }
            }
            _ => {
                self.advance();
            }
        }

        Ok(())
    fn tokenize_comment(&mut self, tokens: &mut Vec<TemplateToken>) -> crate::error::Result<()> {
        let start = self.position;

        while !self.is_at_end() && !self.check_delimiter(&self.delimiters.comment.1) {
            self.advance();
        if self.position > start {
            let comment = self.input[start..self.position].to_string();
            tokens.push(TemplateToken::Text(comment)); // Comments are treated as text internally
        if self.check_delimiter(&self.delimiters.comment.1) {
            self.advance_by(self.delimiters.comment.1.len());
            tokens.push(TemplateToken::CommentEnd);
            self.mode = LexerMode::Text;
        Ok(())
    fn read_string(&mut self) -> crate::error::Result<()> {
        let quote_char = self.current_char();
        self.advance(); // Skip opening quote

        let mut value = String::new();
        while !self.is_at_end() && self.current_char() != quote_char {
            if self.current_char() == '\\' {
                self.advance();
                if !self.is_at_end() {
                    match self.current_char() {
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
        if self.is_at_end() {
            return Err(CursedError::TemplateCursedError {
            });
        self.advance(); // Skip closing quote
        Ok(value)
    fn read_number(&mut self) -> crate::error::Result<()> {
        let start = self.position;
        while !self.is_at_end() && (self.current_char().is_ascii_digit() || self.current_char() == '.') {
            self.advance();
        let number_str = &self.input[start..self.position];
        number_str.parse::<f64>()
            .map_err(|_| CursedError::TemplateCursedError {
            })
    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while !self.is_at_end() && 
              (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        self.input[start..self.position].to_string()
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
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
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
impl TemplateParser {
    pub fn new(tokens: Vec<TemplateToken>) -> Self {
        Self { tokens, current: 0 }
    }

    #[instrument(skip(self))]
    pub fn parse(&mut self) -> crate::error::Result<()> {
        debug!(token_count = self.tokens.len(), "Starting template parsing");
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            nodes.push(self.parse_node()?);
        debug!(node_count = nodes.len(), "Template parsing completed");
        Ok(TemplateAst { nodes })
    fn parse_node(&mut self) -> crate::error::Result<()> {
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
    fn parse_variable(&mut self) -> crate::error::Result<()> {
        let location = self.current_location();
        let expression = self.parse_expression()?;

        let mut filters = Vec::new();
        while matches!(self.tokens[self.current], TemplateToken::Pipe) {
            self.advance(); // Skip |
            filters.push(self.parse_filter()?);
        Ok(TemplateNode::Variable { 
            location: Some(location) 
        })
    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(1, 1, self.current) // Simplified for now
    fn current_error_location(&self) -> ErrorSourceLocation {
        ErrorSourceLocation::new(1, 1) // Convert to error module's SourceLocation
    fn parse_filter(&mut self) -> crate::error::Result<()> {
        let name = match &self.tokens[self.current] {
            _ => return Err(CursedError::TemplateCursedError {
        self.advance();

        let mut args = Vec::new();
        // Parse filter arguments (simplified for now)
        while !matches!(self.tokens[self.current], TemplateToken::Pipe | TemplateToken::VariableEnd) {
            args.push(self.parse_expression()?);
        Ok(FilterCall { name, args })
    fn parse_block(&mut self) -> crate::error::Result<()> {
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
                            TemplateToken::Else => {
                                self.advance(); // Skip {%
                                self.advance(); // Skip else
                                self.advance(); // Skip %}
                                else_branch = Some(self.parse_block_body()?);
                                break;
                        }
                    } else {
                        break;
                    }
                }

                Ok(TemplateNode::Block {
                    block: BlockNode::If {
                })
            }
            TemplateToken::For => {
                self.advance();
                let variable = match &self.tokens[self.current] {
                    _ => return Err(CursedError::TemplateCursedError {
                self.advance();
                self.expect(&TemplateToken::In)?;
                let iterator = self.parse_expression()?;
                self.advance(); // Skip block end

                let body = self.parse_block_body()?;

                Ok(TemplateNode::Block {
                    block: BlockNode::For {
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

                Ok(TemplateNode::LowkeyIf {
                })
            }
            TemplateToken::Stan => {
                // CURSED-style loop: {% stan item in items %}
                self.advance();
                let variable = match &self.tokens[self.current] {
                    _ => return Err(CursedError::TemplateCursedError {
                self.advance();
                self.expect(&TemplateToken::In)?;
                let iterator = self.parse_expression()?;
                self.advance(); // Skip block end

                let body = self.parse_block_body()?;

                Ok(TemplateNode::StanLoop {
                })
            }
            TemplateToken::Include => {
                self.advance();
                let template_name = match &self.tokens[self.current] {
                    _ => return Err(CursedError::TemplateCursedError {
                self.advance();

                Ok(TemplateNode::Include {
                })
            }
            TemplateToken::Set => {
                self.advance();
                let name = match &self.tokens[self.current] {
                    _ => return Err(CursedError::TemplateCursedError {
                self.advance();
                self.expect(&TemplateToken::Equal)?;
                let value = self.parse_expression()?;

                Ok(TemplateNode::Set {
                })
            }
            _ => Err(CursedError::TemplateCursedError {
        }
    }

    fn parse_block_body(&mut self) -> crate::error::Result<()> {
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
        // Skip the end block
        if matches!(self.tokens[self.current], TemplateToken::BlockStart) {
            self.advance(); // Skip {%
            self.advance(); // Skip end/else
            self.advance(); // Skip %}
        Ok(body)
    fn parse_expression(&mut self) -> crate::error::Result<()> {
        self.parse_conditional_expression()
    fn parse_conditional_expression(&mut self) -> crate::error::Result<()> {
        let mut expr = self.parse_logical_or_expression()?;

        if matches!(self.tokens[self.current], TemplateToken::Question) {
            self.advance(); // Skip ?
            let then_expr = Box::new(self.parse_expression()?);
            self.expect(&TemplateToken::Colon)?;
            let else_expr = Box::new(self.parse_expression()?);
            expr = TemplateExpression::Conditional {
        Ok(expr)
    fn parse_logical_or_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_logical_and_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::Or) {
            self.advance();
            let right = self.parse_logical_and_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_logical_and_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_equality_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::And) {
            self.advance();
            let right = self.parse_equality_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_equality_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_relational_expression()?;

                      TemplateToken::EqualEqual | TemplateToken::NotEqual | 
                      TemplateToken::Vibe | TemplateToken::NoVibe) {
            let operator = match self.tokens[self.current] {
            self.advance();
            let right = self.parse_relational_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_relational_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_additive_expression()?;

                      TemplateToken::LessThan | TemplateToken::LessEqual |
                      TemplateToken::GreaterThan | TemplateToken::GreaterEqual |
                      TemplateToken::Slay | TemplateToken::NoSlay) {
            let operator = match self.tokens[self.current] {
            self.advance();
            let right = self.parse_additive_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_additive_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_multiplicative_expression()?;

        while matches!(self.tokens[self.current], TemplateToken::Plus | TemplateToken::Minus) {
            let operator = match self.tokens[self.current] {
            self.advance();
            let right = self.parse_multiplicative_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_multiplicative_expression(&mut self) -> crate::error::Result<()> {
        let mut left = self.parse_unary_expression()?;

                      TemplateToken::Star | TemplateToken::Slash | TemplateToken::Percent) {
            let operator = match self.tokens[self.current] {
            self.advance();
            let right = self.parse_unary_expression()?;
            left = TemplateExpression::BinaryOp {
        Ok(left)
    fn parse_unary_expression(&mut self) -> crate::error::Result<()> {
        match &self.tokens[self.current] {
            TemplateToken::Not => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::UnaryOp {
                })
            }
            TemplateToken::Minus => {
                self.advance();
                let operand = Box::new(self.parse_unary_expression()?);
                Ok(TemplateExpression::UnaryOp {
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
        }
    }

    fn parse_postfix_expression(&mut self) -> crate::error::Result<()> {
        let mut expr = self.parse_primary_expression()?;

        loop {
            match &self.tokens[self.current] {
                TemplateToken::Dot => {
                    self.advance(); // Skip .
                    let property = match &self.tokens[self.current] {
                        _ => return Err(CursedError::TemplateCursedError {
                    self.advance();
                    expr = TemplateExpression::PropertyAccess {
                }
                TemplateToken::LeftBracket => {
                    self.advance(); // Skip [
                    let index = Box::new(self.parse_expression()?);
                    self.expect(&TemplateToken::RightBracket)?;
                    expr = TemplateExpression::IndexAccess {
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
                        return Err(CursedError::TemplateCursedError {
                        });
                    }
                }
            }
        }

        Ok(expr)
    fn parse_primary_expression(&mut self) -> crate::error::Result<()> {
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
                        _ => return Err(CursedError::TemplateCursedError {
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
            _ => Err(CursedError::TemplateCursedError {
        }
    }

    fn expect(&mut self, expected: &TemplateToken) -> crate::error::Result<()> {
        if std::mem::discriminant(&self.tokens[self.current]) == std::mem::discriminant(expected) {
            self.advance();
            Ok(())
        } else {
            Err(CursedError::TemplateCursedError {
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

