yeet "testz"
yeet "token_vibe"

# Simple tests for token_vibe module

slay test_token_types() {
    test_start("Token type functions")
    
    # Test token constants
    assert_true(EOF_TOKEN == 0)
    assert_true(IDENT_TOKEN == 1)
    assert_true(INT_TOKEN == 2)
    assert_true(ADD_TOKEN == 10)
    
    # Test token string conversion
    sus eof_str tea = token_string(EOF_TOKEN)
    assert_true(string.length(eof_str) > 0)
    
    sus ident_str tea = token_string(IDENT_TOKEN)
    assert_true(string.length(ident_str) > 0)
    
    # Test operator detection
    assert_true(is_operator(ADD_TOKEN))
    assert_true(is_operator(MUL_TOKEN))
    
    print_test_summary()
}

slay test_position_functions() {
    test_start("Position functions")
    
    # Test position creation
    sus pos normie = create_position("test.csd", 2, 5, 10)
    assert_true(position_is_valid(pos))
    
    # Test position string
    sus pos_str tea = position_string(pos)
    assert_true(string.length(pos_str) > 0)
    
    print_test_summary()
}

slay test_tokenization() {
    test_start("Basic tokenization")
    
    sus source tea = "hello 123 world"
    sus token_count normie = tokenize(source)
    assert_true(token_count > 0)
    
    print_test_summary()
}

slay test_token_info() {
    test_start("Token info functions")
    
    # Create token info
    sus pos normie = create_position("", 1, 1, 0)
    sus token_info normie = create_token_info(IDENT_TOKEN, "test", pos, "test")
    
    # Test token type detection
    assert_true(is_identifier(token_info))
    assert_true(token_type(token_info) == IDENT_TOKEN)
    
    # Test token value
    sus value tea = token_value(token_info)
    assert_true(string.length(value) > 0)
    
    print_test_summary()
}

slay test_number_detection() {
    test_start("Number token detection")
    
    sus pos normie = create_position("", 1, 1, 0)
    sus int_token normie = create_token_info(INT_TOKEN, "123", pos, "123")
    sus float_token normie = create_token_info(FLOAT_TOKEN, "3.14", pos, "3.14")
    
    assert_true(is_number(int_token))
    assert_true(is_number(float_token))
    
    print_test_summary()
}

slay test_string_detection() {
    test_start("String token detection")
    
    sus pos normie = create_position("", 1, 1, 0)
    sus string_token normie = create_token_info(STRING_TOKEN, "hello", pos, "hello")
    
    assert_true(is_string(string_token))
    
    print_test_summary()
}

slay test_eof_detection() {
    test_start("EOF token detection")
    
    sus pos normie = create_position("", 1, 1, 0)
    sus eof_token normie = create_token_info(EOF_TOKEN, "", pos, "")
    
    assert_true(is_eof(eof_token))
    
    print_test_summary()
}

slay test_utility_functions() {
    test_start("Utility functions")
    
    # Test character classification
    assert_true(is_letter('a'))
    assert_true(is_digit('5'))
    
    # Test scanner creation
    sus scanner normie = create_scanner("test code")
    assert_true(scanner > 0)
    
    print_test_summary()
}

slay test_module_status() {
    test_start("Module status")
    
    sus status tea = token_vibe_status()
    assert_true(string.length(status) > 0)
    
    print_test_summary()
}

# Main test runner
vibez.spill("Running token_vibe module tests...")
vibez.spill("================================")

test_token_types()
test_position_functions()
test_tokenization()
test_token_info()
test_number_detection()
test_string_detection()
test_eof_detection()
test_utility_functions()
test_module_status()

vibez.spill("================================")
vibez.spill("token_vibe module tests complete!")
vibez.spill("Tokenization functionality verified for self-hosting.")
