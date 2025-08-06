yeet "testz"

test_start("Generics Performance and Monomorphization Test")

fr fr Generic Stack implementation
be_like Stack[T] squad {
    data []T
    size normie
    capacity normie
}

slay stack_new[T](initial_capacity normie) Stack[T] {
    damn Stack[T]{
        data: make([]T, initial_capacity),
        size: 0,
        capacity: initial_capacity
    }
}

slay stack_push[T](stack @Stack[T], value T) {
    lowkey (stack.size >= stack.capacity) {
        panic("Stack overflow")
    }
    stack.data[stack.size] = value
    stack.size = stack.size + 1
}

slay stack_pop[T](stack @Stack[T]) T {
    lowkey (stack.size <= 0) {
        panic("Stack underflow")
    }
    stack.size = stack.size - 1
    damn stack.data[stack.size]
}

slay stack_is_empty[T](stack @Stack[T]) lit {
    damn stack.size == 0
}

fr fr Generic function with constraints
slay generic_max[T: Comparable](a T, b T) T {
    lowkey (a > b) {
        damn a
    } nah {
        damn b
    }
}

fr fr Test performance with different instantiations
slay test_monomorphization_performance() {
    fr fr Test integer stack
    sus int_stack = stack_new[normie](10)
    stack_push[normie](@int_stack, 1)
    stack_push[normie](@int_stack, 2)
    stack_push[normie](@int_stack, 3)
    
    sus value1 = stack_pop[normie](@int_stack)
    assert_eq_int(value1, 3)
    
    sus value2 = stack_pop[normie](@int_stack)
    assert_eq_int(value2, 2)
    
    assert_false(stack_is_empty[normie](@int_stack))
    
    fr fr Test string stack (different monomorphization)
    sus string_stack = stack_new[tea](5)
    stack_push[tea](@string_stack, "first")
    stack_push[tea](@string_stack, "second")
    
    sus str_value = stack_pop[tea](@string_stack)
    assert_eq_string(str_value, "second")
    
    assert_false(stack_is_empty[tea](@string_stack))
    
    fr fr Test float stack (third monomorphization)
    sus float_stack = stack_new[meal](3)
    stack_push[meal](@float_stack, 3.14)
    stack_push[meal](@float_stack, 2.71)
    
    sus float_value = stack_pop[meal](@float_stack)
    assert_true(float_value > 2.0)
    
    assert_false(stack_is_empty[meal](@float_stack))
}

fr fr Test constraint validation performance
slay test_constraint_performance() {
    fr fr Multiple calls to constrained generic function
    sus max1 = generic_max[normie](10, 20)
    assert_eq_int(max1, 20)
    
    sus max2 = generic_max[normie](50, 30)
    assert_eq_int(max2, 50)
    
    sus max3 = generic_max[drip](100, 200)
    assert_eq_int(max3, 200)
    
    sus max4 = generic_max[meal](3.14, 2.71)
    assert_true(max4 > 3.0)
}

fr fr Test nested generic types
slay test_nested_generics() {
    be_like Pair[A, B] squad {
        first A
        second B
    }
    
    be_like Triple[A, B, C] squad {
        first A
        second B
        third C
    }
    
    fr fr Nested instantiation
    sus nested = Pair[Stack[normie], Stack[tea]]{
        first: stack_new[normie](5),
        second: stack_new[tea](5)
    }
    
    stack_push[normie](@nested.first, 42)
    stack_push[tea](@nested.second, "hello")
    
    sus int_val = stack_pop[normie](@nested.first)
    sus str_val = stack_pop[tea](@nested.second)
    
    assert_eq_int(int_val, 42)
    assert_eq_string(str_val, "hello")
    
    fr fr Triple nesting
    sus triple = Triple[normie, tea, lit]{
        first: 100,
        second: "world",
        third: based
    }
    
    assert_eq_int(triple.first, 100)
    assert_eq_string(triple.second, "world")
    assert_true(triple.third)
}

fr fr Test generic inheritance simulation
slay test_generic_composition() {
    be_like Container[T] squad {
        items []T
        count normie
    }
    
    slay container_add[T](container @Container[T], item T) {
        container.items = append(container.items, item)
        container.count = container.count + 1
    }
    
    slay container_get[T](container @Container[T], index normie) T {
        lowkey (index >= container.count) {
            panic("Index out of bounds")
        }
        damn container.items[index]
    }
    
    fr fr Test with different types
    sus int_container = Container[normie]{items: [], count: 0}
    container_add[normie](@int_container, 1)
    container_add[normie](@int_container, 2)
    container_add[normie](@int_container, 3)
    
    sus first = container_get[normie](@int_container, 0)
    sus second = container_get[normie](@int_container, 1)
    sus third = container_get[normie](@int_container, 2)
    
    assert_eq_int(first, 1)
    assert_eq_int(second, 2)
    assert_eq_int(third, 3)
    assert_eq_int(int_container.count, 3)
}

fr fr Test monomorphization caching (same types should reuse code)
slay test_monomorphization_caching() {
    fr fr Multiple stacks of same type should share monomorphized code
    sus stack1 = stack_new[normie](10)
    sus stack2 = stack_new[normie](20)
    sus stack3 = stack_new[normie](30)
    
    stack_push[normie](@stack1, 10)
    stack_push[normie](@stack2, 20)
    stack_push[normie](@stack3, 30)
    
    sus val1 = stack_pop[normie](@stack1)
    sus val2 = stack_pop[normie](@stack2)
    sus val3 = stack_pop[normie](@stack3)
    
    assert_eq_int(val1, 10)
    assert_eq_int(val2, 20)
    assert_eq_int(val3, 30)
    
    fr fr All should be empty now
    assert_true(stack_is_empty[normie](@stack1))
    assert_true(stack_is_empty[normie](@stack2))
    assert_true(stack_is_empty[normie](@stack3))
}

fr fr Run all performance tests
test_monomorphization_performance()
test_constraint_performance()
test_nested_generics()
test_generic_composition()
test_monomorphization_caching()

print_test_summary()
