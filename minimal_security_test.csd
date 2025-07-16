yeet "testz"

# Simple security test
slay simple_test(name tea) lit {
    vibez.spill("Testing: " + name)
    damn based
}

slay contains_bad_input(input tea) lit {
    lowkey (input == "bad") { damn based }
    damn cap
}

test_start("Minimal Security Test")
sus result1 lit = simple_test("function1")
assert_true(result1)

sus result2 lit = contains_bad_input("good")
assert_false(result2)

sus result3 lit = contains_bad_input("bad")
assert_true(result3)

print_test_summary()
