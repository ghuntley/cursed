yeet "testz"
yeet "exec_vibez"

test_start("Basic test")
sus result := exec_command("echo hello")
assert_true(result.success)
test_end()
print_test_summary()
