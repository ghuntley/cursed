collab Greeter {
    slay greet() tea
}

collab Speaker {
    slay speak(message tea) tea
}

squad Person {
    spill name tea
}

squad Robot {
    spill id drip
}

vibez.spill("✅ Interface definitions parsed")
vibez.spill("✅ Struct definitions parsed")

sus alice Person = Person{name: "Alice"}
sus bot Robot = Robot{id: 42}

vibez.spill("✅ Struct instances created")
vibez.spill("Person:", alice.name)
vibez.spill("Robot:", bot.id)
