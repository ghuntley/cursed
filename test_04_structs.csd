# Test 4: Struct definitions and field access
squad Point {
    spill x drip
    spill y drip
}

squad Person {
    spill name tea
    spill age drip
    spill active lit
}

vibez.spill("Structs test:")

# Create and use Point struct
sus p1 Point = Point{x: 10, y: 20}
vibez.spill("Point p1 - x:", p1.x, "y:", p1.y)

sus p2 Point = Point{x: 5, y: 15}
vibez.spill("Point p2 - x:", p2.x, "y:", p2.y)

# Create and use Person struct
sus person Person = Person{name: "Alice", age: 30, active: based}
vibez.spill("Person - name:", person.name, "age:", person.age, "active:", person.active)

# Modify struct fields
p1.x = 100
p1.y = 200
vibez.spill("Modified p1 - x:", p1.x, "y:", p1.y)

person.age = 31
person.active = cringe
vibez.spill("Modified person - age:", person.age, "active:", person.active)
