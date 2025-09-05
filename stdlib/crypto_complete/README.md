# Comprehensive Cryptography Module

Production-grade cryptographic functions implemented in pure CURSED without FFI dependencies.

## Features

### 🔐 Secure Hash Functions
- **SHA-256**: Industry-standard 256-bit cryptographic hash
- **SHA-512**: High-security 512-bit cryptographic hash
- Full compliance with FIPS 180-4 standard
- Optimized for performance and security

### 🔒 Symmetric Encryption
- **AES-256**: Advanced Encryption Standard with 256-bit keys
- CBC mode with PKCS#7 padding
- Secure block cipher implementation
- Constant-time operations to prevent side-channel attacks

### ✍️ Digital Signatures
- **RSA-2048**: Industry-standard public key cryptography
- PSS padding for enhanced security
- Key generation, signing, and verification
- Protection against timing and padding oracle attacks

### 🎲 Secure Random Number Generation
- Cryptographically secure random byte generation
- Entropy gathering from system sources
- Secure random integers and strings
- Distribution testing and validation

### 🛡️ Cryptographic Utilities
- **HMAC-SHA256**: Message authentication codes
- **PBKDF2**: Password-based key derivation
- Constant-time string comparison
- Secure memory wiping
- Input validation and sanitization

## Usage Examples

### Hash Functions
```cursed
yeet "crypto_complete"

# SHA-256 hashing
sus message tea = "Hello, World!"
sus hash tea = sha256_hash(message)
vibez.spill("SHA-256: " + hash)

# SHA-512 hashing
sus secure_hash tea = sha512_hash(message)
vibez.spill("SHA-512: " + secure_hash)
```

### AES Encryption
```cursed
# AES-256 encryption/decryption
sus plaintext tea = "Confidential data"
sus key tea = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"

sus ciphertext tea = aes_encrypt(plaintext, key)
sus decrypted tea = aes_decrypt(ciphertext, key)

vibez.spill("Original: " + plaintext)
vibez.spill("Encrypted: " + ciphertext)
vibez.spill("Decrypted: " + decrypted)
```

### RSA Digital Signatures
```cursed
# Generate RSA key pair
sus (public_key, private_key) = rsa_generate_keypair()

# Sign a message
sus message tea = "Important document"
sus signature tea = rsa_sign(message, private_key)

# Verify signature
sus is_valid lit = rsa_verify(message, signature, public_key)
vibez.spill("Signature valid: " + (is_valid ? "Yes" : "No"))
```

### Secure Random Generation
```cursed
# Generate random bytes
sus random_bytes tea = secure_random_bytes(32)
vibez.spill("Random bytes: " + random_bytes)

# Generate random integers
sus random_number normie = secure_random_int(1, 1000)
vibez.spill("Random number: " + int_to_string(random_number))

# Generate random strings
sus charset tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
sus random_password tea = secure_random_string(16, charset)
vibez.spill("Random password: " + random_password)
```

### Cryptographic Utilities
```cursed
# HMAC authentication
sus key tea = "secret_key"
sus data tea = "authenticated_data"
sus mac tea = hmac_sha256(key, data)
vibez.spill("HMAC: " + mac)

# Key derivation
sus password tea = "user_password"
sus salt tea = secure_random_bytes(16)
sus derived_key tea = pbkdf2_derive_key(password, salt, 10000, 32)
vibez.spill("Derived key: " + derived_key)

# Secure comparison
sus token1 tea = "authentication_token"
sus token2 tea = "authentication_token"
sus tokens_equal lit = constant_time_compare(token1, token2)
vibez.spill("Tokens equal: " + (tokens_equal ? "Yes" : "No"))
```

## Security Features

### Timing Attack Prevention
- Constant-time string comparisons
- Consistent execution paths for cryptographic operations
- Side-channel attack mitigation

### Memory Security
- Secure memory wiping for sensitive data
- Proper cleanup of cryptographic keys
- Protection against memory disclosure

### Input Validation
- Comprehensive parameter validation
- Length checking and bounds verification
- Secure error handling

### Entropy Management
- High-quality entropy sources
- Proper entropy pool management
- Regular entropy refreshing

## API Reference

### Hash Functions
- `sha256_hash(input tea) tea`: Compute SHA-256 hash
- `sha512_hash(input tea) tea`: Compute SHA-512 hash

### Symmetric Encryption
- `aes_encrypt(plaintext tea, key tea) tea`: AES-256 encryption
- `aes_decrypt(ciphertext tea, key tea) tea`: AES-256 decryption

### Digital Signatures
- `rsa_generate_keypair() (tea, tea)`: Generate RSA key pair
- `rsa_sign(message tea, private_key tea) tea`: Sign message
- `rsa_verify(message tea, signature tea, public_key tea) lit`: Verify signature

### Random Generation
- `secure_random_bytes(length normie) tea`: Generate random bytes
- `secure_random_int(min normie, max normie) normie`: Generate random integer
- `secure_random_string(length normie, charset tea) tea`: Generate random string

### Utilities
- `constant_time_compare(a tea, b tea) lit`: Timing-safe comparison
- `pbkdf2_derive_key(password tea, salt tea, iterations normie, key_length normie) tea`: Key derivation
- `hmac_sha256(key tea, message tea) tea`: HMAC authentication
- `crypto_secure_wipe(data tea) lit`: Secure memory wipe
- `crypto_validate_input(input tea, min_length normie, max_length normie) lit`: Input validation
- `crypto_timing_safe_equal(expected tea, actual tea) lit`: Timing-safe equality

## Testing

Run comprehensive tests to verify all cryptographic functions:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/crypto_complete/test_crypto_complete.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/crypto_complete/test_crypto_complete.💀
./test_crypto_complete
```

## Security Compliance

This module implements cryptographic standards and best practices:

- **FIPS 180-4**: SHA-256 and SHA-512 implementations
- **FIPS 197**: AES-256 encryption standard
- **RFC 3447**: RSA PKCS #1 v2.1 with PSS padding
- **RFC 2104**: HMAC authentication
- **RFC 2898**: PBKDF2 key derivation
- **NIST SP 800-90A**: Cryptographically secure random number generation

## Performance

Optimized for production use with:
- Efficient algorithm implementations
- Minimal memory allocation
- Constant-time critical operations
- Suitable for high-throughput applications

## Security Considerations

- Always use secure random number generation for keys and salts
- Implement proper key management and rotation
- Use appropriate key lengths (AES-256, RSA-2048+)
- Validate all inputs and handle errors securely
- Clear sensitive data from memory after use
- Use timing-safe operations for authentication
- Regularly update and audit cryptographic implementations

This module provides enterprise-grade cryptographic functionality suitable for production applications requiring strong security guarantees.
