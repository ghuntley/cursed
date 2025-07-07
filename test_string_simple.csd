// Simple String Function Test

slay main() {
    vibez.spill("Testing String Functions:");
    
    // Test basic string operations
    sus text tea = "Hello World";
    sus length normie = string_len(text);
    vibez.spill("String length: " + length.toString());
    
    sus upper tea = string_to_upper(text);
    vibez.spill("Uppercase: " + upper);
    
    sus lower tea = string_to_lower(text);
    vibez.spill("Lowercase: " + lower);
    
    // Test contains
    lit contains = string_contains(text, "World");
    vibez.spill("Contains 'World': " + contains.toString());
    
    // Test trim
    sus whitespace tea = "  hello  ";
    sus trimmed tea = string_trim(whitespace);
    vibez.spill("Trimmed: '" + trimmed + "'");
    
    // Test repeat
    sus repeated tea = string_repeat("ha", 3);
    vibez.spill("Repeated: " + repeated);
    
    vibez.spill("String tests completed!");
}
