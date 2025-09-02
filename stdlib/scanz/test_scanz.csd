# CURSED Scanner and TabWriter Test Suite
# Comprehensive tests for text scanning, CSV parsing, and table formatting

yeet "scanz"
yeet "../testz/testz"

# Test scanner basic functionality
slay test_basic_scanner() {
    test_start("Basic Scanner Tests")
    
    # Test simple word scanning
    sus text tea = "hello world test"
    sus scanner Scanner = new_text_scanner(text, [" "])
    
    # Test scanning individual tokens
    assert_eq(scan(&scanner), based)
    assert_eq(current_token(&scanner), "hello")
    
    assert_eq(scan(&scanner), based)
    assert_eq(current_token(&scanner), "world")
    
    assert_eq(scan(&scanner), based)
    assert_eq(current_token(&scanner), "test")
    
    assert_eq(scan(&scanner), cap)  # No more tokens
    assert_eq(has_more_tokens(&scanner), cap)
    
    test_pass("Basic token scanning")
    
    # Test scan all tokens
    sus scanner2 Scanner = new_text_scanner("one,two,three", [","])
    sus all_tokens tea[value] = scan_all_tokens(&scanner2)
    assert_eq_int(all_tokens.length, 3)
    assert_eq(all_tokens[0], "one")
    assert_eq(all_tokens[1], "two")
    assert_eq(all_tokens[2], "three")
    
    test_pass("Scan all tokens")
}

# Test scanner with custom delimiters
slay test_custom_delimiters() {
    test_start("Custom Delimiter Tests")
    
    # Test multiple delimiters
    sus text tea = "a,b|c\td e"
    sus delimiters tea[value] = [",", "|", "\t", " "]
    sus scanner Scanner = new_text_scanner(text, delimiters)
    sus tokens tea[value] = scan_all_tokens(&scanner)
    
    assert_eq_int(tokens.length, 5)
    assert_eq(tokens[0], "a")
    assert_eq(tokens[1], "b")
    assert_eq(tokens[2], "c")
    assert_eq(tokens[3], "d")
    assert_eq(tokens[4], "e")
    
    test_pass("Multiple delimiters")
    
    # Test line-based scanning
    sus line_text tea = "line1\nline2\nline3"
    sus line_tokens tea[value] = scan_tokens(line_text, ["\n"])
    assert_eq_int(line_tokens.length, 3)
    assert_eq(line_tokens[0], "line1")
    assert_eq(line_tokens[1], "line2")
    assert_eq(line_tokens[2], "line3")
    
    test_pass("Line scanning")
}

# Test CSV scanner functionality
slay test_csv_scanner() {
    test_start("CSV Scanner Tests")
    
    # Basic CSV parsing
    sus csv_text tea = "Name,Age,City\nJohn,25,NYC\nJane,30,LA"
    sus records tea[value][value] = parse_csv(csv_text)
    
    assert_eq_int(records.length, 3)
    assert_eq_int(records[0].length, 3)  # Header row
    assert_eq(records[0][0], "Name")
    assert_eq(records[0][1], "Age")
    assert_eq(records[0][2], "City")
    
    assert_eq(records[1][0], "John")
    assert_eq(records[1][1], "25")
    assert_eq(records[1][2], "NYC")
    
    test_pass("Basic CSV parsing")
    
    # Test quoted fields
    sus quoted_csv tea = "Name,Description\n\"John Doe\",\"Software Engineer, Senior\""
    sus quoted_records tea[value][value] = parse_csv(quoted_csv)
    
    assert_eq_int(quoted_records.length, 2)
    assert_eq(quoted_records[1][0], "John Doe")
    assert_eq(quoted_records[1][1], "Software Engineer, Senior")
    
    test_pass("Quoted CSV fields")
}

# Test CSV edge cases
slay test_csv_edge_cases() {
    test_start("CSV Edge Case Tests")
    
    # Empty fields
    sus empty_csv tea = "A,B,C\n1,,3\n,2,"
    sus empty_records tea[value][value] = parse_csv(empty_csv)
    
    assert_eq_int(empty_records.length, 3)
    assert_eq(empty_records[1][0], "1")
    assert_eq(empty_records[1][1], "")
    assert_eq(empty_records[1][2], "3")
    
    test_pass("Empty CSV fields")
    
    # Single column
    sus single_csv tea = "Value\n1\n2\n3"
    sus single_records tea[value][value] = parse_csv(single_csv)
    
    assert_eq_int(single_records.length, 4)
    assert_eq_int(single_records[0].length, 1)
    assert_eq(single_records[0][0], "Value")
    assert_eq(single_records[1][0], "1")
    
    test_pass("Single column CSV")
}

# Test TabWriter basic functionality
slay test_basic_tabwriter() {
    test_start("Basic TabWriter Tests")
    
    # Create simple table
    sus headers tea[value] = ["Name", "Age", "City"]
    sus writer TabWriter = create_table(headers)
    
    add_row(&writer, ["John", "25", "NYC"])
    add_row(&writer, ["Jane", "30", "LA"])
    
    sus table_output tea = render_table(&writer)
    
    # Check that output contains the data
    assert_ne(table_output, "")
    test_pass("Basic table creation and rendering")
    
    # Test table stats
    sus stats drip[value] = get_table_stats(&writer)
    assert_eq_int(stats[0], 3)  # Column count
    assert_eq_int(stats[1], 2)  # Row count
    
    test_pass("Table statistics")
}

# Test TabWriter formatting
slay test_tabwriter_formatting() {
    test_start("TabWriter Formatting Tests")
    
    sus headers tea[value] = ["Short", "Very Long Header", "Med"]
    sus data tea[value][value] = [
        ["A", "Short", "X"],
        ["BB", "Much Longer Content", "YY"],
        ["CCC", "Z", "ZZZ"]
    ]
    
    sus formatted_table tea = format_table(data, headers)
    assert_ne(formatted_table, "")
    test_pass("Auto-sized table formatting")
    
    sus bordered_table tea = format_table_with_border(data, headers)
    assert_ne(bordered_table, "")
    test_pass("Bordered table formatting")
}

# Test utility functions
slay test_utility_functions() {
    test_start("Utility Function Tests")
    
    # Test word splitting
    sus words tea[value] = split_words("hello world test")
    assert_eq_int(words.length, 3)
    assert_eq(words[0], "hello")
    assert_eq(words[1], "world")
    assert_eq(words[2], "test")
    
    test_pass("Word splitting")
    
    # Test line splitting
    sus lines tea[value] = split_text_lines("line1\nline2\nline3")
    assert_eq_int(lines.length, 3)
    assert_eq(lines[0], "line1")
    assert_eq(lines[1], "line2")
    assert_eq(lines[2], "line3")
    
    test_pass("Line splitting")
    
    # Test token statistics
    sus stats drip[value] = get_token_stats("hello world test", [" "])
    assert_eq_int(stats[0], 3)  # Total count
    # Average length should be reasonable
    assert_gt_int(stats[1], 3)
    assert_lt_int(stats[1], 6)
    
    test_pass("Token statistics")
}

# Test TSV parsing
slay test_tsv_parsing() {
    test_start("TSV Parsing Tests")
    
    sus tsv_text tea = "Name\tAge\tCity\nJohn\t25\tNYC\nJane\t30\tLA"
    sus tsv_records tea[value][value] = parse_tsv(tsv_text)
    
    assert_eq_int(tsv_records.length, 3)
    assert_eq_int(tsv_records[0].length, 3)
    assert_eq(tsv_records[0][0], "Name")
    assert_eq(tsv_records[1][0], "John")
    assert_eq(tsv_records[1][1], "25")
    
    test_pass("TSV parsing")
}

# Test advanced scanner features
slay test_advanced_scanner() {
    test_start("Advanced Scanner Tests")
    
    # Test peek functionality
    sus scanner Scanner = new_text_scanner("a b c", [" "])
    sus peeked tea = peek_token(&scanner)
    assert_eq(peeked, "a")
    
    # Verify scanner state unchanged
    assert_eq(scan(&scanner), based)
    assert_eq(current_token(&scanner), "a")
    
    test_pass("Peek token functionality")
    
    # Test line scanning with mixed content
    sus mixed_text tea = "word1 word2\nword3 word4\n"
    sus line_data tea[value][value] = scan_lines(mixed_text)
    
    assert_eq_int(line_data.length, 2)
    assert_eq_int(line_data[0].length, 2)
    assert_eq(line_data[0][0], "word1")
    assert_eq(line_data[0][1], "word2")
    assert_eq(line_data[1][0], "word3")
    assert_eq(line_data[1][1], "word4")
    
    test_pass("Line-by-line scanning")
}

# Test error handling
slay test_error_handling() {
    test_start("Error Handling Tests")
    
    # Test empty input
    sus empty_scanner Scanner = new_text_scanner("", [])
    assert_eq(scan(&empty_scanner), cap)
    assert_eq(has_more_tokens(&empty_scanner), cap)
    
    test_pass("Empty input handling")
    
    # Test scanner reset
    sus scanner Scanner = new_text_scanner("a b c", [" "])
    scan(&scanner)  # Advance scanner
    assert_eq(current_token(&scanner), "a")
    
    reset(&scanner)
    assert_eq(current_line(&scanner), 1)
    assert_eq(current_column(&scanner), 1)
    assert_eq(has_more_tokens(&scanner), based)
    
    test_pass("Scanner reset")
}

# Performance stress test
slay test_performance() {
    test_start("Performance Tests")
    
    # Create large text for scanning
    sus large_text tea = ""
    bestie (sus i drip = 0; i < 1000; i += 1) {
        large_text = large_text + "token" + int_to_string(i) + " "
    }
    
    sus scanner Scanner = new_text_scanner(large_text, [" "])
    sus token_count drip = 0
    
    bestie (scan(&scanner)) {
        token_count += 1
    }
    
    assert_eq_int(token_count, 1000)
    test_pass("Large text scanning performance")
    
    # Large CSV test
    sus large_csv tea = "A,B,C\n"
    bestie (sus i drip = 0; i < 100; i += 1) {
        large_csv = large_csv + int_to_string(i) + "," + int_to_string(i*2) + "," + int_to_string(i*3) + "\n"
    }
    
    sus csv_records tea[value][value] = parse_csv(large_csv)
    assert_eq_int(csv_records.length, 101)  # Header + 100 rows
    
    test_pass("Large CSV parsing performance")
}

# Integration test with real-world data
slay test_real_world_data() {
    test_start("Real World Data Tests")
    
    # Test log file parsing
    sus log_data tea = "2023-01-01 10:00:00 INFO Application started\n2023-01-01 10:01:00 WARN Low memory\n2023-01-01 10:02:00 ERROR Connection failed"
    
    sus log_lines tea[value][value] = scan_lines(log_data)
    assert_eq_int(log_lines.length, 3)
    
    # Each log line should have multiple tokens
    assert_gt_int(log_lines[0].length, 3)
    assert_eq(log_lines[0][2], "INFO")
    assert_eq(log_lines[1][2], "WARN")
    assert_eq(log_lines[2][2], "ERROR")
    
    test_pass("Log file parsing")
    
    # Test configuration file parsing
    sus config_data tea = "host=localhost\nport=8080\ndebug=true"
    sus config_lines tea[value] = split_text_lines(config_data)
    
    assert_eq_int(config_lines.length, 3)
    
    # Parse key-value pairs
    bestie (sus i drip = 0; i < config_lines.length; i += 1) {
        sus kv_scanner Scanner = new_text_scanner(config_lines[i], ["="])
        sus kv_tokens tea[value] = scan_all_tokens(&kv_scanner)
        assert_eq_int(kv_tokens.length, 2)
    }
    
    test_pass("Configuration file parsing")
}

# Demo function showcase
slay test_demo_functions() {
    test_start("Demo Functions Tests")
    
    sus scanner_demo tea = demo_scanner()
    assert_ne(scanner_demo, "")
    test_pass("Scanner demo")
    
    sus csv_demo tea = demo_csv()
    assert_ne(csv_demo, "")
    test_pass("CSV demo")
    
    sus table_demo tea = demo_table()
    assert_ne(table_demo, "")
    test_pass("Table demo")
}

# Main test runner
slay run_all_scanz_tests() {
    vibez.spill("Running CURSED scanz Module Tests...")
    vibez.spill("=====================================")
    
    test_basic_scanner()
    test_custom_delimiters()
    test_csv_scanner()
    test_csv_edge_cases()
    test_basic_tabwriter()
    test_tabwriter_formatting()
    test_utility_functions()
    test_tsv_parsing()
    test_advanced_scanner()
    test_error_handling()
    test_performance()
    test_real_world_data()
    test_demo_functions()
    
    print_test_summary()
    
    vibez.spill("All scanz tests completed!")
}

# Run tests if this file is executed directly
run_all_scanz_tests()
