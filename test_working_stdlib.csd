// Test working stdlib functions

yeet "vibez"

slay main() {
    vibez.spill("=== Testing Working Stdlib Functions ===");
    
    // Test basic vibez functions that work
    vibez.spill("✅ Basic vibez.spill works");
    vibez.spill_int(42);
    vibez.spill_float(3.14);
    vibez.spill_bool(based);
    vibez.spill_char('A');
    
    // Test format functions
    sus int_str tea = vibez.format_int(99);
    vibez.spill("✅ Formatted int: " + int_str);
    
    sus float_str tea = vibez.format_float(1.23);
    vibez.spill("✅ Formatted float: " + float_str);
    
    sus bool_str tea = vibez.format_bool(based);
    vibez.spill("✅ Formatted bool: " + bool_str);
    
    // Test debug functions
    vibez.debug_print("This is a debug message");
    vibez.info_print("This is an info message");
    vibez.success_print("This is a success message");
    
    // Test color functions
    sus red_text tea = vibez.color_red("Red text");
    vibez.spill("✅ Color test: " + red_text);
    
    sus green_text tea = vibez.color_green("Green text");
    vibez.spill("✅ Color test: " + green_text);
    
    vibez.spill("=== All working stdlib functions tested! ===");
}
