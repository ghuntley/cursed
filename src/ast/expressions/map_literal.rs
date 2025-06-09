//! AST node for map literal expressions in the CURSED language.
//!
//! This module defines the AST representation for map literal expressions,
//! which create map values directly in code with specified key and value types
//! and a list of initial key-value pairs.
//!
//! Map literals have the syntax: `tea[KeyType]ValueType{key1: value1, key2: value2, ...}`
//! where KeyType and ValueType are the key and value types respectively.

use crate::ast::{Expression, Node};
use crate::lexer::Token;
use std::any::Any;

/// Represents a map literal expression in the AST.
///
/// A map literal creates a map value directly in code by specifying
/// the key and value types and listing initial key-value pairs inside curly braces.
/// The map type is indicated by the `tea[KeyType]ValueType` prefix before the braces.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// tea[tea]normie{"name": 42, "age": 25}     // Map of strings to integers
/// tea[normie]tea{1: "one", 2: "two"}       // Map of integers to strings
/// tea[tea]tea{}                             // Empty map of strings to strings
/// ```
///
/// The AST would have a `MapLiteral` with:
/// - key_type: an expression representing the key type (e.g., Identifier("tea"))
/// - value_type: an expression representing the value type (e.g., Identifier("normie"))
/// - pairs: a vector of (key_expr, value_expr) tuples for each initial pair
pub struct MapLiteral {
    /// The token that starts the map literal (usually the 'tea' token)
    pub token: Token,
    /// The type of keys in the map (e.g., "tea", "normie", etc.)
    pub key_type: Box<dyn Expression>,
    /// The type of values in the map (e.g., "normie", "tea", etc.)
    pub value_type: Box<dyn Expression>,
    /// The list of key-value pairs that will be the initial contents of the map
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl std::fmt::Debug for MapLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapLiteral")
            .field("token", &self.token.token_literal())
            .field("key_type", &self.key_type.string())
            .field("value_type", &self.value_type.string())
            .field("pairs", &self.pairs.iter().map(|(k, v)| format!("{}: {}", k.string(), v.string())).collect::<Vec<String>>())
            .finish()
    }
}

impl Node for MapLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let pairs = self
            .pairs
            .iter()
            .map(|(k, v)| format!("{}: {}", k.string(), v.string()))
            .collect::<Vec<String>>()
            .join(", ");

        format!("tea[{}]{}{{{}}}", 
            self.key_type.string(), 
            self.value_type.string(), 
            pairs)
    }
}

impl Expression for MapLiteral {
    fn expression_node(&self) {}
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(MapLiteral {
            token: self.token.clone(),
            key_type: self.key_type.clone_box(),
            value_type: self.value_type.clone_box(),
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
                .collect(),
        })
    }
}

impl MapLiteral {
    /// Creates a new map literal with the given key type, value type, and pairs
    ///
    /// # Arguments
    ///
    /// * `token` - The token that starts the map literal
    /// * `key_type` - An expression representing the type of keys
    /// * `value_type` - An expression representing the type of values
    /// * `pairs` - A vector of (key_expr, value_expr) tuples for the initial pairs
    ///
    /// # Returns
    ///
    /// A new `MapLiteral` instance
    pub fn new(
        token: Token,
        key_type: Box<dyn Expression>,
        value_type: Box<dyn Expression>,
        pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
    ) -> Self {
        MapLiteral {
            token,
            key_type,
            value_type,
            pairs,
        }
    }
    
    /// Returns true if this is an empty map literal (no pairs)
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
    
    /// Returns the number of key-value pairs in the map literal
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    
    /// Returns the key type expression
    pub fn get_key_type(&self) -> &dyn Expression {
        self.key_type.as_ref()
    }
    
    /// Returns the value type expression
    pub fn get_value_type(&self) -> &dyn Expression {
        self.value_type.as_ref()
    }
    
    /// Returns an iterator over the key-value pairs
    pub fn pairs_iter(&self) -> impl Iterator<Item = (&dyn Expression, &dyn Expression)> {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}
