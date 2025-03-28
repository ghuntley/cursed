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

/// AssignmentExpression represents an assignment expression (e.g., x = 5)
pub struct AssignmentExpression {
    pub token: String, // Token::Assign
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Node for AssignmentExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {} {}", self.name.string(), self.token_literal(), self.value.string())
    }
}

impl Expression for AssignmentExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
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

/// WhileStatement represents a while loop (periodt in CURSED)
pub struct WhileStatement {
    pub token: String, // Token::Periodt
    pub condition: Box<dyn Expression>,
    pub body: BlockStatement,
}

impl Node for WhileStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("periodt {} {}", self.condition.string(), self.body.string())
    }
}

impl Statement for WhileStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// ForStatement represents a for loop (bestie in CURSED)
/// A for loop can have three different forms:
/// 1. C-style: bestie init; condition; post { body }
/// 2. Condition-only: bestie condition { body }
/// 3. Infinite loop: bestie { body }
pub struct ForStatement {
    pub token: String, // Token::Bestie
    pub init: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Expression>>,
    pub post: Option<Box<dyn Statement>>,
    pub body: BlockStatement,
}

impl Node for ForStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("bestie ");
        
        if let Some(init) = &self.init {
            out.push_str(&init.string());
            out.push_str("; ");
        }
        
        if let Some(cond) = &self.condition {
            out.push_str(&cond.string());
        }
        
        if let Some(post) = &self.post {
            out.push_str("; ");
            out.push_str(&post.string());
        }
        
        out.push_str(" ");
        out.push_str(&self.body.string());
        
        out
    }
}

impl Statement for ForStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// SwitchStatement represents a switch statement (vibe_check in CURSED)
pub struct SwitchStatement {
    pub token: String, // Token::VibeCheck
    pub value: Box<dyn Expression>,
    pub cases: Vec<CaseStatement>,
    pub default: Option<BlockStatement>,
}

impl Node for SwitchStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("vibe_check {} {{\n", self.value.string());
        
        for case in &self.cases {
            out.push_str(&format!("    {}\n", case.string()));
        }
        
        if let Some(default) = &self.default {
            out.push_str(&format!("    basic: {}\n", default.string()));
        }
        
        out.push_str("}");
        out
    }
}

impl Statement for SwitchStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// CaseStatement represents a case clause in a switch statement
pub struct CaseStatement {
    pub token: String, // Token::Mood
    pub expressions: Vec<Box<dyn Expression>>,
    pub body: BlockStatement,
}

impl Node for CaseStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let exprs: Vec<String> = self.expressions.iter()
            .map(|expr| expr.string())
            .collect();
        format!("mood {}: {}", exprs.join(", "), self.body.string())
    }
}

impl Statement for CaseStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// SquadStatement represents a struct definition
pub struct SquadStatement {
    pub token: String, // Token::Squad
    pub name: Identifier,
    pub fields: Vec<FieldStatement>,
}

impl Node for SquadStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("be_like {} squad {{\n", self.name.string()));
        
        for field in &self.fields {
            out.push_str(&format!("    {}\n", field.string()));
        }
        
        out.push_str("}\n");
        out
    }
}

impl Statement for SquadStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// FieldStatement represents a field definition in a struct
pub struct FieldStatement {
    pub token: String, // Usually the identifier token
    pub name: Identifier,
    pub type_name: Identifier,
}

impl Node for FieldStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("{} {}", self.name.string(), self.type_name.string())
    }
}

impl Statement for FieldStatement {
    fn statement_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// BeLikeExpression represents a struct instantiation expression
pub struct BeLikeExpression {
    pub token: String,
    pub struct_name: Identifier,
    pub fields: Vec<(String, Box<dyn Expression>)>,
}

impl Node for BeLikeExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let mut out = format!("be_like {}", self.struct_name.string());
        
        if !self.fields.is_empty() {
            out.push_str(" with {");
            let fields_str: Vec<String> = self.fields.iter()
                .map(|(name, expr)| format!("{}: {}", name, expr.string()))
                .collect();
            out.push_str(&fields_str.join(", "));
            out.push_str("}");
        }
        
        out
    }
}

impl Expression for BeLikeExpression {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_node() {
        let identifier = Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        };
        
        assert_eq!(identifier.token_literal(), "x");
        assert_eq!(identifier.string(), "x");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &identifier;
        assert_eq!(expr.token_literal(), "x");
        assert_eq!(expr.string(), "x");
        assert!(!expr.is_prefix_expression());
        assert!(!expr.is_infix_expression());
        assert!(!expr.is_call_expression());
        assert!(!expr.is_index_expression());
    }
    
    #[test]
    fn test_integer_literal_node() {
        let int_literal = IntegerLiteral {
            token: "42".to_string(),
            value: 42,
        };
        
        assert_eq!(int_literal.token_literal(), "42");
        assert_eq!(int_literal.string(), "42");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &int_literal;
        assert_eq!(expr.token_literal(), "42");
        assert_eq!(expr.string(), "42");
    }
    
    #[test]
    fn test_float_literal_node() {
        let float_literal = FloatLiteral {
            token: "3.14".to_string(),
            value: 3.14,
        };
        
        assert_eq!(float_literal.token_literal(), "3.14");
        assert_eq!(float_literal.string(), "3.14");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &float_literal;
        assert_eq!(expr.token_literal(), "3.14");
        assert_eq!(expr.string(), "3.14");
    }
    
    #[test]
    fn test_string_literal_node() {
        let string_literal = StringLiteral {
            token: "hello".to_string(),
            value: "hello".to_string(),
        };
        
        assert_eq!(string_literal.token_literal(), "hello");
        assert_eq!(string_literal.string(), "\"hello\"");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &string_literal;
        assert_eq!(expr.token_literal(), "hello");
        assert_eq!(expr.string(), "\"hello\"");
    }
    
    #[test]
    fn test_boolean_literal_node() {
        // Test true value
        let true_literal = BooleanLiteral {
            token: "true".to_string(),
            value: true,
        };
        
        assert_eq!(true_literal.token_literal(), "true");
        assert_eq!(true_literal.string(), "true");
        
        // Test false value
        let false_literal = BooleanLiteral {
            token: "false".to_string(),
            value: false,
        };
        
        assert_eq!(false_literal.token_literal(), "false");
        assert_eq!(false_literal.string(), "false");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &true_literal;
        assert_eq!(expr.token_literal(), "true");
        assert_eq!(expr.string(), "true");
    }
    
    #[test]
    fn test_expression_statement() {
        let expr = Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        }) as Box<dyn Expression>;
        
        let stmt = ExpressionStatement {
            token: "expression".to_string(),
            expression: Some(expr),
        };
        
        assert_eq!(stmt.token_literal(), "expression");
        assert_eq!(stmt.string(), "x");
        
        // Test Statement trait implementation
        let stmt_trait: &dyn Statement = &stmt;
        assert_eq!(stmt_trait.token_literal(), "expression");
        assert_eq!(stmt_trait.string(), "x");
    }
    
    #[test]
    fn test_program_with_statements() {
        let stmt1 = Box::new(ExpressionStatement {
            token: "expression".to_string(),
            expression: Some(Box::new(IntegerLiteral {
                token: "5".to_string(),
                value: 5,
            }) as Box<dyn Expression>),
        }) as Box<dyn Statement>;
        
        let stmt2 = Box::new(ExpressionStatement {
            token: "expression".to_string(),
            expression: Some(Box::new(StringLiteral {
                token: "hello".to_string(),
                value: "hello".to_string(),
            }) as Box<dyn Expression>),
        }) as Box<dyn Statement>;
        
        let program = Program {
            statements: vec![stmt1, stmt2],
        };
        
        // First statement's token should be the program's token
        assert_eq!(program.token_literal(), "expression");
        
        // String representation should not be empty
        let program_str = program.string();
        assert!(!program_str.is_empty());
    }
    
    #[test]
    fn test_empty_program() {
        let program = Program {
            statements: Vec::new(),
        };
        
        assert_eq!(program.token_literal(), "");
        assert_eq!(program.string(), "");
    }
    
    #[test]
    fn test_prefix_expression() {
        let expr = Box::new(Identifier {
            token: "x".to_string(),
            value: "x".to_string(),
        }) as Box<dyn Expression>;
        
        let prefix_expr = PrefixExpression {
            token: Token::Bang,
            operator: "!".to_string(),
            right: expr,
        };
        
        assert_eq!(prefix_expr.token_literal(), "!");
        assert_eq!(prefix_expr.string(), "! x");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &prefix_expr;
        assert!(expr.is_prefix_expression());
        
        if let Some((op, right)) = expr.as_prefix_expression() {
            assert_eq!(op, "!");
            assert_eq!(right.string(), "x");
        } else {
            panic!("Expected prefix expression");
        }
    }
    
    #[test]
    fn test_infix_expression() {
        let left = Box::new(IntegerLiteral {
            token: "5".to_string(),
            value: 5,
        }) as Box<dyn Expression>;
        
        let right = Box::new(IntegerLiteral {
            token: "10".to_string(),
            value: 10,
        }) as Box<dyn Expression>;
        
        let infix_expr = InfixExpression {
            token: Token::Plus,
            left,
            operator: "+".to_string(),
            right,
        };
        
        assert_eq!(infix_expr.token_literal(), "+");
        assert_eq!(infix_expr.string(), "5 + 10");
        
        // Test Expression trait implementation
        let expr: &dyn Expression = &infix_expr;
        assert!(expr.is_infix_expression());
        
        if let Some((left, op, right)) = expr.as_infix_expression() {
            assert_eq!(left.string(), "5");
            assert_eq!(op, "+");
            assert_eq!(right.string(), "10");
        } else {
            panic!("Expected infix expression");
        }
    }
} 