fr fr Comprehensive struct system test for CURSED Zig implementation

fr fr Basic struct definition with squad keyword
squad Person {
    spill name tea
    spill age normie
    spill active lit
}

fr fr Nested struct definition
squad Address {
    spill street tea
    spill city tea
    spill zip normie
}

fr fr Complex struct with multiple field types
squad Employee {
    spill person Person
    spill address Address
    spill salary meal
    spill id normie
}

fr fr Generic struct
squad Container<T> {
    spill value T
    spill size normie
}

fr fr Struct with methods (using receiver syntax)
squad Calculator {
    spill value meal
}

slay (calc Calculator) add(x meal) Calculator {
    calc.value = calc.value + x
    damn calc
}

slay (calc Calculator) getValue() meal {
    damn calc.value
}

fr fr Main test function
slay test_basic_structs() {
    vibez.spill("=== Testing Basic Struct Operations ===")
    
    fr fr Test 1: Basic struct instantiation
    sus person Person = Person{
        name: "John Doe",
        age: 30,
        active: based
    }
    
    vibez.spill("Created person:", person.name)
    vibez.spill("Person age:", person.age)
    vibez.spill("Person active:", person.active)
    
    fr fr Test 2: Field access and modification
    person.age = 31
    person.active = cringe
    
    vibez.spill("Updated person age:", person.age)
    vibez.spill("Updated person active:", person.active)
    
    fr fr Test 3: Nested struct instantiation
    sus address Address = Address{
        street: "123 Main St",
        city: "New York", 
        zip: 10001
    }
    
    sus employee Employee = Employee{
        person: person,
        address: address,
        salary: 75000.5,
        id: 12345
    }
    
    vibez.spill("Employee name:", employee.person.name)
    vibez.spill("Employee city:", employee.address.city)
    vibez.spill("Employee salary:", employee.salary)
    
    fr fr Test 4: Struct copying
    sus person2 Person = person
    person2.name = "Jane Smith"
    
    vibez.spill("Original person:", person.name)
    vibez.spill("Copied person:", person2.name)
}

slay test_struct_methods() {
    vibez.spill("=== Testing Struct Methods ===")
    
    sus calc Calculator = Calculator{value: 10.0}
    calc = calc.add(5.0)
    calc = calc.add(3.0)
    
    sus result meal = calc.getValue()
    vibez.spill("Calculator result:", result)
}

slay test_generic_structs() {
    vibez.spill("=== Testing Generic Structs ===")
    
    sus int_container Container<normie> = Container<normie>{
        value: 42,
        size: 1
    }
    
    sus string_container Container<tea> = Container<tea>{
        value: "Hello",
        size: 5
    }
    
    vibez.spill("Int container value:", int_container.value)
    vibez.spill("String container value:", string_container.value)
}

slay test_struct_arrays() {
    vibez.spill("=== Testing Struct Arrays ===")
    
    sus people []Person = [
        Person{name: "Alice", age: 25, active: based},
        Person{name: "Bob", age: 30, active: cringe},
        Person{name: "Charlie", age: 35, active: based}
    ]
    
    bestie i := 0; i < people.len(); i = i + 1 {
        vibez.spill("Person", i, ":", people[i].name, "age", people[i].age)
    }
}

slay main() {
    vibez.spill("Starting comprehensive struct system test...")
    
    test_basic_structs()
    test_struct_methods() 
    test_generic_structs()
    test_struct_arrays()
    
    vibez.spill("Struct system test completed!")
}

main()
