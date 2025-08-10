use crate::error::{Result, CursedError};


#[derive(Debug, Clone)]
pub struct FieldElement {
    pub value: Vec<u8>,
}

pub trait FieldArithmetic {
    fn add(&self, other: &Self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn inverse(&self) -> Option<Self>;
}

impl FieldArithmetic for FieldElement {
    fn add(&self, other: &Self) -> Self {
        Self { value: self.value.clone() }
    }
    
    fn multiply(&self, other: &Self) -> Self {
        Self { value: self.value.clone() }
    }
    
    fn inverse(&self) -> Option<Self> {
        Some(Self { value: self.value.clone() })
    }
}
