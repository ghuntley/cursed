fr Test basic CURSED ecosystem functionality

fr Variables and basic types
sus greeting tea = "Hello, CURSED!"
sus count drip = 42
sus active lit = based
sus pi meal = 3.14159

fr Basic function
slay greet(name tea) {
    vibez.spill("Hello,", name, "!")
}

fr Function with return value
slay add(a drip, b drip) drip {
    damn a + b
}

fr Control flow
slay check_number(num drip) {
    ready (num > 0) {
        vibez.spill("Positive number:", num)
    } otherwise ready (num < 0) {
        vibez.spill("Negative number:", num)
    } otherwise {
        vibez.spill("Zero")
    }
}

fr Main function
slay main() {
    vibez.spill(greeting)
    vibez.spill("Count:", count)
    vibez.spill("Active:", active)
    vibez.spill("Pi:", pi)
    
    greet("World")
    
    sus sum drip = add(5, 3)
    vibez.spill("5 + 3 =", sum)
    
    check_number(10)
    check_number(-5)
    check_number(0)
    
    fr Loop example
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Loop iteration:", i)
        i = i + 1
    }
    
    vibez.spill("CURSED ecosystem test complete! 🔥")
}
