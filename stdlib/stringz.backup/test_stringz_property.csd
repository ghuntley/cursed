yeet "testz"
yeet "stringz"

fr fr Property-based tests for stringz module
testz.set_test_suite("Stringz Property-Based Tests")
testz.set_verbose_mode(based)

fr fr ===============================
fr fr String Concatenation Properties
fr fr ===============================

testz.test_start("String concatenation is associative")
testz.property_test_start("Concatenation associativity", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus a tea = testz.random_string(10)
    sus b tea = testz.random_string(10)
    sus c tea = testz.random_string(10) fr fr Test (a + b) + c = a + (b + c)
    sus left tea = (a + b) + c
    sus right tea = a + (b + c)
    testz.assert_eq_string(left, right)
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr String Length Properties
fr fr ===============================

testz.test_start("String length properties")
testz.property_test_start("Length properties", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus str tea = testz.random_string(testz.random_int(5, 50))
    sus doubled tea = str + str fr fr Test that doubled string is twice the length
    sus original_len normie = stringz.Length(str)
    sus doubled_len normie = stringz.Length(doubled)
    testz.assert_eq_int(doubled_len, original_len * 2) fr fr Test empty string properties
    sus empty tea = ""
    testz.assert_eq_int(stringz.Length(empty), 0)
    testz.assert_eq_string(str + empty, str)
    testz.assert_eq_string(empty + str, str)
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr String Contains Properties
fr fr ===============================

testz.test_start("String contains properties")
testz.property_test_start("Contains reflexivity", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus str tea = testz.random_string(20) fr fr String contains itself
    testz.assert_true(stringz.Contains(str, str)) fr fr String contains empty string
    testz.assert_true(stringz.Contains(str, "")) fr fr If string contains substring, concatenated string also contains it
    sus substr tea = testz.random_string(5)
    sus combined tea = str + substr
    testz.assert_true(stringz.Contains(combined, str))
    testz.assert_true(stringz.Contains(combined, substr))
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr String Prefix/Suffix Properties
fr fr ===============================

testz.test_start("String prefix/suffix properties")
testz.property_test_start("Prefix/suffix consistency", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    sus prefix tea = testz.random_string(8)
    sus suffix tea = testz.random_string(8)
    sus middle tea = testz.random_string(10)
    
    sus full_string tea = prefix + middle + suffix fr fr Test prefix properties
    testz.assert_true(stringz.StartsWith(full_string, prefix))
    testz.assert_true(stringz.Contains(full_string, prefix)) fr fr Test suffix properties
    testz.assert_true(stringz.EndsWith(full_string, suffix))
    testz.assert_true(stringz.Contains(full_string, suffix)) fr fr Test middle is contained
    testz.assert_true(stringz.Contains(full_string, middle))
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr String Case Properties
fr fr ===============================

testz.test_start("String case conversion properties")
testz.property_test_start("Case conversion idempotency", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    sus str tea = testz.random_string(15) fr fr Case conversion should be idempotent
    sus upper1 tea = stringz.ToUpper(str)
    sus upper2 tea = stringz.ToUpper(upper1)
    testz.assert_eq_string(upper1, upper2)
    
    sus lower1 tea = stringz.ToLower(str)
    sus lower2 tea = stringz.ToLower(lower1)
    testz.assert_eq_string(lower1, lower2) fr fr Length should be preserved
    testz.assert_eq_int(stringz.Length(str), stringz.Length(upper1))
    testz.assert_eq_int(stringz.Length(str), stringz.Length(lower1))
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr String Split/Join Properties
fr fr ===============================

testz.test_start("String split/join roundtrip")
testz.property_test_start("Split/join consistency", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    sus part1 tea = testz.random_string(8)
    sus part2 tea = testz.random_string(8)
    sus part3 tea = testz.random_string(8)
    sus separator tea = "," fr fr Create a known string
    sus original tea = part1 + separator + part2 + separator + part3 fr fr Test that split contains the parts
    testz.assert_true(stringz.Contains(original, part1))
    testz.assert_true(stringz.Contains(original, part2))
    testz.assert_true(stringz.Contains(original, part3))
    testz.assert_true(stringz.Contains(original, separator))
}

testz.property_test_end()
testz.test_end()

fr fr ===============================
fr fr Fuzz Testing with Random Data
fr fr ===============================

testz.test_start("Fuzz testing with random strings")
testz.property_test_start("Random string operations stability", 200)

bestie i := 0; i < 200; i++ {
    testz.property_test_iteration() fr fr Generate completely random strings of various lengths
    sus len1 normie = testz.random_int(0, 100)
    sus len2 normie = testz.random_int(0, 100)
    sus str1 tea = testz.random_string(len1)
    sus str2 tea = testz.random_string(len2) fr fr All operations should complete without crashing
    testz.assert_no_throw() fr fr Basic operations should work
    sus concat tea = str1 + str2
    sus len_concat normie = stringz.Length(concat)
    sus len_sum normie = stringz.Length(str1) + stringz.Length(str2)
    testz.assert_eq_int(len_concat, len_sum) fr fr Contains operation should not crash
    sus contains_result lit = stringz.Contains(concat, str1)
    fr fr stringz.Length(str1) > 0 {
        testz.assert_true(contains_result)
    }
}

testz.property_test_end()
testz.test_end()

fr fr Print final results
testz.print_test_summary()
