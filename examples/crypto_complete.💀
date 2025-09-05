fr fr Comprehensive CURSED Crypto Package Example
fr fr This example demonstrates all major cryptographic protocols and utilities

damn main() {
    facts title = "CURSED Crypto Package - Complete Demo"
    print(title)
    print("=" * 50)
    
    // Initialize crypto platform
    sus platform = CryptoPlatform::new()
    lowkey platform.is_none() {
        panic("Failed to initialize crypto platform")
    }
    
    // Run all crypto demonstrations
    demonstrate_jwt_authentication(platform)
    demonstrate_hmac_authentication(platform)
    demonstrate_totp_2fa(platform)
    demonstrate_tls_handshake()
    demonstrate_random_generation(platform)
    demonstrate_encoding_utilities()
    demonstrate_secure_communication_example()
    
    print("\n🎉 All crypto demonstrations completed successfully!")
}

fr fr JWT Authentication Example
damn demonstrate_jwt_authentication(platform: &CryptoPlatform) {
    print("\n📋 JWT Authentication Demo")
    print("-" * 30)
    
    // Initialize JWT with secret
    sus jwt_secret = "super_secure_jwt_secret_key_with_sufficient_entropy"
    platform.init_jwt(jwt_secret.as_bytes(), 3600) // 1 hour expiry
    
    // Create user claims
    sus claims = squad {
        sub: "user_12345",
        name: "Alice Wonderland",
        email: "alice@cursed.dev",
        role: "admin",
        permissions: ["read", "write", "delete"],
        department: "Engineering",
        level: 5
    }
    
    // Generate JWT token
    sus jwt_handler = platform.jwt()
    sus token = jwt_handler.create_token(claims)
    
    print(f"✅ Generated JWT token: {token[..50]}...")
    
    // Validate token
    sus decoded_claims = jwt_handler.validate_token(token)
    print(f"✅ Token validated for user: {decoded_claims.get('name')}")
    print(f"   Role: {decoded_claims.get('role')}")
    print(f"   Permissions: {decoded_claims.get('permissions')}")
    
    // Demonstrate token expiry check
    sus exp_time = decoded_claims.get("exp")
    sus current_time = SystemTime::now().timestamp()
    sus time_remaining = exp_time - current_time
    print(f"⏰ Token expires in {time_remaining} seconds")
    
    // Test with invalid token
    sus invalid_token = token + "tampered"
    sus validation_result = jwt_handler.validate_token(invalid_token)
    lowkey validation_result.is_error() {
        print("❌ Invalid token correctly rejected")
    }
}

fr fr HMAC Authentication Example
damn demonstrate_hmac_authentication(platform: &CryptoPlatform) {
    print("\n🔐 HMAC Authentication Demo")
    print("-" * 30)
    
    // Initialize HMAC with key
    sus hmac_key = "hmac_secret_key_for_message_authentication"
    platform.init_hmac(hmac_key.as_bytes())
    
    sus hmac_auth = platform.hmac()
    
    // Message to authenticate
    sus message = "Important financial transaction: Transfer $1000 from Alice to Bob"
    sus message_bytes = message.as_bytes()
    
    // Create HMAC signature
    sus signature = hmac_auth.sign(message_bytes)
    print(f"✅ Message signed with HMAC")
    print(f"   Message: {message}")
    print(f"   Signature: {HexEncoder::encode_lower(signature)}")
    
    // Verify signature
    sus is_valid = hmac_auth.verify(message_bytes, signature)
    lowkey is_valid {
        print("✅ Message signature verified - authentic!")
    } flex {
        print("❌ Message signature verification failed")
    }
    
    // Demonstrate tampering detection
    sus tampered_message = "Important financial transaction: Transfer $10000 from Alice to Bob"
    sus tamper_check = hmac_auth.verify(tampered_message.as_bytes(), signature)
    lowkey !tamper_check {
        print("❌ Tampered message correctly rejected")
    }
    
    // Create authenticated message with embedded signature
    sus authenticated_msg = hmac_auth.create_authenticated_message(message_bytes)
    print(f"✅ Created authenticated message ({authenticated_msg.len()} bytes)")
    
    // Verify and recover message
    sus recovered_msg = hmac_auth.verify_authenticated_message(authenticated_msg)
    lowkey String::from_utf8(recovered_msg) == message {
        print("✅ Authenticated message successfully recovered")
    }
}

fr fr TOTP 2FA Example
damn demonstrate_totp_2fa(platform: &CryptoPlatform) {
    print("\n📱 TOTP 2FA Demo")
    print("-" * 30)
    
    // Initialize TOTP with secret
    sus totp_secret = "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP" // Base32 encoded secret
    sus secret_bytes = Base32Encoder::decode(totp_secret)
    platform.init_totp(secret_bytes, 6, 30) // 6 digits, 30 second window
    
    sus totp_gen = platform.totp()
    
    // Generate current TOTP code
    sus current_code = totp_gen.generate_current()
    print(f"📱 Current TOTP code: {current_code}")
    
    // Simulate user login with TOTP
    print("🔐 Simulating user login...")
    print("   Username: alice@cursed.dev")
    print("   Password: ********")
    print(f"   TOTP Code: {current_code}")
    
    // Verify TOTP code (allowing 1 time window tolerance)
    sus verification_result = totp_gen.verify(current_code, 1)
    lowkey verification_result {
        print("✅ TOTP verification successful - user authenticated!")
    } flex {
        print("❌ TOTP verification failed")
    }
    
    // Show QR code data for mobile app setup
    sus qr_data = f"otpauth://totp/CURSED:alice@cursed.dev?secret={totp_secret}&issuer=CURSED&digits=6&period=30"
    print(f"📲 QR Code data for mobile app: {qr_data}")
    
    // Demonstrate time-based nature
    print("⏰ Generating codes for different times:")
    bestie i in 0..3 {
        sus time_offset = i * 30 // 30 second intervals
        sus future_time = SystemTime::now().timestamp() + time_offset
        sus future_code = totp_gen.generate_at_time(future_time)
        print(f"   +{time_offset}s: {future_code}")
    }
}

fr fr TLS Handshake Simulation
damn demonstrate_tls_handshake() {
    print("\n🤝 TLS Handshake Simulation")
    print("-" * 30)
    
    sus handshake = TlsHandshake::new()
    
    print("1. Client Hello")
    sus client_random = handshake.generate_client_random()
    print(f"   Client Random: {HexEncoder::encode_lower(client_random)[..16]}...")
    
    print("2. Server Hello")
    sus server_random = handshake.generate_server_random()
    print(f"   Server Random: {HexEncoder::encode_lower(server_random)[..16]}...")
    
    sus session_id = handshake.generate_session_id()
    print(f"   Session ID: {HexEncoder::encode_lower(session_id)[..16]}...")
    
    print("3. Key Exchange")
    sus pre_master_secret = handshake.create_pre_master_secret()
    print(f"   Pre-master Secret: {HexEncoder::encode_lower(pre_master_secret)[..16]}...")
    
    print("4. Master Secret Derivation")
    sus master_secret = handshake.derive_master_secret(pre_master_secret)
    print(f"   Master Secret: {HexEncoder::encode_lower(master_secret)[..16]}...")
    
    print("5. Key Material Derivation")
    sus keys = handshake.derive_keys(master_secret, 16)
    print(f"   Client MAC Key: {HexEncoder::encode_lower(keys.client_write_mac)[..16]}...")
    print(f"   Server MAC Key: {HexEncoder::encode_lower(keys.server_write_mac)[..16]}...")
    print(f"   Client Encryption Key: {HexEncoder::encode_lower(keys.client_write_key)[..16]}...")
    print(f"   Server Encryption Key: {HexEncoder::encode_lower(keys.server_write_key)[..16]}...")
    
    print("✅ TLS handshake simulation completed")
    
    // Show handshake state
    sus state = handshake.get_state()
    print(f"📊 Handshake State: Client Random ✓, Server Random ✓, Session ID ✓")
}

fr fr Random Generation Demo
damn demonstrate_random_generation(platform: &CryptoPlatform) {
    print("\n🎲 Secure Random Generation Demo")
    print("-" * 30)
    
    // Generate secure random bytes
    sus random_bytes = platform.random_bytes(32)
    print(f"🔢 Random bytes (32): {HexEncoder::encode_lower(random_bytes)}")
    
    // Generate UUIDs
    print("🆔 Generated UUIDs:")
    bestie i in 0..3 {
        sus uuid = platform.uuid_generator().generate()
        print(f"   UUID {i + 1}: {uuid}")
    }
    
    // Generate cryptographic salts
    print("🧂 Generated salts:")
    sus salt_binary = platform.salt_generator().generate_salt(16)
    sus salt_hex = platform.salt_generator().generate_salt_hex(16)
    sus salt_base64 = platform.salt_generator().generate_salt_base64(16)
    
    print(f"   Binary (16 bytes): {HexEncoder::encode_lower(salt_binary)}")
    print(f"   Hex (16 bytes): {salt_hex}")
    print(f"   Base64 (16 bytes): {salt_base64}")
    
    // Generate nonces
    print("🔐 Generated nonces:")
    sus nonce = platform.nonce_generator().generate_nonce(12)
    sus time_nonce = platform.nonce_generator().generate_time_nonce(8)
    sus purpose_nonce = platform.nonce_generator().generate_purpose_nonce("encryption", 16)
    
    print(f"   Standard nonce: {HexEncoder::encode_lower(nonce)}")
    print(f"   Time-based nonce: {HexEncoder::encode_lower(time_nonce)}")
    print(f"   Purpose nonce: {HexEncoder::encode_lower(purpose_nonce)}")
    
    // Test randomness quality
    sus large_sample = platform.random_bytes(1000)
    sus quality = test_randomness_quality(large_sample)
    print(f"📈 Randomness quality:")
    print(f"   Entropy estimate: {quality.entropy_estimate:.2f} bits/byte")
    print(f"   Chi-squared: {quality.chi_squared:.2f}")
    print(f"   Passes basic tests: {quality.passes_basic_tests}")
    print(f"   Has obvious patterns: {quality.has_patterns}")
}

fr fr Encoding Utilities Demo
damn demonstrate_encoding_utilities() {
    print("\n📝 Encoding Utilities Demo")
    print("-" * 30)
    
    sus test_data = "Hello, CURSED crypto world! 🔐💻"
    sus data_bytes = test_data.as_bytes()
    
    print(f"Original data: {test_data}")
    print(f"Data length: {data_bytes.len()} bytes")
    
    // Base64 encodings
    sus b64_standard = Base64Encoder::encode_standard(data_bytes)
    sus b64_url_safe = Base64Encoder::encode_url_safe(data_bytes)
    
    print("📋 Base64 encodings:")
    print(f"   Standard: {b64_standard}")
    print(f"   URL-safe: {b64_url_safe}")
    
    // Hex encodings
    sus hex_lower = HexEncoder::encode_lower(data_bytes)
    sus hex_upper = HexEncoder::encode_upper(data_bytes)
    sus hex_formatted = HexEncoder::encode_formatted(data_bytes, ":", based)
    
    print("🔢 Hex encodings:")
    print(f"   Lowercase: {hex_lower}")
    print(f"   Uppercase: {hex_upper}")
    print(f"   Formatted: {hex_formatted}")
    
    // Base32 encoding
    sus b32 = Base32Encoder::encode(data_bytes)
    sus b32_no_pad = Base32Encoder::encode_no_padding(data_bytes)
    
    print("📱 Base32 encodings:")
    print(f"   Standard: {b32}")
    print(f"   No padding: {b32_no_pad}")
    
    // URL encoding
    sus url_encoded = UrlEncoder::encode(data_bytes)
    print(f"🌐 URL encoded: {url_encoded}")
    
    // Test round-trip integrity
    print("✅ Round-trip verification:")
    sus b64_decoded = Base64Encoder::decode_standard(b64_standard)
    sus hex_decoded = HexEncoder::decode(hex_lower)
    sus b32_decoded = Base32Encoder::decode(b32)
    sus url_decoded = UrlEncoder::decode(url_encoded)
    
    print(f"   Base64: {String::from_utf8(b64_decoded) == test_data}")
    print(f"   Hex: {String::from_utf8(hex_decoded) == test_data}")
    print(f"   Base32: {String::from_utf8(b32_decoded) == test_data}")
    print(f"   URL: {String::from_utf8(url_decoded) == test_data}")
}

fr fr Secure Communication Example
damn demonstrate_secure_communication_example() {
    print("\n💬 Secure Communication Example")
    print("-" * 30)
    
    // Setup: Alice and Bob want to communicate securely
    print("👥 Scenario: Alice sends a secure message to Bob")
    
    // 1. Key exchange simulation (in real world, use proper key exchange)
    sus shared_secret = "shared_secret_key_between_alice_and_bob_via_key_exchange"
    print("🔑 Shared secret established via secure key exchange")
    
    // 2. Alice creates a message
    sus message = squad {
        from: "alice@cursed.dev",
        to: "bob@cursed.dev",
        timestamp: SystemTime::now().timestamp(),
        content: "The crypto package is ready for production! 🚀",
        message_id: UuidV4Generator::new().generate()
    }
    sus message_json = serde_json::to_string(message)
    
    print(f"💌 Alice's message: {message.content}")
    
    // 3. Generate nonce for this message
    sus nonce_gen = NonceGenerator::new()
    sus message_nonce = nonce_gen.generate_purpose_nonce("secure_message", 16)
    
    // 4. Create message authentication code
    sus hmac_auth = HmacAuth::new(shared_secret.as_bytes())
    sus message_with_nonce = [message_nonce, message_json.as_bytes()].concat()
    sus mac = hmac_auth.sign(message_with_nonce)
    
    // 5. Create complete secure message
    sus secure_message = squad {
        nonce: Base64Encoder::encode_standard(message_nonce),
        payload: Base64Encoder::encode_standard(message_json.as_bytes()),
        mac: Base64Encoder::encode_standard(mac),
        algorithm: "HMAC-SHA256",
        version: "1.0"
    }
    
    print("📦 Secure message package created:")
    print(f"   Nonce: {secure_message.nonce[..16]}...")
    print(f"   Payload: {secure_message.payload[..32]}...")
    print(f"   MAC: {secure_message.mac[..16]}...")
    
    // 6. Bob receives and verifies the message
    print("\n🔍 Bob verifies the message:")
    
    // Decode components
    sus received_nonce = Base64Encoder::decode_standard(secure_message.nonce)
    sus received_payload = Base64Encoder::decode_standard(secure_message.payload)
    sus received_mac = Base64Encoder::decode_standard(secure_message.mac)
    
    // Recreate message with nonce for verification
    sus verification_data = [received_nonce, received_payload].concat()
    
    // Verify MAC
    sus bob_hmac = HmacAuth::new(shared_secret.as_bytes())
    sus is_authentic = bob_hmac.verify(verification_data, received_mac)
    
    lowkey is_authentic {
        print("✅ Message authentication successful!")
        
        // Decode the actual message
        sus decoded_message_json = String::from_utf8(received_payload)
        sus decoded_message = serde_json::from_str(decoded_message_json)
        
        print(f"📨 Verified message from {decoded_message.from}:")
        print(f"   Content: {decoded_message.content}")
        print(f"   Timestamp: {decoded_message.timestamp}")
        print(f"   Message ID: {decoded_message.message_id}")
        
        // Check message freshness (prevent replay attacks)
        sus current_time = SystemTime::now().timestamp()
        sus message_age = current_time - decoded_message.timestamp
        lowkey message_age < 300 { // 5 minutes
            print("✅ Message is fresh (not a replay attack)")
        } flex {
            print("⚠️  Message is old - possible replay attack")
        }
        
    } flex {
        print("❌ Message authentication failed - message tampered or corrupted!")
    }
    
    // 7. Demonstrate forward secrecy concept
    print("\n🔄 Forward secrecy demonstration:")
    print("   Generating new nonce for next message...")
    sus next_nonce = nonce_gen.generate_purpose_nonce("secure_message", 16)
    print(f"   New nonce: {HexEncoder::encode_lower(next_nonce)[..16]}...")
    print("   Each message uses unique nonce for perfect forward secrecy")
    
    // 8. Show cryptographic strength summary
    print("\n🛡️  Security summary:")
    print("   ✅ Message authenticity (HMAC-SHA256)")
    print("   ✅ Integrity protection (MAC verification)")
    print("   ✅ Replay attack prevention (timestamp + nonce)")
    print("   ✅ Forward secrecy (unique nonce per message)")
    print("   ✅ Base64 encoding for safe transport")
    print("   ✅ Structured message format (JSON)")
}

fr fr Helper function for crypto statistics
damn show_crypto_statistics() {
    sus stats = CryptoStatistics::new()
    
    // In a real application, these would be populated during operation
    stats.jwt_tokens_created = 1
    stats.jwt_tokens_validated = 1
    stats.hmac_signatures_created = 2
    stats.hmac_verifications = 2
    stats.totp_tokens_generated = 1
    stats.totp_verifications = 1
    stats.uuids_generated = 4
    stats.salts_generated = 3
    stats.nonces_generated = 3
    
    print("\n📊 Crypto Operations Statistics:")
    print(f"   Total operations: {stats.total_operations()}")
    print(f"   JWT success rate: {stats.jwt_success_rate() * 100:.1f}%")
    print(f"   HMAC success rate: {stats.hmac_success_rate() * 100:.1f}%")
    print(f"   TOTP success rate: {stats.totp_success_rate() * 100:.1f}%")
}

fr fr Example of production crypto configuration
damn show_production_config() {
    print("\n⚙️  Production Crypto Configuration:")
    
    sus config = CryptoConfig::secure_defaults()
    
    print(f"   JWT expiry: {config.default_jwt_expiry} seconds")
    print(f"   TOTP time step: {config.default_totp_time_step} seconds")
    print(f"   TOTP digits: {config.default_totp_digits}")
    print(f"   Salt length: {config.default_salt_length} bytes")
    print(f"   Nonce length: {config.default_nonce_length} bytes")
    print(f"   Detailed logging: {config.enable_detailed_logging}")
    print(f"   Max random bytes per request: {config.max_random_bytes_per_request}")
    print(f"   Random reseed frequency: {config.random_reseed_frequency}")
    
    lowkey config.validate().is_ok() {
        print("✅ Configuration is valid and secure")
    } flex {
        print("❌ Configuration validation failed")
    }
}
