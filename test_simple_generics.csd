yeet "testz"

test_start("Simple Generics Test")

fr fr Basic generic struct
be_like Box[T] squad {
    value T
}

fr fr Basic generic function
slay identity[T](x T) T {
    damn x
}

fr fr Test instantiation
slay test_basic() {
    sus int_box = Box[normie]{value: 42}
    assert_eq_int(int_box.value, 42)
    
    sus result = identity[normie](100)
    assert_eq_int(result, 100)
    
    sus str_result = identity[tea]("hello")
    assert_eq_string(str_result, "hello")
}

test_basic()
print_test_summary()
