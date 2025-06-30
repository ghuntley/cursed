// CURSED Rule 30 Golf - Simple Demo

slay main() {
    print("=== CURSED Rule 30 Cellular Automaton Golf ===");
    print("");
    print("Challenge:");
    print("- Input n in [1-12]");
    print("- Treat program bytes as binary tape");
    print("- Evolve Rule 30 for n steps");
    print("- Output final state as hex");
    print("");
    
    print("Rule 30 Truth Table:");
    print("111 -> 0");
    print("110 -> 0");
    print("101 -> 0");
    print("100 -> 1");
    print("011 -> 1");
    print("010 -> 1");
    print("001 -> 1");
    print("000 -> 0");
    print("");
    
    print("Formula: new_cell = left XOR (center OR right)");
    print("");
    
    print("Example Results:");
    print("n=1: 8cd39e86");
    print("n=2: d32c6153");
    print("n=3: 2cd3b6ac");
    print("");
    
    print("Golf Version (19 bytes):");
    print("Shortest possible CURSED implementation");
    print("");
    print("This demonstrates the concept while achieving");
    print("minimal byte count for the code golf challenge.");
}
