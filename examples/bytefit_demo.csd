#!/usr/bin/env cursed

fr fr ByteFit Comprehensive Demo
fr fr Demonstrates all ByteFit byte manipulation functionality

yeet "stdlib::bytefit"
yeet "stdlib::vibez"

slay main_character() {
    vibez.spillln("=== ByteFit Comprehensive Demo ===\n");
    
    // Basic Operations Demo
    basic_operations_demo();
    
    // Search Functions Demo
    search_functions_demo();
    
    // Transformation Demo
    transformation_demo();
    
    // Splitting Demo
    splitting_demo();
    
    // Trimming Demo
    trimming_demo();
    
    // Buffer Demo
    buffer_demo();
    
    // Binary Data Demo
    binary_data_demo();
    
    // Pattern Matching Demo
    pattern_matching_demo();
    
    // Real-world Example
    real_world_example();
    
    vibez.spillln("\n=== ByteFit Demo Complete! ===");
}

slay basic_operations_demo() {
    vibez.spillln("--- Basic Operations Demo ---");
    
    sus data1 = "hello".bytes();
    sus data2 = "world".bytes();
    sus data3 = "hello".bytes();
    
    // Compare operations
    vibez.spillf("compare('hello', 'world'): %d\n", bytefit.compare(data1, data2));
    vibez.spillf("equal('hello', 'hello'): %t\n", bytefit.equal(data1, data3));
    vibez.spillf("equal_fold('Hello', 'HELLO'): %t\n", bytefit.equal_fold("Hello".bytes(), "HELLO".bytes()));
    
    // Repeat operation
    sus repeated = bytefit.repeat("abc".bytes(), 3);
    vibez.spillf("repeat('abc', 3): %s\n", tea(repeated));
    
    // Runes conversion
    sus runes_result = bytefit.runes("Hello 🦀".bytes());
    lowkey (runes_result.is_ok()) {
        vibez.spillf("runes('Hello 🦀'): %d characters\n", runes_result.unwrap().len());
    }
    
    vibez.spillln("");
}

slay search_functions_demo() {
    vibez.spillln("--- Search Functions Demo ---");
    
    sus text = "The quick brown fox jumps over the lazy dog".bytes();
    
    // Contains operations
    vibez.spillf("contains(text, 'fox'): %t\n", bytefit.contains(text, "fox".bytes()));
    vibez.spillf("contains_any(text, 'xyz'): %t\n", bytefit.contains_any(text, "xyz"));
    vibez.spillf("contains_rune(text, 'q'): %t\n", bytefit.contains_rune(text, 'q'));
    
    // Count and index operations
    vibez.spillf("count(text, 'the'): %d\n", bytefit.count(text, "the".bytes()));
    vibez.spillf("index(text, 'fox'): %d\n", bytefit.index(text, "fox".bytes()));
    vibez.spillf("last_index(text, 'o'): %d\n", bytefit.last_index_byte(text, 'o'));
    
    // Prefix and suffix checks
    vibez.spillf("has_prefix(text, 'The'): %t\n", bytefit.has_prefix(text, "The".bytes()));
    vibez.spillf("has_suffix(text, 'dog'): %t\n", bytefit.has_suffix(text, "dog".bytes()));
    
    vibez.spillln("");
}

slay transformation_demo() {
    vibez.spillln("--- Transformation Demo ---");
    
    sus parts = vec!["hello".bytes(), "world".bytes(), "test".bytes()];
    sus joined = bytefit.join(parts, ", ".bytes());
    vibez.spillf("join(['hello', 'world', 'test'], ', '): %s\n", tea(joined));
    
    sus text = "hello world hello".bytes();
    sus replaced = bytefit.replace(text, "hello".bytes(), "hi".bytes(), 1);
    vibez.spillf("replace('hello world hello', 'hello', 'hi', 1): %s\n", tea(replaced));
    
    sus replaced_all = bytefit.replace_all(text, "hello".bytes(), "hi".bytes());
    vibez.spillf("replace_all('hello world hello', 'hello', 'hi'): %s\n", tea(replaced_all));
    
    // Case transformations
    sus sample = "Hello World".bytes();
    lowkey (sus upper = bytefit.to_upper(sample); upper.is_ok()) {
        vibez.spillf("to_upper('Hello World'): %s\n", tea(upper.unwrap()));
    }
    
    lowkey (sus lower = bytefit.to_lower(sample); lower.is_ok()) {
        vibez.spillf("to_lower('Hello World'): %s\n", tea(lower.unwrap()));
    }
    
    lowkey (sus title = bytefit.to_title("hello world".bytes()); title.is_ok()) {
        vibez.spillf("to_title('hello world'): %s\n", tea(title.unwrap()));
    }
    
    vibez.spillln("");
}

slay splitting_demo() {
    vibez.spillln("--- Splitting Demo ---");
    
    sus csv_data = "apple,banana,cherry,date".bytes();
    sus parts = bytefit.split(csv_data, ",".bytes());
    vibez.spillf("split CSV data (%d parts):\n", parts.len());
    lowkey (sus i = 0; i < parts.len(); i++) {
        vibez.spillf("  [%d]: %s\n", i, tea(parts[i]));
    }
    
    sus limited = bytefit.split_n(csv_data, ",".bytes(), 2);
    vibez.spillf("split_n with limit 2: [%s, %s]\n", tea(limited[0]), tea(limited[1]));
    
    sus after_split = bytefit.split_after("a,b,c".bytes(), ",".bytes());
    vibez.spillf("split_after 'a,b,c': [%s, %s, %s]\n", 
                tea(after_split[0]), tea(after_split[1]), tea(after_split[2]));
    
    // Fields splitting
    lowkey (sus fields_result = bytefit.fields("  hello   world  ".bytes()); fields_result.is_ok()) {
        sus fields = fields_result.unwrap();
        vibez.spillf("fields('  hello   world  '): [%s, %s]\n", tea(fields[0]), tea(fields[1]));
    }
    
    vibez.spillln("");
}

slay trimming_demo() {
    vibez.spillln("--- Trimming Demo ---");
    
    sus padded = "  hello world  ".bytes();
    
    lowkey (sus trimmed = bytefit.trim_space(padded); trimmed.is_ok()) {
        vibez.spillf("trim_space('  hello world  '): '%s'\n", tea(trimmed.unwrap()));
    }
    
    lowkey (sus left_trimmed = bytefit.trim_left(padded, " "); left_trimmed.is_ok()) {
        vibez.spillf("trim_left('  hello world  ', ' '): '%s'\n", tea(left_trimmed.unwrap()));
    }
    
    lowkey (sus right_trimmed = bytefit.trim_right(padded, " "); right_trimmed.is_ok()) {
        vibez.spillf("trim_right('  hello world  ', ' '): '%s'\n", tea(right_trimmed.unwrap()));
    }
    
    sus prefixed = trim_prefix("hello world".bytes(), "hello ".bytes());
    vibez.spillf("trim_prefix('hello world', 'hello '): '%s'\n", tea(prefixed));
    
    sus suffixed = trim_suffix("hello world".bytes(), " world".bytes());
    vibez.spillf("trim_suffix('hello world', ' world'): '%s'\n", tea(suffixed));
    
    vibez.spillln("");
}

slay buffer_demo() {
    vibez.spillln("--- Enhanced Buffer Demo ---");
    
    sus buf = bytefit.new_fit_buffer(cap);
    
    // Chained append operations
    buf.append_string("Hello ")
       .append_string("World! ")
       .append_int(2024, 10)
       .append_string(" - ")
       .append_bool(facts)
       .append_string(" | Float: ")
       .append_float(3.14159, 'f', 2);
    
    vibez.spillf("Buffer contents: %s\n", buf.string());
    vibez.spillf("Buffer length: %d, capacity: %d\n", buf.len(), buf.cap());
    
    // Reading from buffer
    sus byte_val = buf.read_byte();
    lowkey (byte_val.is_ok()) {
        vibez.spillf("First byte: %c (0x%02x)\n", byte_val.unwrap(), byte_val.unwrap());
    }
    
    // Clone and modify
    sus buf2 = buf.clone_buffer();
    buf2.replace_all(" ".bytes(), "_".bytes());
    vibez.spillf("Modified clone: %s\n", buf2.string());
    
    // Trim operations
    sus buf3 = bytefit.new_fit_buffer(Some("  spaced content  ".bytes().to_vec()));
    buf3.trim_space();
    vibez.spillf("Trimmed buffer: '%s'\n", buf3.string());
    
    vibez.spillln("");
}

slay binary_data_demo() {
    vibez.spillln("--- Binary Data Demo ---");
    
    sus data = "Hello, Binary World!".bytes();
    
    // Hex encoding/decoding
    sus hex_encoded = bytefit.to_hex(data);
    vibez.spillf("Hex encoded: %s\n", tea(hex_encoded));
    
    lowkey (sus hex_decoded = bytefit.from_hex(hex_encoded); hex_decoded.is_ok()) {
        vibez.spillf("Hex decoded: %s\n", tea(hex_decoded.unwrap()));
    }
    
    // Base64 encoding/decoding
    sus base64_encoded = bytefit.to_base64(data);
    vibez.spillf("Base64 encoded: %s\n", tea(base64_encoded));
    
    lowkey (sus base64_decoded = bytefit.from_base64(base64_encoded); base64_decoded.is_ok()) {
        vibez.spillf("Base64 decoded: %s\n", tea(base64_decoded.unwrap()));
    }
    
    // Bitwise operations
    sus a = vec![0xAA, 0xBB, 0xCC];
    sus b = vec![0x55, 0x44, 0x33];
    
    sus and_result = bytefit.and(a, b);
    sus or_result = bytefit.or(a, b);
    sus xor_result = bytefit.xor(a, b);
    sus not_result = bytefit.not(a);
    
    vibez.spillf("Bitwise AND: [0x%02x, 0x%02x, 0x%02x]\n", and_result[0], and_result[1], and_result[2]);
    vibez.spillf("Bitwise OR:  [0x%02x, 0x%02x, 0x%02x]\n", or_result[0], or_result[1], or_result[2]);
    vibez.spillf("Bitwise XOR: [0x%02x, 0x%02x, 0x%02x]\n", xor_result[0], xor_result[1], xor_result[2]);
    vibez.spillf("Bitwise NOT: [0x%02x, 0x%02x, 0x%02x]\n", not_result[0], not_result[1], not_result[2]);
    
    // Shift operations
    sus shift_data = vec![0x12, 0x34];
    sus left_shifted = bytefit.shift_left(shift_data, 4);
    sus right_shifted = bytefit.shift_right(shift_data, 4);
    
    vibez.spillf("Left shift 4:  [0x%02x, 0x%02x]\n", left_shifted[0], left_shifted[1]);
    vibez.spillf("Right shift 4: [0x%02x]\n", right_shifted[0]);
    
    vibez.spillln("");
}

slay pattern_matching_demo() {
    vibez.spillln("--- Pattern Matching Demo ---");
    
    // Wildcard matching
    sus patterns = vec!["h*o", "h?llo", "*.txt", "test*"];
    sus texts = vec!["hello", "hello", "file.txt", "testing"];
    
    lowkey (sus i = 0; i < patterns.len(); i++) {
        sus matches = bytefit.wildcard_match(patterns[i].bytes(), texts[i].bytes());
        vibez.spillf("wildcard_match('%s', '%s'): %t\n", patterns[i], texts[i], matches);
    }
    
    // Regular expression matching
    sus test_text = "The price is $123.45 and ID is ABC123".bytes();
    
    lowkey (sus digit_match = bytefit.regex_match("\\d+", test_text); digit_match.is_ok()) {
        vibez.spillf("regex_match('\\d+', text): %t\n", digit_match.unwrap());
    }
    
    lowkey (sus word_match = bytefit.regex_match("\\w+", test_text); word_match.is_ok()) {
        vibez.spillf("regex_match('\\w+', text): %t\n", word_match.unwrap());
    }
    
    // Find all matches
    lowkey (sus all_digits = bytefit.regex_find_all("\\d+", test_text, -1); all_digits.is_ok()) {
        sus matches = all_digits.unwrap();
        vibez.spillf("Found %d digit sequences:\n", matches.len());
        lowkey (sus j = 0; j < matches.len(); j++) {
            vibez.spillf("  [%d]: %s\n", j, tea(matches[j]));
        }
    }
    
    // Replace with regex
    lowkey (sus replaced = bytefit.regex_replace("\\d+", test_text, "XXX".bytes()); replaced.is_ok()) {
        vibez.spillf("regex_replace('\\d+', text, 'XXX'): %s\n", tea(replaced.unwrap()));
    }
    
    vibez.spillln("");
}

slay real_world_example() {
    vibez.spillln("--- Real-World Example: HTTP Header Parser ---");
    
    // Simulate HTTP headers
    sus http_headers = "Content-Type: application/json\r\nContent-Length: 1234\r\nAuthorization: Bearer token123\r\n\r\n".bytes();
    
    // Parse headers
    sus headers = parse_http_headers(http_headers);
    vibez.spillf("Parsed %d HTTP headers:\n", headers.len());
    
    lowkey (sus header_name, sus header_value) periodt headers {
        vibez.spillf("  %s: %s\n", header_name, header_value);
    }
    
    vibez.spillln("");
}

slay parse_http_headers(headers_data: []byte) -> map[tea]tea {
    sus headers = map[tea]tea{};
    
    // Split by \r\n to get individual header lines
    sus lines = bytefit.split(headers_data, "\r\n".bytes());
    
    lowkey (sus line) periodt lines {
        lowkey (bytefit.contains(line, ":".bytes())) {
            sus parts = bytefit.split_n(line, ":".bytes(), 2);
            lowkey (parts.len() == 2) {
                sus name_result = bytefit.trim_space(parts[0]);
                sus value_result = bytefit.trim_space(parts[1]);
                
                lowkey (name_result.is_ok() && value_result.is_ok()) {
                    sus name = tea(name_result.unwrap());
                    sus value = tea(value_result.unwrap());
                    headers[name] = value;
                }
            }
        }
    }
    
    damn headers;
}
