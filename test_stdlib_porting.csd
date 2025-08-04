yeet "testz"
yeet "string"
yeet "collections"
yeet "io"

fr fr Test current stdlib modules to see what's working vs needs porting

test_start("String module test")
assert_eq_int(string_length("hello"), 5)
assert_eq_string(string_to_upper("hello"), "HELLO")
assert_eq_string(string_concat("hello", " world"), "hello world")

test_start("Collections module test")
sus vec [extra] = Vec_new()
vec = Vec_push(vec, 42)
assert_eq_int(Vec_len(vec), 1)

sus map tea = Map_new()
map = Map_insert(map, "name", "John")
assert_eq_string(Map_get(map, "name"), "John")

test_start("IO module test")
assert_eq_lit(file_exists("test.txt"), based)
(content, err) := read_file("test.txt")
assert_eq_string(err, "")
assert_true(len(content) > 0)

print_test_summary()
