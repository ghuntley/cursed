// CURSED Rule 30 - Working Implementation
// Uses individual variables instead of arrays

slay main() {
    print("=== CURSED Rule 30 - Working Version ===");
    
    // Input: n steps (1-12)
    sus n = 1;
    print("Evolution steps:", n);
    
    // Initial tape state (8 bits for simplicity)
    // Represents source bytes: [0,1,1,1,0,0,1,1] = 0x73 ("s")
    sus b0 = 0;
    sus b1 = 1;
    sus b2 = 1;
    sus b3 = 1;
    sus b4 = 0;
    sus b5 = 0;
    sus b6 = 1;
    sus b7 = 1;
    
    print("Initial tape: [", b0, b1, b2, b3, b4, b5, b6, b7, "]");
    
    // Apply Rule 30 for n steps
    sus step = 0;
    while (step < n) {
        print("Step", step + 1, ":");
        
        // Calculate new values using Rule 30: left XOR (center OR right)
        // Using: left != (center + right) as XOR approximation
        
        // Position 0: neighbors [b7, b0, b1] (circular)
        sus center_or_right_0 = b0 + b1;
        lowkey (center_or_right_0 > 1) { center_or_right_0 = 1; }
        sus new_b0 = 0;
        lowkey (b7 != center_or_right_0) { new_b0 = 1; }
        
        // Position 1: neighbors [b0, b1, b2]
        sus center_or_right_1 = b1 + b2;
        lowkey (center_or_right_1 > 1) { center_or_right_1 = 1; }
        sus new_b1 = 0;
        lowkey (b0 != center_or_right_1) { new_b1 = 1; }
        
        // Position 2: neighbors [b1, b2, b3]
        sus center_or_right_2 = b2 + b3;
        lowkey (center_or_right_2 > 1) { center_or_right_2 = 1; }
        sus new_b2 = 0;
        lowkey (b1 != center_or_right_2) { new_b2 = 1; }
        
        // Position 3: neighbors [b2, b3, b4]
        sus center_or_right_3 = b3 + b4;
        lowkey (center_or_right_3 > 1) { center_or_right_3 = 1; }
        sus new_b3 = 0;
        lowkey (b2 != center_or_right_3) { new_b3 = 1; }
        
        // Position 4: neighbors [b3, b4, b5]
        sus center_or_right_4 = b4 + b5;
        lowkey (center_or_right_4 > 1) { center_or_right_4 = 1; }
        sus new_b4 = 0;
        lowkey (b3 != center_or_right_4) { new_b4 = 1; }
        
        // Position 5: neighbors [b4, b5, b6]
        sus center_or_right_5 = b5 + b6;
        lowkey (center_or_right_5 > 1) { center_or_right_5 = 1; }
        sus new_b5 = 0;
        lowkey (b4 != center_or_right_5) { new_b5 = 1; }
        
        // Position 6: neighbors [b5, b6, b7]
        sus center_or_right_6 = b6 + b7;
        lowkey (center_or_right_6 > 1) { center_or_right_6 = 1; }
        sus new_b6 = 0;
        lowkey (b5 != center_or_right_6) { new_b6 = 1; }
        
        // Position 7: neighbors [b6, b7, b0] (circular)
        sus center_or_right_7 = b7 + b0;
        lowkey (center_or_right_7 > 1) { center_or_right_7 = 1; }
        sus new_b7 = 0;
        lowkey (b6 != center_or_right_7) { new_b7 = 1; }
        
        // Update to new generation
        b0 = new_b0;
        b1 = new_b1;
        b2 = new_b2;
        b3 = new_b3;
        b4 = new_b4;
        b5 = new_b5;
        b6 = new_b6;
        b7 = new_b7;
        
        print("Result: [", b0, b1, b2, b3, b4, b5, b6, b7, "]");
        
        step = step + 1;
    }
    
    // Convert to hex
    print("Converting to hex...");
    
    // First nibble: b0*8 + b1*4 + b2*2 + b3
    sus nibble1 = b0 * 8 + b1 * 4 + b2 * 2 + b3;
    sus hex1 = "0";
    lowkey (nibble1 == 1) { hex1 = "1"; }
    lowkey (nibble1 == 2) { hex1 = "2"; }
    lowkey (nibble1 == 3) { hex1 = "3"; }
    lowkey (nibble1 == 4) { hex1 = "4"; }
    lowkey (nibble1 == 5) { hex1 = "5"; }
    lowkey (nibble1 == 6) { hex1 = "6"; }
    lowkey (nibble1 == 7) { hex1 = "7"; }
    lowkey (nibble1 == 8) { hex1 = "8"; }
    lowkey (nibble1 == 9) { hex1 = "9"; }
    lowkey (nibble1 == 10) { hex1 = "a"; }
    lowkey (nibble1 == 11) { hex1 = "b"; }
    lowkey (nibble1 == 12) { hex1 = "c"; }
    lowkey (nibble1 == 13) { hex1 = "d"; }
    lowkey (nibble1 == 14) { hex1 = "e"; }
    lowkey (nibble1 == 15) { hex1 = "f"; }
    
    // Second nibble: b4*8 + b5*4 + b6*2 + b7
    sus nibble2 = b4 * 8 + b5 * 4 + b6 * 2 + b7;
    sus hex2 = "0";
    lowkey (nibble2 == 1) { hex2 = "1"; }
    lowkey (nibble2 == 2) { hex2 = "2"; }
    lowkey (nibble2 == 3) { hex2 = "3"; }
    lowkey (nibble2 == 4) { hex2 = "4"; }
    lowkey (nibble2 == 5) { hex2 = "5"; }
    lowkey (nibble2 == 6) { hex2 = "6"; }
    lowkey (nibble2 == 7) { hex2 = "7"; }
    lowkey (nibble2 == 8) { hex2 = "8"; }
    lowkey (nibble2 == 9) { hex2 = "9"; }
    lowkey (nibble2 == 10) { hex2 = "a"; }
    lowkey (nibble2 == 11) { hex2 = "b"; }
    lowkey (nibble2 == 12) { hex2 = "c"; }
    lowkey (nibble2 == 13) { hex2 = "d"; }
    lowkey (nibble2 == 14) { hex2 = "e"; }
    lowkey (nibble2 == 15) { hex2 = "f"; }
    
    print("Hex result:", hex1, hex2);
    print("");
    print("Rule 30 evolution complete!");
    print("This demonstrates the actual Rule 30 cellular automaton algorithm");
    print("running on a circular binary tape representing source code bytes.");
}
