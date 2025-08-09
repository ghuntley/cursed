yeet "testz"
yeet "ioz_enhanced"

fr fr Test the enhanced I/O module

test_start("Basic Output Operations")
assert_true(print_line("Test message"))
assert_true(print_with_prefix("INFO", "This is a test"))
assert_true(print_header("Test Section"))

test_start("Path Operations")
assert_eq_string(join_path_two("home", "user"), "home/user")
assert_eq_string(join_path_three("home", "user", "docs"), "home/user/docs")
assert_eq_string(get_filename_from_path("/home/user/file.txt"), "file.txt")
assert_eq_string(get_directory_from_path("/home/user/file.txt"), "/home/user")

test_start("File Extension Operations")
assert_eq_string(get_file_extension("main.csd"), "csd")
assert_eq_string(get_file_extension("image.png"), "png")
assert_true(has_extension("test.json", "json"))
assert_false(has_extension("readme.md", "txt"))

test_start("File Existence Checks")
assert_true(file_exists_check("/etc/passwd"))
assert_false(file_exists_check("nonexistent.file"))
assert_true(is_directory_check("/home"))
assert_false(is_directory_check("/etc/passwd"))

test_start("File Type Detection")
assert_true(is_text_file("document.txt"))
assert_true(is_image_file("photo.png"))
assert_true(is_executable_file("script.sh"))
assert_false(is_text_file("image.png"))

test_start("File Size Estimation")
assert_eq_int(get_file_size_estimate("config.toml"), 256)
assert_true(is_small_file("test.json"))
assert_true(is_large_file("video.mp4"))

test_start("File Operations Simulation")
assert_true(create_file_simulation("new_file.txt"))
assert_true(copy_file_simulation("src/main.csd", "backup.csd"))
assert_true(write_file_simulation("output.txt", "Hello, world!"))
assert_true(append_file_simulation("log.txt", "New entry"))

test_start("Directory Operations Simulation")
assert_true(create_directory_simulation("new_folder"))
assert_eq_int(list_directory_simulation("src"), 3)
assert_true(remove_directory_simulation("temp"))

test_start("File Content Operations")
sus content tea = read_file_simulation("README.md")
assert_true(content != "")
assert_eq_string(read_file_simulation("nonexistent.txt"), "")

test_start("Path Utilities")
assert_eq_string(normalize_path("./file.txt"), "file.txt")
assert_true(is_absolute_path("/home/user/file.txt"))
assert_true(is_relative_path("src/main.csd"))
assert_eq_string(get_current_directory(), ".")
assert_eq_string(get_parent_directory(), "..")

print_test_summary()
