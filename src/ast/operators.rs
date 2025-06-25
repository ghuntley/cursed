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
impl BinaryExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
impl Node for BinaryExpression {
    fn string(&self) -> String {
        format!("({} {} {})", self.left.string(), self.operator, self.right.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(BinaryExpression {
        })
    }
}

/// Alias for BinaryExpression for backward compatibility
pub type InfixExpression = BinaryExpression;

/// Unary expression (operator operand)
#[derive(Debug, Clone)]
pub struct UnaryExpression {
impl UnaryExpression {
    pub fn new(token: String, operator: String, operand: Box<dyn Expression>) -> Self {
        Self {
        }
    }
impl Node for UnaryExpression {
    fn string(&self) -> String {
        format!("({}{})", self.operator, self.operand.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for UnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(UnaryExpression {
        })
    }
}

/// Alias for UnaryExpression for backward compatibility
pub type PrefixExpression = UnaryExpression;

/// Assignment expression (left = right)
#[derive(Debug, Clone)]
pub struct AssignmentExpression {
impl AssignmentExpression {
    pub fn new(token: String, name: Box<dyn Expression>, value: Box<dyn Expression>) -> Self {
        Self { token, name, value }
    }
impl Node for AssignmentExpression {
    fn string(&self) -> String {
        format!("{} = {}", self.name.string(), self.value.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for AssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(AssignmentExpression {
        })
    }
}

/// Compound assignment expression (+=, -=, *=, /=, etc.)
#[derive(Debug, Clone)]
pub struct CompoundAssignmentExpression {
impl CompoundAssignmentExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
impl Node for CompoundAssignmentExpression {
    fn string(&self) -> String {
        format!("{} {}= {}", self.name.string(), self.operator, self.value.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for CompoundAssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(CompoundAssignmentExpression {
        })
    }
}

/// Index expression (array[index])
#[derive(Debug, Clone)]
pub struct IndexExpression {
impl IndexExpression {
    pub fn new(token: String, left: Box<dyn Expression>, index: Box<dyn Expression>) -> Self {
        Self { token, left, index }
    }
impl Node for IndexExpression {
    fn string(&self) -> String {
        format!("{}[{}]", self.left.string(), self.index.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for IndexExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IndexExpression {
        })
    }
}

/// Increment expression (x++ or ++x)
#[derive(Debug, Clone)]
pub struct IncrementExpression {
    pub prefix: bool, // true for ++x, false for x++
impl IncrementExpression {
    pub fn new(token: String, operand: Box<dyn Expression>, prefix: bool) -> Self {
        Self {
        }
    }
    
    pub fn postfix(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, false)
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
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IncrementExpression {
        })
    }
}

/// Decrement expression (x-- or --x)
#[derive(Debug, Clone)]
pub struct DecrementExpression {
    pub prefix: bool, // true for --x, false for x--
impl DecrementExpression {
    pub fn new(token: String, operand: Box<dyn Expression>, prefix: bool) -> Self {
        Self {
        }
    }
    
    pub fn postfix(token: String, operand: Box<dyn Expression>) -> Self {
        Self::new(token, operand, false)
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
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(DecrementExpression {
        })
    }
}

/// Type conversion expression (cast)
#[derive(Debug, Clone)]
pub struct TypeConversionExpression {
impl TypeConversionExpression {
    pub fn new(
    ) -> Self {
        Self {
        }
    }
impl Node for TypeConversionExpression {
    fn string(&self) -> String {
        format!("{}({})", self.target_type.string(), self.expression.string())
    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for TypeConversionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(TypeConversionExpression {
        })
    }
}

/// Binary operator types
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// Addition (+)
    /// Subtraction (-)
    /// Multiplication (*)
    /// Division (/)
    /// Modulo (%)
    /// Exponentiation (**)
    /// Equality (==)
    /// Inequality (!=)
    /// Less than (<)
    /// Less than or equal (<=)
    /// Greater than (>)
    /// Greater than or equal (>=)
    /// Logical AND (&&)
    /// Logical OR (||)
    /// Bitwise AND (&)
    /// Bitwise OR (|)
    /// Bitwise XOR (^)
    /// Left shift (<<)
    /// Right shift (>>)
    /// String concatenation (+)
    /// Range operator (..)
    /// Inclusive range (...) 
impl BinaryOperator {
    /// Get the string representation of the operator
    pub fn as_str(&self) -> &'static str {
        match self {
            BinaryOperator::Divide => "/",
        }
    }

    /// Parse operator from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "/" => Some(BinaryOperator::Divide),
        }
    }

    /// Get operator precedence
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual |
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
    /// Unary minus (-)
    /// Logical NOT (!)
    /// Bitwise NOT (~)
    /// Reference (&)
    /// Dereference (*)
    /// Pre-increment (++)
    /// Pre-decrement (--)
    /// Post-increment (++)
    /// Post-decrement (--)
impl UnaryOperator {
    /// Get the string representation of the operator
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }

    /// Parse operator from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
        }
    }

    /// Check if operator is prefix
    pub fn is_prefix(&self) -> bool {
            UnaryOperator::Plus | UnaryOperator::Minus | UnaryOperator::LogicalNot |
            UnaryOperator::BitwiseNot | UnaryOperator::Reference | UnaryOperator::Dereference |
            UnaryOperator::PreIncrement | UnaryOperator::PreDecrement
        )
    /// Check if operator is postfix
    pub fn is_postfix(&self) -> bool {
        matches!(self, UnaryOperator::PostIncrement | UnaryOperator::PostDecrement)
    }
}

/// Helper functions for creating operator expressions
pub fn binary_expr(left: Box<dyn Expression>, op: &str, right: Box<dyn Expression>) -> BinaryExpression {
    BinaryExpression::new(op.to_string(), left, op.to_string(), right)
pub fn unary_expr(op: &str, operand: Box<dyn Expression>) -> UnaryExpression {
    UnaryExpression::new(op.to_string(), op.to_string(), operand)
pub fn assign_expr(name: Box<dyn Expression>, value: Box<dyn Expression>) -> AssignmentExpression {
    AssignmentExpression::new("=".to_string(), name, value)
}
