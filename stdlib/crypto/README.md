# CURSED Crypto Library Tests

This directory contains comprehensive tests for the CURSED cryptography standard library.

## Test Coverage

The `test_crypto.csd` file provides complete test coverage for all crypto functions:

### Hash Functions
- `crypto_sha256()` - SHA-256 hashing
- `crypto_sha512()` - SHA-512 hashing
- `crypto_md5()` - MD5 hashing (legacy)
- `crypto_blake3()` - BLAKE3 hashing

### Random Generation
- `crypto_random_bytes()` - Cryptographically secure random bytes
- `crypto_random_int()` - Secure random integers
- `crypto_random_string()` - Secure random strings
- `crypto_secure_random()` - Secure random floats

### Base Encoding
- `crypto_base64_encode()` / `crypto_base64_decode()` - Base64 encoding/decoding
- `crypto_hex_encode()` / `crypto_hex_decode()` - Hexadecimal encoding/decoding

### Symmetric Encryption
- `crypto_aes_encrypt()` / `crypto_aes_decrypt()` - AES encryption/decryption

### Message Authentication
- `crypto_hmac_sha256()` - HMAC with SHA-256
- `crypto_hmac_sha512()` - HMAC with SHA-512

### Key Derivation
- `crypto_pbkdf2()` - PBKDF2 key derivation
- `crypto_scrypt()` - Scrypt key derivation

### Digital Signatures
- `crypto_ed25519_keypair()` - Generate Ed25519 key pair
- `crypto_ed25519_sign()` - Sign with Ed25519
- `crypto_ed25519_verify()` - Verify Ed25519 signature

### Password Hashing
- `crypto_argon2_hash()` / `crypto_argon2_verify()` - Argon2 password hashing
- `crypto_bcrypt_hash()` / `crypto_bcrypt_verify()` - bcrypt password hashing

### Security Utilities
- `crypto_constant_time_eq()` - Constant-time string comparison
- `crypto_generate_salt()` - Generate cryptographic salt

### Edge Cases Tested
- Empty input handling
- Large data processing
- Invalid key/data combinations
- Round-trip encoding/decoding
- Key derivation with different parameters

## Security Considerations

The crypto library implements industry-standard cryptographic algorithms:

- **Hash Functions**: Collision-resistant, suitable for integrity checking
- **Random Generation**: Cryptographically secure pseudorandom number generation
- **Encryption**: AES with proper key handling
- **Password Hashing**: Argon2 and bcrypt with salt and cost parameters
- **Digital Signatures**: Ed25519 for high-security applications
- **Key Derivation**: PBKDF2 and Scrypt for password-based key generation

## Running Tests

```bash
# Run crypto tests specifically
cargo run --bin cursed stdlib/crypto/test_crypto.csd

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct cryptographic operations
- Proper key generation and management
- Secure random number generation
- Round-trip encryption/decryption
- Password hashing verification
- Digital signature validation

The tests ensure that cryptographic functions maintain security properties and work correctly in both interpretation and native compilation modes.

## Important Notes

- **Never use MD5 for security-critical applications** - included for legacy compatibility only
- **Always use proper salt for password hashing**
- **Verify digital signatures before trusting signed data**
- **Use constant-time comparison for sensitive string comparisons**
- **Generate keys using cryptographically secure random functions**
