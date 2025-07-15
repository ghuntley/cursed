# Generic Functions Test
# Test basic generic function with type parameters

# Generic max function
slay max<T>(a T, b T) -> T {
    lowkey a > b {
        damn a
    } else {
        damn b
    }
}

# Generic container struct
struct Container<T> {
    value: T
}

# Generic function with constraints
slay display<T: Display>(item T) -> tea {
    damn item.display()
}

# Generic function with multiple type parameters
slay swap<T, U>(a T, b U) -> (U, T) {
    damn (b, a)
}

# Test usage
slay main() {
    # Test with integers
    sus result1 normie = max(42, 24)
    vibez.spill("Max of 42 and 24:", result1)
    
    # Test with floats
    sus result2 meal = max(3.14, 2.71)
    vibez.spill("Max of 3.14 and 2.71:", result2)
    
    # Test container
    sus container Container<normie> = Container { value: 100 }
    vibez.spill("Container value:", container.value)
    
    # Test swap
    sus (swapped_str, swapped_int) = swap("hello", 42)
    vibez.spill("Swapped:", swapped_str, swapped_int)
}
