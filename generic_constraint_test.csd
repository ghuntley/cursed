yeet "vibez"

// Test basic generic function with constraint
slay identity[T](value T) T {
    damn value
}

// Test generic function with numeric constraint  
slay add_numbers[T: Numeric](a T, b T) T {
    damn a + b
}

// Test generic function with comparable constraint
slay max_value[T: Comparable](a T, b T) T {
    ready (a > b) {
        damn a
    } otherwise {
        damn b
    }
}

// Test generic struct with constraints
squad Container[T: Sized] {
    value T
    size drip
}

// Test interface constraint
collab Printable {
    slay print(self) vibes
}

slay print_item[T: Printable](item T) vibes {
    item.print()
}

// Test const generic with bounds
slay fixed_array[const N: drip](data []T) []T where N > 0, N < 1000 {
    // Array operations with fixed size N
    damn data
}

// Main function to test all constraints
slay main() vibes {
    // Test identity with different types
    sus int_val drip = identity(42)
    sus str_val tea = identity("hello")
    sus bool_val lit = identity(based)
    
    vibez.spill("Identity tests:")
    vibez.spill("Int:", int_val)
    vibez.spill("String:", str_val) 
    vibez.spill("Bool:", bool_val)
    
    // Test numeric constraints
    sus sum_int drip = add_numbers(10, 20)
    sus sum_float meal = add_numbers(3.14, 2.86)
    
    vibez.spill("Numeric constraint tests:")
    vibez.spill("Sum int:", sum_int)
    vibez.spill("Sum float:", sum_float)
    
    // Test comparable constraints
    sus max_int drip = max_value(100, 200)
    sus max_str tea = max_value("apple", "banana")
    
    vibez.spill("Comparable constraint tests:")
    vibez.spill("Max int:", max_int)
    vibez.spill("Max str:", max_str)
    
    // Test generic struct
    sus container Container[drip] = Container[drip]{
        value: 42,
        size: 8
    }
    
    vibez.spill("Generic struct test:")
    vibez.spill("Container value:", container.value)
    vibez.spill("Container size:", container.size)
    
    // Test const generic constraints  
    sus arr []drip = [1, 2, 3, 4, 5]
    sus fixed_arr []drip = fixed_array[5](arr)
    
    vibez.spill("Const generic test:")
    vibez.spill("Fixed array:", fixed_arr)
}

// Test constraint violations (should fail type checking)
// Uncomment these to test error handling:

/*
slay test_violations() vibes {
    // This should fail - trying to add non-numeric types
    sus bad_add = add_numbers("hello", "world")
    
    // This should fail - trying to compare non-comparable types
    sus bad_max = max_value(based, nocap)
    
    // This should fail - const generic bounds violation
    sus bad_array = fixed_array[-1](arr)  // N < 0
    sus huge_array = fixed_array[9999](arr)  // N >= 1000
    
    // This should fail - interface not implemented
    squad NonPrintable {
        data drip
    }
    
    sus non_printable = NonPrintable{data: 123}
    print_item(non_printable)  // NonPrintable doesn't implement Printable
}
*/
