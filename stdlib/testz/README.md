# TESTZ Module - Testing Framework

Essential testing functions for CURSED programs.

## Functions

### Test Organization
- `test_start(name tea)` - Start a new test
- `print_test_summary()` - Print final test results

### Assertions
- `assert_true(condition lit)` - Assert condition is true
- `assert_false(condition lit)` - Assert condition is false
- `assert_eq_int(actual drip, expected drip)` - Assert integer equality
- `assert_eq_string(actual tea, expected tea)` - Assert string equality

### Test Information
- `get_test_count() drip` - Get total test count
- `get_pass_count() drip` - Get passed test count
- `get_fail_count() drip` - Get failed test count
- `all_tests_passed() lit` - Check if all tests passed

## Usage

```cursed
yeet "testz"

test_start("basic math test")
assert_eq_int(2 + 2, 4)
assert_true(5 > 3)

test_start("string test")
assert_eq_string("hello", "hello")
assert_false(is_empty_string("test"))

print_test_summary()
```

## Test Output

The framework provides colored output:
- ✅ PASS messages for successful assertions
- ❌ FAIL messages for failed assertions
- 📊 Summary with pass/fail counts
- 🎉 Success message if all tests pass

## Example Test File

```cursed
yeet "testz"
yeet "mathz"

test_start("abs_normie function")
sus result drip = abs_normie(-5)
assert_eq_int(result, 5)

test_start("max_normie function")
sus max_result drip = max_normie(10, 5)
assert_eq_int(max_result, 10)

print_test_summary()
```
