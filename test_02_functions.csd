# Test 2: Function definitions, calls, and return values
slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(x drip, y drip) drip {
    vibez.spill("Multiplying", x, "and", y)
    damn x * y
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay greet(name tea) tea {
    damn "Hello, " + name + "!"
}

vibez.spill("Functions test:")
sus sum drip = add(10, 5)
vibez.spill("add(10, 5) =", sum)

sus product drip = multiply(4, 7)
vibez.spill("multiply(4, 7) =", product)

sus fact5 drip = factorial(5)
vibez.spill("factorial(5) =", fact5)

sus greeting tea = greet("CURSED")
vibez.spill("greet('CURSED') =", greeting)
