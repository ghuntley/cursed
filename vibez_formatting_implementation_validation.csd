fr fr VIBEZ Formatting Implementation Validation
fr fr Test the enhanced formatting without placeholders

yeet "vibez"

fr fr ===== BASIC VALIDATION TESTS =====

spill("=== VIBEZ Enhanced Formatting Validation ===")
spill("\n")

fr fr Test 1: Basic output functions
spill("Test 1: Basic output functions")
spill("\n")

spillln("Hello from enhanced VIBEZ!")
spill_two("Multiple", "arguments")
spill("\n")

fr fr Test 2: Console formatting
spill("Test 2: Console formatting")
spill("\n")

print_header("VIBEZ Testing")
print_success("Formatting system loaded successfully")
print_info("Running validation tests")
print_warning("This is a test warning")
print_separator()

fr fr Test 3: String operations  
spill("Test 3: String operations")
spill("\n")

sus greeting tea = "Hello"
sus target tea = "World"
sus combined tea = string_concat(greeting, " ")
combined = string_concat(combined, target)

spill("Combined string: ")
spillln(combined)

sus length drip = string_length(combined)
spill("String length: ")
spillln(number_to_string(length))

fr fr Test 4: Basic file operations simulation
spill("Test 4: File operations")
spill("\n")

sus test_filename tea = "/tmp/vibez_test.txt" 
sus test_content tea = "This is test content from enhanced VIBEZ module."

sus write_result lit = write_file(test_filename, test_content)
ready write_result {
    spillln("✅ File write successful")
}
otherwise {
    spillln("❌ File write failed")
}

sus exists_result lit = file_exists(test_filename)  
ready exists_result {
    spillln("✅ File exists check successful")
}
otherwise {
    spillln("❌ File exists check failed")
}

sus read_result tea = read_file(test_filename)
spill("Read file length: ")
spillln(number_to_string(string_length(read_result)))

fr fr Test 5: Input simulation (scan functions)
spill("Test 5: Input functions")
spill("\n")

spillln("Testing scan functions (simulated input)...")
sus input_result tea = scan()
spill("Scan result length: ")
spillln(number_to_string(string_length(input_result)))

sus input_line tea = scanln()
spill("Scanln result length: ")
spillln(number_to_string(string_length(input_line)))

fr fr Test 6: Error handling
spill("Test 6: Error handling")
spill("\n")

sus null_test lit = spill(cringe)
ready null_test == cap {
    spillln("✅ Null input handling works")
}
otherwise {
    spillln("❌ Null input handling failed")
}

sus empty_concat tea = string_concat("", "test")
spill("Empty concat result: ")
spillln(empty_concat)

fr fr Test 7: Character operations
spill("Test 7: Character operations")
spill("\n")

sus test_char normie = char_at("Hello", 1)
spill("Character at position 1: ")
spillln(number_to_string(test_char))

sus char_string tea = char_to_string(72)  fr fr 'H'
spill("Character 72 as string: ")
spillln(char_string)

fr fr Test 8: Utility functions
spill("Test 8: Utility functions")
spill("\n")

sus find_result drip = find_first_placeholder("Hello {} World")
spill("Placeholder position: ")
spillln(number_to_string(find_result))

sus sub_result tea = substring("Hello World", 0, 5)
spill("Substring result: ")
spillln(sub_result)

fr fr Final validation
spill("\nFinal validation:")
spill("\n")

print_success("All basic VIBEZ functions tested")
print_info("Enhanced formatting system operational")
print_separator()

spillln("🎉 VIBEZ Enhanced Implementation Validation Complete!")
