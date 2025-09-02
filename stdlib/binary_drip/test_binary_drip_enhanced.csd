fr fr Enhanced CURSED Binary Data Module Tests
fr fr Comprehensive test suite for endianness validation, boundary conditions, and stress testing

yeet "testz"
yeet "binary_drip"

fr fr Test endianness detection and validation
slay test_endianness_detection() {
    test_start("endianness_detection") fr fr Test system endianness detection
    sus is_little lit = is_little_endian()
    sus is_big lit = is_big_endian() fr fr Exactly one should be true
    assert_true(is_little != is_big) fr fr Test with known values
    sus test_val normie = 0x12345678
    sus bytes byte[4] = [4]byte{0x00, 0x00, 0x00, 0x00}
    
    yo is_little { fr fr Little endian: least significant byte first
        write_u32_le(bytes, 0, test_val)
        assert_eq_int(bytes[0], 0x78)
        assert_eq_int(bytes[1], 0x56)
        assert_eq_int(bytes[2], 0x34)
        assert_eq_int(bytes[3], 0x12)
    } kinda { fr fr Big endian: most significant byte first
        write_u32_be(bytes, 0, test_val)
        assert_eq_int(bytes[0], 0x12)
        assert_eq_int(bytes[1], 0x34)
        assert_eq_int(bytes[2], 0x56)
        assert_eq_int(bytes[3], 0x78)
    }
    
    vibez.spill("✅ Endianness detection test passed")
}

fr fr Test boundary conditions and edge cases
slay test_boundary_conditions() {
    test_start("boundary_conditions") fr fr Test maximum values for each type
    sus max_data byte[16] = [16]byte{0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF} fr fr Test u8 boundaries
    sus max_u8 byte = read_u8(max_data, 0)
    assert_eq_int(max_u8, 255)
    
    sus min_data byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
    sus min_u8 byte = read_u8(min_data, 0)
    assert_eq_int(min_u8, 0) fr fr Test u16 boundaries
    sus max_u16_le mid = read_u16_le(max_data, 0)
    assert_eq_int(max_u16_le, 65535)
    
    sus max_u16_be mid = read_u16_be(max_data, 0)
    assert_eq_int(max_u16_be, 65535) fr fr Test u32 boundaries
    sus max_u32_le normie = read_u32_le(max_data, 0)
    assert_eq_int(max_u32_le, 4294967295)
    
    sus max_u32_be normie = read_u32_be(max_data, 0)
    assert_eq_int(max_u32_be, 4294967295) fr fr Test u64 boundaries
    sus max_u64_le thicc = read_u64_le(max_data, 0)
    assert_eq_int(max_u64_le, 18446744073709551615)
    
    sus max_u64_be thicc = read_u64_be(max_data, 0)
    assert_eq_int(max_u64_be, 18446744073709551615)
    
    vibez.spill("✅ Boundary conditions test passed")
}

fr fr Test buffer overflow protection
slay test_buffer_overflow_protection() {
    test_start("buffer_overflow_protection")
    
    sus small_buffer byte[4] = [4]byte{0x01, 0x02, 0x03, 0x04} fr fr Test reading beyond buffer bounds
    sus out_of_bounds_8 byte = read_u8(small_buffer, 10)
    assert_eq_int(out_of_bounds_8, 0) fr fr Should return safe default
    
    sus out_of_bounds_16 mid = read_u16_le(small_buffer, 10)
    assert_eq_int(out_of_bounds_16, 0)
    
    sus out_of_bounds_32 normie = read_u32_le(small_buffer, 10)
    assert_eq_int(out_of_bounds_32, 0)
    
    sus out_of_bounds_64 thicc = read_u64_le(small_buffer, 10)
    assert_eq_int(out_of_bounds_64, 0) fr fr Test writing beyond buffer bounds (should fail gracefully)
    sus write_result1 lit = write_u8(small_buffer, 10, 0xFF)
    assert_false(write_result1)
    
    sus write_result2 lit = write_u16_le(small_buffer, 10, 0xFFFF)
    assert_false(write_result2)
    
    sus write_result3 lit = write_u32_le(small_buffer, 10, 0xFFFFFFFF)
    assert_false(write_result3)
    
    sus write_result4 lit = write_u64_le(small_buffer, 10, 0xFFFFFFFFFFFFFFFF)
    assert_false(write_result4)
    
    vibez.spill("✅ Buffer overflow protection test passed")
}

fr fr Test partial read/write operations
slay test_partial_operations() {
    test_start("partial_operations")
    
    sus buffer byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00} fr fr Test reading multi-byte values at buffer edge
    sus edge_u16 mid = read_u16_le(buffer, 7) fr fr Only 1 byte available
    assert_eq_int(edge_u16, 0) fr fr Should handle gracefully
    
    sus edge_u32 normie = read_u32_le(buffer, 6) fr fr Only 2 bytes available
    assert_eq_int(edge_u32, 0)
    
    sus edge_u64 thicc = read_u64_le(buffer, 4) fr fr Only 4 bytes available
    assert_eq_int(edge_u64, 0) fr fr Test writing at exact buffer boundary
    sus write_at_edge lit = write_u32_le(buffer, 4, 0x12345678)
    assert_true(write_at_edge)
    
    sus read_at_edge normie = read_u32_le(buffer, 4)
    assert_eq_int(read_at_edge, 0x12345678) fr fr Test writing that would exceed buffer
    sus write_exceed lit = write_u64_le(buffer, 4, 0x123456789ABCDEF0)
    assert_false(write_exceed) fr fr Should fail
    
    vibez.spill("✅ Partial operations test passed")
}

fr fr Test signed integer operations
slay test_signed_integers() {
    test_start("signed_integers")
    
    sus buffer byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00} fr fr Test negative i16 values
    sus negative_i16 smol = -32768 fr fr Minimum i16 value
    write_i16_le(buffer, 0, negative_i16)
    sus read_negative_i16 smol = read_i16_le(buffer, 0)
    assert_eq_int(read_negative_i16, negative_i16) fr fr Test negative i32 values
    sus negative_i32 normie = -2147483648 fr fr Minimum i32 value
    write_i32_le(buffer, 4, negative_i32)
    sus read_negative_i32 normie = read_i32_le(buffer, 4)
    assert_eq_int(read_negative_i32, negative_i32) fr fr Test negative i64 values
    sus negative_i64 thicc = -9223372036854775808 fr fr Minimum i64 value
    write_i64_le(buffer, 8, negative_i64)
    sus read_negative_i64 thicc = read_i64_le(buffer, 8)
    assert_eq_int(read_negative_i64, negative_i64) fr fr Test positive boundary values
    sus positive_i16 smol = 32767 fr fr Maximum i16 value
    write_i16_be(buffer, 0, positive_i16)
    sus read_positive_i16 smol = read_i16_be(buffer, 0)
    assert_eq_int(read_positive_i16, positive_i16)
    
    sus positive_i32 normie = 2147483647 fr fr Maximum i32 value
    write_i32_be(buffer, 4, positive_i32)
    sus read_positive_i32 normie = read_i32_be(buffer, 4)
    assert_eq_int(read_positive_i32, positive_i32)
    
    sus positive_i64 thicc = 9223372036854775807 fr fr Maximum i64 value
    write_i64_be(buffer, 8, positive_i64)
    sus read_positive_i64 thicc = read_i64_be(buffer, 8)
    assert_eq_int(read_positive_i64, positive_i64)
    
    vibez.spill("✅ Signed integers test passed")
}

fr fr Test floating point operations
slay test_floating_point() {
    test_start("floating_point")
    
    sus buffer byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00} fr fr Test f32 operations
    sus pi_f32 meal = 3.14159
    write_f32_le(buffer, 0, pi_f32)
    sus read_pi_f32 meal = read_f32_le(buffer, 0)
    assert_true(abs(read_pi_f32 - pi_f32) < 0.00001)
    
    write_f32_be(buffer, 4, pi_f32)
    sus read_pi_f32_be meal = read_f32_be(buffer, 4)
    assert_true(abs(read_pi_f32_be - pi_f32) < 0.00001) fr fr Test f64 operations
    sus e_f64 meal = 2.718281828459045
    write_f64_le(buffer, 8, e_f64)
    sus read_e_f64 meal = read_f64_le(buffer, 8)
    assert_true(abs(read_e_f64 - e_f64) < 0.000000000001) fr fr Test special float values
    sus inf_f32 meal = positive_infinity_f32()
    write_f32_le(buffer, 0, inf_f32)
    sus read_inf meal = read_f32_le(buffer, 0)
    assert_true(is_infinity(read_inf))
    
    sus nan_f32 meal = not_a_number_f32()
    write_f32_le(buffer, 4, nan_f32)
    sus read_nan meal = read_f32_le(buffer, 4)
    assert_true(is_nan(read_nan))
    
    vibez.spill("✅ Floating point test passed")
}

fr fr Test varint encoding/decoding edge cases
slay test_varint_edge_cases() {
    test_start("varint_edge_cases") fr fr Test zero encoding/decoding
    sus zero_encoded byte[value] = varint_encode(0)
    assert_eq_int(len(zero_encoded), 1)
    assert_eq_int(zero_encoded[0], 0x00)
    sus zero_decoded thicc = varint_decode(zero_encoded)
    assert_eq_int(zero_decoded, 0) fr fr Test maximum single-byte value
    sus max_single_encoded byte[value] = varint_encode(127)
    assert_eq_int(len(max_single_encoded), 1)
    assert_eq_int(max_single_encoded[0], 0x7F)
    sus max_single_decoded thicc = varint_decode(max_single_encoded)
    assert_eq_int(max_single_decoded, 127) fr fr Test minimum two-byte value
    sus min_double_encoded byte[value] = varint_encode(128)
    assert_eq_int(len(min_double_encoded), 2)
    assert_eq_int(min_double_encoded[0], 0x80)
    assert_eq_int(min_double_encoded[1], 0x01)
    sus min_double_decoded thicc = varint_decode(min_double_encoded)
    assert_eq_int(min_double_decoded, 128) fr fr Test large values
    sus large_value thicc = 268435455 fr fr Maximum 4-byte varint
    sus large_encoded byte[value] = varint_encode(large_value)
    assert_eq_int(len(large_encoded), 4)
    sus large_decoded thicc = varint_decode(large_encoded)
    assert_eq_int(large_decoded, large_value) fr fr Test maximum value
    sus max_value thicc = 18446744073709551615 fr fr Maximum u64
    sus max_encoded byte[value] = varint_encode(max_value)
    assert_true(len(max_encoded) <= 10) fr fr Maximum varint length
    sus max_decoded thicc = varint_decode(max_encoded)
    assert_eq_int(max_decoded, max_value)
    
    vibez.spill("✅ Varint edge cases test passed")
}

fr fr Test bit manipulation operations
slay test_bit_operations() {
    test_start("bit_operations")
    
    sus value normie = 0b11010110 fr fr 214 in binary fr fr Test bit extraction
    assert_true(get_bit(value, 0)) fr fr Bit 0 is 0
    assert_true(get_bit(value, 1)) fr fr Bit 1 is 1
    assert_true(get_bit(value, 2)) fr fr Bit 2 is 1
    assert_false(get_bit(value, 3)) fr fr Bit 3 is 0
    assert_true(get_bit(value, 4)) fr fr Bit 4 is 1
    assert_false(get_bit(value, 5)) fr fr Bit 5 is 0
    assert_true(get_bit(value, 6)) fr fr Bit 6 is 1
    assert_true(get_bit(value, 7)) fr fr Bit 7 is 1 fr fr Test bit setting
    sus modified normie = set_bit(value, 3) fr fr Set bit 3
    assert_eq_int(modified, 0b11011110) fr fr 222 fr fr Test bit clearing
    sus cleared normie = clear_bit(value, 1) fr fr Clear bit 1
    assert_eq_int(cleared, 0b11010100) fr fr 212 fr fr Test bit toggling
    sus toggled normie = toggle_bit(value, 3) fr fr Toggle bit 3
    assert_eq_int(toggled, 0b11011110) fr fr 222 fr fr Test bit field extraction
    sus field normie = extract_bits(value, 2, 4) fr fr Extract bits 2-5
    assert_eq_int(field, 0b0101) fr fr 5 fr fr Test bit counting
    sus pop_count normie = count_set_bits(value)
    assert_eq_int(pop_count, 5) fr fr 5 bits are set
    
    sus leading_zeros normie = count_leading_zeros(value)
    assert_eq_int(leading_zeros, 24) fr fr 32-bit value with 5 leading zeros
    
    vibez.spill("✅ Bit operations test passed")
}

fr fr Test data serialization formats
slay test_serialization_formats() {
    test_start("serialization_formats") fr fr Test network byte order (big-endian) conversion
    sus host_value normie = 0x12345678
    sus network_value normie = host_to_network_u32(host_value)
    sus back_to_host normie = network_to_host_u32(network_value)
    assert_eq_int(back_to_host, host_value) fr fr Test protocol buffer encoding
    sus pb_value thicc = 300
    sus pb_encoded byte[value] = protobuf_encode_varint(pb_value)
    sus pb_decoded thicc = protobuf_decode_varint(pb_encoded)
    assert_eq_int(pb_decoded, pb_value) fr fr Test base64 encoding
    sus original_data byte[value] = byte[value]{0x48, 0x65, 0x6C, 0x6C, 0x6F} fr fr "Hello"
    sus base64_encoded tea = base64_encode(original_data)
    sus base64_decoded byte[value] = base64_decode(base64_encoded)
    assert_eq_int(len(base64_decoded), len(original_data))
    
    bestie i := 0; i < len(original_data); i++ {
        assert_eq_int(base64_decoded[i], original_data[i])
    } fr fr Test hexadecimal encoding
    sus hex_encoded tea = hex_encode(original_data)
    sus hex_decoded byte[value] = hex_decode(hex_encoded)
    assert_eq_int(len(hex_decoded), len(original_data))
    
    bestie i := 0; i < len(original_data); i++ {
        assert_eq_int(hex_decoded[i], original_data[i])
    }
    
    vibez.spill("✅ Serialization formats test passed")
}

fr fr Test checksum and hash functions
slay test_checksum_operations() {
    test_start("checksum_operations")
    
    sus test_data byte[value] = byte[value]{0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64} fr fr "Hello World" fr fr Test CRC32 checksum
    sus crc32_value normie = calculate_crc32(test_data)
    assert_true(crc32_value != 0) fr fr Test that same data produces same checksum
    sus crc32_repeat normie = calculate_crc32(test_data)
    assert_eq_int(crc32_value, crc32_repeat) fr fr Test MD5 hash
    sus md5_hash byte[value] = calculate_md5(test_data)
    assert_eq_int(len(md5_hash), 16) fr fr MD5 is 128 bits = 16 bytes fr fr Test SHA256 hash
    sus sha256_hash byte[value] = calculate_sha256(test_data)
    assert_eq_int(len(sha256_hash), 32) fr fr SHA256 is 256 bits = 32 bytes fr fr Test that different data produces different hashes
    sus different_data byte[value] = byte[value]{0x48, 0x65, 0x6C, 0x6C, 0x6F} fr fr "Hello"
    sus different_md5 byte[value] = calculate_md5(different_data)
    assert_false(bytes_equal(md5_hash, different_md5))
    
    vibez.spill("✅ Checksum operations test passed")
}

fr fr Stress test for binary operations
slay test_binary_stress() {
    test_start("binary_stress")
    
    sus large_buffer byte[value] = make(byte[value], 1048576) fr fr 1MB buffer
    sus iterations normie = 10000 fr fr Fill buffer with pseudo-random data
    bestie i := 0; i < len(large_buffer); i++ {
        large_buffer[i] = (i * 7 + 13) % 256
    } fr fr Perform many read operations
    sus read_count normie = 0
    bestie i := 0; i < iterations; i++ {
        sus offset normie = (i * 17) % (len(large_buffer) - 8)
        
        sus u8_val byte = read_u8(large_buffer, offset)
        sus u16_val mid = read_u16_le(large_buffer, offset)
        sus u32_val normie = read_u32_le(large_buffer, offset)
        sus u64_val thicc = read_u64_le(large_buffer, offset)
        
        yo u8_val != 0 || u16_val != 0 || u32_val != 0 || u64_val != 0 {
            read_count++
        }
    }
    
    assert_true(read_count > 0) fr fr Should have read some non-zero values fr fr Perform many write operations
    sus write_buffer byte[value] = make(byte[value], 1024)
    bestie i := 0; i < 100; i++ {
        sus offset normie = (i * 4) % (len(write_buffer) - 8)
        
        write_u8(write_buffer, offset, i.(byte))
        write_u16_le(write_buffer, offset + 1, (i * 2).(mid))
        write_u32_le(write_buffer, offset + 2, (i * 4).(normie)) fr fr Verify writes
        sus read_u8 byte = read_u8(write_buffer, offset)
        assert_eq_int(read_u8, i)
    }
    
    vibez.spill("✅ Binary stress test passed")
}

fr fr Main test runner
slay main() {
    vibez.spill("🧪 Running Enhanced CURSED Binary Data Module Tests")
    vibez.spill("=========================================================")
    
    test_endianness_detection()
    test_boundary_conditions()
    test_buffer_overflow_protection()
    test_partial_operations()
    test_signed_integers()
    test_floating_point()
    test_varint_edge_cases()
    test_bit_operations()
    test_serialization_formats()
    test_checksum_operations()
    test_binary_stress()
    
    vibez.spill("=========================================================")
    print_test_summary()
    vibez.spill("🎉 All enhanced binary data tests completed!")
}
