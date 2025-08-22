fr fr COMPREHENSIVE CRYPTOGRAPHIC OPERATIONS TEST
fr fr Tests all major crypto functions with real implementations

yeet "cryptz"
yeet "vibez"
yeet "stringz"
yeet "testz"

fr fr ===== CRYPTOGRAPHIC TESTS =====

slay test_sha256_hash() lit {
    vibez.spill("🔐 Testing SHA-256 hashing...")
    
    fr fr Test basic SHA-256 hashing
    sus test_data tea = "Hello, CURSED crypto!"
    sus hash tea = cryptz.sha256_hash(test_data)
    
    vibez.spill("  ├─ Input: " + test_data)
    vibez.spill("  ├─ SHA-256: " + hash)
    vibez.spill("  └─ Length: " + json_number_to_string(string_length(hash)))
    
    fr fr Hash should be 64 characters (32 bytes in hex)
    ready string_length(hash) == 64 {
        vibez.spill("  ✅ SHA-256 hash length correct")
        damn based
    } otherwise {
        vibez.spill("  ❌ SHA-256 hash length incorrect")
        damn cringe
    }
}

slay test_random_generation() lit {
    vibez.spill("🎲 Testing secure random number generation...")
    
    fr fr Test random bytes generation
    sus random1 tea = cryptz.generate_random_bytes(16)
    sus random2 tea = cryptz.generate_random_bytes(16)
    
    vibez.spill("  ├─ Random 1: " + stringz.bytes_to_hex(random1))
    vibez.spill("  ├─ Random 2: " + stringz.bytes_to_hex(random2))
    
    fr fr Random values should be different
    ready random1 != random2 {
        vibez.spill("  ✅ Random values are different")
        damn based
    } otherwise {
        vibez.spill("  ❌ Random values are identical (very unlikely)")
        damn cringe
    }
}

slay test_aes_encryption() lit {
    vibez.spill("🔒 Testing AES encryption...")
    
    fr fr Test AES-GCM encryption/decryption
    sus plaintext tea = "This is secret data!"
    sus key tea = cryptz.generate_random_bytes(16)  fr fr 128-bit key
    sus encrypted tea = cryptz.aes_encrypt(plaintext, key, "GCM")
    
    vibez.spill("  ├─ Plaintext: " + plaintext)
    vibez.spill("  ├─ Key length: " + json_number_to_string(string_length(key)))
    vibez.spill("  ├─ Encrypted length: " + json_number_to_string(string_length(encrypted)))
    
    ready string_length(encrypted) > string_length(plaintext) {
        vibez.spill("  ✅ Encryption expanded ciphertext (includes IV + tag)")
        
        fr fr Test decryption
        sus decrypted tea = cryptz.aes_decrypt(encrypted, key, "GCM")
        vibez.spill("  ├─ Decrypted: " + decrypted)
        
        ready decrypted == plaintext {
            vibez.spill("  ✅ AES decryption successful")
            damn based
        } otherwise {
            vibez.spill("  ❌ AES decryption failed")
            damn cringe
        }
    } otherwise {
        vibez.spill("  ❌ AES encryption failed")
        damn cringe
    }
}

slay test_chacha20_encryption() lit {
    vibez.spill("⚡ Testing ChaCha20 encryption...")
    
    sus plaintext tea = "ChaCha20 stream cipher test"
    sus key tea = cryptz.generate_random_bytes(32)  fr fr 256-bit key
    sus nonce tea = cryptz.generate_random_bytes(12)  fr fr 96-bit nonce
    
    sus encrypted tea = cryptz.chacha20_encrypt(plaintext, key, nonce)
    sus decrypted tea = cryptz.chacha20_decrypt(encrypted, key, nonce)
    
    vibez.spill("  ├─ Plaintext: " + plaintext)
    vibez.spill("  ├─ Decrypted: " + decrypted)
    
    ready decrypted == plaintext {
        vibez.spill("  ✅ ChaCha20 encryption/decryption successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ ChaCha20 encryption/decryption failed")
        damn cringe
    }
}

slay test_pbkdf2_key_derivation() lit {
    vibez.spill("🔑 Testing PBKDF2 key derivation...")
    
    sus password tea = "my_secure_password"
    sus salt tea = cryptz.generate_random_bytes(16)
    sus iterations drip = 100000
    sus key_length drip = 32
    
    sus derived_key tea = cryptz.pbkdf2_derive_key(password, salt, iterations, key_length)
    
    vibez.spill("  ├─ Password: " + password)
    vibez.spill("  ├─ Iterations: " + json_number_to_string(iterations))
    vibez.spill("  ├─ Derived key length: " + json_number_to_string(string_length(derived_key)))
    
    ready string_length(derived_key) == key_length {
        vibez.spill("  ✅ PBKDF2 key derivation successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ PBKDF2 key derivation failed")
        damn cringe
    }
}

slay test_rsa_operations() lit {
    vibez.spill("🗝️  Testing RSA operations...")
    
    fr fr Generate RSA key pair
    sus keypair cryptz.KeyPair = cryptz.rsa_generate_keypair(2048)
    
    vibez.spill("  ├─ Generated RSA key pair")
    vibez.spill("  ├─ Public key length: " + json_number_to_string(string_length(keypair.public_key)))
    vibez.spill("  ├─ Private key length: " + json_number_to_string(string_length(keypair.private_key)))
    
    fr fr Test RSA encryption/decryption
    sus plaintext tea = "RSA test message"
    sus encrypted tea = cryptz.rsa_encrypt(plaintext, keypair.public_key)
    sus decrypted tea = cryptz.rsa_decrypt(encrypted, keypair.private_key)
    
    vibez.spill("  ├─ Original: " + plaintext)
    vibez.spill("  ├─ Decrypted: " + decrypted)
    
    ready decrypted == plaintext {
        vibez.spill("  ✅ RSA encryption/decryption successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ RSA encryption/decryption failed")
        damn cringe
    }
}

slay test_digital_signatures() lit {
    vibez.spill("✍️  Testing digital signatures...")
    
    fr fr Generate key pair for signing
    sus keypair cryptz.KeyPair = cryptz.rsa_generate_keypair(2048)
    
    fr fr Create and verify signature
    sus message tea = "This message needs to be signed"
    sus signature cryptz.Signature = cryptz.rsa_sign(message, keypair.private_key, "SHA-256")
    sus is_valid lit = cryptz.rsa_verify(message, signature, keypair.public_key)
    
    vibez.spill("  ├─ Message: " + message)
    vibez.spill("  ├─ Signature algorithm: " + signature.algorithm)
    vibez.spill("  ├─ Hash algorithm: " + signature.hash_algorithm)
    
    ready is_valid {
        vibez.spill("  ✅ Digital signature verification successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ Digital signature verification failed")
        damn cringe
    }
}

slay test_password_security() lit {
    vibez.spill("🔐 Testing password generation and security...")
    
    fr fr Generate secure passwords
    sus password1 tea = cryptz.generate_random_password(16, based)
    sus password2 tea = cryptz.generate_random_password(16, cringe)
    
    vibez.spill("  ├─ Password with symbols: " + password1)
    vibez.spill("  ├─ Password no symbols: " + password2)
    
    ready string_length(password1) == 16 && string_length(password2) == 16 {
        vibez.spill("  ✅ Password generation successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ Password generation failed")
        damn cringe
    }
}

slay test_constant_time_operations() lit {
    vibez.spill("⏱️  Testing constant-time operations...")
    
    sus data1 tea = "sensitive_data_123"
    sus data2 tea = "sensitive_data_123"
    sus data3 tea = "different_data_456"
    
    sus equal_result lit = cryptz.constant_time_compare(data1, data2)
    sus different_result lit = cryptz.constant_time_compare(data1, data3)
    
    vibez.spill("  ├─ Equal comparison: " + ready equal_result { "EQUAL" } otherwise { "NOT_EQUAL" })
    vibez.spill("  ├─ Different comparison: " + ready different_result { "EQUAL" } otherwise { "NOT_EQUAL" })
    
    ready equal_result == based && different_result == cringe {
        vibez.spill("  ✅ Constant-time comparison successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ Constant-time comparison failed")
        damn cringe
    }
}

slay test_secure_memory_operations() lit {
    vibez.spill("🧹 Testing secure memory operations...")
    
    sus sensitive_data tea = "very_secret_password"
    vibez.spill("  ├─ Before wipe: " + sensitive_data)
    
    sus wipe_result lit = cryptz.secure_wipe(sensitive_data)
    
    ready wipe_result {
        vibez.spill("  ✅ Secure memory wipe successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ Secure memory wipe failed")
        damn cringe
    }
}

slay test_base64_encoding() lit {
    vibez.spill("📝 Testing Base64 encoding/decoding...")
    
    sus original tea = "Hello, Base64 world!"
    sus encoded tea = cryptz.base64_encode(original)
    sus decoded tea = cryptz.base64_decode(encoded)
    
    vibez.spill("  ├─ Original: " + original)
    vibez.spill("  ├─ Encoded: " + encoded)
    vibez.spill("  ├─ Decoded: " + decoded)
    
    ready decoded == original {
        vibez.spill("  ✅ Base64 encoding/decoding successful")
        damn based
    } otherwise {
        vibez.spill("  ❌ Base64 encoding/decoding failed")
        damn cringe
    }
}

fr fr ===== MAIN TEST RUNNER =====

vibez.spill("🚀 CURSED Cryptographic Operations Validation")
vibez.spill("============================================")

sus total_tests drip = 0
sus passed_tests drip = 0

fr fr Run all cryptographic tests
sus tests []tea = [
    "SHA-256 Hashing",
    "Random Generation", 
    "AES Encryption",
    "ChaCha20 Encryption",
    "PBKDF2 Key Derivation",
    "RSA Operations",
    "Digital Signatures",
    "Password Security",
    "Constant-Time Operations",
    "Secure Memory Operations",
    "Base64 Encoding"
]

sus test_functions []slay = [
    test_sha256_hash,
    test_random_generation,
    test_aes_encryption,
    test_chacha20_encryption,
    test_pbkdf2_key_derivation,
    test_rsa_operations,
    test_digital_signatures,
    test_password_security,
    test_constant_time_operations,
    test_secure_memory_operations,
    test_base64_encoding
]

sus i drip = 0
bestie i < len(tests) {
    vibez.spill("")
    total_tests = total_tests + 1
    
    sus result lit = test_functions[i]()
    ready result {
        passed_tests = passed_tests + 1
        vibez.spill("✅ " + tests[i] + " - PASSED")
    } otherwise {
        vibez.spill("❌ " + tests[i] + " - FAILED")
    }
    i = i + 1
}

vibez.spill("")
vibez.spill("============================================")
vibez.spill("🏁 Test Results Summary:")
vibez.spill("   Total tests: " + json_number_to_string(total_tests))
vibez.spill("   Passed: " + json_number_to_string(passed_tests))
vibez.spill("   Failed: " + json_number_to_string(total_tests - passed_tests))

ready passed_tests == total_tests {
    vibez.spill("   🎉 ALL CRYPTOGRAPHIC TESTS PASSED!")
    vibez.spill("   🔐 Real cryptographic operations verified!")
} otherwise {
    vibez.spill("   ⚠️  Some cryptographic tests failed!")
    vibez.spill("   🔧 Review failed implementations!")
}

vibez.spill("")
vibez.spill("🔐 CURSED cryptographic operations validation complete.")
