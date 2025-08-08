slay add(x drip, y drip) drip { damn x + y }
slay multiply(a drip, b drip) drip { damn a * b }
slay is_positive(n drip) drip {
    ready (n > 0) { damn 1 }
    damn 0
}

vibez.spill("Result:", add(5, 3))
vibez.spill("Multiply:", multiply(4, 7))
vibez.spill("Positive check:", is_positive(-5))
vibez.spill("Positive check:", is_positive(10))
sus x drip = add(10, 15)
vibez.spill("Stored result:", x)
