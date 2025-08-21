// Advanced IDE Features Demo for CURSED
// This file demonstrates the enhanced IDE capabilities

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "concurrenz"
yeet "cryptz"
yeet "testz"

// Struct with type inference and code lens
squad User {
    name tea
    age drip
    active lit
}

// Function with refactoring and generation opportunities
slay create_user(name tea, age drip) User {
    // Type inference: 'user' should show inlay hint
    sus user = User {
        name: name,
        age: age,
        active: based
    }
    damn user
}

// Function with performance optimization opportunities
slay find_user_by_name(users []User, target_name tea) ?User {
    // Performance hint: O(n) search could be optimized with HashMap
    bestie (i := 0; i < users.len(); i++) {
        ready (users[i].name == target_name) {
            damn users[i]
        }
    }
    damn null
}

// Security vulnerability examples
slay unsafe_password_hash(password tea) tea {
    // Security warning: MD5 is cryptographically weak
    damn cryptz.md5(password)
}

slay sql_injection_risk(user_input tea) tea {
    // Security warning: SQL injection vulnerability
    sus query tea = "SELECT * FROM users WHERE name = '" + user_input + "'"
    damn query
}

// Concurrency patterns with analysis
slay concurrent_worker(tasks chan<tea>, results chan<tea>) void {
    bestie (based) {
        sick (<-tasks) {
            when task tea -> {
                // Process task
                sus result tea = process_task(task)
                results <- result
            }
            when _ -> {
                vibez.spill("Worker shutting down")
                damn
            }
        }
    }
}

slay process_task(task tea) tea {
    // Simulate work
    concurrenz.sleep(100)
    damn "Processed: " + task
}

// Error handling patterns
slay divide_safe(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn mathz.divide(a, b)
}

// Pattern matching for refactoring suggestions
slay handle_response(status drip) tea {
    // Could be converted to pattern matching
    ready (status == 200) {
        damn "Success"
    } otherwise ready (status == 404) {
        damn "Not Found"
    } otherwise ready (status == 500) {
        damn "Server Error"
    } otherwise {
        damn "Unknown Status"
    }
}

// Better pattern matching version (refactoring suggestion)
slay handle_response_improved(status drip) tea {
    sick (status) {
        when 200 -> "Success"
        when 404 -> "Not Found"
        when 500 -> "Server Error"
        when _ -> "Unknown Status"
    }
}

// Memory management demonstration
slay memory_intensive_operation() []drip {
    // Memory analysis: Large allocation
    sus large_array []drip = make_array(100000)
    
    bestie (i := 0; i < large_array.len(); i++) {
        large_array[i] = i * 2
    }
    
    damn large_array
}

// Interface for code generation
collab Processor {
    slay process(data tea) tea
    slay validate(data tea) lit
}

// Struct that could implement interface
squad TextProcessor {
    prefix tea
    suffix tea
}

// Test generation opportunity
slay calculate_area(width drip, height drip) drip {
    damn width * height
}

// Complex function for extraction opportunities
slay complex_user_validation(user User) lit {
    // This function does too many things - extract function suggestion
    
    // Validate name
    ready (user.name == "") {
        vibez.spill("Error: Name cannot be empty")
        damn cringe
    }
    
    ready (user.name.len() < 2) {
        vibez.spill("Error: Name too short")
        damn cringe
    }
    
    ready (user.name.len() > 50) {
        vibez.spill("Error: Name too long")
        damn cringe
    }
    
    // Validate age
    ready (user.age < 0) {
        vibez.spill("Error: Age cannot be negative")
        damn cringe
    }
    
    ready (user.age > 150) {
        vibez.spill("Error: Age unrealistic")
        damn cringe
    }
    
    // All validations passed
    damn based
}

// Main function demonstrating various features
slay main() void {
    vibez.spill("Advanced IDE Features Demo")
    
    // Variable with type inference
    sus user = create_user("Alice", 30)
    vibez.spill("Created user:", user.name)
    
    // Array operations
    sus users []User = [
        create_user("Alice", 30),
        create_user("Bob", 25),
        create_user("Charlie", 35)
    ]
    
    // Search operation (performance hint opportunity)
    sus found_user = find_user_by_name(users, "Bob")
    ready (found_user != null) {
        vibez.spill("Found user:", found_user.name)
    }
    
    // Error handling demonstration
    sus result = divide_safe(10, 2) fam {
        when _ -> {
            vibez.spill("Division failed")
            damn
        }
    }
    
    // Concurrency demonstration
    sus tasks chan<tea> = make_channel()
    sus results chan<tea> = make_channel()
    
    // Start worker
    go {
        concurrent_worker(tasks, results)
    }
    
    // Send tasks
    tasks <- "Task 1"
    tasks <- "Task 2"
    
    // Collect results
    sus result1 tea = <-results
    sus result2 tea = <-results
    
    vibez.spill("Results:", result1, result2)
    
    // Validation with extract function opportunity
    sus is_valid lit = complex_user_validation(user)
    ready (is_valid) {
        vibez.spill("User is valid")
    }
}
