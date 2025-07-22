# Simple test for enhanced modules
yeet "testz"
yeet "vibez"
yeet "dropz"

test_start("Basic enhanced functionality")

# Test basic string formatting
vibez.spill("Testing enhanced vibez formatting...")
sus result tea = vibez.spillstr("Hello %s", "World")
vibez.spill("Formatted result: " + result)

# Test basic file operations
vibez.spill("Testing enhanced dropz operations...")
sus data, err := dropz.read_file("test.txt")
vibez.spill("Read file result - error: " + err)

# Test string utilities
sus len normie = dropz.string_length("Hello")
vibez.spill("String length: " + vibez.format_number(len))

sus contains lit = dropz.string_contains("Hello World", "World")
vibez.spill("Contains test: " + vibez.format_bool(contains))

print_test_summary()
