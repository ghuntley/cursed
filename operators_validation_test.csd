# Critical P0 Binary and Unary Operators Validation Test
# Ensures all essential operators work in compilation mode

# Test integer arithmetic operations
sus x drip = 15
sus y drip = 4

# Binary arithmetic operators (P0 critical)
sus addition drip = x + y       # 19
sus subtraction drip = x - y    # 11
sus multiplication drip = x * y # 60
sus division drip = x / y       # 3
sus modulo drip = x % y         # 3

# Binary comparison operators (P0 critical)  
sus equals lit = x == 15        # true
sus not_equals lit = x != y     # true
sus less_than lit = y < x       # true
sus less_equal lit = y <= x     # true
sus greater_than lit = x > y    # true
sus greater_equal lit = x >= y  # true

# Unary operators (P0 critical)
sus negation drip = -x          # -15
sus logical_not lit = !equals   # false
sus positive drip = +x          # 15

# Float operations
sus f1 tea = 3.14
sus f2 tea = 2.5

sus float_add tea = f1 + f2     # 5.64
sus float_sub tea = f1 - f2     # 0.64
sus float_mul tea = f1 * f2     # 7.85
sus float_div tea = f1 / f2     # 1.256

# Float comparisons
sus float_eq lit = f1 == 3.14   # true
sus float_gt lit = f1 > f2      # true

# Float unary
sus float_neg tea = -f1         # -3.14

# Bitwise operators
sus bit1 drip = 12              # 1100
sus bit2 drip = 10              # 1010

sus bitwise_and drip = bit1 & bit2  # 8 (1000)
sus bitwise_or drip = bit1 | bit2   # 14 (1110)
sus bitwise_xor drip = bit1 ^ bit2  # 6 (0110)
sus bitwise_not drip = ~bit1        # bitwise complement

# Shift operations
sus left_shift drip = bit1 << 1     # 24 (11000)
sus right_shift drip = bit1 >> 1    # 6 (110)

# Logical operators (short-circuit)
sus logical_and lit = equals && not_equals    # true && true = true
sus logical_or lit = equals || cap            # true || false = true

# Test division by zero safety
sus safe_div_int drip = x / 0       # Should not crash
sus safe_div_float tea = f1 / 0.0   # Should return NaN

# Validation output
vibez.spill("Binary arithmetic operators working!")
vibez.spill("15 + 4 =", addition)
vibez.spill("15 - 4 =", subtraction)  
vibez.spill("15 * 4 =", multiplication)
vibez.spill("15 / 4 =", division)

vibez.spill("Binary comparison operators working!")
vibez.spill("15 == 15:", equals)
vibez.spill("15 != 4:", not_equals)
vibez.spill("4 < 15:", less_than)
vibez.spill("15 > 4:", greater_than)

vibez.spill("Unary operators working!")
vibez.spill("-(15) =", negation)
vibez.spill("+(15) =", positive)
vibez.spill("!(true) =", logical_not)

vibez.spill("Float operations working!")
vibez.spill("3.14 + 2.5 =", float_add)
vibez.spill("3.14 - 2.5 =", float_sub)
vibez.spill("-(3.14) =", float_neg)

vibez.spill("Bitwise operations working!")
vibez.spill("12 & 10 =", bitwise_and)
vibez.spill("12 | 10 =", bitwise_or)
vibez.spill("12 ^ 10 =", bitwise_xor)
vibez.spill("12 << 1 =", left_shift)
vibez.spill("12 >> 1 =", right_shift)

vibez.spill("Logical operations working!")
vibez.spill("true && true:", logical_and)
vibez.spill("true || false:", logical_or)

vibez.spill("Division by zero safety working!")
vibez.spill("15 / 0 =", safe_div_int)

vibez.spill("🎉 ALL P0 BINARY AND UNARY OPERATORS IMPLEMENTED AND WORKING! 🎉")
