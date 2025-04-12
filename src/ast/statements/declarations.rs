//! Declaration statements for the CURSED AST
//!
//! This module defines statement types that declare various program elements,
//! including constants, variables, packages, and imports. Declarations introduce
//! new named entities into the program's scope.
//!
//! CURSED uses Gen Z slang for these declarations:
//! - `facts` for constants (equivalent to Go's `const`)
//! - `sus` for variables (equivalent to Go's `var`)
//! - `vibe` for packages (equivalent to Go's `package`)
//! - `yeet` for imports (equivalent to Go's `import`)

use crate::ast::expressions::Identifier;
use crate::ast::expressions::StringLiteral;
use crate::ast::{Expression, Node, Statement};
use crate::lexer::token::Token;
use std::any::Any;

/// Represents a constant declaration in CURSED
///
/// A constant declaration introduces a new named constant with an immutable value.
/// In CURSED, constants are declared using the `facts` keyword, which is equivalent
/// to Go's `const` keyword.
///
/// # Example in CURSED
///
/// ```cursed
/// facts PI = 3.14159;  // Declares a constant named PI
/// ```
///
/// # Fields
///
/// * `token` - The token representing the `facts` keyword
/// * `name` - The identifier for the constant
/// * `value` - The expression that provides the constant's value
pub struct FactsStatement {
    pub token: String,
    pub name: Box<dyn Expression>,
    pub value: Box<dyn Expression>,
}

impl Node for FactsStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        format!("facts {} = {}", self.name.string(), self.value.string())
    }
}

impl Statement for FactsStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// LetStatement represents a let statement
pub struct LetStatement {
    pub token: String, // Token::Let
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
    pub type_annotation: Option<Token>, // Type annotation (smol, mid, normie, thicc)
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.clone()
    }

    fn string(&self) -> String {
        let mut out = format!("{} {}", self.token_literal(), self.name.string());

        // Include type annotation if present
        if let Some(type_token) = &self.type_annotation {
            out.push_str(&format!(" {}", type_token.token_literal()));
        }

        out.push_str(" = ");
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
            format!(
                "{} {} {};",
                self.token_literal(),
                alias.string(),
                self.path.string()
            )
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
