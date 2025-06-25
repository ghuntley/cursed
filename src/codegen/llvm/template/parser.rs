// Template parser for LLVM code generation
use crate::error_types::CursedError;
use super::lexer::{TemplateLexer, TemplateToken};

/// Template parser for processing LLVM code templates
#[derive(Debug)]
pub struct TemplateParser {
    pub tokens: Vec<TemplateToken>,
    pub current: usize,
}

impl TemplateParser {
    pub fn new(tokens: Vec<TemplateToken>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> crate::error_types::Result<TemplateAst> {
        let mut nodes = Vec::new();
        
        while !self.is_at_end() {
            let node = self.parse_node()?;
            nodes.push(node);
        }
        
        Ok(TemplateAst { nodes })
    }

    fn parse_node(&mut self) -> crate::error_types::Result<TemplateNode> {
        match self.peek() {
            Some(TemplateToken::OpenExpression) => self.parse_expression(),
            Some(TemplateToken::OpenStatement) => self.parse_statement(),
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
            _ => return Err(CursedError::Parse("Expected expression".to_string())),
        };
        
        self.consume(TemplateToken::CloseExpression)?;
        Ok(TemplateNode::Expression(expr))
    }

    fn parse_statement(&mut self) -> crate::error_types::Result<TemplateNode> {
        self.consume(TemplateToken::OpenStatement)?;
        
        let stmt = match self.peek() {
            Some(TemplateToken::If) => self.parse_if_statement()?,
            Some(TemplateToken::For) => self.parse_for_statement()?,
            Some(TemplateToken::Include) => self.parse_include_statement()?,
            Some(TemplateToken::Block) => self.parse_block_statement()?,
            _ => return Err(CursedError::Parse("Expected statement".to_string())),
        };
        
        self.consume(TemplateToken::CloseStatement)?;
        Ok(TemplateNode::Statement(stmt))
    }

    fn parse_if_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::If)?;
        
        let condition = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                TemplateCondition::Variable(name)
            }
            _ => return Err(CursedError::Parse("Expected condition".to_string())),
        };
        
        // Parse the body (simplified - just collect until endif)
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndIf)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        }
        
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndIf)?;
        }
        
        Ok(TemplateStatement::If { condition, body, else_body: None })
    }

    fn parse_for_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::For)?;
        
        let var = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(CursedError::Parse("Expected variable name".to_string())),
        };
        
        // TODO: Parse 'in' keyword and iterable
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndFor)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        }
        
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndFor)?;
        }
        
        Ok(TemplateStatement::For { var, iterable: "items".to_string(), body })
    }

    fn parse_include_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::Include)?;
        
        let template = match self.peek() {
            Some(TemplateToken::String(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(CursedError::Parse("Expected template name".to_string())),
        };
        
        Ok(TemplateStatement::Include(template))
    }

    fn parse_block_statement(&mut self) -> crate::error_types::Result<TemplateStatement> {
        self.consume(TemplateToken::Block)?;
        
        let name = match self.peek() {
            Some(TemplateToken::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(CursedError::Parse("Expected block name".to_string())),
        };
        
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(TemplateToken::EndBlock)) && !self.is_at_end() {
            body.push(self.parse_node()?);
        }
        
        if !self.is_at_end() {
            self.consume_statement_with_token(TemplateToken::EndBlock)?;
        }
        
        Ok(TemplateStatement::Block { name, body })
    }

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
    }

    fn peek(&self) -> Option<&TemplateToken> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&TemplateToken> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek(), Some(TemplateToken::Eof))
    }
}

/// Template AST
#[derive(Debug, Clone)]
pub struct TemplateAst {
    pub nodes: Vec<TemplateNode>,
}

/// Template AST node
#[derive(Debug, Clone)]
pub enum TemplateNode {
    Text(String),
    Expression(TemplateExpression),
    Statement(TemplateStatement),
}

/// Template expression
#[derive(Debug, Clone)]
pub enum TemplateExpression {
    Variable(String),
    String(String),
    Number(i64),
}

/// Template statement
#[derive(Debug, Clone)]
pub enum TemplateStatement {
    If {
        condition: TemplateCondition,
        body: Vec<TemplateNode>,
        else_body: Option<Vec<TemplateNode>>,
    },
    For {
        var: String,
        iterable: String,
        body: Vec<TemplateNode>,
    },
    Include(String),
    Block {
        name: String,
        body: Vec<TemplateNode>,
    },
}

/// Template condition
#[derive(Debug, Clone)]
pub enum TemplateCondition {
    Variable(String),
    Comparison {
        left: Box<TemplateExpression>,
        op: ComparisonOp,
        right: Box<TemplateExpression>,
    },
}

/// Comparison operators
#[derive(Debug, Clone)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
}
