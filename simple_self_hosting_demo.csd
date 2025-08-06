#!/usr/bin/env cursed
# Simple Self-Hosting Demo
# Demonstrates basic self-compilation capabilities

yeet "vibez"

vibe "simple_self_hosting_demo"

# Simple function to test compilation
slay greet(name tea) {
    vibez.spill("Hello, " + name + "!")
}

# Simple arithmetic test
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

# Simple control flow test
slay test_control_flow(value normie) tea {
    lowkey (value > 10) {
        damn "large"
    } highkey lowkey (value > 5) {
        damn "medium"
    } highkey {
        damn "small"
    }
}

# Main function
slay main() {
    vibez.spill("=== Simple Self-Hosting Demo ===")
    
    # Test function calls
    greet("Self-Hosting")
    
    # Test arithmetic
    sus result normie = add_numbers(15, 25)
    vibez.spill("15 + 25 = " + result)
    
    # Test control flow
    sus size_description tea = test_control_flow(8)
    vibez.spill("Size: " + size_description)
    
    # Test basic variables
    sus message tea = "Self-hosting works!"
    sus count normie = 42
    sus active lit = based
    
    vibez.spill("Message: " + message)
    vibez.spill("Count: " + count)
    vibez.spill("Active: " + active)
    
    vibez.spill("=== Demo completed successfully ===")
}
