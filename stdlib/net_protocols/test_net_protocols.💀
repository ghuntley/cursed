yeet "testz"
yeet "net_protocols"

fr fr ================================
fr fr Network Protocols Module Tests
fr fr Comprehensive testing of TLS, SSH, FTP, SMTP
fr fr ================================

test_start("Net Protocols Initialization Test")
sus init_result lit = net_protocols_initialize()
assert_true(init_result)

test_start("TLS Connection Initialization Test")
sus tls_init lit = tls_init_connection()
assert_true(tls_init)
assert_eq_int(tls_connection_state, 0) fr fr Should be initialized

test_start("TLS Client Hello Creation Test")
tls_init_connection()
sus client_hello tea = tls_create_client_hello()
assert_true(string_length(client_hello) > 0)
assert_true(string_length(client_hello) > 50) fr fr Should be substantial message

test_start("TLS Extensions Building Test")
sus extensions tea = tls_build_extensions()
assert_true(string_length(extensions) > 0)
assert_true(string_contains(extensions, "www.example.com"))

test_start("TLS Master Secret Generation Test")
sus pre_master tea = "test_pre_master_secret_data"
sus master_result lit = tls_generate_master_secret(pre_master)
assert_true(master_result)
assert_true(tls_master_secret[0] != 0) fr fr Should have generated data

test_start("TLS Key Derivation Test")
tls_generate_master_secret("test_secret")
(client_key, server_key, client_iv, server_iv) := tls_derive_keys()
assert_true(string_length(client_key) > 0)
assert_true(string_length(server_key) > 0)
assert_true(string_length(client_iv) > 0)
assert_true(string_length(server_iv) > 0)

test_start("TLS Encryption/Decryption Test")
sus test_data tea = "Hello TLS World"
sus key tea = "test_encryption_key_32_bytes_long"
sus iv tea = "test_iv_16_bytes"
sus encrypted tea = tls_encrypt_application_data(test_data, key, iv)
assert_true(string_length(encrypted) > string_length(test_data))
sus decrypted tea = tls_decrypt_application_data(encrypted, key, iv)
assert_eq_string(decrypted, test_data)

test_start("SSH Connection Initialization Test")
sus ssh_init lit = ssh_init_connection()
assert_true(ssh_init)
assert_eq_int(ssh_connection_state, 0)

test_start("SSH Version Exchange Test")
ssh_init_connection()
sus version_msg tea = ssh_create_version_exchange()
assert_true(string_contains(version_msg, "SSH-2.0-CURSED_SSH"))
assert_true(string_ends_with(version_msg, "\r\n"))
assert_eq_int(ssh_connection_state, 1)

test_start("SSH Server Version Parsing Test")
sus server_version tea = "SSH-2.0-OpenSSH_8.0\r\n"
sus parse_result lit = ssh_parse_server_version(server_version)
assert_true(parse_result)
assert_eq_int(ssh_connection_state, 2)

test_start("SSH KEX Init Creation Test")
sus kex_init tea = ssh_create_kex_init()
assert_true(string_length(kex_init) > 0)
assert_true(string_length(ssh_client_kex_init) > 0)

test_start("SSH DH Key Exchange Test")
sus dh_message tea = ssh_perform_dh_key_exchange()
assert_true(string_length(dh_message) > 0)

test_start("SSH Password Authentication Test")
sus auth_msg tea = ssh_authenticate_password("testuser", "testpass")
assert_true(string_length(auth_msg) > 0)
assert_true(string_contains(auth_msg, "testuser"))

test_start("FTP Connection Test")
sus ftp_welcome tea = ftp_connect()
assert_true(string_contains(ftp_welcome, "220"))
assert_true(string_contains(ftp_welcome, "FTP Server Ready"))
assert_eq_int(ftp_connection_state, 1)

test_start("FTP Authentication Test")
sus auth_response tea = ftp_authenticate("anonymous", "guest")
assert_true(string_contains(auth_response, "230"))
assert_eq_int(ftp_connection_state, 2)

test_start("FTP Command Handling Test")
sus pwd_response tea = ftp_handle_command("PWD ")
assert_true(string_contains(pwd_response, "257"))
assert_true(string_contains(pwd_response, "/"))

sus cwd_response tea = ftp_handle_command("CWD /home")
assert_true(string_contains(cwd_response, "250"))
assert_eq_string(ftp_current_directory, "/home")

sus type_response tea = ftp_handle_command("TYPE I")
assert_true(string_contains(type_response, "200"))
assert_eq_string(ftp_transfer_mode, "BINARY")

test_start("FTP Directory Listing Test")
sus list_response tea = ftp_list_directory()
assert_true(string_contains(list_response, "150"))
assert_true(string_contains(list_response, "226"))
assert_true(string_contains(list_response, "file1.txt"))

test_start("FTP File Transfer Test")
sus retr_response tea = ftp_retrieve_file("test.txt")
assert_true(string_contains(retr_response, "150"))
assert_true(string_contains(retr_response, "226"))
assert_true(string_contains(retr_response, "test.txt"))

sus stor_response tea = ftp_store_file("upload.txt")
assert_true(string_contains(stor_response, "150"))
assert_true(string_contains(stor_response, "226"))

test_start("FTP Passive Mode Test")
sus pasv_response tea = ftp_enter_passive_mode()
assert_true(string_contains(pasv_response, "227"))
assert_true(string_contains(pasv_response, "192,168,1,100"))
assert_true(ftp_passive_mode)

test_start("SMTP Connection Test")
sus smtp_greeting tea = smtp_connect()
assert_true(string_contains(smtp_greeting, "220"))
assert_true(string_contains(smtp_greeting, "CURSED Mail Server"))
assert_eq_int(smtp_connection_state, 1)

test_start("SMTP HELO Command Test")
sus helo_response tea = smtp_handle_command("HELO client.example.com")
assert_true(string_contains(helo_response, "250"))
assert_eq_string(smtp_helo_domain, "client.example.com")
assert_eq_int(smtp_connection_state, 2)

test_start("SMTP EHLO Command Test")
sus ehlo_response tea = smtp_handle_command("EHLO client.example.com")
assert_true(string_contains(ehlo_response, "250-"))
assert_true(string_contains(ehlo_response, "8BITMIME"))
assert_true(string_contains(ehlo_response, "AUTH"))

test_start("SMTP Mail Transaction Test")
smtp_handle_command("HELO client.example.com")
sus mail_response tea = smtp_handle_command("MAIL FROM:<sender@example.com>")
assert_true(string_contains(mail_response, "250"))
assert_eq_string(smtp_mail_from, "sender@example.com")

sus rcpt_response tea = smtp_handle_command("RCPT TO:<recipient@example.com>")
assert_true(string_contains(rcpt_response, "250"))

sus data_response tea = smtp_handle_command("DATA")
assert_true(string_contains(data_response, "354"))

test_start("SMTP Message Data Processing Test")
smtp_handle_command("HELO client.example.com")
smtp_handle_command("MAIL FROM:<test@example.com>")
smtp_handle_command("RCPT TO:<dest@example.com>")
smtp_handle_command("DATA")

sus end_response tea = smtp_process_message_data(".")
assert_true(string_contains(end_response, "250"))
assert_true(string_contains(end_response, "Message accepted"))

test_start("SMTP Authentication Test")
sus auth_plain tea = smtp_authenticate("PLAIN", "dGVzdFx0ZXN0")
assert_true(string_contains(auth_plain, "235"))

sus auth_login tea = smtp_authenticate("LOGIN", "credentials")
assert_true(string_contains(auth_login, "235"))

sus auth_invalid tea = smtp_authenticate("INVALID", "data")
assert_true(string_contains(auth_invalid, "504"))

test_start("Utility Functions Test")
sus upper_result tea = string_to_upper("Hello World")
assert_eq_string(upper_result, "HELLO WORLD")

sus index_result normie = string_index_of("hello world", "world")
assert_eq_int(index_result, 6)

sus not_found normie = string_index_of("hello", "xyz")
assert_eq_int(not_found, -1)

sus string_result tea = string(42)
assert_eq_string(string_result, "42")

sus neg_string tea = string(-123)
assert_eq_string(neg_string, "-123")

test_start("Protocol Integration Test")
fr fr Test that all protocols can be initialized and basic operations work
tls_init_connection()
ssh_init_connection()
ftp_connect()
smtp_connect()

sus tls_hello tea = tls_create_client_hello()
sus ssh_version tea = ssh_create_version_exchange()
sus ftp_response tea = ftp_handle_command("SYST")
sus smtp_response tea = smtp_handle_command("HELP")

assert_true(string_length(tls_hello) > 0)
assert_true(string_length(ssh_version) > 0)
assert_true(string_length(ftp_response) > 0)
assert_true(string_length(smtp_response) > 0)

test_start("Net Protocols Test Suite")
sus test_result lit = net_protocols_test()
assert_true(test_result)

print_test_summary()
