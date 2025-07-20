yeet "testz"

test_start("Simple Performance Test")

slay test_basic_performance() lit {
    vibez.spill("Testing basic compiler performance...")
    
    // Simple arithmetic
    sus a drip = 42
    sus b drip = 13
    sus result drip = a + b
    
    assert_eq_int(result, 55)
    
    // Simple function
    slay add(x drip, y drip) drip {
        damn x + y
    }
    
    sus function_result drip = add(10, 20)
    assert_eq_int(function_result, 30)
    
    // Simple loop
    sus sum drip = 0
    bestie i drip in 0..5 {
        sum = sum + i
    }
    assert_eq_int(sum, 10)
    
    vibez.spill("Basic performance test completed")
    damn based
}

test_basic_performance()
print_test_summary()
