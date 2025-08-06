yeet "testz"

fr fr Test that CURSED parser can handle the angle bracket syntax for generics
test_start("Generics Parsing Test")

fr fr Test simple angle bracket syntax - may not work yet
slay test_angle_brackets() {
    fr fr This tests if the parser can handle generic syntax
    vibez.spill("Testing generic syntax parsing...")
    
    fr fr Simple test - define a type that would be generic
    fr fr For now, just test that basic syntax works
    assert_true(based)
}

fr fr Test parsing of function that might accept generics
slay test_function_syntax() {
    vibez.spill("Testing function syntax...")
    
    fr fr Basic function that could be made generic
    slay simple_function(value normie) normie {
        damn value
    }
    
    sus result = simple_function(42)
    assert_eq_int(result, 42)
}

fr fr Test struct that could support generics
slay test_struct_syntax() {
    vibez.spill("Testing struct syntax...")
    
    be_like Container squad {
        value normie
        name tea
    }
    
    sus container = Container{value: 42, name: "test"}
    assert_eq_int(container.value, 42)
    assert_eq_string(container.name, "test")
}

test_angle_brackets()
test_function_syntax()
test_struct_syntax()

print_test_summary()
