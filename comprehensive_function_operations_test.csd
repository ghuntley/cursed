yeet "stringz"
yeet "mathz"

vibez.spill("=== Testing Complex Function Operations ===")

# 1. Basic stdlib functions
vibez.spill("1. Basic stdlib functions:")
sus text tea = "hello"
sus len drip = len_str(text)
vibez.spill("Length of 'hello':", len)

sus num drip = -42
sus abs_val drip = abs_normie(num)
vibez.spill("Absolute of -42:", abs_val)

# 2. User-defined functions calling stdlib
vibez.spill("2. User-defined functions calling stdlib:")
slay string_processor(s tea) drip {
    sus length drip = len_str(s)
    damn length * 2
}

sus processed drip = string_processor("test")
vibez.spill("String processor result:", processed)

# 3. Mathematical function chains  
vibez.spill("3. Mathematical function chains:")
slay math_chain(x drip) drip {
    sus step1 drip = abs_normie(x)
    sus step2 drip = step1 + 10
    damn abs_normie(step2)
}

sus chained drip = math_chain(-5)
vibez.spill("Math chain result:", chained)

# 4. Direct function call chains
vibez.spill("4. Direct function call chains:")
sus direct_chain drip = abs_normie(abs_normie(-3) + 7)
vibez.spill("Direct chain result:", direct_chain)

# 5. Complex nested expressions
vibez.spill("5. Complex nested expressions:")
sus complex1 drip = abs_normie(-10)
sus complex2 drip = abs_normie(complex1 + 5)
vibez.spill("Complex nested result:", complex2)

# 6. Function calls in arithmetic
vibez.spill("6. Function calls in arithmetic:")
sus arithmetic drip = abs_normie(-8) + abs_normie(-2)
vibez.spill("Arithmetic result:", arithmetic)

vibez.spill("=== All Function Operation Tests Complete ===")
