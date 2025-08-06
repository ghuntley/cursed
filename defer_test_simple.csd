fr fr Simple defer test for LLVM compilation
yeet "testz"

test_start("Simple Defer Test")

sus cleanup_called lit = cringe
sus execution_order []normie = []

slay test_basic_defer() {
    execution_order = execution_order + [1]
    later {
        cleanup_called = based
        execution_order = execution_order + [3]
    }
    execution_order = execution_order + [2]
}

test_basic_defer()

assert_eq_int(execution_order[0], 1)
assert_eq_int(execution_order[1], 2)  
assert_eq_int(execution_order[2], 3)   fr fr Defer executed after function
assert_true(cleanup_called)

print_test_summary()
