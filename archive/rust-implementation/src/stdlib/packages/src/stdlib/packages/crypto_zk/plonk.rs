use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct PlonkProver {
    pub proving_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PlonkVerifier {
    pub verification_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Plonk {
    pub proof: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PlonkGate {
    pub gate_type: String,
}

#[derive(Debug, Clone)]
pub struct PlonkPolynomial {
    pub coefficients: Vec<u8>,
}
