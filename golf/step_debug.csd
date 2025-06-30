// Step by step debug of the issues

slay main() {
    print("=== Debug Step by Step ===");
    
    // Test 1: XOR operation
    sus a = 1;
    sus b = 0;
    sus result = 0;
    lowkey (a != b) {
        result = 1;
        print("XOR test: 1 != 0 =", result);
    }
    
    // Test 2: OR operation using max
    sus x = 1;
    sus y = 0;
    sus or_result = x;
    lowkey (y > or_result) { or_result = y; }
    print("OR test: max(1,0) =", or_result);
    
    // Test 3: Hex conversion
    sus nibble = 7;
    sus hex = "unknown";
    
    print("Testing nibble", nibble);
    lowkey (nibble == 7) { 
        hex = "7";
        print("Found nibble 7, setting hex to", hex);
    }
    
    // Test 4: One Rule 30 step manually
    print("Manual Rule 30 test:");
    print("Position 0: [1,0,1] -> OR(0,1)=1, XOR(1,1)=0");
    
    sus left = 1;
    sus center = 0; 
    sus right = 1;
    
    sus or_val = center;
    lowkey (right > or_val) { or_val = right; }
    print("OR result:", or_val);
    
    sus xor_val = 0;
    lowkey (left != or_val) { xor_val = 1; }
    print("XOR result:", xor_val);
    
    print("Expected: 0, Got:", xor_val);
}
