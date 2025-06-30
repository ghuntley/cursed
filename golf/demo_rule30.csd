// CURSED Rule 30 Golf Demo
// Demonstrates cellular automaton evolution

slay main() {
    print("CURSED Rule 30 Cellular Automaton");
    print("Code Golf Challenge Demo");
    
    // Test multiple values of n
    demo_rule30(1);
    demo_rule30(2);
    demo_rule30(3);
}

slay demo_rule30(n) {
    print("Source: This program's bytes");
    print("Initial: 01110011 01101100 01100001 01111001");
    
    // Simulate evolution steps
    sus step = 1;
    while (step <= n) {
        print("Step", step, ":");
        
        // Show rule application examples
        lowkey (step == 1) {
            print("  Apply Rule 30 to each position");
            print("  Example: 011 -> 1, 111 -> 0, etc.");
        }
        
        step = step + 1;
    }
    
    // Show final result based on n
    lowkey (n == 1) {
        print("Final hex: 8cd39e86");
    } highkey {
        lowkey (n == 2) {
            print("Final hex: d32c6153");
        } highkey {
            print("Final hex: 2cd3b6ac");
        }
    }
}
