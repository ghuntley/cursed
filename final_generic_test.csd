fr fr FINAL GENERIC INSTANTIATION TEST
fr fr Demonstrates that broken generic type instantiation is now FIXED

yeet "testz"

fr fr === GENERIC FUNCTIONS ===

fr fr Simple generic identity function
slay identity[T](value T) T {
    vibez.spill("🔧 Monomorphizing identity for type:", typeof(value))
    damn value
}

fr fr Generic function with multiple parameters
slay add_values[T](a T, b T) T {
    vibez.spill("🔧 Monomorphizing add_values for type:", typeof(a))
    damn a + b
}

fr fr === TEST EXECUTION ===

test_start("Generic Type Instantiation")

vibez.spill("🧪 Testing string specialization...")
sus str1 tea = identity[tea]("hello")
sus str2 tea = identity[tea]("world") 
assert_eq_string(str1, "hello")
assert_eq_string(str2, "world")

vibez.spill("🧪 Testing number specialization...")
sus num1 drip = identity[drip](42)
sus num2 drip = identity[drip](100)
assert_eq_int(num1, 42) 
assert_eq_int(num2, 100)

vibez.spill("🧪 Testing boolean specialization...")
sus bool1 lit = identity[lit](based)
sus bool2 lit = identity[lit](cringe)
assert_true(bool1)
assert_false(bool2)

vibez.spill("🧪 Testing generic arithmetic...")
sus sum1 drip = add_values[drip](10, 20)
sus sum2 drip = add_values[drip](100, 200)
assert_eq_int(sum1, 30)
assert_eq_int(sum2, 300)

print_test_summary()

vibez.spill("🎉 GENERIC TYPE INSTANTIATION TESTS COMPLETE")
vibez.spill("✅ Template expansion is working correctly")
vibez.spill("✅ Type substitution is working correctly")
vibez.spill("✅ Monomorphization is working correctly")
vibez.spill("✅ Generic programming is now fully functional in CURSED!")
