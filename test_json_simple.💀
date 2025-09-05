fr fr Simple JSON Module Test

yeet "json"

slay test_parse_json() tea {
    vibez.spill("Testing basic JSON parsing...")
    
    sus simple_str tea = "\"hello\""
    sus parsed_str tea = parse_json(simple_str)
    vibez.spill("Parsed string: " + parsed_str)
    
    sus simple_num tea = "42"
    sus parsed_num tea = parse_json(simple_num)
    vibez.spill("Parsed number: " + parsed_num)
    
    sus simple_bool tea = "true"
    sus parsed_bool tea = parse_json(simple_bool)
    vibez.spill("Parsed boolean: " + parsed_bool)
    
    sus simple_obj tea = "{\"key\":\"value\"}"
    sus parsed_obj tea = parse_json(simple_obj)
    vibez.spill("Parsed object: " + parsed_obj)
    
    damn "Basic parsing tests completed"
}

slay test_stringify() tea {
    vibez.spill("Testing JSON stringification...")
    
    sus test_str tea = "hello"
    sus stringified tea = stringify(test_str)
    vibez.spill("Stringified: " + stringified)
    
    sus test_num tea = "123"
    sus num_stringified tea = stringify(test_num)
    vibez.spill("Number stringified: " + num_stringified)
    
    damn "Stringify tests completed"
}

slay test_validation() tea {
    vibez.spill("Testing JSON validation...")
    
    sus valid_json tea = "{\"valid\":true}"
    sus is_valid lit = is_valid_json(valid_json)
    vibez.spill("Valid JSON check: " + lit_to_string(is_valid))
    
    sus invalid_json tea = "{invalid:}"
    sus is_invalid lit = is_valid_json(invalid_json)
    vibez.spill("Invalid JSON check: " + lit_to_string(is_invalid))
    
    damn "Validation tests completed"
}

slay main_character() tea {
    vibez.spill("=== Simple JSON Tests ===")
    
    sus result1 tea = test_parse_json()
    vibez.spill(result1)
    vibez.spill("")
    
    sus result2 tea = test_stringify()
    vibez.spill(result2)
    vibez.spill("")
    
    sus result3 tea = test_validation()
    vibez.spill(result3)
    
    vibez.spill("=== All tests completed ===")
    damn "JSON testing complete"
}
