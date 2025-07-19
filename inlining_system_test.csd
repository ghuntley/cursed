yeet "testz"

// Test function inlining system
slay simple_add(a drip, b drip) drip {
    damn a + b
}

slay small_function(x drip) drip {
    damn x * 2
}

slay leaf_function() drip {
    damn 42
}

// Generic function that should be inlined
slay generic_multiply<T>(a T, b T) T {
    damn a * b
}

// Interface for testing interface inlining
interface Addable {
    slay add(self, other Self) Self
}

struct SimpleNum {
    value drip
}

impl Addable for SimpleNum {
    slay add(self, other SimpleNum) SimpleNum {
        damn SimpleNum { value: self.value + other.value }
    }
}

slay test_basic_inlining() {
    test_start("Basic function inlining")
    
    // These calls should be inlined
    sus result1 drip = simple_add(5, 3)
    assert_eq_int(result1, 8)
    
    sus result2 drip = small_function(10)
    assert_eq_int(result2, 20)
    
    sus result3 drip = leaf_function()
    assert_eq_int(result3, 42)
    
    // Test that small functions are candidates for inlining
    vibez.spill("Basic inlining test passed")
}

slay test_generic_inlining() {
    test_start("Generic function inlining")
    
    // Generic function calls that should be inlined after monomorphization
    sus int_result drip = generic_multiply<drip>(4, 5)
    assert_eq_int(int_result, 20)
    
    sus float_result meal = generic_multiply<meal>(2.5, 4.0)
    assert_true(float_result == 10.0)
    
    vibez.spill("Generic inlining test passed")
}

slay test_interface_inlining() {
    test_start("Interface method inlining")
    
    // Interface method calls that could be devirtualized and inlined
    sus num1 SimpleNum = SimpleNum { value: 10 }
    sus num2 SimpleNum = SimpleNum { value: 15 }
    sus result SimpleNum = num1.add(num2)
    
    assert_eq_int(result.value, 25)
    
    vibez.spill("Interface inlining test passed")
}

slay performance_test() {
    test_start("Inlining performance test")
    
    // Test that shows performance gain from inlining
    sus iterations drip = 1000
    sus sum drip = 0
    
    // Loop that should benefit from inlining
    loop i := 0; i < iterations; i += 1 {
        sum += simple_add(i, leaf_function())
    }
    
    assert_true(sum > 0)
    vibez.spill("Performance test completed with sum: " + sum.to_string())
}

// Main test runner
test_basic_inlining()
test_generic_inlining()
test_interface_inlining()
performance_test()
print_test_summary()

vibez.spill("Function inlining system test completed!")
