slay greet(name tea) {
    vibez.spill("Hello,", name)
}

slay add(a drip, b drip) {
    sus result drip = a + b
    vibez.spill("Result:", result)
}

slay factorial(n drip) {
    lowkey (n <= 1) {
        vibez.spill("Base case:", n)
    } bestie {
        vibez.spill("Recursive case:", n)
    }
}

greet("Alice")
add(5, 3)
factorial(4)
