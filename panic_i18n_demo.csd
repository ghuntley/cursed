# CURSED Panic Message Internationalization (i18n) Demo
# This program demonstrates the enhanced panic message system with Unicode support
# and localized error messages in multiple languages

yeet "testz"

# Test various panic scenarios with i18n support
slay demonstrate_i18n_panics() {
    vibez.spill("🌍 CURSED Panic Message Internationalization Demo")
    vibez.spill("=========================================")
    vibez.spill("")
    
    # Test 1: Basic yikes error with localization
    vibez.spill("Test 1: Basic yikes error")
    fam {
        yikes "This is a test error message"
    } shook (error) {
        vibez.spill("Caught error:", error)
    }
    
    # Test 2: Division by zero with locale-specific formatting
    vibez.spill("\nTest 2: Division by zero error")
    fam {
        sus result drip = 10 / 0  # This should trigger division by zero
        vibez.spill("Result:", result)
    } shook (error) {
        vibez.spill("Caught division by zero:", error)
    }
    
    # Test 3: Array bounds error with index information
    vibez.spill("\nTest 3: Array bounds error")
    fam {
        sus arr []drip = [1, 2, 3]
        sus value drip = arr[10]  # Out of bounds access
        vibez.spill("Value:", value)
    } shook (error) {
        vibez.spill("Caught bounds error:", error)
    }
    
    # Test 4: Memory allocation error simulation
    vibez.spill("\nTest 4: Memory error simulation")
    fam {
        # Simulate memory allocation failure
        yikes "Failed to allocate memory for large array"
    } shook (error) {
        vibez.spill("Caught memory error:", error)
    }
    
    # Test 5: Type mismatch error
    vibez.spill("\nTest 5: Type mismatch error")
    fam {
        sus number drip = "not a number"  # Type mismatch
        vibez.spill("Number:", number)
    } shook (error) {
        vibez.spill("Caught type error:", error)
    }
    
    # Test 6: Unicode message handling
    vibez.spill("\nTest 6: Unicode error messages")
    fam {
        yikes "Error with Unicode: 🚨 エラー 错误 خطأ"
    } shook (error) {
        vibez.spill("Caught Unicode error:", error)
    }
    
    # Test 7: Nested error with stack trace
    vibez.spill("\nTest 7: Nested function error")
    nested_function_that_fails()
}

slay nested_function_that_fails() {
    helper_function()
}

slay helper_function() {
    deep_function()
}

slay deep_function() {
    fam {
        yikes "Deep nested error with stack trace information"
    } shook (error) {
        vibez.spill("Caught in deep function:", error)
    }
}

# Test locale-specific formatting
slay test_locale_formatting() {
    vibez.spill("\n🌐 Locale-Specific Formatting Tests")
    vibez.spill("===================================")
    
    # Test number formatting for different locales
    sus large_number drip = 1234567
    vibez.spill("US format (1,234,567):", large_number)
    
    # Test currency formatting
    sus price drip = 99
    vibez.spill("Price formatting test:", price)
    
    # Test date/time formatting (simulated)
    vibez.spill("Current timestamp formatting test")
    
    # Test RTL language support simulation
    vibez.spill("RTL text simulation: مرحبا بالعالم")
}

# Test different error types with context
slay test_error_types_with_context() {
    vibez.spill("\n🎯 Error Types with Context")
    vibez.spill("============================")
    
    # Runtime error with function context
    fam {
        runtime_error_function()
    } shook (error) {
        vibez.spill("Runtime error caught:", error)
    }
    
    # Invalid operation error
    fam {
        invalid_operation_function()
    } shook (error) {
        vibez.spill("Invalid operation caught:", error)
    }
    
    # Null pointer simulation
    fam {
        null_pointer_function()
    } shook (error) {
        vibez.spill("Null pointer error caught:", error)
    }
}

slay runtime_error_function() {
    yikes "Runtime error in function execution"
}

slay invalid_operation_function() {
    yikes "Invalid operation: Cannot divide string by number"
}

slay null_pointer_function() {
    yikes "Null pointer dereference in memory access"
}

# Test language pack switching simulation
slay test_language_switching() {
    vibez.spill("\n🔄 Language Pack Switching Demo")
    vibez.spill("================================")
    
    vibez.spill("Simulating English (en-US) messages:")
    fam {
        yikes "Sample error message"
    } shook (error) {
        vibez.spill("EN:", error)
    }
    
    vibez.spill("\nSimulating Spanish (es-ES) messages:")
    # In a real implementation, this would switch the locale
    fam {
        yikes "Mensaje de error de muestra"
    } shook (error) {
        vibez.spill("ES:", error)
    }
    
    vibez.spill("\nSimulating French (fr-FR) messages:")
    fam {
        yikes "Message d'erreur d'exemple"
    } shook (error) {
        vibez.spill("FR:", error)
    }
    
    vibez.spill("\nSimulating German (de-DE) messages:")
    fam {
        yikes "Beispiel-Fehlermeldung"
    } shook (error) {
        vibez.spill("DE:", error)
    }
    
    vibez.spill("\nSimulating Japanese (ja-JP) messages:")
    fam {
        yikes "サンプルエラーメッセージ"
    } shook (error) {
        vibez.spill("JA:", error)
    }
    
    vibez.spill("\nSimulating Chinese (zh-CN) messages:")
    fam {
        yikes "示例错误消息"
    } shook (error) {
        vibez.spill("ZH:", error)
    }
    
    vibez.spill("\nSimulating Arabic (ar-SA) messages:")
    fam {
        yikes "رسالة خطأ عينة"
    } shook (error) {
        vibez.spill("AR:", error)
    }
}

# Test panic message formatting with placeholders
slay test_message_formatting() {
    vibez.spill("\n📝 Message Formatting with Placeholders")
    vibez.spill("========================================")
    
    # Test with multiple placeholders
    fam {
        sus file_name tea = "test.csd"
        sus line_number drip = 42
        sus error_code drip = 404
        
        # Simulate formatted error message
        yikes "Error in file {file} at line {line} (code: {code})"
    } shook (error) {
        vibez.spill("Formatted error:", error)
    }
    
    # Test with numeric formatting
    fam {
        sus array_size drip = 10
        sus invalid_index drip = 15
        
        yikes "Array index {index} exceeds bounds (size: {size})"
    } shook (error) {
        vibez.spill("Bounds error:", error)
    }
}

# Test Unicode normalization and RTL support
slay test_unicode_features() {
    vibez.spill("\n🔤 Unicode and RTL Support Tests")
    vibez.spill("==================================")
    
    # Test Unicode normalization
    vibez.spill("Unicode normalization test:")
    vibez.spill("Combined: é (e + ´)")
    vibez.spill("Precomposed: é")
    
    # Test emoji in error messages
    fam {
        yikes "Error with emojis: 🚨🔥💥🎯🌍"
    } shook (error) {
        vibez.spill("Emoji error:", error)
    }
    
    # Test mixed scripts
    fam {
        yikes "Mixed scripts: Hello 世界 مرحبا Здравствуй"
    } shook (error) {
        vibez.spill("Mixed script error:", error)
    }
    
    # Test RTL text handling
    vibez.spill("\nRTL text handling:")
    vibez.spill("Arabic: مرحبا بكم في CURSED")
    vibez.spill("Hebrew: שלום וברכה ל-CURSED")
    vibez.spill("Persian: خوش آمدید به CURSED")
}

# Main demonstration function
slay main() {
    test_start("I18n Panic Message System Demo")
    
    vibez.spill("🚀 Starting CURSED I18n Panic Message System Demo")
    vibez.spill("==================================================")
    vibez.spill("")
    
    # Run all test suites
    demonstrate_i18n_panics()
    test_locale_formatting()
    test_error_types_with_context()
    test_language_switching()
    test_message_formatting()
    test_unicode_features()
    
    vibez.spill("\n✅ Demo completed successfully!")
    vibez.spill("The i18n panic message system provides:")
    vibez.spill("- Localized error messages in 30+ languages")
    vibez.spill("- Unicode support with proper normalization")
    vibez.spill("- RTL language support for Arabic, Hebrew, Persian")
    vibez.spill("- Template-based message formatting")
    vibez.spill("- Automatic locale detection")
    vibez.spill("- Configurable message truncation")
    vibez.spill("- Stack trace localization")
    vibez.spill("- Context-aware error formatting")
    
    print_test_summary()
}

# Run the demonstration
main()
