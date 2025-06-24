use crate::error::{CursedError, Error};
use base64::{Engine as _, engine::general_purpose};
use tracing::{debug, info, warn, error, instrument};

/// Base64 encoding utilities for cryptographic operations
pub struct Base64Encoder;

impl Base64Encoder {
    /// Encode bytes to standard base64
    #[instrument(skip(data))]
    pub fn encode_standard(data: &[u8]) -> String {
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        debug!(input_length = data.len(), output_length = encoded.len(), "Encoded to standard base64");
        encoded
    }

    /// Decode standard base64
    #[instrument(skip(encoded))]
    pub fn decode_standard(encoded: &str) -> Result<(), Error> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(encoded.trim())
            .map_err(|e| CursedError::new("base64_error", &format!("Invalid base64: {}", e)))?;
        debug!(input_length = encoded.len(), output_length = decoded.len(), "Decoded from standard base64");
        Ok(decoded)
    }

    /// Encode bytes to URL-safe base64 (no padding)
    #[instrument(skip(data))]
    pub fn encode_url_safe(data: &[u8]) -> String {
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data);
        debug!(input_length = data.len(), output_length = encoded.len(), "Encoded to URL-safe base64");
        encoded
    }

    /// Decode URL-safe base64
    #[instrument(skip(encoded))]
    pub fn decode_url_safe(encoded: &str) -> Result<(), Error> {
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded.trim())
            .map_err(|e| CursedError::new("base64_error", &format!("Invalid URL-safe base64: {}", e)))?;
        debug!(input_length = encoded.len(), output_length = decoded.len(), "Decoded from URL-safe base64");
        Ok(decoded)
    }

    /// Encode with custom alphabet and padding
    #[instrument(skip(data, alphabet))]
    pub fn encode_custom(data: &[u8], alphabet: &str, padding: Option<char>) -> Result<(), Error> {
        if alphabet.len() != 64 {
            return Err(CursedError::new("base64_error", "Custom alphabet must be exactly 64 characters"));
        }

        let alphabet_bytes = alphabet.as_bytes();
        let mut result = String::new();
        let mut i = 0;

        // Process 3-byte groups
        while i + 2 < data.len() {
            let chunk = [data[i], data[i + 1], data[i + 2]];
            let n = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32);
            
            result.push(alphabet_bytes[(n >> 18) as usize] as char);
            result.push(alphabet_bytes[((n >> 12) & 63) as usize] as char);
            result.push(alphabet_bytes[((n >> 6) & 63) as usize] as char);
            result.push(alphabet_bytes[(n & 63) as usize] as char);
            
            i += 3;
        }

        // Handle remaining bytes
        match data.len() % 3 {
            1 => {
                let n = (data[i] as u32) << 16;
                result.push(alphabet_bytes[(n >> 18) as usize] as char);
                result.push(alphabet_bytes[((n >> 12) & 63) as usize] as char);
                if let Some(pad) = padding {
                    result.push(pad);
                    result.push(pad);
                }
            }
            2 => {
                let n = (data[i] as u32) << 16 | (data[i + 1] as u32) << 8;
                result.push(alphabet_bytes[(n >> 18) as usize] as char);
                result.push(alphabet_bytes[((n >> 12) & 63) as usize] as char);
                result.push(alphabet_bytes[((n >> 6) & 63) as usize] as char);
                if let Some(pad) = padding {
                    result.push(pad);
                }
            }
            _ => {}
        }

        debug!(input_length = data.len(), output_length = result.len(), "Encoded with custom base64");
        Ok(result)
    }
}

/// Hexadecimal encoding utilities
pub struct HexEncoder;

impl HexEncoder {
    /// Encode bytes to lowercase hex
    #[instrument(skip(data))]
    pub fn encode_lower(data: &[u8]) -> String {
        let encoded = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        debug!(input_length = data.len(), output_length = encoded.len(), "Encoded to lowercase hex");
        encoded
    }

    /// Encode bytes to uppercase hex
    #[instrument(skip(data))]
    pub fn encode_upper(data: &[u8]) -> String {
        let encoded = data.iter().map(|b| format!("{:02X}", b)).collect::<String>();
        debug!(input_length = data.len(), output_length = encoded.len(), "Encoded to uppercase hex");
        encoded
    }

    /// Decode hex string to bytes
    #[instrument(skip(hex))]
    pub fn decode(hex: &str) -> Result<(), Error> {
        let clean_hex = hex.trim().replace(' ', "");
        
        if clean_hex.len() % 2 != 0 {
            return Err(CursedError::new("hex_error", "Hex string length must be even"));
        }

        let mut result = Vec::with_capacity(clean_hex.len() / 2);
        for chunk in clean_hex.as_bytes().chunks(2) {
            let hex_str = std::str::from_utf8(chunk)
                .map_err(|e| CursedError::new("hex_error", &format!("Invalid UTF-8 in hex: {}", e)))?;
            let byte = u8::from_str_radix(hex_str, 16)
                .map_err(|e| CursedError::new("hex_error", &format!("Invalid hex digit: {}", e)))?;
            result.push(byte);
        }

        debug!(input_length = hex.len(), output_length = result.len(), "Decoded from hex");
        Ok(result)
    }

    /// Encode with custom formatting (spaces, colons, etc.)
    #[instrument(skip(data))]
    pub fn encode_formatted(data: &[u8], separator: &str, uppercase: bool) -> String {
        let hex_chars = if uppercase { "0123456789ABCDEF" } else { "0123456789abcdef" };
        let mut result = String::with_capacity(data.len() * (2 + separator.len()));
        
        for (i, &byte) in data.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            result.push(hex_chars.chars().nth((byte >> 4) as usize).unwrap());
            result.push(hex_chars.chars().nth((byte & 0x0f) as usize).unwrap());
        }
        
        debug!(input_length = data.len(), output_length = result.len(), separator, uppercase, "Encoded formatted hex");
        result
    }

    /// Decode formatted hex (ignores separators)
    #[instrument(skip(formatted_hex))]
    pub fn decode_formatted(formatted_hex: &str) -> Result<(), Error> {
        let clean_hex: String = formatted_hex.chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect();
        Self::decode(&clean_hex)
    }
}

/// Base32 encoding for TOTP and other applications
pub struct Base32Encoder;

impl Base32Encoder {
    const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    /// Encode bytes to base32
    #[instrument(skip(data))]
    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        let mut buffer = 0u64;
        let mut bits = 0;

        for &byte in data {
            buffer = (buffer << 8) | (byte as u64);
            bits += 8;

            while bits >= 5 {
                let index = ((buffer >> (bits - 5)) & 0x1f) as usize;
                result.push(Self::ALPHABET.chars().nth(index).unwrap());
                bits -= 5;
            }
        }

        // Handle remaining bits
        if bits > 0 {
            let index = ((buffer << (5 - bits)) & 0x1f) as usize;
            result.push(Self::ALPHABET.chars().nth(index).unwrap());
        }

        // Add padding
        while result.len() % 8 != 0 {
            result.push('=');
        }

        debug!(input_length = data.len(), output_length = result.len(), "Encoded to base32");
        result
    }

    /// Decode base32 to bytes
    #[instrument(skip(encoded))]
    pub fn decode(encoded: &str) -> Result<(), Error> {
        let clean = encoded.trim().replace('=', "").to_uppercase();
        let mut result = Vec::new();
        let mut buffer = 0u64;
        let mut bits = 0;

        for ch in clean.chars() {
            let value = Self::ALPHABET.find(ch)
                .ok_or_else(|| CursedError::new("base32_error", &format!("Invalid base32 character: {}", ch)))?;
            
            buffer = (buffer << 5) | (value as u64);
            bits += 5;

            if bits >= 8 {
                result.push(((buffer >> (bits - 8)) & 0xff) as u8);
                bits -= 8;
            }
        }

        debug!(input_length = encoded.len(), output_length = result.len(), "Decoded from base32");
        Ok(result)
    }

    /// Encode without padding
    #[instrument(skip(data))]
    pub fn encode_no_padding(data: &[u8]) -> String {
        Self::encode(data).trim_end_matches('=').to_string()
    }

    /// Encode with custom alphabet
    #[instrument(skip(data, alphabet))]
    pub fn encode_custom(data: &[u8], alphabet: &str) -> Result<(), Error> {
        if alphabet.len() != 32 {
            return Err(CursedError::new("base32_error", "Custom alphabet must be exactly 32 characters"));
        }

        let mut result = String::new();
        let mut buffer = 0u64;
        let mut bits = 0;

        for &byte in data {
            buffer = (buffer << 8) | (byte as u64);
            bits += 8;

            while bits >= 5 {
                let index = ((buffer >> (bits - 5)) & 0x1f) as usize;
                result.push(alphabet.chars().nth(index).unwrap());
                bits -= 5;
            }
        }

        if bits > 0 {
            let index = ((buffer << (5 - bits)) & 0x1f) as usize;
            result.push(alphabet.chars().nth(index).unwrap());
        }

        debug!(input_length = data.len(), output_length = result.len(), "Encoded with custom base32");
        Ok(result)
    }
}

/// Basic ASN.1 parsing utilities for cryptographic data
pub struct Asn1Parser;

impl Asn1Parser {
    /// Parse ASN.1 tag and length
    #[instrument(skip(data))]
    pub fn parse_tag_length(data: &[u8]) -> Result<(), Error> {
        if data.is_empty() {
            return Err(CursedError::new("asn1_error", "Empty ASN.1 data"));
        }

        let tag = data[0];
        let mut offset = 1;

        if offset >= data.len() {
            return Err(CursedError::new("asn1_error", "Incomplete ASN.1 length"));
        }

        let length = if data[offset] & 0x80 == 0 {
            // Short form
            let len = data[offset] as usize;
            offset += 1;
            len
        } else {
            // Long form
            let length_octets = (data[offset] & 0x7f) as usize;
            if length_octets == 0 {
                return Err(CursedError::new("asn1_error", "Indefinite length not supported"));
            }
            if length_octets > 4 {
                return Err(CursedError::new("asn1_error", "Length too large"));
            }
            
            offset += 1;
            if offset + length_octets > data.len() {
                return Err(CursedError::new("asn1_error", "Incomplete ASN.1 length octets"));
            }

            let mut length = 0usize;
            for i in 0..length_octets {
                length = (length << 8) | (data[offset + i] as usize);
            }
            offset += length_octets;
            length
        };

        if offset + length > data.len() {
            return Err(CursedError::new("asn1_error", "ASN.1 content exceeds data length"));
        }

        let content = &data[offset..offset + length];
        let element = Asn1Element {
            tag,
            length,
            content: content.to_vec(),
            total_length: offset + length,
        };

        debug!(tag, length, total_length = element.total_length, "Parsed ASN.1 element");
        Ok(element)
    }

    /// Parse ASN.1 sequence
    #[instrument(skip(data))]
    pub fn parse_sequence(data: &[u8]) -> Result<(), Error> {
        let sequence_element = Self::parse_tag_length(data)?;
        
        if sequence_element.tag != 0x30 {
            return Err(CursedError::new("asn1_error", "Expected SEQUENCE tag (0x30)"));
        }

        let mut elements = Vec::new();
        let mut offset = 0;
        let content = &sequence_element.content;

        while offset < content.len() {
            let element = Self::parse_tag_length(&content[offset..])?;
            offset += element.total_length;
            elements.push(element);
        }

        debug!(element_count = elements.len(), "Parsed ASN.1 sequence");
        Ok(elements)
    }

    /// Parse ASN.1 integer
    #[instrument(skip(data))]
    pub fn parse_integer(data: &[u8]) -> Result<(), Error> {
        let element = Self::parse_tag_length(data)?;
        
        if element.tag != 0x02 {
            return Err(CursedError::new("asn1_error", "Expected INTEGER tag (0x02)"));
        }

        if element.content.is_empty() {
            return Err(CursedError::new("asn1_error", "Empty integer content"));
        }

        debug!(integer_length = element.content.len(), "Parsed ASN.1 integer");
        Ok(element.content)
    }

    /// Parse ASN.1 octet string
    #[instrument(skip(data))]
    pub fn parse_octet_string(data: &[u8]) -> Result<(), Error> {
        let element = Self::parse_tag_length(data)?;
        
        if element.tag != 0x04 {
            return Err(CursedError::new("asn1_error", "Expected OCTET STRING tag (0x04)"));
        }

        debug!(octet_string_length = element.content.len(), "Parsed ASN.1 octet string");
        Ok(element.content)
    }

    /// Parse ASN.1 bit string
    #[instrument(skip(data))]
    pub fn parse_bit_string(data: &[u8]) -> Result<(), Error> {
        let element = Self::parse_tag_length(data)?;
        
        if element.tag != 0x03 {
            return Err(CursedError::new("asn1_error", "Expected BIT STRING tag (0x03)"));
        }

        if element.content.is_empty() {
            return Err(CursedError::new("asn1_error", "Empty bit string content"));
        }

        let unused_bits = element.content[0];
        if unused_bits > 7 {
            return Err(CursedError::new("asn1_error", "Invalid unused bits count"));
        }

        let bit_string = Asn1BitString {
            unused_bits,
            data: element.content[1..].to_vec(),
        };

        debug!(unused_bits, data_length = bit_string.data.len(), "Parsed ASN.1 bit string");
        Ok(bit_string)
    }

    /// Create ASN.1 DER encoding for integer
    #[instrument(skip(value))]
    pub fn encode_integer(value: &[u8]) -> Vec<u8> {
        let mut result = Vec::from([0x02]); // INTEGER tag
        
        // Remove leading zeros but keep at least one byte
        let mut trimmed = value;
        while trimmed.len() > 1 && trimmed[0] == 0 {
            trimmed = &trimmed[1..];
        }
        
        // Add zero byte if first bit is set (to keep it positive)
        let needs_zero_byte = !trimmed.is_empty() && trimmed[0] & 0x80 != 0;
        let content_length = trimmed.len() + if needs_zero_byte { 1 } else { 0 };
        
        // Encode length
        if content_length < 0x80 {
            result.push(content_length as u8);
        } else {
            let length_bytes = content_length.to_be_bytes();
            let significant_bytes = length_bytes.iter().position(|&b| b != 0).unwrap_or(7);
            result.push(0x80 | (8 - significant_bytes) as u8);
            result.extend_from_slice(&length_bytes[significant_bytes..]);
        }
        
        // Add content
        if needs_zero_byte {
            result.push(0);
        }
        result.extend_from_slice(trimmed);
        
        debug!(input_length = value.len(), output_length = result.len(), "Encoded ASN.1 integer");
        result
    }
}

/// ASN.1 element structure
#[derive(Debug, Clone)]
pub struct Asn1Element {
    pub tag: u8,
    pub length: usize,
    pub content: Vec<u8>,
    pub total_length: usize,
}

/// ASN.1 bit string
#[derive(Debug, Clone)]
pub struct Asn1BitString {
    pub unused_bits: u8,
    pub data: Vec<u8>,
}

/// URL encoding utilities for crypto parameters
pub struct UrlEncoder;

impl UrlEncoder {
    /// URL encode bytes
    #[instrument(skip(data))]
    pub fn encode(data: &[u8]) -> String {
        let mut result = String::new();
        for &byte in data {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    result.push(byte as char);
                }
                _ => {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        debug!(input_length = data.len(), output_length = result.len(), "URL encoded data");
        result
    }

    /// URL decode string
    #[instrument(skip(encoded))]
    pub fn decode(encoded: &str) -> Result<(), Error> {
        let mut result = Vec::new();
        let mut chars = encoded.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '%' => {
                    let hex1 = chars.next()
                        .ok_or_else(|| CursedError::new("url_error", "Incomplete percent encoding"))?;
                    let hex2 = chars.next()
                        .ok_or_else(|| CursedError::new("url_error", "Incomplete percent encoding"))?;
                    
                    let hex_str = format!("{}{}", hex1, hex2);
                    let byte = u8::from_str_radix(&hex_str, 16)
                        .map_err(|e| CursedError::new("url_error", &format!("Invalid hex in URL encoding: {}", e)))?;
                    result.push(byte);
                }
                '+' => {
                    result.push(b' '); // '+' represents space in form data
                }
                _ => {
                    result.push(ch as u8);
                }
            }
        }
        
        debug!(input_length = encoded.len(), output_length = result.len(), "URL decoded data");
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::error::Error;

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        
        let encoded = Base64Encoder::encode_standard(data);
        let decoded = Base64Encoder::decode_standard(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
        
        let url_encoded = Base64Encoder::encode_url_safe(data);
        let url_decoded = Base64Encoder::decode_url_safe(&url_encoded).unwrap();
        assert_eq!(data, url_decoded.as_slice());
    }

    #[test]
    fn test_hex_encoding() {
        let data = Vec::from([0x00, 0x01, 0x02, 0xfe, 0xff]);
        
        let encoded = HexEncoder::encode_lower(&data);
        assert_eq!(encoded, "000102feff");
        
        let decoded = HexEncoder::decode(&encoded).unwrap();
        assert_eq!(data, decoded);
        
        let formatted = HexEncoder::encode_formatted(&data, ":", true);
        assert_eq!(formatted, "00:01:02:FE:FF");
        
        let formatted_decoded = HexEncoder::decode_formatted(&formatted).unwrap();
        assert_eq!(data, formatted_decoded);
    }

    #[test]
    fn test_base32_encoding() {
        let data = b"Hello";
        
        let encoded = Base32Encoder::encode(data);
        let decoded = Base32Encoder::decode(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
        
        let no_padding = Base32Encoder::encode_no_padding(data);
        assert!(!no_padding.contains('='));
    }

    #[test]
    fn test_asn1_parsing() {
        // Simple INTEGER: 0x02 0x01 0x05 (integer value 5)
        let int_data = Vec::from([0x02, 0x01, 0x05]);
        let parsed_int = Asn1Parser::parse_integer(&int_data).unwrap();
        assert_eq!(parsed_int, Vec::from([0x05]));
        
        // Simple OCTET STRING: 0x04 0x05 "hello"
        let octet_data = Vec::from([0x04, 0x05, b'h', b'e', b'l', b'l', b'o']);
        let parsed_octet = Asn1Parser::parse_octet_string(&octet_data).unwrap();
        assert_eq!(parsed_octet, b"hello");
    }

    #[test]
    fn test_url_encoding() {
        let data = b"hello world!@#$%";
        
        let encoded = UrlEncoder::encode(data);
        let decoded = UrlEncoder::decode(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
        
        // Test specific encodings
        assert!(encoded.contains("%20") || encoded.contains("+")); // space
        assert!(encoded.contains("%21")); // !
        assert!(encoded.contains("%40")); // @
    }

    #[test]
    fn test_encoding_round_trips() {
        let test_data = vec![
            Vec::from([]),
            Vec::from([0]),
            Vec::from([255]),
            b"Simple ASCII text".to_vec(),
            Vec::from([0x00, 0x01, 0x02, 0x7f, 0x80, 0xfe, 0xff]),
            (0..=255).collect::<Vec<u8>>(),
        ];
        
        for data in test_data {
            // Base64 round trips
            let b64 = Base64Encoder::encode_standard(&data);
            assert_eq!(data, Base64Encoder::decode_standard(&b64).unwrap());
            
            let b64_url = Base64Encoder::encode_url_safe(&data);
            assert_eq!(data, Base64Encoder::decode_url_safe(&b64_url).unwrap());
            
            // Hex round trips
            let hex = HexEncoder::encode_lower(&data);
            assert_eq!(data, HexEncoder::decode(&hex).unwrap());
            
            // Base32 round trips (if not empty)
            if !data.is_empty() {
                let b32 = Base32Encoder::encode(&data);
                assert_eq!(data, Base32Encoder::decode(&b32).unwrap());
            }
            
            // URL encoding round trips
            let url = UrlEncoder::encode(&data);
            assert_eq!(data, UrlEncoder::decode(&url).unwrap());
        }
    }
}
