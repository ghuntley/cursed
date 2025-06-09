//! Signature Format Utilities
//! 
//! Signature format handling for CURSED crypto.

/// Signature format utilities
pub struct SignatureFormat;

impl SignatureFormat {
    pub fn encode(signature: &[u8]) -> String {
        // Placeholder implementation - using hex encoding instead of base64
        hex::encode(signature)
    }
    
    pub fn decode(encoded: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Placeholder implementation - using hex decoding
        Ok(hex::decode(encoded)?)
    }
}
