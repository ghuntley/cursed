yeet "validation"

vibez.spill("🔍 Testing CURSED Validation Module")
vibez.spill("==================================")

// Test basic validation result creation
sus result ValidationResult = create_validation_result()
lowkey result.is_valid {
    vibez.spill("✅ Basic validation result creation works")
} else {
    vibez.spill("❌ Basic validation result creation failed")
}

// Test simple not empty validation
sus test_result ValidationResult = validate_not_empty("test")
lowkey test_result.is_valid {
    vibez.spill("✅ Not empty validation works")
} else {
    vibez.spill("❌ Not empty validation failed")
}

// Test empty validation (should fail)
sus empty_result ValidationResult = validate_not_empty("")
lowkey empty_result.is_valid {
    vibez.spill("❌ Empty validation should have failed")
} else {
    vibez.spill("✅ Empty validation correctly failed")
}

// Test email validation
sus email_result ValidationResult = validate_email("test@example.com")
lowkey email_result.is_valid {
    vibez.spill("✅ Email validation works")
} else {
    vibez.spill("❌ Email validation failed")
}

// Test phone validation
sus phone_result ValidationResult = validate_phone_number("1234567890")
lowkey phone_result.is_valid {
    vibez.spill("✅ Phone validation works")
} else {
    vibez.spill("❌ Phone validation failed")
}

// Test URL validation
sus url_result ValidationResult = validate_url("https://www.example.com")
lowkey url_result.is_valid {
    vibez.spill("✅ URL validation works")
} else {
    vibez.spill("❌ URL validation failed")
}

// Test credit card validation
sus card_result ValidationResult = validate_credit_card("4111111111111111")
lowkey card_result.is_valid {
    vibez.spill("✅ Credit card validation works")
} else {
    vibez.spill("❌ Credit card validation failed")
}

// Test date validation
sus date_result ValidationResult = validate_date_format("2023-12-31", "YYYY-MM-DD")
lowkey date_result.is_valid {
    vibez.spill("✅ Date validation works")
} else {
    vibez.spill("❌ Date validation failed")
}

vibez.spill("🎉 All validation tests completed!")
