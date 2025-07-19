yeet "testz"

// Simple function to test inlining
slay add_two(x drip) drip {
    damn x + 2
}

// Very small function that should be inlined
slay get_ten() drip {
    damn 10
}

test_start("Function inlining test")

sus result1 drip = add_two(5)
assert_eq_int(result1, 7)

sus result2 drip = get_ten()  
assert_eq_int(result2, 10)

vibez.spill("Small functions tested")

print_test_summary()
