# CURSED Data Structures Test
# Testing arrays, structs, interfaces

# Test arrays
spill("=== DATA STRUCTURES TEST ===")
spill("Testing arrays:")

sus numbers []drip = [1, 2, 3, 4, 5]
spill("Array:", numbers)
spill("Length:", len(numbers))
spill("First element:", numbers[0])
spill("Last element:", numbers[len(numbers) - 1])

# Modify array
numbers[2] = 99
spill("After modification:", numbers)

# Test dynamic arrays
sus dynamic_array []tea = []
append(dynamic_array, "first")
append(dynamic_array, "second")
append(dynamic_array, "third")
spill("Dynamic array:", dynamic_array)

# Test structs (squad)
squad Person {
    name tea
    age drip
    active lit
}

slay new_person(name tea, age drip) Person {
    damn Person{name: name, age: age, active: based}
}

slay (p Person) introduce() tea {
    damn "Hi, I'm " + p.name + " and I'm " + str(p.age) + " years old"
}

sus person1 Person = new_person("Alice", 30)
sus person2 Person = Person{name: "Bob", age: 25, active: nah}

spill("Person 1:", person1.introduce())
spill("Person 2:", person2.introduce())
spill("Person 1 active:", person1.active)
spill("Person 2 active:", person2.active)

# Test interfaces (collab) if implemented
collab Greeter {
    greet() tea
}

slay (p Person) greet() tea {
    damn "Hello from " + p.name
}

sus greeter Greeter = person1
spill("Interface greeting:", greeter.greet())

# Test enums if implemented
# enum Color {
#     Red
#     Green
#     Blue
# }
# 
# sus favorite_color Color = Color.Blue
# spill("Favorite color:", favorite_color)
