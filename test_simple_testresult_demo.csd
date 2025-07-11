fr fr Simple TestResult Demo
fr fr Demonstrates basic TestResult functionality without complex imports

fr fr Use the existing testz module  
yeet "testz"

fr fr ================================
fr fr TestResult Demo
fr fr ================================

slay main() {
    vibez.spill("Starting Simple TestResult System Demo")
    vibez.spill("======================================")
    
    fr fr Test basic functionality
    test_start("Basic TestResult Demo")
    
    fr fr Test basic assertions
    assert_eq_int(42, 42)
    assert_eq_string("hello", "hello")
    assert_true(based)
    assert_false(cap)
    
    fr fr Test some failing assertions to show the system works
    assert_eq_int(1, 2)
    assert_eq_string("hello", "world")
    
    vibez.spill("")
    print_test_summary()
    
    vibez.spill("")
    vibez.spill("Simple TestResult System Demo Complete")
    vibez.spill("=====================================")
    
    lowkey get_failed_count() == 0 {
        vibez.spill("🎉 All tests passed! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed (expected for demo)")
    }
}
