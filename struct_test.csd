squad PersonName {
    spill name tea
    spill age drip
}

sus person PersonName = PersonName{name: "John", age: 25}
vibez.spill("Person name:", person.name)
vibez.spill("Person age:", person.age)

person.age = 26
vibez.spill("Updated age:", person.age)
