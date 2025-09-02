vibe main
yeet "vibez"

slay main() {
    vibez.spill("Testing runtime abs function directly...")
    
    // Test normal case with basic arithmetic
    sus negative drip = -42
    sus manual_abs drip = -negative  
    vibez.spill("Manual abs of -42:")
    vibez.spill(manual_abs)
    
    // Test case that would cause garbage values  
    sus problematic drip = -2147483648
    sus problematic_abs drip = -problematic
    vibez.spill("Manual abs of -2147483648 (may be garbage):")
    vibez.spill(problematic_abs)
}
