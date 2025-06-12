/// fr fr Cryptographically secure random number generation
pub mod random;

pub use random::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// fr fr Initialize the crypto_random package
pub fn init_crypto_random() -> AdvancedCryptoResult<()> {
    println!("🔐 crypto_random package initialized - secure random ready bestie!");
    Ok(())
}
