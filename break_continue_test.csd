yeet "testz"

test_start("Break and Continue Test")

# Test break in for loop
sus counter drip = 0
bestie i := 0; i < 10; i++ {
    lowkey i == 5 {
        ghosted
    }
    counter++
}
assert_eq_int(counter, 5)
vibez.spill("Break test passed")

# Test continue in for loop  
sus skip_counter drip = 0
bestie i := 0; i < 10; i++ {
    lowkey i % 2 == 0 {
        simp
    }
    skip_counter++
}
assert_eq_int(skip_counter, 5)
vibez.spill("Continue test passed")

# Test break in while loop
sus while_counter drip = 0
periodt while_counter < 100 {
    while_counter++
    lowkey while_counter >= 7 {
        ghosted
    }
}
assert_eq_int(while_counter, 7)
vibez.spill("While break test passed")

# Test continue in while loop
sus while_skip_counter drip = 0
sus while_total drip = 0
periodt while_total < 10 {
    while_total++
    lowkey while_total % 2 == 0 {
        simp
    }
    while_skip_counter++
}
assert_eq_int(while_skip_counter, 5)
vibez.spill("While continue test passed")

print_test_summary()
