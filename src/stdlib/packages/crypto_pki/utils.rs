/// PKI Utility Functions

// use crate::stdlib::packages::crypto_pki::types::{PkiResult, PkiError};
use crate::error::CursedError;

/// Utility functions for PKI operations
pub struct PkiUtils;

impl PkiUtils {
    /// Generate random serial number
    pub fn generate_serial_number() -> Vec<u8> {
        use rand::RngCore;
        let mut serial = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut serial);
        serial
    }

    /// Convert DER to PEM format
    pub fn der_to_pem(der_data: &[u8], label: &str) -> String {
        let encoded = base64::encode(der_data);
        let mut pem = format!("-----BEGIN {}-----\n", label);
        
        // Split into 64-character lines
        for chunk in encoded.as_bytes().chunks(64) {
            pem.push_str(&String::from_utf8_lossy(chunk));
            pem.push('\n');
        }
        
        pem.push_str(&format!("-----END {}-----\n", label));
        pem
    }

    /// Convert PEM to DER format
    pub fn pem_to_der(pem_data: &str) -> PkiResult<Vec<u8>> {
        let lines: Vec<&str> = pem_data.lines().collect();
        let mut base64_data = String::new();
        let mut in_content = false;

        for line in lines {
            if line.starts_with("-----BEGIN") {
                in_content = true;
                continue;
            }
            if line.starts_with("-----END") {
                break;
            }
            if in_content {
                base64_data.push_str(line.trim());
            }
        }

        base64::decode(&base64_data)
            .map_err(|e| PkiError::Internal(format!("Invalid base64 in PEM: {}", e)))
    }

    /// Calculate SHA-256 fingerprint
    pub fn calculate_fingerprint(data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Format fingerprint as hex string with colons
    pub fn format_fingerprint(fingerprint: &[u8]) -> String {
        fingerprint
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(":")
    }
}

