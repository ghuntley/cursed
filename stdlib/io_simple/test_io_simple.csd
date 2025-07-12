yeet "testz"
yeet "io_simple"

slay test_console_output() {
    test_start("Console Output")
    
    // Test basic output functions
    io_print("Hello, ")
    io_println("World!")
    io_print_int(42)
    io_print_bool(based)
    io_print_bool(cap)
    
    print_test_summary()
}

slay test_file_operations() {
    test_start("File Operations")
    
    // Test file operation placeholders
    assert_true(io_create_file("test.txt"))
    assert_false(io_file_exists("nonexistent.txt"))
    assert_true(io_write_file("test.txt", "content"))
    assert_eq_string(io_read_file("test.txt"), "file contents")
    assert_true(io_delete_file("test.txt"))
    
    print_test_summary()
}

slay test_directory_operations() {
    test_start("Directory Operations")
    
    // Test directory operation placeholders
    assert_true(io_create_directory("test_dir"))
    assert_false(io_directory_exists("nonexistent_dir"))
    sus files := io_list_directory(".")
    
    print_test_summary()
}

test_console_output()
test_file_operations()
test_directory_operations()
