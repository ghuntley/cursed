vibe main

slay main() {
    vibez.spill("Testing Standard Library")
    vibez.spillf("Simple format: %s %d", "number", 42)
    
    tea formatted := vibez.spillstr("Value: %f", 3.14)
    vibez.spill(formatted)
}