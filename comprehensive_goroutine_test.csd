vibez.spill("=== CURSED Goroutine Compilation Test ===")

vibez.spill("1. Basic goroutine execution:")
stan {
    vibez.spill("  - Goroutine 1 executed")
}

vibez.spill("2. Multiple goroutines:")
stan {
    vibez.spill("  - Goroutine 2 executed")
}
stan {
    vibez.spill("  - Goroutine 3 executed")
}

vibez.spill("3. Goroutine with variable access:")
sus count drip = 42
stan {
    vibez.spill("  - Goroutine sees count:", count)
}

vibez.spill("4. Main thread execution:")
vibez.spill("  - Main thread completed")

vibez.spill("=== Test completed ===")
