// Simple test of math functionality

// Test basic math operations
sus a normie = 10
sus b normie = 5

// Test addition
sus result_add normie = math.add(a, b)
vibez.spill("10 + 5 = ")
vibez.spill(result_add)

// Test subtraction
sus result_sub normie = math.subtract(a, b)
vibez.spill("10 - 5 = ")
vibez.spill(result_sub)

// Test multiplication
sus result_mul normie = math.multiply(a, b)
vibez.spill("10 * 5 = ")
vibez.spill(result_mul)

// Test division
sus result_div normie = math.divide(a, b)
vibez.spill("10 / 5 = ")
vibez.spill(result_div)

// Test power
sus result_pow normie = math.power(2, 3)
vibez.spill("2^3 = ")
vibez.spill(result_pow)

// Test absolute value
sus result_abs normie = math.abs(-42)
vibez.spill("abs(-42) = ")
vibez.spill(result_abs)

vibez.spill("Math test complete!")
