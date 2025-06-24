use crate::error::Error;
/// fr fr Asymmetric algorithms - re-export from key_generator
/// 
/// This module provides the algorithm enumeration and related functionality
/// for asymmetric cryptography operations. The actual implementation is in
/// the key_generator module to avoid circular dependencies.

pub use super::key_generator::{AsymmetricAlgorithm, GeneratedKeyPair, KeyGenerator, KeyGeneratorError};

// Re-export for backward compatibility
pub use AsymmetricAlgorithm::{Ed25519, Rsa2048, EcdsaP256};
