//! PEM/DER Encoding and Decoding
//! 
//! Handle PEM and DER format encoding and decoding for certificates and keys.

use crate::stdlib::packages::crypto_pki::error::{PkiError, PkiResult};

/// PEM/DER codec for encoding and decoding certificates and keys
#[derive(Debug)]
pub struct PemDerCodec;

/// Encoding format specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncodingFormat {
    /// PEM format (Base64 with headers)
    Pem,
    /// DER format (Binary ASN.1)
    Der,
}

impl PemDerCodec {
    /// Encode data to PEM format
    pub fn encode_pem(data: &[u8], label: &str) -> String {
        let base64_data = base64_encode(data);
        let mut pem = format!("-----BEGIN {}-----\n", label);
        
        // Split base64 data into lines of 64 characters
        for chunk in base64_data.as_bytes().chunks(64) {
            pem.push_str(std::str::from_utf8(chunk).unwrap_or(""));
            pem.push('\n');
        }
        
        pem.push_str(&format!("-----END {}-----\n", label));
        pem
    }
    
    /// Decode PEM format to DER data
    pub fn decode_pem(pem_data: &str) -> PkiResult<Vec<u8>> {
        // Find BEGIN and END markers
        let lines: Vec<&str> = pem_data.lines().collect();
        let mut start_line = None;
        let mut end_line = None;
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("-----BEGIN ") {
                start_line = Some(i);
            } else if line.starts_with("-----END ") {
                end_line = Some(i);
                break;
            }
        }
        
        let start = start_line.ok_or_else(|| PkiError::encoding_error("Missing BEGIN marker", "PEM"))?;
        let end = end_line.ok_or_else(|| PkiError::encoding_error("Missing END marker", "PEM"))?;
        
        // Extract base64 data
        let base64_lines = &lines[start + 1..end];
        let base64_data = base64_lines.join("");
        
        base64_decode(&base64_data)
    }
    
    /// Detect encoding format
    pub fn detect_format(data: &[u8]) -> EncodingFormat {
        if let Ok(text) = std::str::from_utf8(data) {
            if text.contains("-----BEGIN ") && text.contains("-----END ") {
                return EncodingFormat::Pem;
            }
        }
        EncodingFormat::Der
    }
}

/// Simple base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b0 = buf[0] as u32;
        let b1 = buf[1] as u32;
        let b2 = buf[2] as u32;
        
        let combined = (b0 << 16) | (b1 << 8) | b2;
        
        result.push(BASE64_CHARS[((combined >> 18) & 0x3F) as usize] as char);
        result.push(BASE64_CHARS[((combined >> 12) & 0x3F) as usize] as char);
        result.push(if chunk.len() > 1 { BASE64_CHARS[((combined >> 6) & 0x3F) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { BASE64_CHARS[(combined & 0x3F) as usize] as char } else { '=' });
    }
    
    result
}

/// Simple base64 decoding
fn base64_decode(data: &str) -> PkiResult<Vec<u8>> {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    let mut result = Vec::new();
    let bytes = data.as_bytes();
    
    for chunk in bytes.chunks(4) {
        let mut group = [0u8; 4];
        let mut padding = 0;
        
        for (i, &byte) in chunk.iter().enumerate() {
            if byte == b'=' {
                padding += 1;
                group[i] = 0;
            } else {
                let pos = BASE64_CHARS.iter().position(|&b| b == byte)
                    .ok_or_else(|| PkiError::encoding_error(
                        format!("Invalid base64 character: {}", byte as char),
                        "Base64"
                    ))?;
                group[i] = pos as u8;
            }
        }
        
        let decoded = [
            (group[0] << 2) | (group[1] >> 4),
            (group[1] << 4) | (group[2] >> 2),
            (group[2] << 6) | group[3],
        ];
        
        let bytes_to_add = 3 - padding;
        result.extend_from_slice(&decoded[..bytes_to_add]);
    }
    
    Ok(result)
}
