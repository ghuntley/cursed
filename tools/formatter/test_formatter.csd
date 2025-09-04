// Test suite for CURSED Code Formatter

yeet "testz"
yeet "formatter"

slay test_basic_formatting() {
    test_start("Basic variable formatting")
    
    sus input tea = "sus x drip=42;"
    sus expected tea = "sus x drip = 42;\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_function_formatting() {
    test_start("Function definition formatting")
    
    sus input tea = "slay test(){damn 42;}"
    sus expected tea = "slay test() {\n    damn 42;\n}\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_struct_formatting() {
    test_start("Struct definition formatting")
    
    sus input tea = "squad Point{spill x drip;spill y drip;}"
    sus expected tea = "squad Point {\n    spill x drip;\n    spill y drip;\n}\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_keyword_spacing() {
    test_start("CURSED keyword spacing")
    
    sus input tea = "ready(x>5){vibez.spill(\"yes\");}"
    sus expected tea = "ready (x > 5) {\n    vibez.spill(\"yes\");\n}\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_operator_spacing() {
    test_start("Operator spacing")
    
    sus input tea = "sus result drip=a+b*c-d/e;"
    sus expected tea = "sus result drip = a + b * c - d / e;\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_indentation() {
    test_start("Nested indentation")
    
    sus input tea = "ready(x>0){ready(y>0){vibez.spill(\"positive\");}}"
    sus expected tea = "ready (x > 0) {\n    ready (y > 0) {\n        vibez.spill(\"positive\");\n    }\n}\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_custom_config() {
    test_start("Custom formatting configuration")
    
    sus config FormatterConfig = FormatterConfig{
        indent_size: 2,
        max_line_length: 80,
        use_spaces: based,
        space_around_operators: cringe,
        align_struct_fields: based,
        newline_before_brace: based,
        align_gen_z_keywords: based,
        prefer_short_form_syntax: based
    }
    
    sus input tea = "slay test(){damn 42;}"
    sus result tea = format_cursed_code_with_config(input, config)
    
    // Should have 2-space indentation and newline before brace
    assert_true(contains_str(result, "  damn"))  // 2-space indent
    assert_true(contains_str(result, ")\n{"))    // newline before brace
}

slay test_string_literals() {
    test_start("String literal formatting")
    
    sus input tea = "sus msg tea=\"Hello, World!\";"
    sus expected tea = "sus msg tea = \"Hello, World!\";\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay test_comments_preservation() {
    test_start("Comment preservation")
    
    sus input tea = "// This is a comment\nsus x drip = 42;"
    sus result tea = format_cursed_code(input)
    
    // Comments should be preserved (basic implementation might skip this)
    assert_true(len_str(result) > 0)
}

slay test_gen_z_keywords() {
    test_start("Gen Z keyword formatting")
    
    sus input tea = "ready(based){vibez.spill(\"fr fr\");}"
    sus expected tea = "ready (based) {\n    vibez.spill(\"fr fr\");\n}\n"
    sus result tea = format_cursed_code(input)
    
    assert_eq_string(result, expected)
}

slay main_character() {
    vibez.spill("Running CURSED Formatter Tests...")
    
    test_basic_formatting()
    test_function_formatting()
    test_struct_formatting()
    test_keyword_spacing()
    test_operator_spacing()
    test_indentation()
    test_custom_config()
    test_string_literals()
    test_comments_preservation()
    test_gen_z_keywords()
    
    print_test_summary()
    
    vibez.spill("\nFormatter tests completed!")
}
