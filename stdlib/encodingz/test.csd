yeet "testz"
yeet "encodingz"

test_start("Encodingz Module Tests")

fr fr === Base64 Encoding Tests ===
test_case("Base64 Basic Encoding") {
    sus test_data tea = "Hello, CURSED!"
    sus encoded tea = base64_encode(test_data)
    sus decoded tea = base64_decode(encoded)
    
    assert_eq_string(decoded, test_data)
    assert_not_empty(encoded)
}

test_case("Base64 URL Safe Encoding") {
    sus test_data tea = "Testing URL safe encoding with special chars: +/="
    sus encoded tea = base64_encode_url_safe(test_data) 
    sus decoded tea = base64_decode_url_safe(encoded)
    
    assert_eq_string(decoded, test_data)
    fr fr URL safe should not contain + or /
    assert_not_contains(encoded, "+")
    assert_not_contains(encoded, "/")
}

test_case("Base64 Empty String") {
    sus empty tea = ""
    sus encoded tea = base64_encode(empty)
    sus decoded tea = base64_decode(encoded)
    
    assert_eq_string(decoded, empty)
}

fr fr === Hex Encoding Tests ===
test_case("Hex Basic Encoding") {
    sus test_data tea = "CURSED"
    sus encoded tea = hex_encode(test_data)
    sus decoded tea = hex_decode(encoded)
    
    assert_eq_string(decoded, test_data)
    fr fr Hex should be lowercase by default
    assert_eq_string(encoded, "43555253454b")
}

test_case("Hex Upper Case Encoding") {
    sus test_data tea = "test"
    sus encoded tea = hex_encode_upper(test_data)
    
    fr fr Should be uppercase hex
    assert_contains(encoded, "A", "B", "C", "D", "E", "F")
    assert_not_contains(encoded, "a", "b", "c", "d", "e", "f")
}

test_case("Hex Binary Data") {
    sus binary_data []drip = [0, 255, 128, 64]
    sus encoded tea = hex_encode(binary_data)
    sus decoded []drip = hex_decode(encoded)
    
    assert_eq_int(decoded.len(), 4)
    assert_eq_int(decoded[0], 0)
    assert_eq_int(decoded[1], 255)
}

fr fr === ASCII85 Encoding Tests ===
test_case("ASCII85 Basic Encoding") {
    sus test_data tea = "Hello, World!"
    sus encoded tea = ascii85_encode(test_data)
    
    assert_not_empty(encoded)
    fr fr ASCII85 should be more compact than base64
    sus base64_encoded tea = base64_encode(test_data)
    assert_less_than(encoded.len(), base64_encoded.len())
}

fr fr === URL Encoding Tests ===
test_case("URL Encoding Special Characters") {
    sus test_data tea = "hello world!@#$%^&*()"
    sus encoded tea = url_encode(test_data)
    sus decoded tea = url_decode(encoded)
    
    assert_eq_string(decoded, test_data)
    fr fr Spaces should be encoded as %20
    assert_contains(encoded, "%20")
    assert_not_contains(encoded, " ")
}

test_case("URL Encoding Unicode") {
    sus unicode_data tea = "Héllo, 世界!"
    sus encoded tea = url_encode(unicode_data)
    sus decoded tea = url_decode(encoded)
    
    assert_eq_string(decoded, unicode_data)
    fr fr Unicode should be percent encoded
    assert_contains(encoded, "%")
}

fr fr === Streaming Encoder Tests ===
test_case("Streaming Base64 Encoder") {
    sus encoder = create_stream_encoder("base64")
    
    sus chunk1 tea = "First chunk"
    sus chunk2 tea = "Second chunk" 
    sus chunk3 tea = "Final chunk"
    
    sus result1 tea = stream_encode_chunk(encoder, chunk1)
    sus result2 tea = stream_encode_chunk(encoder, chunk2)
    sus result3 tea = stream_encode_chunk(encoder, chunk3)
    sus final tea = stream_finalize(encoder)
    
    sus full_result tea = result1 + result2 + result3 + final
    
    fr fr Should match non-streaming encoding
    sus expected tea = base64_encode(chunk1 + chunk2 + chunk3)
    assert_eq_string(full_result, expected)
}

fr fr === Performance Benchmark Tests ===
test_case("Encoding Performance Benchmark") {
    sus test_data tea = "Performance test data for benchmarking encoding operations"
    sus iterations drip = 1000
    
    sus results = benchmark_encoding(test_data, iterations)
    
    fr fr Should return timing results
    assert_not_null(results)
    assert_greater_than(results.base64_time, 0)
    assert_greater_than(results.hex_time, 0)
}

fr fr === Error Handling Tests ===
test_case("Invalid Base64 Decoding") {
    sus invalid_b64 tea = "invalid base64!!!"
    
    fam {
        sus result tea = base64_decode(invalid_b64)
        fail("Should have thrown error for invalid base64")
    } shook (err tea) {
        assert_contains(err, "invalid")
    }
}

test_case("Invalid Hex Decoding") {
    sus invalid_hex tea = "GHIJ"  fr fr Not valid hex
    
    fam {
        sus result = hex_decode(invalid_hex)
        fail("Should have thrown error for invalid hex")
    } shook (err tea) {
        assert_contains(err, "invalid")
    }
}

fr fr === Large Data Tests ===
test_case("Large Data Base64 Encoding") {
    sus large_data tea = ""
    bestie (sus i drip = 0; i < 10000; i++) {
        large_data = large_data + "CURSED data chunk " + i + "\n"
    }
    
    sus encoded tea = base64_encode(large_data)
    sus decoded tea = base64_decode(encoded)
    
    assert_eq_string(decoded, large_data)
    assert_eq_int(decoded.len(), large_data.len())
}

fr fr === Edge Case Tests ===
test_case("Null and Empty Input Handling") {
    sus null_result tea = base64_encode("")
    assert_eq_string(null_result, "")
    
    sus hex_null_result tea = hex_encode("")
    assert_eq_string(hex_null_result, "")
}

test_case("Single Character Encoding") {
    sus single_char tea = "A"
    
    sus b64_encoded tea = base64_encode(single_char)
    sus b64_decoded tea = base64_decode(b64_encoded)
    assert_eq_string(b64_decoded, single_char)
    
    sus hex_encoded tea = hex_encode(single_char)
    sus hex_decoded tea = hex_decode(hex_encoded)
    assert_eq_string(hex_decoded, single_char)
}

print_test_summary()
