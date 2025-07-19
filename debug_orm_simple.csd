yeet "testz"

# Test simple type definition
be_like SimpleType = {
    name tea
    value normie
}

slay test_simple_type() lit {
    test_start("Simple type test")
    
    sus simple SimpleType = SimpleType{
        name: "test",
        value: 42
    }
    
    assert_eq_string(simple.name, "test")
    assert_eq_int(simple.value, 42)
    
    damn based
}

test_simple_type()
print_test_summary()
