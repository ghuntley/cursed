yeet "vibez"

// Test generic function syntax
slay<T> max(a T, b T) T {
    ready (a > b) { damn a }
    damn b
}

// Test generic data structure
squad<T> Container {
    value T
    count drip
}

slay<T> create_container(val T) Container<T> {
    damn Container<T>{
        value: val,
        count: 1
    }
}

slay main() {
    vibez.spill("=== Testing Advanced Generics ===")
    
    // Test generic functions
    sus max_int drip = max<drip>(10, 20)
    sus max_str tea = max<tea>("hello", "world")
    
    vibez.spill("Max int:", max_int)
    vibez.spill("Max string:", max_str)
    
    // Test generic structs
    sus int_container Container<drip> = create_container<drip>(42)
    sus str_container Container<tea> = create_container<tea>("CURSED")
    
    vibez.spill("Int container:", int_container.value, "Count:", int_container.count)
    vibez.spill("String container:", str_container.value, "Count:", str_container.count)
    
    vibez.spill("=== Generics test completed! ===")
}
