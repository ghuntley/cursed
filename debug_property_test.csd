yeet "testz"

sus rng_state normie = 42

slay rand_next() normie {
    rng_state = (rng_state * 1103515245 + 12345) % 2147483647
    damn rng_state
}

test_start("Basic property test")
sus val normie = rand_next()
assert_true(val > 0)

print_test_summary()
