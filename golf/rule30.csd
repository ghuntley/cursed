// Rule 30 Cellular Automaton - Reference Implementation
// Reads own source code and evolves using Rule 30

slay main() {
    // Get input n
    sus n = read_input();
    
    // Read own source file
    sus source_bytes = read_own_file();
    
    // Convert to binary tape
    sus binary_tape = bytes_to_binary(source_bytes);
    
    // Evolve for n steps
    sus i = 0;
    while (i < n) {
        binary_tape = evolve_rule30(binary_tape);
        i = i + 1;
    }
    
    // Convert to hex and print
    sus hex_result = binary_to_hex(binary_tape);
    print(hex_result);
}

slay read_input() {
    // For now, hardcode test values
    // In real version, this would read from stdin or args
    return 1;
}

slay read_own_file() {
    // Simulate reading own source code
    // In real implementation, would read __FILE__ or similar
    sus example_bytes = [0x73, 0x6C, 0x61, 0x79]; // "slay" 
    return example_bytes;
}

slay bytes_to_binary(bytes) {
    sus binary = [];
    sus i = 0;
    
    while (i < length(bytes)) {
        sus byte_val = bytes[i];
        sus bit = 7;
        
        while (bit >= 0) {
            sus bit_val = (byte_val >> bit) & 1;
            binary = append(binary, bit_val);
            bit = bit - 1;
        }
        i = i + 1;
    }
    
    return binary;
}

slay evolve_rule30(tape) {
    sus len = length(tape);
    sus new_tape = [];
    sus i = 0;
    
    while (i < len) {
        // Get neighbors (circular)
        sus left = tape[(i - 1 + len) % len];
        sus center = tape[i];
        sus right = tape[(i + 1) % len];
        
        // Apply Rule 30: new_cell = left XOR (center OR right)
        sus new_cell = left ^ (center | right);
        new_tape = append(new_tape, new_cell);
        
        i = i + 1;
    }
    
    return new_tape;
}

slay binary_to_hex(binary) {
    sus hex_chars = "0123456789abcdef";
    sus result = "";
    sus i = 0;
    
    // Pad to multiple of 4 bits
    while (length(binary) % 4 != 0) {
        binary = append(binary, 0);
    }
    
    while (i < length(binary)) {
        sus val = binary[i] * 8 + binary[i+1] * 4 + binary[i+2] * 2 + binary[i+3];
        result = result + hex_chars[val];
        i = i + 4;
    }
    
    return result;
}

// Helper functions (would need to be implemented in stdlib)
slay length(arr) {
    return 4; // Placeholder
}

slay append(arr, item) {
    return arr; // Placeholder  
}
