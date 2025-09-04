fr fr CURSED Formatter Test Suite - Comprehensive Testing
fr fr Tests all formatter features including AST parsing, configuration, and error recovery

yeet "testz"
yeet "formatter"

fr fr ===== BASIC FORMATTING TESTS =====

slay test_basic_variable_formatting() {
    test_start("Basic Variable Formatting")
    
    sus input tea = "sus x drip=42;"
    sus expected tea = "sus x drip = 42;\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_basic_function_formatting() {
    test_start("Basic Function Formatting")
    
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n    damn 42;\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_function_with_parameters() {
    test_start("Function with Parameters")
    
    sus input tea = "slay add(x drip,y drip)drip{damn x+y;}"
    sus expected tea = "slay add(x drip, y drip) drip {\n    damn x + y;\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_struct_formatting() {
    test_start("Struct Formatting")
    
    sus input tea = "squad Point{spill x drip spill y drip}"
    sus expected tea = "squad Point {\n    spill x drip\n    spill y drip\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_interface_formatting() {
    test_start("Interface Formatting")
    
    sus input tea = "collab Drawable{slay draw();}"
    sus expected tea = "collab Drawable {\n    slay draw();\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_if_statement_formatting() {
    test_start("If Statement Formatting")
    
    sus input tea = "ready(x>0){vibez.spill(x);}otherwise{vibez.spill(0);}"
    sus expected tea = "ready (x > 0) {\n    vibez.spill(x);\n} otherwise {\n    vibez.spill(0);\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_while_loop_formatting() {
    test_start("While Loop Formatting")
    
    sus input tea = "bestie(i<10){i=i+1;}"
    sus expected tea = "bestie (i < 10) {\n    i = i + 1;\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_import_formatting() {
    test_start("Import Formatting")
    
    sus input tea = "yeet\"stringz\";yeet\"arrayz\";"
    sus expected tea = "yeet \"stringz\";\nyeet \"arrayz\";\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

fr fr ===== CONFIGURATION TESTS =====

slay test_indent_size_configuration() {
    test_start("Indent Size Configuration")
    
    sus config FormatterConfig = default_formatter_config()
    config.indent_size = 2
    
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n  damn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_tab_indentation() {
    test_start("Tab Indentation")
    
    sus config FormatterConfig = default_formatter_config()
    config.use_tabs = based
    
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n\tdamn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_brace_style_new_line() {
    test_start("Brace Style New Line")
    
    sus config FormatterConfig = default_formatter_config()
    config.opening_brace_style = "new_line"
    
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test()\n{\n    damn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_space_around_operators() {
    test_start("Space Around Operators")
    
    sus config FormatterConfig = default_formatter_config()
    config.space_around_operators = cringe
    
    sus input tea = "sus x drip = 1 + 2;"
    sus expected tea = "sus x drip=1+2;\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_compact_configuration() {
    test_start("Compact Configuration")
    
    sus config FormatterConfig = compact_formatter_config()
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n  damn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_google_style_configuration() {
    test_start("Google Style Configuration")
    
    sus config FormatterConfig = google_style_config()
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n  damn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

fr fr ===== ADVANCED FEATURE TESTS =====

slay test_blank_lines_between_functions() {
    test_start("Blank Lines Between Functions")
    
    sus config FormatterConfig = default_formatter_config()
    config.blank_lines_before_functions = 2
    
    sus input tea = "slay first(){damn 1;}slay second(){damn 2;}"
    sus expected tea = "slay first() {\n    damn 1;\n}\n\n\nslay second() {\n    damn 2;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_import_sorting() {
    test_start("Import Sorting")
    
    sus config FormatterConfig = default_formatter_config()
    config.sort_imports = based
    
    sus input tea = "yeet\"stringz\";slay test(){damn 42;}yeet\"arrayz\";"
    sus expected tea = "yeet \"stringz\";\nyeet \"arrayz\";\n\nslay test() {\n    damn 42;\n}\n"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    assert_eq_string(actual, expected)
}

slay test_comment_formatting() {
    test_start("Comment Formatting")
    
    sus input tea = "fr fr This is a comment\nsus x drip = 42;"
    sus expected tea = "fr fr This is a comment\nsus x drip = 42;\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

slay test_nested_structures() {
    test_start("Nested Structures")
    
    sus input tea = "ready(x>0){ready(y>0){damn x+y;}}"
    sus expected tea = "ready (x > 0) {\n    ready (y > 0) {\n        damn x + y;\n    }\n}\n"
    sus actual tea = format_cursed_code_ast(input)
    
    assert_eq_string(actual, expected)
}

fr fr ===== ERROR RECOVERY TESTS =====

slay test_malformed_syntax_recovery() {
    test_start("Malformed Syntax Recovery")
    
    sus config FormatterConfig = default_formatter_config()
    config.continue_on_errors = based
    config.preserve_malformed_syntax = based
    
    sus input tea = "sus x drip = ; slay test() { damn 42; }"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    fr fr Should not crash and should preserve some formatting
    assert_true(string_length(actual) > 0)
}

slay test_missing_semicolon_recovery() {
    test_start("Missing Semicolon Recovery")
    
    sus config FormatterConfig = default_formatter_config()
    config.continue_on_errors = based
    
    sus input tea = "sus x drip = 42 sus y drip = 43;"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    fr fr Should handle missing semicolon gracefully
    assert_true(contains_substring(actual, "42"))
    assert_true(contains_substring(actual, "43"))
}

slay test_unmatched_braces_recovery() {
    test_start("Unmatched Braces Recovery")
    
    sus config FormatterConfig = default_formatter_config()
    config.continue_on_errors = based
    
    sus input tea = "slay test() { damn 42;"
    sus actual tea = format_cursed_code_with_config_ast(input, config)
    
    fr fr Should handle unmatched braces
    assert_true(contains_substring(actual, "test"))
    assert_true(contains_substring(actual, "42"))
}

fr fr ===== DIFF GENERATION TESTS =====

slay test_diff_generation() {
    test_start("Diff Generation")
    
    sus original tea = "sus x drip=42;"
    sus config FormatterConfig = default_formatter_config()
    sus diff_output tea = format_with_diff(original, config)
    
    fr fr Should contain diff markers
    assert_true(contains_substring(diff_output, "+"))
    assert_true(contains_substring(diff_output, "-"))
}

slay test_no_changes_diff() {
    test_start("No Changes Diff")
    
    sus already_formatted tea = "sus x drip = 42;\n"
    sus config FormatterConfig = default_formatter_config()
    sus diff_output tea = format_with_diff(already_formatted, config)
    
    fr fr Should show unchanged lines
    assert_true(contains_substring(diff_output, "  "))
}

fr fr ===== SYNTAX VALIDATION TESTS =====

slay test_valid_syntax_validation() {
    test_start("Valid Syntax Validation")
    
    sus valid_code tea = "sus x drip = 42; slay test() { damn x; }"
    sus errors tea[value] = validate_syntax(valid_code)
    
    assert_eq_int(len(errors), 0)
}

slay test_invalid_syntax_validation() {
    test_start("Invalid Syntax Validation")
    
    sus invalid_code tea = "sus x drip = ; slay ( { damn ;"
    sus errors tea[value] = validate_syntax(invalid_code)
    
    assert_true(len(errors) > 0)
}

fr fr ===== CONFIGURATION SERIALIZATION TESTS =====

slay test_config_serialization() {
    test_start("Configuration Serialization")
    
    sus config FormatterConfig = default_formatter_config()
    sus config_text tea = save_config_to_string(config)
    
    assert_true(contains_substring(config_text, "indent_size"))
    assert_true(contains_substring(config_text, "max_line_length"))
    assert_true(contains_substring(config_text, "use_tabs"))
}

slay test_config_deserialization() {
    test_start("Configuration Deserialization")
    
    sus config_text tea = "indent_size=8\nmax_line_length=120\nuse_tabs=true\n"
    sus config FormatterConfig = load_config_from_string(config_text)
    
    assert_eq_int(config.indent_size, 8)
    assert_eq_int(config.max_line_length, 120)
    assert_true(config.use_tabs)
}

fr fr ===== TOKENIZER TESTS =====

slay test_advanced_tokenization() {
    test_start("Advanced Tokenization")
    
    sus source tea = "sus x drip = \"hello\"; fr fr comment"
    sus ctx TokenizerContext = tokenize_advanced(source)
    
    fr fr Should tokenize properly
    assert_true(len(ctx.tokens) > 0)
    assert_eq_int(len(ctx.errors), 0)
}

slay test_string_literal_tokenization() {
    test_start("String Literal Tokenization")
    
    sus source tea = "\"hello world\""
    sus ctx TokenizerContext = tokenize_advanced(source)
    
    fr fr Should identify string token
    assert_eq_int(len(ctx.tokens), 1)
    assert_eq_string(ctx.tokens[0].type, "STRING")
}

slay test_comment_tokenization() {
    test_start("Comment Tokenization")
    
    sus source tea = "fr fr This is a comment"
    sus ctx TokenizerContext = tokenize_advanced(source)
    
    fr fr Should identify comment token
    assert_true(len(ctx.tokens) > 0)
    fr fr Note: simplified tokenizer might handle this differently
}

fr fr ===== PERFORMANCE TESTS =====

slay test_large_file_formatting() {
    test_start("Large File Formatting Performance")
    
    fr fr Generate a moderately large code sample
    sus large_code tea = ""
    sus i drip = 0
    bestie (i < 10) {
        large_code = large_code + "slay func" + int_to_string(i) + "(){damn " + int_to_string(i) + ";}"
        i = i + 1
    }
    
    sus formatted tea = format_cursed_code_ast(large_code)
    
    fr fr Should complete without errors
    assert_true(string_length(formatted) > string_length(large_code))
}

slay test_deeply_nested_formatting() {
    test_start("Deeply Nested Formatting")
    
    sus nested_code tea = "ready(a){ready(b){ready(c){damn 1;}}}"
    sus formatted tea = format_cursed_code_ast(nested_code)
    
    fr fr Should handle deep nesting
    assert_true(contains_substring(formatted, "    "))  fr fr Multiple indent levels
}

fr fr ===== EDGE CASE TESTS =====

slay test_empty_input() {
    test_start("Empty Input")
    
    sus empty tea = ""
    sus formatted tea = format_cursed_code_ast(empty)
    
    fr fr Should handle empty input gracefully
    assert_eq_string(formatted, "")
}

slay test_whitespace_only_input() {
    test_start("Whitespace Only Input")
    
    sus whitespace tea = "   \n\t  \n  "
    sus formatted tea = format_cursed_code_ast(whitespace)
    
    fr fr Should normalize whitespace
    assert_true(string_length(formatted) <= string_length(whitespace))
}

slay test_single_character_input() {
    test_start("Single Character Input")
    
    sus single tea = "x"
    sus formatted tea = format_cursed_code_ast(single)
    
    fr fr Should handle single character
    assert_true(string_length(formatted) >= 1)
}

fr fr ===== MAIN TEST RUNNER =====

slay main_character() {
    vibez.spill("🧪 CURSED Formatter Test Suite")
    vibez.spill("Testing AST-based formatter with comprehensive features")
    vibez.spill("")
    
    fr fr Basic formatting tests
    test_basic_variable_formatting()
    test_basic_function_formatting()
    test_function_with_parameters()
    test_struct_formatting()
    test_interface_formatting()
    test_if_statement_formatting()
    test_while_loop_formatting()
    test_import_formatting()
    
    fr fr Configuration tests
    test_indent_size_configuration()
    test_tab_indentation()
    test_brace_style_new_line()
    test_space_around_operators()
    test_compact_configuration()
    test_google_style_configuration()
    
    fr fr Advanced feature tests
    test_blank_lines_between_functions()
    test_import_sorting()
    test_comment_formatting()
    test_nested_structures()
    
    fr fr Error recovery tests
    test_malformed_syntax_recovery()
    test_missing_semicolon_recovery()
    test_unmatched_braces_recovery()
    
    fr fr Diff generation tests
    test_diff_generation()
    test_no_changes_diff()
    
    fr fr Syntax validation tests
    test_valid_syntax_validation()
    test_invalid_syntax_validation()
    
    fr fr Configuration serialization tests
    test_config_serialization()
    test_config_deserialization()
    
    fr fr Tokenizer tests
    test_advanced_tokenization()
    test_string_literal_tokenization()
    test_comment_tokenization()
    
    fr fr Performance tests
    test_large_file_formatting()
    test_deeply_nested_formatting()
    
    fr fr Edge case tests
    test_empty_input()
    test_whitespace_only_input()
    test_single_character_input()
    
    print_test_summary()
}
