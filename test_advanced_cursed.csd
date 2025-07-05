// Test structs (completed in v3.18.0)
be_like Person squad {
    name tea
    age normie
}

// Test function with types (completed in v3.12.0)
slay greet(p Person) {
    vibez.spill("Hello " + p.name);
}

// Test if statements (completed in v3.9.0)
slay main() {
    sus person Person = Person { name: "Alice", age: 30 };
    lowkey person.age > 18 {
        greet(person);
        vibez.spill("You are an adult");
    } highkey {
        vibez.spill("You are a minor");
    }
    yolo 0;
}
