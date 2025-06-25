// Template parser for LLVM code generation
use crate::error_types::CursedError;
use super::lexer::{TemplateLexer, TemplateToken};

/// Template parser for processing LLVM code templates
#[derive(Debug)]
pub struct TemplateParser {
impl TemplateParser {
    pub fn new(tokens: Vec<TemplateToken>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> crate::error_types::Result<TemplateAst> {
        let mut nodes = Vec::new();
        
        while !self.is_at_end() {
            let node = self.parse_node()?;
            nodes.push(node);
        Ok(TemplateAst { nodes })
    fn parse_node(&mut self) -> crate::error_types::Result<TemplateNode> {
        match self.peek() {
            Some(TemplateToken::Text(text)) => {
                let text = text.clone();
                self.advance();
                Ok(TemplateNode::Text(text))
            }
            _ => {
                let token = self.advance().cloned().unwrap_or(TemplateToken::Eof);
                Err(CursedError::Parse(format!("Unexpected token: {:?}", token)))
            }
        }
    fn parse_expression(&mut self) -> crate::error_types::Result<TemplateNode> {
        self.consume(TemplateToken::OpenExpression)?;
        
        let expr = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                TemplateExpression::Variable(name)
            }
            Some(TemplateToken::String(value)) => {
                let value = value.clone();
                self.advance();
                TemplateExpression::String(value)
            }
            Some(TemplateToken::Number(value)) => {
                let value = *value;
                self.advance();
                TemplateExpression::Number(value)
            }
        
        self.consume(TemplateToken::CloseExpression)?;
        Ok(TemplateNode::Expression(expr))
    fn parse_statement(&mut self) -> crate::error_types::Result<TemplateNode> {
        self.consume(TemplateToken::OpenStatement)?;
        
        let stmt = match self.peek() {
        
        self.consume(TemplateToken::CloseStatement)?;
        Ok(TemplateNode::Statement(stmt))
    fn parse_if_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::If)?;
        
        let condition = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                TemplateCondition::Variable(name)
            }
        
        // Parse the body (simplified - just collect until endif)
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndIf)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndIf)?;
        Ok(TemplateStatement::If { condition, body, else_body: None })
    fn parse_for_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::For)?;
        
        let var = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
        
        // TODO: Parse 'in' keyword and iterable
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndFor)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndFor)?;
        Ok(TemplateStatement::For { var, iterable: "items".to_string(), body })
    fn parse_include_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::Include)?;
        
        let template = match self.peek() {
            Some(TemplateToken::String(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
        
        Ok(TemplateStatement::Include(template))
    fn parse_block_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::Block)?;
        
        let name = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndBlock)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndBlock)?;
        Ok(TemplateStatement::Block { name, body })
    fn consume(&mut self, expected: TemplateToken) -> crate::error_types::Result<()> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(CursedError::Parse(format!("Expected {:?}, found {:?}", expected, self.peek())))
        }
    }

    fn consume_statement_with_token(&mut self, token: TemplateToken) -> crate::error_types::Result<()> {
        self.consume(TemplateToken::OpenStatement)?;
        self.consume(token)?;
        self.consume(TemplateToken::CloseStatement)?;
        Ok(())
    fn peek(&self) -> Option<&TemplateToken> {
        self.tokens.get(self.current)
    fn advance(&mut self) -> Option<&TemplateToken> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek(), Some(TemplateToken::Eof))
    }
}

/// Template AST
#[derive(Debug, Clone)]
pub struct TemplateAst {
/// Template AST node
#[derive(Debug, Clone)]
pub enum TemplateNode {
/// Template expression
#[derive(Debug, Clone)]
pub enum TemplateExpression {
/// Template statement
#[derive(Debug, Clone)]
pub enum TemplateStatement {
    If {
    For {
    Block {
/// Template condition
#[derive(Debug, Clone)]
pub enum TemplateCondition {
    Comparison {
/// Comparison operators
#[derive(Debug, Clone)]
pub enum ComparisonOp {
}
