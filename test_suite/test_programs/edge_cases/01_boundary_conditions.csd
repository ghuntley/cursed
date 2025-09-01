vibe main
yeet "vibez"

fr fr Boundary Conditions Test - Edge Values  
fr fr Tests: Zero values, extreme inputs, boundary arithmetic
fr fr Expected: Graceful handling of all boundary conditions

slay custom_abs(value normie) normie {
    ready value < 0 {
        damn -value
    }
    damn value
}

slay custom_max(a normie, b normie) normie {
    ready a > b {
        damn a
    }
    damn b
}

slay custom_min(a normie, b normie) normie {
    ready a < b {
        damn a
    }
    damn b
}

slay main() {
    vibez.spill("=== Boundary Conditions Test ===")
    
    vibez.spill("Testing zero values...")
    sus zero = 0
    vibez.spill("Zero value:", zero)
    vibez.spill("Zero abs:", custom_abs(zero))
    
    vibez.spill("Testing negative values...")
    sus negative = -999
    vibez.spill("Negative abs:", custom_abs(negative))
    
    vibez.spill("Testing string boundaries...")
    sus empty_str = ""
    vibez.spill("Empty string:", empty_str)
    
    sus single_char = "a"
    vibez.spill("Single char:", single_char)
    
    vibez.spill("Testing extreme arithmetic...")
    vibez.spill("Large + zero:", 999999 + zero)
    vibez.spill("Large - zero:", 999999 - zero)
    vibez.spill("Large * zero:", 999999 * zero)
    
    vibez.spill("Testing boundary math operations...")
    sus max_val = custom_max(999, -999)
    vibez.spill("Max(999, -999):", max_val)
    
    sus min_val = custom_min(999, -999)  
    vibez.spill("Min(999, -999):", min_val)
    
    vibez.spill("Testing zero division safety...")
    vibez.spill("999 / 1:", 999 / 1)
    vibez.spill("0 / 1:", zero / 1)
    
    vibez.spill("Boundary conditions test completed")
}
