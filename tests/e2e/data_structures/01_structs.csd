yeet "testz"

test_start("Struct Data Structure Tests")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create struct instance
sus p1 Point = Point{x: 10, y: 20}

vibez.spill("Point 1: (" + str(p1.x) + ", " + str(p1.y) + ")")
assert_eq_int(p1.x, 10)
assert_eq_int(p1.y, 20)

# Struct with different types
squad Person {
    spill name tea
    spill age drip
    spill active lit
}

sus person Person = Person{
    name: "Alice",
    age: 30,
    active: based
}

vibez.spill("Person: " + person.name + ", age " + str(person.age))
assert_eq_string(person.name, "Alice")
assert_eq_int(person.age, 30)
assert_true(person.active)

# Struct methods
slay Point.distance_from_origin() meal {
    damn sqrt(real(this.x * this.x + this.y * this.y))
}

slay Point.move(dx drip, dy drip) {
    this.x = this.x + dx
    this.y = this.y + dy
}

# Test methods
sus distance meal = p1.distance_from_origin()
vibez.spill("Distance from origin: " + str(distance))

p1.move(5, -5)
vibez.spill("After move: (" + str(p1.x) + ", " + str(p1.y) + ")")
assert_eq_int(p1.x, 15)
assert_eq_int(p1.y, 15)

# Nested structs
squad Address {
    spill street tea
    spill city tea
    spill zip tea
}

squad Employee {
    spill person Person
    spill address Address
    spill salary drip
}

sus emp Employee = Employee{
    person: Person{name: "Bob", age: 25, active: based},
    address: Address{street: "123 Main St", city: "Anytown", zip: "12345"},
    salary: 50000
}

vibez.spill("Employee: " + emp.person.name + " in " + emp.address.city)
assert_eq_string(emp.person.name, "Bob")
assert_eq_string(emp.address.city, "Anytown")
assert_eq_int(emp.salary, 50000)

print_test_summary()
