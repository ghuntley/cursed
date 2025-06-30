vibe main

slay main() {
    sus a = 5; // 0101
    sus b = 3; // 0011
    
    sus xor_result = a ^ b; // Should be 0110 = 6
    sus or_result = a | b;  // Should be 0111 = 7
    
    vibez.spill("5 XOR 3 = ");
    vibez.spill(xor_result);
    vibez.spill("5 OR 3 = ");
    vibez.spill(or_result);
}
