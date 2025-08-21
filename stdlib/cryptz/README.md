# CRYPTZ - Production Cryptographic Library for CURSED

## Overview

The **CRYPTZ** module provides a comprehensive, production-ready cryptographic library for CURSED applications. It implements industry-standard algorithms with security-first design principles, constant-time operations to prevent timing attacks, and memory-safe implementations.

This library is designed for real-world security applications and follows established cryptographic best practices and standards including FIPS 140-2 guidelines and RFC specifications.

## 🔒 Security Features

- **Memory-Safe Implementation**: Pure CURSED with no FFI dependencies
- **Constant-Time Operations**: Prevents timing attack vulnerabilities
- **Secure Key Generation**: Uses cryptographically secure random number generation
- **Industry Standards**: Implements NIST-approved algorithms (AES, SHA-2, ECDSA)
- **Authenticated Encryption**: AES-GCM provides both confidentiality and integrity
- **Modern Algorithms**: Includes cutting-edge cryptography (Ed25519, ChaCha20, BLAKE3)

## 🚀 Quick Start

```cursed
yeet "cryptz"

fr fr Generate secure random data
sus secret_key []drip = cryptz.generate_secure_key(32)
sus random_bytes []drip = cryptz.generate_random_bytes(16)

fr fr Hash data
sus message tea = "Hello, World!"
sus hash []drip = cryptz.sha256_hash(message)
sus hex_hash tea = cryptz.bytes_to_hex(hash)

fr fr Encrypt data with password
sus plaintext tea = "Secret information"
sus password tea = "secure_password_123"
sus encrypted tea = cryptz.encrypt_data(plaintext, password)
sus decrypted tea = cryptz.decrypt_data(encrypted, password)

fr fr Digital signatures
sus keypair cryptz.KeyPair = cryptz.ed25519_generate_keypair()
sus signature []drip = cryptz.ed25519_sign("Important document", keypair.private_key)
sus is_valid lit = cryptz.ed25519_verify("Important document", signature, keypair.public_key)
```

## 📚 API Reference

### Random Number Generation

#### `generate_random_bytes(length drip) []drip`
Generates cryptographically secure random bytes using a ChaCha20-based CSPRNG.

**Parameters:**
- `length` - Number of bytes to generate

**Returns:** Array of random bytes

**Security:** Uses multiple entropy sources and cryptographically secure algorithms

```cursed
sus random_data []drip = cryptz.generate_random_bytes(32)
sus hex_random tea = cryptz.bytes_to_hex(random_data)
```

#### `generate_secure_key(key_size drip) []drip`
Generates a cryptographically secure encryption key of specified size.

**Parameters:**
- `key_size` - Key size in bytes (minimum 16)

**Returns:** Secure random key

```cursed
sus aes_key []drip = cryptz.generate_secure_key(32)  fr fr AES-256 key
sus hmac_key []drip = cryptz.generate_secure_key(32)
```

#### `random_password(length drip, complexity tea) tea`
Generates a secure random password with specified complexity.

**Parameters:**
- `length` - Password length (minimum 8)
- `complexity` - "simple" (alphanumeric), "complex" (with symbols), or "standard"

**Returns:** Random password string

```cursed
sus user_password tea = cryptz.random_password(16, "complex")
sus api_token tea = cryptz.random_password(32, "simple")
```

### Cryptographic Hash Functions

#### `sha256_hash(data tea) []drip`
Computes SHA-256 hash of input data.

**Parameters:**
- `data` - Input data to hash

**Returns:** 32-byte SHA-256 digest

**Security:** Collision-resistant, suitable for digital signatures and integrity checks

```cursed
sus hash []drip = cryptz.sha256_hash("The quick brown fox")
sus hex_hash tea = cryptz.bytes_to_hex(hash)
fr fr Result: 32-byte hash as hex string
```

#### `sha512_hash(data tea) []drip`
Computes SHA-512 hash of input data.

**Parameters:**
- `data` - Input data to hash

**Returns:** 64-byte SHA-512 digest

**Security:** Higher security margin than SHA-256, recommended for long-term security

```cursed
sus large_hash []drip = cryptz.sha512_hash("Large data set")
```

#### `blake3_hash(data tea) []drip`
Computes BLAKE3 hash of input data.

**Parameters:**
- `data` - Input data to hash

**Returns:** 32-byte BLAKE3 digest

**Security:** Modern, high-performance hash function with strong security properties

```cursed
sus modern_hash []drip = cryptz.blake3_hash("Modern hashing")
```

### Message Authentication

#### `hmac_sha256(key []drip, message []drip) []drip`
Computes HMAC using SHA-256.

**Parameters:**
- `key` - Secret authentication key
- `message` - Data to authenticate

**Returns:** 32-byte HMAC digest

**Security:** Prevents message tampering, requires secret key for verification

```cursed
sus mac_key []drip = cryptz.generate_secure_key(32)
sus message_bytes []drip = stringz.bytes("Important message")
sus mac []drip = cryptz.hmac_sha256(mac_key, message_bytes)
```

#### `hmac_sha512(key []drip, message []drip) []drip`
Computes HMAC using SHA-512.

**Parameters:**
- `key` - Secret authentication key
- `message` - Data to authenticate

**Returns:** 64-byte HMAC digest

```cursed
sus strong_mac []drip = cryptz.hmac_sha512(mac_key, message_bytes)
```

### Symmetric Encryption

#### `aes_gcm_encrypt(plaintext tea, key []drip, additional_data tea) []drip`
Encrypts data using AES-GCM authenticated encryption.

**Parameters:**
- `plaintext` - Data to encrypt
- `key` - AES key (16, 24, or 32 bytes for AES-128/192/256)
- `additional_data` - Additional authenticated data (can be empty)

**Returns:** Encrypted data with embedded IV and authentication tag

**Security:** Provides both confidentiality and integrity, prevents tampering

```cursed
sus aes_key []drip = cryptz.generate_secure_key(32)  fr fr AES-256
sus encrypted []drip = cryptz.aes_gcm_encrypt("Secret data", aes_key, "metadata")
```

#### `aes_gcm_decrypt(encrypted_data []drip, key []drip, additional_data tea) []drip`
Decrypts data using AES-GCM authenticated encryption.

**Parameters:**
- `encrypted_data` - Encrypted data with IV and tag
- `key` - AES decryption key
- `additional_data` - Same additional data used during encryption

**Returns:** Plaintext data, or empty array if authentication fails

**Security:** Authentication failure prevents decryption with wrong key or tampered data

```cursed
sus decrypted []drip = cryptz.aes_gcm_decrypt(encrypted, aes_key, "metadata")
ready len(decrypted) == 0 {
    vibez.spill("Decryption failed - wrong key or tampered data")
} otherwise {
    sus plaintext tea = stringz.from_bytes(decrypted)
}
```

#### `chacha20_encrypt(plaintext tea, key []drip, nonce []drip) []drip`
Encrypts/decrypts data using ChaCha20 stream cipher.

**Parameters:**
- `plaintext` - Data to encrypt (or ciphertext to decrypt)
- `key` - 32-byte ChaCha20 key
- `nonce` - 12-byte unique nonce

**Returns:** Encrypted/decrypted data

**Security:** Modern stream cipher, same operation for encryption and decryption

```cursed
sus chacha_key []drip = cryptz.generate_secure_key(32)
sus nonce []drip = cryptz.generate_random_bytes(12)
sus encrypted []drip = cryptz.chacha20_encrypt("Stream data", chacha_key, nonce)
sus decrypted []drip = cryptz.chacha20_encrypt(stringz.from_bytes(encrypted), chacha_key, nonce)
```

### Digital Signatures

#### `ed25519_generate_keypair() KeyPair`
Generates Ed25519 key pair for digital signatures.

**Returns:** KeyPair structure with public and private keys

**Security:** Modern elliptic curve signatures, fast and secure

```cursed
sus keypair cryptz.KeyPair = cryptz.ed25519_generate_keypair()
vibez.spill("Algorithm:", keypair.algorithm)  fr fr "Ed25519"
vibez.spill("Key size:", keypair.key_size)    fr fr 32 bytes
```

#### `ed25519_sign(message tea, private_key []drip) []drip`
Signs a message using Ed25519 private key.

**Parameters:**
- `message` - Message to sign
- `private_key` - Ed25519 private key

**Returns:** 64-byte digital signature

**Security:** Deterministic signatures, secure against various attacks

```cursed
sus message tea = "Contract agreement"
sus signature []drip = cryptz.ed25519_sign(message, keypair.private_key)
```

#### `ed25519_verify(message tea, signature []drip, public_key []drip) lit`
Verifies Ed25519 digital signature.

**Parameters:**
- `message` - Original message
- `signature` - Digital signature to verify
- `public_key` - Ed25519 public key

**Returns:** `based` if signature is valid, `cringe` if invalid

```cursed
sus is_valid lit = cryptz.ed25519_verify(message, signature, keypair.public_key)
ready is_valid {
    vibez.spill("Signature is valid")
} otherwise {
    vibez.spill("Signature verification failed")
}
```

#### `rsa_generate_keypair(key_size drip) KeyPair`
Generates RSA key pair with specified bit size.

**Parameters:**
- `key_size` - Key size in bits (minimum 2048, recommended 3072+)

**Returns:** KeyPair structure with RSA keys

**Security:** Use minimum 2048 bits, prefer Ed25519 for new applications

```cursed
sus rsa_keypair cryptz.KeyPair = cryptz.rsa_generate_keypair(2048)
ready rsa_keypair.algorithm == "" {
    vibez.spill("Key generation failed - key size too small")
}
```

### Key Derivation Functions

#### `pbkdf2_derive_key(password tea, salt []drip, iterations drip, output_length drip) []drip`
Derives cryptographic key from password using PBKDF2.

**Parameters:**
- `password` - Password or passphrase
- `salt` - Unique salt (minimum 16 bytes)
- `iterations` - Iteration count (minimum 100,000)
- `output_length` - Desired key length

**Returns:** Derived key

**Security:** Use high iteration count, unique salt per password

```cursed
sus salt []drip = cryptz.generate_random_bytes(32)
sus derived_key []drip = cryptz.pbkdf2_derive_key("user_password", salt, 100000, 32)
```

#### `argon2_derive_key(password tea, salt []drip, memory_cost drip, time_cost drip, parallelism drip, output_length drip) []drip`
Derives key using Argon2id (memory-hard function).

**Parameters:**
- `password` - Password or passphrase
- `salt` - Unique salt
- `memory_cost` - Memory usage in bytes (minimum 65536)
- `time_cost` - Time cost parameter (minimum 3)
- `parallelism` - Parallel threads (typically 1)
- `output_length` - Desired key length

**Returns:** Derived key

**Security:** Memory-hard function, resistant to GPU/ASIC attacks

```cursed
sus argon2_key []drip = cryptz.argon2_derive_key(
    "password", salt, 65536, 3, 1, 32
)
```

#### `scrypt_derive_key(password tea, salt []drip, n drip, r drip, p drip, output_length drip) []drip`
Derives key using Scrypt memory-hard function.

**Parameters:**
- `password` - Password or passphrase
- `salt` - Unique salt
- `n` - CPU/memory cost parameter (power of 2, minimum 16384)
- `r` - Block size parameter (typically 8)
- `p` - Parallelization parameter (typically 1)
- `output_length` - Desired key length

**Returns:** Derived key

```cursed
sus scrypt_key []drip = cryptz.scrypt_derive_key("password", salt, 16384, 8, 1, 32)
```

### Constant-Time Security Operations

#### `constant_time_bytes_equal(a []drip, b []drip) lit`
Compares two byte arrays in constant time.

**Parameters:**
- `a`, `b` - Byte arrays to compare

**Returns:** `based` if equal, `cringe` if different or different lengths

**Security:** Prevents timing attacks when comparing secrets

```cursed
sus stored_hash []drip = [...] fr fr Previously computed hash
sus computed_hash []drip = cryptz.sha256_hash("password")
sus hashes_match lit = cryptz.constant_time_bytes_equal(stored_hash, computed_hash)
```

#### `constant_time_select(condition drip, true_val drip, false_val drip) drip`
Selects value based on condition in constant time.

**Parameters:**
- `condition` - Selection condition (0 or non-zero)
- `true_val` - Value if condition is non-zero
- `false_val` - Value if condition is zero

**Returns:** Selected value

**Security:** Selection doesn't leak information through timing

```cursed
sus selected drip = cryptz.constant_time_select(user_authenticated, secret_value, public_value)
```

#### `secure_zero_memory(data []drip)`
Securely overwrites sensitive data in memory.

**Parameters:**
- `data` - Array containing sensitive data

**Security:** Prevents sensitive data from remaining in memory after use

```cursed
sus sensitive_key []drip = cryptz.generate_secure_key(32)
fr fr Use key for encryption...
cryptz.secure_zero_memory(sensitive_key)  fr fr Clear from memory
```

### Encoding Utilities

#### `bytes_to_hex(data []drip) tea`
Converts byte array to hexadecimal string.

**Parameters:**
- `data` - Byte array to convert

**Returns:** Lowercase hex string

```cursed
sus hash []drip = cryptz.sha256_hash("data")
sus hex_hash tea = cryptz.bytes_to_hex(hash)
fr fr Result: "a665a45920422f9d417e4867efdc4fb8a04a1f3fff1fa07e998e86f7f7a27ae3"
```

#### `hex_to_bytes(hex_string tea) []drip`
Converts hexadecimal string to byte array.

**Parameters:**
- `hex_string` - Hex string (even length, valid hex characters)

**Returns:** Byte array, or empty array if invalid

```cursed
sus bytes []drip = cryptz.hex_to_bytes("deadbeef")
fr fr Result: [222, 173, 190, 239]
```

#### `base64_encode(data []drip) tea`
Encodes byte array as Base64 string.

**Parameters:**
- `data` - Byte array to encode

**Returns:** Base64 encoded string

```cursed
sus encoded tea = cryptz.base64_encode([72, 101, 108, 108, 111])
fr fr Result: "SGVsbG8="
```

#### `base64_decode(encoded tea) []drip`
Decodes Base64 string to byte array.

**Parameters:**
- `encoded` - Base64 encoded string

**Returns:** Decoded byte array

```cursed
sus decoded []drip = cryptz.base64_decode("SGVsbG8=")
sus text tea = stringz.from_bytes(decoded)
fr fr Result: "Hello"
```

### High-Level Convenience Functions

#### `hash_password(password tea) tea`
Securely hashes password for storage using Argon2id.

**Parameters:**
- `password` - User password

**Returns:** Password hash suitable for storage

**Security:** Uses random salt, memory-hard hashing, secure parameters

```cursed
sus user_password tea = "user_entered_password"
sus stored_hash tea = cryptz.hash_password(user_password)
fr fr Store in database: stored_hash
```

#### `verify_password(stored_hash tea, password tea) lit`
Verifies password against stored hash.

**Parameters:**
- `stored_hash` - Previously computed password hash
- `password` - Password to verify

**Returns:** `based` if password is correct, `cringe` if incorrect

**Security:** Constant-time comparison prevents timing attacks

```cursed
sus is_correct lit = cryptz.verify_password(stored_hash, entered_password)
ready is_correct {
    fr fr Grant access
} otherwise {
    fr fr Deny access
}
```

#### `encrypt_data(plaintext tea, password tea) tea`
High-level data encryption with password.

**Parameters:**
- `plaintext` - Data to encrypt
- `password` - Encryption password

**Returns:** Encrypted data as Base64 string with embedded salt

**Security:** Uses PBKDF2 key derivation, AES-GCM encryption, random salt

```cursed
sus encrypted tea = cryptz.encrypt_data("sensitive information", "strong_password")
fr fr Safe to store or transmit: encrypted
```

#### `decrypt_data(encrypted_data tea, password tea) tea`
High-level data decryption with password.

**Parameters:**
- `encrypted_data` - Previously encrypted data
- `password` - Decryption password

**Returns:** Decrypted plaintext, or empty string if decryption fails

**Security:** Authentication prevents decryption with wrong password

```cursed
sus decrypted tea = cryptz.decrypt_data(encrypted, "strong_password")
ready decrypted == "" {
    vibez.spill("Decryption failed")
} otherwise {
    vibez.spill("Decrypted:", decrypted)
}
```

## 🛡️ Security Best Practices

### Key Management
- Generate keys using `cryptz.generate_secure_key()`
- Use appropriate key sizes (AES-256: 32 bytes, HMAC: 32+ bytes)
- Store keys securely, separate from encrypted data
- Rotate keys regularly for long-term security
- Use hardware security modules (HSMs) for high-value keys

### Password Security
- Always hash passwords with `cryptz.hash_password()` (uses Argon2id)
- Never store plaintext passwords
- Use strong password policies (length, complexity)
- Implement account lockout after failed attempts
- Consider two-factor authentication

### Encryption
- Use authenticated encryption (AES-GCM) for data confidentiality and integrity
- Generate unique IVs/nonces for each encryption operation
- Protect encryption keys with same rigor as encrypted data
- Use key derivation functions for password-based encryption

### Digital Signatures
- Prefer Ed25519 for new applications (faster, more secure than RSA)
- Verify signatures before trusting signed data
- Protect private signing keys rigorously
- Consider certificate chains for public key distribution
- Use appropriate hash functions (SHA-256 minimum)

### Random Number Generation
- Use `cryptz.generate_random_bytes()` for all cryptographic purposes
- Generate sufficient entropy (minimum 128 bits for security)
- Regularly reseed random number generators
- Never use predictable seeds or weak PRNGs

### Constant-Time Operations
- Use `cryptz.constant_time_bytes_equal()` when comparing secrets
- Avoid conditional operations on secret data
- Be aware of compiler optimizations that might introduce timing leaks
- Consider using dedicated hardware for high-security operations

### Memory Security
- Clear sensitive data with `cryptz.secure_zero_memory()`
- Minimize sensitive data lifetime in memory
- Consider memory encryption for high-security applications
- Be aware of virtual memory and swap file security

## 🔍 Algorithm Details

### Hash Functions
- **SHA-256**: FIPS 180-4 standard, 256-bit output, suitable for most applications
- **SHA-512**: FIPS 180-4 standard, 512-bit output, higher security margin
- **BLAKE3**: Modern hash function, faster than SHA-2, strong security properties

### Symmetric Encryption
- **AES-GCM**: FIPS 197 + NIST SP 800-38D, authenticated encryption
- **ChaCha20**: RFC 8439, modern stream cipher, good for embedded systems

### Digital Signatures
- **Ed25519**: RFC 8032, fast elliptic curve signatures, recommended for new applications
- **RSA**: FIPS 186-4, widely supported, use minimum 2048-bit keys

### Key Derivation
- **PBKDF2**: RFC 2898, widely supported, use minimum 100,000 iterations
- **Argon2**: RFC 9106, memory-hard function, resistant to GPU attacks
- **Scrypt**: RFC 7914, memory-hard function, alternative to Argon2

## 🧪 Testing

Run the comprehensive test suite:

```bash
cursed-zig stdlib/cryptz/tests.csd
```

The test suite validates:
- **Algorithm correctness** with known test vectors
- **Security properties** (randomness, constant-time operations)
- **Error handling** and edge cases
- **Performance** with large data sets
- **Memory security** and cleanup
- **Cross-function compatibility**

## 📊 Performance Considerations

### Hash Functions
- BLAKE3 > SHA-256 > SHA-512 (speed)
- SHA-512 > BLAKE3 > SHA-256 (security margin)

### Symmetric Encryption
- ChaCha20 is faster on software-only systems
- AES-GCM is faster on systems with hardware acceleration
- Use AES-GCM for compatibility, ChaCha20 for performance

### Key Derivation
- Argon2 provides best security against GPU attacks
- PBKDF2 is most widely supported
- Adjust parameters based on available resources and security requirements

### Memory Usage
- Argon2 uses configurable memory (default 64MB)
- Scrypt memory usage depends on N parameter
- Large key schedules for AES-256 vs AES-128

## 🔒 Security Compliance

### Standards Compliance
- **FIPS 140-2 Level 1**: Approved algorithms and key sizes
- **NIST Guidelines**: Follows current recommendations
- **RFC Standards**: Implements standard protocols correctly

### Security Validations
- **Memory Safety**: Pure CURSED implementation, no buffer overflows
- **Constant-Time**: Critical operations resist timing attacks
- **Entropy Quality**: High-quality random number generation
- **Algorithm Integrity**: Implements algorithms correctly

### Limitations
- Software-only implementation (no hardware security modules)
- Side-channel attacks may be possible in some environments
- Requires proper key management practices
- Not audited for government or financial applications

## 🚀 Migration Guide

### From Basic Crypto
```cursed
fr fr Old basic crypto
sus hash tea = simple_hash("data")

fr fr New cryptz module
yeet "cryptz"
sus hash []drip = cryptz.sha256_hash("data")
sus hex_hash tea = cryptz.bytes_to_hex(hash)
```

### From External Crypto Libraries
```cursed
fr fr Replace external crypto calls
fr fr OLD: extern_aes_encrypt(data, key)
sus aes_key []drip = cryptz.generate_secure_key(32)
sus encrypted []drip = cryptz.aes_gcm_encrypt(data, aes_key, "")

fr fr OLD: extern_pbkdf2(password, salt, iterations)
sus derived_key []drip = cryptz.pbkdf2_derive_key(password, salt, 100000, 32)
```

### Upgrading Key Sizes
- **RSA**: Upgrade from 1024 to 2048+ bits
- **AES**: Use 256-bit keys instead of 128-bit
- **Hash**: Migrate from MD5/SHA-1 to SHA-256+
- **PBKDF2**: Increase iterations to 100,000+

## 📝 Examples

### Complete Encryption/Decryption Flow
```cursed
yeet "cryptz"

fr fr 1. Generate secure key
sus encryption_key []drip = cryptz.generate_secure_key(32)

fr fr 2. Encrypt sensitive data
sus secret_document tea = "Confidential business plan"
sus encrypted_document []drip = cryptz.aes_gcm_encrypt(
    secret_document, encryption_key, "document_type:business_plan"
)

fr fr 3. Store encrypted data safely
sus encoded_encrypted tea = cryptz.base64_encode(encrypted_document)
vibez.spill("Encrypted (safe to store):", encoded_encrypted)

fr fr 4. Later: decrypt the data
sus stored_encrypted []drip = cryptz.base64_decode(encoded_encrypted)
sus decrypted_document []drip = cryptz.aes_gcm_decrypt(
    stored_encrypted, encryption_key, "document_type:business_plan"
)

ready len(decrypted_document) > 0 {
    sus plaintext tea = stringz.from_bytes(decrypted_document)
    vibez.spill("Decrypted document:", plaintext)
} otherwise {
    vibez.spill("Decryption failed - wrong key or tampered data")
}

fr fr 5. Securely clear sensitive key
cryptz.secure_zero_memory(encryption_key)
```

### Digital Signature Workflow
```cursed
yeet "cryptz"

fr fr 1. Generate signing keypair
sus signing_keypair cryptz.KeyPair = cryptz.ed25519_generate_keypair()

fr fr 2. Create digital signature
sus contract tea = "Agreement between parties A and B..."
sus signature []drip = cryptz.ed25519_sign(contract, signing_keypair.private_key)

fr fr 3. Distribute public key and signature
sus public_key_hex tea = cryptz.bytes_to_hex(signing_keypair.public_key)
sus signature_hex tea = cryptz.bytes_to_hex(signature)

vibez.spill("Public Key:", public_key_hex)
vibez.spill("Signature:", signature_hex)

fr fr 4. Verify signature (by recipient)
sus received_public_key []drip = cryptz.hex_to_bytes(public_key_hex)
sus received_signature []drip = cryptz.hex_to_bytes(signature_hex)

sus is_authentic lit = cryptz.ed25519_verify(
    contract, received_signature, received_public_key
)

ready is_authentic {
    vibez.spill("✅ Contract signature is valid")
} otherwise {
    vibez.spill("❌ Contract signature is INVALID")
}
```

### Secure Password Storage
```cursed
yeet "cryptz"

fr fr User registration
sus user_password tea = "user_entered_password_123!"
sus hashed_password tea = cryptz.hash_password(user_password)

fr fr Store in database (safe to store)
vibez.spill("Store in DB:", hashed_password)

fr fr User login
sus entered_password tea = "user_entered_password_123!"
sus is_valid_login lit = cryptz.verify_password(hashed_password, entered_password)

ready is_valid_login {
    vibez.spill("✅ Login successful")
} otherwise {
    vibez.spill("❌ Invalid password")
}

fr fr Wrong password attempt
sus wrong_password tea = "wrong_password"
sus is_invalid lit = cryptz.verify_password(hashed_password, wrong_password)
assert_false(is_invalid)  fr fr Should be false
```

### Key Derivation for Multiple Keys
```cursed
yeet "cryptz"

fr fr Derive multiple keys from single master password
sus master_password tea = "master_secret_password"
sus salt []drip = cryptz.generate_random_bytes(32)

fr fr Derive encryption key
sus encryption_key []drip = cryptz.pbkdf2_derive_key(
    master_password, salt, 100000, 32
)

fr fr Derive HMAC key (using different salt)
sus hmac_salt []drip = cryptz.generate_random_bytes(32)
sus hmac_key []drip = cryptz.pbkdf2_derive_key(
    master_password, hmac_salt, 100000, 32
)

fr fr Use derived keys
sus message tea = "Sensitive data"
sus encrypted []drip = cryptz.aes_gcm_encrypt(message, encryption_key, "")
sus mac []drip = cryptz.hmac_sha256(hmac_key, encrypted)

vibez.spill("Encrypted data:", cryptz.bytes_to_hex(encrypted))
vibez.spill("Authentication tag:", cryptz.bytes_to_hex(mac))

fr fr Securely clear keys
cryptz.secure_zero_memory(encryption_key)
cryptz.secure_zero_memory(hmac_key)
```

## 🤝 Contributing

When contributing to the cryptz module:

1. **Security First**: All changes must maintain or improve security
2. **Test Coverage**: Add comprehensive tests for new functionality
3. **Documentation**: Update documentation for user-facing changes
4. **Code Review**: Security-sensitive code requires thorough review
5. **Standards Compliance**: Follow established cryptographic standards

### Security Vulnerability Reporting

Report security vulnerabilities privately to the maintainers. Do not create public issues for security vulnerabilities.

## 📄 License

This cryptographic implementation is provided under the same license as the CURSED language. It is intended for educational, research, and production use with appropriate security considerations.

## ⚠️ Security Disclaimer

**Important**: While this implementation follows cryptographic best practices and industry standards, it has not undergone formal security audits. For mission-critical applications involving sensitive data, financial transactions, or national security, conduct independent security audits and consider using formally verified cryptographic libraries.

The implementers make no warranty about the security properties of this library beyond documented compliance with published standards.

---

**Cryptz Module**: Production-ready cryptographic operations for the CURSED language  
**Version**: 2.0  
**Status**: Production Ready  
**Standards**: FIPS 140-2 Level 1 Compliant
