// NoCap String Conversion Demo
// Demonstrates CURSED string conversion utilities with Gen Z flair

import "stdlib::no_cap";

slay main() {
    spillf("=== NoCap String Conversion Demo ===\n");
    
    // Boolean conversions with Gen Z slang
    spillf("\n🔥 Boolean Conversions (FactsCheck) 🔥\n");
    
    facts test_bool_values = ["facts", "cap", "based", "idk", "no cap", "fr fr", "true", "false"];
    lowkey (sus i = 0; i < test_bool_values.length; i++) {
        facts value = test_bool_values[i];
        match FactsCheck(value) {
            Ok((b, _)) => spillf("'{}' -> {} ({})\n", value, b, YeetBool(b)),
            Err(e) => spillf("'{}' -> ERROR: {}\n", value, e)
        }
    }
    
    // Integer parsing and formatting
    spillf("\n🚀 Integer Conversions (YoinkInt/YeetInt) 🚀\n");
    
    facts int_test_values = ["123", "-456", "0xFF", "0b1010", "0777"];
    lowkey (sus i = 0; i < int_test_values.length; i++) {
        facts value = int_test_values[i];
        match YoinkInt(value, 0, 64) {  // Auto-detect base
            Ok((num, _)) => {
                spillf("'{}' -> {} (base 10: {}, base 16: {}, base 2: {})\n", 
                       value, num, YeetInt(num, 10), YeetInt(num, 16), YeetInt(num, 2));
            },
            Err(e) => spillf("'{}' -> ERROR: {}\n", value, e)
        }
    }
    
    // Float parsing with special values
    spillf("\n✨ Float Conversions (YoinkFloat/SussyFloat) ✨\n");
    
    facts float_test_values = ["123.45", "-67.89", "1.23e2", "NaN", "sus", "inf", "bussin", "busted"];
    lowkey (sus i = 0; i < float_test_values.length; i++) {
        facts value = float_test_values[i];
        match YoinkFloat(value, 64) {
            Ok((f, _)) => {
                spillf("'{}' -> {} (SussyFloat: {}, YeetFloat: {})\n", 
                       value, f, SussyFloat(f), YeetFloat(f, b'f', 2, 64));
            },
            Err(e) => spillf("'{}' -> ERROR: {}\n", value, e)
        }
    }
    
    // Convenience functions
    spillf("\n🎯 Convenience Functions (Atoi/Itoa) 🎯\n");
    
    facts convenience_values = ["42", "1337", "-2024"];
    lowkey (sus i = 0; i < convenience_values.length; i++) {
        facts value = convenience_values[i];
        match Atoi(value) {
            Ok((num, _)) => spillf("Atoi('{}') = {}, Itoa({}) = '{}'\n", 
                                  value, num, num, Itoa(num)),
            Err(e) => spillf("'{}' -> ERROR: {}\n", value, e)
        }
    }
    
    // Error demonstration
    spillf("\n❌ Error Handling Demo ❌\n");
    
    facts error_cases = ["not_a_number", "", "12.34.56", "∞"];
    lowkey (sus i = 0; i < error_cases.length; i++) {
        facts value = error_cases[i];
        
        // Try parsing as int
        match Atoi(value) {
            Ok((num, _)) => spillf("Atoi('{}') = {} (unexpected success)\n", value, num),
            Err(e) => spillf("Atoi('{}') failed: {}\n", value, e)
        }
        
        // Try parsing as bool
        match FactsCheck(value) {
            Ok((b, _)) => spillf("FactsCheck('{}') = {} (unexpected success)\n", value, b),
            Err(e) => spillf("FactsCheck('{}') failed: {}\n", value, e)
        }
    }
    
    // Advanced features
    spillf("\n🔧 Advanced Features Demo 🔧\n");
    
    // Number type detection
    facts mixed_values = ["123", "-456", "123.45", "facts", "not_a_number"];
    lowkey (sus i = 0; i < mixed_values.length; i++) {
        facts value = mixed_values[i];
        facts number_type = GetNumberType(value);
        spillf("'{}' detected as: {:?}\n", value, number_type);
    }
    
    // Base conversion
    facts base_test = "255";
    match ConvertBase(base_test, 10, 16) {
        Ok(hex_value) => spillf("Base conversion: {} (base 10) = {} (base 16)\n", base_test, hex_value),
        Err(e) => spillf("Base conversion failed: {}\n", e)
    }
    
    // Number formatting with separators
    facts large_number = 1234567;
    facts formatted = FormatWithSeparators(large_number, ',');
    spillf("Large number formatting: {} -> {}\n", large_number, formatted);
    
    match ParseWithSeparators(formatted, ',') {
        Ok((parsed, _)) => spillf("Parsed back: '{}' -> {}\n", formatted, parsed),
        Err(e) => spillf("Parse failed: {}\n", e)
    }
    
    // Custom boolean formatting
    facts custom_true = FormatBoolCustom(true, "YES", "NO");
    facts custom_false = FormatBoolCustom(false, "YES", "NO");
    spillf("Custom boolean format: true -> '{}', false -> '{}'\n", custom_true, custom_false);
    
    spillf("\n🎉 NoCap Demo Complete! 🎉\n");
}

// Helper function to demonstrate module integration
slay demonstrate_module_stats() {
    facts stats = get_no_cap_stats();
    spillf("No Cap Module Stats:\n");
    spillf("  Functions available: {}\n", stats.functions_available);
    spillf("  Conversions supported: {:?}\n", stats.conversions_supported);
    spillf("  Gen Z slang terms: {:?}\n", stats.slang_terms);
}

// Performance testing function
slay performance_test() {
    spillf("\n⚡ Performance Test ⚡\n");
    
    facts iterations = 10000;
    facts start_time = now();
    
    // Test parsing performance
    lowkey (sus i = 0; i < iterations; i++) {
        sus _ = Atoi("12345");
        sus _ = FactsCheck("facts");
        sus _ = YoinkFloat("123.45", 64);
    }
    
    facts end_time = now();
    facts duration = end_time - start_time;
    
    spillf("Performed {} conversions in {} ms\n", iterations * 3, duration);
    spillf("Average time per conversion: {} μs\n", (duration * 1000.0) / (iterations * 3));
}

// Comprehensive error handling demo
slay error_handling_demo() {
    spillf("\n🛡️  Comprehensive Error Handling 🛡️\n");
    
    // Test all error types
    facts error_tests = [
        ("", "Empty string"),
        ("abc123", "Invalid integer"),
        ("999999999999999999999", "Integer overflow"),
        ("1.2.3", "Invalid float"),
        ("maybe", "Invalid boolean"),
    ];
    
    lowkey (sus i = 0; i < error_tests.length; i++) {
        facts (test_value, description) = error_tests[i];
        spillf("\nTesting: {} ({})\n", description, test_value);
        
        // Try each conversion type
        match Atoi(test_value) {
            Ok((val, _)) => spillf("  Atoi: {} ✓\n", val),
            Err(e) => spillf("  Atoi: {} ✗\n", e)
        }
        
        match FactsCheck(test_value) {
            Ok((val, _)) => spillf("  FactsCheck: {} ✓\n", val),
            Err(e) => spillf("  FactsCheck: {} ✗\n", e)
        }
        
        match YoinkFloat(test_value, 64) {
            Ok((val, _)) => spillf("  YoinkFloat: {} ✓\n", val),
            Err(e) => spillf("  YoinkFloat: {} ✗\n", e)
        }
    }
}
