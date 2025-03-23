use std::fmt;
use std::any::Any;
use crate::lexer::Token;

/// Node represents a node in the abstract syntax tree
pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

/// Statement represents a statement node in the AST
pub trait Statement: Node {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

/// Expression represents an expression node in the AST
pub trait Expression: Node {
    fn expression_node(&self);
    fn as_any(&self) -> &dyn Any;

    /// Returns true if this expression is a prefix expression
    fn is_prefix_expression(&self) -> bool {
        false
    }

    /// Returns the operator and right expression if this is a prefix expression
    fn as_prefix_expression(&self) -> Option<(String, &dyn Expression)> {
        None
    }

    /// Returns true if this expression is an infix expression
    fn is_infix_expression(&self) -> bool {
        false
    }

    /// Returns the left, operator, and right expressions if this is an infix expression
    fn as_infix_expression(&self) -> Option<(&dyn Expression, String, &dyn Expression)> {
        None
    }

    /// Returns true if this expression is a call expression
    fn is_call_expression(&self) -> bool {
        false
    }

    /// Returns the function and arguments if this is a call expression
    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        None
    }

    /// Returns true if this expression is an index expression
    fn is_index_expression(&self) -> bool {
        false
    }

    /// Returns the left and index expressions if this is an index expression
    fn as_index_expression(&self) -> Option<(&dyn Expression, &dyn Expression)> {
        None
    }
}

/// Program represents a CURSED program
#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::new()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program {{")?;
        for (i, stmt) in self.statements.iter().enumerate() {
            writeln!(f, "  Statement {}: {}", i, stmt.string())?;
        }
        writeln!(f, "}}")
    }
}

/// ExpressionStatement represents an expression used as a statement
pub struct ExpressionStatement {
    pub token: String,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        if let Some(expr) = &self.expression {
            expr.string()
        } else {
            String::new()
        }
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// PackageStatement represents a package declaration
pub struct PackageStatement {
    pub token: String, // Token::Vibe
    pub name: Identifier,
}

impl Node for PackageStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {};", self.token_literal(), self.name.string())
    }
}

impl Statement for PackageStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ImportStatement represents an import declaration
pub struct ImportStatement {
    pub token: String, // Token::Yeet
    pub path: StringLiteral,
    pub alias: Option<Identifier>,
}

impl Node for ImportStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        if let Some(alias) = &self.alias {
            format!("{} {} {};", self.token_literal(), alias.string(), self.path.string())
        } else {
            format!("{} {};", self.token_literal(), self.path.string())
        }
    }
}

impl Statement for ImportStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Identifier represents an identifier
pub struct Identifier {
    pub token: String,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// StringLiteral represents a string literal
pub struct StringLiteral {
    pub token: String,
    pub value: String,
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("\"{}\"", self.value)
    }
}

impl Expression for StringLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// IntegerLiteral represents an integer literal
pub struct IntegerLiteral {
    pub token: String,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// FloatLiteral represents a floating-point literal
pub struct FloatLiteral {
    pub token: String,
    pub value: f64,
}

impl Node for FloatLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for FloatLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// BooleanLiteral represents a boolean literal
pub struct BooleanLiteral {
    pub token: String,
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for BooleanLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// PrefixExpression represents a prefix expression
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {}", self.operator, self.right.string())
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_prefix_expression(&self) -> bool {
        true
    }

    fn as_prefix_expression(&self) -> Option<(String, &dyn Expression)> {
        Some((self.operator.clone(), self.right.as_ref()))
    }
}

/// InfixExpression represents an infix expression
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left.string(), self.operator, self.right.string())
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_infix_expression(&self) -> bool {
        true
    }

    fn as_infix_expression(&self) -> Option<(&dyn Expression, String, &dyn Expression)> {
        Some((self.left.as_ref(), self.operator.clone(), self.right.as_ref()))
    }
}

/// CallExpression represents a call expression
pub struct CallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("{} {} {}", self.function.string(), self.token_literal(), args.join(", "))
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_call_expression(&self) -> bool {
        true
    }

    fn as_call_expression(&self) -> Option<(&dyn Expression, Vec<&dyn Expression>)> {
        let args: Vec<&dyn Expression> = self.arguments.iter()
            .map(|arg| arg.as_ref() as &dyn Expression)
            .collect();
        Some((self.function.as_ref(), args))
    }
}

/// IndexExpression represents an index expression
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
}

impl Node for IndexExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.left.string(), self.token_literal(), self.index.string())
    }
}

impl Expression for IndexExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_index_expression(&self) -> bool {
        true
    }

    fn as_index_expression(&self) -> Option<(&dyn Expression, &dyn Expression)> {
        Some((self.left.as_ref(), self.index.as_ref()))
    }
}

/// LetStatement represents a let statement
pub struct LetStatement {
    pub token: String, // Token::Let
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("{} {} = ", self.token_literal(), self.name.string());
        if let Some(value) = &self.value {
            out.push_str(&value.string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ReturnStatement represents a return statement
pub struct ReturnStatement {
    pub token: String, // Token::Return
    pub return_value: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("{} ", self.token_literal());
        if let Some(value) = &self.return_value {
            out.push_str(&value.string());
        }
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// BlockStatement represents a block of statements
pub struct BlockStatement {
    pub token: String, // Token::LBrace
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// IfStatement represents an if statement
pub struct IfStatement {
    pub token: String, // Token::If
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("if {} {}", self.condition.string(), self.consequence.string());
        if let Some(alt) = &self.alternative {
            out.push_str(&format!(" else {}", alt.string()));
        }
        out
    }
}

impl Statement for IfStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
} 