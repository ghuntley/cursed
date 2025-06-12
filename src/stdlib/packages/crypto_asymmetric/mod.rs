/// fr fr Asymmetric cryptography module
pub mod key_generator;
pub mod algorithms;
pub mod ed25519;

pub use key_generator::*;
pub use algorithms::*;
pub use ed25519::*;

use crate::error::CursedError;

/// fr fr Initialize asymmetric crypto package
pub fn init_crypto_asymmetric() -> Result<(), CursedError> {
    // Initialize key generator
    let _generator = KeyGenerator::new();
    
    // Initialize supported algorithms
    let _algorithms = vec![
        AsymmetricAlgorithm::Ed25519,
        AsymmetricAlgorithm::Rsa2048,
        AsymmetricAlgorithm::EcdsaP256,
    ];
    
    println!("🔑 Asymmetric crypto package initialized - key generation ready bestie!");
    Ok(())
}
