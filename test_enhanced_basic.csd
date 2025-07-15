yeet "testz"

testz.set_verbose_mode(based)
testz.set_test_suite("Basic Enhanced Testz Test")

testz.test_start("Basic functionality test")
testz.assert_true(based)
testz.assert_false(cap)
testz.assert_eq_string("hello", "hello")
testz.assert_eq_int(42, 42)
testz.test_end()

testz.test_start("Enhanced assertions test")
testz.assert_ne_int(10, 20)
testz.assert_gt_int(100, 50)
testz.assert_contains("hello world", "world")
testz.assert_starts_with("hello", "hel")
testz.test_end()

testz.print_test_summary()
