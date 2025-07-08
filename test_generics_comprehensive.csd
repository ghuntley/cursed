yeet "testz"

// Generic function tests
slay identity[T](value T) T {
    damn value
}

slay test_generic_identity() {
    sus int_result = identity[normie](42)
    sus string_result = identity[tea]("hello")
    sus bool_result = identity[lit](based)
    
    assert_eq_int(int_result, 42)
    assert_eq_string(string_result, "hello")
    assert_true(bool_result)
}

// Generic struct tests
be_like Stack[T] squad {
    items []T
    size normie
}

slay (s @Stack[T]) push[T](item T) {
    s.items = append(s.items, item)
    s.size++
}

slay (s @Stack[T]) pop[T]() (T, yikes) {
    vibe_check s.size == 0 {
        sus zero T
        damn zero, yikes("Stack is empty")
    }
    
    s.size--
    damn s.items[s.size], cringe
}

slay (s @Stack[T]) peek[T]() (T, yikes) {
    vibe_check s.size == 0 {
        sus zero T
        damn zero, yikes("Stack is empty")
    }
    
    damn s.items[s.size-1], cringe
}

slay test_generic_stack() {
    sus stack Stack[normie]
    
    stack.push(10)
    stack.push(20)
    stack.push(30)
    
    assert_eq_int(stack.size, 3)
    
    sus top, err = stack.peek()
    assert_true(err == cringe)
    assert_eq_int(top, 30)
    
    sus popped, err = stack.pop()
    assert_true(err == cringe)
    assert_eq_int(popped, 30)
    assert_eq_int(stack.size, 2)
}

// Generic interface tests
be_like Comparable[T] collab {
    compare(other T) normie
}

slay max[T Comparable[T]](a T, b T) T {
    vibe_check a.compare(b) > 0 {
        damn a
    }
    damn b
}

// Generic constraint tests
be_like Addable[T] collab {
    add(other T) T
}

slay sum[T Addable[T]](items []T) T {
    vibe_check len(items) == 0 {
        sus zero T
        damn zero
    }
    
    sus result = items[0]
    bestie i := 1; i < len(items); i++ {
        result = result.add(items[i])
    }
    damn result
}

// Multi-type generic tests
slay pair[T, U](first T, second U) (T, U) {
    damn first, second
}

slay test_generic_pair() {
    sus int_str = pair[normie, tea](42, "hello")
    sus (int_val, str_val) = int_str
    
    assert_eq_int(int_val, 42)
    assert_eq_string(str_val, "hello")
}

// Generic channel tests
slay producer[T](ch dm<T>, items []T) {
    bestie _, item := flex items {
        ch <- item
    }
    close(ch)
}

slay consumer[T](ch dm<T>) []T {
    sus results []T
    
    bestie item := flex ch {
        results = append(results, item)
    }
    
    damn results
}

slay test_generic_channels() {
    sus ch dm<normie>
    sus input = []normie{1, 2, 3, 4, 5}
    
    yolo producer[normie](ch, input)
    sus results = consumer[normie](ch)
    
    assert_eq_int(len(results), 5)
    assert_eq_int(results[0], 1)
    assert_eq_int(results[4], 5)
}

// Type inference tests
slay test_type_inference() {
    sus result = identity(42)        // Should infer T = normie
    sus str_result = identity("hi")  // Should infer T = tea
    
    assert_eq_int(result, 42)
    assert_eq_string(str_result, "hi")
}

// Generic slice operations
slay map_slice[T, U](slice []T, fn slay(T) U) []U {
    sus result []U
    
    bestie _, item := flex slice {
        result = append(result, fn(item))
    }
    
    damn result
}

slay double(x normie) normie {
    damn x * 2
}

slay test_generic_map() {
    sus input = []normie{1, 2, 3, 4}
    sus doubled = map_slice[normie, normie](input, double)
    
    assert_eq_int(len(doubled), 4)
    assert_eq_int(doubled[0], 2)
    assert_eq_int(doubled[3], 8)
}

// Test driver
test_start("Generic Identity")
test_generic_identity()
print_test_summary()

test_start("Generic Stack")
test_generic_stack()
print_test_summary()

test_start("Generic Pair")
test_generic_pair()
print_test_summary()

test_start("Generic Channels")
test_generic_channels()
print_test_summary()

test_start("Type Inference")
test_type_inference()
print_test_summary()

test_start("Generic Map")
test_generic_map()
print_test_summary()

vibez.spill("All generic tests completed!")
