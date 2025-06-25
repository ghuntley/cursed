fr fr Enhanced interface type assertion test

collab Stringer {
    toString() tea;
}

squad Person {
    name tea,
    age lit
}

slay (p Person) toString() tea {
    return p.name
}

slay main() {
    fr fr Create a Person that implements Stringer
    sus person = Person{name: "John", age: 30};
    
    fr fr Use person as a Stringer interface
    sus stringer Stringer = person;
    
    fr fr Print the string representation
    println(stringer.toString());
    
    fr fr Do a type assertion back to Person
    sus concretePerson, ok = stringer.(Person);
    
    if ok {
        println("Type assertion succeeded!");
        println(concretePerson.name);
        puts(concretePerson.age);
    } else {
        println("Type assertion failed!");
    }
    
    return 0;
}