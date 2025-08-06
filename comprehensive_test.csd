slay comprehensive_test() {
    // Test basic functionality
    sus x normie = 42
    sus message tea = "CURSED is working!"
    vibez.spill(message)
    
    // Test goroutines
    stan {
        vibez.spill("Goroutine executed!")
    }
    
    // Test functions
    slay inner_func(val normie) normie {
        damn val * 2
    }
    
    sus result normie = inner_func(21)
    vibez.spillf("Function result: {}", result)
    
    // Test error handling
    lowkey (result > 40) {
        vibez.spill("Success!")
    } highkey {
        vibez.spill("Error!")
    }
}

comprehensive_test()
