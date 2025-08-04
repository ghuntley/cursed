slay calculate_stats(x normie, y normie) {
    sus sum normie = x + y
    sus diff normie = x - y
    sus product normie = x * y
    
    vibez.spill("Input values:", x, y)
    vibez.spill("Sum:", sum, "Difference:", diff)
    vibez.spill("Product:", product)
    
    nah (sum > 100) {
        vibez.spill("Large sum detected:", sum)
    } else {
        vibez.spill("Normal range sum:", sum)
    }
}

slay main_character() {
    vibez.spill("CURSED Multi-Argument Output Test")
    vibez.spill("=" * 40)  // This might not work yet, but testing
    
    calculate_stats(42, 73)
    calculate_stats(15, 8)
    
    vibez.spill("Test completed successfully!")
}
