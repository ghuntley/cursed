yeet "testz"

testz.test_start("Integer equality test")
testz.assert_eq_int(42, 42)
testz.assert_eq_int(5 + 5, 10)

testz.test_start("String equality test")
testz.assert_eq_string("hello", "hello")
testz.assert_eq_string("world", "world")

testz.test_start("Boolean assertion test")
testz.assert_true(based)
testz.assert_false(cap)
testz.assert_true(5 > 3)
testz.assert_false(10 < 5)

testz.print_test_summary()
