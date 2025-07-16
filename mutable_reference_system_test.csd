// Test mutable reference handling in CURSED language
yeet "testz"

test_start("Mutable Reference System Test")

// Test 1: Basic mutable reference
sus x normie = 42
sus y &normie = &x  // mutable reference to x
*y = 100           // dereference and assign
assert_eq_int(x, 100)

// Test 2: Immutable reference  
sus z normie = 25
sus w &normie = &z  // immutable reference to z
// *w = 50  // This should be a compile error
assert_eq_int(z, 25)

// Test 3: Reference to reference
sus a normie = 10
sus b &normie = &a
sus c &&normie = &b
assert_eq_int(**c, 10)

// Test 4: Borrowing rules test
sus val normie = 123
sus ref1 &normie = &val
sus ref2 &normie = &val  // Multiple immutable borrows OK
assert_eq_int(*ref1, 123)
assert_eq_int(*ref2, 123)

print_test_summary()
