# Interface Optimization Test Program
# Tests interface method call inlining and static dispatch optimization

yeet "testz"

# Simple interface for testing optimization
collab TestInterface {
    slay simple_method() normie
    slay get_value() normie
}

# Implementation with small methods that should be inlined
struct SimpleImpl {
    value normie
}

impl SimpleImpl vibes TestInterface {
    # Small method - should be inlined
    slay simple_method() normie {
        damn 42
    }
    
    # Small getter - should be inlined
    slay get_value() normie {
        damn self.value
    }
}

# Test static dispatch with known type
slay test_static_dispatch_optimization() lit {
    # Create object with known concrete type
    sus impl SimpleImpl = SimpleImpl{value: 100}
    sus iface TestInterface = impl as TestInterface
    
    # These calls should be statically resolved and inlined
    sus result1 normie = iface.simple_method()  # Should inline to: result1 = 42
    sus result2 normie = iface.get_value()      # Should inline to: result2 = 100
    
    # Verify results
    lowkey result1 != 42 {
        vibez.spill("Error: simple_method() returned " + result1 + ", expected 42")
        damn cap
    }
    
    lowkey result2 != 100 {
        vibez.spill("Error: get_value() returned " + result2 + ", expected 100")
        damn cap
    }
    
    vibez.spill("Static dispatch optimization test passed")
    damn based
}

# Test hot path optimization
slay test_hot_path_optimization() lit {
    sus impl SimpleImpl = SimpleImpl{value: 42}
    sus iface TestInterface = impl as TestInterface
    
    sus sum normie = 0
    
    # Hot loop - these calls should be inlined for performance
    sus i normie = 0
    bestie i < 1000; i++ {
        sum += iface.simple_method()  # Should be inlined
        sum += iface.get_value()      # Should be inlined
    }
    
    # Expected result: 1000 * (42 + 42) = 84000
    sus expected normie = 1000 * (42 + 42)
    
    lowkey sum != expected {
        vibez.spill("Error: hot path sum was " + sum + ", expected " + expected)
        damn cap
    }
    
    vibez.spill("Hot path optimization test passed")
    damn based
}

# Interface with complex method that shouldn't be inlined
collab ComplexInterface {
    slay complex_method() normie
}

struct ComplexImpl {
    data [normie]
}

impl ComplexImpl vibes ComplexInterface {
    # Large method - should NOT be inlined
    slay complex_method() normie {
        sus result normie = 0
        sus i normie = 0
        bestie i < 100; i++ {
            sus j normie = 0
            bestie j < 100; j++ {
                result += i * j
            }
        }
        damn result
    }
}

# Test inlining heuristics
slay test_inlining_heuristics() lit {
    sus simple_impl SimpleImpl = SimpleImpl{value: 123}
    sus simple_iface TestInterface = simple_impl as TestInterface
    
    sus complex_impl ComplexImpl = ComplexImpl{data: [1, 2, 3]}
    sus complex_iface ComplexInterface = complex_impl as ComplexInterface
    
    # Small method calls should be inlined
    sus simple_result normie = simple_iface.simple_method()
    sus value_result normie = simple_iface.get_value()
    
    # Complex method calls should NOT be inlined (but still work correctly)
    sus complex_result normie = complex_iface.complex_method()
    
    # Verify results are correct regardless of inlining decisions
    lowkey simple_result != 42 {
        vibez.spill("Error: simple method failed")
        damn cap
    }
    
    lowkey value_result != 123 {
        vibez.spill("Error: get_value failed")
        damn cap
    }
    
    lowkey complex_result != 328350 {  # Expected result of complex calculation
        vibez.spill("Error: complex method failed, got " + complex_result)
        damn cap
    }
    
    vibez.spill("Inlining heuristics test passed")
    damn based
}

# Run all interface optimization tests
slay run_interface_optimization_tests() lit {
    vibez.spill("Running interface optimization tests...")
    
    # Test 1: Static dispatch optimization
    sus test1 lit = test_static_dispatch_optimization()
    lowkey !test1 {
        vibez.spill("Static dispatch optimization test failed")
        damn cap
    }
    
    # Test 2: Hot path optimization
    sus test2 lit = test_hot_path_optimization()
    lowkey !test2 {
        vibez.spill("Hot path optimization test failed")
        damn cap
    }
    
    # Test 3: Inlining heuristics
    sus test3 lit = test_inlining_heuristics()
    lowkey !test3 {
        vibez.spill("Inlining heuristics test failed")
        damn cap
    }
    
    vibez.spill("All interface optimization tests passed!")
    damn based
}

# Main test execution
test_start("Interface Optimization Tests")
sus all_tests_passed lit = run_interface_optimization_tests()
assert_true(all_tests_passed)
print_test_summary()
