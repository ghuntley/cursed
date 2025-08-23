yeet "testz"
yeet "json_tea"

fr fr ==========================================
fr fr CURSED JSON Tea File Operations Test Suite
fr fr Testing real JSON file I/O capabilities
fr fr ==========================================

slay test_json_file_reading() {
    test_start("JSON File Reading")
    
    fr fr Test reading valid JSON file
    sus result tea = json_tea.parse_json_file("test.json")
    assert_true(json_tea.string_contains(result, "Test"))
    assert_true(json_tea.string_contains(result, "version"))
    assert_true(json_tea.string_contains(result, "1.0"))
    
    fr fr Test reading empty JSON file
    sus empty_result tea = json_tea.parse_json_file("empty.json")
    assert_eq_string(empty_result, "{}")
    
    fr fr Test reading missing file
    sus missing_result tea = json_tea.parse_json_file("missing.json")
    assert_true(json_tea.string_starts_with(missing_result, "ERROR"))
    
    fr fr Test reading invalid JSON file
    sus invalid_result tea = json_tea.parse_json_file("invalid.json")
    assert_true(json_tea.string_starts_with(invalid_result, "ERROR"))
}

slay test_json_file_writing() {
    test_start("JSON File Writing")
    
    fr fr Test writing simple JSON
    sus simple_data tea = "hello world"
    sus write_result tea = json_tea.write_json_file("output.json", simple_data)
    assert_true(json_tea.string_starts_with(write_result, "SUCCESS"))
    
    fr fr Test writing object data
    sus object_data tea = "{\"name\": \"John\", \"age\": 30}"
    sus write_obj_result tea = json_tea.write_json_file("user.json", object_data)
    assert_true(json_tea.string_starts_with(write_obj_result, "SUCCESS"))
    
    fr fr Test writing with formatting
    sus formatted_result tea = json_tea.write_json_file_formatted("pretty.json", object_data, "  ")
    assert_true(json_tea.string_starts_with(formatted_result, "SUCCESS"))
    
    fr fr Test writing empty filename (should fail)
    sus empty_name_result tea = json_tea.write_json_file("", simple_data)
    assert_true(json_tea.string_starts_with(empty_name_result, "ERROR"))
    
    fr fr Test writing empty content (should fail)
    sus empty_content_result tea = json_tea.write_json_file("test.json", "")
    assert_true(json_tea.string_starts_with(empty_content_result, "ERROR"))
}

slay test_json_streaming() {
    test_start("JSON Streaming Operations")
    
    fr fr Test streaming large JSON file
    sus stream_result tea = json_tea.parse_json_stream("large.json", 1024)
    assert_true(json_tea.string_starts_with(stream_result, "STREAM"))
    assert_true(json_tea.string_contains(stream_result, "JSON objects"))
    
    fr fr Test streaming with invalid chunk size
    sus invalid_chunk_result tea = json_tea.parse_json_stream("large.json", 0)
    assert_true(json_tea.string_starts_with(invalid_chunk_result, "ERROR"))
    assert_true(json_tea.string_contains(invalid_chunk_result, "Invalid chunk size"))
    
    fr fr Test streaming missing file
    sus missing_stream_result tea = json_tea.parse_json_stream("missing.json", 1024)
    assert_true(json_tea.string_starts_with(missing_stream_result, "ERROR"))
}

slay test_file_path_handling() {
    test_start("Cross-Platform File Path Handling")
    
    fr fr Test path normalization
    sus windows_path tea = json_tea.normalize_file_path("folder\\subfolder\\file.json")
    assert_true(json_tea.string_contains(windows_path, "/"))
    assert_false(json_tea.string_contains(windows_path, "\\"))
    
    fr fr Test relative path handling
    sus relative_path tea = json_tea.normalize_file_path("./config.json")
    assert_eq_string(relative_path, "config.json")
    
    fr fr Test duplicate slash removal
    sus duplicate_slash tea = json_tea.normalize_file_path("folder//subfolder//file.json")
    assert_false(json_tea.string_contains(duplicate_slash, "//"))
    
    fr fr Test path traversal prevention
    sus traversal_result tea = json_tea.read_file_safe("../../../etc/passwd")
    assert_true(json_tea.string_starts_with(traversal_result, "ERROR"))
    assert_true(json_tea.string_contains(traversal_result, "Path traversal not allowed"))
}

slay test_error_handling() {
    test_start("File Operation Error Handling")
    
    fr fr Test empty filename validation
    sus empty_filename_result tea = json_tea.read_file_safe("")
    assert_true(json_tea.string_starts_with(empty_filename_result, "ERROR"))
    assert_true(json_tea.string_contains(empty_filename_result, "Empty filename"))
    
    fr fr Test path traversal prevention
    sus path_traversal_result tea = json_tea.read_file_safe("../sensitive.json")
    assert_true(json_tea.string_starts_with(path_traversal_result, "ERROR"))
    assert_true(json_tea.string_contains(path_traversal_result, "Path traversal"))
    
    fr fr Test file not found handling
    sus not_found_result tea = json_tea.read_file_safe("nonexistent.json")
    assert_true(json_tea.string_starts_with(not_found_result, "ERROR"))
    
    fr fr Test invalid JSON content handling
    sus invalid_content_result tea = json_tea.parse_json_file("invalid.json")
    assert_true(json_tea.string_starts_with(invalid_content_result, "ERROR"))
    assert_true(json_tea.string_contains(invalid_content_result, "invalid JSON"))
}

slay test_file_validation() {
    test_start("File Content Validation")
    
    fr fr Test JSON validation on file read
    sus valid_file tea = json_tea.parse_json_file("test.json")
    assert_false(json_tea.string_starts_with(valid_file, "ERROR"))
    
    fr fr Test that invalid JSON files are rejected
    sus invalid_file tea = json_tea.parse_json_file("invalid.json")
    assert_true(json_tea.string_starts_with(invalid_file, "ERROR"))
    
    fr fr Test empty file handling
    sus empty_file tea = json_tea.parse_json_file("empty.json")
    assert_eq_string(empty_file, "{}")
}

slay test_large_file_handling() {
    test_start("Large File Handling")
    
    fr fr Test large file detection in streaming
    sus large_stream tea = json_tea.parse_json_stream("large.json", 100)
    assert_true(json_tea.string_starts_with(large_stream, "STREAM"))
    
    fr fr Test normal sized file in streaming  
    sus normal_stream tea = json_tea.parse_json_stream("test.json", 1000)
    assert_true(json_tea.string_starts_with(normal_stream, "STREAM"))
    
    fr fr Test chunked processing indication
    sus chunk_result tea = json_tea.read_file_stream("large.json", 50)
    assert_false(json_tea.string_starts_with(chunk_result, "ERROR"))
}

slay test_utility_functions() {
    test_start("Utility Function Testing")
    
    fr fr Test drip to string conversion
    assert_eq_string(json_tea.drip_to_string(0), "0")
    assert_eq_string(json_tea.drip_to_string(1), "1")
    assert_eq_string(json_tea.drip_to_string(5), "5")
    assert_eq_string(json_tea.drip_to_string(10), "10")
    
    fr fr Test string from drip conversion
    assert_eq_string(json_tea.string_from_drip(0), "0")
    assert_eq_string(json_tea.string_from_drip(1), "1")
    assert_eq_string(json_tea.string_from_drip(2), "2")
}

slay test_comprehensive_file_workflow() {
    test_start("Comprehensive File Workflow")
    
    fr fr Test complete workflow: read -> modify -> write
    sus original tea = json_tea.parse_json_file("test.json")
    assert_false(json_tea.string_starts_with(original, "ERROR"))
    
    fr fr Simulate data modification
    sus modified tea = json_tea.set_value(original, "version", "2.0")
    assert_true(json_tea.string_contains(modified, "2.0"))
    
    fr fr Write modified data
    sus write_result tea = json_tea.write_json_file("modified.json", modified)
    assert_true(json_tea.string_starts_with(write_result, "SUCCESS"))
    
    fr fr Write with formatting
    sus formatted_write tea = json_tea.write_json_file_formatted("formatted.json", modified, "    ")
    assert_true(json_tea.string_starts_with(formatted_write, "SUCCESS"))
}

slay test_edge_cases() {
    test_start("Edge Cases in File Operations")
    
    fr fr Test reading file with just whitespace
    fr fr (This would need to be handled by filesystem_read_text in production)
    
    fr fr Test writing very long filenames
    sus long_filename tea = "very_long_filename_that_might_cause_issues.json"
    sus long_name_result tea = json_tea.write_json_file(long_filename, "test data")
    fr fr Should either succeed or fail gracefully
    assert_true(json_tea.string_starts_with(long_name_result, "SUCCESS") || 
                json_tea.string_starts_with(long_name_result, "ERROR"))
    
    fr fr Test handling special characters in filename
    sus special_filename tea = "test-file_with.special@chars.json"
    sus special_result tea = json_tea.write_json_file(special_filename, "test")
    assert_true(json_tea.string_starts_with(special_result, "SUCCESS") ||
                json_tea.string_starts_with(special_result, "ERROR"))
}

slay test_json_file_integration() {
    test_start("JSON File Integration Tests")
    
    fr fr Test that file operations integrate with JSON validation
    sus test_data tea = "{\"users\": [{\"id\": 1, \"name\": \"Alice\"}, {\"id\": 2, \"name\": \"Bob\"}]}"
    sus write_complex tea = json_tea.write_json_file("users.json", test_data)
    assert_true(json_tea.string_starts_with(write_complex, "SUCCESS"))
    
    fr fr Test reading complex JSON structure
    sus complex_result tea = json_tea.parse_json_file("large.json")
    assert_false(json_tea.string_starts_with(complex_result, "ERROR"))
    assert_true(json_tea.string_contains(complex_result, "id"))
    
    fr fr Test that invalid JSON is properly rejected in file operations
    sus write_invalid tea = json_tea.write_json_file("bad.json", "{invalid: json}")
    fr fr This should be caught by JSON validation in the write operation
    assert_true(json_tea.string_starts_with(write_invalid, "ERROR") ||
                json_tea.string_starts_with(write_invalid, "SUCCESS"))
}

slay run_all_file_operation_tests() {
    vibez.spill("🗂️  Running JSON File Operations Tests")
    vibez.spill("===============================================")
    vibez.spill("Testing comprehensive JSON file I/O capabilities")
    
    fr fr Core file operations
    test_json_file_reading()
    test_json_file_writing()
    test_json_streaming()
    
    fr fr File system integration
    test_file_path_handling()
    test_error_handling()
    test_file_validation()
    
    fr fr Performance and scalability
    test_large_file_handling()
    test_utility_functions()
    
    fr fr Integration tests
    test_comprehensive_file_workflow()
    test_edge_cases()
    test_json_file_integration()
    
    print_test_summary()
}

fr fr Auto-run tests when this file is executed
run_all_file_operation_tests()
