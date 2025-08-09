// Type errors that should be caught
sus number drip = 42
sus text tea = "hello"

// Type mismatch in assignment
sus wrong drip = text

// Invalid arithmetic on strings
sus bad_math drip = text + number

// Function parameter type mismatch
slay expects_number(x drip) drip {
    damn x * 2
}

sus result drip = expects_number(text)

// Array type mismatch
sus numbers []drip = ["not", "numbers"]

// Invalid array indexing
sus invalid drip = text[0]
