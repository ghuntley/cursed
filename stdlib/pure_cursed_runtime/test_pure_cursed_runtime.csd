yeet "testz"
yeet "pure_cursed_runtime"

test_start("pure CURSED runtime bridge tests")

fr fr Test basic I/O
assert_true(print("test message"))
assert_true(println("test message with newline"))

fr fr Test string operations
assert_eq_int(string_length("hello"), 5)
assert_eq_string(string_concat("hello", " world"), "hello world")

fr fr Test file operations
assert_true(file_write("/tmp/test.txt", "test content"))
assert_true(file_exists("/tmp/test.txt"))
assert_eq_string(file_read("/tmp/test.txt"), "test content")

fr fr Test time operations
sus start_time := time_now_ms()
sleep_ms(10)
sus end_time := time_now_ms()
assert_true(end_time > start_time)

fr fr Test crypto operations
sus hash := sha256("test data")
assert_true(string_length(hash) == 64) fr fr SHA256 produces 64 hex characters

sus random_data := random_bytes(16)
assert_true(string_length(random_data) == 16)

print_test_summary()
