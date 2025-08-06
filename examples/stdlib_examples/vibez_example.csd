fr fr CURSED I/O Framework (vibez) Example
fr fr Demonstrates comprehensive I/O and formatting capabilities

yeet "vibez"
yeet "mathz"

slay demonstrate_basic_output() cringe {
    vibez.spill("=== Basic Output Functions ===")
    
    fr fr Basic text output
    vibez.spill("Simple text output")
    vibez.spillln("Text with automatic newline")
    
    fr fr Multiple value output
    vibez.spill_values("Multiple", "values", "in", "one", "call")
    vibez.spill_sep(" | ", "Pipe", "separated", "values")
    
    vibez.spill("")  fr fr Empty line for spacing
}

slay demonstrate_formatted_output() cringe {
    vibez.spill("=== Formatted Output Functions ===")
    
    fr fr String formatting
    vibez.spillf("Hello %s, welcome to CURSED!", "developer")
    vibez.spillf("User: %s, ID: %d", "Alice", 12345)
    vibez.spillf("Name: %s, Age: %d", "Bob", 30)
    
    fr fr Number formatting
    sus pi_val meal = mathz.PI
    sus factorial_5 normie = mathz.factorial(5)
    vibez.spillf("Mathematical constants: PI = %f", pi_val)
    vibez.spillf("Calculations: 5! = %d", factorial_5)
    
    fr fr Complex formatting
    vibez.spillf("%s %s %s", "Multiple", "string", "arguments")
    vibez.spillf("Error: %s", "File not found")
    vibez.spillf("Result: %s", "Operation successful")
    
    vibez.spill("")
}

slay demonstrate_colored_output() cringe {
    vibez.spill("=== Colored Output Functions ===")
    
    fr fr Colored text output
    vibez.spill_colored("This text should appear in red!", "red")
    vibez.spill_colored("This text should appear in green!", "green")
    vibez.spill_colored("This text should appear in blue!", "blue")
    
    fr fr Manual color control
    vibez.set_color("red")
    vibez.spill("Manual red text")
    vibez.set_color("green")
    vibez.spill("Manual green text")
    vibez.set_color("reset")
    vibez.spill("Back to normal color")
    
    vibez.spill("")
}

slay demonstrate_specialized_output() cringe {
    vibez.spill("=== Specialized Output Functions ===")
    
    fr fr Specialized message types
    vibez.spill_error("This is an error message")
    vibez.spill_warning("This is a warning message")
    vibez.spill_debug("This is a debug message")
    
    fr fr Timestamped output
    vibez.spill_with_time("This message has a timestamp")
    
    vibez.spill("")
}

slay demonstrate_string_formatting() cringe {
    vibez.spill("=== String Formatting Functions ===")
    
    fr fr Format strings without immediate output
    sus formatted1 tea = vibez.spillstr("Hello %s", "World")
    sus formatted2 tea = vibez.spillstr("Number: %d", 42)
    sus formatted3 tea = vibez.spillstr("%s: %s", "Status", "Active")
    
    vibez.spill("Formatted strings:")
    vibez.spill(formatted1)
    vibez.spill(formatted2)
    vibez.spill(formatted3)
    
    vibez.spill("")
}

slay demonstrate_console_control() cringe {
    vibez.spill("=== Console Control Functions ===")
    
    vibez.spill("Console control features:")
    vibez.spill("- Clear screen (commented out to avoid clearing terminal)")
    vibez.spill("- Color control (demonstrated above)")
    vibez.spill("- Text formatting and positioning")
    
    fr fr Note: clear_screen() commented out to preserve output
    fr vibez.clear_screen()
    
    vibez.spill("")
}

slay demonstrate_performance_features() cringe {
    vibez.spill("=== Performance and Advanced Features ===")
    
    fr fr Efficient multi-value output
    vibez.spill("Efficient output for multiple values:")
    vibez.spill_values("Value1", "Value2", "Value3", "Value4", "Value5")
    
    fr fr Custom separators for data export
    vibez.spill("CSV-style output:")
    vibez.spill_sep(",", "Name", "Age", "City", "Country")
    vibez.spill_sep(",", "Alice", "25", "New York", "USA")
    vibez.spill_sep(",", "Bob", "30", "London", "UK")
    
    fr fr Formatted output with newlines
    vibez.spillfln("Formatted with newline: %s = %d", "Answer", 42)
    
    vibez.spill("")
}

slay main() cringe {
    vibez.spill("💬 CURSED I/O Framework (vibez) Demonstration")
    vibez.spill("=============================================")
    vibez.spill("")
    
    demonstrate_basic_output()
    demonstrate_formatted_output()
    demonstrate_colored_output()
    demonstrate_specialized_output()
    demonstrate_string_formatting()
    demonstrate_console_control()
    demonstrate_performance_features()
    
    vibez.spill("✅ vibez I/O Framework Demonstration Complete!")
    vibez.spill("")
    vibez.spill("Key Features Demonstrated:")
    vibez.spill("- Basic text output with spill(), spillln()")
    vibez.spill("- Formatted output with spillf(), spillstr()")
    vibez.spill("- Colored output with spill_colored(), set_color()")
    vibez.spill("- Multi-value output with spill_values(), spill_sep()")
    vibez.spill("- Specialized messages (error, warning, debug)")
    vibez.spill("- String formatting and console control")
    vibez.spill("- Performance-optimized output functions")
}
