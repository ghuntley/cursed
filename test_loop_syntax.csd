yeet "testz"

# Test correct loop syntax
test_start("Loop syntax test")

# This should work (correct syntax):
bestie i := 0; i < 5; i++ {
    vibez.spill("Iteration: " + i.(tea))
}

# This should fail (incorrect syntax from stdlib):
# stan i < 5 {
#     vibez.spill("Wrong syntax")
# }

assert_true(based)
print_test_summary()
