yeet "testz"
yeet "cryptz"

test_start("cryptz Security Comprehensive Tests")

fr fr ===== HASH FUNCTION TESTS =====

test_group("Cryptographic Hash Functions")

fr fr Test SHA-256 basic functionality
sus hash_result tea = sha256_hash("test")
assert_not_empty(hash_result, "sha256_hash produces output")
assert_string_contains(hash_result, "hex", "sha256_hash hex format")

hash_result = sha256_hash("hello world")
assert_not_empty(hash_result, "sha256_hash with longer input")

fr fr Test SHA-256 consistency
sus hash1 tea = sha256_hash("same input")
sus hash2 tea = sha256_hash("same input")
assert_string_equals(hash1, hash2, "sha256_hash deterministic")

sus hash3 tea = sha256_hash("different input")
assert_string_not_equals(hash1, hash3, "sha256_hash different outputs")

fr fr Test SHA-512 functionality
hash_result = sha512_hash("test")
assert_not_empty(hash_result, "sha512_hash produces output")

fr fr Test BLAKE2b functionality
hash_result = blake2b_hash("test", 32)
assert_not_empty(hash_result, "blake2b_hash produces output")

fr fr Test MD5 security enforcement
hash_result = md5_hash_deprecated_insecure("test")
assert_string_equals(hash_result, "INSECURE_MD5_PERMANENTLY_DISABLED", "MD5 properly disabled")

fr fr ===== SYMMETRIC ENCRYPTION TESTS =====

test_group("Symmetric Encryption")

fr fr Test AES encryption basic functionality
sus plaintext tea = "Hello, CURSED!"
sus key_128 tea = "1234567890123456"  fr fr 16 bytes = 128 bits
sus key_256 tea = "12345678901234567890123456789012"  fr fr 32 bytes = 256 bits

fr fr Test AES-128 ECB mode
sus encrypted tea = aes_encrypt(plaintext, key_128, "ECB")
assert_not_empty(encrypted, "AES ECB encryption produces output")
assert_string_not_equals(encrypted, plaintext, "AES ECB encryption changes data")

sus decrypted tea = aes_decrypt(encrypted, key_128, "ECB")
assert_string_equals(decrypted, plaintext, "AES ECB round trip")

fr fr Test AES-256 CBC mode
encrypted = aes_encrypt(plaintext, key_256, "CBC")
assert_not_empty(encrypted, "AES CBC encryption produces output")
assert_string_length_greater(encrypted, plaintext, "AES CBC includes IV")

decrypted = aes_decrypt(encrypted, key_256, "CBC")
assert_string_equals(decrypted, plaintext, "AES CBC round trip")

fr fr Test AES-256 GCM mode (authenticated encryption)
encrypted = aes_encrypt(plaintext, key_256, "GCM")
assert_not_empty(encrypted, "AES GCM encryption produces output")

decrypted = aes_decrypt(encrypted, key_256, "GCM")
assert_string_equals(decrypted, plaintext, "AES GCM round trip")

fr fr Test invalid key sizes
sus invalid_key tea = "123"  fr fr Too short
sus invalid_result tea = aes_encrypt(plaintext, invalid_key, "ECB")
assert_string_empty(invalid_result, "AES rejects invalid key size")

fr fr Test ChaCha20 encryption
sus chacha_key tea = "12345678901234567890123456789012"  fr fr 32 bytes
sus chacha_nonce tea = "123456789012"  fr fr 12 bytes

encrypted = chacha20_encrypt(plaintext, chacha_key, chacha_nonce)
assert_not_empty(encrypted, "ChaCha20 encryption produces output")

decrypted = chacha20_decrypt(encrypted, chacha_key, chacha_nonce)
assert_string_equals(decrypted, plaintext, "ChaCha20 round trip")

fr fr Test ChaCha20 invalid parameters
invalid_result = chacha20_encrypt(plaintext, "short_key", chacha_nonce)
assert_string_empty(invalid_result, "ChaCha20 rejects short key")

invalid_result = chacha20_encrypt(plaintext, chacha_key, "short")
assert_string_empty(invalid_result, "ChaCha20 rejects short nonce")

fr fr ===== ASYMMETRIC CRYPTOGRAPHY TESTS =====

test_group("Asymmetric Cryptography")

fr fr Test RSA key generation
sus rsa_keypair KeyPair = rsa_generate_keypair(2048)
assert_not_empty(rsa_keypair.public_key, "RSA keypair has public key")
assert_not_empty(rsa_keypair.private_key, "RSA keypair has private key")
assert_string_equals(rsa_keypair.algorithm, "RSA", "RSA keypair algorithm")
assert_eq_int(rsa_keypair.key_size, 2048, "RSA keypair key size")

fr fr Test RSA key format
assert_string_contains(rsa_keypair.public_key, "BEGIN RSA PUBLIC KEY", "RSA public key format")
assert_string_contains(rsa_keypair.private_key, "BEGIN RSA PRIVATE KEY", "RSA private key format")

fr fr Test RSA invalid key sizes
sus invalid_keypair KeyPair = rsa_generate_keypair(1024)
assert_string_empty(invalid_keypair.public_key, "RSA rejects weak key size")

fr fr Test ECDSA key generation
sus ecdsa_keypair KeyPair = ecdsa_generate_keypair("P-256")
assert_not_empty(ecdsa_keypair.public_key, "ECDSA keypair has public key")
assert_not_empty(ecdsa_keypair.private_key, "ECDSA keypair has private key")
assert_string_equals(ecdsa_keypair.algorithm, "ECDSA", "ECDSA keypair algorithm")

fr fr Test ECDSA key format
assert_string_contains(ecdsa_keypair.public_key, "BEGIN EC PUBLIC KEY", "ECDSA public key format")
assert_string_contains(ecdsa_keypair.private_key, "BEGIN EC PRIVATE KEY", "ECDSA private key format")

fr fr Test ECDSA invalid curves
sus invalid_ecdsa KeyPair = ecdsa_generate_keypair("invalid-curve")
assert_string_empty(invalid_ecdsa.public_key, "ECDSA rejects invalid curve")

fr fr ===== RANDOM NUMBER GENERATION TESTS =====

test_group("Cryptographic Random Number Generation")

fr fr Test random byte generation
sus random_bytes tea = generate_random_bytes(16)
assert_not_empty(random_bytes, "Random bytes generated")
assert_eq_int(string_length(random_bytes), 16, "Random bytes correct length")

sus random_bytes2 tea = generate_random_bytes(16)
assert_string_not_equals(random_bytes, random_bytes2, "Random bytes are different")

fr fr Test random with different sizes
sus small_random tea = generate_random_bytes(1)
assert_eq_int(string_length(small_random), 1, "Small random bytes")

sus large_random tea = generate_random_bytes(64)
assert_eq_int(string_length(large_random), 64, "Large random bytes")

fr fr ===== KEY DERIVATION TESTS =====

test_group("Key Derivation Functions")

fr fr Test PBKDF2 key derivation
sus password tea = "secure_password123"
sus salt tea = "random_salt_value"
sus derived_key tea = pbkdf2_derive_key(password, salt, 1000, 32)
assert_not_empty(derived_key, "PBKDF2 key derivation")
assert_eq_int(string_length(derived_key), 32, "PBKDF2 correct key length")

fr fr Test PBKDF2 consistency
sus derived_key2 tea = pbkdf2_derive_key(password, salt, 1000, 32)
assert_string_equals(derived_key, derived_key2, "PBKDF2 deterministic")

fr fr Test different salt produces different key
sus different_key tea = pbkdf2_derive_key(password, "different_salt", 1000, 32)
assert_string_not_equals(derived_key, different_key, "PBKDF2 salt affects output")

fr fr Test Argon2 key derivation
derived_key = argon2_derive_key(password, salt, 32)
assert_not_empty(derived_key, "Argon2 key derivation")
assert_eq_int(string_length(derived_key), 32, "Argon2 correct key length")

fr fr ===== DIGITAL SIGNATURE TESTS =====

test_group("Digital Signatures")

fr fr Test HMAC signatures
sus message tea = "Important message to sign"
sus hmac_key tea = "signing_key_123456789012345678901234567890"
sus signature tea = hmac_sign(message, hmac_key, "SHA256")
assert_not_empty(signature, "HMAC signature generated")

sus valid lit = hmac_verify(message, signature, hmac_key, "SHA256")
assert_bool(valid, "HMAC signature verification")

sus invalid lit = hmac_verify("tampered message", signature, hmac_key, "SHA256")
assert_bool(!invalid, "HMAC detects tampering")

fr fr ===== CONSTANT-TIME OPERATIONS TESTS =====

test_group("Constant-Time Security")

fr fr Test constant-time comparison
sus string1 tea = "same_value"
sus string2 tea = "same_value"
sus string3 tea = "different"

sus equal lit = constant_time_compare(string1, string2)
assert_bool(equal, "Constant time equal comparison")

sus not_equal lit = constant_time_compare(string1, string3)
assert_bool(!not_equal, "Constant time unequal comparison")

fr fr Test timing attack resistance
sus auth_token tea = "secret_auth_token_12345"
sus valid_token tea = "secret_auth_token_12345"
sus invalid_token tea = "invalid_token_67890000"

equal = constant_time_compare(auth_token, valid_token)
assert_bool(equal, "Token validation accepts valid")

not_equal = constant_time_compare(auth_token, invalid_token)
assert_bool(!not_equal, "Token validation rejects invalid")

fr fr ===== CERTIFICATE AND PKI TESTS =====

test_group("Public Key Infrastructure")

fr fr Test certificate generation
sus cert_request tea = create_certificate_request("CN=test.example.com")
assert_not_empty(cert_request, "Certificate request generated")
assert_string_contains(cert_request, "BEGIN CERTIFICATE REQUEST", "CSR format")

fr fr Test certificate validation
sus self_signed_cert tea = generate_self_signed_certificate(rsa_keypair, "CN=localhost")
assert_not_empty(self_signed_cert, "Self-signed certificate generated")
assert_string_contains(self_signed_cert, "BEGIN CERTIFICATE", "Certificate format")

sus cert_valid lit = verify_certificate_signature(self_signed_cert, rsa_keypair.public_key)
assert_bool(cert_valid, "Certificate signature verification")

fr fr ===== SECURE COMMUNICATION TESTS =====

test_group("Secure Communication")

fr fr Test TLS handshake simulation
sus tls_context tea = create_tls_context("TLS_1_3")
assert_not_empty(tls_context, "TLS context created")

sus client_hello tea = create_client_hello(tls_context)
assert_not_empty(client_hello, "Client hello message")
assert_string_contains(client_hello, "TLS", "TLS protocol in handshake")

fr fr Test secure channel establishment
sus shared_secret tea = establish_shared_secret(rsa_keypair, "client_random_data")
assert_not_empty(shared_secret, "Shared secret established")

fr fr ===== EDGE CASE AND SECURITY TESTS =====

test_group("Security Edge Cases")

fr fr Test empty input handling
hash_result = sha256_hash("")
assert_not_empty(hash_result, "Hash of empty string")

sus empty_encrypted tea = aes_encrypt("", key_128, "ECB")
assert_string_empty(empty_encrypted, "Empty plaintext handling")

fr fr Test large input handling
sus large_input tea = create_large_string(1000)
hash_result = sha256_hash(large_input)
assert_not_empty(hash_result, "Hash of large input")

fr fr Test null byte handling
sus null_data tea = "data\x00with\x00nulls"
hash_result = sha256_hash(null_data)
assert_not_empty(hash_result, "Hash with null bytes")

fr fr Test boundary key sizes
sus boundary_key tea = "12345678901234567890123456789012"  fr fr Exactly 32 bytes
encrypted = aes_encrypt(plaintext, boundary_key, "ECB")
assert_not_empty(encrypted, "Boundary key size accepted")

fr fr Test memory safety with repeated operations
sus i drip = 0
bestie (i < 10) {
    hash_result = sha256_hash("iteration_" + json_number_to_string(i))
    assert_not_empty(hash_result, "Repeated hash operations")
    i = i + 1
}

fr fr Test concurrent safety simulation
sus concurrent_results tea[value] = []
i = 0
bestie (i < 5) {
    sus concurrent_hash tea = sha256_hash("concurrent_" + json_number_to_string(i))
    array_push(concurrent_results, concurrent_hash)
    i = i + 1
}
assert_eq_int(array_length(concurrent_results), 5, "Concurrent operations completed")

fr fr ===== CRYPTOGRAPHIC PROTOCOL TESTS =====

test_group("Cryptographic Protocols")

fr fr Test key exchange protocols
sus dh_params tea = generate_diffie_hellman_params()
assert_not_empty(dh_params, "DH parameters generated")

sus alice_keypair KeyPair = dh_generate_keypair(dh_params)
sus bob_keypair KeyPair = dh_generate_keypair(dh_params)

sus alice_shared tea = dh_compute_shared_secret(alice_keypair.private_key, bob_keypair.public_key)
sus bob_shared tea = dh_compute_shared_secret(bob_keypair.private_key, alice_keypair.public_key)

assert_string_equals(alice_shared, bob_shared, "DH shared secret agreement")

fr fr Test authenticated encryption
sus aead_key tea = "32_byte_authenticated_encryption_key"
sus aead_nonce tea = "unique_nonce_12_bytes"
sus associated_data tea = "public_associated_data"

encrypted = aead_encrypt(plaintext, aead_key, aead_nonce, associated_data)
assert_not_empty(encrypted, "AEAD encryption")

decrypted = aead_decrypt(encrypted, aead_key, aead_nonce, associated_data)
assert_string_equals(decrypted, plaintext, "AEAD round trip")

fr fr Test AEAD with tampered associated data
sus tampered_decryption tea = aead_decrypt(encrypted, aead_key, aead_nonce, "tampered_data")
assert_string_empty(tampered_decryption, "AEAD detects tampering")

print_test_summary()
