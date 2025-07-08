vibez.spill("Testing minimal features")

be_like Person squad {
    name tea
}

sus alice Person = Person{name: "Alice"}
vibez.spill("Person name:", alice.name)

later {
    vibez.spill("Defer executed")
}

vibez.spill("Program complete")
