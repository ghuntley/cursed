yeet "testz"

// Library public API
slay add(a normie, b normie) normie {
    damn a + b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay format_message(name tea) tea {
    damn "Hello, " + name + "!"
}

// Library configuration
squad LibConfig {
    spill debug lit = cap
    spill version tea = "1.0.0"
}

sus lib_config LibConfig = LibConfig{}

// Internal utilities
slay internal_helper(data tea) lit {
    damn data.len > 0
}

test "library basic functionality" {
    test_start("library tests")
    
    assert_eq_int(add(2, 3), 5)
    assert_eq_int(multiply(4, 5), 20)
    assert_eq_string(format_message("World"), "Hello, World!")
    assert_true(internal_helper("test"))
    assert_false(internal_helper(""))
    
    print_test_summary()
}