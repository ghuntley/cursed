// Generic Signature Scheme Interface
// 
// Common trait and types for digital signature schemes in CURSED crypto.

/// Generic signature scheme trait
pub trait SignatureScheme {
    fn sign(&self, message: &[u8]) -> Vec<u8>;
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}
