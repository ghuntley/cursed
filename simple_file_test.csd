yeet "vibez"

vibez.spill("Testing basic file operations...")

fr fr Test runtime function directly
sus content tea = runtime_read_file("nonexistent.txt")
vibez.spill("Direct runtime call result:", content)

fr fr Test writing
sus write_result lit = runtime_write_file("test_output.txt", "Hello World!")
vibez.spill("Write result:", write_result)

fr fr Test reading back
sus read_content tea = runtime_read_file("test_output.txt") 
vibez.spill("Read content:", read_content)

vibez.spill("Test completed!")
