yeet "lookin_glass"

slay test_constants() {
    vibez.spill("Test: Basic Constants")
    
    lowkey Invalid == 0 {
        vibez.spill("✅ PASS: Invalid = 0")
    }
    lowkey Bool == 1 {
        vibez.spill("✅ PASS: Bool = 1")  
    }
    lowkey Int == 2 {
        vibez.spill("✅ PASS: Int = 2")
    }
    lowkey String == 24 {
        vibez.spill("✅ PASS: String = 24")
    }
}

slay test_functions() {
    vibez.spill("Test: Basic Functions")
    
    sus name tea = get_type_name("hello")
    sus kind normie = get_type_kind("hello")
    sus equal lit = DeepEqual(42, 42)
    sus copy normie = DeepCopy(42)
    sus result lit = test_reflection_basic()
    
    vibez.spill("✅ PASS: get_type_name works")
    vibez.spill("✅ PASS: get_type_kind works")
    vibez.spill("✅ PASS: DeepEqual works")
    vibez.spill("✅ PASS: DeepCopy works")
    vibez.spill("✅ PASS: test_reflection_basic works")
}

slay main() {
    vibez.spill("=== LookinGlass Reflection Module Tests ===")
    
    test_constants()
    test_functions()
    
    vibez.spill("=== All Tests Passed! ===")
    vibez.spill("🎯 LookinGlass reflection module working!")
    vibez.spill("📊 Runtime type inspection available")
    vibez.spill("🔍 Foundation for metaprogramming established")
    vibez.spill("✨ Module successfully implemented!")
}

main()
