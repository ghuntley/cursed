yeet "testz"

fr fr Test basic enhanced stdlib functionality

test_start("Basic Enhanced Math Functions")

fr fr Test power function  
sus pow_result drip = 1
sus base drip = 2
sus exp drip = 3
sus i drip = 0
bestie (i < exp) {
    pow_result = pow_result * base
    i = i + 1
}
assert_eq_int(pow_result, 8)

fr fr Test absolute value
slay abs_test(x drip) drip {
    ready (x < 0) {
        damn -x
    }
    damn x
}
assert_eq_int(abs_test(-5), 5)
assert_eq_int(abs_test(3), 3)

test_start("Enhanced String Operations")

fr fr Test string concatenation
slay concat_test(a tea, b tea) tea {
    damn a + b
}
assert_eq_string(concat_test("hello", "world"), "helloworld")

fr fr Test string repetition
slay repeat_test(s tea, times drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < times) {
        result = result + s
        i = i + 1
    }
    damn result
}
assert_eq_string(repeat_test("x", 3), "xxx")

test_start("Basic Array Operations")
sus test_array []drip = [1, 2, 3, 4, 5]

fr fr Test array sum
slay array_sum_test(arr []drip) drip {
    yeet "arrayz"
    sus total drip = 0
    sus length drip = len(arr)
    sus i drip = 0
    bestie (i < length) {
        total = total + arr[i]
        i = i + 1
    }
    damn total
}
assert_eq_int(array_sum_test(test_array), 15)

fr fr Test array find
slay array_find_test(arr []drip, value drip) drip {
    yeet "arrayz"
    sus length drip = len(arr)
    sus i drip = 0
    bestie (i < length) {
        ready (arr[i] == value) {
            damn i
        }
        i = i + 1
    }
    damn -1
}
assert_eq_int(array_find_test(test_array, 3), 2)

print_test_summary()
