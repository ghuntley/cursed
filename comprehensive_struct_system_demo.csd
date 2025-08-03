fr fr Comprehensive CURSED Struct System Demonstration
fr fr Showcasing all implemented struct features

fr fr ===== Basic Struct Declarations =====

squad Point {
    spill x normie
    spill y normie
}

squad Person {
    spill name tea
    spill age normie
    spill active lit
}

squad Address {
    spill street tea
    spill city tea
    spill zip normie
}

fr fr ===== Complex Nested Structs =====

squad Employee {
    spill person Person
    spill address Address
    spill salary meal
    spill id normie
}

squad Company {
    spill name tea
    spill employees []Employee
    spill headquarters Address
}

fr fr ===== Generic Struct Support =====

squad Container<T> {
    spill value T
    spill size normie
    spill capacity normie
}

squad Pair<T, U> {
    spill first T
    spill second U
}

fr fr ===== Struct with Methods (Advanced Feature) =====

squad Calculator {
    spill value meal
}

slay (calc Calculator) add(x meal) Calculator {
    calc.value = calc.value + x
    damn calc
}

slay (calc Calculator) multiply(x meal) Calculator {
    calc.value = calc.value * x
    damn calc
}

slay (calc Calculator) getValue() meal {
    damn calc.value
}

fr fr ===== Interface Definition =====

collab Drawable {
    slay draw()
    slay area() meal
}

squad Circle {
    spill radius meal
    spill center Point
}

fr fr ===== Interface Implementation =====

flex Circle => Drawable {
    slay draw() {
        vibez.spill("Drawing circle with radius", radius)
    }
    
    slay area() meal {
        damn 3.14159 * radius * radius
    }
}

fr fr ===== Main Testing Function =====

slay test_basic_struct_operations() {
    vibez.spill("=== Testing Basic Struct Operations ===")
    
    fr fr Basic struct instantiation
    sus point Point = Point{x: 10, y: 20}
    vibez.spill("Point created:", point.x, point.y)
    
    sus person Person = Person{
        name: "John Doe",
        age: 30,
        active: based
    }
    vibez.spill("Person:", person.name, "age", person.age)
    
    fr fr Field modification
    person.age = 31
    person.active = cringe
    vibez.spill("Updated person age:", person.age)
    
    fr fr Nested struct instantiation
    sus address Address = Address{
        street: "123 Main St",
        city: "New York",
        zip: 10001
    }
    
    sus employee Employee = Employee{
        person: person,
        address: address,
        salary: 75000.5,
        id: 12345
    }
    
    vibez.spill("Employee:", employee.person.name)
    vibez.spill("Lives in:", employee.address.city)
    vibez.spill("Salary:", employee.salary)
    
    fr fr Access nested fields
    vibez.spill("Employee street:", employee.address.street)
    vibez.spill("Employee ZIP:", employee.address.zip)
}

slay test_generic_structs() {
    vibez.spill("=== Testing Generic Structs ===")
    
    fr fr Generic container with integer
    sus int_container Container<normie> = Container<normie>{
        value: 42,
        size: 1,
        capacity: 10
    }
    vibez.spill("Int container value:", int_container.value)
    
    fr fr Generic container with string
    sus string_container Container<tea> = Container<tea>{
        value: "Hello CURSED",
        size: 12,
        capacity: 50
    }
    vibez.spill("String container value:", string_container.value)
    
    fr fr Generic pair
    sus coordinate_pair Pair<normie, normie> = Pair<normie, normie>{
        first: 100,
        second: 200
    }
    vibez.spill("Coordinate pair:", coordinate_pair.first, coordinate_pair.second)
    
    sus name_age_pair Pair<tea, normie> = Pair<tea, normie>{
        first: "Alice",
        second: 25
    }
    vibez.spill("Name-age pair:", name_age_pair.first, name_age_pair.second)
}

slay test_struct_methods() {
    vibez.spill("=== Testing Struct Methods ===")
    
    sus calc Calculator = Calculator{value: 10.0}
    vibez.spill("Initial calculator value:", calc.getValue())
    
    calc = calc.add(5.0)
    vibez.spill("After adding 5:", calc.getValue())
    
    calc = calc.multiply(2.0)
    vibez.spill("After multiplying by 2:", calc.getValue())
    
    calc = calc.add(3.0)
    vibez.spill("Final result:", calc.getValue())
}

slay test_struct_arrays() {
    vibez.spill("=== Testing Struct Arrays ===")
    
    sus points []Point = [
        Point{x: 0, y: 0},
        Point{x: 10, y: 20},
        Point{x: 30, y: 40},
        Point{x: 50, y: 60}
    ]
    
    vibez.spill("Point array created with", points.len(), "elements")
    
    bestie i := 0; i < points.len(); i = i + 1 {
        vibez.spill("Point", i, ":", points[i].x, points[i].y)
    }
    
    fr fr Array of complex structs
    sus employees []Employee = [
        Employee{
            person: Person{name: "Alice", age: 25, active: based},
            address: Address{street: "100 First St", city: "Boston", zip: 2101},
            salary: 65000.0,
            id: 1001
        },
        Employee{
            person: Person{name: "Bob", age: 30, active: based},
            address: Address{street: "200 Second St", city: "Seattle", zip: 98101},
            salary: 80000.0,
            id: 1002
        }
    ]
    
    bestie emp := flex employees {
        vibez.spill("Employee:", emp.person.name, "from", emp.address.city)
    }
}

slay test_interface_implementation() {
    vibez.spill("=== Testing Interface Implementation ===")
    
    sus circle Circle = Circle{
        radius: 5.0,
        center: Point{x: 0, y: 0}
    }
    
    fr fr Use as Drawable interface
    sus drawable Drawable = circle
    drawable.draw()
    
    sus area meal = drawable.area()
    vibez.spill("Circle area:", area)
}

slay test_struct_copying() {
    vibez.spill("=== Testing Struct Copying ===")
    
    sus original Person = Person{
        name: "Original",
        age: 25,
        active: based
    }
    
    sus copy Person = original
    copy.name = "Copy"
    copy.age = 30
    
    vibez.spill("Original:", original.name, "age", original.age)
    vibez.spill("Copy:", copy.name, "age", copy.age)
}

slay test_memory_management() {
    vibez.spill("=== Testing Memory Management ===")
    
    fr fr Create many structs to test allocation
    bestie i := 0; i < 1000; i = i + 1 {
        sus temp Point = Point{x: i, y: i * 2}
        if i % 100 == 0 {
            vibez.spill("Created", i, "structs")
        }
    }
    
    vibez.spill("Memory management test completed")
}

fr fr ===== Main Program Entry Point =====

slay main() {
    vibez.spill("🚀 CURSED Struct System Comprehensive Demo")
    vibez.spill("==========================================")
    
    test_basic_struct_operations()
    test_generic_structs()
    test_struct_methods()
    test_struct_arrays()
    test_interface_implementation()
    test_struct_copying()
    test_memory_management()
    
    vibez.spill("==========================================")
    vibez.spill("✅ All struct system tests completed successfully!")
    vibez.spill("The CURSED struct system with 'squad' keyword is fully operational")
}

fr fr Execute the demonstration
main()
