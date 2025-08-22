# Test 2: Functions - definitions, calls, parameters, return values

# Simple function with no parameters
slay greet() tea {
    damn "Hello from function!"
}

# Function with parameters
slay add(a drip, b drip) drip {
    damn a + b
}

# Function with string parameters
slay make_greeting(name tea) tea {
    damn "Hello " + name + "!"
}

# Function with boolean logic
slay is_adult(age drip) lit {
    damn age >= 18
}

# Function with multiple parameters and complex logic
slay calculate_discount(price drip, age drip, is_member lit) drip {
    sus discount drip = 0
    ready (age >= 65) {
        discount = 0.20
    } otherwise ready (is_member) {
        discount = 0.10
    } otherwise {
        discount = 0.05
    }
    damn price * (1 - discount)
}

# Test function calls
sus greeting tea = greet()
vibez.spill(greeting)

sus sum drip = add(15, 25)
vibez.spill("15 + 25 =", sum)

sus personalized tea = make_greeting("Alice")
vibez.spill(personalized)

sus adult_status lit = is_adult(21)
vibez.spill("Is 21 adult:", adult_status)

sus final_price drip = calculate_discount(100, 70, based)
vibez.spill("Final price:", final_price)
