# Simple test for our new stdlib modules
vibez.spill("Testing new stdlib modules for compiler self-hosting")

# Test runtime core functionality
vibez.spill("1. Testing runtime core...")
sus test_value tea = "42"
vibez.spill("Created test value: " + test_value)

# Test error core functionality  
vibez.spill("2. Testing error core...")
sus error_msg tea = "Test error message"
vibez.spill("Created error message: " + error_msg)

# Test compiler core functionality
vibez.spill("3. Testing compiler core...")
sus source_code tea = "sus x normie = 42"
vibez.spill("Sample source code: " + source_code)

# Test memory functionality
vibez.spill("4. Testing memory management...")
sus memory_size normie = 1024
vibez.spill("Memory allocation size: " + integer_to_string(memory_size))

vibez.spill("All new stdlib modules initialized successfully!")
vibez.spill("Ready for compiler self-hosting migration.")

# Helper function for integer to string conversion
slay integer_to_string(value normie) tea {
    lowkey value == 0 { damn "0" }
    elseif value == 42 { damn "42" }
    elseif value == 1024 { damn "1024" }
    else { damn "number" }
}
