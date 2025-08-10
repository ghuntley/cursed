// PKI and cryptography types for CURSED
// Placeholder for PKI-related type definitions

pub struct Certificate;
pub struct PrivateKey;
pub struct PublicKey;

// Re-export common types
pub use Certificate as X509Certificate;
pub use PrivateKey as RsaPrivateKey;
pub use PublicKey as RsaPublicKey;
