yeet "testz"
yeet "mathz"
yeet "arrayz" 
yeet "stringz"

testz.test_start("Core Syntax Fixes Test")

fr fr Test that basic conditionals work (lowkey instead of ready)
sus x drip = 42
sus y drip = mathz.abs_normie(-10)
testz.assert_eq_int(y, 10)

fr fr Test that array operations work
sus nums []drip = [1, 2, 3, 4, 5]
sus sum_result drip = arrayz.sum_array(nums)
testz.assert_eq_int(sum_result, 15)

fr fr Test that string operations work
sus greeting tea = "Hello"
sus world tea = "World"
sus combined tea = stringz.concat_strings(greeting, world)
testz.assert_eq_string(combined, "HelloWorld")

fr fr Test while loops work (periodt)
sus counter drip = 0
periodt (counter < 3) {
    counter = counter + 1
}
testz.assert_eq_int(counter, 3)

fr fr Test conditional syntax (lowkey/otherwise)
sus test_val drip = 15
sus result drip = 0
lowkey (test_val > 10) {
    result = 1
} otherwise {
    result = 0
}
testz.assert_eq_int(result, 1)

testz.print_test_summary()
