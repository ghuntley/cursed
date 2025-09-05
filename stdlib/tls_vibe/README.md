# TLS/SSL Vibe Module - Production-Ready TLS Implementation

## Overview

The `tls_vibe` module provides a comprehensive, production-ready implementation of the Transport Layer Security (TLS) protocol for the CURSED programming language. This module supports TLS 1.2 and TLS 1.3, offering enterprise-grade security features, performance optimizations, and extensive configurability.

## Features

### Core TLS Support
- **TLS 1.2 and 1.3**: Full support for modern TLS versions with secure defaults
- **Strong Cipher Suites**: Only secure cipher suites with perfect forward secrecy
- **Certificate Validation**: Comprehensive certificate chain validation and hostname verification
- **Session Management**: Efficient session resumption and caching
- **ALPN Support**: Application-Layer Protocol Negotiation for HTTP/2, HTTP/1.1, and custom protocols

### Security Features
- **Perfect Forward Secrecy**: All supported cipher suites provide PFS
- **Certificate Transparency**: Built-in CT log verification
- **OCSP Stapling**: Online Certificate Status Protocol support
- **Secure Renegotiation**: Protection against renegotiation attacks
- **Extended Master Secret**: Enhanced key derivation security
- **Constant-Time Operations**: Protection against timing attacks

### Enterprise Features
- **Mutual TLS (mTLS)**: Full client certificate authentication
- **SNI Support**: Server Name Indication for multi-domain servers
- **Certificate Rotation**: Hot certificate updates without service interruption
- **Connection Metrics**: Comprehensive performance monitoring
- **Error Handling**: Detailed error reporting and recovery
- **Security Policies**: Configurable security requirements

## Quick Start

### Basic TLS Client

```csd
yeet "tls_vibe"

slay tls_client_example() {
    fr fr Create TLS configuration
    sus config TLSConfig = tls_config_new()
    config = tls_config_set_server_name(config, "example.com")
    
    fr fr Establish connection
    sus conn TLSConnection = tls_dial("example.com", 443, config)
    
    fr fr Check connection security
    vibes tls_is_connection_secure(conn) {
        vibez.spill("✅ Secure connection established")
        vibez.spill("🔒 Security level: " + tls_get_security_level(conn))
        vibez.spill("🔐 Cipher suite: " + tls_get_cipher_suite_name(conn.cipher_suite))
        vibez.spill("📊 TLS version: " + tls_get_version_name(conn.version))
    }
    
    fr fr Write data
    sus bytes_written normie = tls_write(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")
    vibez.spill("📤 Sent " + tea(bytes_written) + " bytes")
    
    fr fr Read response
    sus response tea = tls_read(conn, 1024)
    vibez.spill("📥 Received: " + response)
    
    fr fr Close connection
    tls_close(conn)
    vibez.spill("🔌 Connection closed")
}
```

### Basic TLS Server

```csd
yeet "tls_vibe"

slay tls_server_example() {
    fr fr Create server configuration
    sus config TLSConfig = tls_config_new()
    sus cert_files [tea] = ["server.crt"]
    sus key_files [tea] = ["server.key"]
    config = tls_config_set_certificates(config, cert_files, key_files)
    
    fr fr Start listening
    sus server_conn TLSConnection = tls_listen(8443, config)
    vibez.spill("🚀 TLS server listening on port 8443")
    
    fr fr Accept connection
    sus client_conn TLSConnection = tls_accept(server_conn, config)
    
    fr fr Check connection state
    vibes client_conn.state == TLS_STATE_CONNECTED {
        vibez.spill("✅ Client connected successfully")
        vibez.spill("🔒 Security level: " + tls_get_security_level(client_conn))
        
        fr fr Read client request
        sus request tea = tls_read(client_conn, 1024)
        vibez.spill("📥 Client request: " + request)
        
        fr fr Send response
        sus response tea = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!"
        tls_write(client_conn, response)
        
        fr fr Close connection
        tls_close(client_conn)
    }
}
```

### Mutual TLS (mTLS) Example

```csd
yeet "tls_vibe"

slay mtls_server_example() {
    fr fr Create server configuration with client cert requirement
    sus config TLSConfig = tls_config_new()
    sus cert_files [tea] = ["server.crt"]
    sus key_files [tea] = ["server.key"]
    sus client_ca_files [tea] = ["client-ca.crt"]
    
    config = tls_config_set_certificates(config, cert_files, key_files)
    config = tls_config_set_client_ca_certificates(config, client_ca_files)
    
    fr fr Start listening with mutual TLS
    sus server_conn TLSConnection = tls_listen(8443, config)
    vibez.spill("🔐 mTLS server listening on port 8443")
    
    fr fr Accept connection with client certificate verification
    sus client_conn TLSConnection = tls_accept(server_conn, config)
    
    vibes client_conn.state == TLS_STATE_CONNECTED {
        vibez.spill("✅ Client authenticated with certificate")
        sus peer_certs [tea] = tls_get_peer_certificates(client_conn)
        vibez.spill("📜 Client certificates: " + tea(len(peer_certs)))
    }
}
```

## API Reference

### Configuration Functions

#### `tls_config_new() TLSConfig`
Creates a new TLS configuration with secure defaults.

#### `tls_config_set_certificates(config TLSConfig, cert_files [tea], key_files [tea]) TLSConfig`
Sets the certificate chain and private keys for the TLS endpoint.

#### `tls_config_set_ca_certificates(config TLSConfig, ca_files [tea]) TLSConfig`
Sets the root CA certificates for verifying peer certificates.

#### `tls_config_set_client_ca_certificates(config TLSConfig, ca_files [tea]) TLSConfig`
Sets the client CA certificates for mutual TLS authentication.

#### `tls_config_set_version_range(config TLSConfig, min_version normie, max_version normie) TLSConfig`
Sets the acceptable TLS version range.

#### `tls_config_set_cipher_suites(config TLSConfig, cipher_suites [normie]) TLSConfig`
Sets the allowed cipher suites.

#### `tls_config_set_server_name(config TLSConfig, server_name tea) TLSConfig`
Sets the server name for SNI (Server Name Indication).

#### `tls_config_set_alpn_protocols(config TLSConfig, protocols [tea]) TLSConfig`
Sets the ALPN protocols for application-layer protocol negotiation.

#### `tls_config_set_insecure_skip_verify(config TLSConfig, skip lit) TLSConfig`
Disables certificate verification (for testing only).

### Connection Functions

#### `tls_dial(hostname tea, port normie, config TLSConfig) TLSConnection`
Establishes a TLS connection to a server.

#### `tls_listen(port normie, config TLSConfig) TLSConnection`
Creates a TLS server listening on the specified port.

#### `tls_accept(server_conn TLSConnection, config TLSConfig) TLSConnection`
Accepts an incoming TLS connection.

#### `tls_read(conn TLSConnection, buffer_size normie) tea`
Reads data from a TLS connection.

#### `tls_write(conn TLSConnection, data tea) normie`
Writes data to a TLS connection.

#### `tls_close(conn TLSConnection) lit`
Closes a TLS connection gracefully.

### Handshake Functions

#### `tls_perform_handshake(conn TLSConnection, config TLSConfig, is_server lit) lit`
Performs the TLS handshake process.

#### `tls_verify_certificate_chain(conn TLSConnection, config TLSConfig) lit`
Verifies the peer's certificate chain.

#### `tls_verify_hostname(conn TLSConnection, hostname tea) lit`
Verifies the hostname against the peer's certificate.

### Session Management

#### `tls_session_new(conn TLSConnection) tea`
Creates a new TLS session for resumption.

#### `tls_session_resume(session_data tea) TLSConnection`
Resumes a TLS session from saved data.

### Security Functions

#### `tls_is_connection_secure(conn TLSConnection) lit`
Checks if a connection meets security requirements.

#### `tls_get_security_level(conn TLSConnection) tea`
Returns the security level assessment of a connection.

#### `tls_get_cipher_suite_name(cipher_suite normie) tea`
Returns the name of a cipher suite.

#### `tls_get_version_name(version normie) tea`
Returns the name of a TLS version.

### Alert Functions

#### `tls_send_alert(conn TLSConnection, level normie, description normie) lit`
Sends a TLS alert message.

#### `tls_handle_alert(conn TLSConnection, alert tea) lit`
Handles received TLS alert messages.

### Utility Functions

#### `tls_get_connection_state(conn TLSConnection) tea`
Returns comprehensive connection state information.

#### `tls_get_peer_certificates(conn TLSConnection) [tea]`
Returns the peer's certificate chain.

#### `tls_get_connection_metrics(conn TLSConnection) tea`
Returns connection performance metrics.

## Configuration Examples

### High-Security Configuration

```csd
slay create_high_security_config() TLSConfig {
    sus config TLSConfig = tls_config_new()
    
    fr fr Require TLS 1.3 only
    config = tls_config_set_version_range(config, TLS_VERSION_1_3, TLS_VERSION_1_3)
    
    fr fr Use only the strongest cipher suites
    sus cipher_suites [normie] = [
        TLS_AES_256_GCM_SHA384,
        TLS_CHACHA20_POLY1305_SHA256
    ]
    config = tls_config_set_cipher_suites(config, cipher_suites)
    
    fr fr Enable all security features
    config.verify_peer_certificate = based
    config.verify_hostname = based
    config.ocsp_stapling = based
    config.certificate_transparency = based
    config.session_tickets_disabled = based  # Disable for maximum security
    
    fr fr Set strict timeouts
    config.max_handshake_time = 5000  # 5 seconds
    config.read_timeout = 10000       # 10 seconds
    config.write_timeout = 10000      # 10 seconds
    
    damn config
}
```

### Performance-Optimized Configuration

```csd
slay create_performance_config() TLSConfig {
    sus config TLSConfig = tls_config_new()
    
    fr fr Allow TLS 1.2 and 1.3 for compatibility
    config = tls_config_set_version_range(config, TLS_VERSION_1_2, TLS_VERSION_1_3)
    
    fr fr Use balanced cipher suites
    sus cipher_suites [normie] = [
        TLS_AES_128_GCM_SHA256,
        TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
        TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
    ]
    config = tls_config_set_cipher_suites(config, cipher_suites)
    
    fr fr Enable session resumption
    config.session_tickets_disabled = cap
    config.session_cache_size = 10000
    config.session_timeout = 3600  # 1 hour
    
    fr fr Optimize for throughput
    config.max_fragment_len = 16384
    config.max_early_data = 8192
    
    damn config
}
```

## TLS Protocol Constants

### TLS Versions
- `TLS_VERSION_1_0` = 0x0301
- `TLS_VERSION_1_1` = 0x0302
- `TLS_VERSION_1_2` = 0x0303
- `TLS_VERSION_1_3` = 0x0304

### Connection States
- `TLS_STATE_INIT` = 0
- `TLS_STATE_HANDSHAKE` = 1
- `TLS_STATE_CONNECTED` = 2
- `TLS_STATE_CLOSED` = 3
- `TLS_STATE_ERROR` = 4

### Cipher Suites
- `TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384` = 0xc030
- `TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384` = 0xc02c
- `TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256` = 0xc02f
- `TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256` = 0xc02b
- `TLS_AES_256_GCM_SHA384` = 0x1302
- `TLS_AES_128_GCM_SHA256` = 0x1301
- `TLS_CHACHA20_POLY1305_SHA256` = 0x1303

### Alert Levels
- `TLS_ALERT_WARNING` = 1
- `TLS_ALERT_FATAL` = 2

### Alert Descriptions
- `TLS_ALERT_CLOSE_NOTIFY` = 0
- `TLS_ALERT_UNEXPECTED_MESSAGE` = 10
- `TLS_ALERT_BAD_RECORD_MAC` = 20
- `TLS_ALERT_HANDSHAKE_FAILURE` = 40
- `TLS_ALERT_BAD_CERTIFICATE` = 42
- `TLS_ALERT_CERTIFICATE_EXPIRED` = 45
- `TLS_ALERT_CERTIFICATE_UNKNOWN` = 46
- `TLS_ALERT_ILLEGAL_PARAMETER` = 47

## Security Considerations

### Certificate Verification
- Always verify certificates in production environments
- Use `insecure_skip_verify` only for testing
- Implement proper certificate pinning for high-security applications
- Regularly update CA certificate stores

### Cipher Suite Selection
- Use only cipher suites with perfect forward secrecy
- Prefer AEAD cipher suites (GCM, ChaCha20-Poly1305)
- Avoid deprecated cipher suites (RC4, DES, MD5)
- Regular security audits of cipher suite choices

### Protocol Versions
- Use TLS 1.3 when possible for maximum security
- TLS 1.2 is acceptable with proper configuration
- Disable TLS 1.0 and 1.1 in production
- Monitor for protocol downgrade attacks

### Session Management
- Use session resumption for performance
- Implement proper session timeout policies
- Secure session storage and transmission
- Regular session key rotation

## Performance Optimization

### Connection Pooling
- Reuse TLS connections when possible
- Implement connection pooling for client applications
- Monitor connection metrics for optimization

### Handshake Optimization
- Use session resumption to reduce handshake overhead
- Implement TLS 1.3 0-RTT when appropriate
- Optimize certificate chain length

### Cipher Suite Performance
- Prefer hardware-accelerated cipher suites
- Balance security and performance requirements
- Consider ChaCha20-Poly1305 for mobile devices

## Error Handling

### Common Errors
- **Certificate Verification Failures**: Check certificate validity, chain, and hostname
- **Protocol Version Mismatches**: Ensure compatible TLS versions
- **Cipher Suite Negotiations**: Verify supported cipher suites on both ends
- **Handshake Timeouts**: Check network connectivity and server responsiveness

### Error Recovery
- Implement proper error logging and monitoring
- Use graceful degradation when possible
- Provide meaningful error messages to users
- Implement retry mechanisms for transient failures

## Testing

### Unit Testing
```bash
# Run TLS module tests
cargo run --bin cursed stdlib/tls_vibe/test_tls_vibe.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/tls_vibe/test_tls_vibe.💀
./test_tls_vibe
```

### Integration Testing
```bash
# Test with real TLS connections
cargo run --bin cursed examples/tls_client_example.💀
cargo run --bin cursed examples/tls_server_example.💀
```

## Dependencies

The `tls_vibe` module depends on:
- `crypto` - Core cryptographic operations
- `x509_certs_tea` - X.509 certificate handling
- `atomic_drip` - Atomic operations for thread safety
- `timez` - Time and duration handling
- `testz` - Testing framework

## License

This module is part of the CURSED programming language and is distributed under the same license terms.

## Contributing

Contributions to the TLS implementation are welcome. Please ensure:
- All security-related changes are thoroughly reviewed
- Comprehensive tests are included
- Documentation is updated appropriately
- Performance implications are considered

## Support

For issues, questions, or contributions related to the TLS module, please refer to the main CURSED project repository.

---

*This implementation provides production-ready TLS support for CURSED applications with enterprise-grade security and performance characteristics.*
