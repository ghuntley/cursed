# Complete test for monomorphization system integration

# Generic function with void handling
slay identity<T>(value T) T {
    damn value
}

# Generic function with constraints
slay max<T>(a T, b T) T {
    lowkey a > b {
        damn a
    } sheesh {
        damn b
    }
}

# Generic struct
struct Container<T> {
    value T
    count normie
}

# Test program that triggers monomorphization
slay main() {
    # Test generic function with different types
    sus int_result normie = identity(42)
    vibez.spill("Integer identity:")
    vibez.spill(int_result)
    
    sus string_result tea = identity("hello world")
    vibez.spill("String identity:")
    vibez.spill(string_result)
    
    sus bool_result lit = identity(based)
    vibez.spill("Boolean identity:")
    vibez.spill(bool_result)
    
    # Test max function
    sus max_int normie = max(10, 20)
    vibez.spill("Max int result:")
    vibez.spill(max_int)
    
    sus max_str tea = max("apple", "banana")
    vibez.spill("Max string result:")
    vibez.spill(max_str)
    
    vibez.spill("Monomorphization test complete!")
}
