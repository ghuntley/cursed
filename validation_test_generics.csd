// Test generic functions and types
slay generic_swap[T](a T, b T) (T, T) {
    damn b, a
}

slay generic_max[T](a T, b T) T {
    ready (a > b) {
        damn a
    }
    damn b
}

squad Container[T] {
    spill value T
    
    slay get() T {
        damn value
    }
    
    slay set(new_value T) {
        value = new_value
    }
}

slay main() {
    // Test generic functions
    sus x drip = 10
    sus y drip = 20
    sus swapped_x, swapped_y = generic_swap[drip](x, y)
    vibez.spill("Swapped:", swapped_x, swapped_y)
    
    sus max_val drip = generic_max[drip](15, 25)
    vibez.spill("Max:", max_val)
    
    // Test generic structs
    sus container Container[drip] = Container[drip]{value: 42}
    vibez.spill("Container value:", container.get())
    
    container.set(99)
    vibez.spill("Updated value:", container.get())
}

main()
