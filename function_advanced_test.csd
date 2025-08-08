slay multiply(a drip, b drip) drip { damn a * b }
slay factorial(n drip) drip { 
    ready (n <= 1) { damn 1 }
    damn n * factorial(n - 1)
}

vibez.spill("2 * 3 =", multiply(2, 3))
vibez.spill("5! =", factorial(5))
sus result drip = multiply(4, 5)
vibez.spill("Stored result:", result)
