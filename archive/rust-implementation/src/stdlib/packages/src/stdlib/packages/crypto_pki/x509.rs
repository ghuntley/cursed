
use crate::error::{Result, CursedError};

pub trait X509Operations {
    fn parse(&self, data: &[u8]) -> Result<()>;
    fn validate(&self) -> Result<bool>;
}

#[derive(Debug, Clone)]
pub struct X509 {
    pub data: Vec<u8>,
}

impl X509Operations for X509 {
    fn parse(&self, data: &[u8]) -> Result<()> {
        Ok(())
    }
    
    fn validate(&self) -> Result<bool> {
        Ok(true)
    }
}
