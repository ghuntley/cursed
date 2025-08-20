fr fr Test generic functions
yeet "testz"

fr fr Define a simple generic identity function
slay identity[T](value T) T {
    vibez.spill("Inside identity function with:", value)
    damn value
}

fr fr Define a generic pair function  
slay make_pair[T, U](first T, second U) (T, U) {
    damn (first, second)
}

fr fr Define a generic container struct
squad Container[T] {
    value T
}

fr fr Test the generic functions
test_start("Generic Function Tests")

vibez.spill("Testing generic identity function...")

fr fr Test 1: String identity
sus str_result tea = identity[tea]("hello")
vibez.spill("String result:", str_result)
assert_eq_string(str_result, "hello")

fr fr Test 2: Number identity  
sus num_result drip = identity[drip](42)
vibez.spill("Number result:", num_result) 
assert_eq_int(num_result, 42)

fr fr Test 3: Generic struct
vibez.spill("Testing generic struct...")
sus container Container[drip] = Container { value: 123 }
vibez.spill("Container value:", container.value)
assert_eq_int(container.value, 123)

print_test_summary()

vibez.spill("🎉 GENERIC FUNCTION TESTS COMPLETE")
