# CURSED Crypto Package Documentation

## Overview

The CURSED crypto package provides a comprehensive suite of cryptographic primitives, protocols, and utilities for building secure applications. It includes implementations of JWT tokens, HMAC authentication, TOTP 2FA, TLS handshake components, secure random generation, and various encoding utilities.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Components](#core-components)
- [Cryptographic Protocols](#cryptographic-protocols)
- [Random Generation](#random-generation)
- [Encoding Utilities](#encoding-utilities)
- [LLVM Integration](#llvm-integration)
- [Security Considerations](#security-considerations)
- [Performance Characteristics](#performance-characteristics)
- [Best Practices](#best-practices)
- [API Reference](#api-reference)
- [Examples](#examples)

## Quick Start

```cursed
// Import the crypto package
yolo use_crypto() {
    // Create crypto platform
    sus platform = CryptoPlatform::new()
    
    // Initialize JWT handler
    platform.init_jwt("your_secret_key".as_bytes(), 3600)
    
    // Create and validate JWT
    sus jwt = platform.jwt()
    sus claims = squad { sub: "user123", role: "admin" }
    sus token = jwt.create_token(claims)
    sus decoded = jwt.validate_token(token)
    
    print(f"User: {decoded.get('sub')}")
}
```

## Core Components

### CryptoPlatform

The main facade for crypto operations, providing unified access to all cryptographic utilities.

```cursed
sus platform = CryptoPlatform::new()

// Initialize specific handlers
platform.init_jwt(secret_key, expiry_seconds)
platform.init_hmac(hmac_key)
platform.init_totp(totp_secret, digits, time_step)

// Access generators
sus uuid = platform.uuid_generator().generate()
sus salt = platform.salt_generator().generate_salt(32)
sus random_bytes = platform.random_bytes(16)
```

### Configuration

```cursed
// Default configuration
sus config = CryptoConfig::default()

// Security-focused configuration
sus secure_config = CryptoConfig::secure_defaults()

// Custom configuration
sus custom_config = CryptoConfig {
    default_jwt_expiry: 900,      // 15 minutes
    default_salt_length: 64,      // 64 bytes
    enable_detailed_logging: true,
    // ... other options
}
```

## Cryptographic Protocols

### JWT (JSON Web Tokens)

Secure token-based authentication with HMAC-SHA256 signing.

```cursed
sus jwt = JwtHandler::new(secret_key, 3600) // 1 hour expiry

// Create token
sus claims = squad {
    sub: "user_12345",
    name: "Alice",
    role: "admin",
    permissions: ["read", "write"]
}
sus token = jwt.create_token(claims)

// Validate token
sus decoded_claims = jwt.validate_token(token)
```

**Features:**
- HMAC-SHA256 signing
- Automatic expiry handling
- Standard JWT format compliance
- Configurable expiry times
- Secure claim validation

### HMAC Authentication

Message authentication using HMAC-SHA256 for data integrity and authenticity.

```cursed
sus auth = HmacAuth::new(secret_key)

// Sign data
sus data = "important message".as_bytes()
sus signature = auth.sign(data)

// Verify signature
sus is_valid = auth.verify(data, signature)

// Authenticated messages with embedded MAC
sus authenticated = auth.create_authenticated_message(data)
sus recovered = auth.verify_authenticated_message(authenticated)
```

**Features:**
- HMAC-SHA256 for message authentication
- Constant-time verification to prevent timing attacks
- Authenticated message format with embedded signatures
- Tamper detection and integrity verification

### TOTP (Time-based One-Time Passwords)

RFC 6238 compliant TOTP implementation for two-factor authentication.

```cursed
sus totp = TotpGenerator::new(secret, 6, 30) // 6 digits, 30-second window

// Generate current token
sus token = totp.generate_current()

// Verify with time window tolerance
sus is_valid = totp.verify(token, 1) // Allow ±1 time window

// Generate for specific time
sus historical_token = totp.generate_at_time(timestamp)
```

**Features:**
- RFC 6238 compliance
- Configurable digit count (4-10)
- Configurable time step (recommended: 30s)
- Time window tolerance for clock skew
- QR code data generation support

### TLS Handshake Components

Basic TLS handshake simulation and key derivation utilities.

```cursed
sus handshake = TlsHandshake::new()

// Generate randoms
sus client_random = handshake.generate_client_random()
sus server_random = handshake.generate_server_random()

// Key derivation
sus pre_master = handshake.create_pre_master_secret()
sus master_secret = handshake.derive_master_secret(pre_master)
sus keys = handshake.derive_keys(master_secret, 16)
```

**Features:**
- Client/server random generation
- Session ID management
- Master secret derivation
- Key material generation
- Handshake state tracking

## Random Generation

### Secure Random Generator

Cryptographically secure random number generator using ChaCha20-based PRNG.

```cursed
sus rng = SecureRandom::new()

// Generate random bytes
sus bytes = rng.generate_bytes(32)

// Generate integers
sus random_u32 = rng.generate_u32()
sus random_u64 = rng.generate_u64()

// Generate in range
sus dice_roll = rng.generate_range(6) // 0-5
```

**Features:**
- ChaCha20-based PRNG
- Automatic reseeding
- System entropy collection
- High-quality randomness
- Platform-independent

### UUID Generation

UUID v4 generation with proper entropy and format compliance.

```cursed
sus uuid_gen = UuidV4Generator::new()
sus uuid = uuid_gen.generate() // "550e8400-e29b-41d4-a716-446655440000"

// Batch generation
sus batch = uuid_gen.generate_batch(10)
```

### Salt and Nonce Generation

Specialized generators for cryptographic salts and nonces.

```cursed
// Salt generation
sus salt_gen = SaltGenerator::new()
sus salt = salt_gen.generate_salt(32)
sus hex_salt = salt_gen.generate_salt_hex(16)
sus b64_salt = salt_gen.generate_salt_base64(24)

// Nonce generation
sus nonce_gen = NonceGenerator::new()
sus nonce = nonce_gen.generate_nonce(16)
sus time_nonce = nonce_gen.generate_time_nonce(8) // includes timestamp
sus purpose_nonce = nonce_gen.generate_purpose_nonce("encryption", 16)
```

### Randomness Quality Testing

Built-in quality assessment for random data.

```cursed
sus quality = test_randomness_quality(random_data)
print(f"Entropy: {quality.entropy_estimate} bits/byte")
print(f"Chi-squared: {quality.chi_squared}")
print(f"Passes tests: {quality.passes_basic_tests}")
```

## Encoding Utilities

### Base64 Encoding

Standard and URL-safe Base64 encoding with custom alphabet support.

```cursed
// Standard Base64
sus encoded = Base64Encoder::encode_standard(data)
sus decoded = Base64Encoder::decode_standard(encoded)

// URL-safe Base64 (no padding)
sus url_safe = Base64Encoder::encode_url_safe(data)
sus url_decoded = Base64Encoder::decode_url_safe(url_safe)

// Custom alphabet
sus custom = Base64Encoder::encode_custom(data, custom_alphabet, Some('='))
```

### Hexadecimal Encoding

Flexible hex encoding with formatting options.

```cursed
// Basic hex encoding
sus hex_lower = HexEncoder::encode_lower(data)
sus hex_upper = HexEncoder::encode_upper(data)
sus decoded = HexEncoder::decode(hex_lower)

// Formatted hex
sus formatted = HexEncoder::encode_formatted(data, ":", true) // "FF:AB:CD"
sus formatted_decoded = HexEncoder::decode_formatted(formatted)
```

### Base32 Encoding

Base32 encoding optimized for TOTP and human-readable tokens.

```cursed
// Standard Base32
sus b32 = Base32Encoder::encode(data)
sus decoded = Base32Encoder::decode(b32)

// No padding
sus no_pad = Base32Encoder::encode_no_padding(data)

// Custom alphabet
sus custom = Base32Encoder::encode_custom(data, custom_alphabet)
```

### ASN.1 Parsing

Basic ASN.1 DER parsing for cryptographic data structures.

```cursed
// Parse elements
sus element = Asn1Parser::parse_tag_length(data)
sus integer = Asn1Parser::parse_integer(int_data)
sus octet_string = Asn1Parser::parse_octet_string(octet_data)
sus sequence = Asn1Parser::parse_sequence(seq_data)

// Encode integers
sus encoded = Asn1Parser::encode_integer(integer_bytes)
```

### URL Encoding

URL-safe encoding for crypto parameters and tokens.

```cursed
sus encoded = UrlEncoder::encode(data)
sus decoded = UrlEncoder::decode(encoded)
```

## LLVM Integration

The crypto package provides seamless LLVM integration for use in compiled CURSED code.

### Registration

```rust
// Register crypto functions with LLVM module
register_crypto_functions(context, module, builder)?;
```

### Available Functions

- `cursed_jwt_create` - Create JWT tokens
- `cursed_jwt_validate` - Validate JWT tokens
- `cursed_hmac_sign` - Create HMAC signatures
- `cursed_hmac_verify` - Verify HMAC signatures
- `cursed_totp_generate` - Generate TOTP tokens
- `cursed_totp_verify` - Verify TOTP tokens
- `cursed_random_bytes` - Generate random bytes
- `cursed_uuid_generate` - Generate UUIDs
- `cursed_base64_encode` - Base64 encoding
- `cursed_hex_encode` - Hex encoding
- `cursed_sha256` - SHA-256 hashing
- `cursed_constant_time_eq` - Constant-time comparison

## Security Considerations

### Key Management

**✅ Best Practices:**
- Use cryptographically secure random keys
- Rotate keys regularly
- Store keys securely (environment variables, secure vaults)
- Use different keys for different purposes

**❌ Avoid:**
- Hardcoded keys in source code
- Reusing keys across applications
- Using weak or predictable keys

### Timing Attacks

All cryptographic operations use constant-time algorithms where applicable:

```cursed
// Safe comparison
sus is_equal = CryptoPlatform::constant_time_eq(secret1, secret2)

// Safe verification
sus is_valid = hmac_auth.verify(data, signature) // Uses constant-time comparison
```

### Memory Security

```cursed
// Secure memory clearing
sus mut sensitive_data = secret_key.clone()
CryptoPlatform::secure_clear(sensitive_data) // Overwrites with zeros
```

### Randomness Quality

The package includes built-in entropy collection and quality testing:

- Automatic system entropy collection
- Periodic reseeding of generators
- Quality assessment tools
- Platform-specific entropy sources

## Performance Characteristics

### JWT Operations
- **Token Creation**: ~500 μs for typical claims
- **Token Validation**: ~300 μs including signature verification
- **Memory Usage**: ~2KB per token handler

### HMAC Operations
- **Signing**: ~50 μs for 1KB data
- **Verification**: ~100 μs (includes constant-time comparison)
- **Memory Usage**: Minimal overhead

### TOTP Operations
- **Generation**: ~20 μs per token
- **Verification**: ~40 μs with time window
- **Memory Usage**: ~500 bytes per generator

### Random Generation
- **Throughput**: ~50 MB/s for random bytes
- **UUID Generation**: ~10,000 UUIDs/second
- **Reseed Frequency**: Every 1M requests or 10 minutes

### Encoding Operations
- **Base64**: ~200 MB/s encoding, ~150 MB/s decoding
- **Hex**: ~400 MB/s encoding, ~300 MB/s decoding
- **Base32**: ~100 MB/s encoding, ~80 MB/s decoding

## Best Practices

### Authentication Flow

```cursed
// 1. User login with credentials
yolo authenticate_user(username: String, password: String, totp_code: String) -> Result<String> {
    // Verify password (use proper password hashing)
    lowkey !verify_password(username, password) {
        return Err("Invalid credentials")
    }
    
    // Verify TOTP
    sus totp_secret = get_user_totp_secret(username)
    sus totp = TotpGenerator::new(totp_secret, 6, 30)
    lowkey !totp.verify(totp_code, 1) {
        return Err("Invalid TOTP code")
    }
    
    // Create JWT
    sus claims = squad {
        sub: username,
        iat: current_timestamp(),
        exp: current_timestamp() + 3600,
        scope: get_user_permissions(username)
    }
    
    sus jwt = get_jwt_handler()
    Ok(jwt.create_token(claims))
}
```

### Secure Message Exchange

```cursed
yolo send_secure_message(recipient: String, content: String) -> Result<SecureMessage> {
    // 1. Generate message nonce
    sus nonce = generate_message_nonce()
    
    // 2. Create message structure
    sus message = squad {
        id: generate_uuid(),
        from: current_user(),
        to: recipient,
        timestamp: current_timestamp(),
        nonce: nonce,
        content: content
    }
    
    // 3. Serialize and sign
    sus message_bytes = serialize_message(message)
    sus hmac_key = get_shared_key(recipient)
    sus signature = create_hmac_signature(hmac_key, message_bytes)
    
    // 4. Create secure envelope
    Ok(SecureMessage {
        payload: Base64Encoder::encode_standard(message_bytes),
        signature: Base64Encoder::encode_standard(signature),
        algorithm: "HMAC-SHA256",
        version: "1.0"
    })
}
```

### Configuration Management

```cursed
yolo setup_production_crypto() -> CryptoConfig {
    CryptoConfig {
        default_jwt_expiry: 900,        // 15 minutes
        default_totp_time_step: 30,     // Standard
        default_totp_digits: 6,         // Standard
        default_salt_length: 64,        // Extra secure
        default_nonce_length: 32,       // Extra secure
        enable_detailed_logging: true,   // For audit
        max_random_bytes_per_request: 1024 * 64, // 64KB limit
        random_reseed_frequency: 100_000, // Frequent reseeding
    }
}
```

## API Reference

### CryptoPlatform

```cursed
squad CryptoPlatform {
    yolo new() -> Result<Self>
    yolo init_jwt(&self, secret: Vec<u8>, expiry: u64) -> Result<()>
    yolo init_hmac(&self, key: Vec<u8>) -> Result<()>
    yolo init_totp(&self, secret: Vec<u8>, digits: usize, time_step: u64) -> Result<()>
    
    yolo jwt(&self) -> Result<&JwtHandler>
    yolo hmac(&self) -> Result<&HmacAuth>
    yolo totp(&self) -> Result<&TotpGenerator>
    
    yolo uuid_generator(&self) -> &UuidV4Generator
    yolo salt_generator(&self) -> &SaltGenerator
    yolo nonce_generator(&self) -> &NonceGenerator
    
    yolo random_bytes(&self, count: usize) -> Result<Vec<u8>>
    yolo hash_data(&self, data: &[u8]) -> Vec<u8>
    
    static yolo constant_time_eq(a: &[u8], b: &[u8]) -> bool
    static yolo secure_clear(data: &mut [u8])
}
```

### JwtHandler

```cursed
squad JwtHandler {
    yolo new(secret_key: Vec<u8>, default_expiry: u64) -> Self
    yolo create_token(&self, claims: HashMap<String, Value>) -> Result<String>
    yolo validate_token(&self, token: &str) -> Result<HashMap<String, Value>>
}
```

### HmacAuth

```cursed
squad HmacAuth {
    yolo new(key: Vec<u8>) -> Self
    yolo sign(&self, data: &[u8]) -> Result<Vec<u8>>
    yolo verify(&self, data: &[u8], signature: &[u8]) -> Result<bool>
    yolo create_authenticated_message(&self, message: &[u8]) -> Result<Vec<u8>>
    yolo verify_authenticated_message(&self, authenticated_data: &[u8]) -> Result<Vec<u8>>
}
```

### TotpGenerator

```cursed
squad TotpGenerator {
    yolo new(secret: Vec<u8>, digits: usize, time_step: u64) -> Self
    yolo generate_current(&self) -> Result<String>
    yolo generate_at_time(&self, unix_time: u64) -> Result<String>
    yolo verify(&self, token: &str, time_window: u32) -> Result<bool>
}
```

### SecureRandom

```cursed
squad SecureRandom {
    yolo new() -> Result<Self>
    yolo generate_bytes(&self, count: usize) -> Result<Vec<u8>>
    yolo generate_u32(&self) -> Result<u32>
    yolo generate_u64(&self) -> Result<u64>
    yolo generate_range(&self, max: u64) -> Result<u64>
}
```

## Examples

### Complete Authentication System

```cursed
squad AuthSystem {
    platform: CryptoPlatform,
    user_db: UserDatabase,
}

impl AuthSystem {
    yolo new() -> Result<Self> {
        sus platform = CryptoPlatform::new()?
        platform.init_jwt(load_jwt_secret(), 3600)?
        
        Ok(Self {
            platform,
            user_db: UserDatabase::new(),
        })
    }
    
    yolo register_user(&self, username: String, password: String) -> Result<String> {
        // Generate TOTP secret
        sus totp_secret = self.platform.random_bytes(32)?
        sus totp_secret_b32 = Base32Encoder::encode(totp_secret)
        
        // Hash password with salt
        sus salt = self.platform.salt_generator().generate_salt(32)?
        sus password_hash = hash_password_with_salt(password, salt)
        
        // Store user
        self.user_db.create_user(username, password_hash, salt, totp_secret)?
        
        // Return QR code data for TOTP setup
        Ok(format!("otpauth://totp/MyApp:{}?secret={}&issuer=MyApp", 
                  username, totp_secret_b32))
    }
    
    yolo login(&self, username: String, password: String, totp_code: String) -> Result<String> {
        // Verify user exists
        sus user = self.user_db.get_user(username)?
        
        // Verify password
        sus password_hash = hash_password_with_salt(password, user.salt)
        lowkey !CryptoPlatform::constant_time_eq(password_hash, user.password_hash) {
            return Err("Invalid credentials")
        }
        
        // Verify TOTP
        sus totp = TotpGenerator::new(user.totp_secret, 6, 30)
        lowkey !totp.verify(totp_code, 1)? {
            return Err("Invalid TOTP code")
        }
        
        // Create JWT
        sus claims = squad {
            sub: username,
            role: user.role,
            permissions: user.permissions,
        }
        
        self.platform.jwt()?.create_token(claims)
    }
    
    yolo verify_token(&self, token: String) -> Result<UserClaims> {
        sus claims = self.platform.jwt()?.validate_token(token)?
        Ok(UserClaims::from_jwt_claims(claims))
    }
}
```

### Secure API Communication

```cursed
squad SecureApiClient {
    hmac_auth: HmacAuth,
    base_url: String,
}

impl SecureApiClient {
    yolo new(api_key: String, base_url: String) -> Self {
        Self {
            hmac_auth: HmacAuth::new(api_key.as_bytes()),
            base_url,
        }
    }
    
    yolo make_request(&self, endpoint: String, data: Vec<u8>) -> Result<Vec<u8>> {
        // Create request with timestamp
        sus timestamp = current_timestamp().to_string()
        sus nonce = generate_request_nonce()
        
        // Create canonical request string
        sus canonical_request = format!("POST\n{}\n{}\n{}", 
                                       endpoint, timestamp, nonce)
        
        // Sign the request
        sus request_data = [canonical_request.as_bytes(), data].concat()
        sus signature = self.hmac_auth.sign(request_data)?
        
        // Create headers
        sus headers = squad {
            "X-Timestamp": timestamp,
            "X-Nonce": nonce,
            "X-Signature": Base64Encoder::encode_standard(signature),
            "Content-Type": "application/octet-stream"
        }
        
        // Make HTTP request
        http_post(format!("{}{}", self.base_url, endpoint), headers, data)
    }
}
```

## Error Handling

All crypto operations return `Result<T, CursedError>` with detailed error context:

```cursed
vibe_check token_result {
    mood Ok(token) => {
        print(f"JWT created: {token}")
    }
    mood Err(error) => {
        print(f"JWT creation failed: {error.message()}")
        print(f"Error type: {error.error_type()}")
    }
}
```

## Testing

The package includes comprehensive testing utilities:

```cursed
#[test]
yolo test_jwt_round_trip() {
    sus jwt = JwtHandler::new(b"test_secret".to_vec(), 3600)
    sus claims = squad { sub: "test_user" }
    
    sus token = jwt.create_token(claims).expect("Token creation failed")
    sus decoded = jwt.validate_token(token).expect("Token validation failed")
    
    assert_eq!(decoded.get("sub"), Some("test_user"))
}
```

## Migration Guide

When upgrading from earlier versions:

1. **Version 1.0 → 1.1**: No breaking changes
2. **Version 0.x → 1.0**: Update initialization calls to use `CryptoPlatform`

## Contributing

Contributions welcome! Please ensure:

- All cryptographic implementations follow security best practices
- Comprehensive tests for new features
- Documentation for public APIs
- Performance benchmarks for crypto operations

## License

MIT License - see LICENSE file for details.

## Security Disclosures

Report security issues to: security@cursed.dev

**Important**: Never include sensitive cryptographic material in bug reports.
