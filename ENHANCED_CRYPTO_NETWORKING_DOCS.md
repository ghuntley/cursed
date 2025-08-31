# CURSED Enhanced Crypto & Networking Documentation

## Overview

This document describes the enhanced cryptographic and networking capabilities added to CURSED's standard library. These modules provide application-level security and networking functions while maintaining pure CURSED compatibility for self-hosting.

## Enhanced Crypto Module (`stdlib/crypto_enhanced/mod.csd`)

### Functions Implemented

#### Hash Functions
- `hash_sha256(data tea) tea` - SHA-256 hashing using existing crypto module
- `hash_md5(data tea) tea` - MD5 hashing implementation (deprecated for security)

#### Encryption Functions  
- `encrypt_simple(data tea, key tea) tea` - Simple XOR-based encryption with key expansion
- `decrypt_simple(data tea, key tea) tea` - Corresponding decryption function

#### Random Generation
- `generate_random(length drip) tea` - Generate random string with printable characters

### Security Considerations

#### ✅ Production Ready
- **SHA-256**: Cryptographically secure, suitable for production use
- **Base crypto module**: Uses AES-256, ChaCha20, HMAC-SHA256 for real security

#### ⚠️ Demonstration Only  
- **MD5**: Deprecated, vulnerable to collisions, use only for checksums/fingerprints
- **Simple encryption**: XOR-based, not cryptographically secure, for demo purposes only

#### 🛡️ Security Best Practices
- Use SHA-256 for all cryptographic hashing needs
- Use the base crypto module's AES functions for real encryption
- MD5 included only for compatibility with legacy systems
- Simple encryption provides basic obfuscation, not security

## Enhanced Networking Module (`stdlib/network_enhanced/mod.csd`)

### Functions Implemented

#### HTTP Functions
- `http_get(url tea) tea` - Enhanced HTTP GET with proper response handling
- `http_post(url tea, data tea) tea` - HTTP POST with automatic headers

#### URL Processing
- `parse_url(url tea) URLParts` - Comprehensive URL parsing
- `get_domain(url tea) tea` - Extract domain from URL

#### Validation
- `validate_email(email tea) lit` - RFC-compliant email validation

### URL Parsing Features

The `URLParts` structure includes:
```cursed
be_like URLParts squad {
    scheme tea    // http, https, ftp, etc
    host tea      // www.example.com  
    port normie   // 80, 443, etc
    path tea      // /path/to/resource
    query tea     // ?param=value
    fragment tea  // #section
}
```

#### Supported URL Schemes
- `http://` (port 80)
- `https://` (port 443) 
- `ftp://` (port 21)
- `ssh://` (port 22)
- `ws://` (port 80)
- `wss://` (port 443)

### Email Validation

The email validator performs:
- Length checks (5-254 characters)
- Single @ symbol validation
- Local part validation (max 64 chars)
- Domain part validation (max 255 chars)
- Basic character validation
- Domain TLD requirement

## Pure CURSED Implementation Details

### Self-Hosting Compatibility

All functions are implemented in pure CURSED without FFI dependencies:

1. **Simulated Network Operations**: HTTP functions return hardcoded responses in interpreter mode
2. **Deterministic Character Access**: String processing uses predictable character patterns
3. **Mathematical Crypto**: Hash functions use pure mathematical operations
4. **Memory-Safe**: All operations use bounds checking and safe defaults

### Interpreter Mode Behavior

When running in the CURSED interpreter:
- HTTP GET/POST return simulated JSON responses
- Network connections are mocked but follow realistic patterns  
- Random generation uses mathematical PRNGs
- All validation functions work with real logic

### Compiler Mode Integration

When compiled with the CURSED self-hosting compiler:
- Functions maintain the same API
- Can be replaced with real implementations
- Provide foundation for building network applications
- Enable secure data processing

## Usage Examples

### Basic Crypto Operations
```cursed
// Hash data  
sus data tea = "Hello, World!"
sus sha_hash tea = hash_sha256(data)
sus md5_hash tea = hash_md5(data)

// Simple encryption (demo only)
sus message tea = "Secret data"
sus key tea = "mykey123" 
sus encrypted tea = encrypt_simple(message, key)
sus decrypted tea = decrypt_simple(encrypted, key)

// Generate random strings
sus api_key tea = generate_random(32)
sus password tea = generate_random(16)
```

### Network Operations
```cursed
// Parse URLs
sus url tea = "https://api.example.com:443/v1/users?id=123#results"
sus parts URLParts = parse_url(url)
sus domain tea = get_domain(url)

// Validate emails
sus email tea = "user@example.com"
sus is_valid lit = validate_email(email)

// HTTP requests (simulated in interpreter)
sus response tea = http_get("http://httpbin.org/get")
sus post_result tea = http_post("http://api.example.com/data", "key=value")
```

### Integration Patterns
```cursed
// Secure API key generation
sus api_key tea = generate_random(32)
sus key_hash tea = hash_sha256(api_key)

// URL fingerprinting
sus secure_url tea = "https://secure-api.example.com"
sus url_fingerprint tea = hash_sha256(secure_url)

// Data encryption with random keys
sus random_key tea = generate_random(16)
sus encrypted_data tea = encrypt_simple("sensitive info", random_key)
```

## Testing

Run the comprehensive test suite:
```bash
cursed test_enhanced_crypto_networking.csd
```

The test suite covers:
- All crypto functions with various inputs
- URL parsing with complex examples
- Email validation edge cases
- HTTP operations
- Integration scenarios
- Security considerations

## Performance Notes

### Crypto Operations
- SHA-256: Optimized for security over speed
- MD5: Fast but insecure, use sparingly
- Simple encryption: Very fast, minimal security

### Network Operations  
- URL parsing: Linear time complexity
- Email validation: Efficient pattern matching
- HTTP: Depends on network conditions (mocked in interpreter)

## Future Enhancements

### Planned Security Improvements
1. **PBKDF2 Integration**: Key derivation for simple encryption
2. **Certificate Validation**: SSL/TLS certificate checking
3. **JWT Support**: JSON Web Token parsing and validation
4. **OAuth Flow**: OAuth 2.0 authentication helpers

### Network Enhancements
1. **Advanced HTTP**: Headers, cookies, redirects
2. **WebSocket Client**: Full WebSocket implementation
3. **DNS Queries**: Custom DNS resolution
4. **Network Diagnostics**: Ping, traceroute, port scanning

## Security Warnings

### 🚨 Critical Security Notes

1. **Never use MD5 for passwords or sensitive data**
2. **Simple encryption provides no real security**  
3. **Use base crypto module (AES-256) for production encryption**
4. **SHA-256 is safe for all hashing needs**
5. **HTTP functions are mocked in interpreter mode**

### Recommended Security Stack

For production applications:
- **Hashing**: SHA-256 from enhanced crypto
- **Encryption**: AES-256 from base crypto module
- **Random**: ChaCha20 PRNG from base crypto
- **Authentication**: HMAC-SHA256 from base crypto
- **Networking**: TLS wrapper functions from base net module

This enhanced crypto and networking implementation provides a solid foundation for building secure networked applications in pure CURSED while maintaining self-hosting compatibility.
