// Test core CURSED features without constants

slay simpleTest() {
    // Test arrays and array indexing
    sus numbers []normie = [1, 2, 3, 4, 5]
    sus first normie = numbers[0]
    vibez.spill("First number:")
    vibez.spill(first)
    
    // Test for-in loops
    bestie num in numbers {
        vibez.spill("Processing number:")
        vibez.spill(num)
    }
    
    // Test character type
    sus letter sip = 'A'
    sus word tea = letter + "dvanced"
    vibez.spill(word)
    
    // Test mixed arithmetic
    sus integer normie = 10
    sus float normie = 3.14
    sus result normie = integer * float
    vibez.spill("Mixed arithmetic result:")
    vibez.spill(result)
    
    // Test tuples
    sus coords normie = (5, 10, 15)
    sus x normie = coords.0
    sus y normie = coords.1
    sus z normie = coords.2
    vibez.spill("Tuple values:")
    vibez.spill(x)
    vibez.spill(y)
    vibez.spill(z)
    
    // Test boolean operations
    sus flag1 lit = based
    sus flag2 lit = sus
    lowkey flag1 == based {
        vibez.spill("Boolean test passed!")
    }
    
    // Test simple comparison without constants
    lowkey 100 > 50 {
        vibez.spill("Comparison test passed!")
    }
    
    yolo 42
}

slay main() {
    vibez.spill("Testing core CURSED features...")
    sus testResult normie = simpleTest()
    vibez.spill("Test completed with result:")
    vibez.spill(testResult)
}
