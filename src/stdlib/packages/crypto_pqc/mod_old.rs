/// fr fr Post-quantum cryptography
pub mod stub;
pub mod kyber;
pub mod sphincs;
pub mod falcon;

pub use stub::*;
pub use kyber::*;
pub use sphincs::*;
pub use falcon::*;

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// fr fr Initialize the crypto_pqc package
pub fn init_crypto_pqc() -> AdvancedCryptoResult<()> {
    // Initialize SPHINCS+ hash-based signatures
    sphincs::init_sphincs()?;
    
    // Initialize Falcon lattice-based signatures
    falcon::init_falcon()?;
    
    println!("🔐 crypto_pqc package initialized - post-quantum crypto ready bestie!");
    println!("✨ Available algorithms:");
    println!("   - Kyber (lattice-based key exchange)");
    println!("   - SPHINCS+ (hash-based signatures)");
    println!("   - Falcon (lattice-based signatures)");
    Ok(())
}
