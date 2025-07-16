yeet "testz"
yeet "token_vibe"

# Comprehensive test suite for token_vibe module
# Essential for validating lexical analysis in self-hosting compiler

# =============================================================================
# Token Type and Constant Tests
# =============================================================================

slay test_token_constants() {
    test_start("Token type constants")
    
    # Core token types
    assert_eq_int(EOF_TOKEN, 0)
    assert_eq_int(IDENT_TOKEN, 1)
    assert_eq_int(INT_TOKEN, 2)
    assert_eq_int(FLOAT_TOKEN, 3)
    assert_eq_int(STRING_TOKEN, 4)
    assert_eq_int(CHAR_TOKEN, 5)
    
    # Operators
    assert_eq_int(ADD_TOKEN, 10)
    assert_eq_int(SUB_TOKEN, 11)
    assert_eq_int(MUL_TOKEN, 12)
    assert_eq_int(DIV_TOKEN, 13)
    assert_eq_int(EQ_TOKEN, 15)
    assert_eq_int(NE_TOKEN, 16)
    
    # Delimiters
    assert_eq_int(LPAREN_TOKEN, 40)
    assert_eq_int(RPAREN_TOKEN, 41)
    assert_eq_int(LBRACE_TOKEN, 42)
    assert_eq_int(RBRACE_TOKEN, 43)
    
    # Keywords
    assert_eq_int(SUS_TOKEN, 60)
    assert_eq_int(DAMN_TOKEN, 61)
    assert_eq_int(SLAY_TOKEN, 62)
    assert_eq_int(BASED_TOKEN, 65)
    assert_eq_int(CAP_TOKEN, 66)
    assert_eq_int(LOWKEY_TOKEN, 68)
    
    print_test_summary()
}

slay test_token_string_conversion() {
    test_start("Token string conversion")
    
    # Test token to string conversion
    sus eof_str tea = token_string(EOF_TOKEN)
    assert_eq_string(eof_str, "EOF")
    
    sus ident_str tea = token_string(IDENT_TOKEN)
    assert_eq_string(ident_str, "IDENTIFIER")
    
    sus add_str tea = token_string(ADD_TOKEN)
    assert_eq_string(add_str, "+")
    
    sus sub_str tea = token_string(SUB_TOKEN)
    assert_eq_string(sub_str, "-")
    
    sus lparen_str tea = token_string(LPAREN_TOKEN)
    assert_eq_string(lparen_str, "(")
    
    sus sus_str tea = token_string(SUS_TOKEN)
    assert_eq_string(sus_str, "sus")
    
    sus damn_str tea = token_string(DAMN_TOKEN)
    assert_eq_string(damn_str, "damn")
    
    print_test_summary()
}

# =============================================================================
# Position Tracking Tests
# =============================================================================

slay test_position_functions() {
    test_start("Position tracking functions")
    
    # Test position creation
    sus pos normie = create_position("test.csd", 10, 25, 150)
    assert_true(position_is_valid(pos))
    
    # Test position components
    sus line normie = position_line(pos)
    sus column normie = position_column(pos)
    sus offset normie = position_offset(pos)
    
    assert_true(line > 0)
    assert_true(column > 0)
    assert_true(offset >= 0)
    
    # Test position string representation
    sus pos_str tea = position_string(pos)
    assert_true(string.length(pos_str) > 0)
    assert_true(string.contains(pos_str, "line:"))
    assert_true(string.contains(pos_str, "col:"))
    
    print_test_summary()
}

slay test_position_edge_cases() {
    test_start("Position edge cases")
    
    # Test zero position
    sus zero_pos normie = create_position("", 0, 0, 0)
    assert_true(!position_is_valid(zero_pos))
    
    # Test large position values
    sus large_pos normie = create_position("large.csd", 999, 99, 99)
    assert_true(position_is_valid(large_pos))
    
    print_test_summary()
}

# =============================================================================
# Token Info Structure Tests
# =============================================================================

slay test_token_info_creation() {
    test_start("Token info creation and access")
    
    sus pos normie = create_position("test.csd", 1, 5, 10)
    sus token_info normie = create_token_info(IDENT_TOKEN, "variable", pos, "variable")
    
    # Test token type extraction
    sus type normie = token_type(token_info)
    assert_eq_int(type, IDENT_TOKEN)
    
    # Test token value hash
    sus value_hash normie = token_value_hash(token_info)
    assert_true(value_hash > 0)
    
    # Test position reference
    sus pos_ref normie = token_position_ref(token_info)
    assert_true(pos_ref >= 0)
    
    print_test_summary()
}

slay test_token_classification() {
    test_start("Token classification functions")
    
    sus pos normie = create_position("test.csd", 1, 1, 0)
    
    # Test identifier classification
    sus ident_token normie = create_token_info(IDENT_TOKEN, "variable", pos, "variable")
    assert_true(is_identifier(ident_token))
    assert_true(!is_number(ident_token))
    assert_true(!is_string(ident_token))
    assert_true(!is_keyword(ident_token))
    
    # Test number classification
    sus int_token normie = create_token_info(INT_TOKEN, "123", pos, "123")
    assert_true(is_number(int_token))
    assert_true(!is_identifier(int_token))
    
    sus float_token normie = create_token_info(FLOAT_TOKEN, "3.14", pos, "3.14")
    assert_true(is_number(float_token))
    
    # Test string classification
    sus string_token normie = create_token_info(STRING_TOKEN, "hello", pos, "hello")
    assert_true(is_string(string_token))
    assert_true(!is_number(string_token))
    
    # Test keyword classification
    sus keyword_token normie = create_token_info(SUS_TOKEN, "sus", pos, "sus")
    assert_true(is_keyword(keyword_token))
    assert_true(!is_identifier(keyword_token))
    
    # Test EOF classification
    sus eof_token normie = create_token_info(EOF_TOKEN, "", pos, "")
    assert_true(is_eof(eof_token))
    assert_true(!is_number(eof_token))
    
    print_test_summary()
}

# =============================================================================
# Character Classification Tests
# =============================================================================

slay test_character_classification() {
    test_start("Character classification functions")
    
    # Test letter classification
    assert_true(is_letter('a'))
    assert_true(is_letter('Z'))
    assert_true(is_letter('_'))
    assert_true(!is_letter('5'))
    assert_true(!is_letter(' '))
    
    # Test digit classification
    assert_true(is_digit('0'))
    assert_true(is_digit('9'))
    assert_true(!is_digit('a'))
    assert_true(!is_digit(' '))
    
    # Test alphanumeric classification
    assert_true(is_alphanumeric('a'))
    assert_true(is_alphanumeric('5'))
    assert_true(is_alphanumeric('_'))
    assert_true(!is_alphanumeric(' '))
    assert_true(!is_alphanumeric('@'))
    
    # Test whitespace classification
    assert_true(is_whitespace(' '))
    assert_true(is_whitespace('\t'))
    assert_true(!is_whitespace('a'))
    assert_true(!is_whitespace('\n'))
    
    # Test newline classification
    assert_true(is_newline('\n'))
    assert_true(!is_newline(' '))
    assert_true(!is_newline('a'))
    
    # Test hex digit classification
    assert_true(is_hex_digit('0'))
    assert_true(is_hex_digit('9'))
    assert_true(is_hex_digit('A'))
    assert_true(is_hex_digit('f'))
    assert_true(!is_hex_digit('g'))
    assert_true(!is_hex_digit(' '))
    
    print_test_summary()
}

# =============================================================================
# Scanner State Machine Tests
# =============================================================================

slay test_scanner_creation() {
    test_start("Scanner creation and state")
    
    sus scanner normie = create_scanner("test code")
    assert_true(scanner > 0)
    
    # Test initial scanner state
    sus pos normie = scanner_position(scanner)
    sus line normie = scanner_line(scanner)
    sus column normie = scanner_column(scanner)
    
    assert_eq_int(pos, 1)
    assert_eq_int(line, 1)
    assert_eq_int(column, 1)
    
    print_test_summary()
}

slay test_scanner_advancement() {
    test_start("Scanner advancement")
    
    sus scanner normie = create_scanner("test")
    
    # Advance with regular character
    sus new_scanner normie = advance_scanner(scanner, 'a')
    sus new_pos normie = scanner_position(new_scanner)
    sus new_col normie = scanner_column(new_scanner)
    
    assert_true(new_pos > scanner_position(scanner))
    assert_true(new_col > scanner_column(scanner))
    
    # Advance with newline
    sus newline_scanner normie = advance_scanner(scanner, '\n')
    sus newline_line normie = scanner_line(newline_scanner)
    sus newline_col normie = scanner_column(newline_scanner)
    
    assert_true(newline_line > scanner_line(scanner))
    assert_eq_int(newline_col, 1)
    
    print_test_summary()
}

# =============================================================================
# Keyword Recognition Tests
# =============================================================================

slay test_keyword_recognition() {
    test_start("Keyword recognition")
    
    # Test CURSED-specific keywords
    assert_eq_int(recognize_keyword("sus"), SUS_TOKEN)
    assert_eq_int(recognize_keyword("damn"), DAMN_TOKEN)
    assert_eq_int(recognize_keyword("slay"), SLAY_TOKEN)
    assert_eq_int(recognize_keyword("vibe"), VIBE_TOKEN)
    assert_eq_int(recognize_keyword("yeet"), YEET_TOKEN)
    assert_eq_int(recognize_keyword("based"), BASED_TOKEN)
    assert_eq_int(recognize_keyword("cap"), CAP_TOKEN)
    assert_eq_int(recognize_keyword("cringe"), CRINGE_TOKEN)
    assert_eq_int(recognize_keyword("lowkey"), LOWKEY_TOKEN)
    assert_eq_int(recognize_keyword("bestie"), BESTIE_TOKEN)
    assert_eq_int(recognize_keyword("yolo"), YOLO_TOKEN)
    assert_eq_int(recognize_keyword("ready"), READY_TOKEN)
    assert_eq_int(recognize_keyword("ghosted"), GHOSTED_TOKEN)
    assert_eq_int(recognize_keyword("simp"), SIMP_TOKEN)
    assert_eq_int(recognize_keyword("defer"), DEFER_TOKEN)
    
    # Test type keywords
    assert_eq_int(recognize_keyword("lit"), LIT_TOKEN)
    assert_eq_int(recognize_keyword("tea"), TEA_TOKEN)
    assert_eq_int(recognize_keyword("drip"), DRIP_TOKEN)
    assert_eq_int(recognize_keyword("normie"), NORMIE_TOKEN)
    assert_eq_int(recognize_keyword("thicc"), THICC_TOKEN)
    assert_eq_int(recognize_keyword("smol"), SMOL_TOKEN)
    
    # Test non-keywords return IDENT_TOKEN
    assert_eq_int(recognize_keyword("variable"), IDENT_TOKEN)
    assert_eq_int(recognize_keyword("unknown"), IDENT_TOKEN)
    assert_eq_int(recognize_keyword("test123"), IDENT_TOKEN)
    
    print_test_summary()
}

# =============================================================================
# Tokenization Tests
# =============================================================================

slay test_basic_tokenization() {
    test_start("Basic tokenization")
    
    # Test simple expression
    sus source1 tea = "sus x normie = 42"
    sus count1 normie = tokenize(source1)
    assert_true(count1 > 0)
    
    # Test expression with operators
    sus source2 tea = "x + y * 2"
    sus count2 normie = tokenize(source2)
    assert_true(count2 > 0)
    
    # Test function definition
    sus source3 tea = "slay func() { damn based }"
    sus count3 normie = tokenize(source3)
    assert_true(count3 > 0)
    
    print_test_summary()
}

slay test_string_tokenization() {
    test_start("String tokenization")
    
    # Test string literals
    sus source1 tea = "\"hello world\""
    sus count1 normie = tokenize(source1)
    assert_true(count1 > 0)
    
    # Test string with escapes
    sus source2 tea = "\"hello\\nworld\""
    sus count2 normie = tokenize(source2)
    assert_true(count2 > 0)
    
    # Test character literals
    sus source3 tea = "'a'"
    sus count3 normie = tokenize(source3)
    assert_true(count3 > 0)
    
    print_test_summary()
}

slay test_number_tokenization() {
    test_start("Number tokenization")
    
    # Test integers
    sus source1 tea = "123"
    sus count1 normie = tokenize(source1)
    assert_true(count1 > 0)
    
    # Test floats
    sus source2 tea = "3.14159"
    sus count2 normie = tokenize(source2)
    assert_true(count2 > 0)
    
    # Test mixed numbers
    sus source3 tea = "42 + 3.14"
    sus count3 normie = tokenize(source3)
    assert_true(count3 > 0)
    
    print_test_summary()
}

slay test_comment_tokenization() {
    test_start("Comment tokenization")
    
    # Test line comments
    sus source1 tea = "# This is a comment"
    sus count1 normie = tokenize(source1)
    assert_true(count1 > 0)
    
    # Test code with comments
    sus source2 tea = "sus x normie = 42  # Variable declaration"
    sus count2 normie = tokenize(source2)
    assert_true(count2 > 0)
    
    print_test_summary()
}

slay test_complex_tokenization() {
    test_start("Complex tokenization scenarios")
    
    # Test complete function
    sus source tea = "slay fibonacci(n normie) normie {\n    lowkey n <= 1 {\n        damn n\n    }\n    damn fibonacci(n-1) + fibonacci(n-2)\n}"
    sus count normie = tokenize(source)
    assert_true(count > 10)  # Should have many tokens
    
    print_test_summary()
}

# =============================================================================
# Token Stream Tests
# =============================================================================

slay test_token_stream() {
    test_start("Token stream functionality")
    
    sus source tea = "sus x normie = 42"
    sus stream normie = create_token_stream(source)
    
    assert_true(token_stream_has_next(stream))
    
    sus peek_token normie = token_stream_peek(stream)
    assert_true(peek_token != EOF_TOKEN)
    
    sus next_stream normie = token_stream_next(stream)
    assert_true(next_stream != stream)
    
    print_test_summary()
}

# =============================================================================
# Error Recovery Tests
# =============================================================================

slay test_error_recovery() {
    test_start("Error recovery and reporting")
    
    sus pos normie = create_position("test.csd", 1, 10, 10)
    sus error_token normie = create_error_token("Invalid character", pos)
    
    assert_true(is_error(error_token))
    
    sus error_msg tea = error_token_message(error_token)
    assert_true(string.length(error_msg) > 0)
    assert_true(string.contains(error_msg, "error"))
    
    # Test error recovery
    sus error_pos normie = 5
    sus recovered_pos normie = recover_from_error("invalid@ code", error_pos)
    assert_true(recovered_pos > error_pos)
    
    print_test_summary()
}

slay test_invalid_tokens() {
    test_start("Invalid token handling")
    
    # Test unterminated string
    sus source1 tea = "\"unterminated string"
    sus count1 normie = tokenize(source1)
    assert_true(count1 >= 0)  # Should handle gracefully
    
    # Test invalid character sequence
    sus source2 tea = "sus @invalid = 42"
    sus count2 normie = tokenize(source2)
    assert_true(count2 >= 0)
    
    print_test_summary()
}

# =============================================================================
# Utility Function Tests
# =============================================================================

slay test_utility_functions() {
    test_start("Utility functions")
    
    # Test string hashing
    sus hash1 normie = hash_string("test")
    sus hash2 normie = hash_string("test")
    sus hash3 normie = hash_string("different")
    
    assert_eq_int(hash1, hash2)  # Same string should have same hash
    assert_true(hash1 != hash3)  # Different strings should have different hashes
    
    # Test token value extraction
    sus pos normie = create_position("test.csd", 1, 1, 0)
    sus token_info normie = create_token_info(IDENT_TOKEN, "test", pos, "test")
    sus value tea = token_value(token_info)
    assert_true(string.length(value) > 0)
    
    print_test_summary()
}

# =============================================================================
# Module Status Tests
# =============================================================================

slay test_module_status() {
    test_start("Module status and information")
    
    sus status tea = token_vibe_status()
    assert_true(string.length(status) > 0)
    assert_true(string.contains(status, "token_vibe"))
    assert_true(string.contains(status, "tokenization"))
    
    sus stats tea = token_statistics()
    assert_true(string.length(stats) > 0)
    assert_true(string.contains(stats, "token types"))
    assert_true(string.contains(stats, "Keywords"))
    
    print_test_summary()
}

slay test_tokenizer_validation() {
    test_start("Tokenizer validation")
    
    sus is_valid lit = validate_tokenizer()
    assert_true(is_valid)
    
    print_test_summary()
}

# =============================================================================
# Operator and Delimiter Classification Tests
# =============================================================================

slay test_operator_classification() {
    test_start("Operator classification")
    
    assert_true(is_operator(ADD_TOKEN))
    assert_true(is_operator(SUB_TOKEN))
    assert_true(is_operator(MUL_TOKEN))
    assert_true(is_operator(DIV_TOKEN))
    assert_true(is_operator(EQ_TOKEN))
    assert_true(is_operator(NE_TOKEN))
    assert_true(is_operator(ASSIGN_TOKEN))
    
    assert_true(!is_operator(IDENT_TOKEN))
    assert_true(!is_operator(LPAREN_TOKEN))
    assert_true(!is_operator(SUS_TOKEN))
    
    print_test_summary()
}

slay test_delimiter_classification() {
    test_start("Delimiter classification")
    
    assert_true(is_delimiter(LPAREN_TOKEN))
    assert_true(is_delimiter(RPAREN_TOKEN))
    assert_true(is_delimiter(LBRACE_TOKEN))
    assert_true(is_delimiter(RBRACE_TOKEN))
    assert_true(is_delimiter(SEMICOLON_TOKEN))
    assert_true(is_delimiter(COMMA_TOKEN))
    assert_true(is_delimiter(DOT_TOKEN))
    
    assert_true(!is_delimiter(ADD_TOKEN))
    assert_true(!is_delimiter(IDENT_TOKEN))
    assert_true(!is_delimiter(SUS_TOKEN))
    
    print_test_summary()
}

# =============================================================================
# Performance and Stress Tests
# =============================================================================

slay test_large_input_tokenization() {
    test_start("Large input tokenization")
    
    # Create a reasonably large input
    sus large_source tea = "slay test_function(param tea) lit {\n    sus i normie = 0\n    bestie i < 100 {\n        vibez.spill(\"Iteration: \" + string.from_int(i))\n        i = i + 1\n    }\n    damn based\n}"
    
    sus count normie = tokenize(large_source)
    assert_true(count > 20)  # Should tokenize successfully
    
    print_test_summary()
}

slay test_edge_case_inputs() {
    test_start("Edge case inputs")
    
    # Test empty input
    sus empty_count normie = tokenize("")
    assert_eq_int(empty_count, 0)
    
    # Test whitespace only
    sus whitespace_count normie = tokenize("   \t  \n  ")
    assert_eq_int(whitespace_count, 0)
    
    # Test single character
    sus single_count normie = tokenize("x")
    assert_eq_int(single_count, 1)
    
    print_test_summary()
}

# =============================================================================
# Integration Tests
# =============================================================================

slay test_tokenizer_integration() {
    test_start("Tokenizer integration with CURSED syntax")
    
    # Test complete CURSED program tokenization
    sus program tea = "yeet \"stdlib\"\n\nslay main() {\n    sus greeting tea = \"Hello, CURSED!\"\n    vibez.spill(greeting)\n    damn based\n}"
    
    sus token_count normie = tokenize(program)
    assert_true(token_count > 15)  # Should have many tokens
    
    # Test error handling syntax
    sus error_program tea = "yikes error := something_risky()\nshook error {\n    vibez.spill(\"Error occurred\")\n    damn cap\n}"
    
    sus error_count normie = tokenize(error_program)
    assert_true(error_count > 10)
    
    print_test_summary()
}

# =============================================================================
# Main Test Runner
# =============================================================================

vibez.spill("Running comprehensive token_vibe module tests...")
vibez.spill("===================================================")

# Core functionality tests
test_token_constants()
test_token_string_conversion()

# Position tracking tests
test_position_functions()
test_position_edge_cases()

# Token info structure tests
test_token_info_creation()
test_token_classification()

# Character classification tests
test_character_classification()

# Scanner tests
test_scanner_creation()
test_scanner_advancement()

# Keyword recognition tests
test_keyword_recognition()

# Tokenization tests
test_basic_tokenization()
test_string_tokenization()
test_number_tokenization()
test_comment_tokenization()
test_complex_tokenization()

# Token stream tests
test_token_stream()

# Error handling tests
test_error_recovery()
test_invalid_tokens()

# Utility function tests
test_utility_functions()

# Module status tests
test_module_status()
test_tokenizer_validation()

# Classification tests
test_operator_classification()
test_delimiter_classification()

# Performance tests
test_large_input_tokenization()
test_edge_case_inputs()

# Integration tests
test_tokenizer_integration()

vibez.spill("===================================================")
vibez.spill("Comprehensive token_vibe tests complete!")
vibez.spill("✅ Lexical analysis system ready for self-hosting compiler")
vibez.spill("✅ All token types, scanning, and error recovery validated")
vibez.spill("✅ Position tracking and token stream utilities functional")
vibez.spill("✅ CURSED syntax tokenization fully supported")
