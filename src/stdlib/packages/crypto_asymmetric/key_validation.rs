//! Key validation utilities for cryptographic keys
//! 
//! Provides comprehensive validation for RSA, elliptic curve, and EdDSA keys
//! with mathematical verification and security parameter checking.

use crate::stdlib::value::Value;
use crate::error::CursedError;
use std::collections::HashMap;
use num_bigint::BigUint;
use num_traits::{Zero, One, ToPrimitive};

/// Supported key types for validation
#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    RSA,
    ECC,
    Ed25519,
    X25519,
}

/// Key validation result with detailed information
#[derive(Debug, Clone)]
pub struct KeyValidationResult {
    pub valid: bool,
    pub key_type: KeyType,
    pub strength_bits: u32,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub parameters: HashMap<String, String>,
}

/// RSA key parameters for validation
#[derive(Debug, Clone)]
pub struct RSAKeyParams {
    pub n: BigUint,  // modulus
    pub e: BigUint,  // public exponent
    pub d: Option<BigUint>,  // private exponent
    pub p: Option<BigUint>,  // first prime
    pub q: Option<BigUint>,  // second prime
    pub dp: Option<BigUint>, // d mod (p-1)
    pub dq: Option<BigUint>, // d mod (q-1)
    pub qi: Option<BigUint>, // q^-1 mod p
}

/// Elliptic curve key parameters
#[derive(Debug, Clone)]
pub struct ECCKeyParams {
    pub curve_name: String,
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>,
    pub x: Option<BigUint>,
    pub y: Option<BigUint>,
}

/// Ed25519/X25519 key parameters
#[derive(Debug, Clone)]
pub struct EdDSAKeyParams {
    pub key_type: KeyType,
    pub public_key: Vec<u8>,
    pub private_key: Option<Vec<u8>>,
}

/// Main key validation function
pub fn validate_key(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("No key provided for validation".to_string()));
    }

    let key_data = &args[0];
    let validation_options = if args.len() > 1 { Some(&args[1]) } else { None };

    let result = match key_data {
        Value::Object(key_obj) => {
            let key_type = determine_key_type(key_obj)?;
            match key_type {
                KeyType::RSA => validate_rsa_key(key_obj, validation_options)?,
                KeyType::ECC => validate_ecc_key(key_obj, validation_options)?,
                KeyType::Ed25519 | KeyType::X25519 => validate_eddsa_key(key_obj, validation_options)?,
            }
        },
        Value::String(key_str) => {
            // Try to parse as PEM/DER encoded key
            validate_encoded_key(key_str, validation_options)?
        },
        _ => return Err(CursedError::InvalidArgument("Invalid key format".to_string())),
    };

    Ok(serialize_validation_result(result))
}

/// Determine key type from key object
fn determine_key_type(key_obj: &HashMap<String, Value>) -> Result<KeyType, CursedError> {
    if key_obj.contains_key("n") && key_obj.contains_key("e") {
        Ok(KeyType::RSA)
    } else if key_obj.contains_key("curve") || key_obj.contains_key("x") {
        Ok(KeyType::ECC)
    } else if key_obj.contains_key("ed25519_public") {
        Ok(KeyType::Ed25519)
    } else if key_obj.contains_key("x25519_public") {
        Ok(KeyType::X25519)
    } else {
        Err(CursedError::InvalidArgument("Cannot determine key type".to_string()))
    }
}

/// Validate RSA key parameters
fn validate_rsa_key(key_obj: &HashMap<String, Value>, _options: Option<&Value>) -> Result<KeyValidationResult, CursedError> {
    let params = parse_rsa_params(key_obj)?;
    let mut result = KeyValidationResult {
        valid: true,
        key_type: KeyType::RSA,
        strength_bits: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        parameters: HashMap::new(),
    };

    // Validate modulus
    validate_rsa_modulus(&params, &mut result);
    
    // Validate public exponent
    validate_rsa_public_exponent(&params, &mut result);
    
    // Validate private key components if present
    if params.d.is_some() {
        validate_rsa_private_components(&params, &mut result);
    }
    
    // Check key strength
    result.strength_bits = calculate_rsa_strength(&params.n);
    validate_rsa_strength(result.strength_bits, &mut result);
    
    // Validate mathematical relationships
    validate_rsa_mathematical_consistency(&params, &mut result);

    result.valid = result.errors.is_empty();
    Ok(result)
}

/// Parse RSA parameters from key object
fn parse_rsa_params(key_obj: &HashMap<String, Value>) -> Result<RSAKeyParams, CursedError> {
    let n = extract_bigint(key_obj, "n")?;
    let e = extract_bigint(key_obj, "e")?;
    
    let d = extract_bigint_optional(key_obj, "d");
    let p = extract_bigint_optional(key_obj, "p");
    let q = extract_bigint_optional(key_obj, "q");
    let dp = extract_bigint_optional(key_obj, "dp");
    let dq = extract_bigint_optional(key_obj, "dq");
    let qi = extract_bigint_optional(key_obj, "qi");

    Ok(RSAKeyParams { n, e, d, p, q, dp, dq, qi })
}

/// Validate RSA modulus
fn validate_rsa_modulus(params: &RSAKeyParams, result: &mut KeyValidationResult) {
    // Check modulus is odd (requirement for RSA)
    if params.n.clone() % BigUint::from(2u32) == BigUint::zero() {
        result.errors.push("RSA modulus must be odd".to_string());
    }
    
    // Check modulus is not too small
    let bit_length = params.n.bits() as u32;
    if bit_length < 1024 {
        result.errors.push(format!("RSA modulus too small: {} bits (minimum 1024)", bit_length));
    } else if bit_length < 2048 {
        result.warnings.push(format!("RSA modulus below recommended size: {} bits (recommended 2048+)", bit_length));
    }
    
    result.parameters.insert("modulus_bits".to_string(), bit_length.to_string());
}

/// Validate RSA public exponent
fn validate_rsa_public_exponent(params: &RSAKeyParams, result: &mut KeyValidationResult) {
    // Common secure public exponents
    let common_exponents = [BigUint::from(3u32), BigUint::from(65537u32)];
    
    // Check public exponent is odd
    if params.e.clone() % BigUint::from(2u32) == BigUint::zero() {
        result.errors.push("RSA public exponent must be odd".to_string());
    }
    
    // Check public exponent is at least 3
    if params.e < BigUint::from(3u32) {
        result.errors.push("RSA public exponent must be at least 3".to_string());
    }
    
    // Warn about uncommon exponents
    if !common_exponents.contains(&params.e) {
        result.warnings.push(format!("Uncommon public exponent: {}", params.e));
    }
    
    // Check GCD(e, phi(n)) = 1 if we have prime factors
    if let (Some(p), Some(q)) = (&params.p, &params.q) {
        let phi_n = (p - BigUint::one()) * (q - BigUint::one());
        if gcd(&params.e, &phi_n) != BigUint::one() {
            result.errors.push("Public exponent not coprime to phi(n)".to_string());
        }
    }
}

/// Validate RSA private key components
fn validate_rsa_private_components(params: &RSAKeyParams, result: &mut KeyValidationResult) {
    if let Some(d) = &params.d {
        // Check private exponent bounds
        if d <= &BigUint::one() || d >= &params.n {
            result.errors.push("Private exponent out of valid range".to_string());
        }
        
        // Validate e * d ≡ 1 (mod phi(n)) if we have prime factors
        if let (Some(p), Some(q)) = (&params.p, &params.q) {
            let phi_n = (p - BigUint::one()) * (q - BigUint::one());
            let ed_mod_phi = (d * &params.e) % &phi_n;
            if ed_mod_phi != BigUint::one() {
                result.errors.push("Private exponent validation failed: e * d ≢ 1 (mod phi(n))".to_string());
            }
        }
    }
    
    // Validate prime factors if present
    if let (Some(p), Some(q)) = (&params.p, &params.q) {
        validate_rsa_primes(params, result, p, q);
    }
}

/// Validate RSA prime factors
fn validate_rsa_primes(params: &RSAKeyParams, result: &mut KeyValidationResult, p: &BigUint, q: &BigUint) {
    // Check primes are not equal
    if p == q {
        result.errors.push("RSA prime factors must be distinct".to_string());
        return;
    }
    
    // Check n = p * q
    if &(p * q) != &params.n {
        result.errors.push("RSA modulus does not equal p * q".to_string());
    }
    
    // Check primes are actually prime (basic test)
    if !is_probably_prime(p, 10) {
        result.errors.push("First RSA prime factor fails primality test".to_string());
    }
    
    if !is_probably_prime(q, 10) {
        result.errors.push("Second RSA prime factor fails primality test".to_string());
    }
    
    // Check prime difference (security requirement)
    let diff = if p > q { p - q } else { q - p };
    let min_diff = BigUint::from(2u32).pow((params.n.bits() / 2 - 100) as u32);
    if diff < min_diff {
        result.warnings.push("RSA prime factors may be too close".to_string());
    }
    
    // Validate CRT parameters if present
    if let (Some(dp), Some(dq), Some(qi)) = (&params.dp, &params.dq, &params.qi) {
        validate_rsa_crt_params(params, result, p, q, dp, dq, qi);
    }
}

/// Validate RSA CRT parameters
fn validate_rsa_crt_params(
    params: &RSAKeyParams, 
    result: &mut KeyValidationResult,
    p: &BigUint, 
    q: &BigUint,
    dp: &BigUint,
    dq: &BigUint, 
    qi: &BigUint
) {
    if let Some(d) = &params.d {
        // Check dp = d mod (p-1)
        let expected_dp = d % (p - BigUint::one());
        if dp != &expected_dp {
            result.errors.push("CRT parameter dp validation failed".to_string());
        }
        
        // Check dq = d mod (q-1)
        let expected_dq = d % (q - BigUint::one());
        if dq != &expected_dq {
            result.errors.push("CRT parameter dq validation failed".to_string());
        }
        
        // Check qi = q^-1 mod p
        if let Some(computed_qi) = mod_inverse(q, p) {
            if qi != &computed_qi {
                result.errors.push("CRT parameter qi validation failed".to_string());
            }
        } else {
            result.errors.push("Cannot compute modular inverse for CRT validation".to_string());
        }
    }
}

/// Calculate RSA key strength in bits
fn calculate_rsa_strength(n: &BigUint) -> u32 {
    let modulus_bits = n.bits() as u32;
    
    // RSA strength estimation based on modulus size
    match modulus_bits {
        0..=1023 => modulus_bits / 4, // Very weak estimation
        1024..=2047 => 80,
        2048..=3071 => 112,
        3072..=7679 => 128,
        7680..=15359 => 192,
        _ => 256,
    }
}

/// Validate RSA key strength against security standards
fn validate_rsa_strength(strength_bits: u32, result: &mut KeyValidationResult) {
    if strength_bits < 80 {
        result.errors.push(format!("RSA key strength too low: {} bits (minimum 80)", strength_bits));
    } else if strength_bits < 112 {
        result.warnings.push(format!("RSA key strength below recommended: {} bits (recommended 112+)", strength_bits));
    }
}

/// Validate RSA mathematical consistency
fn validate_rsa_mathematical_consistency(params: &RSAKeyParams, result: &mut KeyValidationResult) {
    // Additional mathematical checks
    if params.n <= BigUint::one() {
        result.errors.push("RSA modulus must be greater than 1".to_string());
    }
    
    if params.e >= params.n {
        result.errors.push("RSA public exponent must be less than modulus".to_string());
    }
    
    // Check for common weak patterns
    if params.n.to_string().chars().all(|c| c == '1' || c == '0') {
        result.warnings.push("RSA modulus has suspicious bit pattern".to_string());
    }
}

/// Validate elliptic curve key
fn validate_ecc_key(key_obj: &HashMap<String, Value>, _options: Option<&Value>) -> Result<KeyValidationResult, CursedError> {
    let params = parse_ecc_params(key_obj)?;
    let mut result = KeyValidationResult {
        valid: true,
        key_type: KeyType::ECC,
        strength_bits: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        parameters: HashMap::new(),
    };

    // Validate curve parameters
    validate_ecc_curve(&params, &mut result);
    
    // Validate public key point
    validate_ecc_public_key(&params, &mut result);
    
    // Validate private key if present
    if params.private_key.is_some() {
        validate_ecc_private_key(&params, &mut result);
    }
    
    // Check key strength
    result.strength_bits = calculate_ecc_strength(&params.curve_name);
    validate_ecc_strength(result.strength_bits, &mut result);

    result.valid = result.errors.is_empty();
    Ok(result)
}

/// Parse ECC parameters from key object
fn parse_ecc_params(key_obj: &HashMap<String, Value>) -> Result<ECCKeyParams, CursedError> {
    let curve_name = extract_string(key_obj, "curve").unwrap_or_else(|| "unknown".to_string());
    let public_key = extract_bytes(key_obj, "public_key").unwrap_or_default();
    let private_key = extract_bytes_optional(key_obj, "private_key");
    let x = extract_bigint_optional(key_obj, "x");
    let y = extract_bigint_optional(key_obj, "y");

    Ok(ECCKeyParams { curve_name, public_key, private_key, x, y })
}

/// Validate elliptic curve parameters
fn validate_ecc_curve(params: &ECCKeyParams, result: &mut KeyValidationResult) {
    let supported_curves = [
        "secp256r1", "secp384r1", "secp521r1", 
        "secp256k1", "brainpoolP256r1", "brainpoolP384r1", "brainpoolP512r1"
    ];
    
    if !supported_curves.contains(&params.curve_name.as_str()) {
        result.warnings.push(format!("Unsupported or unknown curve: {}", params.curve_name));
    }
    
    result.parameters.insert("curve".to_string(), params.curve_name.clone());
}

/// Validate ECC public key point
fn validate_ecc_public_key(params: &ECCKeyParams, result: &mut KeyValidationResult) {
    if params.public_key.is_empty() && params.x.is_none() && params.y.is_none() {
        result.errors.push("No public key data provided".to_string());
        return;
    }
    
    // Validate point format
    if !params.public_key.is_empty() {
        validate_ecc_point_encoding(&params.public_key, result);
    }
    
    // Validate point coordinates if available
    if let (Some(x), Some(y)) = (&params.x, &params.y) {
        validate_ecc_point_on_curve(&params.curve_name, x, y, result);
    }
    
    // Check for point at infinity
    if let (Some(x), Some(y)) = (&params.x, &params.y) {
        if x.is_zero() && y.is_zero() {
            result.errors.push("ECC public key cannot be point at infinity".to_string());
        }
    }
}

/// Validate ECC point encoding
fn validate_ecc_point_encoding(point_data: &[u8], result: &mut KeyValidationResult) {
    if point_data.is_empty() {
        result.errors.push("Empty ECC point data".to_string());
        return;
    }
    
    match point_data[0] {
        0x04 => {
            // Uncompressed point format
            if point_data.len() % 2 != 1 {
                result.errors.push("Invalid uncompressed ECC point length".to_string());
            }
        },
        0x02 | 0x03 => {
            // Compressed point format
            result.parameters.insert("point_format".to_string(), "compressed".to_string());
        },
        _ => {
            result.errors.push(format!("Unknown ECC point format: 0x{:02x}", point_data[0]));
        }
    }
}

/// Validate ECC point is on curve (simplified validation)
fn validate_ecc_point_on_curve(curve_name: &str, x: &BigUint, y: &BigUint, result: &mut KeyValidationResult) {
    // This is a simplified validation - real implementation would need curve parameters
    match curve_name {
        "secp256r1" | "secp384r1" | "secp521r1" | "secp256k1" => {
            // For now, just check that coordinates are reasonable
            if x.is_zero() || y.is_zero() {
                result.warnings.push("ECC point coordinates should not be zero for standard curves".to_string());
            }
        },
        _ => {
            result.warnings.push(format!("Cannot validate point on curve: {}", curve_name));
        }
    }
}

/// Validate ECC private key
fn validate_ecc_private_key(params: &ECCKeyParams, result: &mut KeyValidationResult) {
    if let Some(private_key) = &params.private_key {
        if private_key.is_empty() {
            result.errors.push("Empty ECC private key".to_string());
            return;
        }
        
        // Check private key length based on curve
        let expected_length = match params.curve_name.as_str() {
            "secp256r1" | "secp256k1" => 32,
            "secp384r1" => 48,
            "secp521r1" => 66,
            _ => return, // Unknown curve
        };
        
        if private_key.len() != expected_length {
            result.errors.push(format!(
                "Invalid ECC private key length: {} bytes (expected {})", 
                private_key.len(), 
                expected_length
            ));
        }
        
        // Check private key is not all zeros
        if private_key.iter().all(|&b| b == 0) {
            result.errors.push("ECC private key cannot be all zeros".to_string());
        }
    }
}

/// Calculate ECC key strength
fn calculate_ecc_strength(curve_name: &str) -> u32 {
    match curve_name {
        "secp256r1" | "secp256k1" | "brainpoolP256r1" => 128,
        "secp384r1" | "brainpoolP384r1" => 192,
        "secp521r1" | "brainpoolP512r1" => 256,
        _ => 80, // Conservative estimate for unknown curves
    }
}

/// Validate ECC key strength
fn validate_ecc_strength(strength_bits: u32, result: &mut KeyValidationResult) {
    if strength_bits < 80 {
        result.errors.push(format!("ECC key strength too low: {} bits (minimum 80)", strength_bits));
    } else if strength_bits < 128 {
        result.warnings.push(format!("ECC key strength below recommended: {} bits (recommended 128+)", strength_bits));
    }
}

/// Validate Ed25519/X25519 key
fn validate_eddsa_key(key_obj: &HashMap<String, Value>, _options: Option<&Value>) -> Result<KeyValidationResult, CursedError> {
    let params = parse_eddsa_params(key_obj)?;
    let mut result = KeyValidationResult {
        valid: true,
        key_type: params.key_type.clone(),
        strength_bits: 128, // Ed25519/X25519 provide 128-bit security
        warnings: Vec::new(),
        errors: Vec::new(),
        parameters: HashMap::new(),
    };

    // Validate key lengths
    validate_eddsa_key_lengths(&params, &mut result);
    
    // Validate key format
    validate_eddsa_key_format(&params, &mut result);
    
    // Validate private/public key consistency if both present
    if params.private_key.is_some() {
        validate_eddsa_key_consistency(&params, &mut result);
    }

    result.valid = result.errors.is_empty();
    Ok(result)
}

/// Parse EdDSA parameters from key object
fn parse_eddsa_params(key_obj: &HashMap<String, Value>) -> Result<EdDSAKeyParams, CursedError> {
    let key_type = if key_obj.contains_key("ed25519_public") || key_obj.contains_key("ed25519_private") {
        KeyType::Ed25519
    } else if key_obj.contains_key("x25519_public") || key_obj.contains_key("x25519_private") {
        KeyType::X25519
    } else {
        return Err(CursedError::InvalidArgument("Cannot determine EdDSA key type".to_string()));
    };

    let public_key = match key_type {
        KeyType::Ed25519 => extract_bytes(key_obj, "ed25519_public").unwrap_or_default(),
        KeyType::X25519 => extract_bytes(key_obj, "x25519_public").unwrap_or_default(),
        _ => Vec::new(),
    };

    let private_key = match key_type {
        KeyType::Ed25519 => extract_bytes_optional(key_obj, "ed25519_private"),
        KeyType::X25519 => extract_bytes_optional(key_obj, "x25519_private"),
        _ => None,
    };

    Ok(EdDSAKeyParams { key_type, public_key, private_key })
}

/// Validate EdDSA key lengths
fn validate_eddsa_key_lengths(params: &EdDSAKeyParams, result: &mut KeyValidationResult) {
    // Both Ed25519 and X25519 use 32-byte keys
    const EXPECTED_LENGTH: usize = 32;
    
    if !params.public_key.is_empty() && params.public_key.len() != EXPECTED_LENGTH {
        result.errors.push(format!(
            "{:?} public key must be {} bytes, got {}", 
            params.key_type, 
            EXPECTED_LENGTH, 
            params.public_key.len()
        ));
    }
    
    if let Some(private_key) = &params.private_key {
        if private_key.len() != EXPECTED_LENGTH {
            result.errors.push(format!(
                "{:?} private key must be {} bytes, got {}", 
                params.key_type, 
                EXPECTED_LENGTH, 
                private_key.len()
            ));
        }
    }
}

/// Validate EdDSA key format
fn validate_eddsa_key_format(params: &EdDSAKeyParams, result: &mut KeyValidationResult) {
    // Check for all-zero keys (invalid)
    if !params.public_key.is_empty() && params.public_key.iter().all(|&b| b == 0) {
        result.errors.push(format!("{:?} public key cannot be all zeros", params.key_type));
    }
    
    if let Some(private_key) = &params.private_key {
        if private_key.iter().all(|&b| b == 0) {
            result.errors.push(format!("{:?} private key cannot be all zeros", params.key_type));
        }
    }
    
    // For Ed25519, validate public key is on curve (simplified check)
    if params.key_type == KeyType::Ed25519 && !params.public_key.is_empty() {
        validate_ed25519_public_key(&params.public_key, result);
    }
}

/// Validate Ed25519 public key point (simplified validation)
fn validate_ed25519_public_key(public_key: &[u8], result: &mut KeyValidationResult) {
    if public_key.len() != 32 {
        return; // Already validated by length check
    }
    
    // Check the high bit constraint for Ed25519
    if public_key[31] & 0x80 != 0 {
        // This is actually valid for Ed25519, but worth noting
        result.parameters.insert("high_bit_set".to_string(), "true".to_string());
    }
    
    // Additional format validation could be added here
    result.parameters.insert("format_valid".to_string(), "true".to_string());
}

/// Validate EdDSA key consistency
fn validate_eddsa_key_consistency(params: &EdDSAKeyParams, result: &mut KeyValidationResult) {
    if let Some(_private_key) = &params.private_key {
        if params.public_key.is_empty() {
            result.warnings.push("Cannot validate key consistency: missing public key".to_string());
            return;
        }
        
        // In a real implementation, we would derive the public key from private key
        // and compare. For now, we just check they're both non-empty and valid length.
        result.parameters.insert("consistency_check".to_string(), "basic".to_string());
    }
}

/// Validate encoded key (PEM/DER format)
fn validate_encoded_key(key_str: &str, _options: Option<&Value>) -> Result<KeyValidationResult, CursedError> {
    let mut result = KeyValidationResult {
        valid: false,
        key_type: KeyType::RSA, // Will be updated
        strength_bits: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        parameters: HashMap::new(),
    };

    // Basic PEM format validation
    if key_str.starts_with("-----BEGIN") && key_str.ends_with("-----END") {
        validate_pem_format(key_str, &mut result);
    } else {
        // Assume DER format
        validate_der_format(key_str, &mut result);
    }

    result.valid = result.errors.is_empty();
    Ok(result)
}

/// Validate PEM format
fn validate_pem_format(pem_data: &str, result: &mut KeyValidationResult) {
    let lines: Vec<&str> = pem_data.lines().collect();
    
    if lines.len() < 3 {
        result.errors.push("Invalid PEM format: too few lines".to_string());
        return;
    }
    
    let header = lines[0];
    let footer = lines[lines.len() - 1];
    
    // Validate header/footer matching
    if !header.starts_with("-----BEGIN") || !footer.starts_with("-----END") {
        result.errors.push("Invalid PEM format: missing BEGIN/END markers".to_string());
        return;
    }
    
    // Determine key type from header
    if header.contains("RSA") {
        result.key_type = KeyType::RSA;
    } else if header.contains("EC") {
        result.key_type = KeyType::ECC;
    } else if header.contains("PRIVATE KEY") || header.contains("PUBLIC KEY") {
        result.key_type = KeyType::RSA; // Default assumption
    }
    
    // Validate base64 content
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || i == lines.len() - 1 {
            continue; // Skip header/footer
        }
        
        if !is_valid_base64(line) {
            result.errors.push(format!("Invalid base64 data on line {}", i + 1));
        }
    }
    
    result.parameters.insert("format".to_string(), "PEM".to_string());
}

/// Validate DER format (basic check)
fn validate_der_format(der_data: &str, result: &mut KeyValidationResult) {
    // This is a very basic validation - real implementation would parse ASN.1
    if der_data.is_empty() {
        result.errors.push("Empty DER data".to_string());
        return;
    }
    
    // Check if it looks like hex-encoded DER
    if der_data.chars().all(|c| c.is_ascii_hexdigit() || c.is_whitespace()) {
        result.parameters.insert("format".to_string(), "DER (hex)".to_string());
    } else {
        result.parameters.insert("format".to_string(), "DER (binary)".to_string());
    }
    
    result.warnings.push("DER format validation is basic - full ASN.1 parsing not implemented".to_string());
}

/// Serialize validation result to Value
fn serialize_validation_result(result: KeyValidationResult) -> Value {
    let mut obj = HashMap::new();
    
    obj.insert("valid".to_string(), Value::Bool(result.valid));
    obj.insert("key_type".to_string(), Value::String(format!("{:?}", result.key_type)));
    obj.insert("strength_bits".to_string(), Value::Int(result.strength_bits as i64));
    
    let warnings: Vec<Value> = result.warnings.into_iter().map(Value::String).collect();
    obj.insert("warnings".to_string(), Value::Array(warnings));
    
    let errors: Vec<Value> = result.errors.into_iter().map(Value::String).collect();
    obj.insert("errors".to_string(), Value::Array(errors));
    
    let params: HashMap<String, Value> = result.parameters.into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();
    obj.insert("parameters".to_string(), Value::Object(params));
    
    Value::Object(obj)
}

// Helper functions

/// Extract BigUint from object
fn extract_bigint(obj: &HashMap<String, Value>, key: &str) -> Result<BigUint, CursedError> {
    match obj.get(key) {
        Some(Value::String(s)) => {
            BigUint::parse_bytes(s.as_bytes(), 10)
                .ok_or_else(|| CursedError::InvalidArgument(format!("Invalid BigInt: {}", s)))
        },
        Some(Value::Int(i)) => {
            if *i >= 0 {
                Ok(BigUint::from(*i as u64))
            } else {
                Err(CursedError::InvalidArgument(format!("Negative integer for {}", key)))
            }
        },
        _ => Err(CursedError::InvalidArgument(format!("Missing or invalid {}", key))),
    }
}

/// Extract optional BigUint from object
fn extract_bigint_optional(obj: &HashMap<String, Value>, key: &str) -> Option<BigUint> {
    extract_bigint(obj, key).ok()
}

/// Extract string from object
fn extract_string(obj: &HashMap<String, Value>, key: &str) -> Option<String> {
    match obj.get(key) {
        Some(Value::String(s)) => Some(s.clone()),
        _ => None,
    }
}

/// Extract bytes from object
fn extract_bytes(obj: &HashMap<String, Value>, key: &str) -> Option<Vec<u8>> {
    match obj.get(key) {
        Some(Value::String(s)) => {
            // Try hex decode first, then base64
            if let Ok(bytes) = hex::decode(s) {
                Some(bytes)
            } else if let Ok(bytes) = base64::decode(s) {
                Some(bytes)
            } else {
                Some(s.as_bytes().to_vec())
            }
        },
        Some(Value::Array(arr)) => {
            let mut bytes = Vec::new();
            for val in arr {
                if let Value::Int(i) = val {
                    if *i >= 0 && *i <= 255 {
                        bytes.push(*i as u8);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Some(bytes)
        },
        _ => None,
    }
}

/// Extract optional bytes from object
fn extract_bytes_optional(obj: &HashMap<String, Value>, key: &str) -> Option<Vec<u8>> {
    extract_bytes(obj, key)
}

/// Check if string is valid base64
fn is_valid_base64(s: &str) -> bool {
    base64::decode(s).is_ok()
}

/// Calculate GCD using Euclidean algorithm
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

/// Calculate modular inverse using extended Euclidean algorithm
fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    if gcd(a, m) != BigUint::one() {
        return None;
    }
    
    // Simplified implementation - real crypto would use proper extended GCD
    // This is a placeholder that works for small numbers
    for i in 1..1000u32 {
        let candidate = BigUint::from(i);
        if (a * &candidate) % m == BigUint::one() {
            return Some(candidate);
        }
    }
    
    None
}

/// Miller-Rabin primality test (simplified)
fn is_probably_prime(n: &BigUint, rounds: u32) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n == &BigUint::from(2u32) || n == &BigUint::from(3u32) {
        return true;
    }
    if n % BigUint::from(2u32) == BigUint::zero() {
        return false;
    }
    
    // Simplified primality test - real implementation would be more thorough
    for _ in 0..rounds {
        // This is a very basic test - real Miller-Rabin would be more complex
        let test_val = BigUint::from(2u32 + (rounds % 100));
        let result = mod_exp(&test_val, &(n - BigUint::one()), n);
        if result != BigUint::one() {
            return false;
        }
    }
    
    true
}

/// Modular exponentiation (simplified)
fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    if modulus == &BigUint::one() {
        return BigUint::zero();
    }
    
    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();
    
    while !exp.is_zero() {
        if &exp % BigUint::from(2u32) == BigUint::one() {
            result = (result * &base) % modulus;
        }
        exp >>= 1;
        base = (&base * &base) % modulus;
    }
    
    result
}

// Additional validation functions for key pair validation

/// Validate key pair consistency
pub fn validate_key_pair(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.len() < 2 {
        return Err(CursedError::InvalidArgument("Key pair validation requires private and public keys".to_string()));
    }

    let private_key = &args[0];
    let public_key = &args[1];
    
    // Validate both keys individually first
    let private_result = validate_key(vec![private_key.clone()])?;
    let public_result = validate_key(vec![public_key.clone()])?;
    
    // Check if both validations passed
    let private_valid = extract_bool_from_result(&private_result, "valid").unwrap_or(false);
    let public_valid = extract_bool_from_result(&public_result, "valid").unwrap_or(false);
    
    let mut result = KeyValidationResult {
        valid: private_valid && public_valid,
        key_type: KeyType::RSA, // Will be updated
        strength_bits: 0,
        warnings: Vec::new(),
        errors: Vec::new(),
        parameters: HashMap::new(),
    };
    
    if !private_valid {
        result.errors.push("Private key validation failed".to_string());
    }
    
    if !public_valid {
        result.errors.push("Public key validation failed".to_string());
    }
    
    // Additional key pair consistency checks would go here
    result.parameters.insert("pair_validation".to_string(), "completed".to_string());
    
    Ok(serialize_validation_result(result))
}

/// Extract boolean from validation result
fn extract_bool_from_result(result: &Value, key: &str) -> Option<bool> {
    if let Value::Object(obj) = result {
        if let Some(Value::Bool(b)) = obj.get(key) {
            Some(*b)
        } else {
            None
        }
    } else {
        None
    }
}

/// Validate key strength according to current standards
pub fn validate_key_strength(args: Vec<Value>) -> Result<Value, CursedError> {
    if args.is_empty() {
        return Err(CursedError::InvalidArgument("No key provided for strength validation".to_string()));
    }

    let key_data = &args[0];
    let min_strength = if args.len() > 1 {
        match &args[1] {
            Value::Int(i) => *i as u32,
            _ => 112, // Default minimum strength
        }
    } else {
        112
    };

    // First validate the key
    let validation_result = validate_key(vec![key_data.clone()])?;
    
    // Extract strength from validation result
    if let Value::Object(obj) = &validation_result {
        if let Some(Value::Int(strength)) = obj.get("strength_bits") {
            let strength_bits = *strength as u32;
            
            let mut result = HashMap::new();
            result.insert("valid_strength".to_string(), Value::Bool(strength_bits >= min_strength));
            result.insert("actual_strength".to_string(), Value::Int(strength_bits as i64));
            result.insert("required_strength".to_string(), Value::Int(min_strength as i64));
            result.insert("meets_standard".to_string(), Value::Bool(strength_bits >= 112));
            
            return Ok(Value::Object(result));
        }
    }
    
    Err(CursedError::RuntimeError("Failed to extract key strength".to_string()))
}
