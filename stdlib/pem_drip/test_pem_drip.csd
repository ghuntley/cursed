fr fr PEM Encoding/Decoding Module Tests
fr fr Comprehensive test suite for PEM operations

yeet "testz"
yeet "pem_drip"

fr fr Test PEM encode/decode basic functionality
slay test_pem_encode_decode() {
    test_start("PEM encode/decode basic")
    
    sus test_data tea = "Hello, PEM World!"
    sus encoded tea = pem_encode(test_data, "TEST")
    sus decoded tea = pem_decode(encoded)
    
    assert_eq_string(decoded, test_data)
}

fr fr Test PEM validation
slay test_pem_validate() {
    test_start("PEM validation")
    
    sus valid_pem tea = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----"
    sus invalid_pem tea = "This is not a PEM block"
    
    assert_true(pem_validate(valid_pem))
    assert_false(pem_validate(invalid_pem))
}

fr fr Test PEM parsing
slay test_pem_parse() {
    test_start("PEM parsing")
    
    sus pem_data tea = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----"
    sus blocks tea = pem_parse(pem_data)
    
    assert_true(len(blocks) > 0)
    
    sus label tea = pem_get_label(blocks)
    assert_eq_string(label, "CERTIFICATE")
}

fr fr Test PEM block creation and extraction
slay test_pem_block_operations() {
    test_start("PEM block operations")
    
    sus test_label tea = "TEST BLOCK"
    sus test_headers tea = "Version: 1\nType: test"
    sus test_body tea = "VGVzdCBkYXRh"
    
    sus block tea = pem_block_create(test_label, test_headers, test_body)
    
    sus extracted_label tea = pem_get_label(block)
    sus extracted_headers tea = pem_get_headers(block)
    sus extracted_body tea = pem_get_body(block)
    
    assert_eq_string(extracted_label, test_label)
    assert_eq_string(extracted_headers, test_headers)
    assert_eq_string(extracted_body, test_body)
}

fr fr Test PEM encode block
slay test_pem_encode_block() {
    test_start("PEM encode block")
    
    sus test_block tea = pem_block_create("TEST", "", "VGVzdA==")
    sus encoded tea = pem_encode_block(test_block)
    
    assert_true(pem_validate(encoded))
    assert_true(starts_with(encoded, "-----BEGIN TEST-----"))
}

fr fr Test PEM decode block
slay test_pem_decode_block() {
    test_start("PEM decode block")
    
    sus test_data tea = "Test"
    sus test_block tea = pem_block_create("TEST", "", "VGVzdA==")
    sus decoded tea = pem_decode_block(test_block) fr fr Base64 decode should work
    assert_true(len(decoded) > 0)
}

fr fr Test certificate extraction
slay test_pem_extract_cert() {
    test_start("PEM certificate extraction")
    
    sus cert_pem tea = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----"
    sus cert_data tea = pem_extract_cert(cert_pem)
    
    assert_true(len(cert_data) > 0)
}

fr fr Test private key extraction
slay test_pem_extract_key() {
    test_start("PEM private key extraction")
    
    sus key_pem tea = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC\n-----END PRIVATE KEY-----"
    sus key_data tea = pem_extract_key(key_pem)
    
    assert_true(len(key_data) > 0)
}

fr fr Test public key extraction
slay test_pem_extract_pubkey() {
    test_start("PEM public key extraction")
    
    sus pubkey_pem tea = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END PUBLIC KEY-----"
    sus pubkey_data tea = pem_extract_pubkey(pubkey_pem)
    
    assert_true(len(pubkey_data) > 0)
}

fr fr Test CSR extraction
slay test_pem_extract_csr() {
    test_start("PEM CSR extraction")
    
    sus csr_pem tea = "-----BEGIN CERTIFICATE REQUEST-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE REQUEST-----"
    sus csr_data tea = pem_extract_csr(csr_pem)
    
    assert_true(len(csr_data) > 0)
}

fr fr Test CRL extraction
slay test_pem_extract_crl() {
    test_start("PEM CRL extraction")
    
    sus crl_pem tea = "-----BEGIN X509 CRL-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END X509 CRL-----"
    sus crl_data tea = pem_extract_crl(crl_pem)
    
    assert_true(len(crl_data) > 0)
}

fr fr Test base64 encoding/decoding
slay test_base64_operations() {
    test_start("Base64 operations")
    
    sus test_data tea = "Hello World"
    sus encoded tea = base64_encode(test_data)
    sus decoded tea = base64_decode(encoded)
    
    assert_eq_string(decoded, test_data)
}

fr fr Test multiple PEM blocks
slay test_multiple_pem_blocks() {
    test_start("Multiple PEM blocks")
    
    sus multi_pem tea = "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----\n-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC\n-----END PRIVATE KEY-----"
    sus blocks tea = pem_parse(multi_pem)
    
    assert_true(len(blocks) > 0)
    
    sus cert_data tea = pem_extract_cert(multi_pem)
    sus key_data tea = pem_extract_key(multi_pem)
    
    assert_true(len(cert_data) > 0)
    assert_true(len(key_data) > 0)
}

fr fr Test PEM with headers
slay test_pem_with_headers() {
    test_start("PEM with headers")
    
    sus pem_with_headers tea = "-----BEGIN CERTIFICATE-----\nVersion: 3\nSerial: 12345\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA\n-----END CERTIFICATE-----"
    sus blocks tea = pem_parse(pem_with_headers)
    
    assert_true(len(blocks) > 0)
    
    sus headers tea = pem_get_headers(blocks)
    assert_true(len(headers) > 0)
}

fr fr Test edge cases
slay test_edge_cases() {
    test_start("Edge cases") fr fr Empty data
    sus empty_result tea = pem_decode("")
    assert_eq_string(empty_result, "") fr fr Invalid PEM
    sus invalid_data tea = "Not a PEM block"
    assert_false(pem_validate(invalid_data)) fr fr Empty label
    sus empty_label_pem tea = pem_encode("test", "")
    assert_true(pem_validate(empty_label_pem))
}

fr fr Test PEM line length compliance
slay test_pem_line_length() {
    test_start("PEM line length compliance")
    
    sus long_data tea = "This is a very long string that should be wrapped at 64 characters per line when encoded in PEM format according to RFC specifications"
    sus encoded tea = pem_encode(long_data, "TEST")
    
    assert_true(pem_validate(encoded)) fr fr Check that lines are properly wrapped
    sus lines tea = split(encoded, "\n")
    assert_true(len(lines) > 3) fr fr Should have header, body lines, and footer
}

fr fr Test RFC 7468 compliance
slay test_rfc_compliance() {
    test_start("RFC 7468 compliance")
    
    sus test_data tea = "RFC compliance test data"
    sus encoded tea = pem_encode(test_data, "TEST DATA")
    
    assert_true(pem_validate(encoded))
    assert_true(starts_with(encoded, "-----BEGIN TEST DATA-----"))
    
    sus decoded tea = pem_decode(encoded)
    assert_eq_string(decoded, test_data)
}

fr fr Test security considerations
slay test_security_features() {
    test_start("Security features") fr fr Test that padding is handled correctly
    sus test_data1 tea = "A"
    sus test_data2 tea = "AB"
    sus test_data3 tea = "ABC"
    
    sus encoded1 tea = pem_encode(test_data1, "TEST")
    sus encoded2 tea = pem_encode(test_data2, "TEST")
    sus encoded3 tea = pem_encode(test_data3, "TEST")
    
    sus decoded1 tea = pem_decode(encoded1)
    sus decoded2 tea = pem_decode(encoded2)
    sus decoded3 tea = pem_decode(encoded3)
    
    assert_eq_string(decoded1, test_data1)
    assert_eq_string(decoded2, test_data2)
    assert_eq_string(decoded3, test_data3)
}

fr fr Test performance with large data
slay test_large_data_performance() {
    test_start("Large data performance")
    
    sus large_data tea = ""
    bestie i := 0; i < 100; i++ {
        large_data = large_data + "This is test data for performance testing. "
    }
    
    sus encoded tea = pem_encode(large_data, "LARGE TEST")
    sus decoded tea = pem_decode(encoded)
    
    assert_eq_string(decoded, large_data)
    assert_true(pem_validate(encoded))
}

fr fr Test utility functions
slay test_utility_functions() {
    test_start("Utility functions") fr fr Test starts_with
    assert_true(starts_with("Hello World", "Hello"))
    assert_false(starts_with("Hello World", "World")) fr fr Test contains
    assert_true(contains("Hello World", "World"))
    assert_false(contains("Hello World", "xyz")) fr fr Test trim
    sus trimmed tea = trim("  test  ")
    assert_eq_string(trimmed, "test") fr fr Test split
    sus parts tea = split("a,b,c", ",")
    assert_true(len(parts) > 0)
}

fr fr Main test runner
slay main_character() {
    vibez.spill("Running PEM Drip Module Tests...")
    
    test_pem_encode_decode()
    test_pem_validate()
    test_pem_parse()
    test_pem_block_operations()
    test_pem_encode_block()
    test_pem_decode_block()
    test_pem_extract_cert()
    test_pem_extract_key()
    test_pem_extract_pubkey()
    test_pem_extract_csr()
    test_pem_extract_crl()
    test_base64_operations()
    test_multiple_pem_blocks()
    test_pem_with_headers()
    test_edge_cases()
    test_pem_line_length()
    test_rfc_compliance()
    test_security_features()
    test_large_data_performance()
    test_utility_functions()
    
    print_test_summary()
}

fr fr Helper functions for tests
slay starts_with(str tea, prefix tea) lit {
    sis len(str) < len(prefix) {
        damn cap
    }
    damn str[0:len(prefix)] == prefix
}

slay contains(str tea, substr tea) lit {
    bestie i := 0; i <= len(str) - len(substr); i++ {
        sis str[i:i+len(substr)] == substr {
            damn based
        }
    }
    damn cap
}

slay split(str tea, delimiter tea) tea {
    sus result tea = ""
    sus current tea = ""
    
    bestie i := 0; i < len(str); i++ {
        sus ch sip = str[i]
        sus ch_str tea = char(ch)
        
        sis ch_str == delimiter {
            sis len(result) == 0 {
                result = current
            } else {
                result = result + "|" + current
            }
            current = ""
        } else {
            current = current + ch_str
        }
    } fr fr Add final part
    sis len(current) > 0 {
        sis len(result) == 0 {
            result = current
        } else {
            result = result + "|" + current
        }
    }
    
    damn result
}

slay len(str tea) normie {
    sus count normie = 0
    bestie i := 0; i < 10000; i++ {
        sus ch sip = str[i]
        sis ord(char(ch)) == 0 {
            ghosted
        }
        count++
    }
    damn count
}

slay char(ch sip) tea {
    damn ch
}

slay ord(str tea) normie {
    sis len(str) == 0 {
        damn 0
    }
    sus ch sip = str[0]
    damn ch
}

main()
