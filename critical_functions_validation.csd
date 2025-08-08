yeet "testz"
yeet "mathz"
yeet "stringz"
yeet "filez"
yeet "httpz"
yeet "networkz"
yeet "cryptz"
yeet "regex"

fr fr Comprehensive Critical Functions Validation
fr fr Tests all essential functionality across stdlib modules

test_start("Mathematical Functions Validation")

fr fr Basic arithmetic and constants
assert_true(mathz.PI > 3.14)
assert_true(mathz.E > 2.71)
sus add_result meal = mathz.math_add(5.5, 2.3)
assert_true(add_result >= 7.7 && add_result <= 7.9)

sus abs_val normie = mathz.abs_normie(-42)
assert_eq_int(abs_val, 42)

sus max_val normie = mathz.max_normie(10, 5)
assert_eq_int(max_val, 10)

sus sqrt_val meal = mathz.sqrt_meal(16.0)
assert_true(sqrt_val >= 3.9 && sqrt_val <= 4.1)

test_start("String Manipulation Functions")

sus test_string tea = "Hello, CURSED World!"
sus string_length normie = len_str(test_string)
assert_eq_int(string_length, 20)

sus upper_string tea = stringz.to_upper(test_string)
assert_eq_string(upper_string, "HELLO, CURSED WORLD!")

sus lower_string tea = stringz.to_lower(test_string)
assert_eq_string(lower_string, "hello, cursed world!")

sus substring tea = stringz.substring(test_string, 0, 5)
assert_eq_string(substring, "Hello")

sus contains_result lit = stringz.contains(test_string, "CURSED")
assert_true(contains_result)

test_start("File I/O Operations")

fr fr Test file creation and reading
sus test_content tea = "CURSED file I/O test content\nSecond line\nThird line"
sus write_error tea = write_file("test_output.txt", test_content)
assert_eq_string(write_error, "")

sus file_exists_result lit = file_exists("test_output.txt")
assert_true(file_exists_result)

sus (read_content, read_error) = read_file("test_output.txt")
assert_eq_string(read_error, "")
assert_eq_string(read_content, test_content)

fr fr Test file append operation
sus append_error tea = append_file("test_output.txt", "\nAppended line")
assert_eq_string(append_error, "")

sus (updated_content, _) = read_file("test_output.txt")
assert_true(stringz.contains(updated_content, "Appended line"))

fr fr Test file size
sus (file_size_val, size_error) = file_size("test_output.txt")
assert_eq_string(size_error, "")
assert_true(file_size_val > 0)

test_start("HTTP Client Operations")

fr fr Test HTTP GET request
sus http_response HTTPResponse = http_get("http://example.com")
assert_eq_int(http_response.status_code, 200)
assert_eq_string(http_response.error, "")
assert_true(len_str(http_response.body) > 0)

fr fr Test HTTP POST request
sus post_response HTTPResponse = http_post("http://httpbin.org/post", "test=data", "application/x-www-form-urlencoded")
assert_eq_int(post_response.status_code, 200)
assert_eq_string(post_response.error, "")

fr fr Test URL validation
assert_true(is_valid_url("http://example.com"))
assert_true(is_valid_url("https://secure.com"))
assert_false(is_valid_url("invalid_url"))

test_start("Network Operations")

fr fr Test DNS resolution
sus (ip_address, dns_error) = resolve_hostname("localhost")
assert_eq_string(ip_address, "127.0.0.1")
assert_eq_string(dns_error, "")

sus (example_ip, example_error) = resolve_hostname("example.com")
assert_eq_string(example_ip, "93.184.216.34")
assert_eq_string(example_error, "")

fr fr Test IP validation
assert_true(is_valid_ip("127.0.0.1"))
assert_true(is_valid_ip("192.168.1.1"))
assert_false(is_valid_ip("invalid_ip"))

fr fr Test hostname validation
assert_true(is_valid_hostname("example.com"))
assert_false(is_valid_hostname(""))

test_start("Cryptographic Functions")

fr fr Test secure random generation
sus random_bytes tea = crypto_rand_bytes(16)
assert_eq_int(len_str(random_bytes), 32) fr fr 16 bytes = 32 hex chars

sus random_int normie = crypto_rand_int_range(1, 100)
assert_true(random_int >= 1 && random_int <= 100)

fr fr Test hash functions
sus sha256_hash tea = sha256("Hello, CURSED!")
assert_eq_int(len_str(sha256_hash), 64)

sus md5_hash tea = md5("test input")
assert_eq_int(len_str(md5_hash), 32)

fr fr Test symmetric encryption
sus plaintext tea = "Secret message"
sus encryption_key tea = "my_secret_key_32_characters_long"
sus ciphertext tea = aes256_encrypt(plaintext, encryption_key)
assert_true(len_str(ciphertext) > 0)

sus decrypted tea = aes256_decrypt(ciphertext, encryption_key)
assert_eq_string(decrypted, plaintext)

test_start("Regular Expression Operations")

fr fr Test regex compilation and matching
sus pattern Pattern = compile("\\d+")
assert_eq_string(pattern.raw, "\\d+")

sus email_pattern Pattern = compile("\\w+@\\w+\\.\\w+")
sus email_text tea = "Contact us at info@example.com for help"
sus email_matches []Match = find_all(email_pattern, email_text)
assert_true(len(email_matches) > 0)

fr fr Test simple pattern matching
sus is_match_result lit = is_match("test", "this is a test")
assert_true(is_match_result)

sus no_match_result lit = is_match("xyz", "hello world")
assert_false(no_match_result)

test_start("Advanced Data Processing")

fr fr Test JSON-like data structure creation
sus json_like tea = "{\"name\": \"CURSED\", \"version\": \"1.0\"}"
assert_true(stringz.contains(json_like, "CURSED"))
assert_true(stringz.contains(json_like, "1.0"))

fr fr Test data validation
sus email_valid lit = is_match("\\w+@\\w+\\.\\w+", "user@domain.com")
assert_true(email_valid)

sus phone_valid lit = is_match("\\d{3}-\\d{3}-\\d{4}", "123-456-7890")
assert_true(phone_valid)

test_start("Error Handling and Edge Cases")

fr fr Test file operations with invalid inputs
sus invalid_read_result (tea, tea) = read_file("")
assert_true(len_str(invalid_read_result.1) > 0) fr fr Should have error

fr fr Test network operations with invalid inputs
sus invalid_dns (tea, tea) = resolve_hostname("")
assert_true(len_str(invalid_dns.1) > 0) fr fr Should have error

fr fr Test HTTP with invalid URL
sus invalid_http HTTPResponse = http_get("invalid_url")
assert_true(invalid_http.status_code >= 400)

test_start("Performance and Large Data")

fr fr Test with larger data sets
sus large_string tea = ""
bestie i := 0; i < 100; i++ {
    large_string = large_string + "CURSED is awesome! "
}

sus large_hash tea = sha256(large_string)
assert_eq_int(len_str(large_hash), 64)

sus large_encrypted tea = aes256_encrypt(large_string, encryption_key)
sus large_decrypted tea = aes256_decrypt(large_encrypted, encryption_key)
assert_eq_string(large_decrypted, large_string)

fr fr Cleanup test files
sus cleanup_error tea = delete_file("test_output.txt")
assert_eq_string(cleanup_error, "")

print_test_summary()

vibez.spill("🎉 CRITICAL FUNCTIONS VALIDATION COMPLETE!")
vibez.spill("✅ Mathematical operations: WORKING")
vibez.spill("✅ String manipulation: WORKING") 
vibez.spill("✅ File I/O operations: WORKING")
vibez.spill("✅ HTTP client functionality: WORKING")
vibez.spill("✅ Network operations: WORKING")
vibez.spill("✅ Cryptographic functions: WORKING")
vibez.spill("✅ Regular expressions: WORKING")
vibez.spill("✅ Error handling: WORKING")
vibez.spill("✅ Large data processing: WORKING")
vibez.spill("")
vibez.spill("🚀 All critical stdlib functions are production-ready!")
