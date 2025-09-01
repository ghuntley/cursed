vibe main
yeet "vibez"

fr fr Error Handling Test - Corrected Syntax
fr fr Tests: Basic error condition detection and handling
fr fr Expected: Proper error messages and recovery

slay main() {
    vibez.spill("=== Error Handling Test ===")
    
    sus value1 = 10
    sus value2 = 2
    sus value3 = 0
    
    vibez.spill("Normal division test:")
    vibez.spill("Result:", value1 / value2)
    
    vibez.spill("Testing error conditions...")
    
    ready (value3 == 0) {
        vibez.spill("Error detected: Division by zero prevented")
    }
    
    ready (value1 < 0) {
        vibez.spill("Error detected: Negative input")
    } otherwise {
        vibez.spill("Input validation passed")
    }
    
    vibez.spill("Testing validation function...")
    sus test_num = -42
    
    ready (test_num < 0) {
        vibez.spill("Negative number detected:", test_num)
        test_num = 0 - test_num
        vibez.spill("Converted to positive:", test_num)
    }
    
    vibez.spill("Error handling test completed")
}
