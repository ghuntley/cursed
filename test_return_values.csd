fr fr Test function return values with damn keyword

slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(x drip, y drip) drip {
    damn x * y
}

slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

slay isPositive(num drip) lit {
    damn num > 0
}

fr fr Test basic arithmetic function
sus result drip = add(5, 3)
vibez.spill("5 + 3 =", result)

fr fr Test multiplication
sus product drip = multiply(4, 7)
vibez.spill("4 * 7 =", product)

fr fr Test string function
sus greeting tea = greet("CURSED")
vibez.spill(greeting)

fr fr Test boolean function
sus positive lit = isPositive(10)
vibez.spill("Is 10 positive?", positive)

sus negative lit = isPositive(-5)
vibez.spill("Is -5 positive?", negative)

fr fr Test function calls in expressions
sus combined drip = add(2, 3) + multiply(4, 5)
vibez.spill("add(2,3) + multiply(4,5) =", combined)
