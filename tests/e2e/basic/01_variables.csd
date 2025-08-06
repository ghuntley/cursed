yeet "testz"

test_start("Variable Declaration Tests")

# Basic variable declarations
sus x drip = 42
sus y meal = 3.14
sus name tea = "CURSED"
sus flag lit = based
sus bad_flag lit = cringe

# Short variable declarations
a := 100
b := "short"
c := based

# Type assertions
sus small_num smol = 255
sus big_num thicc = 1000000
sus tiny_num lil = 10

# Test outputs
vibez.spill("x: " + str(x))
vibez.spill("y: " + str(y))
vibez.spill("name: " + name)
vibez.spill("flag: " + str(flag))
vibez.spill("a: " + str(a))
vibez.spill("small_num: " + str(small_num))

# Assertions
assert_eq_int(x, 42)
assert_eq_string(name, "CURSED")
assert_true(flag)
assert_false(bad_flag)
assert_eq_int(a, 100)

print_test_summary()
