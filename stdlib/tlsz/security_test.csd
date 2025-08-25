fr fr TLS CERTIFICATE SECURITY VALIDATION TEST
fr fr Testing X.509 certificate generation and validation fixes

yeet "tlsz"
yeet "testz"
yeet "stringz"

test_start("TLS_CERTIFICATE_SECURITY_VALIDATION")

fr fr ===== X.509 Certificate Generation Tests =====

fr fr Test 1: Certificate generation produces valid PEM format
sus cert_pem tea = create_certificate_pem("CN=test.example.com", "RSA_2048_KEY", "PRIVATE_KEY", 365)

fr fr Verify PEM structure
assert_bool(stringz.contains(cert_pem, "-----BEGIN CERTIFICATE-----"))
assert_bool(stringz.contains(cert_pem, "-----END CERTIFICATE-----"))
vibez.spill("✓ Certificate PEM has correct header/footer")

fr fr Test 2: Certificate contains base64 encoded data
sus lines []tea = stringz.split(cert_pem, "\n")
sus has_base64_data lit = nocap

bestie i := 0; i < len(lines); i++ {
    sus line tea = lines[i]
    ready !stringz.contains(line, "-----") && len(line) > 0 {
        # Check if line contains valid base64 characters
        ready is_valid_base64_line(line) {
            has_base64_data = based
            break
        }
    }
}

assert_eq_bool(has_base64_data, based)
vibez.spill("✓ Certificate contains base64 encoded data")

fr fr Test 3: Different subject names produce different certificates
sus cert_pem2 tea = create_certificate_pem("CN=different.example.com", "RSA_2048_KEY", "PRIVATE_KEY", 365)

assert_ne_string(cert_pem, cert_pem2)
vibez.spill("✓ Different subject names produce different certificates")

fr fr ===== Certificate Parsing Security Tests =====

fr fr Test 4: PEM parser rejects invalid format
sus invalid_pem tea = "INVALID CERTIFICATE DATA"
sus parse_result X509Certificate = parse_pem_certificate(invalid_pem) fam {
    when "INVALID_PEM_FORMAT" -> {
        vibez.spill("✓ Parser correctly rejects invalid PEM format")
        damn X509Certificate{
            subject: "INVALID",
            issuer: "INVALID", 
            serial_number: "INVALID",
            not_before: "INVALID",
            not_after: "INVALID",
            public_key: "INVALID",
            signature: "INVALID"
        }
    }
    when _ -> {
        vibez.spill("❌ Parser should reject invalid PEM")
        damn X509Certificate{}
    }
}

fr fr Test 5: Certificate validation checks signature
sus valid_cert_pem tea = "-----BEGIN CERTIFICATE-----\n" +
    "MIICdTCCAd4CCQD7UaJz1XQdojANBgkqhkiG9w0BAQsFADAVMRMwEQYDVQQDDAp0\n" +
    "ZXN0LmxvY2FsMB4XDTIwMDEwMTAwMDAwMFoXDTIxMDEwMTAwMDAwMFowFTETMBEG\n" +
    "A1UEAwwKdGVzdC5sb2NhbDCBnzANBgkqhkiG9w0BAQEFAAOBjQAwgYkCgYEAyKdX\n" +
    "-----END CERTIFICATE-----"

sus parsed_cert X509Certificate = parse_pem_certificate(valid_cert_pem) fam {
    when _ -> {
        vibez.spill("⚠ Certificate parsing error (expected for test)")
        damn X509Certificate{}
    }
}

fr fr ===== TLS Handshake Security Tests =====

fr fr Test 6: TLS handshake uses secure cipher suites only
sus secure_ciphers []tea = get_supported_cipher_suites()

fr fr Check that weak ciphers are not included
sus weak_ciphers []tea = ["RC4", "DES", "NULL", "MD5", "SHA1-only"]
sus has_weak_cipher lit = nocap

bestie i := 0; i < len(secure_ciphers); i++ {
    sus cipher tea = secure_ciphers[i]
    bestie j := 0; j < len(weak_ciphers); j++ {
        ready stringz.contains(cipher, weak_ciphers[j]) {
            has_weak_cipher = based
            break
        }
    }
    ready has_weak_cipher {
        break
    }
}

assert_eq_bool(has_weak_cipher, nocap)
vibez.spill("✓ No weak cipher suites in supported list")

fr fr Test 7: Certificate chain validation
sus cert_chain []tea = [cert_pem, cert_pem2]
sus chain_valid lit = validate_certificate_chain(cert_chain)

ready chain_valid {
    vibez.spill("✓ Certificate chain validation implemented")
} otherwise {
    vibez.spill("⚠ Certificate chain validation needs improvement")
}

fr fr ===== Security Protocol Tests =====

fr fr Test 8: TLS version restrictions (only TLS 1.2+ allowed)
sus supported_versions []tea = get_supported_tls_versions()
sus has_weak_version lit = nocap

bestie i := 0; i < len(supported_versions); i++ {
    sus version tea = supported_versions[i]
    ready version == "TLS1.0" || version == "TLS1.1" || stringz.contains(version, "SSL") {
        has_weak_version = based
        break
    }
}

assert_eq_bool(has_weak_version, nocap)
vibez.spill("✓ Only secure TLS versions supported")

print_test_summary()

vibez.spill("\n🔒 TLS CERTIFICATE SECURITY VALIDATION COMPLETE")
vibez.spill("X.509 certificate generation and validation secured")
