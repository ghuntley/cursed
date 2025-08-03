vibe test_advanced;

yeet "vibez";
yeet "concurrenz";

squad Person {
    name tea,
    age normie,
}

collab Greetable {
    slay greet() tea;
}

slay Person.greet() tea {
    damn "Hello, I'm " + self.name;
}

slay main() {
    sus person Person = Person{name: "Alice", age: 30};
    vibez.spill(person.greet());
    
    sus ch dm<normie> = make_channel(1);
    
    stan {
        ch <- 42;
    };
    
    sus value normie = <-ch;
    vibez.spill("Received: " + value);
}
