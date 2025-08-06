slay test_comprehensive_v14() {
    // Test FFI-free stdlib
    vibez.spill("Testing pure CURSED stdlib...")
    
    // Test enhanced codegen
    sus x normie = 42
    sus message tea = "CURSED v1.4.0 comprehensive test"
    vibez.spill(message)
    
    // Test JIT improvements
    slay compute_advanced(a normie, b normie) normie {
        sus result normie = a * b + 10
        damn result
    }
    
    sus computation normie = compute_advanced(5, 8)
    vibez.spillf("Enhanced JIT result: {}", computation)
    
    // Test goroutines
    stan {
        vibez.spill("Advanced goroutine with memory safety!")
    }
    
    // Test error handling
    lowkey (computation > 30) {
        vibez.spill("Advanced features working!")
    } highkey {
        vibez.spill("Something wrong!")
    }
}

test_comprehensive_v14()
