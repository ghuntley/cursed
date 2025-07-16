yeet "testz"

test_start("Simple switch statement")

sus value normie = 42
sus result tea = ""

vibe_check value {
    mood 42:
        result = "answer"
    mood 1:
        result = "one"
    basic:
        result = "other"
}

assert_true(result == "answer")
print_test_summary()
