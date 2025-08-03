fr fr Comprehensive struct runtime implementation test
fr fr Testing all aspects of struct lifecycle

yeet "testz"

fr fr Define basic struct 
be_like Point squad {
    x normie
    y normie
}

fr fr Define complex struct with multiple types
be_like Person squad {
    name tea
    age normie
    active lit
    height snack
}

fr fr Define nested struct
be_like Company squad {
    name tea
    owner Person
    employee_count normie
}

slay test_basic_struct_instantiation() {
    test_start("Basic struct instantiation")
    
    fr fr Test struct literal creation
    sus p Point = Point { x: 10, y: 20 }
    
    fr fr Test field access
    assert_eq_int(p.x, 10)
    assert_eq_int(p.y, 20)
}

slay test_complex_struct_operations() {
    test_start("Complex struct operations")
    
    fr fr Create person with multiple field types
    sus alice Person = Person { 
        name: "Alice Johnson", 
        age: 28, 
        active: based,
        height: 5.7
    }
    
    fr fr Test field access for different types
    assert_eq_string(alice.name, "Alice Johnson")
    assert_eq_int(alice.age, 28)
    assert_true(alice.active)
}

slay test_nested_struct_operations() {
    test_start("Nested struct operations")
    
    fr fr Create owner
    sus owner Person = Person {
        name: "Bob Smith",
        age: 35,
        active: based,
        height: 6.0
    }
    
    fr fr Create company with nested struct
    sus company Company = Company {
        name: "Tech Corp",
        owner: owner,
        employee_count: 150
    }
    
    fr fr Test nested field access
    assert_eq_string(company.name, "Tech Corp")
    assert_eq_string(company.owner.name, "Bob Smith")
    assert_eq_int(company.owner.age, 35)
    assert_eq_int(company.employee_count, 150)
}

slay test_struct_field_modification() {
    test_start("Struct field modification")
    
    fr fr Create mutable struct
    sus p Point = Point { x: 5, y: 10 }
    
    fr fr Modify fields
    p.x = 15
    p.y = 25
    
    fr fr Test modifications
    assert_eq_int(p.x, 15)
    assert_eq_int(p.y, 25)
}

slay test_struct_memory_management() {
    test_start("Struct memory management")
    
    fr fr Test multiple struct instances
    sus p1 Point = Point { x: 1, y: 2 }
    sus p2 Point = Point { x: 3, y: 4 }
    sus p3 Point = Point { x: 5, y: 6 }
    
    fr fr Verify independent memory
    assert_eq_int(p1.x, 1)
    assert_eq_int(p2.x, 3)
    assert_eq_int(p3.x, 5)
    
    fr fr Modify one instance
    p2.x = 100
    
    fr fr Verify others unaffected
    assert_eq_int(p1.x, 1)
    assert_eq_int(p2.x, 100)
    assert_eq_int(p3.x, 5)
}

slay main() {
    test_basic_struct_instantiation()
    test_complex_struct_operations()
    test_nested_struct_operations()
    test_struct_field_modification()
    test_struct_memory_management()
    
    print_test_summary()
}
