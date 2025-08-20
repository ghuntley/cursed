# Test advanced struct scenarios that require dereferencing
vibez.spill("=== Advanced Struct Dereferencing Tests ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create a struct
sus p Point = Point{x: 100, y: 200}
vibez.spill("Original point:", p.x, p.y)

# Test parenthesized member access (should work as struct field access)
sus result = (p).x + (p).y
vibez.spill("Sum using parentheses:", result)

# Test struct field access in complex expressions
sus double_x = p.x * 2
sus double_y = p.y * 2
vibez.spill("Doubled coordinates:", double_x, double_y)

# Test method-like access patterns 
squad Calculator {
    spill value drip
}

slay multiply(calc Calculator, factor drip) drip {
    damn calc.value * factor
}

sus calc Calculator = Calculator{value: 50}
sus multiplied = multiply(calc, 3)
vibez.spill("Calculator result:", multiplied)

# Test unary operators on struct fields
sus neg_x = -p.x
sus pos_y = +p.y
vibez.spill("Unary operators on fields:", neg_x, pos_y)

# Test logical operations on computed field values
sus is_large_x lit = !(p.x < 50)
vibez.spill("Is x large?", is_large_x)

vibez.spill("=== All advanced tests passed! ===")
