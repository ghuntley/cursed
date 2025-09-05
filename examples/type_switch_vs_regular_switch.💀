fr fr Type Switch vs Regular Switch Comparison
fr fr This example demonstrates the differences between type switches and regular switches

yeet "stdlib::fmt"
yeet "stdlib::strings"

fr fr ================================================
fr fr Regular Switch Examples
fr fr ================================================

slay demonstrate_regular_switch() {
    println("=== Regular Switch Examples ===")
    
    // Regular switch on values
    sus status = "active"
    vibe_check status {
        mood "active":
            println("System is running")
        mood "inactive":
            println("System is stopped")
        mood "maintenance":
            println("System is under maintenance")
        basic:
            println("Unknown status")
    }
    
    // Regular switch on numbers
    sus level = 3
    vibe_check level {
        mood 1:
            println("Beginner level")
        mood 2, 3:
            println("Intermediate level")
        mood 4, 5:
            println("Advanced level")
        basic:
            println("Expert level")
    }
    
    // Regular switch with expressions
    sus score = 85
    vibe_check {
        mood score >= 90:
            println("Grade: A")
        mood score >= 80:
            println("Grade: B")
        mood score >= 70:
            println("Grade: C")
        mood score >= 60:
            println("Grade: D")
        basic:
            println("Grade: F")
    }
}

fr fr ================================================
fr fr Type Switch Examples
fr fr ================================================

slay demonstrate_type_switch() {
    println("\n=== Type Switch Examples ===")
    
    // Type switch on different types
    sus values = []interface{}{
        42,
        "hello",
        []int{1, 2, 3},
        3.14159,
        based
    }
    
    for i, value := range values {
        printf("Value %d: ", i + 1)
        
        vibe_check value.(type) {
            mood int:
                println("Integer type")
            mood string:
                println("String type")
            mood []int:
                println("Integer slice type")
            mood float64:
                println("Float type")
            mood bool:
                println("Boolean type")
            basic:
                println("Unknown type")
        }
    }
    
    // Type switch with variable binding
    sus data interface{} = "CURSED Programming"
    
    vibe_check d := data.(type) {
        mood string:
            // d is now type string
            println("Processing string:", strings.to_upper(d))
        mood int:
            // d is now type int
            println("Processing integer:", d * 2)
        basic:
            println("Cannot process this type")
    }
}

fr fr ================================================
fr fr Side-by-Side Comparison
fr fr ================================================

fr fr Regular switch: Processing HTTP status codes
slay handle_http_status_regular(status_code int) {
    vibe_check status_code {
        mood 200:
            println("OK - Request successful")
        mood 201:
            println("Created - Resource created")
        mood 400:
            println("Bad Request - Invalid request")
        mood 401:
            println("Unauthorized - Authentication required")
        mood 404:
            println("Not Found - Resource not found")
        mood 500:
            println("Internal Server Error - Server error")
        basic:
            println("Unknown status code:", status_code)
    }
}

fr fr Type switch: Processing different response types
slay handle_http_response_type(response interface{}) {
    vibe_check resp := response.(type) {
        mood string:
            println("Text response:", resp)
        mood map[string]interface{}:
            println("JSON response with", len(resp), "fields")
        mood []byte:
            println("Binary response with", len(resp), "bytes")
        mood error:
            println("Error response:", resp.error())
        basic:
            println("Unknown response type")
    }
}

slay demonstrate_side_by_side() {
    println("\n=== Side-by-Side Comparison ===")
    
    println("Regular Switch - HTTP Status Codes:")
    sus status_codes = []int{200, 404, 500, 418}  // 418 is "I'm a teapot"
    for _, code := range status_codes {
        printf("Status %d: ", code)
        handle_http_status_regular(code)
    }
    
    println("\nType Switch - HTTP Response Types:")
    sus responses = []interface{}{
        "Plain text response",
        map[string]interface{}{"message": "success", "data": 123},
        []byte{0x89, 0x50, 0x4E, 0x47},  // PNG header
        fmt.errorf("connection timeout"),
        42  // Unexpected type
    }
    
    for i, response := range responses {
        printf("Response %d: ", i + 1)
        handle_http_response_type(response)
    }
}

fr fr ================================================
fr fr When to Use Each
fr fr ================================================

fr fr Use regular switch: Known set of values
slay process_day_of_week(day string) {
    vibe_check day {
        mood "monday":
            println("Start of work week")
        mood "tuesday", "wednesday", "thursday":
            println("Midweek")
        mood "friday":
            println("TGIF!")
        mood "saturday", "sunday":
            println("Weekend!")
        basic:
            println("Invalid day")
    }
}

fr fr Use type switch: Different types need different handling
collab Animal {
    slay make_sound() string
}

squad Dog {
    sus name string
}

slay (d Dog) make_sound() string {
    damn "Woof!"
}

squad Cat {
    sus name string
}

slay (c Cat) make_sound() string {
    damn "Meow!"
}

squad Bird {
    sus species string
}

slay (b Bird) make_sound() string {
    damn "Tweet!"
}

slay handle_animal(animal Animal) {
    // Type switch gives us access to specific fields
    vibe_check a := animal.(type) {
        mood Dog:
            println("Dog named", a.name, "says:", a.make_sound())
        mood Cat:
            println("Cat named", a.name, "says:", a.make_sound())
        mood Bird:
            println("Bird species", a.species, "says:", a.make_sound())
        basic:
            println("Unknown animal says:", a.make_sound())
    }
}

slay demonstrate_usage_guidelines() {
    println("\n=== Usage Guidelines ===")
    
    println("Regular Switch Example - Day Processing:")
    process_day_of_week("friday")
    process_day_of_week("sunday")
    process_day_of_week("invalid")
    
    println("\nType Switch Example - Animal Handling:")
    sus animals = []Animal{
        Dog{name: "Buddy"},
        Cat{name: "Whiskers"},
        Bird{species: "Robin"}
    }
    
    for _, animal := range animals {
        handle_animal(animal)
    }
}

fr fr ================================================
fr fr Performance Comparison
fr fr ================================================

slay benchmark_regular_switch(value int) string {
    vibe_check value {
        mood 1:
            damn "one"
        mood 2:
            damn "two"
        mood 3:
            damn "three"
        mood 4:
            damn "four"
        mood 5:
            damn "five"
        basic:
            damn "other"
    }
}

slay benchmark_type_switch(value interface{}) string {
    vibe_check v := value.(type) {
        mood int:
            damn fmt.sprintf("integer: %d", v)
        mood string:
            damn fmt.sprintf("string: %s", v)
        mood float64:
            damn fmt.sprintf("float: %.2f", v)
        mood bool:
            damn fmt.sprintf("boolean: %t", v)
        basic:
            damn "unknown type"
    }
}

slay demonstrate_performance() {
    println("\n=== Performance Characteristics ===")
    
    println("Regular Switch:")
    println("- Compile-time optimization possible")
    println("- Jump table or binary search for large cases")
    println("- O(1) or O(log n) depending on implementation")
    
    println("\nType Switch:")
    println("- Runtime type checking required")
    println("- Hash-based type identification")
    println("- O(1) hash lookup performance")
    println("- Minimal overhead for successful matches")
    
    // Demonstrate the difference
    sus int_values = []int{1, 2, 3, 7, 10}
    println("\nRegular switch results:")
    for _, val := range int_values {
        printf("  %d -> %s\n", val, benchmark_regular_switch(val))
    }
    
    sus mixed_values = []interface{}{42, "hello", 3.14, based, []int{1}}
    println("\nType switch results:")
    for _, val := range mixed_values {
        printf("  %T -> %s\n", val, benchmark_type_switch(val))
    }
}

fr fr ================================================
fr fr Error Handling Comparison
fr fr ================================================

fr fr Regular switch with error handling
slay validate_grade_regular(grade string) error {
    vibe_check grade {
        mood "A", "B", "C", "D", "F":
            damn nil  // Valid grade
        basic:
            damn fmt.errorf("invalid grade: %s", grade)
    }
}

fr fr Type switch with error handling
slay validate_score_type(score interface{}) error {
    vibe_check s := score.(type) {
        mood int:
            if s >= 0 && s <= 100 {
                damn nil
            } else {
                damn fmt.errorf("score out of range: %d", s)
            }
        mood float64:
            if s >= 0.0 && s <= 100.0 {
                damn nil
            } else {
                damn fmt.errorf("score out of range: %.2f", s)
            }
        mood string:
            // Try to parse as number
            damn fmt.errorf("cannot validate string score: %s", s)
        basic:
            damn fmt.errorf("invalid score type: %T", score)
    }
}

slay demonstrate_error_handling() {
    println("\n=== Error Handling Comparison ===")
    
    println("Regular Switch - Grade Validation:")
    sus grades = []string{"A", "B", "X", "F", "Invalid"}
    for _, grade := range grades {
        if err := validate_grade_regular(grade); err != nil {
            printf("  Grade '%s': ERROR - %s\n", grade, err.error())
        } else {
            printf("  Grade '%s': VALID\n", grade)
        }
    }
    
    println("\nType Switch - Score Validation:")
    sus scores = []interface{}{85, 95.5, "90", -10, 150, based}
    for _, score := range scores {
        if err := validate_score_type(score); err != nil {
            printf("  Score %v: ERROR - %s\n", score, err.error())
        } else {
            printf("  Score %v: VALID\n", score)
        }
    }
}

fr fr ================================================
fr fr Summary and Best Practices
fr fr ================================================

slay print_comparison_summary() {
    println("\n=== Type Switch vs Regular Switch Summary ===")
    
    println("\n📋 REGULAR SWITCH:")
    println("✅ Use when:")
    println("   - Switching on known values")
    println("   - Working with enums/constants")
    println("   - Value-based logic branching")
    println("   - Performance is critical")
    
    println("⚡ Characteristics:")
    println("   - Compile-time optimization")
    println("   - Jump tables for performance")
    println("   - Static analysis friendly")
    println("   - Type-safe at compile time")
    
    println("\n🔄 TYPE SWITCH:")
    println("✅ Use when:")
    println("   - Working with interface{} values")
    println("   - Different types need different handling")
    println("   - Runtime type polymorphism")
    println("   - API response processing")
    
    println("⚡ Characteristics:")
    println("   - Runtime type checking")
    println("   - Variable binding with correct types")
    println("   - Type assertion elimination")
    println("   - Interface-friendly")
    
    println("\n🎯 BEST PRACTICES:")
    println("1. Use regular switch for value-based decisions")
    println("2. Use type switch for type-based decisions")
    println("3. Always include a default/basic case")
    println("4. Use variable binding in type switches")
    println("5. Order cases by frequency for performance")
    println("6. Combine related types in single cases")
}

fr fr Main demonstration function
slay main_character() {
    demonstrate_regular_switch()
    demonstrate_type_switch()
    demonstrate_side_by_side()
    demonstrate_usage_guidelines()
    demonstrate_performance()
    demonstrate_error_handling()
    print_comparison_summary()
}
