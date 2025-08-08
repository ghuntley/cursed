yeet "vibez"

fr fr Simple test of enhanced vibez functions

vibez.spill("Testing enhanced I/O functions:")

fr fr Test parse functions
sus num normie = vibez.parse_int("42")
vibez.spill("Parsed integer 42")

sus flag lit = vibez.parse_bool("true")
vibez.spill("Parsed boolean true")

fr fr Test file operations
sus write_ok lit = vibez.write_file("simple_test.txt", "Test content")
vibez.spill("File write attempted")

sus content tea = vibez.read_file("simple_test.txt")
vibez.spill("File read attempted")

sus exists lit = vibez.file_exists("simple_test.txt")
vibez.spill("File existence checked")

fr fr Test string utilities
sus len normie = vibez.string_length("hello")
vibez.spill("String length calculated")

sus contains lit = vibez.string_contains("hello world", "world")
vibez.spill("String contains checked")

fr fr Test path utilities
sus ext tea = vibez.get_file_extension("test.txt")
vibez.spill("File extension extracted")

fr fr Clean up
sus deleted lit = vibez.delete_file("simple_test.txt")
vibez.spill("File deletion attempted")

vibez.spill("Simple test complete!")
