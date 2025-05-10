//! Literal expression types for the CURSED AST
//!
//! This module defines the AST nodes for all literal expressions in the CURSED
//! language. Literals represent constant values that appear directly in the source
//! code, such as numbers, strings, booleans, and character values.
//!
//! Each literal type implements both the `Node` and `Expression` traits,
//! allowing them to be used in the AST as expressions.

use crate::ast::{Expression, Node};
use std::any::Any;

/// Represents a string literal in the source code
///
/// A string literal is a sequence of characters enclosed in double quotes,
/// such as "hello world". In CURSED, these are represented by the `tea` type.
///
/// # Fields
///
/// * `token` - The original token from the lexer
/// * `value` - The actual string value (without surrounding quotes)
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(StringLiteral {
            token: self.token.clone(),
            value: self.value.clone(),
        })
    }
}

/// Represents an integer literal in the source code
///
/// An integer literal is a sequence of digits that represents a whole number,
/// such as 42 or -123. In CURSED, these are represented by types like `normie`
/// (32-bit integer) or `thicc` (64-bit integer).
///
/// # Fields
///
/// * `token` - The original token from the lexer
/// * `value` - The parsed integer value
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

    fn node_type(&self) -> &str {
        "IntegerLiteral"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(IntegerLiteral {
            token: self.token.clone(),
            value: self.value,
        })
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(FloatLiteral {
            token: self.token.clone(),
            value: self.value,
        })
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

    fn node_type(&self) -> &str {
        "BooleanLiteral"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(BooleanLiteral {
            token: self.token.clone(),
            value: self.value,
        })
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ByteLiteral {
            token: self.token.clone(),
            value: self.value,
        })
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
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(RuneLiteral {
            token: self.token.clone(),
            value: self.value,
        })
    }
}
