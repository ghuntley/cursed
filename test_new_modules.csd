// Simple test for new stdlib modules
// Tests basic functionality that the parser can handle

// Test basic variable declarations
vibez.spill("Testing new stdlib modules...")

// Test process module concepts
sus pid normie = 12345
sus cwd tea = "/home/user"
vibez.spill("Process ID: " + tea(pid))
vibez.spill("Current directory: " + cwd)

// Test logging module concepts
sus log_level normie = 2
sus log_message tea = "This is a test message"
vibez.spill("Log level: " + tea(log_level))
vibez.spill("Log message: " + log_message)

// Test validation module concepts
sus is_valid lit = based
sus error_count normie = 0
vibez.spill("Validation result: " + tea(is_valid))
vibez.spill("Error count: " + tea(error_count))

// Test string validation concepts
sus min_length normie = 5
sus max_length normie = 50
sus test_string tea = "Hello, World!"
sus string_length normie = len(test_string)

vibez.spill("String: " + test_string)
vibez.spill("Length: " + tea(string_length))
vibez.spill("Min length: " + tea(min_length))
vibez.spill("Max length: " + tea(max_length))

lowkey string_length >= min_length {
    vibez.spill("String length validation: PASSED (min)")
} highkey {
    vibez.spill("String length validation: FAILED (min)")
}

// Test numeric validation concepts
sus test_number normie = 42
sus min_value normie = 1
sus max_value normie = 100

vibez.spill("Number: " + tea(test_number))
vibez.spill("Min value: " + tea(min_value))
vibez.spill("Max value: " + tea(max_value))

lowkey test_number >= min_value {
    vibez.spill("Numeric validation: PASSED (min)")
} highkey {
    vibez.spill("Numeric validation: FAILED (min)")
}

// Test array validation concepts
sus test_array []tea = []tea{"item1", "item2", "item3"}
sus array_length normie = len(test_array)
sus expected_length normie = 3

vibez.spill("Array length: " + tea(array_length))
vibez.spill("Expected length: " + tea(expected_length))

lowkey array_length == expected_length {
    vibez.spill("Array validation: PASSED")
} highkey {
    vibez.spill("Array validation: FAILED")
}

vibez.spill("All basic module concepts tested successfully!")
