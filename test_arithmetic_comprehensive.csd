yeet "testz"

test_start("Comprehensive Arithmetic Tests")

# Basic integer arithmetic
sus a drip = 10
sus b drip = 5

# Test basic operations
sus add_result drip = a + b
assert_eq_int(add_result, 15)
vibez.spill("✅ Addition:", add_result)

sus sub_result drip = a - b  
assert_eq_int(sub_result, 5)
vibez.spill("✅ Subtraction:", sub_result)

sus mul_result drip = a * b
assert_eq_int(mul_result, 50)
vibez.spill("✅ Multiplication:", mul_result)

sus div_result drip = a / b
assert_eq_int(div_result, 2)
vibez.spill("✅ Division:", div_result)

sus mod_result drip = a % b
assert_eq_int(mod_result, 0)
vibez.spill("✅ Modulus:", mod_result)

# Test complex expressions with parentheses
sus complex1 drip = (a + b) * 2
assert_eq_int(complex1, 30)
vibez.spill("✅ Complex 1 (a+b)*2:", complex1)

sus complex2 drip = a * (b + 3)
assert_eq_int(complex2, 80)
vibez.spill("✅ Complex 2 a*(b+3):", complex2)

sus complex3 drip = (a + b) / (b - 3)
assert_eq_int(complex3, 7)
vibez.spill("✅ Complex 3 (a+b)/(b-3):", complex3)

# Test operator precedence
sus precedence1 drip = 2 + 3 * 4
assert_eq_int(precedence1, 14)
vibez.spill("✅ Precedence 1 (2+3*4):", precedence1)

sus precedence2 drip = 10 - 6 / 2
assert_eq_int(precedence2, 7)
vibez.spill("✅ Precedence 2 (10-6/2):", precedence2)

sus precedence3 drip = 20 / 4 + 3 * 2
assert_eq_int(precedence3, 11)
vibez.spill("✅ Precedence 3 (20/4+3*2):", precedence3)

# Test floating point arithmetic
sus x normie = 10.5
sus y normie = 2.5

sus float_add normie = x + y
vibez.spill("✅ Float addition:", float_add)

sus float_sub normie = x - y
vibez.spill("✅ Float subtraction:", float_sub)

sus float_mul normie = x * y
vibez.spill("✅ Float multiplication:", float_mul)

sus float_div normie = x / y
vibez.spill("✅ Float division:", float_div)

# Test mixed integer and float operations
sus mixed1 normie = 10 + 2.5
vibez.spill("✅ Mixed int+float:", mixed1)

sus mixed2 normie = 10.5 * 3
vibez.spill("✅ Mixed float*int:", mixed2)

# Test comparison operations
sus comp1 lit = a > b
assert_true(comp1)
vibez.spill("✅ Greater than:", comp1)

sus comp2 lit = a < b
assert_false(comp2)
vibez.spill("✅ Less than:", comp2)

sus comp3 lit = a == 10
assert_true(comp3)
vibez.spill("✅ Equality:", comp3)

sus comp4 lit = a != b
assert_true(comp4)
vibez.spill("✅ Not equal:", comp4)

# Test unary operations
sus neg_test drip = -5
assert_eq_int(neg_test, -5)
vibez.spill("✅ Unary minus:", neg_test)

sus neg_var drip = -a
assert_eq_int(neg_var, -10)
vibez.spill("✅ Negate variable:", neg_var)

print_test_summary()
