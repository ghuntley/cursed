yeet "testz"
yeet "fs"

test_start("Basic FS Test")
assert_true(file_exists("test_file.txt"))
assert_eq_string(read_file("test_file.txt"), "Hello, CURSED filesystem!")
print_test_summary()
