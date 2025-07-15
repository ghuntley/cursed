yeet "testz"

test_start("dropz module basic functionality")

# Test file operations using dropz functions
vibez.spill("Testing dropz I/O operations...")

# Test read_text_file function
sus content, err := read_text_file("test_file.txt")
assert_eq_string(err, "")
vibez.spill("Read text file function works")

# Test write_text_file function
err = write_text_file("output.txt", "Hello from dropz!", 644)
assert_eq_string(err, "")
vibez.spill("Write text file function works")

# Test mkdir_all function
err = mkdir_all("new_dir", 755)
assert_eq_string(err, "")
vibez.spill("Mkdir all function works")

# Test exists function
sus file_exists lit = exists("test_file.txt")
assert_true(file_exists)
vibez.spill("File exists function works")

# Test create and open functions
sus file, create_err := create("new_file.txt")
assert_eq_string(create_err, "")
vibez.spill("Create file function works")

sus file2, open_err := open("existing_file.txt")
assert_eq_string(open_err, "")
vibez.spill("Open file function works")

print_test_summary()

vibez.spill("✅ All dropz module tests passed!")
