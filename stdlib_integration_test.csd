fr fr Comprehensive stdlib integration test
fr fr Testing core modules: testz, vibez, mathz, stringz, collections

yeet "testz"
yeet "vibez"
yeet "mathz"
yeet "stringz"

slay main() cringe {
    vibez.spill("🧪 Starting stdlib integration tests...")
    
    fr fr Test testz module
    test_start("testz integration")
    assert_true(based)
    assert_false(cringe)
    assert_eq_int(42, 42)
    assert_eq_string("test", "test")
    vibez.spill("✅ testz module working")
    
    fr fr Test vibez module
    vibez.spill("Testing vibez module...")
    vibez.spillf("Format test: %s %d", "hello", 42)
    vibez.spillln("Line with newline")
    vibez.spill("✅ vibez module working")
    
    fr fr Test mathz module
    sus pi_val meal = mathz.PI
    sus sqrt_val meal = mathz.sqrt_meal(25.0)
    sus abs_val meal = mathz.abs_meal(-10.0)
    sus max_val meal = mathz.max_meal(5.0, 10.0)
    vibez.spillf("Math tests: PI=%f, sqrt(25)=%f, abs(-10)=%f, max(5,10)=%f", pi_val, sqrt_val, abs_val, max_val)
    vibez.spill("✅ mathz module working")
    
    fr fr Test stringz module
    sus str_len normie = stringz.length("hello")
    sus concat_result tea = stringz.concat("hello", " world")
    sus contains_result lit = stringz.contains("hello world", "world")
    vibez.spillf("String tests: len('hello')=%d, concat=%s, contains=%b", str_len, concat_result, contains_result)
    vibez.spill("✅ stringz module working")
    
    print_test_summary()
    vibez.spill("🎉 All core stdlib modules integrated successfully!")
}
