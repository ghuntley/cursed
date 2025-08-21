// Simple type system test to validate our enhancements

yeet "testz"

test_start("Basic Type System Enhancements")

// Test 1: Basic struct validation
squad Person {
    name tea,
    age drip,
    active lit
}

// This should work
sus person Person = Person{
    .name = "Alice",
    .age = 30,
    .active = based
}

vibez.spill("Person created:", person.name, "age:", person.age)

// Test 2: Simple generic function
slay identity[T](value T) T {
    damn value
}

sus int_result drip = identity[drip](42)
sus str_result tea = identity[tea]("hello")

vibez.spill("Generic identity results:", int_result, str_result)

test_passed("Basic type system enhancements")

print_test_summary()
