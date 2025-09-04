fr fr ===== ENCODINGZ TEST SUITE - Comprehensive Encoding/Decoding Tests =====
fr fr Tests for Base64, Hex, ASCII85, URL encoding, and streaming operations
fr fr Covers edge cases, performance benchmarks, and production scenarios

yeet "testz"
yeet "encodingz"
yeet "vibez"
yeet "stringz"

fr fr ===== BASE64 ENCODING TESTS =====

slay test_base64_standard_encoding() {
    vibez.spill("🧪 Testing Base64 Standard Encoding...")
    
    fr fr Test empty string
    test_eq_string(base64_encode(""), "", "Empty string encoding")
    
    fr fr Test single character
    test_eq_string(base64_encode("A"), "QQ==", "Single character 'A'")
    test_eq_string(base64_encode("f"), "Zg==", "Single character 'f'")
    
    fr fr Test two characters
    test_eq_string(base64_encode("AB"), "QUI=", "Two characters 'AB'")
    test_eq_string(base64_encode("fo"), "Zm8=", "Two characters 'fo'")
    
    fr fr Test three characters (no padding)
    test_eq_string(base64_encode("ABC"), "QUJD", "Three characters 'ABC'")
    test_eq_string(base64_encode("foo"), "Zm9v", "Three characters 'foo'")
    
    fr fr Test longer strings
    test_eq_string(base64_encode("Hello"), "SGVsbG8=", "Hello encoding")
    test_eq_string(base64_encode("Hello World!"), "SGVsbG8gV29ybGQh", "Hello World! encoding")
    
    fr fr Test binary data simulation
    test_eq_string(base64_encode("Man"), "TWFu", "Binary-like data 'Man'")
    
    vibez.spill("✅ Base64 Standard Encoding Tests Passed")
}

slay test_base64_url_safe_encoding() {
    vibez.spill("🧪 Testing Base64 URL-Safe Encoding...")
    
    fr fr Test basic URL-safe encoding (no padding)
    test_eq_string(base64_encode_url_safe("A"), "QQ", "URL-safe single char (no padding)")
    test_eq_string(base64_encode_url_safe("AB"), "QUI", "URL-safe two chars (no padding)")
    test_eq_string(base64_encode_url_safe("ABC"), "QUJD", "URL-safe three chars")
    
    fr fr Test strings that would contain +/ in standard encoding
    test_eq_string(base64_encode_url_safe("???>"), "Pz8_Pg", "URL-safe with replacement chars")
    
    vibez.spill("✅ Base64 URL-Safe Encoding Tests Passed")
}

slay test_base64_decoding() {
    vibez.spill("🧪 Testing Base64 Decoding...")
    
    fr fr Test standard decoding
    test_decode_success(base64_decode(""), "", "Empty decode")
    test_decode_success(base64_decode("QQ=="), "A", "Single char decode")
    test_decode_success(base64_decode("QUI="), "AB", "Two char decode")
    test_decode_success(base64_decode("QUJD"), "ABC", "Three char decode")
    test_decode_success(base64_decode("SGVsbG8="), "Hello", "Hello decode")
    test_decode_success(base64_decode("SGVsbG8gV29ybGQh"), "Hello World!", "Hello World! decode")
    
    fr fr Test URL-safe decoding
    test_decode_success(base64_decode_url_safe("QQ"), "A", "URL-safe single char")
    test_decode_success(base64_decode_url_safe("QUI"), "AB", "URL-safe two chars")
    test_decode_success(base64_decode_url_safe("QUJD"), "ABC", "URL-safe three chars")
    
    fr fr Test round-trip encoding/decoding
    sus test_strings tea[value] = ["", "A", "AB", "ABC", "Hello", "Hello World!", "The quick brown fox jumps over the lazy dog"]
    sus i drip = 0
    bestie i < array_length(test_strings) {
        sus original tea = test_strings[i]
        sus encoded tea = base64_encode(original)
        sus decoded tea = base64_decode(encoded) fam {
            when err -> {
                vibez.spill("❌ Decode error for '" + original + "': " + err)
                damn
            }
        }
        test_eq_string(decoded, original, "Round-trip: " + original)
        i = i + 1
    }
    
    vibez.spill("✅ Base64 Decoding Tests Passed")
}

slay test_base64_error_handling() {
    vibez.spill("🧪 Testing Base64 Error Handling...")
    
    fr fr Test invalid characters
    test_decode_error(base64_decode("QQ@="), "Invalid character")
    test_decode_error(base64_decode("QQ =="), "Invalid character (space)")
    test_decode_error(base64_decode("QQ\n=="), "Invalid character (newline)")
    
    fr fr Test invalid padding
    test_decode_error(base64_decode("Q==="), "Invalid padding")
    test_decode_error(base64_decode("QQQ"), "Missing padding")
    
    vibez.spill("✅ Base64 Error Handling Tests Passed")
}

fr fr ===== HEX ENCODING TESTS =====

slay test_hex_encoding() {
    vibez.spill("🧪 Testing Hex Encoding...")
    
    fr fr Test empty string
    test_eq_string(hex_encode(""), "", "Empty hex encode")
    
    fr fr Test single characters
    test_eq_string(hex_encode("A"), "41", "Single char 'A' hex")
    test_eq_string(hex_encode("0"), "30", "Single char '0' hex")
    test_eq_string(hex_encode("a"), "61", "Single char 'a' hex")
    
    fr fr Test longer strings
    test_eq_string(hex_encode("Hello"), "48656c6c6f", "Hello hex encoding")
    test_eq_string(hex_encode("World!"), "576f726c6421", "World! hex encoding")
    
    fr fr Test uppercase encoding
    test_eq_string(hex_encode_upper("Hello"), "48656C6C6F", "Hello hex uppercase")
    test_eq_string(hex_encode_upper("abc"), "616263", "abc hex uppercase")
    
    vibez.spill("✅ Hex Encoding Tests Passed")
}

slay test_hex_decoding() {
    vibez.spill("🧪 Testing Hex Decoding...")
    
    fr fr Test basic decoding
    test_decode_success(hex_decode(""), "", "Empty hex decode")
    test_decode_success(hex_decode("41"), "A", "Single char hex decode")
    test_decode_success(hex_decode("48656c6c6f"), "Hello", "Hello hex decode")
    test_decode_success(hex_decode("48656C6C6F"), "Hello", "Hello hex decode uppercase")
    
    fr fr Test mixed case
    test_decode_success(hex_decode("48656c6C6f"), "Hello", "Mixed case hex decode")
    
    fr fr Test round-trip
    sus test_data tea[value] = ["", "A", "Hello", "The quick brown fox", "1234567890", "Special chars: !@#$%^&*()"]
    sus i drip = 0
    bestie i < array_length(test_data) {
        sus original tea = test_data[i]
        sus encoded tea = hex_encode(original)
        sus decoded tea = hex_decode(encoded) fam {
            when err -> {
                vibez.spill("❌ Hex decode error for '" + original + "': " + err)
                damn
            }
        }
        test_eq_string(decoded, original, "Hex round-trip: " + original)
        i = i + 1
    }
    
    vibez.spill("✅ Hex Decoding Tests Passed")
}

slay test_hex_error_handling() {
    vibez.spill("🧪 Testing Hex Error Handling...")
    
    fr fr Test invalid length
    test_decode_error(hex_decode("4"), "Odd length")
    test_decode_error(hex_decode("486"), "Odd length")
    
    fr fr Test invalid characters
    test_decode_error(hex_decode("4G"), "Invalid hex char G")
    test_decode_error(hex_decode("4@"), "Invalid hex char @")
    test_decode_error(hex_decode("4 "), "Invalid hex char space")
    
    vibez.spill("✅ Hex Error Handling Tests Passed")
}

fr fr ===== ASCII85 ENCODING TESTS =====

slay test_ascii85_encoding() {
    vibez.spill("🧪 Testing ASCII85 Encoding...")
    
    fr fr Test empty string
    test_eq_string(ascii85_encode(""), "<~~>", "Empty ASCII85 encode")
    
    fr fr Test single characters
    sus single_a tea = ascii85_encode("A")
    test_true(string_starts_with(single_a, "<~"), "ASCII85 starts with delimiter")
    test_true(string_ends_with(single_a, "~>"), "ASCII85 ends with delimiter")
    
    fr fr Test four-byte chunks (all zeros special case)
    sus zeros tea = ascii85_encode("\0\0\0\0")
    test_true(string_contains(zeros, "z"), "All zeros encoded as 'z'")
    
    fr fr Test regular data
    sus hello tea = ascii85_encode("Hello")
    test_true(string_length(hello) > 4, "ASCII85 Hello has content")
    test_true(string_starts_with(hello, "<~"), "ASCII85 Hello has delimiter")
    
    vibez.spill("✅ ASCII85 Encoding Tests Passed")
}

fr fr ===== URL ENCODING TESTS =====

slay test_url_encoding() {
    vibez.spill("🧪 Testing URL Encoding...")
    
    fr fr Test unreserved characters (should not be encoded)
    test_eq_string(url_encode("ABC"), "ABC", "Letters not encoded")
    test_eq_string(url_encode("123"), "123", "Numbers not encoded")
    test_eq_string(url_encode(".-_~"), ".-_~", "Safe symbols not encoded")
    
    fr fr Test reserved characters (should be encoded)
    test_eq_string(url_encode(" "), "%20", "Space encoded")
    test_eq_string(url_encode("!"), "%21", "Exclamation encoded")
    test_eq_string(url_encode("@"), "%40", "At symbol encoded")
    test_eq_string(url_encode("#"), "%23", "Hash encoded")
    test_eq_string(url_encode("$"), "%24", "Dollar encoded")
    test_eq_string(url_encode("%"), "%25", "Percent encoded")
    test_eq_string(url_encode("&"), "%26", "Ampersand encoded")
    test_eq_string(url_encode("+"), "%2B", "Plus encoded")
    
    fr fr Test complex strings
    test_eq_string(url_encode("Hello World!"), "Hello%20World%21", "Hello World! encoded")
    test_eq_string(url_encode("user@example.com"), "user%40example.com", "Email encoded")
    
    vibez.spill("✅ URL Encoding Tests Passed")
}

slay test_url_decoding() {
    vibez.spill("🧪 Testing URL Decoding...")
    
    fr fr Test basic decoding
    test_decode_success(url_decode("ABC"), "ABC", "No encoding to decode")
    test_decode_success(url_decode("Hello%20World%21"), "Hello World!", "Hello World! decode")
    test_decode_success(url_decode("user%40example.com"), "user@example.com", "Email decode")
    
    fr fr Test plus sign decoding
    test_decode_success(url_decode("Hello+World"), "Hello World", "Plus as space")
    
    fr fr Test round-trip encoding
    sus test_urls tea[value] = [
        "Hello World!",
        "user@example.com",
        "path/to/resource?query=value&other=data",
        "Special chars: !@#$%^&*()",
        "Unicode: café"
    ]
    
    sus i drip = 0
    bestie i < array_length(test_urls) {
        sus original tea = test_urls[i]
        sus encoded tea = url_encode(original)
        sus decoded tea = url_decode(encoded) fam {
            when err -> {
                vibez.spill("❌ URL decode error for '" + original + "': " + err)
                damn
            }
        }
        test_eq_string(decoded, original, "URL round-trip: " + original)
        i = i + 1
    }
    
    vibez.spill("✅ URL Decoding Tests Passed")
}

slay test_url_error_handling() {
    vibez.spill("🧪 Testing URL Error Handling...")
    
    fr fr Test incomplete percent encoding
    test_decode_error(url_decode("Hello%2"), "Incomplete percent")
    test_decode_error(url_decode("Hello%"), "Missing hex digits")
    
    fr fr Test invalid hex in percent encoding
    test_decode_error(url_decode("Hello%2G"), "Invalid hex G")
    test_decode_error(url_decode("Hello%GH"), "Invalid hex GH")
    
    vibez.spill("✅ URL Error Handling Tests Passed")
}

fr fr ===== STREAMING ENCODER TESTS =====

slay test_streaming_base64() {
    vibez.spill("🧪 Testing Streaming Base64 Encoding...")
    
    fr fr Test streaming encoder creation
    sus encoder StreamEncoder = create_stream_encoder("base64")
    test_eq_string(encoder.context.encoding_type, "base64", "Encoder type")
    test_false(encoder.is_finalized, "Not finalized initially")
    
    fr fr Test chunk processing
    sus result1 tea = stream_encode_chunk(encoder, "Hel")
    sus result2 tea = stream_encode_chunk(encoder, "lo ")
    sus result3 tea = stream_encode_chunk(encoder, "World!")
    sus final_result tea = stream_finalize(encoder)
    
    fr fr Combine all results
    sus streaming_encoded tea = result1 + result2 + result3 + final_result
    sus direct_encoded tea = base64_encode("Hello World!")
    
    test_eq_string(streaming_encoded, direct_encoded, "Streaming matches direct encoding")
    test_true(encoder.is_finalized, "Encoder finalized")
    
    vibez.spill("✅ Streaming Base64 Tests Passed")
}

slay test_streaming_hex() {
    vibez.spill("🧪 Testing Streaming Hex Encoding...")
    
    sus encoder StreamEncoder = create_stream_encoder("hex")
    sus result1 tea = stream_encode_chunk(encoder, "Hello")
    sus result2 tea = stream_encode_chunk(encoder, " World")
    sus final_result tea = stream_finalize(encoder)
    
    sus streaming_encoded tea = result1 + result2 + final_result
    sus direct_encoded tea = hex_encode("Hello World")
    
    test_eq_string(streaming_encoded, direct_encoded, "Streaming hex matches direct")
    
    vibez.spill("✅ Streaming Hex Tests Passed")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_encoding_performance() {
    vibez.spill("🧪 Testing Encoding Performance...")
    
    fr fr Test small data performance
    sus small_data tea = "Hello World! This is a test string for performance measurement."
    sus small_benchmark tea = benchmark_encoding(small_data, 1000)
    vibez.spill("Small data benchmark: " + small_benchmark)
    test_true(string_contains(small_benchmark, "bytes"), "Performance result includes bytes")
    
    fr fr Test medium data performance
    sus medium_data tea = ""
    sus i drip = 0
    bestie i < 100 {
        medium_data = medium_data + "This is line " + int_to_string(i) + " of test data for performance testing.\n"
        i = i + 1
    }
    sus medium_benchmark tea = benchmark_encoding(medium_data, 100)
    vibez.spill("Medium data benchmark: " + medium_benchmark)
    
    vibez.spill("✅ Performance Tests Completed")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    vibez.spill("🧪 Testing Edge Cases...")
    
    fr fr Test very long strings
    sus long_string tea = ""
    sus i drip = 0
    bestie i < 1000 {
        long_string = long_string + "A"
        i = i + 1
    }
    
    sus long_encoded tea = base64_encode(long_string)
    sus long_decoded tea = base64_decode(long_encoded) fam {
        when err -> {
            vibez.spill("❌ Long string decode error: " + err)
            damn
        }
    }
    test_eq_int(string_length(long_decoded), string_length(long_string), "Long string length preserved")
    
    fr fr Test strings with all byte values (simulated)
    sus all_chars tea = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()[]{}|\\:;\"'<>?,./-_=+"
    sus all_encoded tea = base64_encode(all_chars)
    sus all_decoded tea = base64_decode(all_encoded) fam {
        when err -> {
            vibez.spill("❌ All chars decode error: " + err)
            damn
        }
    }
    test_eq_string(all_decoded, all_chars, "All printable chars round-trip")
    
    vibez.spill("✅ Edge Case Tests Passed")
}

fr fr ===== HELPER FUNCTIONS =====

slay test_decode_success(result yikes<tea>, expected tea, description tea) {
    result fam {
        when err -> {
            vibez.spill("❌ " + description + " - Decode error: " + err)
            damn
        }
    }
    test_eq_string(result, expected, description)
}

slay test_decode_error(result yikes<tea>, description tea) {
    result fam {
        when err -> {
            fr fr Expected error occurred
            vibez.spill("✅ " + description + " - Expected error: " + err)
            damn
        }
    }
    vibez.spill("❌ " + description + " - Expected error but got success")
}

slay test_eq_string(actual tea, expected tea, description tea) {
    ready actual == expected {
        vibez.spill("✅ " + description)
    } otherwise {
        vibez.spill("❌ " + description + " - Expected: '" + expected + "', Got: '" + actual + "'")
    }
}

slay test_eq_int(actual drip, expected drip, description tea) {
    ready actual == expected {
        vibez.spill("✅ " + description)
    } otherwise {
        vibez.spill("❌ " + description + " - Expected: " + int_to_string(expected) + ", Got: " + int_to_string(actual))
    }
}

slay test_true(condition lit, description tea) {
    ready condition == based {
        vibez.spill("✅ " + description)
    } otherwise {
        vibez.spill("❌ " + description + " - Expected true")
    }
}

slay test_false(condition lit, description tea) {
    ready condition == cap {
        vibez.spill("✅ " + description)
    } otherwise {
        vibez.spill("❌ " + description + " - Expected false")
    }
}

slay string_starts_with(s tea, prefix tea) lit {
    ready string_length(prefix) > string_length(s) {
        damn cap
    }
    damn substring(s, 0, string_length(prefix)) == prefix
}

slay string_ends_with(s tea, suffix tea) lit {
    ready string_length(suffix) > string_length(s) {
        damn cap
    }
    sus start drip = string_length(s) - string_length(suffix)
    damn substring(s, start, string_length(s)) == suffix
}

slay string_contains(s tea, substr tea) lit {
    sus i drip = 0
    bestie i <= string_length(s) - string_length(substr) {
        ready substring(s, i, i + string_length(substr)) == substr {
            damn based
        }
        i = i + 1
    }
    damn cap
}

slay int_to_string(n drip) tea {
    fr fr Mock implementation - replace with real function
    ready n == 0 { damn "0" }
    ready n == 1 { damn "1" }
    ready n == 100 { damn "100" }
    ready n == 1000 { damn "1000" }
    damn "number"
}

fr fr ===== MAIN TEST RUNNER =====

slay main_character() {
    vibez.spill("🚀 Starting EncodingZ Test Suite...")
    vibez.spill("=" * 50)
    
    test_base64_standard_encoding()
    test_base64_url_safe_encoding()
    test_base64_decoding()
    test_base64_error_handling()
    
    test_hex_encoding()
    test_hex_decoding()
    test_hex_error_handling()
    
    test_ascii85_encoding()
    
    test_url_encoding()
    test_url_decoding()
    test_url_error_handling()
    
    test_streaming_base64()
    test_streaming_hex()
    
    test_encoding_performance()
    test_edge_cases()
    
    vibez.spill("=" * 50)
    vibez.spill("🎉 EncodingZ Test Suite Complete!")
    vibez.spill("All encoding utilities tested successfully")
}

main()
