fr fr Test arithmetic operations
sus x drip = 5
sus y drip = 3
sus sum drip = x + y
sus diff drip = x - y
sus product drip = x * y
sus quotient drip = x / y

vibez.spill("x =", x)
vibez.spill("y =", y)
vibez.spill("sum =", sum)
vibez.spill("diff =", diff)
vibez.spill("product =", product)
vibez.spill("quotient =", quotient)

fr fr Test more complex expressions
sus complex drip = (x + y) * (x - y)
vibez.spill("complex =", complex)

fr fr Test variable reassignment
x = x + 1
vibez.spill("x after increment =", x)
