fr fr Comprehensive crypto test that works with current parser
yeet "vibez"

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("String equality: " + actual)
    } highkey {
        test_fail("String mismatch: got " + actual + ", expected " + expected)
    }
}

slay assert_true(condition lit) {
    lowkey condition == based {
        test_pass("Condition is true")
    } highkey {
        test_fail("Condition is false")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("Integer equality: " + tea(actual))
    } highkey {
        test_fail("Integer mismatch: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay test_crypto_hash_functions() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing hash functions...")
    
    fr fr Test SHA256
    sus sha256_result tea = crypto_sha256("hello world")
    assert_eq_string(sha256_result, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9")
    
    fr fr Test SHA512
    sus sha512_result tea = crypto_sha512("hello world")
    assert_eq_string(sha512_result, "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f")
    
    fr fr Test MD5
    sus md5_result tea = crypto_md5("hello world")
    assert_eq_string(md5_result, "5d41402abc4b2a76b9719d911017c592")
    
    fr fr Test BLAKE3
    sus blake3_result tea = crypto_blake3("hello world")
    assert_true(blake3_result != "")
}

slay test_crypto_encoding() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing encoding functions...")
    
    fr fr Test Base64 encoding/decoding
    sus original tea = "hello world"
    sus encoded tea = crypto_base64_encode(original)
    assert_eq_string(encoded, "aGVsbG8gd29ybGQ=")
    
    sus decoded tea = crypto_base64_decode(encoded)
    assert_eq_string(decoded, original)
    
    fr fr Test hex encoding/decoding  
    sus test_bytes = [72, 101, 108, 108, 111]
    sus hex_encoded tea = crypto_hex_encode(test_bytes)
    assert_eq_string(hex_encoded, "48656c6c6f")
    
    sus hex_decoded = crypto_hex_decode(hex_encoded)
    assert_eq_int(hex_decoded[0], 72)
    assert_eq_int(hex_decoded[1], 101)
}

slay test_crypto_random() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing random functions...")
    
    fr fr Test random integer
    sus rand_int normie = crypto_random_int(1, 100)
    assert_true(rand_int >= 1)
    assert_true(rand_int <= 100)
    
    fr fr Test random string
    sus rand_str tea = crypto_random_string(10)
    assert_eq_int(string_len(rand_str), 10)
    
    fr fr Test random bytes
    sus rand_bytes = crypto_random_bytes(5)
    assert_eq_int(len(rand_bytes), 5)
    
    fr fr Test secure random
    sus rand_float meal = crypto_secure_random()
    assert_true(rand_float >= 0.0)
    assert_true(rand_float <= 1.0)
}

slay test_crypto_hmac() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing HMAC functions...")
    
    fr fr Test HMAC-SHA256
    sus message tea = "hello world"
    sus key tea = "secret-key"
    sus hmac_result tea = crypto_hmac_sha256(message, key)
    assert_true(hmac_result != "")
    assert_eq_int(string_len(hmac_result), 64)
    
    fr fr Test HMAC-SHA512
    sus hmac512_result tea = crypto_hmac_sha512(message, key)
    assert_true(hmac512_result != "")
    assert_eq_int(string_len(hmac512_result), 128)
}

slay test_crypto_encryption() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing encryption functions...")
    
    fr fr Test AES encryption/decryption
    sus plaintext tea = "This is a secret message"
    sus key tea = "my-secret-key-32-bytes-long-test"
    
    sus encrypted tea = crypto_aes_encrypt(plaintext, key)
    assert_true(encrypted != "")
    assert_true(encrypted != plaintext)
    
    sus decrypted tea = crypto_aes_decrypt(encrypted, key)
    assert_eq_string(decrypted, plaintext)
}

slay test_crypto_utilities() {
    test_count = test_count + 1
    vibez.spill("🔐 Testing utility functions...")
    
    fr fr Test salt generation
    sus salt1 tea = crypto_generate_salt(16)
    sus salt2 tea = crypto_generate_salt(16)
    assert_eq_int(string_len(salt1), 32)
    assert_eq_int(string_len(salt2), 32)
    assert_true(salt1 != salt2)
    
    fr fr Test constant time comparison
    sus str1 tea = "hello world"
    sus str2 tea = "hello world"
    sus str3 tea = "different"
    
    assert_true(crypto_constant_time_eq(str1, str2))
    assert_true(crypto_constant_time_eq(str1, str3) == cap)
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== CRYPTO TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed assertions: " + tea(test_passed))
    vibez.spill("Failed assertions: " + tea(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL CRYPTO TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some crypto tests failed")
    }
}

slay run_all_crypto_tests() {
    vibez.spill("🔐 Running comprehensive CURSED crypto tests...")
    vibez.spill("===============================================")
    
    test_crypto_hash_functions()
    test_crypto_encoding()
    test_crypto_random()
    test_crypto_hmac()
    test_crypto_encryption()
    test_crypto_utilities()
    
    print_test_summary()
}

run_all_crypto_tests()
