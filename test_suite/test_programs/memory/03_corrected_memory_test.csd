vibe main
yeet "vibez"

fr fr Corrected Memory Test - Basic Operations
fr fr Tests: Simple memory-related operations with correct syntax
fr fr Expected: Successful execution without memory issues

slay main() {
    vibez.spill("=== Corrected Memory Test ===")
    
    vibez.spill("Testing simple variable allocation...")
    sus num1 = 100
    sus num2 = 200
    sus num3 = 300
    
    vibez.spill("Variables:", num1, num2, num3)
    
    vibez.spill("Testing string allocation...")
    sus text1 = "First"
    sus text2 = "Second"
    sus text3 = "Third"
    
    vibez.spill("Strings:", text1, text2, text3)
    
    vibez.spill("Testing computational memory usage...")
    sus counter = 0
    periodt (counter < 5) {
        sus temp = counter * counter
        vibez.spill("Iteration", counter, "result:", temp)
        counter = counter + 1
    }
    
    vibez.spill("Testing nested scope memory...")
    ready (num1 > 50) {
        sus inner_var = num1 + num2
        vibez.spill("Inner computation:", inner_var)
        
        ready (inner_var > 250) {
            sus deep_var = inner_var + num3
            vibez.spill("Deep computation:", deep_var)
        }
    }
    
    vibez.spill("Corrected memory test completed")
}
