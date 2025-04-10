use std::any::Any;
use crate::ast::{Node, Expression};

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
    
    fn node_type(&self) -> &str {
        "IntegerLiteral"
    }
    
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
    
    fn node_type(&self) -> &str {
        "BooleanLiteral"
    }
    
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