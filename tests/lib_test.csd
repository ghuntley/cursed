yeet "testz"
yeet "../src/lib"

test_start("greet function test")

sus result tea = greet("World")
assert_eq_string(result, "Hello, World!")

print_test_summary()
