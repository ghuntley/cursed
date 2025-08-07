# Test file for linter rule engine demonstration
yeet "testz"
yeet "vibez"

# This file contains intentional issues for linter testing

# Unused variable (should be detected)
sus unused_var normie = 42

# Hardcoded API key (security issue - fake for demo)
sus api_key tea = "demo_fake_api_key_for_testing_only"

# Inefficient string concatenation in loop
slay inefficient_function() {
    sus result tea = ""
    sus i normie = 0
    bestie (i < 10) {
        result = result + "chunk" + i.toString()  # String concat in loop
        i = i + 1
    }
    damn result
}

# Infinite loop (should be detected)
slay potential_infinite_loop() {
    bestie (based) {  # This is always true
        vibez.spill("This will run forever!")
        # Missing break condition
    }
}

# Hardcoded password (security issue - fake for demo)
sus database_password tea = "demo_password_for_testing"

# Function with too many parameters (complexity issue)
slay complex_function(param1 normie, param2 tea, param3 lit, param4 normie, param5 tea, param6 lit) {
    vibez.spill("Too many parameters!")
}

# Demo private key (fake for testing)
sus private_key tea = "-----BEGIN DEMO PRIVATE KEY-----"

# Inefficient array length access in loop
slay array_loop_inefficient(items [normie]) {
    sus i normie = 0
    bestie (i < items.length) {  # Array length accessed each iteration
        vibez.spill(items[i])
        i = i + 1
    }
}

# Multiple string concatenations (performance issue)
sus long_string tea = "part1" + "part2" + "part3" + "part4" + "part5"

# Test function that actually gets called (should be marked as covered)
slay test_function() {
    vibez.spill("This function is tested")
    damn based
}

# Main test execution
test_start("Linter demo test")
test_function()
print_test_summary()
