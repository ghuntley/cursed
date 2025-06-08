//! Method call expressions for the CURSED language AST.
//!
//! This module defines the AST representation for method call expressions,
//! which represent calling methods on receivers.

use crate::ast::expressions::identifiers::Identifier;
use crate::ast::{Expression, Node};
use std::any::Any;

/// MethodCall represents a method call expression (receiver.method(args))
///
/// In CURSED, method calls are invoked using dot notation, where the left side
/// is the receiver and the right side is the method name followed by arguments.
///
/// # Examples
///
/// ```
/// person.getName()
/// person.setAge(25)
/// myArray.length()
/// ```
pub struct MethodCall {
    pub token: String, // Usually the dot token
    pub receiver: Box<dyn Expression>, // The object the method is called on
    pub method: Identifier, // The method name
    pub arguments: Vec<Box<dyn Expression>>, // Method arguments
}

impl Node for MethodCall {
    fn token_literal(&self) -> String {
        self.token.clone()
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

    fn string(&self) -> String {
        let mut out = String::new();
        
        out.push_str(&self.receiver.string());
        out.push_str(".");
        out.push_str(&self.method.string());
        out.push_str("(");
        
        let args: Vec<String> = self.arguments.iter().map(|arg| arg.string()).collect();
        out.push_str(&args.join(", "));
        
        out.push_str(")");
        
        out
    }
}

impl Expression for MethodCall {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MethodCall {
            token: self.token.clone(),
            receiver: self.receiver.clone_box(),
            method: self.method.clone(),
            arguments: self.arguments.iter().map(|arg| arg.clone_box()).collect(),
        })
    }
}
