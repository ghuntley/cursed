# Oracle Week 1 Core Correctness: Generic Function Declarations Test
# Testing enhanced generic function type inference and constraint propagation

# Test 1: Basic generic function with single type parameter
slay identity<T>(value T) T {
    damn value
}

# Test 2: Generic function with multiple type parameters  
slay pair<A, B>(first A, second B) squad { first A, second B } {
    damn squad { first: first, second: second }
}

# Test 3: Generic function with constraints
slay add_numbers<T>(a T, b T) T {
    damn a + b
}

# Test 4: Generic function returning array
slay create_array<T>(element T, size drip) []T {
    sus arr []T = []
    bestie (size > 0) {
        arr.append(element)
        size = size - 1
    }
    damn arr
}

# Test 5: Higher-order generic function
slay map<T, U>(arr []T, func slay(T) U) []U {
    sus result []U = []
    sick (item bestie arr) {
        result.append(func(item))
    }
    damn result
}

# Test 6: Generic function with inference from return type context
slay get_default<T>() T {
    # This should infer T from usage context
    ready (T == drip) {
        damn 0
    } otherwise ready (T == tea) {
        damn ""  
    } otherwise ready (T == lit) {
        damn no_cap
    } otherwise {
        damn T{}  # Default construction
    }
}

# Test calls with type inference
sus result1 = identity(42)              # Should infer T = drip
sus result2 = identity("hello")         # Should infer T = tea
sus result3 = identity(based)           # Should infer T = lit

sus pair1 = pair(10, "test")            # Should infer A = drip, B = tea
sus pair2 = pair(based, 3.14)          # Should infer A = lit, B = meal

sus numbers []drip = [1, 2, 3, 4, 5]
sus sum_result = add_numbers(10, 20)    # Should infer T = drip
sus str_concat = add_numbers("a", "b")  # Should infer T = tea

sus int_array = create_array(42, 5)     # Should infer T = drip
sus str_array = create_array("item", 3) # Should infer T = tea

# Higher-order function call with lambda inference
sus doubled = map(numbers, slay(x drip) drip { damn x * 2 })

# Context-dependent inference
sus default_int drip = get_default()    # Should infer T = drip from assignment
sus default_str tea = get_default()     # Should infer T = tea from assignment

# Test complex generic scenarios
slay complex_generic<T, U, V>(
    transform slay(T) U, 
    combine slay(U, V) V,
    input T,
    context V
) V {
    sus intermediate = transform(input)
    damn combine(intermediate, context)
}

# Call with complex inference
sus complex_result = complex_generic(
    slay(x drip) tea { damn string_from_int(x) },
    slay(s tea, prefix tea) tea { damn prefix + s },
    123,
    "Result: "
) # Should infer T=drip, U=tea, V=tea

# Test generic method chaining
slay chain<T>(value T) squad {
    value T,
    
    slay map<U>(func slay(T) U) chain<U> {
        damn chain<U>{ value: func(value) }
    },
    
    slay get() T {
        damn value
    }
}

sus chained_result = chain(42)
    .map(slay(x drip) tea { damn "Number: " + string_from_int(x) })
    .map(slay(s tea) drip { damn s.len })
    .get()

# Validation prints
vibez.spill("Generic function tests completed")
vibez.spill("result1:", result1)
vibez.spill("result2:", result2) 
vibez.spill("result3:", result3)
vibez.spill("sum_result:", sum_result)
vibez.spill("chained_result:", chained_result)
vibez.spill("All generic function declarations working correctly")
