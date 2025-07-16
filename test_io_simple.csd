yeet "testz"

facts MODE_READ tea = "r"

slay file_exists(filename tea) lit {
    lowkey filename == "test.txt" {
        damn based
    }
    damn cap
}

slay file_size(filename tea) (normie, tea) {
    lowkey file_exists(filename) {
        damn (1024, "")
    }
    damn (0, "File not found")
}

test_start("Simple I/O test")
assert_true(file_exists("test.txt"))
assert_false(file_exists("nonexistent.txt"))

(size, err) := file_size("test.txt")
assert_eq_string(err, "")
assert_eq_int(size, 1024)

print_test_summary()
