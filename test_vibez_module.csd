fr fr Test the new vibez module implementation
fr fr This tests the pure CURSED implementation

yeet "vibez"

fr fr Test basic output
vibez.spill("=== Testing VIBEZ Module ===")
vibez.spillln("Testing spillln() function")

fr fr Test formatted output
sus name tea = "CURSED Developer"
sus age tea = "25"
vibez.spillf("Hello %s, age %s", [name, age])

fr fr Test multiple values
vibez.spill_values(["Value1", "Value2", "Value3"])

fr fr Test custom separator
vibez.spill_sep(" | ", ["A", "B", "C"])

fr fr Test specialized output
vibez.spill_error("This is an error message")
vibez.spill_warning("This is a warning message") 
vibez.spill_debug("This is debug information")

fr fr Test string formatting without printing
sus formatted tea = vibez.spillstr("Formatted: %s", ["Test"])
vibez.spill("Result: " + formatted)

fr fr Test file operations
sus write_success lit, write_error tea = vibez.write_file("/tmp/test.txt", "Hello from CURSED!")
ready write_error != "" {
    vibez.spill_error("Write failed: " + write_error)
} otherwise {
    vibez.spill("✅ File write successful")
    
    sus content tea, read_error tea = vibez.read_file("/tmp/test.txt")
    ready read_error != "" {
        vibez.spill_error("Read failed: " + read_error)
    } otherwise {
        vibez.spill("✅ File content: " + content)
    }
}

vibez.spill("✅ VIBEZ module test completed!")
