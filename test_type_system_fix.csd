fr fr Test the type system mutability tracking fix
yeet "testz"

test_start("Type System Mutability Tracking")

fr fr Test mutable variable
sus mut x normie = 42
x = 100
assert_eq_int(x, 100)

fr fr Test immutable constant  
facts y normie = 5
fr fr This should fail in the type checker:
fr fr y = 10  

print_test_summary()
