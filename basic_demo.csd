vibez.spill("Basic test")

be_like Person squad {
    name tea
}

slay (p Person) greet() tea {
    damn "Hello, " + p.name
}

sus alice Person = Person{name: "Alice"}
sus greeting = alice.greet()
vibez.spill(greeting)
