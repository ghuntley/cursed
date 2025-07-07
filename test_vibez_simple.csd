// Simple test of vibez module
yeet "vibez"

slay main() {
    vibez.println("Testing vibez module...")
    
    // Test basic output
    vibez.spill("Hello, ")
    vibez.spill("World!")
    vibez.println("")
    
    // Test integer output
    vibez.spill("Number: ")
    vibez.spill_int(42)
    vibez.println("")
    
    // Test boolean output
    vibez.spill("Flag: ")
    vibez.spill_bool(based)
    vibez.println("")
    
    // Test character output
    vibez.spill("Character: ")
    vibez.spill_char('X')
    vibez.println("")
    
    vibez.println("Vibez module test complete!")
}

main()
