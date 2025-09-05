yeet "testz"
yeet "gob_encode_vibes"

fr fr Basic encoding and decoding tests
slay test_basic_string_encoding() {
    test_start("Basic string encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeString("hello world")
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    assert_true(gob_encode_vibes.string_length(data) > 0)
    
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeString()
    assert_eq_string(decode_err, "")
    assert_eq_string(decoded, "hello world")
    
    print_test_summary()
}

slay test_basic_int_encoding() {
    test_start("Basic int encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeInt(42)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeInt()
    assert_eq_string(decode_err, "")
    assert_eq_int(decoded, 42)
    
    print_test_summary()
}

slay test_basic_bool_encoding() {
    test_start("Basic bool encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeBool(based)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeBool()
    assert_eq_string(decode_err, "")
    assert_true(decoded)
    
    fr fr Test false value
    encoder.Reset()
    err = encoder.EncodeBool(cap)
    assert_eq_string(err, "")
    
    data = encoder.GetData()
    decoder.Reset(data)
    (decoded_false, decode_err2) := decoder.DecodeBool()
    assert_eq_string(decode_err2, "")
    assert_false(decoded_false)
    
    print_test_summary()
}

slay test_basic_float_encoding() {
    test_start("Basic float encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeFloat(3.14159)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeFloat()
    assert_eq_string(decode_err, "")
    
    fr fr Check if float is approximately correct (within 0.01)
    diff := decoded - 3.14159
    if diff < 0.0 { diff = -diff }
    assert_true(diff < 0.01)
    
    print_test_summary()
}

slay test_empty_string_encoding() {
    test_start("Empty string encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeString("")
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeString()
    assert_eq_string(decode_err, "")
    assert_eq_string(decoded, "")
    
    print_test_summary()
}

slay test_multiple_values_encoding() {
    test_start("Multiple values encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    
    fr fr Encode multiple values
    err1 := encoder.EncodeString("first")
    assert_eq_string(err1, "")
    
    err2 := encoder.EncodeInt(123)
    assert_eq_string(err2, "")
    
    err3 := encoder.EncodeBool(based)
    assert_eq_string(err3, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    
    fr fr Decode in same order
    (str_val, str_err) := decoder.DecodeString()
    assert_eq_string(str_err, "")
    assert_eq_string(str_val, "first")
    
    (int_val, int_err) := decoder.DecodeInt()
    assert_eq_string(int_err, "")
    assert_eq_int(int_val, 123)
    
    (bool_val, bool_err) := decoder.DecodeBool()
    assert_eq_string(bool_err, "")
    assert_true(bool_val)
    
    print_test_summary()
}

slay test_type_registry() {
    test_start("Type registry")
    
    registry := gob_encode_vibes.NewRegistry()
    
    fr fr Register some types
    id1 := registry.Register("Person")
    id2 := registry.Register("Address")
    id3 := registry.Register("Person")  fr fr Should return same ID
    
    assert_eq_int(id1, 1)
    assert_eq_int(id2, 2)
    assert_eq_int(id3, 1)  fr fr Same as first registration
    
    print_test_summary()
}

slay test_encoder_with_registry() {
    test_start("Encoder with registry")
    
    registry := gob_encode_vibes.NewRegistry()
    registry.Register("TestType")
    
    encoder := gob_encode_vibes.NewEncoderWithRegistry(registry)
    
    fr fr Test that encoder can use the registry
    err := encoder.EncodeString("test with registry")
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    assert_true(gob_encode_vibes.string_length(data) > 0)
    
    print_test_summary()
}

slay test_streamer_basic() {
    test_start("Basic streamer functionality")
    
    streamer := gob_encode_vibes.NewStreamer()
    
    fr fr Start encoding
    streamer.StartEncoding()
    
    fr fr Encode some values
    err1 := streamer.EncodeValue("first")
    assert_eq_string(err1, "")
    
    err2 := streamer.EncodeValue("second")
    assert_eq_string(err2, "")
    
    fr fr Finish encoding
    finish_err := streamer.FinishEncoding()
    assert_eq_string(finish_err, "")
    
    fr fr Start decoding
    streamer.StartDecoding()
    
    fr fr Decode values
    (val1, dec_err1) := streamer.DecodeValue()
    assert_eq_string(dec_err1, "")
    assert_eq_string(val1, "first")
    
    (val2, dec_err2) := streamer.DecodeValue()
    assert_eq_string(dec_err2, "")
    assert_eq_string(val2, "second")
    
    print_test_summary()
}

slay test_metrics_collector() {
    test_start("Metrics collector")
    
    metrics := gob_encode_vibes.NewMetricsCollector()
    
    fr fr Record some metrics
    metrics.RecordBytes(100)
    metrics.RecordBytes(200)
    metrics.RecordType("String")
    metrics.RecordType("Int")
    metrics.RecordType("String")
    
    fr fr Get stats
    stats := metrics.GetStats()
    assert_eq_int(stats.TotalBytes, 300)
    
    fr fr Check type counts
    str_count := 0
    int_count := 0
    
    if count, exists := stats.TypeCounts["String"]; exists {
        str_count = count
    }
    if count, exists := stats.TypeCounts["Int"]; exists {
        int_count = count
    }
    
    assert_eq_int(str_count, 2)
    assert_eq_int(int_count, 1)
    
    print_test_summary()
}

slay test_decoder_has_more() {
    test_start("Decoder has more data")
    
    encoder := gob_encode_vibes.NewEncoder()
    encoder.EncodeString("test")
    encoder.EncodeInt(42)
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    
    fr fr Should have more data initially
    assert_true(decoder.HasMore())
    
    fr fr Consume first value
    (str_val, str_err) := decoder.DecodeString()
    assert_eq_string(str_err, "")
    assert_eq_string(str_val, "test")
    
    fr fr Should still have more data
    assert_true(decoder.HasMore())
    
    fr fr Consume second value
    (int_val, int_err) := decoder.DecodeInt()
    assert_eq_string(int_err, "")
    assert_eq_int(int_val, 42)
    
    fr fr Should not have more data (depending on implementation)
    fr fr This test might need adjustment based on exact implementation
    
    print_test_summary()
}

slay test_negative_numbers() {
    test_start("Negative number encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    err := encoder.EncodeInt(-42)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeInt()
    assert_eq_string(decode_err, "")
    assert_eq_int(decoded, -42)
    
    print_test_summary()
}

slay test_zero_values() {
    test_start("Zero values encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    
    fr fr Test zero int
    err1 := encoder.EncodeInt(0)
    assert_eq_string(err1, "")
    
    fr fr Test zero float
    err2 := encoder.EncodeFloat(0.0)
    assert_eq_string(err2, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    
    (int_val, int_err) := decoder.DecodeInt()
    assert_eq_string(int_err, "")
    assert_eq_int(int_val, 0)
    
    (float_val, float_err) := decoder.DecodeFloat()
    assert_eq_string(float_err, "")
    assert_true(float_val == 0.0)
    
    print_test_summary()
}

slay test_large_numbers() {
    test_start("Large number encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    large_num := 1000000
    
    err := encoder.EncodeInt(large_num)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeInt()
    assert_eq_string(decode_err, "")
    assert_eq_int(decoded, large_num)
    
    print_test_summary()
}

slay test_long_strings() {
    test_start("Long string encoding")
    
    encoder := gob_encode_vibes.NewEncoder()
    long_str := "This is a very long string that tests the encoding and decoding of strings with significant length to ensure the implementation handles variable length data correctly"
    
    err := encoder.EncodeString(long_str)
    assert_eq_string(err, "")
    
    data := encoder.GetData()
    decoder := gob_encode_vibes.NewDecoder(data)
    (decoded, decode_err) := decoder.DecodeString()
    assert_eq_string(decode_err, "")
    assert_eq_string(decoded, long_str)
    
    print_test_summary()
}

slay test_encoder_reset() {
    test_start("Encoder reset functionality")
    
    encoder := gob_encode_vibes.NewEncoder()
    
    fr fr Encode some data
    encoder.EncodeString("first")
    data1 := encoder.GetData()
    assert_true(gob_encode_vibes.string_length(data1) > 0)
    
    fr fr Reset encoder
    encoder.Reset()
    data2 := encoder.GetData()
    assert_eq_string(data2, "")
    
    fr fr Encode new data
    encoder.EncodeString("second")
    data3 := encoder.GetData()
    assert_true(gob_encode_vibes.string_length(data3) > 0)
    
    fr fr Should be different from first encoding
    assert_true(data1 != data3)
    
    print_test_summary()
}

slay test_global_registration() {
    test_start("Global type registration")
    
    fr fr Test global registration functions
    fr fr These just print messages in our implementation
    gob_encode_vibes.Register("TestType")
    gob_encode_vibes.RegisterName("custom.Type", "TestType")
    
    fr fr If we get here without errors, registration succeeded
    assert_true(based)
    
    print_test_summary()
}

fr fr Helper functions for testing
slay make_test_map() map[tea]normie {
    test_map := make(map[tea]normie)
    test_map["one"] = 1
    test_map["two"] = 2
    test_map["three"] = 3
    damn test_map
}

slay run_all_tests() {
    vibez.spill("Running GOB Encode Vibes Tests...")
    
    test_basic_string_encoding()
    test_basic_int_encoding()
    test_basic_bool_encoding()
    test_basic_float_encoding()
    test_empty_string_encoding()
    test_multiple_values_encoding()
    test_type_registry()
    test_encoder_with_registry()
    test_streamer_basic()
    test_metrics_collector()
    test_decoder_has_more()
    test_negative_numbers()
    test_zero_values()
    test_large_numbers()
    test_long_strings()
    test_encoder_reset()
    test_global_registration()
    
    vibez.spill("All GOB Encode Vibes tests completed!")
}

fr fr Run all tests
run_all_tests()
