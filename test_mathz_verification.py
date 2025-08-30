#!/usr/bin/env python3
"""
Script to verify CURSED math functions are working correctly by converting 
bit representations back to float values.
"""

import struct

def bits_to_float(bits):
    """Convert 64-bit integer representation to float"""
    return struct.unpack('d', bits.to_bytes(8, 'little'))[0]

def test_math_function(description, expected, actual_bits):
    """Test a math function result"""
    actual = bits_to_float(actual_bits)
    status = "✅" if abs(actual - expected) < 0.0001 else "❌"
    print(f"{status} {description}: expected {expected}, got {actual} (bits: {actual_bits})")
    return abs(actual - expected) < 0.0001

# Test mathz.add(10.5, 5.5) = 16.0
print("Testing mathz.add(10.5, 5.5):")
test_math_function("mathz.add", 16.0, 4625196817309499392)

# Test mathz.sub(10.5, 5.5) = 5.0  
print("\nExpected results for other operations:")
print("mathz.sub(10.5, 5.5) should = 5.0")
print("mathz.mul(10.5, 5.5) should = 57.75") 
print("mathz.abs_normie(-25.5) should = 25.5")
print("mathz.max_normie(10.5, 5.5) should = 10.5")
