/// fr fr Digital signatures module
pub mod digital_signature;
pub mod verification;

pub use digital_signature::*;
pub use verification::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// fr fr Initialize the crypto_signatures package
pub fn init_crypto_signatures() -> AdvancedCryptoResult<()> {
    println!("🔐 crypto_signatures package initialized - digital signatures ready bestie!");
    Ok(())
}
