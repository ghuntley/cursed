// Test channel select runtime integration
vibez.spill("Testing channel select runtime...")

// Create a channel
sus ch chan normie

// Test ready statement (select)
ready {
    ch <- 42 -> vibez.spill("Sent value 42")
    default -> vibez.spill("No operation ready")
}

vibez.spill("Channel select test complete")
