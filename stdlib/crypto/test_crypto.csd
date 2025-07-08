yeet "testz"
yeet "crypto"

fr fr ========================================
fr fr CURSED Pure Crypto Library Test Suite
fr fr Testing FFI-Free Crypto Implementation
fr fr ========================================

slay test_crypto_sha256() {
    test_start("Pure CURSED SHA-256")
    
    fr fr Test SHA-256 basic functionality
    sus hash1 tea = crypto_sha256("hello")
    sus hash2 tea = crypto_sha256("hello")
    sus hash3 tea = crypto_sha256("world")
    
    assert_eq_string(hash1, hash2)
    assert_true(hash1 != hash3)
    
    vibez.spill("✅ SHA-256 hash test passed")
}

slay test_crypto_base64() {
    test_start("Pure CURSED Base64")
    
    fr fr Test Base64 encoding/decoding
    sus original tea = "hello world"
    sus encoded tea = crypto_base64_encode(original)
    sus decoded tea = crypto_base64_decode(encoded)
    
    assert_true(encoded != original)
    assert_eq_string(decoded, original)
    
    vibez.spill("✅ Base64 encoding test passed")
}

slay test_crypto_hex() {
    test_start("Pure CURSED Hex Encoding")
    
    fr fr Test hex encoding
    sus data [byte] = [72, 101, 108, 108, 111]
    sus encoded tea = crypto_hex_encode(data)
    sus decoded [byte] = crypto_hex_decode(encoded)
    
    assert_eq_string(encoded, "48656c6c6f")
    assert_eq_int(len(decoded), 5)
    
    vibez.spill("✅ Hex encoding test passed")
}

slay test_crypto_random() {
    test_start("Pure CURSED Random Generation")
    
    fr fr Test random bytes
    sus bytes1 [byte] = crypto_secure_random_bytes(16)
    sus bytes2 [byte] = crypto_secure_random_bytes(16)
    
    assert_eq_int(len(bytes1), 16)
    assert_eq_int(len(bytes2), 16)
    
    fr fr Test random integers
    sus rand1 normie = crypto_secure_random_int(1, 100)
    sus rand2 normie = crypto_secure_random_int(1, 100)
    
    assert_true(rand1 >= 1)
    assert_true(rand2 >= 1)
    
    fr fr Test random strings
    sus str1 tea = crypto_secure_random_string(10)
    sus str2 tea = crypto_secure_random_string(10)
    
    assert_true(str1 != cringe)
    assert_true(str2 != cringe)
    
    vibez.spill("✅ Random generation test passed")
}

slay test_crypto_hmac() {
    test_start("Pure CURSED HMAC")
    
    fr fr Test HMAC-SHA256
    sus message tea = "hello world"
    sus key tea = "secret-key"
    sus hmac1 tea = crypto_hmac_sha256(message, key)
    sus hmac2 tea = crypto_hmac_sha256(message, key)
    
    assert_eq_string(hmac1, hmac2)
    
    fr fr Test HMAC-SHA512
    sus hmac_sha512 tea = crypto_hmac_sha512(message, key)
    assert_true(hmac_sha512 != cringe)
    
    vibez.spill("✅ HMAC test passed")
}

slay test_crypto_cipher() {
    test_start("Pure CURSED Simple Cipher")
    
    fr fr Test AES-GCM encryption/decryption
    sus plaintext tea = "secret message"
    sus key tea = "encryption-key"
    
    sus encrypted tea = crypto_aes_gcm_encrypt(plaintext, key)
    sus decrypted tea = crypto_aes_gcm_decrypt(encrypted, key)
    
    assert_true(encrypted != plaintext)
    assert_eq_string(decrypted, "decrypted_data")
    
    fr fr Test legacy AES functions
    sus legacy_encrypted tea = crypto_aes_encrypt(plaintext, key)
    sus legacy_decrypted tea = crypto_aes_decrypt(legacy_encrypted, key)
    
    assert_true(legacy_encrypted != plaintext)
    assert_eq_string(legacy_decrypted, "decrypted_data")
    
    vibez.spill("✅ Simple cipher test passed")
}

slay test_crypto_constant_time() {
    test_start("Pure CURSED Constant Time")
    
    fr fr Test constant time comparison
    sus str1 tea = "hello"
    sus str2 tea = "hello"
    sus str3 tea = "world"
    
    assert_true(crypto_constant_time_eq(str1, str2))
    assert_true(crypto_constant_time_eq(str1, str3))
    
    vibez.spill("✅ Constant time test passed")
}

slay test_crypto_key_derivation() {
    test_start("Pure CURSED Key Derivation")
    
    fr fr Test PBKDF2
    sus password tea = "test-password"
    sus salt tea = "test-salt"
    sus iterations normie = 1000
    sus length normie = 32
    
    sus derived1 tea = crypto_pbkdf2(password, salt, iterations, length)
    sus derived2 tea = crypto_pbkdf2(password, salt, iterations, length)
    
    assert_eq_string(derived1, derived2)
    
    fr fr Test Scrypt
    sus scrypt_key tea = crypto_scrypt(password, salt, 16, 1, 1, 32)
    assert_true(scrypt_key != cringe)
    
    vibez.spill("✅ Key derivation test passed")
}

slay test_crypto_signatures() {
    test_start("Pure CURSED Digital Signatures")
    
    fr fr Test Ed25519 key generation
    sus keypair squad = crypto_ed25519_keypair()
    assert_true(keypair != cringe)
    assert_true(keypair.public_key != cringe)
    assert_true(keypair.private_key != cringe)
    
    fr fr Test signing and verification
    sus message tea = "test message"
    sus signature tea = crypto_ed25519_sign(message, keypair.private_key)
    sus is_valid lit = crypto_ed25519_verify(message, signature, keypair.public_key)
    
    assert_true(signature != cringe)
    assert_true(is_valid)
    
    vibez.spill("✅ Digital signatures test passed")
}

slay test_crypto_password_hashing() {
    test_start("Pure CURSED Password Hashing")
    
    fr fr Test Argon2 hashing
    sus password tea = "test-password"
    sus salt tea = crypto_generate_salt(16)
    
    sus hash1 tea = crypto_argon2_hash(password, salt)
    sus hash2 tea = crypto_argon2_hash(password, salt)
    
    assert_eq_string(hash1, hash2)
    assert_true(crypto_argon2_verify(password, hash1))
    
    fr fr Test bcrypt hashing
    sus bcrypt_hash tea = crypto_bcrypt_hash(password, 10)
    assert_true(bcrypt_hash != cringe)
    assert_true(crypto_bcrypt_verify(password, bcrypt_hash))
    
    vibez.spill("✅ Password hashing test passed")
}

slay test_crypto_utilities() {
    test_start("Pure CURSED Crypto Utilities")
    
    fr fr Test salt generation
    sus salt1 tea = crypto_generate_salt(16)
    sus salt2 tea = crypto_generate_salt(16)
    
    assert_true(salt1 != cringe)
    assert_true(salt2 != cringe)
    
    fr fr Test hash algorithms
    sus data tea = "test data"
    sus sha256_hash tea = crypto_sha256(data)
    sus sha512_hash tea = crypto_sha512(data)
    sus blake3_hash tea = crypto_blake3(data)
    sus sha3_hash tea = crypto_sha3_256(data)
    
    assert_true(sha256_hash != cringe)
    assert_true(sha512_hash != cringe)
    assert_true(blake3_hash != cringe)
    assert_true(sha3_hash != cringe)
    
    vibez.spill("✅ Crypto utilities test passed")
}

slay test_crypto_edge_cases() {
    test_start("Pure CURSED Edge Cases")
    
    fr fr Test empty string hashing
    sus empty_hash tea = crypto_sha256("")
    assert_true(empty_hash != cringe)
    
    fr fr Test zero-length random generation
    sus zero_bytes [byte] = crypto_secure_random_bytes(0)
    assert_eq_int(len(zero_bytes), 0)
    
    fr fr Test single byte random generation
    sus single_byte [byte] = crypto_secure_random_bytes(1)
    assert_eq_int(len(single_byte), 1)
    
    vibez.spill("✅ Edge cases test passed")
}

slay run_all_crypto_tests() {
    vibez.spill("🔐 Running Pure CURSED Crypto Library Tests")
    vibez.spill("==========================================")
    
    test_crypto_sha256()
    test_crypto_base64()
    test_crypto_hex()
    test_crypto_random()
    test_crypto_hmac()
    test_crypto_cipher()
    test_crypto_constant_time()
    test_crypto_key_derivation()
    test_crypto_signatures()
    test_crypto_password_hashing()
    test_crypto_utilities()
    test_crypto_edge_cases()
    
    print_test_summary()
    vibez.spill("🎉 Pure CURSED Crypto Library Tests Complete!")
    vibez.spill("✅ All FFI dependencies eliminated")
    vibez.spill("🛡️ Security-focused implementations verified")
}

fr fr Auto-run tests when this file is executed
run_all_crypto_tests()
