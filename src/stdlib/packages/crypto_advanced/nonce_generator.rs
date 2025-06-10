/// fr fr Nonce generator stub
use super::errors::*;

#[derive(Debug, Clone)]
pub struct NonceGenerator;

#[derive(Debug, Clone)]
pub struct SecureNonce(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct NonceCounterMode;

#[derive(Debug, Clone)]
pub struct NonceRandomMode;

#[derive(Debug, Clone)]
pub struct NonceError(pub String);

pub const NONCE_UNIQUENESS_GUARANTEE: bool = true;
