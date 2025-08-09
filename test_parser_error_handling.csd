# Test cases for parser error handling

# Test 1: Invalid syntax - missing closing brace
slay broken_function() {
    sus x drip = 42
    vibez.spill("Missing brace")
# Missing closing brace should be handled gracefully

# Test 2: Invalid number literal
sus invalid_num drip = 123abc

# Test 3: Unclosed string
sus bad_string tea = "unclosed string

# Test 4: Invalid function parameter
slay bad_params( drip, , another_param drip) {
    damn 42
}

# Test 5: Missing variable name
sus = 42

# Test 6: Invalid type annotation
sus bad_type invalid_type_here = 123

# Test 7: Nested structure with errors
squad BadStruct {
    field1 missing_type
    field2 drip
    invalid_field_syntax
}

# Test 8: Complex expression with errors
sus result drip = (1 + 2 * / 3) + missing_var

# Test 9: Function call with mismatched parens
vibez.spill("test", extra_arg, )

# Test 10: Invalid control flow
ready (incomplete_condition {
    vibez.spill("Missing closing paren")
}
