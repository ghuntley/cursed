yeet "testz"
yeet "collections_simple"

slay test_array_operations() {
    test_start("Array Operations")
    
    // Test array creation
    sus arr := array_create()
    assert_eq_int(array_length(arr), 0)
    
    // Test array operations (basic placeholders)
    assert_false(array_contains(arr, 5))
    assert_eq_int(array_get(arr, 0), 0)
    
    print_test_summary()
}

slay test_list_operations() {
    test_start("List Operations")
    
    // Test list creation
    sus lst := list_create()
    assert_eq_int(list_size(lst), 0)
    
    print_test_summary()
}

slay test_stack_operations() {
    test_start("Stack Operations")
    
    // Test stack creation
    sus stack := stack_create()
    assert_true(stack_empty(stack))
    
    print_test_summary()
}

test_array_operations()
test_list_operations()
test_stack_operations()
