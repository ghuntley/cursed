fr fr Comprehensive Network Protocols Testing
fr fr Tests all major protocol implementations including TLS, SSH, FTP, SMTP, HTTP, DNS

yeet "testz"
yeet "net_protocols"
yeet "vibez"

slay main() {
    vibez.spill("🌐 Starting comprehensive network protocols test suite")
    test_start("Network Protocols Implementation")
    
    fr fr Initialize network protocols
    net_protocols_initialize()
    
    fr fr Test TLS implementation
    test_tls_protocol()
    
    fr fr Test SSH implementation
    test_ssh_protocol()
    
    fr fr Test FTP implementation
    test_ftp_protocol()
    
    fr fr Test SMTP implementation  
    test_smtp_protocol()
    
    fr fr Test HTTP implementation
    test_http_protocol()
    
    fr fr Test DNS implementation
    test_dns_protocol()
    
    fr fr Test utility functions
    test_utility_functions()
    
    print_test_summary()
}

slay test_tls_protocol() {
    vibez.spill("🔒 Testing TLS protocol implementation")
    
    fr fr Test connection initialization
    tls_init_connection()
    assert_true(tls_connection_state == 1)
    
    fr fr Test Client Hello generation
    sus client_hello tea = tls_create_client_hello()
    assert_true(string_length(client_hello) > 100)
    assert_true(string_contains(client_hello, char(22))) fr fr Handshake record type
    
    fr fr Test master secret generation
    sus pre_master tea = "test_pre_master_secret_48_bytes_exactly_for_tls"
    tls_generate_master_secret(pre_master)
    assert_true(tls_connection_state == 2)
    
    fr fr Test key derivation
    (sus client_key tea, sus server_key tea, sus client_iv tea, sus server_iv tea) = tls_derive_keys()
    assert_true(string_length(client_key) >= 16)
    assert_true(string_length(server_key) >= 16)
    
    fr fr Test encryption/decryption
    sus plaintext tea = "Hello TLS World!"
    sus encrypted tea = tls_encrypt_application_data(plaintext, client_key, client_iv)
    sus decrypted tea = tls_decrypt_application_data(encrypted, client_key, client_iv)
    assert_eq_string(plaintext, decrypted)
    
    vibez.spill("✅ TLS protocol tests passed")
}

slay test_ssh_protocol() {
    vibez.spill("🔐 Testing SSH protocol implementation")
    
    fr fr Test connection initialization
    ssh_init_connection()
    assert_true(ssh_connection_state == 1)
    
    fr fr Test version exchange
    sus version_msg tea = ssh_create_version_exchange()
    assert_true(string_contains(version_msg, "SSH-2.0"))
    assert_true(string_ends_with(version_msg, "\r\n"))
    
    fr fr Test server version parsing
    assert_true(ssh_parse_server_version("SSH-2.0-OpenSSH_8.0\r\n"))
    assert_false(ssh_parse_server_version("SSH-1.5-OldServer\r\n"))
    
    fr fr Test KEX init message
    sus kex_msg tea = ssh_create_kex_init()
    assert_true(string_length(kex_msg) > 50)
    assert_true(string_contains(kex_msg, "diffie-hellman"))
    
    fr fr Test DH key exchange
    sus dh_msg tea = ssh_perform_dh_key_exchange()
    assert_true(string_length(dh_msg) > 10)
    
    fr fr Test password authentication
    sus auth_msg tea = ssh_authenticate_password("testuser", "testpass")
    assert_true(string_length(auth_msg) > 20)
    
    vibez.spill("✅ SSH protocol tests passed")
}

slay test_ftp_protocol() {
    vibez.spill("📁 Testing FTP protocol implementation")
    
    fr fr Test connection establishment
    sus welcome tea = ftp_connect()
    assert_true(string_contains(welcome, "220"))
    assert_true(ftp_connection_state == 1)
    
    fr fr Test authentication
    sus auth_response tea = ftp_authenticate("testuser", "testpass")
    assert_true(string_contains(auth_response, "230"))
    
    fr fr Test commands
    sus pwd_response tea = ftp_handle_command("PWD")
    assert_true(string_contains(pwd_response, "257"))
    
    sus list_response tea = ftp_handle_command("LIST")
    assert_true(string_contains(list_response, "150"))
    
    sus pasv_response tea = ftp_handle_command("PASV")
    assert_true(string_contains(pasv_response, "227"))
    
    fr fr Test FTPS commands
    sus auth_tls_response tea = ftp_handle_command("AUTH TLS")
    assert_true(string_contains(auth_tls_response, "234"))
    
    vibez.spill("✅ FTP protocol tests passed")
}

slay test_smtp_protocol() {
    vibez.spill("📧 Testing SMTP protocol implementation")
    
    fr fr Test connection establishment
    sus greeting tea = smtp_connect()
    assert_true(string_contains(greeting, "220"))
    assert_true(smtp_connection_state == 1)
    
    fr fr Test EHLO command
    sus ehlo_response tea = smtp_handle_command("EHLO client.example.com")
    assert_true(string_contains(ehlo_response, "250"))
    assert_true(string_contains(ehlo_response, "8BITMIME"))
    
    fr fr Test mail transaction
    sus mail_response tea = smtp_handle_command("MAIL FROM:<test@example.com>")
    assert_true(string_contains(mail_response, "250"))
    
    sus rcpt_response tea = smtp_handle_command("RCPT TO:<dest@example.com>")
    assert_true(string_contains(rcpt_response, "250"))
    
    sus data_response tea = smtp_handle_command("DATA")
    assert_true(string_contains(data_response, "354"))
    
    fr fr Test message data processing
    sus end_response tea = smtp_process_message_data(".\r\n")
    assert_true(string_contains(end_response, "250"))
    assert_true(string_contains(end_response, "cursed-"))
    
    fr fr Test STARTTLS
    sus starttls_response tea = smtp_handle_command("STARTTLS")
    assert_true(string_contains(starttls_response, "220"))
    
    vibez.spill("✅ SMTP protocol tests passed")
}

slay test_http_protocol() {
    vibez.spill("🌐 Testing HTTP protocol implementation")
    
    fr fr Test request creation
    sus get_request tea = http_create_request("GET", "http://example.com/test", "", "")
    assert_true(string_contains(get_request, "GET /test HTTP/1.1"))
    assert_true(string_contains(get_request, "Host: example.com"))
    
    sus post_request tea = http_create_request("POST", "http://api.example.com/data", "Content-Type: application/json\r\n", "{\"test\": true}")
    assert_true(string_contains(post_request, "POST /data HTTP/1.1"))
    assert_true(string_contains(post_request, "Content-Length:"))
    
    fr fr Test response parsing
    sus mock_response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello World!\n"
    (sus status normie, sus headers tea, sus body tea) = http_parse_response(mock_response)
    assert_eq_int(status, 200)
    assert_true(string_contains(headers, "text/html"))
    assert_eq_string(body, "Hello World!\n")
    
    fr fr Test status text lookup
    assert_eq_string(http_status_text(200), "OK")
    assert_eq_string(http_status_text(404), "Not Found")
    assert_eq_string(http_status_text(500), "Internal Server Error")
    
    fr fr Test URL encoding/decoding
    sus encoded tea = http_url_encode("Hello World! @#$")
    assert_true(string_contains(encoded, "+"))
    assert_true(string_contains(encoded, "%"))
    
    sus decoded tea = http_url_decode(encoded)
    assert_eq_string(decoded, "Hello World! @#$")
    
    vibez.spill("✅ HTTP protocol tests passed")
}

slay test_dns_protocol() {
    vibez.spill("📋 Testing DNS protocol implementation")
    
    fr fr Test DNS query creation
    sus query tea = dns_create_query("example.com", dns_query_type_a)
    assert_true(string_length(query) > 12) fr fr At least header + question
    
    fr fr Test DNS response simulation
    sus response tea = dns_simulate_response(query)
    assert_true(string_length(response) > string_length(query))
    
    fr fr Test DNS response parsing
    (sus success lit, sus ip tea) = dns_parse_response(response)
    assert_true(success)
    assert_true(string_contains(ip, "."))
    assert_true(string_contains(ip, "203.0.113."))
    
    fr fr Test DNS resolution
    sus resolved_ip tea = dns_resolve("test.example.com")
    assert_true(string_length(resolved_ip) > 7)
    
    vibez.spill("✅ DNS protocol tests passed")
}

slay test_utility_functions() {
    vibez.spill("🔧 Testing utility functions")
    
    fr fr Test string utilities
    assert_eq_string(string_to_upper("hello"), "HELLO")
    assert_eq_int(string_index_of("hello world", "world"), 6)
    assert_eq_int(string_index_of("hello", "xyz"), -1)
    
    assert_true(string_contains("hello world", "world"))
    assert_false(string_contains("hello", "world"))
    
    assert_true(string_ends_with("hello.txt", ".txt"))
    assert_false(string_ends_with("hello", ".txt"))
    
    fr fr Test number conversion
    assert_eq_string(string(42), "42")
    assert_eq_string(string(-17), "-17")
    assert_eq_string(string(0), "0")
    
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    assert_eq_int(string_to_int("0"), 0)
    
    fr fr Test base64 encoding
    sus data tea = "Hello"
    sus encoded tea = base64_encode(data)
    assert_true(string_length(encoded) > string_length(data))
    
    fr fr Test environment variables
    sus env_val tea = get_env_with_default("TEST_VAR", "default_value")
    assert_eq_string(env_val, "default_value")
    
    vibez.spill("✅ Utility function tests passed")
}

main()
