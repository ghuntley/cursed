// Minimal validation test
yeet "validation"

sus test_email tea = "test@example.com"
vibez.spill("Testing email validation")

sus result ValidationResult = validate_email(test_email)
lowkey result.is_valid {
    vibez.spill("✅ Email validation passed")
} else {
    vibez.spill("❌ Email validation failed")
}

sus test_phone tea = "1234567890"
vibez.spill("Testing phone validation")

sus phone_result ValidationResult = validate_phone_number(test_phone)
lowkey phone_result.is_valid {
    vibez.spill("✅ Phone validation passed")
} else {
    vibez.spill("❌ Phone validation failed")
}

sus test_url tea = "https://www.example.com"
vibez.spill("Testing URL validation")

sus url_result ValidationResult = validate_url(test_url)
lowkey url_result.is_valid {
    vibez.spill("✅ URL validation passed")
} else {
    vibez.spill("❌ URL validation failed")
}

sus test_card tea = "4111111111111111"
vibez.spill("Testing credit card validation")

sus card_result ValidationResult = validate_credit_card(test_card)
lowkey card_result.is_valid {
    vibez.spill("✅ Credit card validation passed")
} else {
    vibez.spill("❌ Credit card validation failed")
}

sus test_date tea = "2023-12-31"
sus date_format tea = "YYYY-MM-DD"
vibez.spill("Testing date validation")

sus date_result ValidationResult = validate_date_format(test_date, date_format)
lowkey date_result.is_valid {
    vibez.spill("✅ Date validation passed")
} else {
    vibez.spill("❌ Date validation failed")
}

vibez.spill("All validation tests completed")
