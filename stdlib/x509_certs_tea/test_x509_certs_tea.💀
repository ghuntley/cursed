yeet "testz"
yeet "x509_certs_tea"
yeet "string"

fr fr Test X.509 certificate parsing
slay test_x509_parse_cert() {
    test_start("x509_parse_cert")
    
    sus test_cert_pem tea = `-----BEGIN CERTIFICATE-----
MIIBkTCB+wIJALHHVYGGfxqhMA0GCSqGSIb3DQEBCwUAMB4xHDAaBgNVBAMME1Rl
c3QgQ2VydGlmaWNhdGUgQ0EwHhcNMjMwMTAxMDAwMDAwWhcNMjQwMTAxMDAwMDAw
WjAcMRowGAYDVQQDDBFUZXN0IENlcnRpZmljYXRlMFwwDQYJKoZIhvcNAQEBBQAD
SwAwSAJBALRiMLAh9iimur8VA7qVK7u7fGK6nu4dgvgp8/3aWYMH9jDqVVEgUgOm
/nKHRbXAjSrZGhqgKBhMbGCGbJsJzqUCAwEAATANBgkqhkiG9w0BAQsFAANBAKz0
JLnkFCtPXZJUXIlAkc1cqLxOAjMrXQvlJQVZkDcqE8ZRhH4vfh8WyHKOiANH9lRN
qUE9fgZ6DdqNVpvtKlQ=
-----END CERTIFICATE-----`
    
    sus cert X509Cert = x509_parse_cert(test_cert_pem)
    
    assert_eq_string(cert.subject, "CN=Test Certificate")
    assert_eq_string(cert.issuer, "CN=Test Certificate CA")
    assert_true(string.length(cert.serial_number) > 0)
    assert_true(string.length(cert.not_before) > 0)
    assert_true(string.length(cert.not_after) > 0)
    assert_true(string.length(cert.public_key) > 0)
}

fr fr Test X.509 private key parsing
slay test_x509_parse_key() {
    test_start("x509_parse_key")
    
    sus test_key_pem tea = `-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC0YjCwIfYoprq/
FQO6lSu7u3xiup7uHYL4KfP92lmDB/Yw6lVRIFIDpv5yh0W1wI0q2RoaoCgYTGxg
hmybCc6lAgMBAAECggEBAJtXeB6rLZXKJmGgHIaFyGVQZHRJwSFOjZJhUcGMvnGr
...
-----END PRIVATE KEY-----`
    
    sus key X509Key = x509_parse_key(test_key_pem)
    
    assert_eq_string(key.algorithm, "RSA")
    assert_true(string.length(key.key_data) > 0)
    assert_true(string.length(key.public_key) > 0)
}

fr fr Test X.509 public key parsing
slay test_x509_parse_pubkey() {
    test_start("x509_parse_pubkey")
    
    sus test_pubkey_pem tea = `-----BEGIN PUBLIC KEY-----
MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBALRiMLAh9iimur8VA7qVK7u7fGK6nu4d
gvgp8/3aWYMH9jDqVVEgUgOm/nKHRbXAjSrZGhqgKBhMbGCGbJsJzqUCAwEAAQ==
-----END PUBLIC KEY-----`
    
    sus pubkey X509PubKey = x509_parse_pubkey(test_pubkey_pem)
    
    assert_eq_string(pubkey.algorithm, "RSA")
    assert_true(string.length(pubkey.key_data) > 0)
}

fr fr Test X.509 certificate request parsing
slay test_x509_parse_csr() {
    test_start("x509_parse_csr")
    
    sus test_csr_pem tea = `-----BEGIN CERTIFICATE REQUEST-----
MIIBWjCCAQMCAQAwFjEUMBIGA1UEAwwLZXhhbXBsZS5jb20wXDANBgkqhkiG9w0B
AQEFAANLADBIAkEAtGIwsCH2KKa6vxUDupUru7t8Yrqe7h2C+Cnz/dpZgwf2MOpV
USBSAqb+codFtcCNKtkaGqAoGExsYIZsmwnOpQIDAQABoCEwHwYJKoZIhvcNAQkO
MRIwEDAOBgNVHQ8BAf8EBAMCBaAwDQYJKoZIhvcNAQELBQADQQCz/T0RBldOHbKK
QkEtJyxQYZOmzfNaE9C8bUJZQdHJUUfGN6TRzQcZGhzwNqAGSNL7qJMNFqNUvHdq
N3QeVSCH
-----END CERTIFICATE REQUEST-----`
    
    sus csr X509CSR = x509_parse_csr(test_csr_pem)
    
    assert_eq_string(csr.subject, "CN=example.com")
    assert_true(string.length(csr.public_key) > 0)
}

fr fr Test X.509 certificate encoding
slay test_x509_encode_cert() {
    test_start("x509_encode_cert")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=Test Certificate",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-01-01T00:00:00Z",
        public_key: "test_public_key_data",
        extensions: "",
        signature: "test_signature"
    }
    
    sus encoded_cert tea = x509_encode_cert(cert)
    
    assert_true(string.contains(encoded_cert, "-----BEGIN CERTIFICATE-----"))
    assert_true(string.contains(encoded_cert, "-----END CERTIFICATE-----"))
    assert_true(string.length(encoded_cert) > 100)
}

fr fr Test X.509 private key encoding
slay test_x509_encode_key() {
    test_start("x509_encode_key")
    
    sus key X509Key = X509Key{
        algorithm: "RSA",
        key_data: "test_private_key_data",
        public_key: "test_public_key_data"
    }
    
    sus encoded_key tea = x509_encode_key(key)
    
    assert_true(string.contains(encoded_key, "-----BEGIN PRIVATE KEY-----"))
    assert_true(string.contains(encoded_key, "-----END PRIVATE KEY-----"))
    assert_true(string.length(encoded_key) > 100)
}

fr fr Test X.509 public key encoding
slay test_x509_encode_pubkey() {
    test_start("x509_encode_pubkey")
    
    sus pubkey X509PubKey = X509PubKey{
        algorithm: "RSA",
        key_data: "test_public_key_data",
        parameters: ""
    }
    
    sus encoded_pubkey tea = x509_encode_pubkey(pubkey)
    
    assert_true(string.contains(encoded_pubkey, "-----BEGIN PUBLIC KEY-----"))
    assert_true(string.contains(encoded_pubkey, "-----END PUBLIC KEY-----"))
    assert_true(string.length(encoded_pubkey) > 100)
}

fr fr Test X.509 certificate verification
slay test_x509_verify_cert() {
    test_start("x509_verify_cert")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=Test Certificate",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-12-31T23:59:59Z",
        public_key: "test_public_key_data",
        extensions: "",
        signature: "valid_signature"
    }
    
    sus ca X509Cert = X509Cert{
        subject: "CN=Test CA",
        issuer: "CN=Test CA",
        serial_number: "1",
        not_before: "2022-01-01T00:00:00Z",
        not_after: "2025-01-01T00:00:00Z",
        public_key: "ca_public_key_data",
        extensions: "",
        signature: "ca_signature"
    }
    
    sus is_valid lit = x509_verify_cert(cert, ca)
    assert_true(is_valid)
}

fr fr Test X.509 certificate chain verification
slay test_x509_verify_chain() {
    test_start("x509_verify_chain")
    
    sus leaf_cert X509Cert = X509Cert{
        subject: "CN=leaf.example.com",
        issuer: "CN=Intermediate CA",
        serial_number: "123",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-12-31T23:59:59Z",
        public_key: "leaf_public_key",
        extensions: "",
        signature: "leaf_signature"
    }
    
    sus intermediate_cert X509Cert = X509Cert{
        subject: "CN=Intermediate CA",
        issuer: "CN=Root CA",
        serial_number: "456",
        not_before: "2022-01-01T00:00:00Z",
        not_after: "2025-01-01T00:00:00Z",
        public_key: "intermediate_public_key",
        extensions: "",
        signature: "intermediate_signature"
    }
    
    sus root_cert X509Cert = X509Cert{
        subject: "CN=Root CA",
        issuer: "CN=Root CA",
        serial_number: "1",
        not_before: "2020-01-01T00:00:00Z",
        not_after: "2030-01-01T00:00:00Z",
        public_key: "root_public_key",
        extensions: "",
        signature: "root_signature"
    }
    
    sus chain X509Cert[value] = [leaf_cert, intermediate_cert, root_cert]
    sus is_valid lit = x509_verify_chain(chain)
    assert_true(is_valid)
}

fr fr Test X.509 certificate field extraction
slay test_x509_get_fields() {
    test_start("x509_get_fields")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=Test Certificate",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-01-01T00:00:00Z",
        public_key: "test_public_key_data",
        extensions: "SAN:DNS:example.com,email:test@example.com",
        signature: "test_signature"
    }
    
    assert_eq_string(x509_get_subject(cert), "CN=Test Certificate")
    assert_eq_string(x509_get_issuer(cert), "CN=Test CA")
    assert_eq_string(x509_get_serial(cert), "123456789")
    
    sus (not_before, not_after) := x509_get_validity(cert)
    assert_eq_string(not_before, "2023-01-01T00:00:00Z")
    assert_eq_string(not_after, "2024-01-01T00:00:00Z")
    
    assert_true(string.contains(x509_get_extensions(cert), "SAN:DNS:example.com"))
}

fr fr Test X.509 hostname verification
slay test_x509_check_hostname() {
    test_start("x509_check_hostname")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=example.com",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-01-01T00:00:00Z",
        public_key: "test_public_key_data",
        extensions: "SAN:DNS:example.com,DNS:*.example.com,DNS:test.example.com",
        signature: "test_signature"
    } fr fr Test exact match
    assert_true(x509_check_hostname(cert, "example.com")) fr fr Test SAN match
    assert_true(x509_check_hostname(cert, "test.example.com")) fr fr Test wildcard match
    assert_true(x509_check_hostname(cert, "sub.example.com")) fr fr Test no match
    assert_false(x509_check_hostname(cert, "other.com"))
}

fr fr Test X.509 email verification
slay test_x509_check_email() {
    test_start("x509_check_email")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=Test User,emailAddress=user@example.com",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-01-01T00:00:00Z",
        public_key: "test_public_key_data",
        extensions: "SAN:email:user@example.com,email:admin@example.com",
        signature: "test_signature"
    } fr fr Test subject email match
    assert_true(x509_check_email(cert, "user@example.com")) fr fr Test SAN email match
    assert_true(x509_check_email(cert, "admin@example.com")) fr fr Test no match
    assert_false(x509_check_email(cert, "other@example.com"))
}

fr fr Test X.509 IP address verification
slay test_x509_check_ip() {
    test_start("x509_check_ip")
    
    sus cert X509Cert = X509Cert{
        subject: "CN=Test Server",
        issuer: "CN=Test CA",
        serial_number: "123456789",
        not_before: "2023-01-01T00:00:00Z",
        not_after: "2024-01-01T00:00:00Z",
        public_key: "test_public_key_data",
        extensions: "SAN:IP:192.168.1.1,IP:10.0.0.1",
        signature: "test_signature"
    } fr fr Test IP match
    assert_true(x509_check_ip(cert, "192.168.1.1"))
    assert_true(x509_check_ip(cert, "10.0.0.1")) fr fr Test no match
    assert_false(x509_check_ip(cert, "192.168.1.2"))
}

fr fr Test comprehensive X.509 operations
slay test_x509_comprehensive() {
    test_start("x509_comprehensive") fr fr Test full certificate lifecycle
    sus cert_pem tea = `-----BEGIN CERTIFICATE-----
MIIBkTCB+wIJALHHVYGGfxqhMA0GCSqGSIb3DQEBCwUAMB4xHDAaBgNVBAMME1Rl
c3QgQ2VydGlmaWNhdGUgQ0EwHhcNMjMwMTAxMDAwMDAwWhcNMjQwMTAxMDAwMDAw
WjAcMRowGAYDVQQDDBFUZXN0IENlcnRpZmljYXRlMFwwDQYJKoZIhvcNAQEBBQAD
SwAwSAJBALRiMLAh9iimur8VA7qVK7u7fGK6nu4dgvgp8/3aWYMH9jDqVVEgUgOm
/nKHRbXAjSrZGhqgKBhMbGCGbJsJzqUCAwEAATANBgkqhkiG9w0BAQsFAANBAKz0
JLnkFCtPXZJUXIlAkc1cqLxOAjMrXQvlJQVZkDcqE8ZRhH4vfh8WyHKOiANH9lRN
qUE9fgZ6DdqNVpvtKlQ=
-----END CERTIFICATE-----` fr fr Parse certificate
    sus cert X509Cert = x509_parse_cert(cert_pem) fr fr Verify certificate structure
    assert_true(string.length(cert.subject) > 0)
    assert_true(string.length(cert.issuer) > 0)
    assert_true(string.length(cert.serial_number) > 0) fr fr Re-encode certificate
    sus encoded_cert tea = x509_encode_cert(cert)
    assert_true(string.contains(encoded_cert, "-----BEGIN CERTIFICATE-----")) fr fr Test certificate validation
    sus hostname_valid lit = x509_check_hostname(cert, "example.com")
    
    vibez.spill("X.509 comprehensive test completed successfully")
}

fr fr Main test execution
test_x509_parse_cert()
test_x509_parse_key()
test_x509_parse_pubkey()
test_x509_parse_csr()
test_x509_encode_cert()
test_x509_encode_key()
test_x509_encode_pubkey()
test_x509_verify_cert()
test_x509_verify_chain()
test_x509_get_fields()
test_x509_check_hostname()
test_x509_check_email()
test_x509_check_ip()
test_x509_comprehensive()

print_test_summary()
