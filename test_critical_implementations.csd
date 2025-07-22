yeet "testz"
yeet "string_simple"
yeet "clock_bait"
yeet "asn1_mood"

fr fr Test the critical implementations we just fixed

test_start("Critical Implementation Tests")

fr fr Test string formatting
sus float_result tea = string_format_float(42.75)
vibez.spill("Float format test: " + float_result)
assert_true(string_length(float_result) > 0)

fr fr Test character extraction
sus char_result sip = string_char_at("hello", 0)
vibez.spill("Character extraction test completed")
assert_true(char_result != '\0')

fr fr Test substring extraction  
sus substr_result tea = string_substring("hello world", 0, 5)
vibez.spill("Substring test: " + substr_result)
assert_true(string_length(substr_result) > 0)

fr fr Test byte to string conversion
sus byte_str tea = string_from_byte(65)
vibez.spill("Byte to string test: " + byte_str)
assert_true(string_length(byte_str) > 0)

fr fr Test sleep function (quick test)
vibez.spill("Testing sleep function...")
Sleep(1000000) fr fr Sleep for 1ms
vibez.spill("Sleep completed")

fr fr Test memory monitoring
test_memory_usage("basic memory test", slay() {
    sus test_var normie = 42
    test_var = test_var + 1
}, 100)

print_test_summary()
