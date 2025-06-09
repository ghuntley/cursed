/// Abstract Syntax Tree for CURSED language

pub trait Node {
    fn string(&self) -> String;
}

pub trait Statement: Node + std::fmt::Debug {}
pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn string(&self) -> String {
        self.statements.iter()
            .map(|s| s.string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// Basic AST types that tests expect but are missing

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub token: crate::lexer::Token,
}

impl Node for Identifier {
    fn string(&self) -> String {
        self.name.clone()
    }
}

impl Expression for Identifier {}

pub struct CallExpression {
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
    pub token: crate::lexer::Token,
}

impl Node for CallExpression {
    fn string(&self) -> String {
        format!("{}({})", 
            self.function.string(),
            self.arguments.iter().map(|a| a.string()).collect::<Vec<_>>().join(", ")
        )
    }
}

impl Expression for CallExpression {}

// Add token_literal method to Node trait for backward compatibility
pub trait NodeExt: Node {
    fn token_literal(&self) -> String {
        // Default implementation - just return empty string
        String::new()
    }
}

// Implement for existing types
impl NodeExt for Program {}
impl NodeExt for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl NodeExt for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
