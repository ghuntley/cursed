use std::any::Any;
use crate::lexer::token::Token;
use crate::ast::{Node, Expression};

/// TypeConversionExpression represents a type conversion expression
pub struct TypeConversionExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
    pub type_name: String,
}

impl Node for TypeConversionExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{} as {}", self.expression.string(), self.type_name)
    }
}

impl Expression for TypeConversionExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// ChannelExpression represents a channel creation expression
pub struct ChannelExpression {
    pub token: Token,
    pub element_type: Box<dyn Expression>,
    pub capacity: Option<Box<dyn Expression>>,
}

impl Node for ChannelExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("chan {}", self.element_type.string())
    }
}

impl Expression for ChannelExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// SendExpression represents sending a value to a channel
pub struct SendExpression {
    pub token: Token,
    pub channel: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl Node for SendExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("{} <- {}", self.channel.string(), self.value.string())
    }
}

impl Expression for SendExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// ReceiveExpression represents receiving a value from a channel
pub struct ReceiveExpression {
    pub token: Token,
    pub channel: Box<dyn Expression>,
}

impl Node for ReceiveExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("<-{}", self.channel.string())
    }
}

impl Expression for ReceiveExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// StanExpression represents a goroutine creation expression
pub struct StanExpression {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Node for StanExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        format!("stan {}", self.expression.string())
    }
}

impl Expression for StanExpression {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// ArrayLiteral represents an array literal expression
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Box<dyn Expression>>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        let elements = self.elements
            .iter()
            .map(|e| e.string())
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("[{}]", elements)
    }
}

impl Expression for ArrayLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// HashLiteral represents a hash literal expression
pub struct HashLiteral {
    pub token: Token,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl Node for HashLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
    
    fn string(&self) -> String {
        let pairs = self.pairs
            .iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("{{{}}}", pairs)
    }
}

impl Expression for HashLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any { self }
}

/// Identifier represents an identifier
#[derive(Clone)]
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
        format!("\"{}\"" , self.value)
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

/// ByteLiteral represents a byte literal (single ASCII character)
pub struct ByteLiteral {
    pub token: String,
    pub value: u8,
}

impl Node for ByteLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("b'{}'", self.value as char)
    }
}

impl Expression for ByteLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// RuneLiteral represents a rune (Unicode character) literal
pub struct RuneLiteral {
    pub token: String,
    pub value: char,
}

impl Node for RuneLiteral {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("'{}'", self.value)
    }
}

impl Expression for RuneLiteral {
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

/// GenericCallExpression represents a call expression with generic type arguments
pub struct GenericCallExpression {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub type_arguments: Vec<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for GenericCallExpression {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let type_args: Vec<String> = self.type_arguments.iter()
            .map(|arg| arg.string())
            .collect();
        let args: Vec<String> = self.arguments.iter()
            .map(|arg| arg.string())
            .collect();
        format!("{} {} [{}] {}", 
                self.function.string(), 
                self.token_literal(),
                type_args.join(", "),
                args.join(", "))
    }
}

impl Expression for GenericCallExpression {
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

/// BeLikeExpression represents a struct instantiation expression
pub struct BeLikeExpression {
    pub token: String,
    pub struct_name: Identifier,
    pub type_arguments: Vec<Box<dyn Expression>>, // Generic type arguments [normie], [tea, normie], etc.
    pub fields: Vec<(String, Box<dyn Expression>)>,
}

impl Node for BeLikeExpression {
    fn token_literal(&self) -> String {
        self.token.clone()
    }
    
    fn string(&self) -> String {
        let mut out = format!("be_like {}", self.struct_name.string());
        
        // Format type arguments if present
        if !self.type_arguments.is_empty() {
            let type_args: Vec<String> = self.type_arguments.iter()
                .map(|arg| arg.string())
                .collect();
            out.push_str(&format!("[{}]", type_args.join(", ")));
        }
        
        if !self.fields.is_empty() {
            out.push_str(" {");
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