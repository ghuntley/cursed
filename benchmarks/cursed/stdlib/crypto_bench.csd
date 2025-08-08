fr fr Cryptography Performance Benchmark Suite

yeet "benchz"
yeet "testz"
yeet "cryptz"
yeet "sha256z"

slay benchmark_hashing_algorithms() lit {
    benchmark_suite_start("Hashing Algorithms")
    
    fr fr SHA-256 benchmarks
    benchmark_precise("SHA-256 Empty String", slay() {
        sus data tea = ""
        sus hash tea = sha256(data)
    })
    
    benchmark_precise("SHA-256 Short Message", slay() {
        sus data tea = "Hello, world!"
        sus hash tea = sha256(data)
    })
    
    benchmark_precise("SHA-256 Medium Message", slay() {
        sus data tea = "This is a medium length message for SHA-256 hashing performance testing"
        sus hash tea = sha256(data)
    })
    
    benchmark_precise("SHA-256 Long Message", slay() {
        sus data tea = "This is a much longer message for SHA-256 hashing performance testing. It contains multiple sentences and should provide a good test of the hashing algorithm's performance with larger input data that exceeds typical short message sizes."
        sus hash tea = sha256(data)
    })
    
    benchmark_precise("SHA-256 Very Long Message", slay() {
        fr fr Create a very long message
        sus data tea = ""
        sus i normie = 0
        bestie (i < 100) {
            data = data + "This is test data for performance measurement. "
            i = i + 1
        }
        sus hash tea = sha256(data)
    })
    
    fr fr Multiple hashing operations
    benchmark_precise("SHA-256 Multiple Small", slay() {
        sus i normie = 0
        bestie (i < 10) {
            sus data tea = "message" + i
            sus hash tea = sha256(data)
            i = i + 1
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_symmetric_encryption() lit {
    benchmark_suite_start("Symmetric Encryption")
    
    fr fr AES encryption benchmarks
    benchmark_precise("AES Encrypt Small", slay() {
        sus key tea = "mysecretkey12345"
        sus plaintext tea = "Hello, world!"
        sus ciphertext tea = aes_encrypt(plaintext, key)
    })
    
    benchmark_precise("AES Decrypt Small", slay() {
        sus key tea = "mysecretkey12345"
        sus ciphertext tea = "encrypted_data_here"
        sus plaintext tea = aes_decrypt(ciphertext, key)
    })
    
    benchmark_precise("AES Encrypt Medium", slay() {
        sus key tea = "mysecretkey12345"
        sus plaintext tea = "This is a medium sized message for AES encryption testing"
        sus ciphertext tea = aes_encrypt(plaintext, key)
    })
    
    benchmark_precise("AES Decrypt Medium", slay() {
        sus key tea = "mysecretkey12345"
        sus plaintext tea = "This is a medium sized message for AES encryption testing"
        sus ciphertext tea = aes_encrypt(plaintext, key)
        sus decrypted tea = aes_decrypt(ciphertext, key)
    })
    
    benchmark_precise("AES Encrypt Large", slay() {
        sus key tea = "mysecretkey12345"
        sus plaintext tea = ""
        sus i normie = 0
        bestie (i < 50) {
            plaintext = plaintext + "This is test data for encryption performance measurement. "
            i = i + 1
        }
        sus ciphertext tea = aes_encrypt(plaintext, key)
    })
    
    benchmark_precise("AES Round Trip", slay() {
        sus key tea = "mysecretkey12345"
        sus original tea = "Secret message for round trip test"
        sus encrypted tea = aes_encrypt(original, key)
        sus decrypted tea = aes_decrypt(encrypted, key)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_digital_signatures() lit {
    benchmark_suite_start("Digital Signatures")
    
    fr fr ECDSA benchmarks
    benchmark_precise("Generate ECDSA Key Pair", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
    })
    
    benchmark_precise("ECDSA Sign Small Message", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
        sus message tea = "Hello, world!"
        sus signature tea = ecdsa_sign(message, keys.private_key)
    })
    
    benchmark_precise("ECDSA Verify Small Message", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
        sus message tea = "Hello, world!"
        sus signature tea = ecdsa_sign(message, keys.private_key)
        sus valid lit = ecdsa_verify(message, signature, keys.public_key)
    })
    
    benchmark_precise("ECDSA Sign Medium Message", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
        sus message tea = "This is a medium sized message for ECDSA signature testing with more content"
        sus signature tea = ecdsa_sign(message, keys.private_key)
    })
    
    benchmark_precise("ECDSA Verify Medium Message", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
        sus message tea = "This is a medium sized message for ECDSA signature testing with more content"
        sus signature tea = ecdsa_sign(message, keys.private_key)
        sus valid lit = ecdsa_verify(message, signature, keys.public_key)
    })
    
    benchmark_precise("ECDSA Sign Large Message", slay() {
        sus keys KeyPair = ecdsa_generate_keypair()
        sus message tea = ""
        sus i normie = 0
        bestie (i < 30) {
            message = message + "This is test data for digital signature performance measurement. "
            i = i + 1
        }
        sus signature tea = ecdsa_sign(message, keys.private_key)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_key_derivation() lit {
    benchmark_suite_start("Key Derivation")
    
    fr fr PBKDF2 benchmarks
    benchmark_precise("PBKDF2 1000 iterations", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 1000
        sus key tea = pbkdf2(password, salt, iterations, 32)
    })
    
    benchmark_precise("PBKDF2 10000 iterations", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 10000
        sus key tea = pbkdf2(password, salt, iterations, 32)
    })
    
    benchmark_precise("PBKDF2 100000 iterations", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 100000
        sus key tea = pbkdf2(password, salt, iterations, 32)
    })
    
    fr fr Different key lengths
    benchmark_precise("PBKDF2 16-byte key", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 10000
        sus key tea = pbkdf2(password, salt, iterations, 16)
    })
    
    benchmark_precise("PBKDF2 32-byte key", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 10000
        sus key tea = pbkdf2(password, salt, iterations, 32)
    })
    
    benchmark_precise("PBKDF2 64-byte key", slay() {
        sus password tea = "mypassword"
        sus salt tea = "randomsalt"
        sus iterations normie = 10000
        sus key tea = pbkdf2(password, salt, iterations, 64)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_random_generation() lit {
    benchmark_suite_start("Random Number Generation")
    
    benchmark_precise("Generate 16 Random Bytes", slay() {
        sus random_data tea = crypto_random_bytes(16)
    })
    
    benchmark_precise("Generate 32 Random Bytes", slay() {
        sus random_data tea = crypto_random_bytes(32)
    })
    
    benchmark_precise("Generate 64 Random Bytes", slay() {
        sus random_data tea = crypto_random_bytes(64)
    })
    
    benchmark_precise("Generate 256 Random Bytes", slay() {
        sus random_data tea = crypto_random_bytes(256)
    })
    
    benchmark_precise("Generate 1024 Random Bytes", slay() {
        sus random_data tea = crypto_random_bytes(1024)
    })
    
    fr fr Random number generation in loop
    benchmark_precise("Multiple Random Generation", slay() {
        sus i normie = 0
        bestie (i < 10) {
            sus random_data tea = crypto_random_bytes(32)
            i = i + 1
        }
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_constant_time_operations() lit {
    benchmark_suite_start("Constant Time Operations")
    
    fr fr Constant time comparison
    benchmark_precise("Constant Time Equal", slay() {
        sus a tea = "secret_data_1234"
        sus b tea = "secret_data_1234"
        sus equal lit = constant_time_compare(a, b)
    })
    
    benchmark_precise("Constant Time Not Equal", slay() {
        sus a tea = "secret_data_1234"
        sus b tea = "secret_data_5678"
        sus equal lit = constant_time_compare(a, b)
    })
    
    benchmark_precise("Constant Time Different Length", slay() {
        sus a tea = "secret_data_1234"
        sus b tea = "secret"
        sus equal lit = constant_time_compare(a, b)
    })
    
    benchmark_precise("Constant Time Long Strings", slay() {
        sus a tea = "this_is_a_very_long_secret_string_for_constant_time_comparison_testing"
        sus b tea = "this_is_a_very_long_secret_string_for_constant_time_comparison_testing"
        sus equal lit = constant_time_compare(a, b)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_mac_authentication() lit {
    benchmark_suite_start("Message Authentication")
    
    fr fr HMAC benchmarks
    benchmark_precise("HMAC-SHA256 Small", slay() {
        sus key tea = "secret_key"
        sus message tea = "Hello, world!"
        sus mac tea = hmac_sha256(message, key)
    })
    
    benchmark_precise("HMAC-SHA256 Medium", slay() {
        sus key tea = "secret_key"
        sus message tea = "This is a medium sized message for HMAC testing"
        sus mac tea = hmac_sha256(message, key)
    })
    
    benchmark_precise("HMAC-SHA256 Large", slay() {
        sus key tea = "secret_key"
        sus message tea = ""
        sus i normie = 0
        bestie (i < 50) {
            message = message + "This is test data for HMAC performance measurement. "
            i = i + 1
        }
        sus mac tea = hmac_sha256(message, key)
    })
    
    benchmark_precise("HMAC Verify Correct", slay() {
        sus key tea = "secret_key"
        sus message tea = "Hello, world!"
        sus mac tea = hmac_sha256(message, key)
        sus valid lit = hmac_verify(message, mac, key)
    })
    
    benchmark_precise("HMAC Verify Incorrect", slay() {
        sus key tea = "secret_key"
        sus message tea = "Hello, world!"
        sus wrong_mac tea = "incorrect_mac_value"
        sus valid lit = hmac_verify(message, wrong_mac, key)
    })
    
    generate_benchmark_report()
    damn based
}

slay benchmark_encryption_modes() lit {
    benchmark_suite_start("Encryption Modes")
    
    fr fr Different AES modes
    benchmark_precise("AES-CBC Encrypt", slay() {
        sus key tea = "mysecretkey12345"
        sus iv tea = "initialization16"
        sus plaintext tea = "This is test data for AES-CBC encryption"
        sus ciphertext tea = aes_cbc_encrypt(plaintext, key, iv)
    })
    
    benchmark_precise("AES-CBC Decrypt", slay() {
        sus key tea = "mysecretkey12345"
        sus iv tea = "initialization16"
        sus plaintext tea = "This is test data for AES-CBC encryption"
        sus ciphertext tea = aes_cbc_encrypt(plaintext, key, iv)
        sus decrypted tea = aes_cbc_decrypt(ciphertext, key, iv)
    })
    
    benchmark_precise("AES-GCM Encrypt", slay() {
        sus key tea = "mysecretkey12345"
        sus nonce tea = "uniquenonce1"
        sus plaintext tea = "This is test data for AES-GCM encryption"
        sus result EncryptionResult = aes_gcm_encrypt(plaintext, key, nonce)
    })
    
    benchmark_precise("AES-GCM Decrypt", slay() {
        sus key tea = "mysecretkey12345"
        sus nonce tea = "uniquenonce1"
        sus plaintext tea = "This is test data for AES-GCM encryption"
        sus encrypted EncryptionResult = aes_gcm_encrypt(plaintext, key, nonce)
        sus decrypted tea = aes_gcm_decrypt(encrypted.ciphertext, key, nonce, encrypted.tag)
    })
    
    generate_benchmark_report()
    damn based
}

slay run_all_crypto_benchmarks() lit {
    vibez.spill("🚀 Running All Cryptography Benchmarks")
    vibez.spill("═══════════════════════════════════════════")
    
    benchmark_hashing_algorithms()
    benchmark_symmetric_encryption()
    benchmark_digital_signatures()
    benchmark_key_derivation()
    benchmark_random_generation()
    benchmark_constant_time_operations()
    benchmark_mac_authentication()
    benchmark_encryption_modes()
    
    vibez.spill("\n✅ All cryptography benchmarks completed!")
    
    fr fr Security performance analysis
    compare_benchmarks("SHA-256 Short Message", "SHA-256 Very Long Message")
    compare_benchmarks("AES Encrypt Small", "AES Encrypt Large")
    compare_benchmarks("ECDSA Sign Small Message", "ECDSA Sign Large Message")
    compare_benchmarks("PBKDF2 1000 iterations", "PBKDF2 100000 iterations")
    
    vibez.spill("\n🔒 Security Notes:")
    vibez.spill("- All operations should run in constant time for security")
    vibez.spill("- Key generation should use cryptographically secure random sources")
    vibez.spill("- Memory should be securely cleared after cryptographic operations")
    
    damn based
}

fr fr Run benchmarks if this file is executed directly
run_all_crypto_benchmarks()
