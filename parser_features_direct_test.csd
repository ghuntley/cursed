#!/usr/bin/env cursed

vibe test_parser_features

# Test specific parser features in isolation
slay test_basic_parsing() {
    # Test array with size expression
    sus size drip = 5
    sus array [size]normie
    
    # Test mutable pointer
    sus ptr *sus normie
    
    # Test slice
    sus slice []normie
    
    # Test error expression with source location
    try {
        yikes("Parser test error")
    } catch {
        vibez.spill("Error handled correctly")
    }
    
    vibez.spill("Basic parser features working")
}

slay main() {
    test_basic_parsing()
    vibez.spill("Parser features test complete")
}
