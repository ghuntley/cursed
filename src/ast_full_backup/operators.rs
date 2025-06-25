/// Operator expressions for the CURSED programming language
/// 
/// This module contains AST nodes for all operator expressions including
/// binary operations, unary operations, and assignment operations.

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Binary expression (left operator right)
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub token: String,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn new(
        token: String,
        left: Box<dyn Expression>,
        operator: String,
        right: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            left,
            operator,
            right,
        }
    }
}

impl Node for BinaryExpression {
    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(BinaryExpression {
            token: self.token.clone(),
            left: self.left.clone_box(),
            operator: self.operator.clone(),
            right: self.right.clone_box(),
        })
    }
}

/// Alias for BinaryExpression for backward compatibility
pub type InfixExpression = BinaryExpression;

/// Unary expression (operator operand)
#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub token: String,
    pub operator: String,
    pub operand: Box<dyn Expression>,
}

impl UnaryExpression {
    pub fn new(token: String, operator: String, operand: Box<dyn Expression>) -> Self {
        Self {
            token,
            operator,
            operand,
        }
    }
}

impl Node for UnaryExpression {
    fn string(&self) -> String {
        format!("({}{})", self.operator, self.operand.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for UnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(UnaryExpression {
            token: self.token.clone(),
            operator: self.operator.clone(),
            operand: self.operand.clone_box(),
        })
    }
}

/// Alias for UnaryExpression for backward compatibility
pub type PrefixExpression = UnaryExpression;

/// Assignment expression (left = right)
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl AssignmentExpression {
    pub fn new(token: String, name: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
}

impl Node for AssignmentExpression {
    fn string(&self) -> String {
        format!("{} = {}", self.name.string(), self.value.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for AssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(AssignmentExpression {
            token: self.token.clone(),
            name: self.name.clone_box(),
            value: self.value.clone_box(),
        })
    }
}

/// Compound assignment expression (+=, -=, *=, /=, etc.)
#[derive(Debug, Clone)]
pub struct CompoundAssignmentExpression {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub operator: String,
    pub value: Box<dyn Expression>,
}

impl CompoundAssignmentExpression {
    pub fn new(
        token: String,
        name: Box<dyn Expression>,
        operator: String,
        value: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            name,
            operator,
            value,
        }
    }
}

impl Node for CompoundAssignmentExpression {
    fn string(&self) -> String {
        format!("{} {}= {}", self.name.string(), self.operator, self.value.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for CompoundAssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(CompoundAssignmentExpression {
            token: self.token.clone(),
            name: self.name.clone_box(),
            operator: self.operator.clone(),
            value: self.value.clone_box(),
        })
    }
}

/// Index expression (array[index])
#[derive(Debug, Clone)]
pub struct IndexExpression {
    pub token: String,
    pub left: Box<dyn Expression>,
    pub index: Box<dyn Expression>,
}

impl IndexExpression {
    pub fn new(token: String, left: Box<dyn Expression>, index: Box<dyn Expression>) -> Self {
        Self { token, left, index }
    }
}

impl Node for IndexExpression {
    fn string(&self) -> String {
        format!("{}[{}]", self.left.string(), self.index.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for IndexExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IndexExpression {
            token: self.token.clone(),
            left: self.left.clone_box(),
            index: self.index.clone_box(),
        })
    }
}

/// Increment expression (x++ or ++x)
#[derive(Debug, Clone)]
pub struct IncrementExpression {
    pub token: String,
    pub operand: Box<dyn Expression>,
    pub prefix: bool, // true for ++x, false for x++
}

impl IncrementExpression {
    pub fn new(token: String, operand: Box<dyn Expression>, prefix: bool) -> Self {
        Self {
            token,
            operand,
            prefix,
        }
    }
    
    pub fn postfix(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, false)
    }
    
    pub fn prefix_inc(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, true)
    }
}

impl Node for IncrementExpression {
    fn string(&self) -> String {
        if self.prefix {
            format!("++{}", self.operand.string())
        } else {
            format!("{}++", self.operand.string())
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for IncrementExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IncrementExpression {
            token: self.token.clone(),
            operand: self.operand.clone_box(),
            prefix: self.prefix,
        })
    }
}

/// Decrement expression (x-- or --x)
#[derive(Debug, Clone)]
pub struct DecrementExpression {
    pub token: String,
    pub operand: Box<dyn Expression>,
    pub prefix: bool, // true for --x, false for x--
}

impl DecrementExpression {
    pub fn new(token: String, operand: Box<dyn Expression>, prefix: bool) -> Self {
        Self {
            token,
            operand,
            prefix,
        }
    }
    
    pub fn postfix(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, false)
    }
    
    pub fn prefix_dec(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, true)
    }
}

impl Node for DecrementExpression {
    fn string(&self) -> String {
        if self.prefix {
            format!("--{}", self.operand.string())
        } else {
            format!("{}--", self.operand.string())
        }
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for DecrementExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DecrementExpression {
            token: self.token.clone(),
            operand: self.operand.clone_box(),
            prefix: self.prefix,
        })
    }
}

/// Type conversion expression (cast)
#[derive(Debug, Clone)]
pub struct TypeConversionExpression {
    pub token: String,
    pub expression: Box<dyn Expression>,
    pub target_type: Box<dyn Expression>,
}

impl TypeConversionExpression {
    pub fn new(
        token: String,
        expression: Box<dyn Expression>,
        target_type: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            expression,
            target_type,
        }
    }
}

impl Node for TypeConversionExpression {
    fn string(&self) -> String {
        format!("{}({})", self.target_type.string(), self.expression.string())
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeConversionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeConversionExpression {
            token: self.token.clone(),
            expression: self.expression.clone_box(),
            target_type: self.target_type.clone_box(),
        })
    }
}

/// Binary operator types
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Subtract,
    /// Multiplication (*)
    Multiply,
    /// Division (/)
    Divide,
    /// Modulo (%)
    Modulo,
    /// Exponentiation (**)
    Power,
    /// Equality (==)
    Equal,
    /// Inequality (!=)
    NotEqual,
    /// Less than (<)
    LessThan,
    /// Less than or equal (<=)
    LessThanOrEqual,
    /// Greater than (>)
    GreaterThan,
    /// Greater than or equal (>=)
    GreaterThanOrEqual,
    /// Logical AND (&&)
    LogicalAnd,
    /// Logical OR (||)
    LogicalOr,
    /// Bitwise AND (&)
    BitwiseAnd,
    /// Bitwise OR (|)
    BitwiseOr,
    /// Bitwise XOR (^)
    BitwiseXor,
    /// Left shift (<<)
    LeftShift,
    /// Right shift (>>)
    RightShift,
    /// String concatenation (+)
    Concatenate,
    /// Range operator (..)
    Range,
    /// Inclusive range (...) 
    InclusiveRange,
}

impl BinaryOperator {
    /// Get the string representation of the operator
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Power => "**",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::LogicalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
            BinaryOperator::BitwiseAnd => "&",
            BinaryOperator::BitwiseOr => "|",
            BinaryOperator::BitwiseXor => "^",
            BinaryOperator::LeftShift => "<<",
            BinaryOperator::RightShift => ">>",
            BinaryOperator::Concatenate => "+",
            BinaryOperator::Range => "..",
            BinaryOperator::InclusiveRange => "...",
        }
    }

    /// Parse operator from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(BinaryOperator::Add),
            "-" => Some(BinaryOperator::Subtract),
            "*" => Some(BinaryOperator::Multiply),
            "/" => Some(BinaryOperator::Divide),
            "%" => Some(BinaryOperator::Modulo),
            "**" => Some(BinaryOperator::Power),
            "==" => Some(BinaryOperator::Equal),
            "!=" => Some(BinaryOperator::NotEqual),
            "<" => Some(BinaryOperator::LessThan),
            "<=" => Some(BinaryOperator::LessThanOrEqual),
            ">" => Some(BinaryOperator::GreaterThan),
            ">=" => Some(BinaryOperator::GreaterThanOrEqual),
            "&&" => Some(BinaryOperator::LogicalAnd),
            "||" => Some(BinaryOperator::LogicalOr),
            "&" => Some(BinaryOperator::BitwiseAnd),
            "|" => Some(BinaryOperator::BitwiseOr),
            "^" => Some(BinaryOperator::BitwiseXor),
            "<<" => Some(BinaryOperator::LeftShift),
            ">>" => Some(BinaryOperator::RightShift),
            ".." => Some(BinaryOperator::Range),
            "..." => Some(BinaryOperator::InclusiveRange),
            _ => None,
        }
    }

    /// Get operator precedence
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::LogicalOr => 1,
            BinaryOperator::LogicalAnd => 2,
            BinaryOperator::BitwiseOr => 3,
            BinaryOperator::BitwiseXor => 4,
            BinaryOperator::BitwiseAnd => 5,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 6,
            BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual |
            BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual => 7,
            BinaryOperator::Range | BinaryOperator::InclusiveRange => 8,
            BinaryOperator::LeftShift | BinaryOperator::RightShift => 9,
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Concatenate => 10,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo => 11,
            BinaryOperator::Power => 12,
        }
    }

    /// Check if operator is right-associative
    pub fn is_right_associative(&self) -> bool {
        matches!(self, BinaryOperator::Power)
    }
}

/// Unary operator types
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// Unary plus (+)
    Plus,
    /// Unary minus (-)
    Minus,
    /// Logical NOT (!)
    LogicalNot,
    /// Bitwise NOT (~)
    BitwiseNot,
    /// Reference (&)
    Reference,
    /// Dereference (*)
    Dereference,
    /// Pre-increment (++)
    PreIncrement,
    /// Pre-decrement (--)
    PreDecrement,
    /// Post-increment (++)
    PostIncrement,
    /// Post-decrement (--)
    PostDecrement,
}

impl UnaryOperator {
    /// Get the string representation of the operator
    pub fn as_str(&self) -> &'static str {
        match self {
            UnaryOperator::Plus => "+",
            UnaryOperator::Minus => "-",
            UnaryOperator::LogicalNot => "!",
            UnaryOperator::BitwiseNot => "~",
            UnaryOperator::Reference => "&",
            UnaryOperator::Dereference => "*",
            UnaryOperator::PreIncrement => "++",
            UnaryOperator::PreDecrement => "--",
            UnaryOperator::PostIncrement => "++",
            UnaryOperator::PostDecrement => "--",
        }
    }

    /// Parse operator from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "+" => Some(UnaryOperator::Plus),
            "-" => Some(UnaryOperator::Minus),
            "!" => Some(UnaryOperator::LogicalNot),
            "~" => Some(UnaryOperator::BitwiseNot),
            "&" => Some(UnaryOperator::Reference),
            "*" => Some(UnaryOperator::Dereference),
            "++" => Some(UnaryOperator::PreIncrement),
            "--" => Some(UnaryOperator::PreDecrement),
            _ => None,
        }
    }

    /// Check if operator is prefix
    pub fn is_prefix(&self) -> bool {
        matches!(self, 
            UnaryOperator::Plus | UnaryOperator::Minus | UnaryOperator::LogicalNot |
            UnaryOperator::BitwiseNot | UnaryOperator::Reference | UnaryOperator::Dereference |
            UnaryOperator::PreIncrement | UnaryOperator::PreDecrement
        )
    }

    /// Check if operator is postfix
    pub fn is_postfix(&self) -> bool {
        matches!(self, UnaryOperator::PostIncrement | UnaryOperator::PostDecrement)
    }
}

/// Helper functions for creating operator expressions
pub fn binary_expr(left: Box<dyn Expression>, op: &str, right: Box<dyn Expression>) -> BinaryExpression {
    BinaryExpression::new(op.to_string(), left, op.to_string(), right)
}
    
pub fn unary_expr(op: &str, operand: Box<dyn Expression>) -> UnaryExpression {
    UnaryExpression::new(op.to_string(), op.to_string(), operand)
}
    
pub fn assign_expr(name: Box<dyn Expression>, value: Box<dyn Expression>) -> AssignmentExpression {
    AssignmentExpression::new("=".to_string(), name, value)
}
