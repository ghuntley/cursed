# CURSED Code Golf: Rule 30 Cellular Automaton

## Challenge Description

**Input:** n ∈ [1‒12]  
**Task:** 
1. Treat program's own bytes as circular binary tape
2. Evolve 1-D Rule 30 for n steps  
3. Print final tape in hex (no 0x prefix)
4. Same code must run for any n
5. **Fewest bytes wins**

## Rule 30 Cellular Automaton

Rule 30 is a elementary cellular automaton rule where each cell's next state depends on itself and its two neighbors:

```
Current pattern → Next center cell
111 → 0
110 → 0  
101 → 0
100 → 1
011 → 1
010 → 1
001 → 1
000 → 0
```

The rule can be expressed as: **new_cell = left XOR (center OR right)**

## Algorithm Steps

1. **Self-Read:** Read the program's own source file bytes
2. **Binary Convert:** Convert each byte to 8-bit binary representation  
3. **Circular Tape:** Treat as circular array (wrap around edges)
4. **Evolution:** Apply Rule 30 for n iterations
5. **Hex Output:** Convert final binary back to hex and print

## Example

If program bytes are `[0x41, 0x42]` (ASCII "AB"):
- Binary: `01000001 01000010`
- After 1 step of Rule 30: `(new binary)`
- Output: `(hex without 0x)`

## Files

- `rule30.csd` - Reference implementation
- `golf.csd` - Code golf version (shortest)
- `test.sh` - Test script for all n values
- `Makefile` - Build system

## CURSED Language Optimizations

For minimum byte count:
- Use shortest variable names (`a`,`b`,`c`)
- Minimize whitespace
- Use shortest function/keyword forms
- Combine operations where possible
- Eliminate unnecessary parentheses

## Testing

The solution must work correctly for all n ∈ [1,2,3,4,5,6,7,8,9,10,11,12].
