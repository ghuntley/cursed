// Test Stage 1: Comprehensive Feature Test
// Tests multiple Stage 1 features together

// Variable declarations
sus program_name: lit = "CURSED Bootstrap Test"
sus version: normie = 1
sus pi: tea = 3.14159

// Function to test basic arithmetic
slay calculate(x: normie, y: normie) -> normie {
    sus result: normie = x * y + 10
    return result
}

// Function to test string operations
slay format_message(name: lit, value: normie) -> lit {
    // Simple string concatenation test
    return name + ": " + "value"  // Note: may need to convert normie to lit
}

// Function to test control flow
slay count_down(start: normie) {
    bestie (sus i: normie = start; i > 0; i--) {
        print(i)
        lowkey (i == 1) {
            print("Blast off!")
            periodt
        }
    }
}

// Main program logic
print(program_name)
print("Version:")
print(version)

sus calc_result: normie = calculate(5, 3)
print("Calculation result:")
print(calc_result)

print("Countdown:")
count_down(5)

// Test nested conditions
lowkey (version == 1) {
    lowkey (calc_result > 20) {
        print("All tests passed!")
    } highkey {
        print("Some tests failed")
    }
} highkey {
    print("Unknown version")
}
