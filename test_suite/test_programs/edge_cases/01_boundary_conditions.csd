vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

fr fr Boundary Conditions Test - Edge Values
fr fr Tests: Zero values, empty collections, extreme inputs
fr fr Expected: Graceful handling of all boundary conditions

slay main() {
    vibez.spill("=== Boundary Conditions Test ===")
    
    vibez.spill("Testing zero values...")
    sus zero = 0
    vibez.spill("Zero value:", zero)
    vibez.spill("Zero abs:", mathz.abs(zero))
    
    vibez.spill("Testing negative values...")
    sus negative = -999
    vibez.spill("Negative abs:", mathz.abs(negative))
    
    vibez.spill("Testing boundary values complete...")
    
    vibez.spill("Testing empty strings...")
    sus empty_str = ""
    vibez.spill("Empty string length:", stringz.length(empty_str))
    
    vibez.spill("Testing single character strings...")
    sus single_char = "a"
    vibez.spill("Single char:", single_char)
    vibez.spill("Single char length:", stringz.length(single_char))
    
    vibez.spill("Testing extreme arithmetic...")
    vibez.spill("Large + zero:", 999999 + zero)
    vibez.spill("Large - zero:", 999999 - zero)
    vibez.spill("Large * zero:", 999999 * zero)
    
    vibez.spill("Boundary conditions test completed")
}
