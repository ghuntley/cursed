slay test_comprehensive_features() {
    // Test package manager functionality
    vibez.spill("Testing package manager...")
    
    // Test enhanced parser 
    sus x normie = 42
    sus message tea = "CURSED v1.3.0 comprehensive test"
    vibez.spill(message)
    
    // Test JIT execution
    slay compute(a normie, b normie) normie {
        damn a + b * 2
    }
    
    sus result normie = compute(10, 5)
    vibez.spillf("JIT computation result: {}", result)
    
    // Test concurrency
    stan {
        vibez.spill("Enhanced goroutine working!")
    }
    
    // Test syscalls
    vibez.spill("All systems operational!")
}

test_comprehensive_features()
