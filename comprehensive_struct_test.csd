// Comprehensive struct (squad) testing for CURSED
// This tests all aspects of struct implementation

// Basic struct definition
squad Person {
    name tea,
    age normie,
    active lit
}

// Struct with methods
squad Calculator {
    value drip
}

// Calculator methods
slay (calc Calculator) add(x drip) Calculator {
    calc.value = calc.value + x
    damn calc
}

slay (calc Calculator) get_value() drip {
    damn calc.value
}

// Struct with visibility modifiers
squad BankAccount {
    balance drip,
    owner tea
}

// Main function to test struct operations
slay main() {
    // Basic struct instantiation
    sus person Person = Person{
        name: "John",
        age: 30,
        active: based
    }
    
    // Access struct fields
    vibez.spill("Name: " + person.name)
    vibez.spill("Age: " + person.age)
    vibez.spill("Active: " + person.active)
    
    // Modify struct fields
    person.age = 31
    person.active = cap
    
    // Test struct methods
    sus calc Calculator = Calculator{value: 10.0}
    calc = calc.add(5.0)
    sus result drip = calc.get_value()
    vibez.spill("Calculator result: " + result)
    
    // Test struct copying
    sus person2 Person = person
    person2.name = "Jane"
    
    vibez.spill("Original name: " + person.name)
    vibez.spill("Copy name: " + person2.name)
    
    // Test embedded structs
    squad Address {
        street tea,
        city tea
    }
    
    squad Employee {
        Person person,
        Address address,
        salary drip
    }
    
    sus emp Employee = Employee{
        person: Person{name: "Bob", age: 25, active: based},
        address: Address{street: "123 Main St", city: "NYC"},
        salary: 50000.0
    }
    
    vibez.spill("Employee: " + emp.person.name + " from " + emp.address.city)
}

main()
