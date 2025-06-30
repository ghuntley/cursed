// Fixed Rule 30 Implementation

slay main() {
    print("=== CURSED Rule 30 - Fixed Version ===");
    
    sus n = 1;
    print("Evolution steps:", n);
    
    // Initial: [0,1,1,1,0,0,1,1] (0x73)
    sus b0 = 0; sus b1 = 1; sus b2 = 1; sus b3 = 1;
    sus b4 = 0; sus b5 = 0; sus b6 = 1; sus b7 = 1;
    
    print("Initial: [", b0, b1, b2, b3, b4, b5, b6, b7, "]");
    
    sus step = 0;
    while (step < n) {
        print("Step", step + 1);
        
        // Rule 30: new_cell = left XOR (center OR right)
        // XOR: use comparison result directly 
        // OR: use max(center, right)
        
        // Position 0: [b7, b0, b1] = [1, 0, 1]
        sus or_01 = b0;
        lowkey (b1 > or_01) { or_01 = b1; }
        sus new_b0 = 0;
        lowkey (b7 != or_01) { new_b0 = 1; }
        
        // Position 1: [b0, b1, b2] = [0, 1, 1]  
        sus or_12 = b1;
        lowkey (b2 > or_12) { or_12 = b2; }
        sus new_b1 = 0;
        lowkey (b0 != or_12) { new_b1 = 1; }
        
        // Position 2: [b1, b2, b3] = [1, 1, 1]
        sus or_23 = b2;
        lowkey (b3 > or_23) { or_23 = b3; }
        sus new_b2 = 0;
        lowkey (b1 != or_23) { new_b2 = 1; }
        
        // Position 3: [b2, b3, b4] = [1, 1, 0]
        sus or_34 = b3;
        lowkey (b4 > or_34) { or_34 = b4; }
        sus new_b3 = 0;
        lowkey (b2 != or_34) { new_b3 = 1; }
        
        // Position 4: [b3, b4, b5] = [1, 0, 0]
        sus or_45 = b4;
        lowkey (b5 > or_45) { or_45 = b5; }
        sus new_b4 = 0;
        lowkey (b3 != or_45) { new_b4 = 1; }
        
        // Position 5: [b4, b5, b6] = [0, 0, 1]
        sus or_56 = b5;
        lowkey (b6 > or_56) { or_56 = b6; }
        sus new_b5 = 0;
        lowkey (b4 != or_56) { new_b5 = 1; }
        
        // Position 6: [b5, b6, b7] = [0, 1, 1]
        sus or_67 = b6;
        lowkey (b7 > or_67) { or_67 = b7; }
        sus new_b6 = 0;
        lowkey (b5 != or_67) { new_b6 = 1; }
        
        // Position 7: [b6, b7, b0] = [1, 1, 0]
        sus or_70 = b7;
        lowkey (b0 > or_70) { or_70 = b0; }
        sus new_b7 = 0;
        lowkey (b6 != or_70) { new_b7 = 1; }
        
        // Update
        b0 = new_b0; b1 = new_b1; b2 = new_b2; b3 = new_b3;
        b4 = new_b4; b5 = new_b5; b6 = new_b6; b7 = new_b7;
        
        print("Result: [", b0, b1, b2, b3, b4, b5, b6, b7, "]");
        step = step + 1;
    }
    
    // Convert to hex
    sus nibble1 = b0 * 8 + b1 * 4 + b2 * 2 + b3;
    sus nibble2 = b4 * 8 + b5 * 4 + b6 * 2 + b7;
    
    print("Nibbles:", nibble1, nibble2);
    
    // Convert nibbles to hex
    sus hex1 = "?"; sus hex2 = "?";
    
    lowkey (nibble1 == 0) { hex1 = "0"; }
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
    
    lowkey (nibble2 == 0) { hex2 = "0"; }
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
    
    print("Final hex result:", hex1, hex2);
    print("");
    print("✅ Rule 30 cellular automaton successfully implemented!");
}
