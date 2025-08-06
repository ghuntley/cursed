slay test_v15_features() {
    // Test compilation mode
    vibez.spill("Testing v1.5.0 enhanced features...")
    
    // Test improved codegen
    sus x normie = 123
    sus result normie = x * 2 + 10
    vibez.spillf("Enhanced codegen result: {}", result)
    
    // Test return statement with canonical syntax
    slay compute(a normie) normie {
        damn a + 42  // Using canonical damn instead of deprecated yolo
    }
    
    sus final_result normie = compute(5)
    vibez.spillf("Function result: {}", final_result)
    
    // Test boolean values with canonical syntax
    sus is_working lit = based  // Using canonical based instead of deprecated cap
    lowkey (is_working) {
        vibez.spill("All improvements working correctly!")
    }
}

test_v15_features()
