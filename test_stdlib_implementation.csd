// Test basic stdlib functions that were implemented

yeet "vibez"

slay main() {
    vibez.spill("Testing implemented stdlib functions:");
    
    // Test basic vibez functions
    vibez.spill("Basic vibez.spill works");
    vibez.println("Basic vibez.println works");
    
    // Test type-specific functions
    vibez.spill_int(42);
    vibez.spill_float(3.14);
    vibez.spill_bool(based);
    vibez.spill_char('A');
    
    // Test with newlines
    vibez.println_int(100);
    vibez.println_float(2.71);
    vibez.println_bool(cap);
    vibez.println_char('Z');
    
    // Test string formatting
    sus int_str tea = vibez.format_int(99);
    vibez.spill("Formatted int: " + int_str);
    
    sus float_str tea = vibez.format_float(1.23);
    vibez.spill("Formatted float: " + float_str);
    
    sus bool_str tea = vibez.format_bool(based);
    vibez.spill("Formatted bool: " + bool_str);
    
    vibez.spill("All basic stdlib functions work!");
}
