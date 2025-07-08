// SMTP Tea Module Tests
// Comprehensive test suite for email functionality

yeet "testz"
yeet "smtp_tea"

// Test SMTP connection
test_start("smtp_connect basic connection")
sus result lit = smtp_connect("smtp.gmail.com", 587)
assert_true(result)

test_start("smtp_connect with default port")
sus result2 lit = smtp_connect("mail.example.com", smtp_default_port)
assert_true(result2)

test_start("smtp_connect with TLS port")
sus result3 lit = smtp_connect("smtp.outlook.com", smtp_tls_port)
assert_true(result3)

// Test SMTP authentication
test_start("smtp_auth with valid credentials")
sus auth_result lit = smtp_auth("user@example.com", "password123")
assert_true(auth_result)

test_start("smtp_auth with empty username")
sus auth_result2 lit = smtp_auth("", "password")
assert_true(auth_result2)

// Test basic email sending
test_start("smtp_send_email basic")
sus send_result lit = smtp_send_email("sender@example.com", "recipient@example.com", "Test Subject", "Test Body")
assert_true(send_result)

test_start("smtp_send_email with long subject")
sus long_subject tea = "This is a very long email subject that should be handled correctly by the SMTP implementation"
sus send_result2 lit = smtp_send_email("test@example.com", "recipient@example.com", long_subject, "Short body")
assert_true(send_result2)

// Test full email sending
test_start("smtp_send_full_email with headers")
sus full_result lit = smtp_send_full_email("sender@example.com", "recipient@example.com", "cc@example.com", "bcc@example.com", "Full Test", "Full Body", "X-Priority: High")
assert_true(full_result)

// Test email validation
test_start("smtp_validate_email valid addresses")
assert_true(smtp_validate_email("user@example.com"))
assert_true(smtp_validate_email("test.email@domain.org"))
assert_true(smtp_validate_email("admin@company.co.uk"))

test_start("smtp_validate_email invalid addresses")
assert_false(smtp_validate_email("invalid.email"))
assert_false(smtp_validate_email("@example.com"))
assert_false(smtp_validate_email("user@"))

// Test message formatting
test_start("smtp_format_message basic")
sus formatted tea = smtp_format_message("from@example.com", "to@example.com", "Test", "Hello World")
assert_true(formatted.contains("From: from@example.com"))
assert_true(formatted.contains("To: to@example.com"))
assert_true(formatted.contains("Subject: Test"))

test_start("smtp_format_message with special characters")
sus formatted2 tea = smtp_format_message("test@example.com", "user@example.com", "Test: Special!", "Body with émojis 🚀")
assert_true(formatted2.contains("Test: Special!"))

// Test HTML email
test_start("smtp_send_html_email basic")
sus html_body tea = "<html><body><h1>Test</h1><p>HTML content</p></body></html>"
sus html_result lit = smtp_send_html_email("sender@example.com", "recipient@example.com", "HTML Test", html_body)
assert_true(html_result)

// Test email with attachments
test_start("smtp_send_with_attachments basic")
sus attachment_result lit = smtp_send_with_attachments("sender@example.com", "recipient@example.com", "With Attachments", "Body", "file1.txt,file2.pdf")
assert_true(attachment_result)

test_start("smtp_send_with_attachments multiple files")
sus attachments tea = "document.pdf,image.jpg,data.csv,report.xlsx"
sus multi_result lit = smtp_send_with_attachments("test@example.com", "user@example.com", "Multiple Files", "See attached", attachments)
assert_true(multi_result)

// Test SMTP configuration
test_start("smtp_set_timeout valid timeout")
sus timeout_result lit = smtp_set_timeout(30)
assert_true(timeout_result)

test_start("smtp_set_timeout zero timeout")
sus timeout_result2 lit = smtp_set_timeout(0)
assert_true(timeout_result2)

test_start("smtp_enable_tls")
sus tls_result lit = smtp_enable_tls()
assert_true(tls_result)

// Test SMTP status and connection
test_start("smtp_get_status")
sus status tea = smtp_get_status()
assert_eq_string(status, "SMTP server ready")

test_start("smtp_is_connected")
sus connected lit = smtp_is_connected()
assert_true(connected)

test_start("smtp_disconnect")
sus disconnect_result lit = smtp_disconnect()
assert_true(disconnect_result)

// Test bulk email sending
test_start("smtp_send_bulk_emails basic")
sus recipients tea = "user1@example.com,user2@example.com,user3@example.com"
sus bulk_count normie = smtp_send_bulk_emails("sender@example.com", recipients, "Bulk Test", "Bulk message")
assert_eq_int(bulk_count, 3)

test_start("smtp_send_bulk_emails single recipient")
sus single_recipient tea = "user@example.com"
sus single_count normie = smtp_send_bulk_emails("sender@example.com", single_recipient, "Single Test", "Single message")
assert_eq_int(single_count, 1)

// Test error handling
test_start("smtp_get_last_error")
sus error_msg tea = smtp_get_last_error()
assert_eq_string(error_msg, "No errors")

// Test debug functionality
test_start("smtp_set_debug enabled")
sus debug_result lit = smtp_set_debug(based)
assert_true(debug_result)

test_start("smtp_set_debug disabled")
sus debug_result2 lit = smtp_set_debug(cap)
assert_true(debug_result2)

// Test SMTP capabilities
test_start("smtp_get_capabilities")
sus capabilities tea = smtp_get_capabilities()
assert_true(capabilities.contains("EHLO"))
assert_true(capabilities.contains("STARTTLS"))
assert_true(capabilities.contains("AUTH"))

// Test base64 encoding/decoding
test_start("smtp_encode_base64 basic")
sus encoded tea = smtp_encode_base64("test_data")
assert_eq_string(encoded, "test_data_encoded")

test_start("smtp_decode_base64 basic")
sus decoded tea = smtp_decode_base64("test_data_encoded")
assert_eq_string(decoded, "test_data")

test_start("smtp_encode_decode_roundtrip")
sus original tea = "authentication_data"
sus encoded_rt tea = smtp_encode_base64(original)
sus decoded_rt tea = smtp_decode_base64(encoded_rt)
assert_eq_string(decoded_rt, original)

// Test header parsing
test_start("smtp_parse_headers basic")
sus headers tea = "X-Priority: High\r\nX-Mailer: CURSED"
sus parsed tea = smtp_parse_headers(headers)
assert_eq_string(parsed, headers)

print_test_summary()
