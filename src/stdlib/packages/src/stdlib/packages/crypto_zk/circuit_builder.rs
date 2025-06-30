use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct CircuitBuilder {
    pub gates: Vec<Gate>,
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub inputs: Vec<Wire>,
}

#[derive(Debug, Clone)]
pub struct Wire {
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct R1CSConstraint {
    pub a: Vec<u8>,
    pub b: Vec<u8>,
    pub c: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Circuits;
