# ByteFit Module Tests - Comprehensive Test Suite
yeet "testz"
yeet "bytefit"

# Test byte manipulation functions
slay test_byte_bit_operations() {
    test_start("byte_set_bit")
    sus b1 byte = 0b00000001
    sus result1 byte = byte_set_bit(b1, 2)
    assert_eq_int(result1, 0b00000101)
    
    test_start("byte_clear_bit")
    sus b2 byte = 0b11111111
    sus result2 byte = byte_clear_bit(b2, 3)
    assert_eq_int(result2, 0b11110111)
    
    test_start("byte_toggle_bit")
    sus b3 byte = 0b00001010
    sus result3 byte = byte_toggle_bit(b3, 2)
    assert_eq_int(result3, 0b00001110)
    
    test_start("byte_test_bit")
    sus b4 byte = 0b00001000
    assert_true(byte_test_bit(b4, 3))
    assert_false(byte_test_bit(b4, 2))
}

slay test_byte_counting() {
    test_start("byte_count_ones")
    sus b1 byte = 0b11010101
    assert_eq_int(byte_count_ones(b1), 5)
    
    test_start("byte_count_zeros")
    sus b2 byte = 0b11010101
    assert_eq_int(byte_count_zeros(b2), 3)
    
    test_start("byte_count_ones_all_set")
    sus b3 byte = 0b11111111
    assert_eq_int(byte_count_ones(b3), 8)
    
    test_start("byte_count_zeros_all_clear")
    sus b4 byte = 0b00000000
    assert_eq_int(byte_count_zeros(b4), 8)
}

slay test_byte_transformations() {
    test_start("byte_reverse_bits")
    sus b1 byte = 0b10110001
    sus result1 byte = byte_reverse_bits(b1)
    assert_eq_int(result1, 0b10001101)
    
    test_start("byte_rotate_left")
    sus b2 byte = 0b11000001
    sus result2 byte = byte_rotate_left(b2, 2)
    assert_eq_int(result2, 0b00000111)
    
    test_start("byte_rotate_right")
    sus b3 byte = 0b11000001
    sus result3 byte = byte_rotate_right(b3, 2)
    assert_eq_int(result3, 0b01110000)
    
    test_start("byte_swap_nibbles")
    sus b4 byte = 0b11110000
    sus result4 byte = byte_swap_nibbles(b4)
    assert_eq_int(result4, 0b00001111)
}

slay test_byte_array_operations() {
    test_start("byte_array_create")
    sus arr1 [byte] = byte_array_create(5)
    assert_eq_int(len(arr1), 5)
    
    test_start("byte_array_fill")
    sus arr2 [byte] = byte_array_create(3)
    sus filled [byte] = byte_array_fill(arr2, 0xFF)
    assert_eq_int(filled[0], 0xFF)
    assert_eq_int(filled[1], 0xFF)
    assert_eq_int(filled[2], 0xFF)
    
    test_start("byte_array_reverse")
    sus arr3 [byte] = [1, 2, 3, 4, 5]
    sus reversed [byte] = byte_array_reverse(arr3)
    assert_eq_int(reversed[0], 5)
    assert_eq_int(reversed[1], 4)
    assert_eq_int(reversed[2], 3)
    assert_eq_int(reversed[3], 2)
    assert_eq_int(reversed[4], 1)
    
    test_start("byte_array_find")
    sus arr4 [byte] = [10, 20, 30, 40, 50]
    sus index normie = byte_array_find(arr4, 30)
    assert_eq_int(index, 2)
    
    test_start("byte_array_find_not_found")
    sus not_found normie = byte_array_find(arr4, 99)
    assert_eq_int(not_found, -1)
}

slay test_byte_array_arithmetic() {
    test_start("byte_array_count")
    sus arr1 [byte] = [1, 2, 1, 3, 1, 4]
    sus count normie = byte_array_count(arr1, 1)
    assert_eq_int(count, 3)
    
    test_start("byte_array_sum")
    sus arr2 [byte] = [1, 2, 3, 4, 5]
    sus sum normie = byte_array_sum(arr2)
    assert_eq_int(sum, 15)
    
    test_start("byte_array_xor")
    sus arr3 [byte] = [0b11110000, 0b00001111]
    sus arr4 [byte] = [0b10101010, 0b01010101]
    sus xor_result [byte] = byte_array_xor(arr3, arr4)
    assert_eq_int(xor_result[0], 0b01011010)
    assert_eq_int(xor_result[1], 0b01011010)
    
    test_start("byte_array_and")
    sus and_result [byte] = byte_array_and(arr3, arr4)
    assert_eq_int(and_result[0], 0b10100000)
    assert_eq_int(and_result[1], 0b00000101)
    
    test_start("byte_array_or")
    sus or_result [byte] = byte_array_or(arr3, arr4)
    assert_eq_int(or_result[0], 0b11111010)
    assert_eq_int(or_result[1], 0b01011111)
}

slay test_byte_encoding() {
    test_start("byte_to_hex")
    sus b1 byte = 0xFF
    sus hex1 tea = byte_to_hex(b1)
    assert_eq_string(hex1, "FF")
    
    test_start("hex_to_byte")
    sus b2 byte = hex_to_byte("A5")
    assert_eq_int(b2, 0xA5)
    
    test_start("byte_array_to_hex")
    sus arr1 [byte] = [0x12, 0x34, 0xAB]
    sus hex_string tea = byte_array_to_hex(arr1)
    assert_eq_string(hex_string, "1234AB")
    
    test_start("hex_to_byte_array")
    sus arr2 [byte] = hex_to_byte_array("FACE")
    assert_eq_int(len(arr2), 2)
    assert_eq_int(arr2[0], 0xFA)
    assert_eq_int(arr2[1], 0xCE)
}

slay test_binary_encoding() {
    test_start("byte_to_binary")
    sus b1 byte = 0b11010101
    sus binary1 tea = byte_to_binary(b1)
    assert_eq_string(binary1, "11010101")
    
    test_start("binary_to_byte")
    sus b2 byte = binary_to_byte("10101010")
    assert_eq_int(b2, 0b10101010)
    
    test_start("byte_to_binary_zero")
    sus b3 byte = 0b00000000
    sus binary2 tea = byte_to_binary(b3)
    assert_eq_string(binary2, "00000000")
    
    test_start("binary_to_byte_max")
    sus b4 byte = binary_to_byte("11111111")
    assert_eq_int(b4, 0xFF)
}

slay test_bit_patterns() {
    test_start("get_bit_pattern")
    sus b1 byte = 0b11010110
    sus pattern1 byte = get_bit_pattern(b1, 2, 3)
    assert_eq_int(pattern1, 0b101)
    
    test_start("set_bit_pattern")
    sus b2 byte = 0b11111111
    sus result byte = set_bit_pattern(b2, 2, 3, 0b010)
    assert_eq_int(result, 0b11101011)
    
    test_start("get_bit_pattern_single")
    sus pattern2 byte = get_bit_pattern(0b10000000, 7, 1)
    assert_eq_int(pattern2, 1)
    
    test_start("set_bit_pattern_zero")
    sus result2 byte = set_bit_pattern(0b11111111, 0, 4, 0b0000)
    assert_eq_int(result2, 0b11110000)
}

slay test_byte_checksums() {
    test_start("byte_parity_even")
    sus b1 byte = 0b11000011
    assert_true(byte_parity(b1))
    
    test_start("byte_parity_odd")
    sus b2 byte = 0b11000001
    assert_false(byte_parity(b2))
    
    test_start("byte_checksum")
    sus arr1 [byte] = [0x12, 0x34, 0x56, 0x78]
    sus checksum byte = byte_checksum(arr1)
    assert_eq_int(checksum, 0x1A)
    
    test_start("byte_crc8")
    sus arr2 [byte] = [0x31, 0x32, 0x33, 0x34]
    sus crc byte = byte_crc8(arr2)
    assert_true(crc != 0)
}

slay test_byte_character_utils() {
    test_start("byte_is_ascii")
    assert_true(byte_is_ascii(65))
    assert_false(byte_is_ascii(200))
    
    test_start("byte_is_printable")
    assert_true(byte_is_printable(65))
    assert_false(byte_is_printable(10))
    
    test_start("byte_is_digit")
    assert_true(byte_is_digit('5'))
    assert_false(byte_is_digit('A'))
    
    test_start("byte_is_alpha")
    assert_true(byte_is_alpha('A'))
    assert_true(byte_is_alpha('z'))
    assert_false(byte_is_alpha('5'))
    
    test_start("byte_is_uppercase")
    assert_true(byte_is_uppercase('A'))
    assert_false(byte_is_uppercase('a'))
    
    test_start("byte_is_lowercase")
    assert_true(byte_is_lowercase('a'))
    assert_false(byte_is_lowercase('A'))
}

slay test_byte_case_conversion() {
    test_start("byte_to_uppercase")
    sus upper byte = byte_to_uppercase('a')
    assert_eq_int(upper, 'A')
    
    test_start("byte_to_lowercase")
    sus lower byte = byte_to_lowercase('Z')
    assert_eq_int(lower, 'z')
    
    test_start("byte_to_uppercase_already_upper")
    sus upper2 byte = byte_to_uppercase('A')
    assert_eq_int(upper2, 'A')
    
    test_start("byte_to_lowercase_already_lower")
    sus lower2 byte = byte_to_lowercase('a')
    assert_eq_int(lower2, 'a')
}

slay test_edge_cases() {
    test_start("byte_operations_zero")
    sus zero byte = 0
    assert_eq_int(byte_count_ones(zero), 0)
    assert_eq_int(byte_count_zeros(zero), 8)
    
    test_start("byte_operations_max")
    sus max byte = 0xFF
    assert_eq_int(byte_count_ones(max), 8)
    assert_eq_int(byte_count_zeros(max), 0)
    
    test_start("empty_array_operations")
    sus empty [byte]
    assert_eq_int(byte_array_sum(empty), 0)
    assert_eq_int(byte_array_count(empty, 1), 0)
    assert_eq_int(byte_array_find(empty, 1), -1)
}

# Run all tests
test_byte_bit_operations()
test_byte_counting()
test_byte_transformations()
test_byte_array_operations()
test_byte_array_arithmetic()
test_byte_encoding()
test_binary_encoding()
test_bit_patterns()
test_byte_checksums()
test_byte_character_utils()
test_byte_case_conversion()
test_edge_cases()

print_test_summary()
