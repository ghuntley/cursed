/// Abstract Syntax Tree for CURSED language

pub trait Node {
    fn string(&self) -> String;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

#[derive(Debug, Clone)]
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
