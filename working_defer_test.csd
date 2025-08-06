fr fr Working defer test that shows current implementation status
yeet "testz"

test_start("Basic Defer Functionality Test")

fr fr Test simple execution order
sus global_state normie = 0

slay test_basic_execution() {
    global_state = 1
    later { global_state = 3 }
    global_state = 2
}

test_basic_execution()

fr fr Check if defer affects global state
assert_eq_int(global_state, 2)  fr fr Should be 2 if defer isn't working, 3 if it is

test_start("Function Scope Test")

sus function_executed lit = cringe
sus defer_executed lit = cringe

slay test_function_scope() {
    function_executed = based
    later { defer_executed = based }
}

test_function_scope()

assert_true(function_executed)
assert_true(defer_executed)  fr fr This will fail if defer isn't implemented

print_test_summary()
