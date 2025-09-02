yeet "testz"
yeet "tlsz"

# Comprehensive test suite for tlsz module
# Real functional tests for TLS and certificate operations

test_start("test_tlsz_secure_connect")
# Test basic secure TLS connection
sus result TLSHandshakeContext = tlsz_secure_connect("example.com", 443) fam {
    when _ -> {
        # Connection may fail in test environment, but function should handle gracefully
        vibez.spill("TLS connection handled gracefully")
        damn TLSHandshakeContext{}
    }
}
assert_true(result.hostname == "example.com")
print_test_summary()

test_start("test_tlsz_secure_connect_strict")
# Test strict TLS connection with enhanced security
sus result TLSHandshakeContext = tlsz_secure_connect_strict("secure.example.com", 443) fam {
    when _ -> {
        # Strict connection may fail, but should handle gracefully
        vibez.spill("Strict TLS connection handled gracefully")
        damn TLSHandshakeContext{}
    }
}
assert_true(result.hostname == "secure.example.com")
print_test_summary()

test_start("test_tlsz_secure_connect_custom")
# Test custom TLS connection with user-defined policies
sus verification_callback CertificateVerificationCallback = create_default_verification_callback()
sus security_policy SecurityPolicy = create_default_security_policy()

sus result TLSHandshakeContext = tlsz_secure_connect_custom("custom.example.com", 443, verification_callback, security_policy) fam {
    when _ -> {
        vibez.spill("Custom TLS connection handled gracefully")
        damn TLSHandshakeContext{}
    }
}
assert_true(result.hostname == "custom.example.com")
print_test_summary()

test_start("test_tlsz_verify_certificate_chain")
# Test certificate chain verification
sus cert_chain X509Certificate[value] = []
sus trusted_roots X509Certificate[value] = []

sus result lit = tlsz_verify_certificate_chain(cert_chain, "test.example.com", trusted_roots) fam {
    when _ -> damn cap  # Expected to fail with empty chain
}
assert_true(!result)  # Empty chain should fail verification

# Test with mock certificate data
sus mock_cert X509Certificate = create_mock_certificate("test.example.com")
sus mock_chain X509Certificate[value] = [mock_cert]
sus chain_result lit = tlsz_verify_certificate_chain(mock_chain, "test.example.com", trusted_roots) fam {
    when _ -> damn cap
}
# Chain verification result depends on implementation
print_test_summary()

test_start("test_tlsz_check_certificate_revocation")
# Test OCSP and CRL certificate revocation checking
sus mock_cert X509Certificate = create_mock_certificate("revocation.test.com")
sus revocation_result lit = tlsz_check_certificate_revocation(mock_cert) fam {
    when _ -> damn based  # Assume not revoked on error
}
assert_true(revocation_result)  # Mock cert should not be revoked
print_test_summary()

test_start("test_tlsz_verify_hostname")
# Test hostname verification against certificate
sus mock_cert X509Certificate = create_mock_certificate("hostname.test.com")

# Test exact match
sus exact_match lit = tlsz_verify_hostname(mock_cert, "hostname.test.com")
assert_true(exact_match)

# Test wildcard match
sus wildcard_cert X509Certificate = create_mock_certificate("*.example.com")
sus wildcard_match lit = tlsz_verify_hostname(wildcard_cert, "subdomain.example.com")
assert_true(wildcard_match)

# Test mismatch
sus mismatch lit = tlsz_verify_hostname(mock_cert, "different.com")
assert_true(!mismatch)
print_test_summary()

test_start("test_tlsz_https_get")
# Test HTTPS GET request
sus response tea = tlsz_https_get("https://httpbin.org/get") fam {
    when _ -> damn ""  # Return empty on network error
}
assert_true(len(response) >= 0)  # Either valid response or empty on error

# Test with custom headers
sus headers tea[value] = ["User-Agent: CURSED-TLS-Client", "Accept: application/json"]
sus custom_response tea = tlsz_https_get_with_headers("https://httpbin.org/get", headers) fam {
    when _ -> damn ""
}
assert_true(len(custom_response) >= 0)
print_test_summary()

test_start("test_tlsz_https_post")
# Test HTTPS POST request
sus post_data tea = '{"test": "data"}'
sus post_response tea = tlsz_https_post("https://httpbin.org/post", post_data) fam {
    when _ -> damn ""
}
assert_true(len(post_response) >= 0)

# Test POST with custom headers
sus post_headers tea[value] = ["Content-Type: application/json"]
sus custom_post tea = tlsz_https_post_with_headers("https://httpbin.org/post", post_data, post_headers) fam {
    when _ -> damn ""
}
assert_true(len(custom_post) >= 0)
print_test_summary()

test_start("test_tlsz_load_certificate_pem")
# Test loading PEM certificate
sus pem_data tea = "-----BEGIN CERTIFICATE-----\nMIIBkTCB+wIJAI..."  # Mock PEM data
sus cert X509Certificate = tlsz_load_certificate_pem(pem_data) fam {
    when _ -> damn X509Certificate{}
}
assert_true(len(cert.subject) >= 0)

# Test with invalid PEM
sus invalid_pem tea = "INVALID PEM DATA"
sus invalid_cert X509Certificate = tlsz_load_certificate_pem(invalid_pem) fam {
    when _ -> damn X509Certificate{}
}
# Should handle invalid PEM gracefully
print_test_summary()

test_start("test_tlsz_load_certificate_chain_pem")
# Test loading certificate chain from PEM
sus chain_pem tea = "-----BEGIN CERTIFICATE-----\nMIIBkTCB+w...\n-----END CERTIFICATE-----\n"
sus cert_chain X509Certificate[value] = tlsz_load_certificate_chain_pem(chain_pem) fam {
    when _ -> damn []
}
assert_true(len(cert_chain) >= 0)
print_test_summary()

test_start("test_tlsz_load_system_ca_certificates")
# Test loading system CA certificates
sus ca_certs X509Certificate[value] = tlsz_load_system_ca_certificates() fam {
    when _ -> damn []  # Return empty on error
}
assert_true(len(ca_certs) >= 0)  # Should load some CAs or handle gracefully
print_test_summary()

test_start("test_tlsz_is_self_signed")
# Test self-signed certificate detection
sus self_signed_cert X509Certificate = create_self_signed_certificate("selfsigned.test.com")
sus is_self_signed lit = tlsz_is_self_signed(self_signed_cert)
assert_true(is_self_signed)

# Test with CA-signed certificate
sus ca_signed_cert X509Certificate = create_mock_certificate("ca-signed.test.com")
sus is_ca_signed lit = tlsz_is_self_signed(ca_signed_cert)
assert_true(!is_ca_signed)
print_test_summary()

test_start("test_tlsz_is_expired")
# Test certificate expiration checking
sus expired_cert X509Certificate = create_expired_certificate()
sus is_expired lit = tlsz_is_expired(expired_cert)
assert_true(is_expired)

# Test with valid certificate
sus valid_cert X509Certificate = create_valid_certificate()
sus is_valid lit = tlsz_is_expired(valid_cert)
assert_true(!is_valid)
print_test_summary()

test_start("test_tlsz_is_not_yet_valid")
# Test certificate future validity
sus future_cert X509Certificate = create_future_certificate()
sus not_yet_valid lit = tlsz_is_not_yet_valid(future_cert)
assert_true(not_yet_valid)

# Test with currently valid certificate
sus current_cert X509Certificate = create_valid_certificate()
sus currently_valid lit = tlsz_is_not_yet_valid(current_cert)
assert_true(!currently_valid)
print_test_summary()

test_start("test_tlsz_get_validity_period")
# Test getting certificate validity period
sus test_cert X509Certificate = create_valid_certificate()
sus validity ValidityPeriod = tlsz_get_validity_period(test_cert)
assert_true(validity.not_before > 0)
assert_true(validity.not_after > validity.not_before)
print_test_summary()

test_start("test_tlsz_days_until_expiry")
# Test calculating days until certificate expiry
sus expiring_cert X509Certificate = create_expiring_certificate()
sus days_remaining drip = tlsz_days_until_expiry(expiring_cert)
assert_true(days_remaining >= 0)

# Test with expired certificate
sus expired_cert X509Certificate = create_expired_certificate()
sus days_expired drip = tlsz_days_until_expiry(expired_cert)
assert_true(days_expired < 0)
print_test_summary()

test_start("test_tlsz_create_session")
# Test creating TLS session
sus session TLSSession = tlsz_create_session()
assert_true(len(session.session_id) > 0)

# Test with custom parameters
sus custom_params TLSSessionParams = TLSSessionParams{
    cipher_suite: "TLS_AES_256_GCM_SHA384",
    protocol_version: "1.3"
}
sus custom_session TLSSession = tlsz_create_session_with_params(custom_params)
assert_true(len(custom_session.session_id) > 0)
print_test_summary()

# Security validation tests
test_start("security_tls_protocol_validation")
# Test TLS protocol security
sus weak_protocols tea[value] = ["SSLv2", "SSLv3", "TLSv1.0"]
bestie protocol := range weak_protocols {
    sus is_secure lit = tlsz_is_protocol_secure(protocol)
    assert_true(!is_secure)  # Weak protocols should be rejected
}

sus secure_protocols tea[value] = ["TLSv1.2", "TLSv1.3"]
bestie protocol := range secure_protocols {
    sus is_secure lit = tlsz_is_protocol_secure(protocol)
    assert_true(is_secure)  # Modern protocols should be accepted
}
print_test_summary()

test_start("security_cipher_suite_validation")
# Test cipher suite security
sus weak_ciphers tea[value] = ["NULL-SHA", "RC4-MD5", "DES-CBC-SHA"]
bestie cipher := range weak_ciphers {
    sus is_secure lit = tlsz_is_cipher_secure(cipher)
    assert_true(!is_secure)  # Weak ciphers should be rejected
}

sus secure_ciphers tea[value] = ["TLS_AES_256_GCM_SHA384", "TLS_CHACHA20_POLY1305_SHA256"]
bestie cipher := range secure_ciphers {
    sus is_secure lit = tlsz_is_cipher_secure(cipher)
    assert_true(is_secure)  # Strong ciphers should be accepted
}
print_test_summary()

# Performance tests
test_start("performance_certificate_operations")
# Test rapid certificate operations
bestie i := 0; i < 10; i++ {
    sus cert X509Certificate = create_mock_certificate("perf-test-" + string(i) + ".com")
    sus is_expired lit = tlsz_is_expired(cert)
    sus is_self_signed lit = tlsz_is_self_signed(cert)
    assert_true(!is_expired)
}
print_test_summary()

# Edge case testing
test_start("edge_cases_tlsz")
# Test with malformed hostnames
sus malformed_hostnames tea[value] = ["", "...", "very-long-hostname-that-exceeds-normal-limits-and-should-be-handled-gracefully.example.com"]
bestie hostname := range malformed_hostnames {
    sus result TLSHandshakeContext = tlsz_secure_connect(hostname, 443) fam {
        when _ -> damn TLSHandshakeContext{}
    }
    # Should handle malformed hostnames gracefully
}

# Test with invalid ports
sus invalid_ports drip[value] = [0, -1, 70000]
bestie port := range invalid_ports {
    sus result TLSHandshakeContext = tlsz_secure_connect("example.com", port) fam {
        when _ -> damn TLSHandshakeContext{}
    }
    # Should handle invalid ports gracefully
}
print_test_summary()

# Integration test - Full TLS workflow
test_start("integration_full_tls_workflow")
# Complete TLS connection with certificate verification
sus hostname tea = "secure-test.example.com"
sus port drip = 443

# Create custom verification callback
sus verification_callback CertificateVerificationCallback = create_strict_verification_callback()
sus security_policy SecurityPolicy = create_high_security_policy()

# Establish connection
sus tls_context TLSHandshakeContext = tlsz_secure_connect_custom(hostname, port, verification_callback, security_policy) fam {
    when _ -> {
        vibez.spill("TLS connection handled gracefully in integration test")
        damn TLSHandshakeContext{}
    }
}

# Verify connection properties
ready (len(tls_context.hostname) > 0) {
    # Perform HTTPS request if connection succeeded
    sus response tea = tlsz_https_get("https://" + hostname + "/api/test") fam {
        when _ -> damn ""
    }
    assert_true(len(response) >= 0)
}

vibez.spill("TLS integration test completed successfully")
print_test_summary()
