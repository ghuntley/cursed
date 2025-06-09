/// Additional literal expressions for CURSED

use crate::ast::traits::{Node, Expression};
use crate::lexer::Token;
use std::any::Any;

/// Array literal [1, 2, 3]
#[derive(Debug)]
pub struct ArrayLiteral {
    pub token: String,
    pub elements: Vec<Box<dyn Expression>>,
}

impl ArrayLiteral {
    pub fn new(token: String, elements: Vec<Box<dyn Expression>>) -> Self {
        Self { token, elements }
    }
}

impl Node for ArrayLiteral {
    fn string(&self) -> String {
        let elements: Vec<String> = self.elements.iter()
            .map(|e| e.string())
            .collect();
        format!("[{}]", elements.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for ArrayLiteral {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        }
    }
}

impl Expression for ArrayLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Hash literal {key: value, ...}
#[derive(Debug)]
pub struct HashLiteral {
    pub token: String,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl HashLiteral {
    pub fn new(token: String, pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>) -> Self {
        Self { token, pairs }
    }
}

impl Node for HashLiteral {
    fn string(&self) -> String {
        let pairs: Vec<String> = self.pairs.iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect();
        format!("{{{}}}", pairs.join(", "))
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Clone for HashLiteral {
    fn clone(&self) -> Self {
        Self {
            token: self.token.clone(),
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
                .collect(),
        }
    }
}

impl Expression for HashLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

/// Nil literal (no_cap)
#[derive(Debug, Clone)]
pub struct NilLiteral {
    pub token: String,
}

impl NilLiteral {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Node for NilLiteral {
    fn string(&self) -> String {
        "no_cap".to_string()
    }

    fn token_literal(&self) -> String {
        self.token.clone()
    }
}

impl Expression for NilLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}
