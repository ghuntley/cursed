# CURSED Rule 30 Test Framework

This directory contains a comprehensive test framework for the Rule 30 cellular automaton implementation used in the CURSED code golf challenge.

## Overview

The Rule 30 implementation reads its own source code, converts it to a binary tape, evolves it using the Rule 30 cellular automaton for `n` steps, and outputs the result as hexadecimal.

## Test Files

### Core Test Files

1. **`test_rule30.csd`** - Tests the Rule 30 algorithm itself
   - Rule 30 truth table validation
   - Circular tape wrapping
   - Known pattern evolution
   - Source code evolution

2. **`test_conversion.csd`** - Tests binary/hex conversions
   - Byte to binary conversion
   - Binary to hex conversion
   - Round-trip conversions
   - Edge cases and padding

3. **`test_integration.csd`** - Integration tests
   - Complete pipeline testing
   - Performance testing
   - Boundary conditions
   - n-value progression (1-12)

### Validation Scripts

4. **`validate.sh`** - Comprehensive validation script
   - Runs all tests
   - Compares outputs with expected results
   - Performance benchmarking
   - Detailed reporting

5. **`run_tests.sh`** - Quick test runner
   - Individual test execution
   - Selective test running
   - Simple pass/fail reporting

### Reference Data

6. **`expected_results.txt`** - Expected outputs for validation
   - Rule 30 truth table results
   - Known evolution sequences
   - Conversion test results
   - Performance benchmarks

## Rule 30 Algorithm

Rule 30 is a cellular automaton rule that determines the next state of each cell based on its current state and its two neighbors:

```
new_cell = left XOR (center OR right)
```

### Truth Table
```
Left | Center | Right | Result
-----|--------|-------|-------
  1  |   1    |   1   |   0
  1  |   1    |   0   |   0
  1  |   0    |   1   |   0
  1  |   0    |   0   |   1
  0  |   1    |   1   |   1
  0  |   1    |   0   |   1
  0  |   0    |   1   |   1
  0  |   0    |   0   |   0
```

## Test Categories

### 1. Algorithm Correctness
- Truth table validation
- Circular boundary handling
- Single-step evolution
- Multi-step evolution

### 2. Data Conversion
- ASCII to bytes
- Bytes to binary
- Binary to hexadecimal
- Padding and alignment

### 3. Integration Testing
- Complete pipeline flow
- Input/output validation
- Performance characteristics
- Memory usage

### 4. Edge Cases
- Empty inputs
- Single character inputs
- Maximum n values
- Large inputs

## Expected Results

For the input "slay" (bytes: [0x73, 0x6C, 0x61, 0x79]):

| n | Expected Hex Output |
|---|-------------------|
| 1 | 8cd39e86         |
| 2 | d32c6153         |
| 3 | 2cd3b6ac         |
| 4 | 5b698d59         |
| 5 | b6d31ab2         |

## Running Tests

### Quick Tests
```bash
# Run all tests
./run_tests.sh

# Run specific test
./run_tests.sh rule30
./run_tests.sh conversion
./run_tests.sh integration
```

### Comprehensive Validation
```bash
# Full validation with expected results comparison
./validate.sh
```

### Individual Test Execution
```bash
# Run CURSED compiler on test files
cursed test_rule30.csd
cursed test_conversion.csd
cursed test_integration.csd
```

## Test Structure

Each test file follows this structure:

1. **Header** - Test description and purpose
2. **Main Function** - Test orchestration
3. **Test Functions** - Individual test cases
4. **Implementation Functions** - Code being tested
5. **Helper Functions** - Test utilities

## Validation Criteria

### Correctness
- All Rule 30 truth table entries must be correct
- Circular wrapping must work properly
- Binary/hex conversions must be accurate
- Expected outputs must match for known inputs

### Performance
- Single evolution step: < 100ms
- n=12 evolution: < 1000ms
- Binary conversion: < 50ms
- Hex conversion: < 50ms

### Memory Usage
- Binary tape: O(file_size × 8) bits
- Temporary storage: O(tape_length)
- String operations: O(output_length)

## Implementation Notes

### Circular Wrapping
The tape wraps around so that:
- Left neighbor of position 0 is position (length-1)
- Right neighbor of position (length-1) is position 0

### Binary Representation
Bytes are converted to binary in big-endian format:
- Byte 0x73 becomes [0,1,1,1,0,0,1,1]
- Most significant bit first

### Hex Output
Binary is converted to hex in groups of 4 bits:
- [0,1,1,1] becomes '7'
- [0,0,1,1] becomes '3'
- Padding with zeros if necessary

## Debugging

If tests fail:

1. Check Rule 30 logic implementation
2. Verify circular wrapping calculations
3. Validate binary conversion bit order
4. Confirm hex conversion grouping
5. Test with known simple cases first

## Contributing

When adding new tests:

1. Follow existing naming conventions
2. Include expected results in comments
3. Test edge cases thoroughly
4. Update expected_results.txt
5. Add validation to validate.sh

## References

- [Rule 30 - Wolfram MathWorld](https://mathworld.wolfram.com/Rule30.html)
- [Cellular Automaton - Wikipedia](https://en.wikipedia.org/wiki/Cellular_automaton)
- [Elementary Cellular Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
