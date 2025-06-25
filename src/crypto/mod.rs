// Cryptography module for CURSED
pub mod ed25519;
pub mod types;

// Re-export key types
pub use ed25519::{Ed25519Keypair, Ed25519PublicKey, Ed25519SecretKey, Ed25519Signature};
pub use types::{CryptoPlatform, PolynomialCommitment, CryptoError};
