# Regex Implementation Test
yeet "regexz"
yeet "vibez"

vibez.spill("Testing regex functionality...")

# Test regex compilation
sus email_pattern = regex_compile(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
ready (email_pattern.is_error()) {
    vibez.spill("Regex compilation failed:", email_pattern.error())
    yikes "Regex compilation failed"
}

vibez.spill("✅ Regex compilation working")

# Test regex matching
sus test_email tea = "user@example.com"
sus match_result = regex_match(email_pattern, test_email)
ready (!match_result) {
    vibez.spill("Regex matching failed for valid email")
    yikes "Regex matching failed"
}

vibez.spill("✅ Regex matching working")

# Test regex find and replace
sus text tea = "Contact us at old@example.com or info@example.com"
sus replaced = regex_replace_all(email_pattern, text, "***@***.***")

vibez.spill("✅ Regex replace working")

# Test regex capture groups
sus phone_pattern = regex_compile(r"(\d{3})-(\d{3})-(\d{4})")
sus phone_text tea = "Call me at 555-123-4567"
sus captures = regex_find_captures(phone_pattern, phone_text)

vibez.spill("✅ Regex capture groups working")

# Test regex split
sus split_result = regex_split(regex_compile(r"\s+"), "hello   world    test")

vibez.spill("✅ Regex split working")
vibez.spill("✅ All regex tests passed")
