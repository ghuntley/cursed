/// fr fr Comprehensive tests for crypto format conversions
/// 
/// Tests all format conversion functionality including PEM, DER, JWK formats
/// and ensures proper integration with the crypto ecosystem.

use cursed::stdlib::crypto::format_conversions::*;
use cursed::stdlib::crypto::asymmetric::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;

#[test]
fn test_key_format_detection() {
    // Test PEM format detection
    let pem_data = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA...\n-----END PUBLIC KEY-----";
    assert_eq!(FormatConverter::detect_key_format(pem_data), KeyFormat::Pem);
    
    // Test JWK format detection
    let jwk_data = r#"{"kty":"RSA","n":"0vx7agoebGcQ...","e":"AQAB"}"#;
    assert_eq!(FormatConverter::detect_key_format(jwk_data), KeyFormat::Jwk);
    
    // Test hex format detection
    let hex_data = "deadbeefcafebabe1234567890abcdef";
    assert_eq!(FormatConverter::detect_key_format(hex_data), KeyFormat::Raw);
    
    // Test base64 format detection
    let base64_data = "SGVsbG8gV29ybGQ=";
    assert_eq!(FormatConverter::detect_key_format(base64_data), KeyFormat::Der);
}

#[test]
fn test_supported_formats() {
    let rsa_formats = FormatConverter::supported_formats("rsa");
    assert!(rsa_formats.contains(&KeyFormat::Pem));
    assert!(rsa_formats.contains(&KeyFormat::Der));
    assert!(rsa_formats.contains(&KeyFormat::Jwk));
    assert!(rsa_formats.contains(&KeyFormat::Pkcs8));
    assert!(rsa_formats.contains(&KeyFormat::Spki));
    
    let ec_formats = FormatConverter::supported_formats("ec");
    assert!(ec_formats.contains(&KeyFormat::Pem));
    assert!(ec_formats.contains(&KeyFormat::Der));
    assert!(ec_formats.contains(&KeyFormat::Jwk));
    assert!(ec_formats.contains(&KeyFormat::Raw));
    
    let x448_formats = FormatConverter::supported_formats("x448");
    assert!(x448_formats.contains(&KeyFormat::Raw));
    assert!(x448_formats.contains(&KeyFormat::Jwk));
}

#[test]
fn test_format_conversion_api() {
    // Test detect_format function
    let args = vec![Value::String("-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----".to_string())];
    let result = detect_format(args).unwrap();
    assert_eq!(result, Value::String("PEM".to_string()));
    
    // Test JWK conversion
    let jwk_args = vec![
        Value::String("test_key_data".to_string()),
        Value::String("RSA".to_string()),
        Value::String("test_kid".to_string()),
    ];
    let jwk_result = key_to_jwk(jwk_args).unwrap();
    
    if let Value::Object(map) = jwk_result {
        assert_eq!(map.get("kty"), Some(&Value::String("RSA".to_string())));
        assert_eq!(map.get("kid"), Some(&Value::String("test_kid".to_string())));
        assert_eq!(map.get("use"), Some(&Value::String("sig".to_string())));
        assert_eq!(map.get("alg"), Some(&Value::String("RS256".to_string())));
    } else {
        panic!("Expected object result");
    }
    
    // Test EC JWK conversion
    let ec_args = vec![
        Value::String("test_ec_data".to_string()),
        Value::String("EC".to_string()),
    ];
    let ec_result = key_to_jwk(ec_args).unwrap();
    
    if let Value::Object(map) = ec_result {
        assert_eq!(map.get("kty"), Some(&Value::String("EC".to_string())));
        assert_eq!(map.get("use"), Some(&Value::String("sig".to_string())));
        assert_eq!(map.get("alg"), Some(&Value::String("ES256".to_string())));
    } else {
        panic!("Expected object result");
    }
}

#[test]
fn test_der_encoding_decoding() {
    // Test DER encoding
    let test_data = vec![0x01, 0x02, 0x03, 0x04];
    let encoded = FormatConverter::enhanced_der_encode(&test_data, "octet_string").unwrap();
    
    // DER-encoded octet string should start with tag and length
    assert_eq!(encoded[0], 0x04); // OCTET STRING tag
    assert_eq!(encoded[1], 0x04); // Length
    assert_eq!(&encoded[2..], &test_data);
    
    // Test DER decoding
    let (tag, content) = FormatConverter::enhanced_der_decode(&encoded).unwrap();
    assert_eq!(tag, "octet_string");
    assert_eq!(content, test_data);
    
    // Test API functions
    let encode_args = vec![
        Value::String(hex::encode(&test_data)),
        Value::String("octet_string".to_string()),
    ];
    let encode_result = key_to_der(encode_args).unwrap();
    
    if let Value::String(der_hex) = encode_result {
        let decode_args = vec![Value::String(der_hex)];
        let decode_result = der_decode(decode_args).unwrap();
        
        if let Value::Object(map) = decode_result {
            assert_eq!(map.get("tag"), Some(&Value::String("octet_string".to_string())));
            assert_eq!(map.get("content"), Some(&Value::String(hex::encode(&test_data))));
        } else {
            panic!("Expected object result");
        }
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_jwk_json_parsing() {
    let jwk_json = r#"{
        "kty": "RSA",
        "use": "sig",
        "alg": "RS256",
        "kid": "test-key-1",
        "n": "0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4n3oknjhMstn64tZ_2W-5JsGY4Hc5n9yBXArwl93lqt7_RN5w6Cf0h4QyQ5v-65YGjQR0_FDW2QvzqY368QQMicAtaSqzs8KJZgnYb9c7d0zgdAZHzu6qMQvRL5hajrn1n91CbOpbISD08qNLyrdkt-bFTWhAI4vMQFh6WeZu0fM4lFd2NcRwr3XPksINHaQ-G_xBniIqbw0Ls1jF44-csFCur-kEgU8awapJzKnqDKgw",
        "e": "AQAB"
    }"#;
    
    let args = vec![Value::String(jwk_json.to_string())];
    let result = jwk_from_json(args).unwrap();
    
    if let Value::String(parsed_json) = result {
        // Should be able to parse back
        let jwk = FormatConverter::jwk_from_json(&parsed_json).unwrap();
        assert_eq!(jwk.kty, "RSA");
        assert_eq!(jwk.use_, Some("sig".to_string()));
        assert_eq!(jwk.alg, Some("RS256".to_string()));
        assert_eq!(jwk.kid, Some("test-key-1".to_string()));
        assert!(jwk.n.is_some());
        assert!(jwk.e.is_some());
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_error_handling() {
    // Test invalid format detection
    let args = vec![Value::Number(123.0)];
    assert!(detect_format(args).is_err());
    
    // Test invalid JWK creation  
    let invalid_args = vec![
        Value::String("test".to_string()),
        Value::String("INVALID".to_string()),
    ];
    assert!(key_to_jwk(invalid_args).is_err());
    
    // Test invalid DER data
    let invalid_der_args = vec![Value::String("invalid_hex".to_string())];
    assert!(der_decode(invalid_der_args).is_err());
    
    // Test empty data
    assert!(FormatConverter::enhanced_der_encode(&[], "sequence").is_err());
    
    // Test unsupported DER tag
    let test_data = vec![0x01, 0x02];
    assert!(FormatConverter::enhanced_der_encode(&test_data, "unsupported").is_err());
    
    // Test truncated DER data
    let truncated_der = vec![0x30, 0x10, 0x01]; // Sequence claiming 16 bytes but only 1 byte follows
    assert!(FormatConverter::enhanced_der_decode(&truncated_der).is_err());
}

#[test]
fn test_key_format_names() {
    assert_eq!(KeyFormat::Pem.name(), "PEM");
    assert_eq!(KeyFormat::Der.name(), "DER");
    assert_eq!(KeyFormat::Jwk.name(), "JWK");
    assert_eq!(KeyFormat::Raw.name(), "Raw");
    assert_eq!(KeyFormat::Pkcs8.name(), "PKCS#8");
    assert_eq!(KeyFormat::Spki.name(), "SPKI");
}

#[test]
fn test_key_format_from_string() {
    assert_eq!(KeyFormat::from_str("pem").unwrap(), KeyFormat::Pem);
    assert_eq!(KeyFormat::from_str("DER").unwrap(), KeyFormat::Der);
    assert_eq!(KeyFormat::from_str("jwk").unwrap(), KeyFormat::Jwk);
    assert_eq!(KeyFormat::from_str("raw").unwrap(), KeyFormat::Raw);
    assert_eq!(KeyFormat::from_str("pkcs8").unwrap(), KeyFormat::Pkcs8);
    assert_eq!(KeyFormat::from_str("spki").unwrap(), KeyFormat::Spki);
    
    assert!(KeyFormat::from_str("invalid").is_err());
}

#[test]
fn test_comprehensive_format_support() {
    // Test all supported combinations
    let test_cases = vec![
        ("rsa", vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Pkcs8, KeyFormat::Spki]),
        ("ec", vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw]),
        ("ecdsa", vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw]),
        ("ed25519", vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw]),
        ("x25519", vec![KeyFormat::Pem, KeyFormat::Der, KeyFormat::Jwk, KeyFormat::Raw]),
        ("x448", vec![KeyFormat::Raw, KeyFormat::Jwk]),
    ];
    
    for (key_type, expected_formats) in test_cases {
        let actual_formats = FormatConverter::supported_formats(key_type);
        for format in expected_formats {
            assert!(actual_formats.contains(&format), 
                "Key type {} should support format {}", key_type, format.name());
        }
    }
    
    // Test unknown key type
    let unknown_formats = FormatConverter::supported_formats("unknown");
    assert_eq!(unknown_formats, vec![KeyFormat::Raw]);
}

#[test]
fn test_der_structure_validation() {
    // Test valid DER structures
    let test_cases = vec![
        ("sequence", 0x30),
        ("octet_string", 0x04),
        ("bit_string", 0x03),
        ("integer", 0x02),
    ];
    
    for (tag_name, expected_tag) in test_cases {
        let test_data = vec![0x12, 0x34, 0x56, 0x78];
        let encoded = FormatConverter::enhanced_der_encode(&test_data, tag_name).unwrap();
        
        assert_eq!(encoded[0], expected_tag);
        assert_eq!(encoded[1], test_data.len() as u8);
        assert_eq!(&encoded[2..], &test_data);
        
        let (decoded_tag, decoded_content) = FormatConverter::enhanced_der_decode(&encoded).unwrap();
        assert_eq!(decoded_tag, tag_name);
        assert_eq!(decoded_content, test_data);
    }
}

#[test]
fn test_format_detection_edge_cases() {
    // Test empty string
    assert_eq!(FormatConverter::detect_key_format(""), KeyFormat::Raw);
    
    // Test whitespace-only string
    assert_eq!(FormatConverter::detect_key_format("   \n  \t  "), KeyFormat::Raw);
    
    // Test partial PEM (missing end)
    let partial_pem = "-----BEGIN PUBLIC KEY-----\ndata here";
    assert_eq!(FormatConverter::detect_key_format(partial_pem), KeyFormat::Raw);
    
    // Test malformed JSON
    let malformed_json = r#"{"kty":"RSA","incomplete"#;
    assert_eq!(FormatConverter::detect_key_format(malformed_json), KeyFormat::Raw);
    
    // Test valid JSON but not JWK
    let non_jwk_json = r#"{"name":"test","value":123}"#;
    assert_eq!(FormatConverter::detect_key_format(non_jwk_json), KeyFormat::Raw);
    
    // Test mixed case hex
    let mixed_hex = "DeAdBeEfCaFeBaBe";
    assert_eq!(FormatConverter::detect_key_format(mixed_hex), KeyFormat::Raw);
    
    // Test invalid hex
    let invalid_hex = "deadbeefXYZ";
    assert_eq!(FormatConverter::detect_key_format(invalid_hex), KeyFormat::Raw);
}

#[test]
fn test_api_parameter_validation() {
    // Test insufficient parameters
    assert!(key_to_jwk(vec![]).is_err());
    assert!(key_to_jwk(vec![Value::String("test".to_string())]).is_err());
    
    assert!(jwk_from_json(vec![]).is_err());
    
    assert!(key_to_der(vec![]).is_err());
    assert!(key_to_der(vec![Value::String("test".to_string())]).is_err());
    
    assert!(der_decode(vec![]).is_err());
    
    assert!(detect_format(vec![]).is_err());
    
    // Test wrong parameter types
    assert!(key_to_jwk(vec![Value::Number(123.0), Value::String("RSA".to_string())]).is_err());
    assert!(key_to_jwk(vec![Value::String("test".to_string()), Value::Number(123.0)]).is_err());
    
    assert!(jwk_from_json(vec![Value::Number(123.0)]).is_err());
    
    assert!(key_to_der(vec![Value::Number(123.0), Value::String("tag".to_string())]).is_err());
    assert!(key_to_der(vec![Value::String("test".to_string()), Value::Number(123.0)]).is_err());
    
    assert!(der_decode(vec![Value::Number(123.0)]).is_err());
    
    assert!(detect_format(vec![Value::Number(123.0)]).is_err());
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_format_conversion_round_trip() {
        // Test round-trip conversion for various data types
        let test_data = vec![
            vec![0x01, 0x02, 0x03, 0x04],
            vec![0xFF, 0xFE, 0xFD, 0xFC],
            vec![0x00, 0x00, 0x00, 0x00],
            (0..255).collect::<Vec<u8>>(),
        ];
        
        for data in test_data {
            // Test DER round-trip
            let encoded = FormatConverter::enhanced_der_encode(&data, "octet_string").unwrap();
            let (tag, decoded) = FormatConverter::enhanced_der_decode(&encoded).unwrap();
            
            assert_eq!(tag, "octet_string");
            assert_eq!(decoded, data);
        }
    }
    
    #[test]
    fn test_multiple_format_detection() {
        let test_cases = vec![
            ("-----BEGIN CERTIFICATE-----\nMIIC...\n-----END CERTIFICATE-----", KeyFormat::Pem),
            ("-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n-----END RSA PRIVATE KEY-----", KeyFormat::Pem),
            (r#"{"kty":"EC","crv":"P-256","x":"...","y":"..."}"#, KeyFormat::Jwk),
            (r#"{"kty":"OKP","crv":"Ed25519","x":"..."}"#, KeyFormat::Jwk),
            ("MIIB1jCCAX+gAwIBAgIJAL...", KeyFormat::Der),
            ("deadbeefcafebabe0123456789abcdef", KeyFormat::Raw),
        ];
        
        for (data, expected_format) in test_cases {
            let detected = FormatConverter::detect_key_format(data);
            assert_eq!(detected, expected_format, "Failed for data: {}", data);
        }
    }
}
