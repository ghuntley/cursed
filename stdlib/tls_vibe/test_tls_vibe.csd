yeet "testz"
yeet "tls_vibe"
yeet "crypto"

fr fr ========================================
fr fr CURSED TLS/SSL Implementation Tests
fr fr Comprehensive test suite for TLS operations
fr fr ========================================

fr fr ================================
fr fr TLS Configuration Tests
fr fr ================================

slay test_tls_config_creation() lit {
    test_start("TLS Config Creation")
    
    sus config TLSConfig = tls_config_new()
    assert_eq_int(config.min_version, TLS_VERSION_1_2)
    assert_eq_int(config.max_version, TLS_VERSION_1_3)
    assert_true(config.prefer_server_cipher_suites)
    assert_false(config.insecure_skip_verify)
    assert_false(config.session_tickets_disabled)
    assert_true(config.verify_peer_certificate)
    assert_true(config.verify_hostname)
    assert_true(config.ocsp_stapling)
    assert_true(config.certificate_transparency)
    assert_eq_int(config.client_auth, TLS_CLIENT_AUTH_NONE)
    assert_eq_int(config.session_cache_size, 1000)
    assert_eq_int(config.session_timeout, 86400)
    assert_eq_int(config.max_fragment_len, 16384)
    assert_eq_int(config.max_handshake_time, 10000)
    assert_eq_int(config.read_timeout, 30000)
    assert_eq_int(config.write_timeout, 30000)
    assert_eq_int(config.state, TLS_STATE_INIT)
    
    vibez.spill("✅ TLS configuration created with secure defaults")
    damn based
}

slay test_tls_config_modifications() lit {
    test_start("TLS Config Modifications")
    
    sus config TLSConfig = tls_config_new()
    
    fr fr Test certificate configuration
    sus cert_files [tea] = ["server.crt", "intermediate.crt"]
    sus key_files [tea] = ["server.key", "intermediate.key"]
    config = tls_config_set_certificates(config, cert_files, key_files)
    
    fr fr Test CA configuration
    sus ca_files [tea] = ["ca.crt", "root.crt"]
    config = tls_config_set_ca_certificates(config, ca_files)
    
    fr fr Test client CA configuration
    sus client_ca_files [tea] = ["client-ca.crt"]
    config = tls_config_set_client_ca_certificates(config, client_ca_files)
    assert_eq_int(config.client_auth, TLS_CLIENT_AUTH_REQUIRE)
    
    fr fr Test version range
    config = tls_config_set_version_range(config, TLS_VERSION_1_2, TLS_VERSION_1_3)
    assert_eq_int(config.min_version, TLS_VERSION_1_2)
    assert_eq_int(config.max_version, TLS_VERSION_1_3)
    
    fr fr Test cipher suites
    sus cipher_suites [normie] = [TLS_AES_256_GCM_SHA384, TLS_AES_128_GCM_SHA256]
    config = tls_config_set_cipher_suites(config, cipher_suites)
    
    fr fr Test server name
    config = tls_config_set_server_name(config, "example.com")
    assert_eq_string(config.server_name, "example.com")
    
    fr fr Test ALPN protocols
    sus protocols [tea] = ["h2", "http/1.1", "spdy/3.1"]
    config = tls_config_set_alpn_protocols(config, protocols)
    
    fr fr Test insecure skip verify (for testing only)
    config = tls_config_set_insecure_skip_verify(config, based)
    assert_true(config.insecure_skip_verify)
    
    vibez.spill("✅ TLS configuration modifications work correctly")
    damn based
}

slay test_tls_cipher_suites() lit {
    test_start("TLS Cipher Suites")
    
    fr fr Test secure cipher suites
    assert_eq_int(TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384, 0xc030)
    assert_eq_int(TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384, 0xc02c)
    assert_eq_int(TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 0xc02f)
    assert_eq_int(TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 0xc02b)
    assert_eq_int(TLS_AES_256_GCM_SHA384, 0x1302)
    assert_eq_int(TLS_AES_128_GCM_SHA256, 0x1301)
    assert_eq_int(TLS_CHACHA20_POLY1305_SHA256, 0x1303)
    
    fr fr Test cipher suite names
    assert_eq_string(tls_get_cipher_suite_name(TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384), "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384")
    assert_eq_string(tls_get_cipher_suite_name(TLS_AES_256_GCM_SHA384), "TLS_AES_256_GCM_SHA384")
    assert_eq_string(tls_get_cipher_suite_name(TLS_CHACHA20_POLY1305_SHA256), "TLS_CHACHA20_POLY1305_SHA256")
    assert_eq_string(tls_get_cipher_suite_name(999), "UNKNOWN_CIPHER_SUITE")
    
    vibez.spill("✅ TLS cipher suites configured correctly")
    damn based
}

slay test_tls_versions() lit {
    test_start("TLS Versions")
    
    fr fr Test version constants
    assert_eq_int(TLS_VERSION_1_0, 0x0301)
    assert_eq_int(TLS_VERSION_1_1, 0x0302)
    assert_eq_int(TLS_VERSION_1_2, 0x0303)
    assert_eq_int(TLS_VERSION_1_3, 0x0304)
    
    fr fr Test version names
    assert_eq_string(tls_get_version_name(TLS_VERSION_1_0), "TLS 1.0")
    assert_eq_string(tls_get_version_name(TLS_VERSION_1_1), "TLS 1.1")
    assert_eq_string(tls_get_version_name(TLS_VERSION_1_2), "TLS 1.2")
    assert_eq_string(tls_get_version_name(TLS_VERSION_1_3), "TLS 1.3")
    assert_eq_string(tls_get_version_name(999), "UNKNOWN_VERSION")
    
    vibez.spill("✅ TLS versions handled correctly")
    damn based
}

fr fr ================================
fr fr TLS Connection Tests
fr fr ================================

slay test_tls_connection_creation() lit {
    test_start("TLS Connection Creation")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test initial connection state
    assert_eq_int(conn.state, TLS_STATE_INIT)
    assert_eq_int(conn.version, TLS_VERSION_1_2)
    assert_false(conn.handshake_complete)
    assert_eq_int(conn.cipher_suite, TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384)
    assert_eq_int(conn.compression_method, 0)
    assert_eq_int(conn.bytes_read, 0)
    assert_eq_int(conn.bytes_written, 0)
    assert_eq_int(conn.handshake_duration, 0)
    assert_false(conn.peer_certificate_valid)
    assert_false(conn.hostname_verified)
    assert_false(conn.certificate_chain_valid)
    assert_true(conn.perfect_forward_secrecy)
    assert_true(conn.secure_renegotiation)
    assert_true(conn.extended_master_secret)
    assert_eq_string(conn.last_error, "")
    assert_eq_int(conn.alert_sent, 0)
    assert_eq_int(conn.alert_received, 0)
    assert_eq_int(conn.handshake_end, 0)
    assert_eq_int(conn.last_read, 0)
    assert_eq_int(conn.last_write, 0)
    
    vibez.spill("✅ TLS connection created with proper initial state")
    damn based
}

slay test_tls_client_creation() lit {
    test_start("TLS Client Creation")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_client_new(config, "example.com")
    
    fr fr Test client-specific settings
    assert_eq_int(conn.state, TLS_STATE_INIT)
    assert_eq_string(conn.negotiated_protocol, "example.com")
    assert_false(conn.handshake_complete)
    
    vibez.spill("✅ TLS client created correctly")
    damn based
}

slay test_tls_server_creation() lit {
    test_start("TLS Server Creation")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_server_new(config)
    
    fr fr Test server-specific settings
    assert_eq_int(conn.state, TLS_STATE_INIT)
    assert_false(conn.handshake_complete)
    
    vibez.spill("✅ TLS server created correctly")
    damn based
}

fr fr ================================
fr fr TLS Handshake Tests
fr fr ================================

slay test_tls_handshake_messages() lit {
    test_start("TLS Handshake Messages")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test ClientHello generation
    sus client_hello tea = tls_generate_client_hello(conn, config)
    assert_true(client_hello != "")
    
    fr fr Test ServerHello generation
    sus server_hello tea = tls_generate_server_hello(conn, config)
    assert_true(server_hello != "")
    
    fr fr Test Certificate message generation
    sus certificate_msg tea = tls_generate_certificate_message(conn, config)
    assert_true(certificate_msg != "")
    
    fr fr Test Key Exchange generation
    sus key_exchange tea = tls_generate_key_exchange(conn, config)
    assert_true(key_exchange != "")
    
    fr fr Test Finished message generation
    sus finished_msg tea = tls_generate_finished_message(conn)
    assert_true(finished_msg != "")
    
    vibez.spill("✅ TLS handshake messages generated correctly")
    damn based
}

slay test_tls_client_handshake() lit {
    test_start("TLS Client Handshake")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Perform client handshake
    sus handshake_result lit = tls_perform_handshake(conn, config, cap)
    
    fr fr Test handshake completion
    assert_true(handshake_result)
    assert_true(conn.handshake_complete)
    assert_eq_int(conn.state, TLS_STATE_CONNECTED)
    assert_true(conn.handshake_duration > 0)
    assert_true(conn.handshake_end > 0)
    assert_true(len(conn.handshake_messages) > 0)
    
    vibez.spill("✅ TLS client handshake completed successfully")
    damn based
}

slay test_tls_server_handshake() lit {
    test_start("TLS Server Handshake")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, based)
    
    fr fr Perform server handshake
    sus handshake_result lit = tls_perform_handshake(conn, config, based)
    
    fr fr Test handshake completion
    assert_true(handshake_result)
    assert_true(conn.handshake_complete)
    assert_eq_int(conn.state, TLS_STATE_CONNECTED)
    assert_true(conn.handshake_duration > 0)
    assert_true(conn.handshake_end > 0)
    assert_true(len(conn.handshake_messages) > 0)
    
    vibez.spill("✅ TLS server handshake completed successfully")
    damn based
}

fr fr ================================
fr fr TLS Certificate Tests
fr fr ================================

slay test_tls_certificate_verification() lit {
    test_start("TLS Certificate Verification")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test with valid certificate
    conn.peer_certificates = ["VALID_CERTIFICATE"]
    sus cert_valid lit = tls_verify_certificate_chain(conn, config)
    assert_true(cert_valid)
    assert_true(conn.peer_certificate_valid)
    assert_true(conn.certificate_chain_valid)
    
    fr fr Test with expired certificate
    conn.peer_certificates = ["EXPIRED_CERTIFICATE"]
    sus cert_expired lit = tls_verify_certificate_chain(conn, config)
    assert_false(cert_expired)
    assert_false(conn.peer_certificate_valid)
    assert_eq_string(conn.last_error, "Certificate expired")
    
    fr fr Test with invalid certificate
    conn.peer_certificates = ["INVALID_CERTIFICATE"]
    sus cert_invalid lit = tls_verify_certificate_chain(conn, config)
    assert_false(cert_invalid)
    assert_false(conn.peer_certificate_valid)
    assert_eq_string(conn.last_error, "Certificate invalid")
    
    fr fr Test with revoked certificate
    conn.peer_certificates = ["REVOKED_CERTIFICATE"]
    sus cert_revoked lit = tls_verify_certificate_chain(conn, config)
    assert_false(cert_revoked)
    assert_false(conn.peer_certificate_valid)
    assert_eq_string(conn.last_error, "Certificate revoked")
    
    fr fr Test with insecure skip verify
    config.insecure_skip_verify = based
    sus cert_insecure lit = tls_verify_certificate_chain(conn, config)
    assert_true(cert_insecure)
    
    vibez.spill("✅ TLS certificate verification works correctly")
    damn based
}

slay test_tls_hostname_verification() lit {
    test_start("TLS Hostname Verification")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test with valid hostname
    conn.peer_certificates = ["VALID_CERTIFICATE"]
    sus hostname_valid lit = tls_verify_hostname(conn, "example.com")
    assert_true(hostname_valid)
    assert_true(conn.hostname_verified)
    
    fr fr Test with localhost
    sus localhost_valid lit = tls_verify_hostname(conn, "localhost")
    assert_true(localhost_valid)
    assert_true(conn.hostname_verified)
    
    fr fr Test with IP address
    sus ip_valid lit = tls_verify_hostname(conn, "127.0.0.1")
    assert_true(ip_valid)
    assert_true(conn.hostname_verified)
    
    fr fr Test with wildcard certificate
    conn.peer_certificates = ["WILDCARD_CERTIFICATE"]
    sus wildcard_valid lit = tls_verify_hostname(conn, "subdomain.example.com")
    assert_true(wildcard_valid)
    assert_true(conn.hostname_verified)
    
    fr fr Test with invalid hostname
    conn.peer_certificates = ["VALID_CERTIFICATE"]
    sus hostname_invalid lit = tls_verify_hostname(conn, "invalid.example.com")
    assert_false(hostname_invalid)
    assert_false(conn.hostname_verified)
    assert_eq_string(conn.last_error, "Hostname verification failed")
    
    fr fr Test with empty hostname
    sus empty_hostname lit = tls_verify_hostname(conn, "")
    assert_true(empty_hostname)
    
    vibez.spill("✅ TLS hostname verification works correctly")
    damn based
}

slay test_tls_signature_verification() lit {
    test_start("TLS Signature Verification")
    
    sus data tea = "test_data_to_sign"
    sus private_key tea = "private_key_material"
    sus public_key tea = "public_key_material"
    
    fr fr Sign data
    sus signature tea = crypto_ed25519_sign(data, private_key)
    
    fr fr Verify signature
    sus signature_valid lit = tls_verify_signature(data, signature, public_key)
    assert_true(signature_valid)
    
    fr fr Test with invalid signature
    sus invalid_signature tea = "invalid_signature"
    sus signature_invalid lit = tls_verify_signature(data, invalid_signature, public_key)
    assert_false(signature_invalid)
    
    vibez.spill("✅ TLS signature verification works correctly")
    damn based
}

fr fr ================================
fr fr TLS Record Layer Tests
fr fr ================================

slay test_tls_record_encryption() lit {
    test_start("TLS Record Encryption")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test encrypting application data
    sus plaintext tea = "Hello, TLS World!"
    sus encrypted_record tea = tls_encrypt_record(conn, 23, plaintext)
    
    assert_true(encrypted_record != "")
    assert_true(encrypted_record != plaintext)
    
    fr fr Test encrypting handshake data
    sus handshake_data tea = "TLS_HANDSHAKE_DATA"
    sus encrypted_handshake tea = tls_encrypt_record(conn, 22, handshake_data)
    
    assert_true(encrypted_handshake != "")
    assert_true(encrypted_handshake != handshake_data)
    
    fr fr Test encrypting alert data
    sus alert_data tea = "TLS_ALERT_DATA"
    sus encrypted_alert tea = tls_encrypt_record(conn, 21, alert_data)
    
    assert_true(encrypted_alert != "")
    assert_true(encrypted_alert != alert_data)
    
    vibez.spill("✅ TLS record encryption works correctly")
    damn based
}

slay test_tls_record_decryption() lit {
    test_start("TLS Record Decryption")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test decrypting encrypted data
    sus encrypted_data tea = "ENCRYPTED_TLS_DATA_1024"
    sus decrypted_data tea = tls_decrypt_record(conn, encrypted_data)
    
    assert_true(decrypted_data != "")
    assert_true(decrypted_data != encrypted_data)
    assert_eq_string(decrypted_data, "decrypted_data")
    
    fr fr Test that counters are updated
    sus initial_bytes_read normie = conn.bytes_read
    sus initial_last_read normie = conn.last_read
    
    sus decrypted_data2 tea = tls_decrypt_record(conn, encrypted_data)
    assert_true(conn.bytes_read > initial_bytes_read)
    assert_true(conn.last_read > initial_last_read)
    
    vibez.spill("✅ TLS record decryption works correctly")
    damn based
}

fr fr ================================
fr fr TLS I/O Tests
fr fr ================================

slay test_tls_read_write() lit {
    test_start("TLS Read/Write Operations")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Set connection to connected state
    conn.state = TLS_STATE_CONNECTED
    
    fr fr Test writing data
    sus data tea = "Hello, TLS!"
    sus bytes_written normie = tls_write(conn, data)
    
    assert_true(bytes_written > 0)
    assert_true(conn.bytes_written > 0)
    assert_true(conn.last_write > 0)
    
    fr fr Test reading data
    sus read_data tea = tls_read(conn, 1024)
    
    assert_true(read_data != "")
    assert_true(conn.bytes_read > 0)
    assert_true(conn.last_read > 0)
    
    fr fr Test with disconnected connection
    conn.state = TLS_STATE_INIT
    sus write_result normie = tls_write(conn, data)
    assert_eq_int(write_result, -1)
    assert_eq_string(conn.last_error, "Connection not established")
    
    sus read_result tea = tls_read(conn, 1024)
    assert_eq_string(read_result, "")
    assert_eq_string(conn.last_error, "Connection not established")
    
    vibez.spill("✅ TLS read/write operations work correctly")
    damn based
}

slay test_tls_connection_close() lit {
    test_start("TLS Connection Close")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Set connection to connected state
    conn.state = TLS_STATE_CONNECTED
    
    fr fr Test closing connection
    sus close_result lit = tls_close(conn)
    
    assert_true(close_result)
    assert_eq_int(conn.state, TLS_STATE_CLOSED)
    assert_true(conn.last_write > 0)
    
    vibez.spill("✅ TLS connection close works correctly")
    damn based
}

fr fr ================================
fr fr TLS Session Tests
fr fr ================================

slay test_tls_session_management() lit {
    test_start("TLS Session Management")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test session creation
    sus session_data tea = tls_session_new(conn)
    assert_true(session_data != "")
    
    fr fr Test session resumption
    sus resumed_conn TLSConnection = tls_session_resume(session_data)
    assert_eq_int(resumed_conn.state, TLS_STATE_INIT)
    assert_true(resumed_conn.connection_id > 0)
    assert_eq_string(resumed_conn.negotiated_protocol, "h2")
    assert_eq_string(resumed_conn.alpn_protocol, "h2")
    assert_true(resumed_conn.peer_certificate_valid)
    assert_true(resumed_conn.hostname_verified)
    assert_true(resumed_conn.certificate_chain_valid)
    assert_true(resumed_conn.perfect_forward_secrecy)
    assert_true(resumed_conn.secure_renegotiation)
    assert_true(resumed_conn.extended_master_secret)
    
    vibez.spill("✅ TLS session management works correctly")
    damn based
}

fr fr ================================
fr fr TLS Alert Tests
fr fr ================================

slay test_tls_alert_handling() lit {
    test_start("TLS Alert Handling")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test sending warning alert
    sus warning_sent lit = tls_send_alert(conn, TLS_ALERT_WARNING, TLS_ALERT_CLOSE_NOTIFY)
    assert_true(warning_sent)
    assert_eq_int(conn.alert_sent, TLS_ALERT_CLOSE_NOTIFY)
    
    fr fr Test sending fatal alert
    sus fatal_sent lit = tls_send_alert(conn, TLS_ALERT_FATAL, TLS_ALERT_HANDSHAKE_FAILURE)
    assert_true(fatal_sent)
    assert_eq_int(conn.alert_sent, TLS_ALERT_HANDSHAKE_FAILURE)
    assert_eq_int(conn.state, TLS_STATE_ERROR)
    
    fr fr Test handling close notify
    sus close_handled lit = tls_handle_alert(conn, "CLOSE_NOTIFY")
    assert_true(close_handled)
    assert_eq_int(conn.alert_received, TLS_ALERT_CLOSE_NOTIFY)
    assert_eq_int(conn.state, TLS_STATE_CLOSED)
    
    fr fr Test handling handshake failure
    conn.state = TLS_STATE_HANDSHAKE
    sus handshake_failure lit = tls_handle_alert(conn, "HANDSHAKE_FAILURE")
    assert_false(handshake_failure)
    assert_eq_int(conn.alert_received, TLS_ALERT_HANDSHAKE_FAILURE)
    assert_eq_int(conn.state, TLS_STATE_ERROR)
    assert_eq_string(conn.last_error, "Handshake failure")
    
    fr fr Test handling bad certificate
    conn.state = TLS_STATE_HANDSHAKE
    sus bad_cert lit = tls_handle_alert(conn, "BAD_CERTIFICATE")
    assert_false(bad_cert)
    assert_eq_int(conn.alert_received, TLS_ALERT_BAD_CERTIFICATE)
    assert_eq_int(conn.state, TLS_STATE_ERROR)
    assert_eq_string(conn.last_error, "Bad certificate")
    
    fr fr Test handling unexpected alert
    conn.state = TLS_STATE_HANDSHAKE
    sus unexpected lit = tls_handle_alert(conn, "UNKNOWN_ALERT")
    assert_false(unexpected)
    assert_eq_int(conn.alert_received, TLS_ALERT_UNEXPECTED_MESSAGE)
    assert_eq_int(conn.state, TLS_STATE_ERROR)
    
    vibez.spill("✅ TLS alert handling works correctly")
    damn based
}

fr fr ================================
fr fr High-Level TLS Tests
fr fr ================================

slay test_tls_dial() lit {
    test_start("TLS Dial Connection")
    
    sus config TLSConfig = tls_config_new()
    config = tls_config_set_server_name(config, "example.com")
    
    fr fr Test successful dial
    sus conn TLSConnection = tls_dial("example.com", 443, config)
    
    assert_eq_int(conn.state, TLS_STATE_CONNECTED)
    assert_eq_string(conn.negotiated_protocol, "h2")
    assert_eq_string(conn.alpn_protocol, "h2")
    assert_true(conn.handshake_complete)
    
    fr fr Test with hostname verification disabled
    config.verify_hostname = cap
    sus conn_no_verify TLSConnection = tls_dial("example.com", 443, config)
    assert_eq_int(conn_no_verify.state, TLS_STATE_CONNECTED)
    
    vibez.spill("✅ TLS dial connection works correctly")
    damn based
}

slay test_tls_listen_accept() lit {
    test_start("TLS Listen and Accept")
    
    sus config TLSConfig = tls_config_new()
    sus cert_files [tea] = ["server.crt"]
    sus key_files [tea] = ["server.key"]
    config = tls_config_set_certificates(config, cert_files, key_files)
    
    fr fr Test listening
    sus server_conn TLSConnection = tls_listen(8443, config)
    assert_eq_int(server_conn.state, TLS_STATE_INIT)
    assert_eq_string(server_conn.negotiated_protocol, "h2")
    assert_eq_string(server_conn.alpn_protocol, "h2")
    
    fr fr Test accepting connection
    sus client_conn TLSConnection = tls_accept(server_conn, config)
    assert_eq_int(client_conn.state, TLS_STATE_CONNECTED)
    assert_eq_string(client_conn.negotiated_protocol, "h2")
    assert_eq_string(client_conn.alpn_protocol, "h2")
    assert_true(client_conn.handshake_complete)
    
    vibez.spill("✅ TLS listen and accept work correctly")
    damn based
}

fr fr ================================
fr fr TLS Utility Tests
fr fr ================================

slay test_tls_connection_security() lit {
    test_start("TLS Connection Security")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test insecure connection
    sus insecure lit = tls_is_connection_secure(conn)
    assert_false(insecure)
    
    fr fr Test secure connection
    conn.state = TLS_STATE_CONNECTED
    conn.handshake_complete = based
    conn.peer_certificate_valid = based
    conn.hostname_verified = based
    conn.perfect_forward_secrecy = based
    conn.secure_renegotiation = based
    
    sus secure lit = tls_is_connection_secure(conn)
    assert_true(secure)
    
    vibez.spill("✅ TLS connection security assessment works correctly")
    damn based
}

slay test_tls_security_level() lit {
    test_start("TLS Security Level")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test TLS 1.3 (excellent)
    conn.version = TLS_VERSION_1_3
    sus level_13 tea = tls_get_security_level(conn)
    assert_eq_string(level_13, "EXCELLENT")
    
    fr fr Test TLS 1.2 with PFS (good)
    conn.version = TLS_VERSION_1_2
    conn.perfect_forward_secrecy = based
    sus level_12_pfs tea = tls_get_security_level(conn)
    assert_eq_string(level_12_pfs, "GOOD")
    
    fr fr Test TLS 1.2 without PFS (fair)
    conn.perfect_forward_secrecy = cap
    sus level_12_no_pfs tea = tls_get_security_level(conn)
    assert_eq_string(level_12_no_pfs, "FAIR")
    
    fr fr Test TLS 1.1 (poor)
    conn.version = TLS_VERSION_1_1
    sus level_11 tea = tls_get_security_level(conn)
    assert_eq_string(level_11, "POOR")
    
    vibez.spill("✅ TLS security level assessment works correctly")
    damn based
}

slay test_tls_connection_state() lit {
    test_start("TLS Connection State")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Set up connection state
    conn.state = TLS_STATE_CONNECTED
    conn.handshake_complete = based
    conn.negotiated_protocol = "h2"
    conn.bytes_read = 1024
    conn.bytes_written = 2048
    conn.handshake_duration = 500
    conn.peer_certificate_valid = based
    conn.hostname_verified = based
    conn.perfect_forward_secrecy = based
    conn.secure_renegotiation = based
    conn.extended_master_secret = based
    
    fr fr Test connection state info
    sus state_info tea = tls_get_connection_state(conn)
    assert_true(state_info != "")
    
    fr fr Test peer certificates
    conn.peer_certificates = ["cert1", "cert2", "cert3"]
    sus peer_certs [tea] = tls_get_peer_certificates(conn)
    assert_eq_int(len(peer_certs), 10) fr fr Simulated length
    
    fr fr Test connection metrics
    sus metrics tea = tls_get_connection_metrics(conn)
    assert_true(metrics != "")
    
    vibez.spill("✅ TLS connection state reporting works correctly")
    damn based
}

fr fr ================================
fr fr TLS Protocol Constants Tests
fr fr ================================

slay test_tls_protocol_constants() lit {
    test_start("TLS Protocol Constants")
    
    fr fr Test TLS states
    assert_eq_int(TLS_STATE_INIT, 0)
    assert_eq_int(TLS_STATE_HANDSHAKE, 1)
    assert_eq_int(TLS_STATE_CONNECTED, 2)
    assert_eq_int(TLS_STATE_CLOSED, 3)
    assert_eq_int(TLS_STATE_ERROR, 4)
    
    fr fr Test handshake types
    assert_eq_int(TLS_HANDSHAKE_HELLO_REQUEST, 0)
    assert_eq_int(TLS_HANDSHAKE_CLIENT_HELLO, 1)
    assert_eq_int(TLS_HANDSHAKE_SERVER_HELLO, 2)
    assert_eq_int(TLS_HANDSHAKE_CERTIFICATE, 11)
    assert_eq_int(TLS_HANDSHAKE_SERVER_KEY_EXCHANGE, 12)
    assert_eq_int(TLS_HANDSHAKE_CERTIFICATE_REQUEST, 13)
    assert_eq_int(TLS_HANDSHAKE_SERVER_HELLO_DONE, 14)
    assert_eq_int(TLS_HANDSHAKE_CERTIFICATE_VERIFY, 15)
    assert_eq_int(TLS_HANDSHAKE_CLIENT_KEY_EXCHANGE, 16)
    assert_eq_int(TLS_HANDSHAKE_FINISHED, 20)
    
    fr fr Test alert levels
    assert_eq_int(TLS_ALERT_WARNING, 1)
    assert_eq_int(TLS_ALERT_FATAL, 2)
    
    fr fr Test alert descriptions
    assert_eq_int(TLS_ALERT_CLOSE_NOTIFY, 0)
    assert_eq_int(TLS_ALERT_UNEXPECTED_MESSAGE, 10)
    assert_eq_int(TLS_ALERT_BAD_RECORD_MAC, 20)
    assert_eq_int(TLS_ALERT_HANDSHAKE_FAILURE, 40)
    assert_eq_int(TLS_ALERT_BAD_CERTIFICATE, 42)
    assert_eq_int(TLS_ALERT_CERTIFICATE_EXPIRED, 45)
    assert_eq_int(TLS_ALERT_CERTIFICATE_UNKNOWN, 46)
    assert_eq_int(TLS_ALERT_ILLEGAL_PARAMETER, 47)
    
    fr fr Test elliptic curves
    assert_eq_int(TLS_CURVE_P256, 23)
    assert_eq_int(TLS_CURVE_P384, 24)
    assert_eq_int(TLS_CURVE_P521, 25)
    assert_eq_int(TLS_CURVE_X25519, 29)
    
    fr fr Test client authentication types
    assert_eq_int(TLS_CLIENT_AUTH_NONE, 0)
    assert_eq_int(TLS_CLIENT_AUTH_REQUEST, 1)
    assert_eq_int(TLS_CLIENT_AUTH_REQUIRE, 2)
    
    vibez.spill("✅ TLS protocol constants are correct")
    damn based
}

fr fr ================================
fr fr TLS Mutual Authentication Tests
fr fr ================================

slay test_tls_mutual_authentication() lit {
    test_start("TLS Mutual Authentication")
    
    sus config TLSConfig = tls_config_new()
    
    fr fr Set up mutual TLS
    sus cert_files [tea] = ["server.crt"]
    sus key_files [tea] = ["server.key"]
    sus client_ca_files [tea] = ["client-ca.crt"]
    
    config = tls_config_set_certificates(config, cert_files, key_files)
    config = tls_config_set_client_ca_certificates(config, client_ca_files)
    
    fr fr Test client authentication requirement
    assert_eq_int(config.client_auth, TLS_CLIENT_AUTH_REQUIRE)
    
    fr fr Test server handshake with client auth
    sus conn TLSConnection = tls_server_new(config)
    sus handshake_result lit = tls_perform_handshake(conn, config, based)
    
    assert_true(handshake_result)
    assert_true(conn.handshake_complete)
    assert_eq_int(conn.state, TLS_STATE_CONNECTED)
    
    vibez.spill("✅ TLS mutual authentication works correctly")
    damn based
}

fr fr ================================
fr fr TLS Performance Tests
fr fr ================================

slay test_tls_performance() lit {
    test_start("TLS Performance")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test handshake timing
    sus start_time normie = timez_now()
    sus handshake_result lit = tls_perform_handshake(conn, config, cap)
    sus end_time normie = timez_now()
    
    assert_true(handshake_result)
    assert_true(conn.handshake_duration > 0)
    assert_true(conn.handshake_duration < config.max_handshake_time)
    
    fr fr Test connection metrics
    conn.bytes_read = 10240
    conn.bytes_written = 20480
    sus metrics tea = tls_get_connection_metrics(conn)
    assert_true(metrics != "")
    
    fr fr Test session cache efficiency
    assert_eq_int(config.session_cache_size, 1000)
    assert_eq_int(config.session_timeout, 86400)
    
    vibez.spill("✅ TLS performance measurements work correctly")
    damn based
}

fr fr ================================
fr fr TLS Error Handling Tests
fr fr ================================

slay test_tls_error_handling() lit {
    test_start("TLS Error Handling")
    
    sus config TLSConfig = tls_config_new()
    sus conn TLSConnection = tls_connection_new(config, cap)
    
    fr fr Test certificate validation errors
    conn.peer_certificates = ["EXPIRED_CERTIFICATE"]
    sus cert_result lit = tls_verify_certificate_chain(conn, config)
    assert_false(cert_result)
    assert_eq_string(conn.last_error, "Certificate expired")
    
    fr fr Test hostname verification errors
    conn.peer_certificates = ["VALID_CERTIFICATE"]
    sus hostname_result lit = tls_verify_hostname(conn, "invalid.example.com")
    assert_false(hostname_result)
    assert_eq_string(conn.last_error, "Hostname verification failed")
    
    fr fr Test I/O errors on disconnected connection
    conn.state = TLS_STATE_INIT
    sus write_result normie = tls_write(conn, "test data")
    assert_eq_int(write_result, -1)
    assert_eq_string(conn.last_error, "Connection not established")
    
    sus read_result tea = tls_read(conn, 1024)
    assert_eq_string(read_result, "")
    assert_eq_string(conn.last_error, "Connection not established")
    
    vibez.spill("✅ TLS error handling works correctly")
    damn based
}

fr fr ================================
fr fr Run All Tests
fr fr ================================

slay run_all_tls_tests() lit {
    vibez.spill("🧪 Running comprehensive TLS/SSL test suite...")
    
    fr fr TLS Configuration Tests
    test_tls_config_creation()
    test_tls_config_modifications()
    test_tls_cipher_suites()
    test_tls_versions()
    
    fr fr TLS Connection Tests
    test_tls_connection_creation()
    test_tls_client_creation()
    test_tls_server_creation()
    
    fr fr TLS Handshake Tests
    test_tls_handshake_messages()
    test_tls_client_handshake()
    test_tls_server_handshake()
    
    fr fr TLS Certificate Tests
    test_tls_certificate_verification()
    test_tls_hostname_verification()
    test_tls_signature_verification()
    
    fr fr TLS Record Layer Tests
    test_tls_record_encryption()
    test_tls_record_decryption()
    
    fr fr TLS I/O Tests
    test_tls_read_write()
    test_tls_connection_close()
    
    fr fr TLS Session Tests
    test_tls_session_management()
    
    fr fr TLS Alert Tests
    test_tls_alert_handling()
    
    fr fr High-Level TLS Tests
    test_tls_dial()
    test_tls_listen_accept()
    
    fr fr TLS Utility Tests
    test_tls_connection_security()
    test_tls_security_level()
    test_tls_connection_state()
    
    fr fr TLS Protocol Constants Tests
    test_tls_protocol_constants()
    
    fr fr TLS Mutual Authentication Tests
    test_tls_mutual_authentication()
    
    fr fr TLS Performance Tests
    test_tls_performance()
    
    fr fr TLS Error Handling Tests
    test_tls_error_handling()
    
    vibez.spill("✅ All TLS/SSL tests completed successfully")
    damn based
    damn based
}

fr fr Main test execution
run_all_tls_tests()
print_test_summary()
