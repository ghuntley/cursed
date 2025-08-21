# CURSED v1.0.0-rc2 Community Test Scenarios

# Scenario 1: Basic Language Features
yeet "vibez"

sus name tea = "Community Tester"
sus age drip = 25
sus active lit = based

slay greet(person tea) tea {
    damn "Hello, " + person + "! Welcome to Bug Bash!"
}

vibez.spill(greet(name))

ready (age >= 18) {
    vibez.spill("Adult tester confirmed")
} otherwise {
    vibez.spill("Youth tester - welcome!")
}

# Test arrays and basic operations
sus numbers []drip = [1, 2, 3, 4, 5]
sus total drip = 0
bestie (sus i drip = 0; i < len(numbers); i = i + 1) {
    total = total + numbers[i]
}
vibez.spill("Array sum:", total)

# Scenario 2: Standard Library Testing
yeet "mathz"
yeet "stringz"
yeet "arrayz"

# Math operations
sus pi drip = 3.14159
sus radius drip = 5
sus area drip = pi * radius * radius
vibez.spill("Circle area:", area)

# String operations  
sus message tea = "CURSED is production ready"
sus upper_msg tea = to_upper(message)
sus msg_len drip = len(message)
vibez.spill("Message:", message)
vibez.spill("Uppercase:", upper_msg)
vibez.spill("Length:", msg_len)

# Array utilities
sus data []drip = [9, 2, 7, 1, 8]
sus sorted_data []drip = sort(data)
sus max_val drip = max(data)
sus min_val drip = min(data)

vibez.spill("Original:", data)
vibez.spill("Sorted:", sorted_data)
vibez.spill("Max:", max_val, "Min:", min_val)

# Scenario 3: Function Testing
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    } otherwise {
        damn n * factorial(n - 1)
    }
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

vibez.spill("Factorial(5):", factorial(5))
vibez.spill("Fibonacci(8):", fibonacci(8))

# Higher-order function test
sus values []drip = [1, 2, 3, 4, 5]
sus doubled []drip = map(values, slay(x drip) drip { damn x * 2 })
sus even_vals []drip = filter(values, slay(x drip) lit { damn x % 2 == 0 })

vibez.spill("Original values:", values)
vibez.spill("Doubled:", doubled)
vibez.spill("Even values:", even_vals)

# Scenario 4: Error Handling
slay safe_divide(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "division by zero error"
    }
    damn a / b
}

slay parse_number(text tea) yikes<drip> {
    ready (text == "") {
        yikes "empty string"
    }
    # Simplified parsing - real implementation would parse
    ready (text == "42") {
        damn 42
    } otherwise {
        yikes "invalid number format"
    }
}

# Test error handling patterns
sus result1 drip = safe_divide(10, 2) fam {
    when "division by zero error" -> {
        vibez.spill("Caught division by zero!")
        damn 0
    }
    when _ -> damn -1
}
vibez.spill("Safe division result:", result1)

sus result2 drip = safe_divide(10, 0) fam {
    when "division by zero error" -> {
        vibez.spill("Division by zero handled!")
        damn -999
    }
    when _ -> damn -1
}
vibez.spill("Zero division result:", result2)

sus num1 drip = parse_number("42") fam {
    when "empty string" -> damn 0
    when "invalid number format" -> damn -1
    when _ -> damn -2
}
vibez.spill("Parsed number:", num1)

sus num2 drip = parse_number("invalid") fam {
    when "empty string" -> damn 0
    when "invalid number format" -> {
        vibez.spill("Invalid number format caught!")
        damn -1
    }
    when _ -> damn -2
}
vibez.spill("Invalid parse result:", num2)

# Scenario 5: Concurrency Testing (Basic)
yeet "concurrenz"

# Simple channel test
sus ch chan<drip> = make_channel()

go {
    vibez.spill("Goroutine: Sending value 42")
    ch <- 42
    vibez.spill("Goroutine: Value sent")
}

vibez.spill("Main: Waiting for channel value")
sus received drip = <-ch
vibez.spill("Main: Received value:", received)

# Multiple goroutines
sus results chan<drip> = make_channel_buffered(3)

go {
    results <- 10
}

go {
    results <- 20
}

go {
    results <- 30
}

vibez.spill("Collecting results from multiple goroutines:")
bestie (sus i drip = 0; i < 3; i = i + 1) {
    sus val drip = <-results
    vibez.spill("Result", i + 1, ":", val)
}

# Scenario 6: Data Structures Testing
squad Person {
    name tea,
    age drip,
    email tea
}

slay create_person(n tea, a drip, e tea) Person {
    damn Person {
        name: n,
        age: a,
        email: e
    }
}

sus person Person = create_person("Alice", 30, "alice@example.com")
vibez.spill("Person:", person.name, "Age:", person.age, "Email:", person.email)

# Array of structs
sus people []Person = [
    create_person("Bob", 25, "bob@test.com"),
    create_person("Carol", 35, "carol@demo.org"),
    create_person("David", 28, "david@sample.net")
]

vibez.spill("Team members:")
bestie (sus i drip = 0; i < len(people); i = i + 1) {
    vibez.spill("-", people[i].name, "(", people[i].age, "years old )")
}

# Scenario 7: Pattern Matching (if supported)
enum Status {
    Pending,
    InProgress,
    Complete,
    Failed
}

slay status_message(s Status) tea {
    sick s {
        Pending -> damn "Task is waiting to start"
        InProgress -> damn "Task is currently running"
        Complete -> damn "Task finished successfully"
        Failed -> damn "Task encountered an error"
    }
}

sus current_status Status = Status.InProgress
vibez.spill("Status:", status_message(current_status))

# Scenario 8: Memory and Performance Testing
slay memory_intensive_test() {
    # Test large array creation and manipulation
    sus large_array []drip = []
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        large_array = append(large_array, i * i)
    }
    
    vibez.spill("Created array with", len(large_array), "elements")
    vibez.spill("First 5:", large_array[0], large_array[1], large_array[2], large_array[3], large_array[4])
    vibez.spill("Last element:", large_array[len(large_array) - 1])
}

memory_intensive_test()

# Test string concatenation performance
slay string_concatenation_test() {
    sus result tea = ""
    bestie (sus i drip = 0; i < 100; i = i + 1) {
        result = result + "test" + to_string(i) + " "
    }
    vibez.spill("Concatenated string length:", len(result))
}

string_concatenation_test()

# Scenario 9: Edge Cases and Boundary Testing
slay boundary_tests() {
    # Test array bounds
    sus small_array []drip = [1, 2, 3]
    vibez.spill("Array length:", len(small_array))
    
    # Test string operations
    sus empty_str tea = ""
    sus single_char tea = "a"
    sus long_str tea = "This is a longer string for testing purposes and edge cases"
    
    vibez.spill("Empty string length:", len(empty_str))
    vibez.spill("Single char length:", len(single_char))
    vibez.spill("Long string length:", len(long_str))
    
    # Test numeric edge cases
    sus zero drip = 0
    sus negative drip = -42
    sus large_num drip = 999999
    
    vibez.spill("Zero value:", zero)
    vibez.spill("Negative value:", negative)
    vibez.spill("Large number:", large_num)
}

boundary_tests()

# Scenario 10: Integration Test
slay integration_test() {
    vibez.spill("=== CURSED Integration Test ===")
    
    # Combine multiple features
    sus test_data []drip = [5, 2, 8, 1, 9, 3]
    
    # Use higher-order functions
    sus processed []drip = map(
        filter(test_data, slay(x drip) lit { damn x > 3 }),
        slay(x drip) drip { damn x * 2 }
    )
    
    vibez.spill("Original data:", test_data)
    vibez.spill("Filtered (>3) and doubled:", processed)
    
    # Test with error handling
    sus safe_result drip = safe_divide(sum(processed), len(processed)) fam {
        when _ -> {
            vibez.spill("Error in average calculation")
            damn 0
        }
    }
    
    vibez.spill("Average of processed data:", safe_result)
    
    # Test with concurrency
    sus notification chan<tea> = make_channel()
    
    go {
        notification <- "Integration test completed successfully!"
    }
    
    sus message tea = <-notification
    vibez.spill(message)
}

integration_test()

vibez.spill("=== ALL TEST SCENARIOS COMPLETED ===")
vibez.spill("If you see this message, basic CURSED functionality is working!")
vibez.spill("Report any errors, crashes, or unexpected behavior to the Bug Bash team.")
