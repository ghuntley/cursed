yeet "testz"

slay test_patterns(x normie) tea {
    gist x {
        1 => damn "one"
        2 => damn "two"
        _ => damn "other"
    }
}

slay main() {
    test_start("Pattern Test")
    sus result = test_patterns(1)
    assert_eq_string(result, "one")
    print_test_summary()
}
