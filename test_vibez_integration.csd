# Test vibez module integration in compiled CURSED code

slay main() normie {
    # Test basic vibez.spill functionality
    vibez.spill("Testing vibez module integration")
    
    # Test format function with placeholders
    sus name tea = "CURSED"
    sus version drip = 2.0
    sus format_result tea = vibez.format("Language: {}, Version: {:.1}", [name, version])
    vibez.spill(format_result)
    
    # Test sprintf function with C-style formatting
    sus sprintf_result tea = vibez.sprintf("Integer: %d, Float: %.2f", [42, 3.14159])
    vibez.spill(sprintf_result)
    
    # Test debug logging
    vibez.debug.log(3, "This is an info message", "main")
    vibez.debug.log(4, "This is a debug message", "test")
    
    # Test debug inspect functionality
    sus test_array [normie] = [1, 2, 3, 4, 5]
    vibez.debug.inspect(test_array, "test_array")
    
    vibez.spill("All vibez functions tested successfully!")
    damn 0
}
