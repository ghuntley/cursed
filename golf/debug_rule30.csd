// Debug Rule 30 Implementation
// Step-by-step verification of the algorithm

slay main() {
    print("=== Rule 30 Debug ===");
    
    // Test simple case: [0,0,0,1,0,0,0,0]
    print("Initial: [0,0,0,1,0,0,0,0]");
    
    // Position 3: neighbors [0,1,0] 
    // center OR right = 1 OR 0 = 1
    // left XOR (center OR right) = 0 XOR 1 = 1
    sus left = 0;
    sus center = 1; 
    sus right = 0;
    sus center_or_right = center + right;
    lowkey (center_or_right > 1) { center_or_right = 1; }
    sus result = 0;
    lowkey (left != center_or_right) { result = 1; }
    print("Position 3: [", left, center, right, "] -> center_or_right =", center_or_right, "-> result =", result);
    
    // Position 2: neighbors [0,0,1]
    // center OR right = 0 OR 1 = 1  
    // left XOR (center OR right) = 0 XOR 1 = 1
    left = 0;
    center = 0;
    right = 1;
    center_or_right = center + right;
    lowkey (center_or_right > 1) { center_or_right = 1; }
    result = 0;
    lowkey (left != center_or_right) { result = 1; }
    print("Position 2: [", left, center, right, "] -> center_or_right =", center_or_right, "-> result =", result);
    
    // Position 4: neighbors [1,0,0]
    // center OR right = 0 OR 0 = 0
    // left XOR (center OR right) = 1 XOR 0 = 1  
    left = 1;
    center = 0;
    right = 0;
    center_or_right = center + right;
    lowkey (center_or_right > 1) { center_or_right = 1; }
    result = 0;
    lowkey (left != center_or_right) { result = 1; }
    print("Position 4: [", left, center, right, "] -> center_or_right =", center_or_right, "-> result =", result);
    
    print("");
    print("Expected: [0,0,1,1,1,0,0,0]");
    print("Rule 30 formula: left XOR (center OR right)");
    print("XOR using: left != operand");
    print("OR using: center + right, capped at 1");
}
