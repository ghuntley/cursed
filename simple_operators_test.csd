# Simple operators test
sus a drip = 10
sus b drip = 5

# Test basic arithmetic
sus add_result drip = a + b
sus sub_result drip = a - b
sus mul_result drip = a * b
sus div_result drip = a / b

# Test comparisons
sus eq_result lit = a == 10
sus ne_result lit = a != b
sus lt_result lit = b < a

# Test unary
sus neg_result drip = -a
sus not_result lit = !eq_result

# Output results
vibez.spill("Addition:", add_result)
vibez.spill("Subtraction:", sub_result)
vibez.spill("Multiplication:", mul_result)
vibez.spill("Division:", div_result)
vibez.spill("Equal:", eq_result)
vibez.spill("Not equal:", ne_result)
vibez.spill("Less than:", lt_result)
vibez.spill("Negative:", neg_result)
vibez.spill("Logical NOT:", not_result)
