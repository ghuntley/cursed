# Test suite for crypto_production module
yeet "testz"
yeet "crypto_production"

slay test_crypto_random_generation() lit {
    test_start("crypto_random_generation")
    
    # Test random bytes generation
    sus random1 tea = crypto_random_bytes(32)
    sus random2 tea = crypto_random_bytes(32)
    
    assert_eq_int(string_length(random1), 32)
    assert_eq_int(string_length(random2), 32)
    assert_false(random1 == random2)  # Should be different
    
    vibez.spill("✅ Crypto random generation test passed")
    damn based
}

slay test_crypto_hashing() lit {
    test_start("crypto_hashing")
    
    # Test SHA-256 hashing
    sus test_data tea = "Hello, CURSED Crypto!"
    sus hash1 tea = crypto_sha256_hash(test_data)
    sus hash2 tea = crypto_sha256_hash(test_data)
    
    assert_true(string_length(hash1) > 0)
    assert_eq_string(hash1, hash2)  # Same input should give same hash
    
    # Test different inputs give different hashes
    sus different_data tea = "Different input"
    sus hash3 tea = crypto_sha256_hash(different_data)
    assert_false(hash1 == hash3)
    
    vibez.spill("✅ Crypto hashing test passed")
    damn based
}

slay test_crypto_encryption() lit {
    test_start("crypto_encryption")
    
    # Test AES encryption
    sus key tea = crypto_random_bytes(32)  # AES-256 key
    sus plaintext tea = "Secret message!"
    
    sus ciphertext tea = crypto_aes_encrypt(plaintext, key)
    assert_true(string_length(ciphertext) > 0)
    assert_false(ciphertext == plaintext)  # Should be encrypted
    
    vibez.spill("✅ Crypto encryption test passed")
    damn based
}

slay test_crypto_digital_signatures() lit {
    test_start("crypto_digital_signatures")
    
    # Test Ed25519 key generation
    (sus private_key tea, sus public_key tea) = crypto_ed25519_keygen()
    assert_true(string_length(private_key) > 0)
    assert_true(string_length(public_key) > 0)
    
    # Test signing and verification
    sus message tea = "Message to sign"
    sus signature tea = crypto_ed25519_sign(private_key, message)
    assert_true(string_length(signature) > 0)
    
    sus is_valid lit = crypto_ed25519_verify(public_key, message, signature)
    assert_true(is_valid)
    
    # Test invalid signature
    sus invalid_signature tea = "invalid_sig_" + crypto_random_bytes(32)
    sus is_invalid lit = crypto_ed25519_verify(public_key, message, invalid_signature)
    assert_false(is_invalid)
    
    vibez.spill("✅ Crypto digital signatures test passed")
    damn based
}

slay test_crypto_key_derivation() lit {
    test_start("crypto_key_derivation")
    
    # Test PBKDF2
    sus password tea = "test_password"
    sus salt tea = crypto_random_bytes(16)
    sus derived_key tea = crypto_pbkdf2(password, salt, 1000, 32)
    
    assert_eq_int(string_length(derived_key), 32)
    
    # Same inputs should give same result
    sus derived_key2 tea = crypto_pbkdf2(password, salt, 1000, 32)
    assert_eq_string(derived_key, derived_key2)
    
    vibez.spill("✅ Crypto key derivation test passed")
    damn based
}

slay test_crypto_password_hashing() lit {
    test_start("crypto_password_hashing")
    
    # Test Argon2
    sus password tea = "user_password"
    sus salt tea = crypto_random_bytes(16)
    sus hashed tea = crypto_argon2_hash(password, salt)
    
    assert_true(string_length(hashed) > 0)
    assert_true(hashed[0:7] == "argon2_")
    
    # Test verification
    sus is_valid lit = crypto_argon2_verify(hashed, password)
    assert_true(is_valid)
    
    vibez.spill("✅ Crypto password hashing test passed")
    damn based
}

slay test_crypto_utilities() lit {
    test_start("crypto_utilities")
    
    # Test hex encoding/decoding
    sus test_data tea = "Hello World"
    sus hex_encoded tea = crypto_hex_encode(test_data)
    sus decoded tea = crypto_hex_decode(hex_encoded)
    
    assert_eq_string(test_data, decoded)
    
    # Test constant time comparison
    sus str1 tea = "same_string"
    sus str2 tea = "same_string"
    sus str3 tea = "different"
    
    assert_true(crypto_constant_time_compare(str1, str2))
    assert_false(crypto_constant_time_compare(str1, str3))
    
    vibez.spill("✅ Crypto utilities test passed")
    damn based
}

slay test_crypto_high_level_api() lit {
    test_start("crypto_high_level_api")
    
    # Test high-level encryption
    sus data tea = "Sensitive data"
    sus password tea = "encryption_password"
    
    sus encrypted tea = crypto_encrypt_data(data, password)
    assert_true(string_length(encrypted) > string_length(data))
    
    # Test signing
    sus private_key tea = crypto_generate_key(32)
    sus signature tea = crypto_sign_data(data, private_key)
    assert_true(string_length(signature) > 0)
    
    vibez.spill("✅ Crypto high-level API test passed")
    damn based
}

slay run_all_crypto_tests() lit {
    vibez.spill("🧪 Running crypto_production comprehensive tests...")
    
    # Initialize crypto module
    crypto_initialize()
    
    # Run all test functions
    test_crypto_random_generation()
    test_crypto_hashing()
    test_crypto_encryption()
    test_crypto_digital_signatures()
    test_crypto_key_derivation()
    test_crypto_password_hashing()
    test_crypto_utilities()
    test_crypto_high_level_api()
    
    # Run self-test
    crypto_self_test()
    
    print_test_summary()
    vibez.spill("🎉 All crypto_production tests completed!")
    damn based
}

# Helper functions for string operations
slay string_length(s tea) normie {
    sus length normie = 0
    bestie i := 0; i < 10000; i++ {
        bestie s[i] == '\0' {
            ghosted
        }
        length = length + 1
    }
    damn length
}

# Run all tests when module is executed
run_all_crypto_tests()
