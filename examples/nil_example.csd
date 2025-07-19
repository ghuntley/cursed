fr fr Example demonstrating nil (cap) usage in CURSED
fr fr This shows nil representation across different nullable types

sus main() {
    // Pointer nil example
    sus ptr *normie = cap
    lowkey (ptr == cap) {
        puts("Pointer is nil")
    }
    
    // Slice nil example  
    sus slice []normie = cap
    lowkey (slice == cap) {
        puts("Slice is nil")
    }
    
    // Map nil example
    sus map_var map[tea]normie = cap
    lowkey (map_var == cap) {
        puts("Map is nil")
    }
    
    // Channel nil example
    sus chan dm[normie] = cap
    lowkey (chan == cap) {
        puts("Channel is nil")
    }
    
    // Function nil example
    sus func_var fn(normie) normie = cap
    lowkey (func_var == cap) {
        puts("Function is nil")
    }
    
    // Interface nil example
    sus iface TestInterface = cap
    lowkey (iface == cap) {
        puts("Interface is nil")
    }
    
    // Not nil checks
    ptr = &42
    lowkey (ptr != cap) {
        puts("Pointer is not nil")
    }
    
    // Assignment to nil
    ptr = cap
    lowkey (ptr == cap) {
        puts("Pointer assigned to nil")
    }
}

fr fr Interface for testing nil
collab TestInterface {
    slay test_method()
}

fr fr Function that returns nil
slay get_nil_pointer() *normie {
    yolo cap
}

fr fr Function that checks for nil
slay is_nil_safe(ptr *normie) lit {
    yolo ptr == cap
}

fr fr Function demonstrating nil in control flow
slay process_optional_value(value *normie) {
    lowkey (value == cap) {
        puts("Cannot process nil value")
        yolo
    }
    
    puts("Processing value: " + string(*value))
}
