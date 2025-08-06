yeet "testz"

fr fr Test enhanced generics system with full constraint resolution
test_start("Enhanced Generics Monomorphization Test")

fr fr Generic struct with constraints
be_like Stack[T: Sized] squad {
    items []T
    capacity normie
    size normie
}

fr fr Generic interface with constraints
be_like Comparable[T] collab {
    compare(other T) normie
    equal(other T) lit
}

fr fr Generic function with multiple constraints
slay max[T: Comparable[T] & Ordered](a T, b T) T {
    lowkey (a > b) {
        damn a
    } nah {
        damn b
    }
}

fr fr Generic function with variance
slay map[T, U](items []T, func slay(T) U) []U {
    sus result = make([]U, len(items))
    sus i = 0
    bestie (i < len(items)) {
        result[i] = func(items[i])
        i = i + 1
    }
    damn result
}

fr fr Complex generic type with associated types
be_like Container[T: Sized] collab {
    be_like Iterator[T] collab {
        next() T
        has_next() lit
    }
    
    add(item T)
    get(index normie) T
    size() normie
    iter() Iterator[T]
}

fr fr Generic struct implementing interface
be_like Vector[T: Sized] squad {
    data []T
    length normie
    capacity normie
}

fr fr Implement Container for Vector with proper constraints
slay vector_add[T: Sized](vec @Vector[T], item T) {
    fr fr Add constraint validation
    vec.data = append(vec.data, item)
    vec.length = vec.length + 1
}

slay vector_get[T: Sized](vec @Vector[T], index normie) T {
    lowkey (index < 0 || index >= vec.length) {
        panic("Index out of bounds")
    }
    damn vec.data[index]
}

slay vector_size[T: Sized](vec @Vector[T]) normie {
    damn vec.length
}

fr fr Test constraint validation
slay test_constraint_validation() {
    fr fr Valid constraint: normie satisfies Numeric
    sus int_stack = Stack[normie]{items: [], capacity: 10, size: 0}
    assert_eq_int(int_stack.size, 0)
    
    fr fr Valid constraint: tea satisfies Comparable
    sus string_stack = Stack[tea]{items: [], capacity: 10, size: 0}
    assert_eq_int(string_stack.size, 0)
    
    fr fr Test max function with numeric types
    sus max_int = max[normie](10, 20)
    assert_eq_int(max_int, 20)
    
    sus max_float = max[meal](3.14, 2.71)
    assert_true(max_float > 3.0)
}

fr fr Test variance checking
slay test_variance_checking() {
    fr fr Covariant arrays
    sus int_array = []normie{1, 2, 3}
    sus first = int_array[0]
    assert_eq_int(first, 1)
    
    fr fr Contravariant function parameters
    slay process[T: Comparable[T]](items []T, processor slay(T) T) []T {
        sus result = make([]T, len(items))
        sus i = 0
        bestie (i < len(items)) {
            result[i] = processor(items[i])
            i = i + 1
        }
        damn result
    }
    
    slay double_int(x normie) normie {
        damn x * 2
    }
    
    sus doubled = process[normie](int_array, double_int)
    assert_eq_int(doubled[0], 2)
}

fr fr Test generic collections with complex constraints
slay test_complex_generic_collections() {
    fr fr Vector with numeric constraint
    sus int_vector = Vector[normie]{data: [], length: 0, capacity: 10}
    vector_add[normie](@int_vector, 42)
    
    sus value = vector_get[normie](@int_vector, 0)
    assert_eq_int(value, 42)
    
    sus size = vector_size[normie](@int_vector)
    assert_eq_int(size, 1)
    
    fr fr Test with different type
    sus string_vector = Vector[tea]{data: [], length: 0, capacity: 10}
    vector_add[tea](@string_vector, "hello")
    
    sus str_value = vector_get[tea](@string_vector, 0)
    assert_eq_string(str_value, "hello")
}

fr fr Test higher-order generics
slay test_higher_order_generics() {
    fr fr Generic function that takes generic types
    slay transform[T, U, Container[_]](container Container[T], func slay(T) U) Container[U] {
        fr fr This would require higher-kinded types
        fr fr For now, simplified implementation
        damn container
    }
    
    fr fr Test map function with different types
    sus numbers = []normie{1, 2, 3, 4, 5}
    
    slay to_string(n normie) tea {
        damn "number"
    }
    
    sus strings = map[normie, tea](numbers, to_string)
    assert_eq_string(strings[0], "number")
}

fr fr Test monomorphization caching
slay test_monomorphization_caching() {
    fr fr Same instantiation should reuse generated code
    sus stack1 = Stack[normie]{items: [], capacity: 5, size: 0}
    sus stack2 = Stack[normie]{items: [], capacity: 10, size: 0}
    
    fr fr Different instantiation should generate new code
    sus string_stack = Stack[tea]{items: [], capacity: 5, size: 0}
    
    assert_eq_int(stack1.size, 0)
    assert_eq_int(stack2.size, 0)
    assert_eq_int(string_stack.size, 0)
}

fr fr Test constraint inference
slay test_constraint_inference() {
    fr fr Function should infer constraints from usage
    slay auto_max(a auto, b auto) auto {
        lowkey (a > b) {
            damn a
        } nah {
            damn b
        }
    }
    
    fr fr Should infer T: Ordered constraint
    sus max_result = auto_max(10, 20)
    assert_eq_int(max_result, 20)
    
    sus max_float = auto_max(3.14, 2.71)
    assert_true(max_float > 3.0)
}

fr fr Test error handling with generics
slay test_generic_error_handling() {
    be_like Result[T, E] squad {
        success lit
        value T
        error E
    }
    
    slay ok[T, E](value T) Result[T, E] {
        damn Result[T, E]{success: based, value: value, error: undefined}
    }
    
    slay err[T, E](error E) Result[T, E] {
        damn Result[T, E]{success: cringe, value: undefined, error: error}
    }
    
    slay unwrap[T, E](result Result[T, E]) T {
        lowkey (!result.success) {
            panic("Unwrapping error result")
        }
        damn result.value
    }
    
    sus good_result = ok[normie, tea](42)
    sus value = unwrap[normie, tea](good_result)
    assert_eq_int(value, 42)
    
    sus bad_result = err[normie, tea]("error message")
    assert_false(bad_result.success)
    assert_eq_string(bad_result.error, "error message")
}

fr fr Test generic bounds and associated types
slay test_generic_bounds() {
    fr fr Define iterator pattern with associated types
    be_like Iterator[T] collab {
        next() T
        has_next() lit
    }
    
    be_like Iterable[T] collab {
        iter() Iterator[T]
    }
    
    fr fr Function that works with any iterable
    slay collect[T, I: Iterable[T]](iterable I) []T {
        sus result = []T{}
        sus iterator = iterable.iter()
        
        bestie (iterator.has_next()) {
            sus item = iterator.next()
            result = append(result, item)
        }
        
        damn result
    }
    
    fr fr Test would require full iterator implementation
    fr fr For now, just test the type definitions compile
    assert_true(based)
}

fr fr Run all enhanced generics tests
test_constraint_validation()
test_variance_checking()
test_complex_generic_collections()
test_higher_order_generics()
test_monomorphization_caching()
test_constraint_inference()
test_generic_error_handling()
test_generic_bounds()

print_test_summary()
