yeet "testz"

fr fr Simple test for new stdlib modules
slay test_new_modules() {
    test_start("New stdlib modules")
    
    fr fr Test basic functionality
    assert_true(based)
    assert_false(cap)
    assert_eq_int(1, 1)
    assert_eq_string("test", "test")
}

slay main() {
    test_new_modules()
    print_test_summary()
}

main()
