# TLSz Module

## Why This Module Exists

The `tlsz` module provides enterprise-grade Transport Layer Security (TLS) implementation for secure communication in CURSED applications. Modern applications require robust encryption, certificate validation, and secure protocol handling to protect data in transit and comply with security regulations.

The module exists because:
- **Security Requirements**: Applications must protect sensitive data (credentials, personal information, financial data) from network interception
- **Compliance Mandates**: Industry regulations (PCI DSS, HIPAA, GDPR) require secure communication channels with specific cryptographic standards
- **Trust Establishment**: Applications need to verify the identity of communication partners through certificate validation
- **Attack Prevention**: Protection against man-in-the-middle attacks, downgrade attacks, and cryptographic vulnerabilities
- **Enterprise Integration**: Support for corporate PKI, mutual authentication, and security monitoring requirements

## Why Testing Is Critical

TLS security testing is absolutely essential because:
- **Cryptographic Correctness**: Even minor implementation errors can completely compromise security (padding oracle attacks, timing attacks)
- **Attack Surface Validation**: TLS implementations are prime targets for sophisticated attacks and must be thoroughly tested against known attack vectors
- **Certificate Validation Logic**: Improper certificate validation is a leading cause of security vulnerabilities in networked applications
- **Protocol Compliance**: Non-compliant implementations may interoperate poorly or be vulnerable to protocol-level attacks
- **Performance Under Attack**: Security implementations must maintain functionality during active attacks (DoS, computational exhaustion)

## Implementation Rationale

### Key Design Decisions:

**1. Security-First Architecture**
- Secure-by-default configuration with only approved cipher suites enabled
- Automatic protection against known vulnerabilities (BEAST, CRIME, POODLE, Heartbleed)
- Constant-time cryptographic operations to prevent timing attacks
- Memory-safe implementation preventing buffer overflows and information disclosure

**2. Standards Compliance**
- Full TLS 1.3 (RFC 8446) and TLS 1.2 (RFC 5246) support
- X.509 certificate validation according to RFC 5280
- Perfect Forward Secrecy (PFS) enforced for all connections
- HSTS, OCSP stapling, and Certificate Transparency support

**3. Performance Optimization**
- Hardware acceleration for AES, ChaCha20, and elliptic curve operations
- Session resumption and 0-RTT support in TLS 1.3
- Optimized handshake processing with minimal memory allocation
- Connection pooling and keepalive optimization

**4. Enterprise Features**
- Client certificate support for mutual authentication
- SNI (Server Name Indication) for virtual hosting
- ALPN (Application-Layer Protocol Negotiation) for HTTP/2, gRPC
- Certificate pinning and trust store management

## API Reference

### TLS Configuration

#### `TlsConfig`
**Purpose**: Configuration structure for TLS connections with security defaults
**Security**: Secure defaults prevent accidental misconfigurations
**Flexibility**: Comprehensive options for enterprise requirements

```cursed
sus config = tlsz.TlsConfig{
    # Certificate configuration
    cert_file: "/path/to/server.crt",
    key_file: "/path/to/server.key",
    ca_file: "/path/to/ca.crt",
    
    # Security settings
    min_version: tlsz.TLS_1_2,
    max_version: tlsz.TLS_1_3,
    cipher_suites: tlsz.SECURE_CIPHER_SUITES,  # Pre-configured secure list
    
    # Client authentication
    client_auth: tlsz.REQUIRE_CLIENT_CERT,
    client_ca_file: "/path/to/client-ca.crt",
    
    # Advanced features
    session_tickets: based,
    sni_callback: slay(server_name tea) TlsConfig { 
        damn get_cert_for_domain(server_name) 
    }
}
```

#### `create_tls_config() TlsConfig`
**Purpose**: Creates TLS configuration with security best practices
**Defaults**: Secure cipher suites, perfect forward secrecy, attack mitigations enabled
**Standards**: Compliant with industry security guidelines

### TLS Server Operations

#### `tls_listen(address: tea, port: drip, config: TlsConfig) yikes<TlsListener>`
**Purpose**: Creates TLS server listener with secure configuration
**Security**: Automatic certificate loading and validation
**Performance**: Optimized for high-concurrency scenarios

```cursed
sus tls_listener = tlsz.tls_listen("0.0.0.0", 443, config) fam {
    when "cert_load_error" -> yikes "Failed to load TLS certificate"
    when "bind_error" -> yikes "Cannot bind to port 443"
}

bestie (connection := tls_listener.accept()) {
    go handle_secure_connection(connection)
}
```

#### `TlsConnection`
**Purpose**: Represents established TLS connection with security information
**Features**: Certificate inspection, cipher suite information, security status
**Integration**: Seamless integration with CURSED's I/O system

```cursed
slay handle_secure_connection(conn TlsConnection) {
    defer conn.close()
    
    # Inspect connection security
    sus cert_info = conn.peer_certificate()
    sus cipher_info = conn.cipher_suite()
    
    vibez.spill("Client certificate:", cert_info.subject)
    vibez.spill("Cipher suite:", cipher_info.name)
    vibez.spill("TLS version:", conn.tls_version())
    
    # Handle secure communication...
}
```

### TLS Client Operations

#### `tls_dial(address: tea, port: drip, config: TlsConfig) yikes<TlsConnection>`
**Purpose**: Establishes TLS client connection with certificate validation
**Security**: Automatic hostname verification, certificate chain validation
**Features**: SNI support, ALPN negotiation, session resumption

```cursed
sus client_config = tlsz.TlsConfig{
    # Client certificate for mutual auth
    cert_file: "/path/to/client.crt",
    key_file: "/path/to/client.key",
    
    # Server verification
    ca_file: "/path/to/trusted-ca.crt",
    server_name: "secure-api.example.com",
    verify_hostname: based,
    
    # Protocol preferences
    alpn_protocols: ["h2", "http/1.1"]
}

sus conn = tlsz.tls_dial("secure-api.example.com", 443, client_config) fam {
    when "cert_verify_error" -> yikes "Server certificate validation failed"
    when "handshake_timeout" -> yikes "TLS handshake timed out"
}
```

### Certificate Management

#### `Certificate`
**Purpose**: Represents X.509 certificates with validation and inspection capabilities
**Security**: Cryptographic signature verification, expiration checking
**Standards**: Full RFC 5280 compliance with extension support

```cursured
sus cert = tlsz.load_certificate("/path/to/cert.pem") fam {
    when "parse_error" -> yikes "Invalid certificate format"
}

# Certificate inspection
vibez.spill("Subject:", cert.subject())
vibez.spill("Issuer:", cert.issuer())
vibez.spill("Valid from:", cert.not_before())
vibez.spill("Valid until:", cert.not_after())
vibez.spill("Key algorithm:", cert.public_key_algorithm())

# Validation
sus is_valid = cert.verify_against_ca(ca_cert)
sus is_expired = cert.is_expired()
```

#### `create_self_signed_cert(config: CertConfig) yikes<Certificate>`
**Purpose**: Generates self-signed certificates for testing and development
**Security**: Strong key generation, appropriate certificate extensions
**Use Case**: Development environments, internal services

### Advanced Security Features

#### `certificate_pinning(pins: []CertPin) PinningConfig`
**Purpose**: Implements certificate pinning to prevent CA compromise attacks
**Security**: Binds connections to specific certificates or public keys
**Flexibility**: Supports backup pins and pin rotation

```cursed
sus pinning_config = tlsz.certificate_pinning([
    tlsz.CertPin{
        hostname: "api.example.com",
        sha256_hash: "1a2b3c4d...",  # Certificate SHA-256 fingerprint
        backup_pins: ["5e6f7g8h..."]
    }
])

sus config = tlsz.TlsConfig{
    # ... other config ...
    certificate_pinning: pinning_config
}
```

#### `ocsp_stapling(enabled: bool) OcspConfig`
**Purpose**: Enables OCSP stapling for real-time certificate revocation checking
**Security**: Prevents use of revoked certificates
**Performance**: Reduces handshake latency compared to OCSP queries

## Usage Examples

### Basic HTTPS Server
```cursed
yeet "tlsz"
yeet "networkz"

# Configure TLS with security best practices
sus tls_config = tlsz.TlsConfig{
    cert_file: "/etc/ssl/certs/server.crt",
    key_file: "/etc/ssl/private/server.key",
    
    # Security settings
    min_version: tlsz.TLS_1_2,
    cipher_suites: tlsz.SECURE_CIPHER_SUITES,
    prefer_server_cipher_suites: based,
    
    # HSTS and security headers
    enable_hsts: based,
    hsts_max_age: 31536000,  # 1 year
}

# Create secure HTTPS server
sus https_listener = tlsz.tls_listen("0.0.0.0", 443, tls_config) fam {
    when "permission_denied" -> yikes "Cannot bind to port 443 (try sudo)"
    when "cert_error" -> yikes "TLS certificate error"
}

vibez.spill("HTTPS server listening on port 443")

bestie (connection := https_listener.accept()) {
    go handle_https_request(connection)
}
```

### Mutual TLS Authentication
```cursed
# Server configuration requiring client certificates
sus mutual_tls_config = tlsz.TlsConfig{
    cert_file: "/etc/ssl/certs/server.crt",
    key_file: "/etc/ssl/private/server.key",
    
    # Require client certificates
    client_auth: tlsz.REQUIRE_AND_VERIFY_CLIENT_CERT,
    client_ca_file: "/etc/ssl/certs/client-ca.crt",
    
    # Additional security
    min_version: tlsz.TLS_1_3,
    session_tickets: cap,  # Disable for maximum security
}

slay handle_mutual_tls_connection(conn TlsConnection) {
    # Verify client certificate
    sus client_cert = conn.peer_certificate() fam {
        vibez.spill("Client certificate required but not provided")
        conn.close()
        damn
    }
    
    # Extract client identity
    sus client_dn = client_cert.subject()
    vibez.spill("Authenticated client:", client_dn)
    
    # Implement business logic based on client identity
    ready (is_authorized(client_dn)) {
        handle_authorized_request(conn)
    } otherwise {
        send_http_error(conn, 403, "Access denied")
    }
}
```

### High-Performance TLS Client
```cursed
# Connection pool for high-performance client applications
squad TlsConnectionPool {
    config: tlsz.TlsConfig
    connections: []TlsConnection
    mutex: sync.Mutex
}

slay (pool TlsConnectionPool) get_connection(hostname tea, port drip) yikes<TlsConnection> {
    pool.mutex.lock()
    defer pool.mutex.unlock()
    
    # Try to reuse existing connection
    bestie (i drip = 0; i < pool.connections.len(); i++) {
        sus conn = pool.connections[i]
        ready (conn.is_usable() && conn.hostname() == hostname) {
            # Remove from pool and return
            pool.connections.remove(i)
            damn conn
        }
    }
    
    # Create new connection with session resumption
    sus new_conn = tlsz.tls_dial(hostname, port, pool.config) fam {
        when "handshake_failed" -> yikes "TLS handshake failed for " + hostname
    }
    
    damn new_conn
}

# Usage with automatic connection pooling
sus pool = TlsConnectionPool{
    config: tlsz.TlsConfig{
        # Enable session resumption for performance
        session_resumption: based,
        session_cache_size: 100,
        
        # Client certificate for enterprise environments
        cert_file: "/etc/ssl/certs/client.crt",
        key_file: "/etc/ssl/private/client.key",
        
        # Certificate validation
        ca_file: "/etc/ssl/certs/ca-bundle.crt",
        verify_hostname: based,
    }
}

sus conn = pool.get_connection("api.example.com", 443)
# Use connection for HTTPS requests...
pool.return_connection(conn)
```

### Certificate Validation and Monitoring
```cursed
# Monitor certificate expiration and validity
slay monitor_certificates(cert_files []tea) {
    bestie (cert_file in cert_files) {
        sus cert = tlsz.load_certificate(cert_file) fam {
            vibez.spill("ERROR: Cannot load certificate:", cert_file)
            continue
        }
        
        sus days_until_expiry = cert.days_until_expiry()
        ready (days_until_expiry < 30) {
            vibez.spill("WARNING: Certificate expires soon:", cert_file, 
                       "expires in", days_until_expiry, "days")
            
            # Send alert notification
            send_alert("certificate_expiring", {
                "file": cert_file,
                "days_remaining": days_until_expiry,
                "subject": cert.subject()
            })
        }
        
        # Validate certificate chain
        sus chain_valid = validate_certificate_chain(cert)
        ready (!chain_valid) {
            vibez.spill("ERROR: Invalid certificate chain:", cert_file)
        }
    }
}

# Automated certificate rotation
slay rotate_certificate_if_needed(cert_path tea, key_path tea) yikes<bool> {
    sus cert = tlsz.load_certificate(cert_path) fam {
        when _ -> yikes "Cannot load current certificate"
    }
    
    ready (cert.days_until_expiry() < 7) {  # Rotate within 7 days
        vibez.spill("Rotating certificate:", cert_path)
        
        # Request new certificate (e.g., from ACME/Let's Encrypt)
        sus new_cert = request_new_certificate(cert.subject()) fam {
            when _ -> yikes "Failed to obtain new certificate"
        }
        
        # Atomic replacement
        sus backup_path = cert_path + ".backup"
        platformz.copy_file(cert_path, backup_path)
        
        tlsz.save_certificate(new_cert, cert_path) fam {
            when _ -> {
                # Restore backup on failure
                platformz.copy_file(backup_path, cert_path)
                yikes "Failed to save new certificate"
            }
        }
        
        vibez.spill("Certificate rotated successfully")
        damn based
    }
    
    damn cap  # No rotation needed
}
```

## Performance Considerations

### Cipher Suite Selection

**High Performance**: AES-GCM and ChaCha20-Poly1305 with hardware acceleration
```cursed
sus performance_config = tlsz.TlsConfig{
    cipher_suites: [
        tlsz.TLS_AES_128_GCM_SHA256,        # Fast AES-GCM
        tlsz.TLS_CHACHA20_POLY1305_SHA256,  # Fast on mobile/ARM
        tlsz.TLS_AES_256_GCM_SHA384         # High security
    ]
}
```

**Maximum Security**: Prefer 256-bit encryption with perfect forward secrecy
```cursed
sus security_config = tlsz.TlsConfig{
    cipher_suites: [
        tlsz.TLS_AES_256_GCM_SHA384,
        tlsz.TLS_CHACHA20_POLY1305_SHA256,
        tlsz.ECDHE_RSA_WITH_AES_256_GCM_SHA384
    ]
}
```

### Connection Optimization

1. **Session Resumption**: Reduce handshake overhead for returning clients
2. **0-RTT Mode**: Enable TLS 1.3 early data for ultra-low latency
3. **Connection Pooling**: Reuse TLS connections across requests
4. **Hardware Acceleration**: Leverage AES-NI, cryptographic coprocessors
5. **Certificate Caching**: Cache validated certificates to avoid repeated validation

### Memory Management

- **Buffer Sizing**: Optimize TLS record buffer sizes for your use case
- **Certificate Storage**: Efficiently store and index large certificate chains
- **Session Cache**: Tune session cache size based on client connection patterns
- **Memory Pools**: Use arena allocators for TLS handshake data

## Security Considerations

### Attack Mitigation

**Protocol Downgrade Attacks**
```cursed
# Prevent downgrade attacks
sus secure_config = tlsz.TlsConfig{
    min_version: tlsz.TLS_1_2,  # Never allow < TLS 1.2
    disable_sslv3: based,       # Explicitly disable SSLv3
    disable_compression: based,  # Prevent CRIME attacks
}
```

**Timing Attacks**
- All cryptographic operations use constant-time implementations
- RSA operations use blinding to prevent timing analysis
- ECDSA signing uses deterministic k generation (RFC 6979)

**Certificate Validation Attacks**
```cursed
# Comprehensive certificate validation
slay validate_peer_certificate(cert Certificate, hostname tea) yikes<bool> {
    # 1. Check certificate signature
    sus sig_valid = cert.verify_signature() fam {
        when _ -> yikes "Invalid certificate signature"
    }
    
    # 2. Check certificate is not expired
    ready (cert.is_expired()) {
        yikes "Certificate has expired"
    }
    
    # 3. Verify hostname matches certificate
    sus hostname_match = cert.verify_hostname(hostname) fam {
        when _ -> yikes "Hostname verification failed"
    }
    
    # 4. Check certificate chain
    sus chain_valid = cert.verify_chain() fam {
        when _ -> yikes "Certificate chain validation failed"
    }
    
    # 5. Check certificate revocation (OCSP/CRL)
    sus revocation_status = check_revocation(cert) fam {
        when _ -> yikes "Certificate revocation check failed"
    }
    
    ready (revocation_status == "revoked") {
        yikes "Certificate has been revoked"
    }
    
    damn based
}
```

### Secure Configuration Guidelines

**Production TLS Configuration**:
```cursed
sus production_config = tlsz.TlsConfig{
    # Certificates
    cert_file: "/etc/ssl/certs/server.crt",
    key_file: "/etc/ssl/private/server.key",
    ca_file: "/etc/ssl/certs/ca-bundle.crt",
    
    # Protocol versions
    min_version: tlsz.TLS_1_2,
    max_version: tlsz.TLS_1_3,
    
    # Cipher suites (only secure ones)
    cipher_suites: tlsz.SECURE_CIPHER_SUITES,
    prefer_server_cipher_suites: based,
    
    # Security features
    enable_hsts: based,
    hsts_max_age: 31536000,  # 1 year
    hsts_include_subdomains: based,
    
    # Certificate transparency
    enable_ct: based,
    
    # OCSP stapling
    enable_ocsp_stapling: based,
    
    # Session security
    session_tickets: cap,  # Disable for maximum security
    session_timeout: 3600,  # 1 hour
    
    # Client authentication (if needed)
    client_auth: tlsz.VERIFY_CLIENT_CERT_IF_GIVEN,
}
```

### Cryptographic Key Management

**Key Generation**:
```cursed
# Generate cryptographically secure keys
sus private_key = tlsz.generate_rsa_key(4096) fam {  # 4096-bit RSA
    when _ -> yikes "Key generation failed"
}

# Or use elliptic curve (faster, same security)
sus ec_key = tlsz.generate_ec_key(tlsz.P384) fam {   # NIST P-384
    when _ -> yikes "EC key generation failed"
}
```

**Key Storage**:
```cursed
# Secure key storage with proper permissions
slay save_private_key_securely(key PrivateKey, path tea) yikes<fam> {
    # Set restrictive file permissions (owner only)
    sus permissions = platformz.file_permissions(0o600)
    
    # Write key with atomic operation
    tlsz.save_private_key(key, path, permissions) fam {
        when _ -> yikes "Failed to save private key securely"
    }
    
    # Verify permissions were set correctly
    sus file_info = platformz.file_info(path) fam {
        when _ -> yikes "Cannot verify key file permissions"
    }
    
    ready (file_info.permissions != 0o600) {
        yikes "Key file permissions not set correctly"
    }
    
    damn fam
}
```

## Integration with CURSED Ecosystem

### Error Handling Integration
```cursed
# TLS errors integrate with CURSED's error system
sus connection = tlsz.tls_dial("example.com", 443, config) fam {
    when "cert_expired" -> handle_expired_certificate()
    when "cert_untrusted" -> handle_untrusted_certificate()
    when "handshake_timeout" -> handle_slow_network()
    when "protocol_error" -> handle_protocol_mismatch()
}
```

### Logging and Monitoring Integration
```cursed
# Automatic security event logging
tlsz.set_security_logger(slay(event TlsSecurityEvent) {
    monitoring.increment_counter("tls_events", [
        "type", event.type,
        "severity", event.severity
    ])
    
    ready (event.severity == "critical") {
        alert_manager.send_alert("tls_security_event", event)
    }
})
```

### Concurrency Integration
```cursed
# TLS operations work seamlessly with goroutines
bestie (request in request_channel) {
    go slay() {
        sus connection = tlsz.tls_dial(request.hostname, 443, config)
        defer connection.close()
        
        # Handle request over secure connection
        handle_secure_request(connection, request)
    }()
}
```

The tlsz module provides enterprise-grade TLS implementation with security as the top priority, while maintaining the performance and usability characteristics expected from the CURSED ecosystem.
