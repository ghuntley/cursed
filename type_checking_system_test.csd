# CURSED Type Checking System Test
# Tests all major language features for proper type checking

# Basic type declarations with inference
sus name tea = "TypeChecker"
sus age drip = 25
sus active lit = based
sus height snack = 5.8

# Array types with proper bounds checking
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Carol"]

# Function with proper parameter and return type checking
slay calculateAge(birth_year drip, current_year drip) drip {
    ready (birth_year > current_year) {
        yikes "Invalid birth year"
    }
    damn current_year - birth_year
}

# Struct definition with type-safe fields
squad Person {
    name tea,
    age drip,
    email tea,
}

# Interface definition with method signatures
collab Drawable {
    slay draw(canvas tea) cap,
    slay getArea() snack,
}

# Generic function with type constraints
slay max[T](a T, b T) T {
    ready (a > b) {
        damn a
    }
    damn b
}

# Concurrency with channel type checking
sus ch chan<drip> = make_channel()

go {
    ch <- 42
    ch <- calculateAge(1990, 2025)
}

sus result drip = <-ch
vibez.spill("Received:", result)

# Pattern matching with exhaustive type checking
sus person Person = Person{name: "Alice", age: 30, email: "alice@example.com"}

sick (person.age) {
    when drip => vibez.spill("Age is a number"),
    otherwise => vibez.spill("Unexpected type"),
}

# Error handling with proper type propagation
slay divideNumbers(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}

sus division_result drip = divideNumbers(10, 2) fam {
    when "Division by zero" => {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when _ => {
        vibez.spill("Unknown error occurred")
        damn -1
    }
}

# Control structures with boolean type checking
bestie (active and age > 18) {
    vibez.spill("Adult user")
    
    ready (age > 65) {
        vibez.spill("Senior citizen")
    } otherwise ready (age > 21) {
        vibez.spill("Can drink alcohol")
    } otherwise {
        vibez.spill("Young adult")
    }
}

# Iterator with proper element type inference
for name : names {
    vibez.spill("Processing:", name)
}

# Complex expression type checking
sus complex_result drip = max(calculateAge(1980, 2025), division_result)
vibez.spill("Complex result:", complex_result)

# Type compatibility and coercion testing
sus small_number smol = 42
sus regular_number drip = small_number  # Should coerce smol to drip
sus big_number thicc = regular_number   # Should coerce drip to thicc

vibez.spill("All type checking tests completed successfully!")
