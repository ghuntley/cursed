/// Minimal AST for CURSED - just enough to parse basic programs

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Facts(String, Expression), // facts x = expression
    Slay(String, Vec<String>, Vec<Statement>), // slay function(args) { body }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Boolean(bool),
    String(String),
    FunctionCall(String, Vec<Expression>),
}
