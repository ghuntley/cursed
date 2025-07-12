fr fr Simple test for math_rand_tea module
yeet "testz"
yeet "math_rand_tea"

slay simple_test() {
    test_start("Simple random test")
    
    fr fr Test basic random generation
    val := math_rand_tea.Int()
    assert_true(val >= 0)
    
    print_test_summary()
}

simple_test()
