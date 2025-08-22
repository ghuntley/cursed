# CURSED Error Handling Test
# Testing yikes, fam, shook constructs

# Function that can throw errors
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

slay parse_number(s tea) yikes<drip> {
    ready (s == "") {
        yikes "empty string"
    }
    ready (s == "invalid") {
        yikes "invalid number format"
    }
    damn 42  # Simplified parsing
}

# Test error handling
spill("=== ERROR HANDLING TEST ===")

# Test successful operations
sus result1 drip = divide(10, 2) fam {
    when "division by zero" -> {
        spill("Caught division by zero error")
        damn 0
    }
}
spill("Division 10/2:", result1)

# Test error case
sus result2 drip = divide(10, 0) fam {
    when "division by zero" -> {
        spill("Successfully caught division by zero")
        damn -1
    }
}
spill("Division 10/0 (caught):", result2)

# Test parsing errors
sus num1 drip = parse_number("123") fam {
    when "empty string" -> {
        spill("Caught empty string error")
        damn 0
    }
    when "invalid number format" -> {
        spill("Caught invalid format error")
        damn 0
    }
}
spill("Parsed valid number:", num1)

sus num2 drip = parse_number("invalid") fam {
    when "empty string" -> {
        spill("Caught empty string error")
        damn 0
    }
    when "invalid number format" -> {
        spill("Caught invalid format error")
        damn -1
    }
}
spill("Parsed invalid number:", num2)

# Test shook (panic) if implemented
# shook "This is a panic test - should terminate program"
