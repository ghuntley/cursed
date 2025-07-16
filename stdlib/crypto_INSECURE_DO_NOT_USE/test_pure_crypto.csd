yeet "testz"
yeet "pure_crypto"

test_start("Pure CURSED Crypto Module Tests")

// Test hash functions
sus data tea = "hello world";
sus hash1 normie = hash_simple(data);
sus hash2 normie = hash_djb2(data);
sus hash3 normie = hash_sdbm(data);

assert_true(hash1 != 0)
assert_true(hash2 != 0)
assert_true(hash3 != 0)
assert_true(hash1 != hash2)

// Test same input produces same hash
assert_eq_int(hash_simple("test"), hash_simple("test"))

// Test Caesar cipher
sus plaintext tea = "hello";
sus encrypted tea = caesar_encrypt(plaintext, 3);
sus decrypted tea = caesar_decrypt(encrypted, 3);

assert_eq_string(decrypted, plaintext)
assert_true(encrypted != plaintext)

// Test XOR cipher
sus key tea = "secret";
sus xor_encrypted tea = xor_encrypt(plaintext, key);
sus xor_decrypted tea = xor_decrypt(xor_encrypted, key);

assert_eq_string(xor_decrypted, plaintext)
assert_true(xor_encrypted != plaintext)

// Test Base64 encoding/decoding
sus original tea = "Hello World";
sus encoded tea = base64_encode(original);
sus decoded tea = base64_decode(encoded);

assert_eq_string(decoded, original)
assert_true(encoded != original)
assert_true(string_length(encoded) > string_length(original))

// Test Hex encoding/decoding
sus hex_encoded tea = hex_encode("ABC");
sus hex_decoded tea = hex_decode(hex_encoded);

assert_eq_string(hex_decoded, "ABC")
assert_true(string_length(hex_encoded) == 6) // 3 bytes = 6 hex chars

// Test CRC32 hash
sus crc1 normie = crc32_hash("test");
sus crc2 normie = crc32_hash("test");
sus crc3 normie = crc32_hash("different");

assert_eq_int(crc1, crc2)
assert_true(crc1 != crc3)

// Test cryptographic random
crypto_seed_random(12345);
sus rand1 normie = crypto_random();
sus rand2 normie = crypto_random();

assert_true(rand1 != rand2)

sus random_bytes [byte] = crypto_random_bytes(10);
assert_eq_int(random_bytes.length, 10)

sus random_string tea = crypto_random_string(8);
assert_eq_int(string_length(random_string), 8)

// Test password hashing
sus password tea = "mypassword";
sus salt tea = "randomsalt";
sus hash tea = password_hash(password, salt);

assert_true(password_verify(password, salt, hash))
assert_false(password_verify("wrongpassword", salt, hash))

// Test PBKDF2-like function
sus pbkdf_result tea = pbkdf2_simple(password, salt, 100);
assert_true(string_length(pbkdf_result) > 0)

// Test HMAC
sus message tea = "important message";
sus hmac_key tea = "secret key";
sus mac tea = hmac_simple(hmac_key, message);

assert_true(string_length(mac) > 0)

// Same message and key should produce same MAC
sus mac2 tea = hmac_simple(hmac_key, message);
assert_eq_string(mac, mac2)

// Different message should produce different MAC
sus different_mac tea = hmac_simple(hmac_key, "different message");
assert_true(mac != different_mac)

// Test key derivation
sus derived_key tea = derive_key(password, salt, 32);
assert_eq_int(string_length(derived_key), 32)

// Test secure comparison
assert_true(secure_compare("same", "same"))
assert_false(secure_compare("different", "strings"))
assert_false(secure_compare("short", "longer"))

// Test crypto utilities
sus test_bytes [byte] = [72, 101, 108, 108, 111]; // "Hello"
sus bytes_hex tea = crypto_bytes_to_hex(test_bytes);
sus hex_bytes [byte] = crypto_hex_to_bytes(bytes_hex);

assert_eq_int(test_bytes.length, hex_bytes.length)

sus salt_generated tea = crypto_generate_salt(16);
assert_eq_int(string_length(salt_generated), 16)

sus nonce tea = crypto_generate_nonce(12);
assert_eq_int(string_length(nonce), 12)

assert_true(crypto_constant_time_compare("test", "test"))
assert_false(crypto_constant_time_compare("test", "fail"))

// Test digital signatures
sus private_key tea = "private123";
sus public_key tea = "public123";
sus signed_msg tea = sign_message("hello", private_key);

assert_true(verify_signature(signed_msg, public_key))
assert_false(verify_signature("tampered|signature", public_key))

// Test RC4-like stream cipher
sus rc4_key tea = "streamkey";
sus rc4_plaintext tea = "confidential";
sus rc4_ciphertext tea = rc4_crypt(rc4_plaintext, rc4_key);
sus rc4_decrypted tea = rc4_crypt(rc4_ciphertext, rc4_key);

assert_eq_string(rc4_decrypted, rc4_plaintext)
assert_true(rc4_ciphertext != rc4_plaintext)

// Test crypto utilities
sus random_password tea = crypto_random_password(12);
assert_eq_int(string_length(random_password), 12)

sus entropy normie = crypto_entropy_estimate("abcdef");
assert_eq_int(entropy, 6) // 6 unique characters

assert_true(crypto_is_strong_password("StrongP@ss1"))
assert_false(crypto_is_strong_password("weak"))
assert_false(crypto_is_strong_password("nouppercase123!"))
assert_false(crypto_is_strong_password("NOLOWERCASE123!"))
assert_false(crypto_is_strong_password("NoNumbers!"))
assert_false(crypto_is_strong_password("NoSpecialChars123"))

// Test wiping (security function)
sus sensitive tea = "secret data";
sus wiped tea = crypto_wipe_string(sensitive);
assert_eq_int(string_length(wiped), string_length(sensitive))

print_test_summary()
