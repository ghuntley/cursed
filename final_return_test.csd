fr fr Final test of CURSED function return values with damn keyword

fr fr Integer arithmetic functions
slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(x drip, y drip) drip {
    damn x * y
}

fr fr Boolean comparison functions  
slay isEven(num drip) lit {
    damn num % 2 == 0
}

slay isGreater(a drip, b drip) lit {
    damn a > b
}

fr fr Simple string function
slay greet(name tea) tea {
    damn "Hello"
}

fr fr Test all function types
sus sum drip = add(10, 5)
vibez.spill("10 + 5 =", sum)

sus product drip = multiply(6, 7) 
vibez.spill("6 * 7 =", product)

sus even lit = isEven(8)
vibez.spill("Is 8 even?", even)

sus greater lit = isGreater(10, 5)
vibez.spill("Is 10 > 5?", greater)

sus greeting tea = greet("World")
vibez.spill("Greeting:", greeting)
