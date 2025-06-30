# CURSED Conversion Utilities

This module provides utility functions for the CURSED Rule 30 cellular automaton golf challenge, specifically for converting between bytes, binary, and hexadecimal representations.

## Functions Provided

### 1. `bytes_to_binary(byte_array) -> bit_array`

Converts an array of bytes to a flat binary bit array where each byte becomes 8 bits.

**Algorithm:**
```cursed
slay bytes_to_binary(byte_array) {
    sus bit_array = [];
    sus i = 0;
    bestie (i < byte_array.length) {
        sus byte = byte_array[i];
        
        // Extract 8 bits from byte (MSB first)
        bit_array.push((byte / 128) % 2);  // bit 7 (MSB)
        bit_array.push((byte / 64) % 2);   // bit 6
        bit_array.push((byte / 32) % 2);   // bit 5
        bit_array.push((byte / 16) % 2);   // bit 4
        bit_array.push((byte / 8) % 2);    // bit 3
        bit_array.push((byte / 4) % 2);    // bit 2
        bit_array.push((byte / 2) % 2);    // bit 1
        bit_array.push(byte % 2);          // bit 0 (LSB)
        
        i = i + 1;
    }
    yolo bit_array;
}
```

**Example:**
- Input: `[115, 108, 97, 121]` ("Slay" in ASCII)
- Output: `[0,1,1,1,0,0,1,1, 0,1,1,0,1,1,0,0, 0,1,1,0,0,0,0,1, 0,1,1,1,1,0,0,1]`
- Binary string: `"01110011011011000110000101111001"`

### 2. `binary_to_hex(bit_array) -> hex_string`

Converts a binary bit array to a hexadecimal string (lowercase, no 0x prefix).

**Algorithm:**
```cursed
slay binary_to_hex(bit_array) {
    sus hex_string = "";
    
    // Pad to multiple of 4 if needed
    sus padded_bits = bit_array.clone();
    bestie (padded_bits.length % 4 != 0) {
        padded_bits.push(0);
    }
    
    // Process 4 bits at a time
    sus i = 0;
    bestie (i + 3 < padded_bits.length) {
        sus nibble = padded_bits[i] * 8 + padded_bits[i+1] * 4 + 
                     padded_bits[i+2] * 2 + padded_bits[i+3];
        
        // Convert nibble (0-15) to hex character
        sus hex_char = match nibble {
            0 => "0", 1 => "1", 2 => "2", 3 => "3",
            4 => "4", 5 => "5", 6 => "6", 7 => "7", 
            8 => "8", 9 => "9", 10 => "a", 11 => "b",
            12 => "c", 13 => "d", 14 => "e", 15 => "f"
        };
        
        hex_string = hex_string + hex_char;
        i = i + 4;
    }
    
    yolo hex_string;
}
```

**Example:**
- Input: `[0,1,1,1,0,0,1,1, 0,1,1,0,1,1,0,0, 0,1,1,0,0,0,0,1, 0,1,1,1,1,0,0,1]`
- Nibbles: `[7,3,6,12,6,1,7,9]` → `["7","3","6","c","6","1","7","9"]`
- Output: `"736c6179"`

### 3. Helper Functions

#### `byte_to_binary_string(byte) -> binary_string`
```cursed
slay byte_to_binary_string(byte) {
    sus binary = "";
    sus bit7 = (byte / 128) % 2;
    sus bit6 = (byte / 64) % 2;
    sus bit5 = (byte / 32) % 2;
    sus bit4 = (byte / 16) % 2;
    sus bit3 = (byte / 8) % 2;
    sus bit2 = (byte / 4) % 2;
    sus bit1 = (byte / 2) % 2;
    sus bit0 = byte % 2;
    
    // Concatenate bits as string
    lowkey (bit7 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit6 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit5 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit4 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit3 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit2 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit1 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    lowkey (bit0 == 1) { binary = binary + "1"; } flex { binary = binary + "0"; }
    
    yolo binary;
}
```

#### `xor_values(a, b) -> result`
```cursed
slay xor_values(a, b) {
    // XOR truth table: 0⊕0=0, 0⊕1=1, 1⊕0=1, 1⊕1=0
    lowkey ((a == 0 && b == 1) || (a == 1 && b == 0)) { yolo 1; }
    yolo 0;
}
```

#### `or_values(a, b) -> result`
```cursed
slay or_values(a, b) {
    // OR truth table: 0∨0=0, 0∨1=1, 1∨0=1, 1∨1=1
    lowkey (a == 1 || b == 1) { yolo 1; }
    yolo 0;
}
```

## Rule 30 Integration

For the cellular automaton golf challenge, these utilities support Rule 30 evolution:

**Rule 30 Formula:** `new_cell = left XOR (center OR right)`

```cursed
slay rule30_step(cells) {
    sus new_cells = [];
    sus i = 0;
    bestie (i < cells.length) {
        sus left = cells[(i - 1 + cells.length) % cells.length];
        sus center = cells[i];
        sus right = cells[(i + 1) % cells.length];
        
        sus or_result = or_values(center, right);
        sus xor_result = xor_values(left, or_result);
        
        new_cells.push(xor_result);
        i = i + 1;
    }
    yolo new_cells;
}
```

## Test Cases

| Input | Expected Binary | Expected Hex | Description |
|-------|----------------|--------------|-------------|
| `[115]` | `"01110011"` | `"73"` | ASCII 's' |
| `[108]` | `"01101100"` | `"6c"` | ASCII 'l' |
| `[97]` | `"01100001"` | `"61"` | ASCII 'a' |
| `[121]` | `"01111001"` | `"79"` | ASCII 'y' |
| `[255]` | `"11111111"` | `"ff"` | Maximum byte |
| `[0]` | `"00000000"` | `"00"` | Zero byte |
| `[1,0,1]` | `"101"` → `"1010"` | `"a"` | Padding test |

## Usage in Golf Challenge

1. **Read source code as bytes:**
   ```cursed
   sus source_bytes = read_program_bytes();
   ```

2. **Convert to binary initial state:**
   ```cursed
   sus initial_state = bytes_to_binary(source_bytes);
   ```

3. **Evolve using Rule 30:**
   ```cursed
   sus state = initial_state;
   sus step = 0;
   bestie (step < n) {
       state = rule30_step(state);
       step = step + 1;
   }
   ```

4. **Convert final state to hex:**
   ```cursed
   sus result = binary_to_hex(state);
   print(result);
   ```

## Implementation Notes

- **No bitwise operators:** Uses arithmetic operations only (`/`, `%`, `+`, `*`)
- **No complex data structures:** Works with simple arrays and loops
- **CURSED compatible:** Uses `sus`, `bestie`, `lowkey`, `flex`, `yolo` syntax
- **Efficient:** O(n) time complexity for all operations
- **Minimal dependencies:** No imports required

This utility module provides all the building blocks needed for the Rule 30 cellular automaton golf challenge while working within CURSED language constraints.
