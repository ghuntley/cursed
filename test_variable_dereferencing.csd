sus x drip = 42
sus y drip = x + 10
sus z tea = "hello"
sus w tea = z + " world"

vibez.spill("x =", x)
vibez.spill("y =", y) 
vibez.spill("z =", z)
vibez.spill("w =", w)

# Test complex expressions with variables
sus a drip = 5
sus b drip = 3
sus result drip = a * b + x
vibez.spill("complex result =", result)

# Test variable in conditional
ready (x > 40) {
    vibez.spill("x is greater than 40")
} otherwise {
    vibez.spill("x is not greater than 40")
}
