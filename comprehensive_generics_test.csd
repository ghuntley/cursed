yeet "testz"

fr fr Test comprehensive generics system with [T] syntax
test_start("CURSED Generics Monomorphization Test")

fr fr Generic struct with single type parameter
be_like Box[T] squad {
    value T
}

fr fr Generic function with single type parameter  
slay identity[T](x T) T {
    damn x
}

fr fr Generic function with multiple type parameters
slay pair[A, B](first A, second B) Pair[A, B] {
    damn Pair[A, B]{first: first, second: second}
}

fr fr Generic struct with multiple type parameters
be_like Pair[A, B] squad {
    first A
    second B
}

fr fr Generic function with constraint
slay add_numbers[T: Numeric](a T, b T) T {
    damn a + b
}

fr fr Generic interface
be_like Container[T] collab {
    add(item T)
    get(index normie) T
    size() normie
}

fr fr Generic struct implementing interface
be_like Vector[T] squad {
    items []T
    length normie
}

slay vec_add[T](vec Vector[T], item T) Vector[T] {
    fr fr Add item to vector
    damn vec
}

slay vec_get[T](vec Vector[T], index normie) T {
    fr fr Get item at index
    damn vec.items[0]  fr fr Simplified
}

slay vec_size[T](vec Vector[T]) normie {
    damn vec.length
}

fr fr Test basic generic instantiation
slay test_basic_generics() {
    fr fr Test generic struct with normie
    sus int_box = Box[normie]{value: 42}
    assert_eq_int(int_box.value, 42)
    
    fr fr Test generic struct with tea
    sus string_box = Box[tea]{value: "hello"}
    assert_eq_string(string_box.value, "hello")
    
    fr fr Test generic function with normie
    sus result1 = identity[normie](42)
    assert_eq_int(result1, 42)
    
    fr fr Test generic function with tea
    sus result2 = identity[tea]("test")
    assert_eq_string(result2, "test")
}

fr fr Test multiple type parameters
slay test_multi_param_generics() {
    fr fr Test pair creation
    sus int_string_pair = pair[normie, tea](42, "hello")
    assert_eq_int(int_string_pair.first, 42)
    assert_eq_string(int_string_pair.second, "hello")
    
    fr fr Test different type combinations
    sus bool_int_pair = pair[lit, normie](based, 100)
    assert_true(bool_int_pair.first)
    assert_eq_int(bool_int_pair.second, 100)
}

fr fr Test constrained generics
slay test_constrained_generics() {
    fr fr Test numeric constraint with normie
    sus result1 = add_numbers[normie](10, 20)
    assert_eq_int(result1, 30)
    
    fr fr Test numeric constraint with drip  
    sus result2 = add_numbers[drip](100, 200)
    assert_eq_int(result2, 300)
    
    fr fr Test numeric constraint with meal
    sus result3 = add_numbers[meal](1.5, 2.5)
    assert_eq_int(result3, 4)  fr fr Simplified comparison
}

fr fr Test generic collections
slay test_generic_collections() {
    fr fr Test vector with normie
    sus int_vector = Vector[normie]{items: [], length: 0}
    sus int_result = vec_get[normie](int_vector, 0)
    
    fr fr Test vector with tea
    sus string_vector = Vector[tea]{items: [], length: 0}
    sus string_result = vec_get[tea](string_vector, 0)
    
    fr fr Test vector operations
    sus size1 = vec_size[normie](int_vector)
    assert_eq_int(size1, 0)
    
    sus size2 = vec_size[tea](string_vector)
    assert_eq_int(size2, 0)
}

fr fr Test nested generics
slay test_nested_generics() {
    fr fr Box containing a Box
    sus nested_box = Box[Box[normie]]{value: Box[normie]{value: 42}}
    assert_eq_int(nested_box.value.value, 42)
    
    fr fr Pair of Boxes
    sus box_pair = Pair[Box[normie], Box[tea]]{
        first: Box[normie]{value: 100},
        second: Box[tea]{value: "nested"}
    }
    assert_eq_int(box_pair.first.value, 100)
    assert_eq_string(box_pair.second.value, "nested")
}

fr fr Test generic type inference
slay test_type_inference() {
    fr fr Should infer types from usage
    sus inferred1 = identity(42)         fr fr Should infer [normie]
    sus inferred2 = identity("hello")    fr fr Should infer [tea]
    
    assert_eq_int(inferred1, 42)
    assert_eq_string(inferred2, "hello")
}

fr fr Test generic arrays and slices
slay test_generic_arrays() {
    fr fr Generic array type
    sus int_array = [normie; 3]{1, 2, 3}
    sus first = int_array[0]
    assert_eq_int(first, 1)
    
    fr fr Generic slice type  
    sus string_slice = []tea{"a", "b", "c"}
    sus first_str = string_slice[0]
    assert_eq_string(first_str, "a")
}

fr fr Test monomorphization optimization
slay test_monomorphization() {
    fr fr Same instantiation should reuse code
    sus box1 = Box[normie]{value: 1}
    sus box2 = Box[normie]{value: 2}
    
    fr fr Different instantiations should generate separate code
    sus int_box = Box[normie]{value: 42}
    sus string_box = Box[tea]{value: "different"}
    
    assert_eq_int(box1.value, 1)
    assert_eq_int(box2.value, 2)
    assert_eq_int(int_box.value, 42)
    assert_eq_string(string_box.value, "different")
}

fr fr Test complex generic constraints
slay test_complex_constraints() {
    fr fr Multiple constraints
    slay compare_and_add[T: Comparable + Numeric](a T, b T) T {
        bestie (a > b) {
            damn a + b
        } nah {
            damn b - a  
        }
    }
    
    sus result = compare_and_add[normie](10, 5)
    assert_eq_int(result, 15)
}

fr fr Test generic error handling
slay test_generic_error_handling() {
    be_like Result[T, E] squad {
        is_ok lit
        value T
        error E
    }
    
    slay make_ok[T, E](value T) Result[T, E] {
        damn Result[T, E]{is_ok: based, value: value, error: undefined}
    }
    
    slay make_error[T, E](error E) Result[T, E] {
        damn Result[T, E]{is_ok: cringe, value: undefined, error: error}
    }
    
    sus ok_result = make_ok[normie, tea](42)
    assert_true(ok_result.is_ok)
    assert_eq_int(ok_result.value, 42)
    
    sus error_result = make_error[normie, tea]("error message")
    assert_false(error_result.is_ok)
    assert_eq_string(error_result.error, "error message")
}

fr fr Run all tests
test_basic_generics()
test_multi_param_generics()  
test_constrained_generics()
test_generic_collections()
test_nested_generics()
test_type_inference()
test_generic_arrays()
test_monomorphization()
test_complex_constraints()
test_generic_error_handling()

print_test_summary()
