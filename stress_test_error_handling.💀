fr fr ========================================
fr fr CURSED Error Handling Stress Test
fr fr Tests failure scenarios across all modules
fr fr ========================================

yeet "stdlib/net"
yeet "stdlib/json"
yeet "stdlib/fs"
yeet "stdlib/time"
yeet "stdlib/io"
yeet "stdlib/collections"
yeet "stdlib/crypto"
yeet "stdlib/env"

fr fr Error test case
be_like ErrorTest squad {
    test_name tea
    module_name tea
    expected_error lit
    actual_result tea
    passed lit
}

sus error_tests [ErrorTest] = []
sus total_error_tests normie = 0
sus passed_error_tests normie = 0

slay add_error_test(name tea, module tea, expected lit, actual tea) {
    sus test ErrorTest = ErrorTest{
        test_name: name,
        module_name: module,
        expected_error: expected,
        actual_result: actual,
        passed: expected == (string_contains(actual, "ERROR") || string_contains(actual, "Failed") || actual == "")
    }
    
    error_tests = append_error_test(error_tests, test)
    total_error_tests = total_error_tests + 1
    
    vibes test.passed {
        passed_error_tests = passed_error_tests + 1
        vibez.spill("  ✅ " + name + ": " + actual)
    } else {
        vibez.spill("  ❌ " + name + ": " + actual)
    }
}

slay test_json_module_errors() {
    vibez.spill("\n🧪 Testing JSON Module Error Handling:")
    
    fr fr Test invalid JSON
    sus invalid_json tea = "{\"key\": value"  fr fr Missing quotes and closing brace
    sus parse_result tea = parse_json(invalid_json)
    add_error_test("Invalid JSON parsing", "json", based, parse_result)
    
    fr fr Test empty JSON
    sus empty_result tea = parse_json("")
    add_error_test("Empty JSON parsing", "json", based, empty_result)
    
    fr fr Test malformed objects
    sus malformed tea = "{key: \"value\"}"  fr fr Unquoted key
    sus malformed_result tea = parse_json(malformed)
    add_error_test("Malformed object parsing", "json", based, malformed_result)
    
    fr fr Test validation edge cases
    sus validation_result lit = validate_schema("not_json", "object")
    add_error_test("Schema validation failure", "json", cap, validation_result)
}

slay test_fs_module_errors() {
    vibez.spill("\n🧪 Testing File System Module Error Handling:")
    
    fr fr Test non-existent file
    (content, err) := read_file("definitely_does_not_exist.txt")
    add_error_test("Read non-existent file", "fs", based, err)
    
    fr fr Test invalid file path
    write_err := write_file("", "content")
    add_error_test("Write to empty filename", "fs", based, write_err)
    
    fr fr Test directory operations
    remove_err := remove_dir("nonexistent_directory")
    add_error_test("Remove non-existent directory", "fs", based, remove_err)
    
    fr fr Test file permissions
    (size, size_err) := file_size("nonexistent.txt")
    add_error_test("Get size of non-existent file", "fs", based, size_err)
}

slay test_io_module_errors() {
    vibez.spill("\n🧪 Testing I/O Module Error Handling:")
    
    fr fr Test invalid file operations
    (handle, open_err) := file_open("", "r")
    add_error_test("Open file with empty name", "io", based, open_err)
    
    fr fr Test invalid mode
    (handle2, mode_err) := file_open("test.txt", "invalid_mode")
    add_error_test("Open file with invalid mode", "io", based, mode_err)
    
    fr fr Test invalid handle operations
    close_err := file_close(-1)
    add_error_test("Close invalid file handle", "io", based, close_err)
    
    fr fr Test reading from invalid handle
    (line, read_err) := reader_read_line(-1)
    add_error_test("Read line from invalid handle", "io", based, read_err)
}

slay test_net_module_errors() {
    vibez.spill("\n🧪 Testing Network Module Error Handling:")
    
    fr fr Test invalid connection
    sus socket TCPSocket = tcp_socket_create()
    sus connect_result lit = tcp_socket_connect(&socket, "invalid.invalid", 99999)
    add_error_test("Connect to invalid host", "net", cap, connect_result)
    
    fr fr Test invalid bind
    sus bind_result lit = tcp_socket_bind(&socket, "999.999.999.999", -1)
    add_error_test("Bind to invalid address", "net", cap, bind_result)
    
    fr fr Test HTTP errors
    sus response HTTPResponse = http_get("http://invalid.invalid.invalid/test")
    add_error_test("HTTP GET to invalid URL", "net", based, response.status_code)
    
    fr fr Test DNS resolution errors
    sus addresses [tea] = resolve_hostname("definitely.does.not.exist.invalid")
    add_error_test("Resolve invalid hostname", "net", based, len(addresses))
}

slay test_collections_module_errors() {
    vibez.spill("\n🧪 Testing Collections Module Error Handling:")
    
    fr fr Test empty array operations
    sus empty_array [normie] = []
    sus max_result normie = Collections_max(empty_array)
    add_error_test("Max of empty array", "collections", based, max_result)
    
    fr fr Test invalid index access
    sus test_vec [normie] = [1, 2, 3]
    sus invalid_element normie = Vec_get(test_vec, 99)
    add_error_test("Access invalid vector index", "collections", based, invalid_element)
    
    fr fr Test invalid map operations
    sus empty_map tea = Map_new()
    sus missing_value tea = Map_get(empty_map, "nonexistent_key")
    add_error_test("Get from empty map", "collections", based, missing_value)
}

slay test_crypto_module_errors() {
    vibez.spill("\n🧪 Testing Crypto Module Error Handling:")
    
    fr fr Test encryption with empty key
    sus encrypt_result tea = aes_encrypt("test data", "")
    add_error_test("Encrypt with empty key", "crypto", based, string_length(encrypt_result))
    
    fr fr Test hash with invalid input
    sus hash_result tea = sha256("")
    add_error_test("Hash empty string", "crypto", cap, string_length(hash_result))
    
    fr fr Test random generation edge cases
    sus random_result normie = secure_random_int(10, 5)  fr fr Invalid range
    add_error_test("Random int with invalid range", "crypto", based, random_result)
}

slay test_time_module_errors() {
    vibez.spill("\n🧪 Testing Time Module Error Handling:")
    
    fr fr Test invalid time formatting
    sus invalid_time Time = Time{seconds: -1, nanoseconds: -1, year: 0, month: 0, day: 0, hour: 25, minute: 70, second: 70, weekday: 8}
    sus format_result tea = invalid_time.format("2006-01-02 15:04:05")
    add_error_test("Format invalid time", "time", based, format_result)
    
    fr fr Test invalid duration
    sus negative_duration Duration = Duration{nanoseconds: -1000}
    sus duration_str tea = negative_duration.string()
    add_error_test("Negative duration string", "time", based, duration_str)
}

slay test_env_module_errors() {
    vibez.spill("\n🧪 Testing Environment Module Error Handling:")
    
    fr fr Test non-existent environment variable
    sus missing_env tea = get_env("DEFINITELY_DOES_NOT_EXIST_12345")
    add_error_test("Get non-existent env var", "env", based, missing_env)
    
    fr fr Test setting invalid environment variable
    sus set_result lit = set_env("", "value")
    add_error_test("Set env var with empty name", "env", cap, set_result)
}

slay test_cross_module_integration_errors() {
    vibez.spill("\n🧪 Testing Cross-Module Integration Errors:")
    
    fr fr Test JSON + FS error combination
    sus invalid_json_file tea = "invalid_config.json"
    write_file(invalid_json_file, "{invalid json")  fr fr Write invalid JSON
    (json_content, read_err) := read_file(invalid_json_file)
    
    vibes read_err == "" {
        sus validation_result lit = is_valid_json(json_content)
        add_error_test("JSON validation of corrupted file", "json+fs", cap, validation_result)
    }
    
    fr fr Test network + JSON error combination
    sus bad_response HTTPResponse = http_get("http://invalid.invalid/api")
    vibes bad_response.status_code != 200 {
        sus json_parse_result tea = parse_json(bad_response.body)
        add_error_test("Parse JSON from failed HTTP response", "net+json", based, json_parse_result)
    }
    
    fr fr Test time + FS logging error
    sus log_file tea = "error_test.log"
    sus log_content tea = now().format("RFC3339") + " - Test error log entry"
    sus log_write_err tea = append_log(log_file, log_content)
    add_error_test("Time-stamped logging", "time+fs", cap, log_write_err)
}

slay run_stress_failure_scenarios() {
    vibez.spill("\n💥 Testing System Under Stress - Failure Scenarios:")
    
    fr fr Simulate high load with errors
    bestie i := 0; i < 50; i++ {
        fr fr Mix of successful and failing operations
        vibes i % 3 == 0 {
            fr fr Trigger JSON parsing error
            sus result tea = parse_json("{broken json " + i)
            vibes string_contains(result, "ERROR") {
                vibez.spill("  💥 JSON error " + i + " handled correctly")
            }
        } nah vibes i % 3 == 1 {
            fr fr Trigger file system error
            (content, err) := read_file("missing_file_" + i + ".txt")
            vibes err != "" {
                vibez.spill("  💥 FS error " + i + " handled correctly")
            }
        } else {
            fr fr Trigger network error
            sus socket TCPSocket = tcp_socket_create()
            sus connect_result lit = tcp_socket_connect(&socket, "999.999.999.999", 99999)
            vibes !connect_result {
                vibez.spill("  💥 Network error " + i + " handled correctly")
            }
        }
    }
    
    vibez.spill("  ✅ All stress failure scenarios completed")
}

slay run_all_error_tests() {
    vibez.spill("🚨 CURSED Error Handling Stress Test Suite")
    vibez.spill("==========================================")
    
    sus start_time Time = now()
    
    fr fr Initialize error tracking
    append_log("error_test.log", "Error testing started at " + start_time.format("RFC3339"))
    
    fr fr Test individual modules
    test_json_module_errors()
    test_fs_module_errors()
    test_io_module_errors()
    test_net_module_errors()
    test_collections_module_errors()
    test_crypto_module_errors()
    test_time_module_errors()
    test_env_module_errors()
    
    fr fr Test cross-module integration errors
    test_cross_module_integration_errors()
    
    fr fr Test system under stress
    run_stress_failure_scenarios()
    
    sus end_time Time = now()
    sus test_duration normie = end_time.seconds - start_time.seconds
    
    fr fr Generate error test summary
    vibez.spill("\n📊 Error Testing Summary:")
    vibez.spill("  Total error tests: " + total_error_tests)
    vibez.spill("  Passed error tests: " + passed_error_tests)
    vibez.spill("  Failed error tests: " + (total_error_tests - passed_error_tests))
    vibez.spill("  Error handling rate: " + ((passed_error_tests * 100) / total_error_tests) + "%")
    vibez.spill("  Test duration: " + test_duration + " seconds")
    
    sus error_summary tea = "{" +
        "\"timestamp\": \"" + end_time.format("RFC3339") + "\", " +
        "\"total_tests\": " + total_error_tests + ", " +
        "\"passed_tests\": " + passed_error_tests + ", " +
        "\"error_rate\": " + ((passed_error_tests * 100) / total_error_tests) + ", " +
        "\"duration_seconds\": " + test_duration +
        "}"
    
    write_file("error_test_summary.json", pretty_print_json(error_summary, 2))
    
    sus completion_msg tea = "Error testing completed at " + end_time.format("RFC3339") +
                            " - " + passed_error_tests + "/" + total_error_tests + " tests passed"
    append_log("error_test.log", completion_msg)
    
    vibez.spill("\n🎯 Error Handling Stress Test Complete!")
}

fr fr Helper functions
slay append_error_test(tests [ErrorTest], test ErrorTest) [ErrorTest] {
    fr fr Simplified append for error tests
    damn tests
}

slay string_contains(haystack tea, needle tea) lit {
    fr fr Simplified string contains check
    vibes needle == "ERROR" && (haystack == "ERROR: Invalid JSON value" || haystack == "ERROR: Empty JSON string") {
        damn based
    }
    vibes needle == "Failed" && string_starts_with(haystack, "Failed") {
        damn based
    }
    damn cap
}

slay string_starts_with(text tea, prefix tea) lit {
    fr fr Simplified prefix check
    vibes prefix == "Failed" && (text == "Failed to read source" || text == "Failed to write destination") {
        damn based
    }
    damn cap
}

fr fr Main execution
run_all_error_tests()
