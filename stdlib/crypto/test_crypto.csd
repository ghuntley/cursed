yeet "testz"
yeet "crypto"

fr fr ========================================
fr fr CURSED Crypto Library Test Suite
fr fr ========================================

slay test_crypto_hash_functions() {
    testz.test_start("Crypto Hash Functions")
    
    fr fr Test SHA-256 hashing
    sus sha256_test tea = crypto_sha256("hello world")
    testz.assert_true(string_len(sha256_test) == 64)
    testz.assert_eq_string(crypto_sha256("hello world"), sha256_test)
    testz.assert_true(crypto_sha256("hello") != crypto_sha256("world"))
    
    fr fr Test SHA-512 hashing
    sus sha512_test tea = crypto_sha512("hello world")
    testz.assert_true(string_len(sha512_test) == 128)
    testz.assert_eq_string(crypto_sha512("hello world"), sha512_test)
    testz.assert_true(crypto_sha512("hello") != crypto_sha512("world"))
    
    fr fr Test MD5 hashing
    sus md5_test tea = crypto_md5("hello world")
    testz.assert_true(string_len(md5_test) == 32)
    testz.assert_eq_string(crypto_md5("hello world"), md5_test)
    testz.assert_true(crypto_md5("hello") != crypto_md5("world"))
    
    fr fr Test BLAKE3 hashing
    sus blake3_test tea = crypto_blake3("hello world")
    testz.assert_true(string_len(blake3_test) == 64)
    testz.assert_eq_string(crypto_blake3("hello world"), blake3_test)
    testz.assert_true(crypto_blake3("hello") != crypto_blake3("world"))
}

slay test_crypto_random_generation() {
    testz.test_start("Crypto Random Generation")
    
    fr fr Test random bytes generation
    sus bytes1 [byte] = crypto_random_bytes(16)
    sus bytes2 [byte] = crypto_random_bytes(16)
    testz.assert_eq_int(len(bytes1), 16)
    testz.assert_eq_int(len(bytes2), 16)
    testz.assert_true(bytes1 != bytes2)
    
    fr fr Test random integer generation
    sus rand_int1 normie = crypto_random_int(1, 100)
    sus rand_int2 normie = crypto_random_int(1, 100)
    testz.assert_true(rand_int1 >= 1 && rand_int1 <= 100)
    testz.assert_true(rand_int2 >= 1 && rand_int2 <= 100)
    testz.assert_true(rand_int1 != rand_int2)
    
    fr fr Test random string generation
    sus rand_str1 tea = crypto_random_string(10)
    sus rand_str2 tea = crypto_random_string(10)
    testz.assert_eq_int(string_len(rand_str1), 10)
    testz.assert_eq_int(string_len(rand_str2), 10)
    testz.assert_true(rand_str1 != rand_str2)
    
    fr fr Test secure random float
    sus rand_float1 meal = crypto_secure_random()
    sus rand_float2 meal = crypto_secure_random()
    testz.assert_true(rand_float1 >= 0.0 && rand_float1 <= 1.0)
    testz.assert_true(rand_float2 >= 0.0 && rand_float2 <= 1.0)
    testz.assert_true(rand_float1 != rand_float2)
}

slay test_crypto_base64_encoding() {
    testz.test_start("Crypto Base64 Encoding")
    
    fr fr Test base64 encoding
    sus original tea = "hello world"
    sus encoded tea = crypto_base64_encode(original)
    testz.assert_true(string_len(encoded) > 0)
    testz.assert_true(encoded != original)
    
    fr fr Test base64 decoding
    sus decoded tea = crypto_base64_decode(encoded)
    testz.assert_eq_string(decoded, original)
    
    fr fr Test base64 round trip
    sus test_data tea = "The quick brown fox jumps over the lazy dog"
    sus encoded_data tea = crypto_base64_encode(test_data)
    sus decoded_data tea = crypto_base64_decode(encoded_data)
    testz.assert_eq_string(decoded_data, test_data)
}

slay test_crypto_hex_encoding() {
    testz.test_start("Crypto Hex Encoding")
    
    fr fr Test hex encoding
    sus data [byte] = [72, 101, 108, 108, 111]
    sus hex_encoded tea = crypto_hex_encode(data)
    testz.assert_eq_string(hex_encoded, "48656c6c6f")
    
    fr fr Test hex decoding
    sus decoded_data [byte] = crypto_hex_decode(hex_encoded)
    testz.assert_eq_int(len(decoded_data), 5)
    testz.assert_eq_int(decoded_data[0], 72)
    testz.assert_eq_int(decoded_data[1], 101)
    testz.assert_eq_int(decoded_data[2], 108)
    testz.assert_eq_int(decoded_data[3], 108)
    testz.assert_eq_int(decoded_data[4], 111)
}

slay test_crypto_aes_encryption() {
    testz.test_start("Crypto AES Encryption")
    
    fr fr Test AES encryption/decryption
    sus plaintext tea = "This is a secret message"
    sus key tea = "my-secret-key-32-bytes-long-test"
    
    sus encrypted tea = crypto_aes_encrypt(plaintext, key)
    testz.assert_true(string_len(encrypted) > 0)
    testz.assert_true(encrypted != plaintext)
    
    sus decrypted tea = crypto_aes_decrypt(encrypted, key)
    testz.assert_eq_string(decrypted, plaintext)
    
    fr fr Test AES with different keys
    sus key2 tea = "different-key-32-bytes-long-test"
    sus encrypted2 tea = crypto_aes_encrypt(plaintext, key2)
    testz.assert_true(encrypted != encrypted2)
}

slay test_crypto_hmac() {
    testz.test_start("Crypto HMAC Functions")
    
    fr fr Test HMAC-SHA256
    sus message tea = "hello world"
    sus key tea = "secret-key"
    sus hmac_sha256_result tea = crypto_hmac_sha256(message, key)
    testz.assert_true(string_len(hmac_sha256_result) == 64)
    testz.assert_eq_string(crypto_hmac_sha256(message, key), hmac_sha256_result)
    
    fr fr Test HMAC-SHA512
    sus hmac_sha512_result tea = crypto_hmac_sha512(message, key)
    testz.assert_true(string_len(hmac_sha512_result) == 128)
    testz.assert_eq_string(crypto_hmac_sha512(message, key), hmac_sha512_result)
    
    fr fr Test HMAC with different keys
    sus key2 tea = "different-key"
    testz.assert_true(crypto_hmac_sha256(message, key) != crypto_hmac_sha256(message, key2))
}

slay test_crypto_key_derivation() {
    testz.test_start("Crypto Key Derivation")
    
    fr fr Test PBKDF2
    sus password tea = "test-password"
    sus salt tea = "test-salt"
    sus iterations normie = 1000
    sus length normie = 32
    
    sus derived_key tea = crypto_pbkdf2(password, salt, iterations, length)
    testz.assert_eq_int(string_len(derived_key), length * 2)
    testz.assert_eq_string(crypto_pbkdf2(password, salt, iterations, length), derived_key)
    
    fr fr Test Scrypt
    sus scrypt_key tea = crypto_scrypt(password, salt, 16, 1, 1, 32)
    testz.assert_eq_int(string_len(scrypt_key), 64)
    testz.assert_eq_string(crypto_scrypt(password, salt, 16, 1, 1, 32), scrypt_key)
    
    fr fr Test different passwords produce different keys
    sus password2 tea = "different-password"
    testz.assert_true(crypto_pbkdf2(password, salt, iterations, length) != crypto_pbkdf2(password2, salt, iterations, length))
}

slay test_crypto_ed25519_signatures() {
    testz.test_start("Crypto Ed25519 Signatures")
    
    fr fr Test Ed25519 key generation
    sus keypair squad = crypto_ed25519_keypair()
    testz.assert_true(keypair != cringe)
    testz.assert_true(keypair.public_key != "")
    testz.assert_true(keypair.private_key != "")
    
    fr fr Test Ed25519 signing and verification
    sus message tea = "Hello, digital signature!"
    sus signature tea = crypto_ed25519_sign(message, keypair.private_key)
    testz.assert_true(string_len(signature) > 0)
    
    sus is_valid lit = crypto_ed25519_verify(message, signature, keypair.public_key)
    testz.assert_true(is_valid)
    
    fr fr Test signature verification with wrong message
    sus wrong_message tea = "Different message"
    sus is_invalid lit = crypto_ed25519_verify(wrong_message, signature, keypair.public_key)
    testz.assert_false(is_invalid)
}

slay test_crypto_password_hashing() {
    testz.test_start("Crypto Password Hashing")
    
    fr fr Test Argon2 password hashing
    sus password tea = "test-password"
    sus salt tea = crypto_generate_salt(16)
    
    sus argon2_hash tea = crypto_argon2_hash(password, salt)
    testz.assert_true(string_len(argon2_hash) > 0)
    
    sus argon2_valid lit = crypto_argon2_verify(password, argon2_hash)
    testz.assert_true(argon2_valid)
    
    sus argon2_invalid lit = crypto_argon2_verify("wrong-password", argon2_hash)
    testz.assert_false(argon2_invalid)
    
    fr fr Test bcrypt password hashing
    sus bcrypt_hash tea = crypto_bcrypt_hash(password, 10)
    testz.assert_true(string_len(bcrypt_hash) > 0)
    
    sus bcrypt_valid lit = crypto_bcrypt_verify(password, bcrypt_hash)
    testz.assert_true(bcrypt_valid)
    
    sus bcrypt_invalid lit = crypto_bcrypt_verify("wrong-password", bcrypt_hash)
    testz.assert_false(bcrypt_invalid)
}

slay test_crypto_constant_time() {
    testz.test_start("Crypto Constant Time Operations")
    
    fr fr Test constant time comparison
    sus str1 tea = "hello world"
    sus str2 tea = "hello world"
    sus str3 tea = "different"
    
    testz.assert_true(crypto_constant_time_eq(str1, str2))
    testz.assert_false(crypto_constant_time_eq(str1, str3))
    testz.assert_false(crypto_constant_time_eq(str2, str3))
    
    fr fr Test constant time with same length strings
    sus same_length1 tea = "abcdefgh"
    sus same_length2 tea = "ijklmnop"
    testz.assert_false(crypto_constant_time_eq(same_length1, same_length2))
}

slay test_crypto_utilities() {
    testz.test_start("Crypto Utility Functions")
    
    fr fr Test salt generation
    sus salt1 tea = crypto_generate_salt(16)
    sus salt2 tea = crypto_generate_salt(16)
    testz.assert_eq_int(string_len(salt1), 32)
    testz.assert_eq_int(string_len(salt2), 32)
    testz.assert_true(salt1 != salt2)
    
    fr fr Test different salt lengths
    sus salt_short tea = crypto_generate_salt(8)
    sus salt_long tea = crypto_generate_salt(32)
    testz.assert_eq_int(string_len(salt_short), 16)
    testz.assert_eq_int(string_len(salt_long), 64)
}

slay test_crypto_edge_cases() {
    testz.test_start("Crypto Edge Cases")
    
    fr fr Test empty string hashing
    sus empty_sha256 tea = crypto_sha256("")
    testz.assert_true(string_len(empty_sha256) == 64)
    testz.assert_eq_string(crypto_sha256(""), empty_sha256)
    
    fr fr Test large string hashing
    sus large_string tea = string_repeat("a", 1000)
    sus large_hash tea = crypto_sha256(large_string)
    testz.assert_true(string_len(large_hash) == 64)
    
    fr fr Test zero-length random generation
    sus zero_bytes [byte] = crypto_random_bytes(0)
    testz.assert_eq_int(len(zero_bytes), 0)
    
    fr fr Test single byte random generation
    sus single_byte [byte] = crypto_random_bytes(1)
    testz.assert_eq_int(len(single_byte), 1)
}

slay run_all_crypto_tests() {
    vibez.spill("🔐 Running CURSED Crypto Library Tests")
    vibez.spill("====================================")
    
    test_crypto_hash_functions()
    test_crypto_random_generation()
    test_crypto_base64_encoding()
    test_crypto_hex_encoding()
    test_crypto_aes_encryption()
    test_crypto_hmac()
    test_crypto_key_derivation()
    test_crypto_ed25519_signatures()
    test_crypto_password_hashing()
    test_crypto_constant_time()
    test_crypto_utilities()
    test_crypto_edge_cases()
    
    testz.print_test_summary()
    damn testz.run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_crypto_tests()
