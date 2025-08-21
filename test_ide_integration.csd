# CURSED IDE Integration Test
# This file demonstrates all IDE features working together

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"
yeet "concurrenz"

# Test 1: Variable declarations with type inference
sus name tea = "CURSED Developer"
sus age drip = 25
sus active lit = based

# Test 2: Function definition with parameters and return type
slay calculate_fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}

# Test 3: Complex data structures
squad Person {
    sus name tea
    sus age drip
    sus email tea
}

# Test 4: Interface definition
collab Processor {
    slay process(data tea) tea
    slay validate(input tea) lit
}

# Test 5: Pattern matching with complex conditions
slay classify_age(age drip) tea {
    sick (age) {
        when 0...12 -> damn "child"
        when 13...19 -> damn "teenager"  
        when 20...64 -> damn "adult"
        when _ -> damn "senior"
    }
}

# Test 6: Error handling with structured errors
slay divide_safe(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

# Test 7: Concurrency with goroutines and channels
slay concurrent_processing(data []drip) {
    sus results chan<drip> = make_channel()
    
    go {
        bestie (sus item drip = range data) {
            sus processed drip = item * 2
            results <- processed
        }
    }
    
    bestie (sus i drip = 0; i < len(data); i += 1) {
        sus result drip = <-results
        vibez.spill("Processed:", result)
    }
}

# Test 8: Array operations and functional programming
slay main() {
    vibez.spill("🚀 CURSED IDE Integration Test")
    
    # Create test person
    sus person Person = Person{
        name: "Alice",
        age: 30,
        email: "alice@example.com"
    }
    
    # Test mathematical operations  
    sus fib_10 drip = calculate_fibonacci(10)
    vibez.spill("Fibonacci(10):", fib_10)
    
    # Test pattern matching
    sus age_category tea = classify_age(person.age)
    vibez.spill(person.name, "is a", age_category)
    
    # Test error handling
    sus safe_result drip = divide_safe(10, 2) fam {
        when "division by zero" -> {
            vibez.spill("Cannot divide by zero!")
            damn 0
        }
    }
    vibez.spill("Division result:", safe_result)
    
    # Test array processing
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus doubled []drip = arrayz.map(numbers, slay(x drip) drip {
        damn x * 2
    })
    
    vibez.spill("Original:", numbers)
    vibez.spill("Doubled:", doubled)
    
    # Test concurrency
    concurrent_processing(numbers)
    
    # Test string operations
    sus greeting tea = stringz.format("Hello, {0}! You are {1} years old.", 
                                     [person.name, stringz.from_int(person.age)])
    vibez.spill(greeting)
    
    vibez.spill("✅ IDE Integration Test Complete!")
}

# This file tests:
# ✓ Syntax highlighting for all CURSED constructs  
# ✓ Code completion for keywords, functions, modules
# ✓ Hover information for symbols and functions
# ✓ Goto definition for user-defined functions
# ✓ Find references across the file
# ✓ Error detection and diagnostics
# ✓ Automatic formatting and indentation
# ✓ Bracket matching and comment toggling
