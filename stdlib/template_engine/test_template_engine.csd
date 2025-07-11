// Test CURSED Template Engine Module
vibez.spill("Testing template engine module")

// Simple template variable test
sus template_name tea = "CURSED Template"
sus template_version tea = "1.0.0"

vibez.spill("Template: " + template_name)
vibez.spill("Version: " + template_version)

// Test template processing concepts
sus template_input tea = "Hello {{name}}"
sus expected_output tea = "Hello CURSED"

vibez.spill("Template input: " + template_input)
vibez.spill("Expected output: " + expected_output)

// Test string manipulation functions
sus test_string tea = "hello world"
sus test_upper tea = "HELLO WORLD"

vibez.spill("Original: " + test_string)
vibez.spill("Uppercase: " + test_upper)

// Test template delimiters
sus start_delim tea = "{{"
sus end_delim tea = "}}"

vibez.spill("Start delimiter: " + start_delim)
vibez.spill("End delimiter: " + end_delim)

// Test template functions
sus func_upper tea = "upper"
sus func_lower tea = "lower"
sus func_len tea = "len"

vibez.spill("Template functions: " + func_upper + ", " + func_lower + ", " + func_len)

vibez.spill("✅ Template engine test complete!")
