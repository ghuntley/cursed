/// fr fr Advanced hash functions
pub mod algorithms;
pub mod hmac;
pub mod hash_functions;

pub use algorithms::*;
pub use hmac::*;
pub use hash_functions::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// fr fr Initialize the crypto_hash_advanced package
pub fn init_crypto_hash_advanced() -> AdvancedCryptoResult<()> {
    println!("🔐 crypto_hash_advanced package initialized - advanced hashing ready bestie!");
    Ok(())
}
