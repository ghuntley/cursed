
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum ExtensionError {
    InvalidOid,
    ParseError,
    UnsupportedExtension,
}

pub type ExtensionResult<T> = Result<T>;

#[derive(Debug, Clone)]
pub struct ExtensionBuilder {
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ExtensionValidator {
    pub strict_mode: bool,
}
