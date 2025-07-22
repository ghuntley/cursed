fr fr Simple test for memory module improvements
yeet "testz"
yeet "memory"

test_start("memory allocation test")
sus addr, err = memory_allocate(1024)
assert_true(addr > 0)
assert_eq_string(err.(tea), "")
print_test_summary()

test_start("memory deallocation test")
sus success, dealloc_err = memory_deallocate(addr)
assert_true(success)
assert_eq_string(dealloc_err.(tea), "")
print_test_summary()

vibez.spill("✅ Memory module enhanced successfully!")
