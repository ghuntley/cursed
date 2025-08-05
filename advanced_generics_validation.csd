fr fr Advanced Generics Validation Test
yeet "testz"

fr fr Generic function with type parameters
slay identity<T>(value T) T {
    damn value
}

fr fr Generic struct with constraints
squad Container<T> {
    spill data T
    spill size normie
}

fr fr Generic method on struct
flex Container<T> {
    slay get() T {
        damn data
    }
    
    slay set(new_value T) {
        data = new_value
    }
    
    slay len() normie {
        damn size
    }
}

fr fr Complex generic function with multiple type parameters
slay pair<T, U>(first T, second U) (T, U) {
    damn (first, second)
}

fr fr Generic function with constraints (comparable)
slay max<T>(a T, b T) T where T: Comparable {
    if a > b {
        damn a
    }
    damn b
}

fr fr Test monomorphization with different types
slay test_generics() {
    test_start("Generic Functions Test")
    
    fr fr Test identity function with different types
    sus int_result normie = identity<normie>(42)
    sus string_result tea = identity<tea>("hello")
    sus float_result meal = identity<meal>(3.14)
    
    assert_eq_int(int_result, 42)
    assert_eq_string(string_result, "hello")
    assert_eq_float(float_result, 3.14)
    
    test_start("Generic Structs Test")
    
    fr fr Test generic struct with different types
    sus int_container Container<normie> = Container<normie>{data: 100, size: 1}
    sus string_container Container<tea> = Container<tea>{data: "test", size: 1}
    
    assert_eq_int(int_container.get(), 100)
    assert_eq_string(string_container.get(), "test")
    
    fr fr Test generic methods
    int_container.set(200)
    string_container.set("updated")
    
    assert_eq_int(int_container.get(), 200)
    assert_eq_string(string_container.get(), "updated")
    
    test_start("Complex Generics Test")
    
    fr fr Test multiple type parameters
    sus result_pair (normie, tea) = pair<normie, tea>(42, "world")
    sus (first, second) = result_pair
    
    assert_eq_int(first, 42)
    assert_eq_string(second, "world")
    
    fr fr Test generic constraints (if implemented)
    fr fr sus max_int normie = max<normie>(10, 20)
    fr fr assert_eq_int(max_int, 20)
    
    print_test_summary()
}

test_generics()
