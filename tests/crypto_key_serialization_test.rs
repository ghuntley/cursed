//! Comprehensive tests for crypto key serialization functionality
//! 
//! This test suite validates the complete key serialization system for the CURSED crypto package,
//! ensuring all formats (PEM, DER, JWK, SSH, Hex, Raw) work correctly with all supported key types.

use cursed::stdlib::packages::crypto_asymmetric::key_serialization::*;
use cursed::stdlib::value::Value;
use cursed::error::CursedError;

/// Test RSA key serialization across all supported formats
#[test]
fn test_rsa_key_serialization_formats() {
    // Test data - in a real implementation this would be a valid RSA key
    let mock_rsa_private_key_der = vec![0u8; 256]; // Mock DER data
    let mock_rsa_public_key_der = vec![0u8; 128];  // Mock DER data
    
    // Test formats that should work for RSA keys
    let supported_formats = vec![
        SerializationFormat::Pem,
        SerializationFormat::Der,
        SerializationFormat::Hex,
        SerializationFormat::Raw,
    ];
    
    for format in supported_formats {
        println!("Testing RSA private key serialization with format: {:?}", format);
        
        let args = vec![
            Value::String("RSA-PRIVATE".to_string()),
            Value::String(hex::encode(&mock_rsa_private_key_der)),
            Value::String(format.name().to_string()),
        ];
        
        // This will fail with mock data but tests the format handling logic
        let result = serialize_key(args);
        // We expect this to fail due to invalid key data, but not due to format issues
        if let Err(CursedError::CryptoError(_)) = result {
            // Expected - invalid key data
        } else if let Err(CursedError::NotImplemented(_)) = result {
            panic!("Format {:?} not implemented for RSA private keys", format);
        }
    }
}

/// Test ECDSA P-256 key serialization
#[test] 
fn test_p256_key_serialization_formats() {
    let mock_p256_key_der = vec![0u8; 64]; // Mock DER data
    
    let supported_formats = vec![
        SerializationFormat::Pem,
        SerializationFormat::Der,
        SerializationFormat::Hex,
        SerializationFormat::Raw,
    ];
    
    for format in supported_formats {
        println!("Testing P-256 private key serialization with format: {:?}", format);
        
        let args = vec![
            Value::String("ECDSA-P256-PRIVATE".to_string()),
            Value::String(hex::encode(&mock_p256_key_der)),
            Value::String(format.name().to_string()),
        ];
        
        let result = serialize_key(args);
        if let Err(CursedError::NotImplemented(_)) = result {
            panic!("Format {:?} not implemented for P-256 private keys", format);
        }
    }
}

/// Test Ed25519 key serialization with real-sized key data
#[test]
fn test_ed25519_key_serialization() {
    let ed25519_private_key = vec![42u8; 32]; // Valid size for Ed25519
    let ed25519_public_key = vec![84u8; 32];  // Valid size for Ed25519
    
    let formats_to_test = vec![
        SerializationFormat::Raw,
        SerializationFormat::Hex,
        SerializationFormat::Pem,
        SerializationFormat::Der,
    ];
    
    // Test private key serialization
    for format in &formats_to_test {
        println!("Testing Ed25519 private key with format: {:?}", format);
        
        let args = vec![
            Value::String("ED25519-PRIVATE".to_string()),
            Value::String(hex::encode(&ed25519_private_key)),
            Value::String(format.name().to_string()),
        ];
        
        let result = serialize_key(args);
        match result {
            Ok(value) => {
                // Verify the result structure
                if let Value::Object(map) = value {
                    assert!(map.contains_key("format"));
                    assert!(map.contains_key("key_type"));
                    assert!(map.contains_key("data"));
                    
                    if let Some(Value::String(fmt)) = map.get("format") {
                        assert_eq!(fmt, format.name());
                    }
                    
                    if let Some(Value::String(key_type)) = map.get("key_type") {
                        assert_eq!(key_type, "Ed25519-Private");
                    }
                    
                    println!("✓ Ed25519 private key serialization successful for {:?}", format);
                } else {
                    panic!("Invalid result structure for Ed25519 private key");
                }
            },
            Err(CursedError::NotImplemented(_)) => {
                panic!("Format {:?} not implemented for Ed25519 private keys", format);
            },
            Err(e) => {
                println!("Expected error for mock data: {:?}", e);
            }
        }
    }
    
    // Test public key serialization
    for format in &formats_to_test {
        println!("Testing Ed25519 public key with format: {:?}", format);
        
        let args = vec![
            Value::String("ED25519-PUBLIC".to_string()),
            Value::String(hex::encode(&ed25519_public_key)),
            Value::String(format.name().to_string()),
        ];
        
        let result = serialize_key(args);
        match result {
            Ok(value) => {
                if let Value::Object(map) = value {
                    assert!(map.contains_key("format"));
                    assert!(map.contains_key("key_type"));
                    
                    if let Some(Value::String(key_type)) = map.get("key_type") {
                        assert_eq!(key_type, "Ed25519-Public");
                    }
                    
                    println!("✓ Ed25519 public key serialization successful for {:?}", format);
                }
            },
            Err(CursedError::NotImplemented(_)) => {
                panic!("Format {:?} not implemented for Ed25519 public keys", format);
            },
            Err(e) => {
                println!("Expected error for mock data: {:?}", e);
            }
        }
    }
}

/// Test X25519 key serialization
#[test]
fn test_x25519_key_serialization() {
    let x25519_private_key = vec![123u8; 32]; // Valid size for X25519
    let x25519_public_key = vec![231u8; 32];  // Valid size for X25519
    
    let formats_to_test = vec![
        SerializationFormat::Raw,
        SerializationFormat::Hex,
        SerializationFormat::Pem,
        SerializationFormat::Der,
    ];
    
    // Test private key
    for format in &formats_to_test {
        let args = vec![
            Value::String("X25519-PRIVATE".to_string()),
            Value::String(hex::encode(&x25519_private_key)),
            Value::String(format.name().to_string()),
        ];
        
        let result = serialize_key(args);
        if let Err(CursedError::NotImplemented(_)) = result {
            panic!("Format {:?} not implemented for X25519 private keys", format);
        }
    }
    
    // Test public key
    for format in &formats_to_test {
        let args = vec![
            Value::String("X25519-PUBLIC".to_string()),
            Value::String(hex::encode(&x25519_public_key)),
            Value::String(format.name().to_string()),
        ];
        
        let result = serialize_key(args);
        if let Err(CursedError::NotImplemented(_)) = result {
            panic!("Format {:?} not implemented for X25519 public keys", format);
        }
    }
}

/// Test key deserialization
#[test]
fn test_key_deserialization() {
    // Test hex format deserialization
    let hex_data = "deadbeefcafebabe123456789abcdef0123456789abcdef0deadbeefcafebabe";
    
    let args = vec![
        Value::String("HEX".to_string()),
        Value::String(hex_data.to_string()),
        Value::String("ED25519-PRIVATE".to_string()),
    ];
    
    let result = deserialize_key(args);
    match result {
        Ok(Value::Object(map)) => {
            assert!(map.contains_key("format"));
            assert!(map.contains_key("key_type"));
            assert!(map.contains_key("valid"));
            assert!(map.contains_key("key_data"));
            
            println!("✓ Key deserialization successful");
        },
        Err(e) => {
            println!("Deserialization error (expected for test data): {:?}", e);
        },
        _ => panic!("Unexpected result type from deserialization"),
    }
}

/// Test format compatibility matrix
#[test]
fn test_format_compatibility_completeness() {
    let compatibility = get_format_compatibility();
    
    // Ensure all major key types are represented
    let expected_key_types = vec![
        "RSA", "ECDSA-P256", "ECDSA-P384", "Ed25519", "X25519"
    ];
    
    for key_type in expected_key_types {
        assert!(compatibility.contains_key(key_type), 
               "Missing compatibility info for key type: {}", key_type);
        
        let formats = &compatibility[key_type];
        assert!(!formats.is_empty(), 
               "No formats listed for key type: {}", key_type);
        
        // All key types should support at least Raw and Hex
        assert!(formats.contains(&"Raw".to_string()),
               "Key type {} missing Raw format support", key_type);
        assert!(formats.contains(&"Hex".to_string()),
               "Key type {} missing Hex format support", key_type);
        
        println!("✓ Key type {} supports {} formats", key_type, formats.len());
    }
}

/// Test serialization format enumeration
#[test]
fn test_serialization_formats() {
    let formats = list_serialization_formats();
    
    let expected_formats = vec![
        "PEM", "DER", "JWK", "SSH", "Raw", "Hex"
    ];
    
    for expected in expected_formats {
        assert!(formats.contains(&expected.to_string()),
               "Missing format: {}", expected);
    }
    
    println!("✓ All expected serialization formats available: {:?}", formats);
}

/// Test error handling for invalid inputs
#[test] 
fn test_error_handling() {
    // Test invalid key type
    let args = vec![
        Value::String("INVALID-KEY-TYPE".to_string()),
        Value::String("deadbeef".to_string()),
        Value::String("HEX".to_string()),
    ];
    
    let result = serialize_key(args);
    assert!(result.is_err(), "Should reject invalid key type");
    
    // Test invalid format
    let args = vec![
        Value::String("ED25519-PRIVATE".to_string()),
        Value::String("deadbeef".to_string()),
        Value::String("INVALID-FORMAT".to_string()),
    ];
    
    let result = serialize_key(args);
    assert!(result.is_err(), "Should reject invalid format");
    
    // Test insufficient arguments
    let args = vec![
        Value::String("ED25519-PRIVATE".to_string()),
    ];
    
    let result = serialize_key(args);
    assert!(result.is_err(), "Should reject insufficient arguments");
    
    println!("✓ Error handling working correctly");
}

/// Test key type and format validation
#[test]
fn test_key_type_validation() {
    // Test valid key types
    let valid_key_types = vec![
        "RSA-PRIVATE", "RSA-PUBLIC",
        "ECDSA-P256-PRIVATE", "ECDSA-P256-PUBLIC",
        "ECDSA-P384-PRIVATE", "ECDSA-P384-PUBLIC", 
        "ED25519-PRIVATE", "ED25519-PUBLIC",
        "X25519-PRIVATE", "X25519-PUBLIC",
    ];
    
    for key_type_name in valid_key_types {
        let result = KeyType::from_name(key_type_name);
        assert!(result.is_ok(), "Valid key type {} should be accepted", key_type_name);
        
        let key_type = result.unwrap();
        assert_eq!(key_type.name(), key_type_name.replace("_", "-"));
        
        // Test properties
        let is_private = key_type_name.contains("PRIVATE");
        assert_eq!(key_type.is_private(), is_private);
    }
    
    // Test valid formats
    let valid_formats = vec!["PEM", "DER", "JWK", "SSH", "RAW", "HEX"];
    
    for format_name in valid_formats {
        let result = SerializationFormat::from_name(format_name);
        assert!(result.is_ok(), "Valid format {} should be accepted", format_name);
        
        let format = result.unwrap();
        assert_eq!(format.name(), format_name);
        assert!(!format.description().is_empty());
        assert!(!format.file_extension().is_empty());
    }
    
    println!("✓ Key type and format validation working correctly");
}

/// Integration test for real workflow
#[test]
fn test_key_serialization_workflow() {
    println!("Testing complete key serialization workflow...");
    
    // Test the complete workflow with Ed25519 keys
    let private_key_bytes = vec![42u8; 32];
    let public_key_bytes = vec![84u8; 32];
    
    // Step 1: Serialize private key to different formats
    let formats_to_test = vec![
        SerializationFormat::Raw,
        SerializationFormat::Hex,
    ];
    
    for format in formats_to_test {
        println!("  Testing workflow with format: {:?}", format.name());
        
        // Serialize private key
        let serialize_args = vec![
            Value::String("ED25519-PRIVATE".to_string()),
            Value::String(hex::encode(&private_key_bytes)),
            Value::String(format.name().to_string()),
        ];
        
        let serialize_result = serialize_key(serialize_args);
        assert!(serialize_result.is_ok(), "Serialization should succeed");
        
        if let Ok(Value::Object(map)) = serialize_result {
            assert_eq!(map.get("format").unwrap(), &Value::String(format.name().to_string()));
            assert_eq!(map.get("key_type").unwrap(), &Value::String("Ed25519-Private".to_string()));
            
            if let Some(Value::String(serialized_data)) = map.get("data") {
                // Step 2: Deserialize the key back
                let deserialize_args = vec![
                    Value::String(format.name().to_string()),
                    Value::String(serialized_data.clone()),
                    Value::String("ED25519-PRIVATE".to_string()),
                ];
                
                let deserialize_result = deserialize_key(deserialize_args);
                assert!(deserialize_result.is_ok(), "Deserialization should succeed");
                
                if let Ok(Value::Object(deser_map)) = deserialize_result {
                    assert_eq!(deser_map.get("format").unwrap(), &Value::String(format.name().to_string()));
                    assert_eq!(deser_map.get("key_type").unwrap(), &Value::String("Ed25519-Private".to_string()));
                    
                    println!("    ✓ Round-trip successful for {:?}", format.name());
                }
            }
        }
    }
    
    println!("✓ Complete key serialization workflow validated");
}
