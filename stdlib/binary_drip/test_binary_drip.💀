fr fr Comprehensive test suite for binary_drip module
yeet "testz"
yeet "binary_drip"

fr fr Test data setup
sus test_data byte[16] = [16]byte{0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10}

fr fr Test read_u8 function
test_start("read_u8 basic functionality")
sus val1 byte = read_u8(test_data, 0)
assert_eq_int(val1, 0x01)
sus val2 byte = read_u8(test_data, 5)
assert_eq_int(val2, 0x06)

fr fr Test read_u16_le function
test_start("read_u16_le little-endian")
sus val16le mid = read_u16_le(test_data, 0)
assert_eq_int(val16le, 0x0201) fr fr 0x02 << 8 | 0x01
sus val16le2 mid = read_u16_le(test_data, 2)
assert_eq_int(val16le2, 0x0403) fr fr 0x04 << 8 | 0x03

fr fr Test read_u16_be function
test_start("read_u16_be big-endian")
sus val16be mid = read_u16_be(test_data, 0)
assert_eq_int(val16be, 0x0102) fr fr 0x01 << 8 | 0x02
sus val16be2 mid = read_u16_be(test_data, 2)
assert_eq_int(val16be2, 0x0304) fr fr 0x03 << 8 | 0x04

fr fr Test read_u32_le function
test_start("read_u32_le little-endian")
sus val32le normie = read_u32_le(test_data, 0)
assert_eq_int(val32le, 0x04030201) fr fr bytes 0x04, 0x03, 0x02, 0x01
sus val32le2 normie = read_u32_le(test_data, 4)
assert_eq_int(val32le2, 0x08070605) fr fr bytes 0x08, 0x07, 0x06, 0x05

fr fr Test read_u32_be function
test_start("read_u32_be big-endian")
sus val32be normie = read_u32_be(test_data, 0)
assert_eq_int(val32be, 0x01020304) fr fr bytes 0x01, 0x02, 0x03, 0x04
sus val32be2 normie = read_u32_be(test_data, 4)
assert_eq_int(val32be2, 0x05060708) fr fr bytes 0x05, 0x06, 0x07, 0x08

fr fr Test read_u64_le function
test_start("read_u64_le little-endian")
sus val64le thicc = read_u64_le(test_data, 0)
assert_eq_int(val64le, 0x0807060504030201) fr fr 8 bytes little-endian

fr fr Test read_u64_be function
test_start("read_u64_be big-endian")
sus val64be thicc = read_u64_be(test_data, 0)
assert_eq_int(val64be, 0x0102030405060708) fr fr 8 bytes big-endian

fr fr Test write_u8 function
test_start("write_u8 basic functionality")
sus write_data byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success1 lit = write_u8(write_data, 0, 0xAA)
assert_true(success1)
sus check1 byte = read_u8(write_data, 0)
assert_eq_int(check1, 0xAA)

fr fr Test write_u16_le function
test_start("write_u16_le little-endian")
sus write_data_16 byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success2 lit = write_u16_le(write_data_16, 0, 0x1234)
assert_true(success2)
sus check2 mid = read_u16_le(write_data_16, 0)
assert_eq_int(check2, 0x1234)

fr fr Test write_u16_be function
test_start("write_u16_be big-endian")
sus write_data_16be byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success3 lit = write_u16_be(write_data_16be, 0, 0x1234)
assert_true(success3)
sus check3 mid = read_u16_be(write_data_16be, 0)
assert_eq_int(check3, 0x1234)

fr fr Test write_u32_le function
test_start("write_u32_le little-endian")
sus write_data_32 byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success4 lit = write_u32_le(write_data_32, 0, 0x12345678)
assert_true(success4)
sus check4 normie = read_u32_le(write_data_32, 0)
assert_eq_int(check4, 0x12345678)

fr fr Test write_u32_be function
test_start("write_u32_be big-endian")
sus write_data_32be byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success5 lit = write_u32_be(write_data_32be, 0, 0x12345678)
assert_true(success5)
sus check5 normie = read_u32_be(write_data_32be, 0)
assert_eq_int(check5, 0x12345678)

fr fr Test write_u64_le function
test_start("write_u64_le little-endian")
sus write_data_64 byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success6 lit = write_u64_le(write_data_64, 0, 0x123456789ABCDEF0)
assert_true(success6)
sus check6 thicc = read_u64_le(write_data_64, 0)
assert_eq_int(check6, 0x123456789ABCDEF0)

fr fr Test write_u64_be function
test_start("write_u64_be big-endian")
sus write_data_64be byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus success7 lit = write_u64_be(write_data_64be, 0, 0x123456789ABCDEF0)
assert_true(success7)
sus check7 thicc = read_u64_be(write_data_64be, 0)
assert_eq_int(check7, 0x123456789ABCDEF0)

fr fr Test varint_encode function
test_start("varint_encode basic values")
sus encoded1 byte[value] = varint_encode(127)
assert_eq_int(len(encoded1), 1)
sus encoded2 byte[value] = varint_encode(128)
assert_eq_int(len(encoded2), 2)
sus encoded3 byte[value] = varint_encode(16383)
assert_eq_int(len(encoded3), 2)

fr fr Test varint_decode function
test_start("varint_decode basic values")
sus varint_data1 byte[1] = [1]byte{0x7F}
sus decoded1 thicc = varint_decode(varint_data1)
assert_eq_int(decoded1, 127)

sus varint_data2 byte[2] = [2]byte{0x80, 0x01}
sus decoded2 thicc = varint_decode(varint_data2)
assert_eq_int(decoded2, 128)

fr fr Test endianness consistency
test_start("endianness consistency check")
sus test_val normie = 0x12345678
sus temp_data byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}

fr fr Write as little-endian, read as little-endian
write_u32_le(temp_data, 0, test_val)
sus read_le normie = read_u32_le(temp_data, 0)
assert_eq_int(read_le, test_val)

fr fr Write as big-endian, read as big-endian
write_u32_be(temp_data, 0, test_val)
sus read_be normie = read_u32_be(temp_data, 0)
assert_eq_int(read_be, test_val)

fr fr Test boundary conditions
test_start("boundary condition tests")
sus boundary_data byte[4] = [4]byte{0xFF, 0xFF, 0xFF, 0xFF}
sus max_u8 byte = read_u8(boundary_data, 0)
assert_eq_int(max_u8, 0xFF)

sus max_u16_le mid = read_u16_le(boundary_data, 0)
assert_eq_int(max_u16_le, 0xFFFF)

sus max_u32_le normie = read_u32_le(boundary_data, 0)
assert_eq_int(max_u32_le, 0xFFFFFFFF)

fr fr Test offset boundary checking
test_start("offset boundary validation")
sus small_data byte[4] = [4]byte{0x01, 0x02, 0x03, 0x04}
sus out_of_bounds byte = read_u8(small_data, 10)
assert_eq_int(out_of_bounds, 0) fr fr Should return 0 for out of bounds

fr fr Test round-trip operations
test_start("round-trip write/read operations")
sus roundtrip_data byte[8] = [8]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
sus original_val mid = 0x5A3C

write_u16_le(roundtrip_data, 0, original_val)
sus recovered_val mid = read_u16_le(roundtrip_data, 0)
assert_eq_int(recovered_val, original_val)

write_u16_be(roundtrip_data, 2, original_val)
sus recovered_val_be mid = read_u16_be(roundtrip_data, 2)
assert_eq_int(recovered_val_be, original_val)

fr fr Test multiple operations on same buffer
test_start("multiple operations on same buffer")
sus multi_data byte[16] = [16]byte{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00}
write_u8(multi_data, 0, 0xAA)
write_u8(multi_data, 1, 0xBB)
write_u8(multi_data, 2, 0xCC)
write_u8(multi_data, 3, 0xDD)

sus combined_le normie = read_u32_le(multi_data, 0)
assert_eq_int(combined_le, 0xDDCCBBAA)

sus combined_be normie = read_u32_be(multi_data, 0)
assert_eq_int(combined_be, 0xAABBCCDD)

fr fr Test varint edge cases
test_start("varint edge cases")
sus varint_zero byte[value] = varint_encode(0)
assert_eq_int(len(varint_zero), 1)
sus decoded_zero thicc = varint_decode(varint_zero)
assert_eq_int(decoded_zero, 0)

sus varint_large byte[value] = varint_encode(16384)
assert_eq_int(len(varint_large), 3)

fr fr Print test summary
print_test_summary()
