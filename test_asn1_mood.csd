yeet "testz"
yeet "asn1_mood"

# Comprehensive ASN.1 Module Test Suite
# Tests all ASN.1 encoding/decoding functionality
# Uses testz v2.0 testing framework

# Test ASN.1 tag creation
slay test_asn1_tag_creation() {
    test_start("ASN.1 Tag Creation")
    
    sus tag ASN1Tag = asn1_tag_new(ASN1_UNIVERSAL, cap, ASN1_INTEGER)
    assert_eq_int(tag.class, ASN1_UNIVERSAL)
    assert_false(tag.constructed)
    assert_eq_int(tag.tag_number, ASN1_INTEGER)
    
    sus seq_tag ASN1Tag = asn1_tag_new(ASN1_UNIVERSAL, based, ASN1_SEQUENCE)
    assert_eq_int(seq_tag.class, ASN1_UNIVERSAL)
    assert_true(seq_tag.constructed)
    assert_eq_int(seq_tag.tag_number, ASN1_SEQUENCE)
}

# Test ASN.1 integer object creation
slay test_asn1_integer_creation() {
    test_start("ASN.1 Integer Object Creation")
    
    sus int_obj ASN1Object = asn1_int_new(42)
    assert_eq_int(int_obj.tag.class, ASN1_UNIVERSAL)
    assert_false(int_obj.tag.constructed)
    assert_eq_int(int_obj.tag.tag_number, ASN1_INTEGER)
    assert_true(int_obj.length > 0)
    
    sus zero_obj ASN1Object = asn1_int_new(0)
    assert_eq_int(zero_obj.tag.tag_number, ASN1_INTEGER)
    assert_true(zero_obj.length > 0)
}

# Test ASN.1 string object creation
slay test_asn1_string_creation() {
    test_start("ASN.1 String Object Creation")
    
    sus str_obj ASN1Object = asn1_string_new("hello")
    assert_eq_int(str_obj.tag.class, ASN1_UNIVERSAL)
    assert_false(str_obj.tag.constructed)
    assert_eq_int(str_obj.tag.tag_number, ASN1_OCTET_STRING)
    assert_eq_string(str_obj.data, "hello")
    
    sus empty_obj ASN1Object = asn1_string_new("")
    assert_eq_int(empty_obj.tag.tag_number, ASN1_OCTET_STRING)
    assert_eq_int(empty_obj.length, 0)
}

# Test ASN.1 sequence creation
slay test_asn1_sequence_creation() {
    test_start("ASN.1 Sequence Creation")
    
    sus seq_obj ASN1Object = asn1_sequence_new()
    assert_eq_int(seq_obj.tag.class, ASN1_UNIVERSAL)
    assert_true(seq_obj.tag.constructed)
    assert_eq_int(seq_obj.tag.tag_number, ASN1_SEQUENCE)
    assert_eq_int(seq_obj.length, 0)
}

# Test ASN.1 set creation
slay test_asn1_set_creation() {
    test_start("ASN.1 Set Creation")
    
    sus set_obj ASN1Object = asn1_set_new()
    assert_eq_int(set_obj.tag.class, ASN1_UNIVERSAL)
    assert_true(set_obj.tag.constructed)
    assert_eq_int(set_obj.tag.tag_number, ASN1_SET)
    assert_eq_int(set_obj.length, 0)
}

# Test ASN.1 OID creation
slay test_asn1_oid_creation() {
    test_start("ASN.1 OID Creation")
    
    sus oid_obj ASN1Object = asn1_oid_new("1.2.3.4")
    assert_eq_int(oid_obj.tag.class, ASN1_UNIVERSAL)
    assert_false(oid_obj.tag.constructed)
    assert_eq_int(oid_obj.tag.tag_number, ASN1_OBJECT_IDENTIFIER)
    assert_true(oid_obj.length > 0)
    
    sus simple_oid ASN1Object = asn1_oid_new("1.2")
    assert_eq_int(simple_oid.tag.tag_number, ASN1_OBJECT_IDENTIFIER)
    assert_true(simple_oid.length > 0)
}

# Test ASN.1 time creation
slay test_asn1_time_creation() {
    test_start("ASN.1 Time Creation")
    
    sus time_obj ASN1Object = asn1_time_new("20231207120000Z")
    assert_eq_int(time_obj.tag.class, ASN1_UNIVERSAL)
    assert_false(time_obj.tag.constructed)
    assert_eq_int(time_obj.tag.tag_number, ASN1_UTC_TIME)
    assert_eq_string(time_obj.data, "20231207120000Z")
}

# Test ASN.1 bit string creation
slay test_asn1_bitstring_creation() {
    test_start("ASN.1 Bit String Creation")
    
    sus bits_obj ASN1Object = asn1_bitstring_new("10110100")
    assert_eq_int(bits_obj.tag.class, ASN1_UNIVERSAL)
    assert_false(bits_obj.tag.constructed)
    assert_eq_int(bits_obj.tag.tag_number, ASN1_BIT_STRING)
    assert_eq_string(bits_obj.data, "10110100")
}

# Test ASN.1 DER encoding
slay test_asn1_der_encoding() {
    test_start("ASN.1 DER Encoding")
    
    sus int_obj ASN1Object = asn1_int_new(42)
    sus encoded tea = asn1_encode_der(int_obj)
    assert_true(string_length(encoded) > 0)
    
    sus str_obj ASN1Object = asn1_string_new("test")
    sus encoded_str tea = asn1_encode_der(str_obj)
    assert_true(string_length(encoded_str) > 0)
}

# Test ASN.1 BER encoding
slay test_asn1_ber_encoding() {
    test_start("ASN.1 BER Encoding")
    
    sus int_obj ASN1Object = asn1_int_new(123)
    sus encoded tea = asn1_encode_ber(int_obj)
    assert_true(string_length(encoded) > 0)
    
    sus seq_obj ASN1Object = asn1_sequence_new()
    sus encoded_seq tea = asn1_encode_ber(seq_obj)
    assert_true(string_length(encoded_seq) > 0)
}

# Test generic ASN.1 encoding
slay test_asn1_generic_encoding() {
    test_start("ASN.1 Generic Encoding")
    
    sus obj ASN1Object = asn1_int_new(255)
    sus encoded tea = asn1_encode(obj)
    assert_true(string_length(encoded) > 0)
    
    sus str_obj ASN1Object = asn1_string_new("encoded")
    sus encoded_str tea = asn1_encode(str_obj)
    assert_true(string_length(encoded_str) > 0)
}

# Test ASN.1 DER parsing
slay test_asn1_der_parsing() {
    test_start("ASN.1 DER Parsing")
    
    # Create and encode an object
    sus original ASN1Object = asn1_int_new(42)
    sus encoded tea = asn1_encode_der(original)
    
    # Parse the encoded data
    sus parsed ASN1Object = asn1_parse_der(encoded)
    assert_eq_int(parsed.tag.class, ASN1_UNIVERSAL)
    assert_eq_int(parsed.tag.tag_number, ASN1_INTEGER)
    assert_false(parsed.tag.constructed)
}

# Test ASN.1 BER parsing
slay test_asn1_ber_parsing() {
    test_start("ASN.1 BER Parsing")
    
    # Create and encode an object
    sus original ASN1Object = asn1_string_new("test")
    sus encoded tea = asn1_encode_ber(original)
    
    # Parse the encoded data
    sus parsed ASN1Object = asn1_parse_ber(encoded)
    assert_eq_int(parsed.tag.class, ASN1_UNIVERSAL)
    assert_eq_int(parsed.tag.tag_number, ASN1_OCTET_STRING)
    assert_false(parsed.tag.constructed)
}

# Test generic ASN.1 decoding
slay test_asn1_generic_decoding() {
    test_start("ASN.1 Generic Decoding")
    
    # Create and encode an object
    sus original ASN1Object = asn1_sequence_new()
    sus encoded tea = asn1_encode(original)
    
    # Decode the encoded data
    sus decoded ASN1Object = asn1_decode(encoded)
    assert_eq_int(decoded.tag.class, ASN1_UNIVERSAL)
    assert_eq_int(decoded.tag.tag_number, ASN1_SEQUENCE)
    assert_true(decoded.tag.constructed)
}

# Test ASN.1 tag constants
slay test_asn1_tag_constants() {
    test_start("ASN.1 Tag Constants")
    
    assert_eq_int(ASN1_UNIVERSAL, 0)
    assert_eq_int(ASN1_APPLICATION, 1)
    assert_eq_int(ASN1_CONTEXT_SPECIFIC, 2)
    assert_eq_int(ASN1_PRIVATE, 3)
    
    assert_eq_int(ASN1_INTEGER, 2)
    assert_eq_int(ASN1_OCTET_STRING, 4)
    assert_eq_int(ASN1_NULL, 5)
    assert_eq_int(ASN1_OBJECT_IDENTIFIER, 6)
    assert_eq_int(ASN1_SEQUENCE, 16)
    assert_eq_int(ASN1_SET, 17)
    assert_eq_int(ASN1_BIT_STRING, 3)
}

# Test ASN.1 length encoding
slay test_asn1_length_encoding() {
    test_start("ASN.1 Length Encoding")
    
    # Test short length encoding (< 128)
    sus short_len tea = encode_length(42)
    assert_true(string_length(short_len) == 1)
    
    # Test long length encoding (>= 128)
    sus long_len tea = encode_length(256)
    assert_true(string_length(long_len) > 1)
    
    # Test zero length
    sus zero_len tea = encode_length(0)
    assert_true(string_length(zero_len) == 1)
}

# Test ASN.1 integer encoding
slay test_asn1_integer_encoding() {
    test_start("ASN.1 Integer Encoding")
    
    # Test positive integer
    sus pos_int tea = encode_integer(42)
    assert_true(string_length(pos_int) > 0)
    
    # Test zero
    sus zero_int tea = encode_integer(0)
    assert_true(string_length(zero_int) > 0)
    
    # Test negative integer
    sus neg_int tea = encode_integer(-42)
    assert_true(string_length(neg_int) > 0)
}

# Test ASN.1 OID encoding
slay test_asn1_oid_encoding() {
    test_start("ASN.1 OID Encoding")
    
    sus oid_encoded tea = encode_oid("1.2.3.4")
    assert_true(string_length(oid_encoded) > 0)
    
    sus simple_oid tea = encode_oid("1.2")
    assert_true(string_length(simple_oid) > 0)
    
    sus complex_oid tea = encode_oid("1.2.840.113549.1.1.1")
    assert_true(string_length(complex_oid) > 0)
}

# Test ASN.1 comprehensive encoding/decoding
slay test_asn1_comprehensive() {
    test_start("ASN.1 Comprehensive Test")
    
    # Test multiple object types
    sus int_obj ASN1Object = asn1_int_new(42)
    sus str_obj ASN1Object = asn1_string_new("hello")
    sus seq_obj ASN1Object = asn1_sequence_new()
    sus oid_obj ASN1Object = asn1_oid_new("1.2.3")
    
    # Encode all objects
    sus encoded_int tea = asn1_encode(int_obj)
    sus encoded_str tea = asn1_encode(str_obj)
    sus encoded_seq tea = asn1_encode(seq_obj)
    sus encoded_oid tea = asn1_encode(oid_obj)
    
    # Verify all encodings are non-empty
    assert_true(string_length(encoded_int) > 0)
    assert_true(string_length(encoded_str) > 0)
    assert_true(string_length(encoded_seq) > 0)
    assert_true(string_length(encoded_oid) > 0)
    
    # Decode all objects
    sus decoded_int ASN1Object = asn1_decode(encoded_int)
    sus decoded_str ASN1Object = asn1_decode(encoded_str)
    sus decoded_seq ASN1Object = asn1_decode(encoded_seq)
    sus decoded_oid ASN1Object = asn1_decode(encoded_oid)
    
    # Verify tag types match
    assert_eq_int(decoded_int.tag.tag_number, ASN1_INTEGER)
    assert_eq_int(decoded_str.tag.tag_number, ASN1_OCTET_STRING)
    assert_eq_int(decoded_seq.tag.tag_number, ASN1_SEQUENCE)
    assert_eq_int(decoded_oid.tag.tag_number, ASN1_OBJECT_IDENTIFIER)
}

# Test ASN.1 error handling
slay test_asn1_error_handling() {
    test_start("ASN.1 Error Handling")
    
    # Test empty OID
    sus empty_oid ASN1Object = asn1_oid_new("")
    assert_eq_int(empty_oid.tag.tag_number, ASN1_OBJECT_IDENTIFIER)
    
    # Test single component OID (should fail gracefully)
    sus single_oid ASN1Object = asn1_oid_new("1")
    assert_eq_int(single_oid.tag.tag_number, ASN1_OBJECT_IDENTIFIER)
    
    # Test empty string
    sus empty_str ASN1Object = asn1_string_new("")
    assert_eq_int(empty_str.length, 0)
}

# Run all ASN.1 tests
slay run_asn1_tests() {
    vibez.spill("Running ASN.1 Module Tests...")
    
    test_asn1_tag_creation()
    test_asn1_integer_creation()
    test_asn1_string_creation()
    test_asn1_sequence_creation()
    test_asn1_set_creation()
    test_asn1_oid_creation()
    test_asn1_time_creation()
    test_asn1_bitstring_creation()
    test_asn1_der_encoding()
    test_asn1_ber_encoding()
    test_asn1_generic_encoding()
    test_asn1_der_parsing()
    test_asn1_ber_parsing()
    test_asn1_generic_decoding()
    test_asn1_tag_constants()
    test_asn1_length_encoding()
    test_asn1_integer_encoding()
    test_asn1_oid_encoding()
    test_asn1_comprehensive()
    test_asn1_error_handling()
    
    print_test_summary()
}

# Main test execution
run_asn1_tests()
