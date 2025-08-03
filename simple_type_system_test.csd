fr fr Simple Type System Test for CURSED
yeet "testz"

fr fr Basic struct definition
be_like Person squad {
    name tea
    age drip
}

fr fr Test function for struct creation
slay test_basic_struct() {
    test_start("Basic Struct Test")
    
    fr fr Create person instance
    sus person drip = Person{
        name: "Alice",
        age: 30
    }
    
    fr fr Test field access
    assert_eq_string(person.name, "Alice")
    assert_eq_int(person.age, 30)
    
    test_end()
}

fr fr Main test function
slay main_character() {
    vibez.spill("Starting Simple Type System Test")
    
    test_basic_struct()
    
    print_test_summary()
}
