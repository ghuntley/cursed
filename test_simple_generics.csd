yeet "testz"

fr fr Simple generics test without advanced syntax
test_start("Basic Generics Test")

fr fr Test basic struct syntax first
be_like SimpleStruct squad {
    value normie
}

slay test_basic_struct() {
    sus s = SimpleStruct{value: 42}
    assert_eq_int(s.value, 42)
    vibez.spill("Basic struct works")
}

fr fr Test function definition
slay identity_int(x normie) normie {
    damn x
}

slay test_basic_function() {
    sus result = identity_int(42)
    assert_eq_int(result, 42)
    vibez.spill("Basic function works")
}

fr fr Now test simple array syntax
slay test_arrays() {
    sus arr = [1, 2, 3]
    assert_eq_int(arr[0], 1)
    vibez.spill("Arrays work")
}

test_basic_struct()
test_basic_function() 
test_arrays()

print_test_summary()
