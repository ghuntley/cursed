yeet "httpz"
yeet "testz"

fr fr ===== HTTPZ SSL/TLS TESTS =====

test_start("TLS Context Creation")

sus tls_context tea = create_tls_context("server.crt", "server.key")
assert_true(contains_substring(tls_context, "TLS_CONTEXT"))
assert_true(contains_substring(tls_context, "server.crt"))
assert_true(contains_substring(tls_context, "server.key"))

test_start("SSL Certificate Verification")

assert_true(verify_ssl_certificate("example.com", "Subject: CN=example.com"))
assert_true(verify_ssl_certificate("sub.domain.com", "Subject: CN=*.domain.com"))
assert_false(verify_ssl_certificate("badsite.com", "Subject: CN=example.com"))

test_start("HTTPS GET Requests")

sus https_response tea = https_get("https://api.secure.com/data")
assert_true(contains_substring(https_response, "200 OK"))
assert_true(contains_substring(https_response, "Strict-Transport-Security"))
assert_true(contains_substring(https_response, "secure"))

sus bank_response tea = https_get("https://bank.example.com/balance")
assert_true(contains_substring(bank_response, "X-Frame-Options: DENY"))
assert_true(contains_substring(bank_response, "Content-Security-Policy"))

test_start("HTTPS POST Requests")

sus post_response tea = https_post("https://api.secure.com/users", "{\"name\":\"test\"}")
assert_true(contains_substring(post_response, "201 Created"))
assert_true(contains_substring(post_response, "secure_123"))

sus payment_response tea = https_post("https://payment.gateway.com/charge", "{\"amount\":100}")
assert_true(contains_substring(payment_response, "transaction_id"))
assert_true(contains_substring(payment_response, "approved"))

test_start("Security Headers")

sus security_headers tea = create_secure_headers()
assert_true(contains_substring(security_headers, "Strict-Transport-Security"))
assert_true(contains_substring(security_headers, "X-Content-Type-Options"))
assert_true(contains_substring(security_headers, "X-Frame-Options"))
assert_true(contains_substring(security_headers, "X-XSS-Protection"))
assert_true(contains_substring(security_headers, "Content-Security-Policy"))

test_start("Secure Response Creation")

sus secure_response tea = create_secure_response(200, "{\"message\":\"hello\"}")
assert_true(contains_substring(secure_response, "200 OK"))
assert_true(contains_substring(secure_response, "Strict-Transport-Security"))
assert_true(contains_substring(secure_response, "X-Frame-Options"))

test_start("TLS Version Validation")

assert_true(validate_tls_version("TLSv1.3"))
assert_true(validate_tls_version("TLSv1.2"))
assert_false(validate_tls_version("TLSv1.1"))
assert_false(validate_tls_version("TLSv1.0"))
assert_false(validate_tls_version("SSLv3"))
assert_false(validate_tls_version("SSLv2"))

test_start("SSL Certificate Generation")

sus cert tea = generate_ssl_certificate("example.com", 365)
assert_true(contains_substring(cert, "BEGIN CERTIFICATE"))
assert_true(contains_substring(cert, "END CERTIFICATE"))
assert_true(contains_substring(cert, "example.com"))
assert_true(contains_substring(cert, "365"))

test_start("SSL Fingerprint Extraction")

sus fingerprint tea = extract_ssl_fingerprint(cert)
assert_true(contains_substring(fingerprint, "SHA256:"))
assert_false(fingerprint == "INVALID_CERTIFICATE")

sus invalid_fingerprint tea = extract_ssl_fingerprint("invalid data")
assert_eq_string(invalid_fingerprint, "INVALID_CERTIFICATE")

test_start("HTTPS Fallback to HTTP")

fr fr Test that HTTPS functions fall back to HTTP for non-HTTPS URLs
sus http_response tea = https_get("http://example.com")
assert_true(contains_substring(http_response, "200 OK"))

sus http_post_response tea = https_post("http://example.com", "data")
assert_true(contains_substring(http_post_response, "200 OK"))

print_test_summary()
