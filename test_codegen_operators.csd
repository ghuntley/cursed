// Code Generation - Binary and Unary Operators Test

// Test binary operators
sus a drip = 15
sus b drip = 7

sus addition drip = a + b
sus subtraction drip = a - b  
sus multiplication drip = a * b
sus division drip = a / b
sus modulo drip = a % b

spill("Binary Operators:")
spill("15 + 7 =", addition)
spill("15 - 7 =", subtraction)
spill("15 * 7 =", multiplication)
spill("15 / 7 =", division)
spill("15 % 7 =", modulo)

// Test unary operators
sus positive drip = +a
sus negative drip = -a

spill("Unary Operators:")
spill("+15 =", positive)
spill("-15 =", negative)

// Test boolean operators
sus x lit = based
sus y lit = cap

sus and_result lit = x && y
sus or_result lit = x || y
sus not_result lit = !x

spill("Boolean Operators:")
spill("true && false =", and_result)
spill("true || false =", or_result)
spill("!true =", not_result)

// Test comparison operators
sus eq_result lit = a == b
sus ne_result lit = a != b
sus lt_result lit = a < b
sus le_result lit = a <= b
sus gt_result lit = a > b
sus ge_result lit = a >= b

spill("Comparison Operators:")
spill("15 == 7 =", eq_result)
spill("15 != 7 =", ne_result)
spill("15 < 7 =", lt_result)
spill("15 <= 7 =", le_result)
spill("15 > 7 =", gt_result)
spill("15 >= 7 =", ge_result)
