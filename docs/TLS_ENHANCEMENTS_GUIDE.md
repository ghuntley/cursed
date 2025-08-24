# CURSED TLS Enhancements Guide

## Overview

The CURSED programming language provides enterprise-grade TLS/SSL advanced features through comprehensive enhancements to the existing `cryptz` and `tlsz` modules. This guide covers all advanced TLS features including Mutual TLS Authentication, Server Name Indication (SNI), Certificate Rotation, and Connection Pooling.

## Table of Contents

- [Mutual TLS Authentication](#mutual-tls-authentication)
- [Server Name Indication (SNI)](#server-name-indication-sni)
- [Certificate Rotation & Management](#certificate-rotation--management)
- [TLS Connection Pooling](#tls-connection-pooling)
- [Security Features](#security-features)
- [Performance Optimization](#performance-optimization)
- [Best Practices](#best-practices)
- [API Reference](#api-reference)

## Mutual TLS Authentication

Mutual TLS (mTLS) provides bidirectional authentication where both client and server present certificates for verification.

### Basic mTLS Setup

```cursed
yeet "tlsz/mutual_tls"

# Create client certificate and key
sus client_cert = load_certificate_pem("client.pem")
sus client_private_key = load_private_key_pem("client-key.pem")
sus cert_chain = [client_cert]
sus trusted_cas = load_ca_certificates("ca-bundle.pem")

# Configure mutual TLS
sus mtls_config = create_mutual_tls_config(
    client_cert, 
    client_private_key, 
    cert_chain, 
    trusted_cas
)

# Establish mutual TLS connection
sus tls_context = perform_mutual_tls_handshake(
    "api.example.com", 
    443, 
    mtls_config, 
    create_default_security_policy()
) fam {
    when _ -> vibez.spill("mTLS handshake failed: " + _)
}
```

### Lenient mTLS Configuration

```cursed
# Create lenient configuration (optional client certificates)
sus lenient_config = create_lenient_mutual_tls_config(
    client_cert, 
    client_private_key, 
    trusted_cas
)

# lenient_config.require_client_cert = false
# lenient_config.revocation_checking_enabled = false
```

### Client Certificate Validation

```cursed
# Server-side client certificate verification
sus client_auth_result = verify_client_certificate_server_side(
    client_cert,
    cert_chain,
    mtls_config
) fam {
    when _ -> vibez.spill("Client certificate verification failed")
}

ready (client_auth_result.is_valid) {
    vibez.spill("Client authenticated: " + client_auth_result.client_identity)
    vibez.spill("Trust level: " + stringz.from_int(client_auth_result.trust_level) + "%")
} otherwise {
    vibez.spill("Client authentication failed: " + 
                arrayz.join(client_auth_result.validation_errors, ", "))
}
```

### Client Authorization

```cursed
# Extract client identity from certificate
sus client_identity = extract_client_identity(client_cert)

# Define access control list
sus api_acl = [
    "client.example.com",
    "admin.example.com", 
    "*.trusted-partners.com"  # Wildcard support
]

# Authorize client access
sus authorized = authorize_client_access(
    client_identity, 
    "/api/secure", 
    api_acl
)

ready (authorized) {
    vibez.spill("Access granted for " + client_identity)
} otherwise {
    vibez.spill("Access denied for " + client_identity)
}
```

## Server Name Indication (SNI)

SNI enables hosting multiple SSL certificates on a single IP address, essential for virtual hosting.

### SNI Configuration

```cursed
yeet "tlsz/sni"

# Create SNI configuration
sus sni_config = create_sni_config()

# Add certificates for different hostnames
sus api_cert = load_certificate_pem("api.example.com.pem")
sus web_cert = load_certificate_pem("web.example.com.pem")
sus private_key = load_private_key_pem("private.key")

sni_config = add_sni_certificate(
    sni_config, 
    "api.example.com", 
    api_cert, 
    private_key, 
    [api_cert]
) fam {
    when _ -> vibez.spill("Failed to add API certificate")
}

sni_config = add_sni_certificate(
    sni_config, 
    "web.example.com", 
    web_cert, 
    private_key, 
    [web_cert]
) fam {
    when _ -> vibez.spill("Failed to add web certificate")
}
```

### Wildcard Certificates

```cursed
# Add wildcard certificate for subdomains
sus wildcard_cert = load_certificate_pem("wildcard.example.com.pem")

sni_config = add_sni_certificate(
    sni_config, 
    "*.example.com", 
    wildcard_cert, 
    private_key, 
    [wildcard_cert]
) fam {
    when _ -> vibez.spill("Failed to add wildcard certificate")
}

# Wildcard matching
testz.assert_true(matches_wildcard_pattern("sub.example.com", "*.example.com"))
testz.assert_false(matches_wildcard_pattern("example.com", "*.example.com"))  # Root doesn't match
```

### SNI Handshake Processing

```cursed
# Process SNI handshake
sus sni_result = process_sni_handshake("api.example.com", sni_config) fam {
    when _ -> vibez.spill("SNI processing failed: " + _)
}

ready (sni_result.sni_matched) {
    vibez.spill("Certificate selected for " + sni_result.hostname)
    vibez.spill("Match type: " + sni_result.match_type)  # "exact", "wildcard", "default"
    
    # Use selected certificate for TLS handshake
    sus selected_cert = sni_result.certificate_selected
    sus cert_chain = sni_result.certificate_chain
} otherwise {
    # Handle SNI mismatch
    vibez.spill("No certificate found for hostname")
    arrayz.foreach(sni_result.warnings, slay(warning tea) {
        vibez.spill("Warning: " + warning)
    })
}
```

### Strict SNI Configuration

```cursed
# Create strict SNI configuration (rejects unknown hostnames)
sus strict_config = create_strict_sni_config()

# strict_config.strict_sni_matching = true
# strict_config.fallback_behavior = "reject"
# strict_config.case_sensitive = true
```

### SNI Statistics

```cursed
# Create and track SNI statistics
sus sni_stats = create_sni_statistics()

# Update statistics after each handshake
sni_stats = update_sni_statistics(sni_stats, sni_result)

# Generate report
sus report = get_sni_statistics_report(sni_stats)
vibez.spill(report)
```

## Certificate Rotation & Management

Automated certificate rotation ensures continuous service availability without manual intervention.

### Rotation Manager Setup

```cursed
yeet "tlsz/cert_rotation"

# Create certificate rotation manager
sus rotation_manager = create_certificate_rotation_manager(
    30,  # Rotation threshold: 30 days before expiry
    "/var/lib/cursed/cert_backups"  # Backup directory
)

# Manual rotation manager (no auto-rotation)
sus manual_manager = create_manual_rotation_manager()
```

### Certificate Installation

```cursed
# Install certificate for rotation management
sus cert = load_certificate_pem("example.com.pem")
sus private_key = load_private_key_pem("example.com-key.pem")
sus cert_chain = [cert]

sus updated_manager = install_certificate(
    rotation_manager, 
    "example.com", 
    cert, 
    private_key, 
    cert_chain
) fam {
    when _ -> vibez.spill("Certificate installation failed: " + _)
}

vibez.spill("Certificate installed successfully")
```

### Certificate Staging

```cursed
# Stage new certificate for future rotation
sus new_cert = load_certificate_pem("example.com-new.pem")
sus new_private_key = load_private_key_pem("example.com-new-key.pem")
sus new_cert_chain = [new_cert]

sus updated_manager = stage_certificate_for_rotation(
    updated_manager, 
    "example.com", 
    new_cert, 
    new_private_key, 
    new_cert_chain
) fam {
    when _ -> vibez.spill("Certificate staging failed: " + _)
}

vibez.spill("New certificate staged for rotation")
```

### Hot Certificate Rotation

```cursed
# Execute hot certificate rotation (zero downtime)
sus rotated_manager = execute_certificate_rotation(
    updated_manager, 
    "example.com"
) fam {
    when _ -> vibez.spill("Certificate rotation failed: " + _)
}

vibez.spill("Certificate rotated successfully without service interruption")
```

### Automated Rotation Processing

```cursed
# Process all pending automatic rotations
sus processed_manager = process_automatic_rotations(rotation_manager) fam {
    when _ -> vibez.spill("Automatic rotation processing failed: " + _)
}

# Schedule specific rotation
sus scheduled_manager = schedule_certificate_rotation(
    processed_manager,
    "example.com", 
    timez.current_timestamp() + 86400  # Rotate in 24 hours
)
```

### Certificate Health Monitoring

```cursed
# Check certificate health
sus health_report = check_certificate_health(rotation_manager) fam {
    when _ -> vibez.spill("Health check failed: " + _)
}

vibez.spill("=== Certificate Health Report ===")
vibez.spill("Total certificates: " + stringz.from_int(health_report.total_certificates))
vibez.spill("Healthy certificates: " + stringz.from_int(health_report.healthy_certificates))
vibez.spill("Expiring soon: " + stringz.from_int(health_report.expiring_soon))
vibez.spill("Expired certificates: " + stringz.from_int(health_report.expired_certificates))

# Check for critical issues
ready (arrayz.length(health_report.critical_issues) > 0) {
    vibez.spill("CRITICAL ISSUES:")
    arrayz.foreach(health_report.critical_issues, slay(issue tea) {
        vibez.spill("  ❌ " + issue)
    })
}

# Check for warnings
ready (arrayz.length(health_report.warnings) > 0) {
    vibez.spill("WARNINGS:")
    arrayz.foreach(health_report.warnings, slay(warning tea) {
        vibez.spill("  ⚠️ " + warning)
    })
}
```

### Certificate Backup & Recovery

```cursed
# Backup certificate
sus backup_path = backup_certificate(
    active_cert, 
    "/var/backups/certificates"
) fam {
    when _ -> vibez.spill("Certificate backup failed: " + _)
}

vibez.spill("Certificate backed up to: " + backup_path)

# Restore certificate from backup
sus restored_manager = restore_certificate_from_backup(
    rotation_manager,
    "example.com",
    "/var/backups/certificates/example.com_cert.pem",
    "/var/backups/certificates/example.com_key.pem", 
    "backup_password"
) fam {
    when _ -> vibez.spill("Certificate restore failed: " + _)
}
```

## TLS Connection Pooling

Connection pooling optimizes TLS performance by reusing established connections.

### Connection Pool Setup

```cursed
yeet "tlsz/connection_pool"

# Create TLS connection pool
sus pool = create_tls_connection_pool(
    10,   # Max connections per host
    100,  # Max total connections
    300   # Idle timeout (5 minutes)
)

# High-performance pool configuration
sus hp_pool = create_high_performance_pool()
# - 20 connections per host
# - 200 total connections
# - Least-used eviction policy

# Conservative pool configuration
sus conservative_pool = create_conservative_pool()
# - 5 connections per host  
# - 50 total connections
# - FIFO eviction policy
```

### Getting Pooled Connections

```cursed
# Get connection from pool
sus connection = get_pooled_connection(
    pool,
    "api.example.com",
    443,
    create_default_security_policy()
) fam {
    when _ -> vibez.spill("Failed to get pooled connection: " + _)
}

vibez.spill("Connection acquired: " + connection.connection_id)
vibez.spill("Times used: " + stringz.from_int(connection.times_used))
vibez.spill("Session resumable: " + stringz.from_bool(connection.session_resumable))

# Use connection for requests
sus response = send_https_request(connection, "/api/data")

# Return connection to pool for reuse
pool = return_connection_to_pool(pool, connection)
```

### Pool Maintenance

```cursed
# Clean up expired connections
pool = cleanup_expired_connections(pool)

# Perform health checks
pool = perform_health_checks(pool)

# Get pool statistics
sus stats = get_pool_statistics(pool)
vibez.spill("Pool hit ratio: " + stringz.from_int(stats.pool_hit_ratio) + "%")
vibez.spill("Active connections: " + stringz.from_int(stats.active_connection_count))
vibez.spill("Idle connections: " + stringz.from_int(stats.idle_connection_count))
vibez.spill("Peak connections: " + stringz.from_int(stats.peak_connection_count))

# Generate detailed report
sus pool_report = generate_pool_report(pool)
vibez.spill(pool_report)
```

### Circuit Breaker

```cursed
# Circuit breaker protects against failing hosts
sus breaker = create_circuit_breaker()

# Record connection failures
breaker = record_failure(breaker)

# Record successful connections
breaker = record_success(breaker)

# Check if connections should be attempted
ready (should_attempt_connection(breaker)) {
    vibez.spill("Circuit breaker allows connection attempts")
} otherwise {
    vibez.spill("Circuit breaker is open - blocking connection attempts")
}

vibez.spill("Circuit breaker state: " + breaker.state)  # CLOSED, OPEN, HALF_OPEN
vibez.spill("Failure count: " + stringz.from_int(breaker.failure_count))
```

## Security Features

### Certificate Fingerprinting

```cursed
# Calculate certificate fingerprint for validation
sus fingerprint = calculate_certificate_fingerprint(certificate)
vibez.spill("Certificate fingerprint: " + fingerprint)

# Verify certificate hasn't changed
ready (stored_fingerprint == fingerprint) {
    vibez.spill("Certificate verified - fingerprint matches")
} otherwise {
    vibez.spill("WARNING: Certificate fingerprint mismatch!")
}
```

### Key Strength Validation

```cursed
# Validate certificate for installation
sus validation_result = validate_certificate_for_installation(
    certificate, 
    private_key, 
    cert_chain, 
    "example.com"
) fam {
    when _ -> vibez.spill("Certificate validation failed")
}

vibez.spill("Certificate validation:")
vibez.spill("  Valid: " + stringz.from_bool(validation_result.is_valid))
vibez.spill("  Trust score: " + stringz.from_int(validation_result.trust_score) + "%")
vibez.spill("  Security score: " + stringz.from_int(validation_result.security_score) + "%")
vibez.spill("  Compatibility score: " + stringz.from_int(validation_result.compatibility_score) + "%")

ready (arrayz.length(validation_result.validation_errors) > 0) {
    vibez.spill("  Errors:")
    arrayz.foreach(validation_result.validation_errors, slay(error tea) {
        vibez.spill("    ❌ " + error)
    })
}
```

### Constant-Time Operations

All cryptographic operations use constant-time implementations to prevent timing attacks:

```cursed
yeet "cryptz"

# Constant-time comparison
sus match = cryptz.constant_time_bytes_equal(hash1, hash2)

# Constant-time selection
sus selected_value = cryptz.constant_time_select(condition, true_val, false_val)

# Secure memory clearing
cryptz.secure_zero_memory(sensitive_data)
```

## Performance Optimization

### Connection Pool Tuning

```cursed
# Optimize pool settings based on usage patterns
sus optimized_pool = create_tls_connection_pool(
    20,   # Higher limit for high-traffic applications
    500,  # Large total connection limit
    600   # Longer idle timeout for persistent connections
)

# Set eviction policy
optimized_pool.eviction_policy = "least_used"  # Keep frequently used connections
optimized_pool.health_check_interval = 120     # More frequent health checks
```

### SNI Performance

```cursed
# Pre-compile certificate mappings for better performance
sus sni_config = create_sni_config()

# Add certificates in order of expected frequency
sni_config = add_sni_certificate(sni_config, "api.example.com", api_cert, key, [api_cert])
sni_config = add_sni_certificate(sni_config, "web.example.com", web_cert, key, [web_cert])

# Use case-insensitive matching for better cache performance
sni_config.case_sensitive = false
```

### Certificate Rotation Optimization

```cursed
# Optimize rotation timing
sus efficient_manager = create_certificate_rotation_manager(
    45,  # Longer threshold for less frequent rotations
    "/fast-storage/cert_backups"  # Use fast storage for backups
)

# Batch process rotations during low-traffic periods
efficient_manager.rotation_callback_func = "batch_rotation_callback"
```

## Best Practices

### Security Best Practices

1. **Always use mutual TLS for API authentication**:
   ```cursed
   sus mtls_config = create_mutual_tls_config(client_cert, client_key, cert_chain, trusted_cas)
   mtls_config.revocation_checking_enabled = true  # Enable revocation checks
   ```

2. **Implement strict SNI matching for production**:
   ```cursed
   sus strict_sni = create_strict_sni_config()
   strict_sni.fallback_behavior = "reject"  # Reject unknown hostnames
   ```

3. **Use comprehensive certificate validation**:
   ```cursed
   sus validation = validate_certificate_for_installation(cert, key, chain, hostname)
   ready (validation.security_score < 80) {
       vibez.spill("WARNING: Certificate has security issues")
   }
   ```

### Performance Best Practices

1. **Configure appropriate connection pool sizes**:
   ```cursed
   # For high-traffic services
   sus pool = create_tls_connection_pool(50, 1000, 300)
   
   # For low-traffic services  
   sus pool = create_tls_connection_pool(5, 50, 120)
   ```

2. **Use wildcard certificates when appropriate**:
   ```cursed
   # Single wildcard certificate for multiple subdomains
   sni_config = add_sni_certificate(sni_config, "*.api.example.com", wildcard_cert, key, [wildcard_cert])
   ```

3. **Implement proactive certificate rotation**:
   ```cursed
   # Rotate certificates well before expiry
   sus manager = create_certificate_rotation_manager(60, backup_dir)  # 60 days before expiry
   ```

### Monitoring Best Practices

1. **Regular health checks**:
   ```cursed
   # Schedule regular certificate health monitoring
   go {
       bestie (true) {
           sus health = check_certificate_health(rotation_manager)
           ready (health.expired_certificates > 0) {
               send_alert("Certificate expired!", health.critical_issues)
           }
           sleep(3600000)  # Check every hour
       }
   }
   ```

2. **Connection pool monitoring**:
   ```cursed
   # Monitor pool performance
   sus stats = get_pool_statistics(pool)
   ready (stats.pool_hit_ratio < 80) {
       vibez.spill("WARNING: Low pool hit ratio - consider tuning")
   }
   ```

3. **SNI usage tracking**:
   ```cursed
   # Track SNI statistics
   sus sni_stats = create_sni_statistics()
   # Update after each handshake
   sni_stats = update_sni_statistics(sni_stats, sni_result)
   
   # Generate periodic reports
   sus report = get_sni_statistics_report(sni_stats)
   ```

## API Reference

### Mutual TLS API

- `create_mutual_tls_config(client_cert, private_key, cert_chain, trusted_cas) -> MutualTLSConfig`
- `create_lenient_mutual_tls_config(client_cert, private_key, trusted_cas) -> MutualTLSConfig`
- `perform_mutual_tls_handshake(hostname, port, mtls_config, security_policy) -> TLSHandshakeContext`
- `verify_client_certificate_server_side(client_cert, cert_chain, mtls_config) -> ClientAuthResult`
- `extract_client_identity(client_cert) -> String`
- `authorize_client_access(client_identity, resource, acl) -> Boolean`

### SNI API

- `create_sni_config() -> SNIConfig`
- `create_strict_sni_config() -> SNIConfig`
- `add_sni_certificate(config, hostname, certificate, private_key, cert_chain) -> SNIConfig`
- `remove_sni_certificate(config, hostname) -> SNIConfig`
- `process_sni_handshake(hostname, config) -> SNIHandshakeResult`
- `matches_wildcard_pattern(hostname, pattern) -> Boolean`
- `validate_certificate_for_hostname(cert, hostname) -> Boolean`

### Certificate Rotation API

- `create_certificate_rotation_manager(threshold_days, backup_dir) -> CertificateRotationManager`
- `install_certificate(manager, hostname, certificate, private_key, cert_chain) -> CertificateRotationManager`
- `stage_certificate_for_rotation(manager, hostname, new_cert, new_key, new_chain) -> CertificateRotationManager`
- `execute_certificate_rotation(manager, hostname) -> CertificateRotationManager`
- `process_automatic_rotations(manager) -> CertificateRotationManager`
- `check_certificate_health(manager) -> CertificateHealthReport`
- `backup_certificate(cert, backup_dir) -> String`

### Connection Pool API

- `create_tls_connection_pool(max_per_host, max_total, idle_timeout) -> TLSConnectionPool`
- `get_pooled_connection(pool, hostname, port, security_policy) -> PooledTLSConnection`
- `return_connection_to_pool(pool, connection) -> TLSConnectionPool`
- `cleanup_expired_connections(pool) -> TLSConnectionPool`
- `perform_health_checks(pool) -> TLSConnectionPool`
- `get_pool_statistics(pool) -> PoolStatistics`
- `create_circuit_breaker() -> CircuitBreaker`

## Conclusion

The CURSED TLS enhancements provide enterprise-grade security and performance features essential for production applications. By implementing mutual TLS authentication, SNI support, automated certificate rotation, and connection pooling, applications can achieve:

- **Enhanced Security**: Bidirectional authentication and comprehensive certificate validation
- **Improved Performance**: Connection reuse and intelligent pooling strategies
- **Zero-Downtime Operations**: Hot certificate rotation without service interruption
- **Scalability**: Support for multiple domains and high-concurrency scenarios
- **Reliability**: Circuit breaker protection and health monitoring

These features make CURSED suitable for mission-critical applications requiring the highest levels of security and performance.
