# Comprehensive Binary and Unary Operators Test for CURSED Code Generation
# This file tests all implemented binary and unary operators

# Test binary arithmetic operators
sus a drip = 10
sus b drip = 3
sus c tea = 5.5
sus d tea = 2.0

# Integer arithmetic
sus add_int drip = a + b     # 13
sus sub_int drip = a - b     # 7
sus mul_int drip = a * b     # 30
sus div_int drip = a / b     # 3 (integer division)
sus mod_int drip = a % b     # 1

# Float arithmetic
sus add_float tea = c + d    # 7.5
sus sub_float tea = c - d    # 3.5
sus mul_float tea = c * d    # 11.0
sus div_float tea = c / d    # 2.75
sus mod_float tea = c % d    # 1.5

# Mixed arithmetic (int + float)
sus mixed_add tea = a + c    # 15.5 (converted to float)
sus mixed_sub tea = a - c    # 4.5
sus mixed_mul tea = a * c    # 55.0
sus mixed_div tea = a / c    # ~1.818

# Test comparison operators
sus eq_int lit = a == 10     # true
sus ne_int lit = a != b      # true
sus lt_int lit = b < a       # true
sus le_int lit = b <= a      # true
sus gt_int lit = a > b       # true
sus ge_int lit = a >= b      # true

# Float comparisons
sus eq_float lit = c == 5.5  # true
sus ne_float lit = c != d    # true
sus lt_float lit = d < c     # true
sus le_float lit = d <= c    # true
sus gt_float lit = c > d     # true
sus ge_float lit = c >= d    # true

# Mixed comparisons
sus mixed_eq lit = a == c    # false (10.0 != 5.5)
sus mixed_lt lit = a < c     # false (10.0 < 5.5)
sus mixed_gt lit = a > c     # true (10.0 > 5.5)

# Test bitwise operators (integers only)
sus bit_a drip = 12          # 1100 in binary
sus bit_b drip = 7           # 0111 in binary

sus bit_and drip = bit_a & bit_b  # 4 (0100)
sus bit_or drip = bit_a | bit_b   # 15 (1111)
sus bit_xor drip = bit_a ^ bit_b  # 11 (1011)
sus bit_shl drip = bit_a << 2     # 48 (110000)
sus bit_shr drip = bit_a >> 2     # 3 (11)

# Test logical operators
sus true_val lit = based
sus false_val lit = cap

sus logical_and lit = true_val && false_val   # false (short-circuit)
sus logical_or lit = true_val || false_val    # true (short-circuit)
sus logical_and2 lit = true_val && true_val   # true
sus logical_or2 lit = false_val || false_val  # false

# Test with numeric values in logical context
sus num_and lit = a && b     # true (both non-zero)
sus num_or lit = a || 0      # true (a is non-zero)
sus zero_and lit = 0 && a    # false (short-circuit on 0)
sus zero_or lit = 0 || a     # true (a is non-zero)

# Test unary arithmetic operators
sus pos_int drip = +a        # 10 (unary plus)
sus neg_int drip = -a        # -10
sus pos_float tea = +c       # 5.5
sus neg_float tea = -c       # -5.5

# Test logical unary operators
sus not_true lit = !true_val      # false
sus not_false lit = !false_val    # true
sus not_num lit = !a              # false (a is 10, truthy)
sus not_zero lit = !0             # true (0 is falsy)

# Alternative 'not' syntax
sus not_alt lit = not true_val    # false

# Test bitwise unary operators
sus bit_not drip = ~bit_a         # bitwise NOT of 12

# Test increment/decrement operators
sus inc_test drip = 5
sus pre_inc drip = ++inc_test     # inc_test becomes 6, returns 6
sus dec_test drip = 5
sus pre_dec drip = --dec_test     # dec_test becomes 4, returns 4

# Test typeof operator (CURSED-specific)
sus type_int tea = typeof a       # "drip"
sus type_float tea = typeof c     # "tea"
sus type_bool tea = typeof true_val # "lit" or similar

# Test address-of and dereference operators
sus ptr_test drip = 42
sus ptr_addr = &ptr_test          # address of ptr_test
sus ptr_deref drip = *ptr_addr    # should be 42

# Test division by zero handling
sus safe_div drip = a / 0         # should return 0 or NaN, not crash
sus safe_div_float tea = c / 0.0  # should return NaN

# Output some results for verification
vibez.spill("Integer arithmetic:")
vibez.spill("10 + 3 =", add_int)
vibez.spill("10 - 3 =", sub_int)
vibez.spill("10 * 3 =", mul_int)
vibez.spill("10 / 3 =", div_int)
vibez.spill("10 % 3 =", mod_int)

vibez.spill("Float arithmetic:")
vibez.spill("5.5 + 2.0 =", add_float)
vibez.spill("5.5 - 2.0 =", sub_float)
vibez.spill("5.5 * 2.0 =", mul_float)
vibez.spill("5.5 / 2.0 =", div_float)

vibez.spill("Comparisons:")
vibez.spill("10 == 10:", eq_int)
vibez.spill("10 != 3:", ne_int)
vibez.spill("3 < 10:", lt_int)
vibez.spill("10 > 3:", gt_int)

vibez.spill("Bitwise operations:")
vibez.spill("12 & 7 =", bit_and)
vibez.spill("12 | 7 =", bit_or)
vibez.spill("12 ^ 7 =", bit_xor)
vibez.spill("12 << 2 =", bit_shl)
vibez.spill("12 >> 2 =", bit_shr)

vibez.spill("Logical operations:")
vibez.spill("true && false:", logical_and)
vibez.spill("true || false:", logical_or)

vibez.spill("Unary operations:")
vibez.spill("-10 =", neg_int)
vibez.spill("!true =", not_true)
vibez.spill("!false =", not_false)

vibez.spill("Increment/Decrement:")
vibez.spill("++5 =", pre_inc)
vibez.spill("--5 =", pre_dec)

vibez.spill("Address/Dereference:")
vibez.spill("*(&42) =", ptr_deref)

vibez.spill("All binary and unary operator tests completed!")
