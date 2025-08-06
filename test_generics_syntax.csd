yeet "testz"

fr fr Test the enhanced [T] syntax support
test_start("Enhanced Generics Syntax Test")

fr fr Test simple struct creation first to establish baseline
slay test_baseline() {
    be_like SimpleBox squad {
        value normie
    }
    
    sus box = SimpleBox{value: 42}
    assert_eq_int(box.value, 42)
    vibez.spill("Baseline struct syntax works")
}

fr fr Test whether parser accepts generic-looking syntax
slay test_generic_syntax_acceptance() {
    vibez.spill("Testing if parser accepts generic-like syntax...")
    
    fr fr Try to define what would be a generic function
    fr fr This tests if the parser can handle the tokens
    slay would_be_generic(value normie) normie {
        damn value
    }
    
    sus result = would_be_generic(100)
    assert_eq_int(result, 100)
    vibez.spill("Function syntax baseline works")
}

fr fr Test parsing of types that look generic
slay test_type_syntax() {
    vibez.spill("Testing type syntax...")
    
    fr fr Define a type that could become generic
    be_like Box squad {
        content normie
    }
    
    sus my_box = Box{content: 42}
    assert_eq_int(my_box.content, 42)
    
    fr fr Test with different type
    be_like StringBox squad {
        content tea
    }
    
    sus str_box = StringBox{content: "hello"}
    assert_eq_string(str_box.content, "hello")
    
    vibez.spill("Type definitions work correctly")
}

fr fr Test if we can parse function parameters that would be generic
slay test_function_parameters() {
    vibez.spill("Testing function parameter syntax...")
    
    fr fr Function that could accept any type (if generics worked)
    slay process_int(item normie) normie {
        damn item + 1
    }
    
    slay process_string(item tea) tea {
        damn item
    }
    
    sus int_result = process_int(41)
    assert_eq_int(int_result, 42)
    
    sus str_result = process_string("test")
    assert_eq_string(str_result, "test")
    
    vibez.spill("Function parameter syntax works")
}

test_baseline()
test_generic_syntax_acceptance()
test_type_syntax()
test_function_parameters()

print_test_summary()
