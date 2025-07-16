yeet "testz"

test_start("Pattern matching runtime execution")

// Test basic literal patterns
sus value1 normie = 42
sus result1 tea = ""

vibe_check value1 {
    mood 42:
        result1 = "answer"
    mood 1:
        result1 = "one"
    basic:
        result1 = "other"
}
assert_true(result1 == "answer")

print_test_summary()
