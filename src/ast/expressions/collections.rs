//! AST nodes for collection expressions in the CURSED language.
//!
//! This module defines the AST representations for collection-related expressions:
//! - Array literals: expressions that create array values directly
//! - Hash/map literals: expressions that create map/dictionary values directly
//! - Index expressions: expressions that access elements of collections by index or key
//!
//! These expressions provide the foundation for working with compound data structures
//! in the language.

use crate::ast::{Expression, Node};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents an array literal expression in the AST.
///
/// An array literal creates an array value directly in code by listing its elements
/// between square brackets. The elements can be any expressions that produce values.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// [1, 2 + 3, "hello"]
/// []
/// ```
///
/// The AST would have an `ArrayLiteral` with elements corresponding to each item inside
/// the brackets, or an empty vector for an empty array.
#[derive(Debug)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Box<dyn Expression>>,
}

impl Node for ArrayLiteral {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }

    fn string(&self) -> String {
        let elements = self
            .elements
            .iter()
            .map(|e| e.string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", elements)
    }
}

impl Expression for ArrayLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(ArrayLiteral {
            token: self.token.clone(),
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        })
    }
}

/// Represents a hash/map literal expression in the AST.
///
/// A hash literal creates a map/dictionary value directly in code by listing
/// key-value pairs inside curly braces. Both keys and values can be any expressions
/// that produce values.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// {"name": "viber", "age": 21, "skills": ["coding", "TikTok"]}
/// {}
/// ```
///
/// The AST would have a `HashLiteral` with pairs corresponding to each key-value pair
/// inside the braces, or an empty vector for an empty map.
#[derive(Debug)]
pub struct HashLiteral {
    pub token: Token,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl Node for HashLiteral {
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

        format!("{{{}}}", pairs)
    }
}

impl Expression for HashLiteral {
    fn expression_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(HashLiteral {
            token: self.token.clone(),
            pairs: self.pairs.iter()
                .map(|(k, v)| (k.clone_box(), v.clone_box()))
                .collect(),
        })
    }
}

/// Represents a typed map literal expression in the AST.
///
/// A map literal creates a map/dictionary value directly in code by specifying
/// the key and value types followed by key-value pairs inside curly braces.
/// This is different from a generic hash literal as it includes explicit type information.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// tea[tea]thicc{"name": 42, "age": 21}
/// tea[normie]tea{1: "one", 2: "two"}
/// ```
///
/// The AST would have a `MapLiteral` with the key type, value type, and pairs
/// corresponding to each key-value pair inside the braces.
#[derive(Debug)]
pub struct MapLiteral {
    pub token: Token,
    pub key_type: Box<dyn Expression>,
    pub value_type: Box<dyn Expression>,
    pub pairs: Vec<(Box<dyn Expression>, Box<dyn Expression>)>,
}

impl MapLiteral {
    /// Creates a new MapLiteral with the given components.
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

    /// Returns the number of key-value pairs in this map literal.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns true if this map literal has no key-value pairs.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Returns a reference to the key type expression.
    pub fn get_key_type(&self) -> &dyn Expression {
        self.key_type.as_ref()
    }

    /// Returns a reference to the value type expression.
    pub fn get_value_type(&self) -> &dyn Expression {
        self.value_type.as_ref()
    }

    /// Returns an iterator over the key-value pairs.
    pub fn pairs_iter(&self) -> impl Iterator<Item = (&dyn Expression, &dyn Expression)> {
        self.pairs.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
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

/// Represents an index access expression in the AST.
///
/// An index expression accesses an element of a collection (array or hash/map)
/// using an index or key enclosed in square brackets. This can be used for
/// both reading and writing elements.
///
/// # Examples
///
/// In CURSED code like:
/// ```
/// arr[0]       // Array access by index
/// map["key"]   // Map access by key
/// matrix[i][j] // Multi-dimensional access (nested IndexExpressions)
/// ```
///
/// The AST would have an `IndexExpression` with:
/// - left: the array or map expression being accessed
/// - index: the index or key expression used for access
#[derive(Debug)]
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
        format!(
            "{} {} {}",
            self.left.string(),
            self.token_literal(),
            self.index.string()
        )
    }
}

impl Expression for IndexExpression {
    fn expression_node(&self) {}

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

    fn is_index_expression(&self) -> bool {
        true
    }

    fn as_index_expression(&self) -> Option<(&dyn Expression, &dyn Expression)> {
        Some((self.left.as_ref(), self.index.as_ref()))
    }
}
