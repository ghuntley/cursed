use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct Groth16Prover {
    pub proving_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Groth16Verifier {
    pub verification_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Groth16 {
    pub proof: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct G1Point {
    pub x: Vec<u8>,
    pub y: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct G2Point {
    pub x: [Vec<u8>; 2],
    pub y: [Vec<u8>; 2],
}
