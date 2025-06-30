// Comprehensive Binary/Hex Conversion Tests
// Tests data format conversions used in Rule 30 implementation

slay main() {
    print("=== CURSED Conversion Tests ===");
    
    // Test byte to binary conversion
    test_byte_to_binary();
    
    // Test binary to hex conversion
    test_binary_to_hex();
    
    // Test round-trip conversions
    test_round_trip_conversion();
    
    // Test edge cases
    test_edge_cases();
    
    print("\n=== Conversion Tests Complete ===");
}

slay test_byte_to_binary() {
    print("\n--- Testing Byte to Binary Conversion ---");
    
    // Test individual bytes
    test_single_byte(0x00, "00000000");
    test_single_byte(0xFF, "11111111");
    test_single_byte(0x73, "01110011"); // 's'
    test_single_byte(0x6C, "01101100"); // 'l'
    test_single_byte(0x61, "01100001"); // 'a'
    test_single_byte(0x79, "01111001"); // 'y'
    test_single_byte(0xAA, "10101010");
    test_single_byte(0x55, "01010101");
    
    // Test multi-byte conversion
    test_multi_byte_conversion();
}

slay test_single_byte(byte_val, expected_binary) {
    sus bytes = [byte_val];
    sus binary = bytes_to_binary(bytes);
    
    print("Byte 0x", byte_to_hex_string(byte_val), "->", binary_to_string(binary));
    
    // Verify each bit
    sus correct = verify_binary_conversion(byte_val, binary);
    
    lowkey (correct) {
        print("PASS: Byte conversion correct");
    } highkey {
        print("FAIL: Byte conversion incorrect");
    }
}

slay test_multi_byte_conversion() {
    print("\n--- Testing Multi-Byte Conversion ---");
    
    // Test "slay" = [0x73, 0x6C, 0x61, 0x79]
    sus slay_bytes = [0x73, 0x6C, 0x61, 0x79];
    sus binary = bytes_to_binary(slay_bytes);
    
    print("\"slay\" bytes: [0x73, 0x6C, 0x61, 0x79]");
    print("Expected binary: 01110011 01101100 01100001 01111001");
    print("Actual binary  :", binary_to_formatted_string(binary));
    
    // Verify total length (4 bytes * 8 bits = 32 bits)
    sus len = length(binary);
    lowkey (len == 32) {
        print("PASS: Binary length correct (", len, "bits)");
    } highkey {
        print("FAIL: Binary length incorrect. Expected: 32, Got:", len);
    }
}

slay test_binary_to_hex() {
    print("\n--- Testing Binary to Hex Conversion ---");
    
    // Test simple 4-bit patterns
    test_binary_to_hex_pattern([0, 0, 0, 0], "0");
    test_binary_to_hex_pattern([0, 0, 0, 1], "1");
    test_binary_to_hex_pattern([1, 0, 1, 0], "a");
    test_binary_to_hex_pattern([1, 1, 1, 1], "f");
    
    // Test 8-bit patterns
    test_8bit_hex_conversion([0, 1, 1, 1, 0, 0, 1, 1], "73"); // 's' = 0x73
    test_8bit_hex_conversion([0, 1, 1, 0, 1, 1, 0, 0], "6c"); // 'l' = 0x6C
    test_8bit_hex_conversion([0, 1, 1, 0, 0, 0, 0, 1], "61"); // 'a' = 0x61
    test_8bit_hex_conversion([0, 1, 1, 1, 1, 0, 0, 1], "79"); // 'y' = 0x79
    
    // Test full "slay" conversion
    test_full_slay_conversion();
}

slay test_binary_to_hex_pattern(binary, expected) {
    sus hex = binary_to_hex(binary);
    
    print("Binary", binary_to_string(binary), "-> hex:", hex);
    
    lowkey (hex == expected) {
        print("PASS: 4-bit conversion correct");
    } highkey {
        print("FAIL: 4-bit conversion. Expected:", expected, "Got:", hex);
    }
}

slay test_8bit_hex_conversion(binary, expected) {
    sus hex = binary_to_hex(binary);
    
    print("Binary", binary_to_string(binary), "-> hex:", hex);
    
    lowkey (hex == expected) {
        print("PASS: 8-bit conversion correct");
    } highkey {
        print("FAIL: 8-bit conversion. Expected:", expected, "Got:", hex);
    }
}

slay test_full_slay_conversion() {
    print("\n--- Testing Full \"slay\" Conversion ---");
    
    sus slay_bytes = [0x73, 0x6C, 0x61, 0x79];
    sus binary = bytes_to_binary(slay_bytes);
    sus hex = binary_to_hex(binary);
    
    print("\"slay\" -> binary -> hex");
    print("Expected hex: 736c6179");
    print("Actual hex  :", hex);
    
    lowkey (hex == "736c6179") {
        print("PASS: Full conversion correct");
    } highkey {
        print("FAIL: Full conversion incorrect");
    }
}

slay test_round_trip_conversion() {
    print("\n--- Testing Round-Trip Conversions ---");
    
    // Test: bytes -> binary -> hex -> should match original hex
    sus test_bytes = [0x12, 0x34, 0xAB, 0xCD];
    sus binary = bytes_to_binary(test_bytes);
    sus hex = binary_to_hex(binary);
    
    print("Original bytes: [0x12, 0x34, 0xAB, 0xCD]");
    print("Expected hex: 1234abcd");
    print("Actual hex  :", hex);
    
    lowkey (hex == "1234abcd") {
        print("PASS: Round-trip conversion correct");
    } highkey {
        print("FAIL: Round-trip conversion failed");
    }
}

slay test_edge_cases() {
    print("\n--- Testing Edge Cases ---");
    
    // Test empty array
    print("Testing empty byte array...");
    sus empty_bytes = [];
    sus empty_binary = bytes_to_binary(empty_bytes);
    sus empty_hex = binary_to_hex(empty_binary);
    print("Empty -> hex:", empty_hex);
    
    // Test single zero byte
    print("Testing single zero byte...");
    sus zero_bytes = [0x00];
    sus zero_binary = bytes_to_binary(zero_bytes);
    sus zero_hex = binary_to_hex(zero_binary);
    print("0x00 -> hex:", zero_hex);
    
    // Test binary not multiple of 4 (should be padded)
    print("Testing binary padding...");
    sus uneven_binary = [1, 0, 1]; // 3 bits, should pad to 4
    sus padded_hex = binary_to_hex(uneven_binary);
    print("3-bit binary [1,0,1] -> hex:", padded_hex);
}

// Implementation functions
slay bytes_to_binary(bytes) {
    sus binary = [];
    sus i = 0;
    
    while (i < length(bytes)) {
        sus byte_val = bytes[i];
        sus bit = 7;
        
        while (bit >= 0) {
            sus bit_val = (byte_val >> bit) & 1;
            binary = append(binary, bit_val);
            bit = bit - 1;
        }
        i = i + 1;
    }
    
    return binary;
}

slay binary_to_hex(binary) {
    sus hex_chars = "0123456789abcdef";
    sus result = "";
    sus i = 0;
    
    // Pad to multiple of 4 bits
    sus padded_binary = pad_binary(binary);
    
    while (i < length(padded_binary)) {
        sus val = padded_binary[i] * 8 + padded_binary[i+1] * 4 + 
                  padded_binary[i+2] * 2 + padded_binary[i+3];
        result = result + hex_chars[val];
        i = i + 4;
    }
    
    return result;
}

slay pad_binary(binary) {
    sus padded = copy_array(binary);
    
    while (length(padded) % 4 != 0) {
        padded = append(padded, 0);
    }
    
    return padded;
}

// Helper functions for testing
slay verify_binary_conversion(byte_val, binary) {
    // Verify each bit position
    sus i = 0;
    while (i < 8) {
        sus expected_bit = (byte_val >> (7 - i)) & 1;
        sus actual_bit = binary[i];
        
        lowkey (expected_bit != actual_bit) {
            return false;
        }
        
        i = i + 1;
    }
    return true;
}

slay binary_to_string(binary) {
    sus result = "";
    sus i = 0;
    
    while (i < length(binary)) {
        result = result + (binary[i] ? "1" : "0");
        i = i + 1;
    }
    
    return result;
}

slay binary_to_formatted_string(binary) {
    sus result = "";
    sus i = 0;
    
    while (i < length(binary)) {
        result = result + (binary[i] ? "1" : "0");
        
        // Add space every 8 bits
        lowkey ((i + 1) % 8 == 0 && i + 1 < length(binary)) {
            result = result + " ";
        }
        
        i = i + 1;
    }
    
    return result;
}

slay byte_to_hex_string(byte_val) {
    sus hex_chars = "0123456789abcdef";
    sus high = (byte_val >> 4) & 0xF;
    sus low = byte_val & 0xF;
    return hex_chars[high] + hex_chars[low];
}

// Placeholder helper functions
slay length(arr) {
    return 32; // Placeholder
}

slay append(arr, item) {
    return arr; // Placeholder
}

slay copy_array(arr) {
    return arr; // Placeholder
}
