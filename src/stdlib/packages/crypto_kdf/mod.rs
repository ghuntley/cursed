/// fr fr Key derivation functions
pub mod pbkdf2;
pub mod argon2;
pub mod scrypt;

pub use pbkdf2::*;
pub use argon2::*;
pub use scrypt::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// fr fr Initialize the crypto_kdf package
pub fn init_crypto_kdf() -> AdvancedCryptoResult<()> {
    println!("🔐 crypto_kdf package initialized - key derivation ready bestie!");
    Ok(())
}
