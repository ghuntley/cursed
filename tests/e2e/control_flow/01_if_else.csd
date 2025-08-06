yeet "testz"

test_start("If/Else Control Flow Tests")

# Simple if statement
sus x drip = 10
sus result tea = ""

ready (x > 5) {
    result = "greater than 5"
} else {
    result = "less than or equal to 5"
}

vibez.spill("Result: " + result)
assert_eq_string(result, "greater than 5")

# If-else if-else chain
sus score drip = 85
sus grade tea = ""

ready (score >= 90) {
    grade = "A"
} else ready (score >= 80) {
    grade = "B"
} else ready (score >= 70) {
    grade = "C"
} else {
    grade = "F"
}

vibez.spill("Grade: " + grade)
assert_eq_string(grade, "B")

# Nested if statements
sus a drip = 15
sus b drip = 20
sus max drip = 0

ready (a > b) {
    max = a
    ready (a > 100) {
        vibez.spill("Very large number")
    }
} else {
    max = b
    ready (b > 100) {
        vibez.spill("Very large number")
    } else {
        vibez.spill("Normal range")
    }
}

assert_eq_int(max, 20)

# Boolean conditions
sus is_even lit = (x % 2 == 0)
sus message tea = ""

ready (is_even) {
    message = "even"
} else {
    message = "odd"
}

vibez.spill("10 is " + message)
assert_eq_string(message, "even")

# Complex conditions
sus num drip = 12
sus category tea = ""

ready (num > 0 && num < 10) {
    category = "single digit positive"
} else ready (num >= 10 && num < 100) {
    category = "double digit"
} else ready (num >= 100) {
    category = "three digits or more"
} else {
    category = "zero or negative"
}

vibez.spill("12 is " + category)
assert_eq_string(category, "double digit")

print_test_summary()
