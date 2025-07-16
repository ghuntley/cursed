yeet "testz"

slay file_exists(filename tea) lit {
    lowkey filename == "test.txt" {
        damn based
    }
    damn cap
}

slay test_basic() {
    test_start("Basic I/O test")
    assert_true(file_exists("test.txt"))
    assert_false(file_exists("nonexistent.txt"))
    print_test_summary()
}

test_basic()
