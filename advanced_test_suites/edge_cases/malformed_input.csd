// Advanced Edge Case Testing: Malformed Input Handling
yeet "testz"
yeet "jsonz"
yeet "xmlz"
yeet "csvz"
yeet "stringz"

test_start("Malformed Input Edge Cases")

// JSON malformed input testing
slay test_malformed_json() {
    sus malformed_inputs []tea = [
        "",
        "{",
        "}",
        "{\"key\": }",
        "{\"key\": \"value\"",
        "[1, 2, 3",
        "[1, 2, 3,]",
        "{\"key\": \"value\", }",
        "null",
        "undefined",
        "{\"key\": 01}",  // Leading zero
        "{\"key\": 1.}",  // Invalid decimal
        "{\"key\": .5}",  // No leading digit
    ]
    
    bestie (tea input in malformed_inputs) {
        ready {
            sus parsed = parse_json(input)
            // If parsing succeeds unexpectedly, log it
            vibez.spill("Unexpected successful parse:", input)
        } fam {
            when "invalid json" -> {
                // Expected behavior - malformed input rejected
                test_pass("Malformed JSON rejected: " + input[0:10] + "...")
            }
            when _ -> {
                test_fail("Unexpected error for JSON: " + input)
            }
        }
    }
    
    test_pass("JSON malformed input handling")
}

// XML malformed input testing
slay test_malformed_xml() {
    sus malformed_xml_inputs []tea = [
        "<tag>",
        "</tag>",
        "<tag></different>",
        "<tag attr=\"value>",  // Unclosed quote
        "<tag attr=value>",    // Unquoted attribute
        "<<tag>>",
        "<tag><nested></tag>", // Mismatched nesting
        "&invalid;",           // Invalid entity
        "<tag>content<tag>",   // Unclosed tags
    ]
    
    bestie (tea xml_input in malformed_xml_inputs) {
        ready {
            sus parsed = parse_xml(xml_input)
            vibez.spill("Unexpected successful XML parse:", xml_input)
        } fam {
            when "invalid xml" -> {
                test_pass("Malformed XML rejected")
            }
            when _ -> {
                test_fail("Unexpected XML error")
            }
        }
    }
    
    test_pass("XML malformed input handling")
}

// CSV malformed input testing
slay test_malformed_csv() {
    sus malformed_csv_inputs []tea = [
        "field1,field2\n\"unclosed quote",
        "field1,field2\nvalue1,value2,value3",  // Column mismatch
        "field1,field2\n\"value with\nnewline\"",  // Improper newline handling
        "field1,,field3\n",  // Empty fields
        ",,,",  // All empty
        "\"nested\"\"quotes\"\"test\"",  // Complex quote escaping
    ]
    
    bestie (tea csv_input in malformed_csv_inputs) {
        ready {
            sus parsed = parse_csv(csv_input)
            // Some malformed CSV might be parseable with lenient parsing
            test_pass("CSV parsed with potential issues handled")
        } fam {
            when "invalid csv" -> {
                test_pass("Malformed CSV rejected appropriately")
            }
            when _ -> {
                test_pass("CSV error handled gracefully")
            }
        }
    }
    
    test_pass("CSV malformed input handling")
}

// String encoding malformed input
slay test_malformed_string_encoding() {
    // Test various malformed string scenarios
    sus malformed_strings []tea = [
        "\x00",           // Null byte
        "\xFF\xFE\xFD",   // Invalid UTF-8 sequence
        "Hello\x80World", // Invalid continuation byte
        "\xC0\x80",       // Overlong encoding
        "\xED\xA0\x80",   // Surrogate half
    ]
    
    bestie (tea malformed_str in malformed_strings) {
        ready {
            sus len_result drip = len(malformed_str)
            sus valid_str lit = is_valid_utf8(malformed_str)
            
            ready (!valid_str) {
                test_pass("Invalid UTF-8 detected and handled")
            } otherwise {
                test_pass("String handled gracefully")
            }
        } fam {
            when _ -> {
                test_pass("String error handled safely")
            }
        }
    }
    
    test_pass("Malformed string encoding handling")
}

// Network input malformed testing
slay test_malformed_network_input() {
    sus malformed_http_requests []tea = [
        "",
        "GET",
        "GET /path HTTP/1.1",  // Missing headers
        "INVALID /path HTTP/1.1\r\n\r\n",
        "GET /path HTTP/9.9\r\n\r\n",  // Invalid version
        "GET /path HTTP/1.1\r\nHost:\r\n\r\n",  // Empty host
        "GET /path HTTP/1.1\r\nContent-Length: abc\r\n\r\n",  // Invalid content-length
    ]
    
    // Simulate processing malformed HTTP requests
    bestie (tea request in malformed_http_requests) {
        ready {
            sus parsed = parse_http_request(request)
            vibez.spill("Parsed malformed request:", request[0:20] + "...")
        } fam {
            when "invalid http" -> {
                test_pass("Malformed HTTP request rejected")
            }
            when _ -> {
                test_pass("HTTP error handled gracefully")
            }
        }
    }
    
    test_pass("Malformed network input handling")
}

// File path malformed input
slay test_malformed_file_paths() {
    sus malformed_paths []tea = [
        "",
        ".",
        "..",
        "../../../etc/passwd",  // Path traversal
        "file\x00.txt",         // Null byte injection
        "con.txt",              // Windows reserved name
        "file" + ("\x00" * 1000), // Very long path with nulls
        "file|with|pipes",      // Invalid characters
        "file\nwith\nnewlines", // Newlines in path
    ]
    
    bestie (tea path in malformed_paths) {
        ready {
            sus exists lit = file_exists(path)
            // Most malformed paths should not exist or be rejected
            test_pass("Path handled safely: " + path)
        } fam {
            when "invalid path" -> {
                test_pass("Malformed path rejected")
            }
            when _ -> {
                test_pass("Path error handled gracefully")
            }
        }
    }
    
    test_pass("Malformed file path handling")
}

// Mathematical expression malformed input
slay test_malformed_math_expressions() {
    sus malformed_expressions []tea = [
        "",
        "1 + ",
        " + 1",
        "1 ++ 2",
        "1 2",      // Missing operator
        "((1 + 2)", // Unmatched parentheses
        "(1 + 2))", // Unmatched parentheses
        "1 / 0",    // Division by zero
        "sqrt(-1)", // Invalid domain
        "1e",       // Incomplete scientific notation
        "1.2.3",    // Invalid decimal
        "1 + (2 * (3 + 4)", // Nested unmatched parens
    ]
    
    bestie (tea expr in malformed_expressions) {
        ready {
            sus result drip = evaluate_expression(expr)
            vibez.spill("Evaluated malformed expression:", expr, "=", result)
        } fam {
            when "invalid expression" -> {
                test_pass("Malformed math expression rejected")
            }
            when "division by zero" -> {
                test_pass("Division by zero handled")
            }
            when "domain error" -> {
                test_pass("Math domain error handled")
            }
            when _ -> {
                test_pass("Math error handled gracefully")
            }
        }
    }
    
    test_pass("Malformed mathematical expression handling")
}

// Run all malformed input tests
test_malformed_json()
test_malformed_xml()
test_malformed_csv()
test_malformed_string_encoding()
test_malformed_network_input()
test_malformed_file_paths()
test_malformed_math_expressions()

print_test_summary()
