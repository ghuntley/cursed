yeet "testz"

// Basic interface definition
be_like Greeter collab {
    greet(name tea) tea
}

// Struct implementing interface
be_like Person squad {
    name tea
    age normie
}

slay (p Person) greet(name tea) tea {
    damn "Hello " + name + ", I'm " + p.name
}

slay test_interface_basic() {
    sus person Person = Person{name: "Alice", age: 30}
    sus greeter Greeter = person
    
    sus result = greeter.greet("Bob")
    assert_eq_string(result, "Hello Bob, I'm Alice")
}

// Multiple implementations
be_like Robot squad {
    model tea
    version normie
}

slay (r Robot) greet(name tea) tea {
    damn "HELLO " + name + ". I AM " + r.model + " VERSION " + string(r.version)
}

slay test_interface_multiple_implementations() {
    sus person Person = Person{name: "Charlie", age: 25}
    sus robot Robot = Robot{model: "R2D2", version: 2}
    
    sus greetings []tea
    
    sus greeters []Greeter = []Greeter{person, robot}
    
    bestie _, g := flex greeters {
        greetings = append(greetings, g.greet("World"))
    }
    
    assert_eq_int(len(greetings), 2)
    assert_eq_string(greetings[0], "Hello World, I'm Charlie")
    assert_eq_string(greetings[1], "HELLO World. I AM R2D2 VERSION 2")
}

// Interface composition
be_like Speaker collab {
    speak() tea
}

be_like Walker collab {
    walk() tea
}

be_like Animal collab {
    Speaker
    Walker
    name() tea
}

be_like Dog squad {
    dog_name tea
}

slay (d Dog) speak() tea {
    damn "Woof! I'm " + d.dog_name
}

slay (d Dog) walk() tea {
    damn d.dog_name + " is walking on four legs"
}

slay (d Dog) name() tea {
    damn d.dog_name
}

slay test_interface_composition() {
    sus dog Dog = Dog{dog_name: "Buddy"}
    sus animal Animal = dog
    
    assert_eq_string(animal.speak(), "Woof! I'm Buddy")
    assert_eq_string(animal.walk(), "Buddy is walking on four legs")
    assert_eq_string(animal.name(), "Buddy")
}

// Type assertion tests
slay test_type_assertion() {
    sus person Person = Person{name: "Dave", age: 35}
    sus greeter Greeter = person
    
    // Type assertion that should succeed
    sus p, ok = greeter.(Person)
    assert_true(ok)
    assert_eq_string(p.name, "Dave")
    
    // Type assertion that should fail
    sus _, ok = greeter.(Robot)
    assert_false(ok)
}

// Interface with generic methods
be_like Container[T] collab {
    add(item T)
    get(index normie) T
    size() normie
}

be_like SimpleList[T] squad {
    items []T
}

slay (sl @SimpleList[T]) add(item T) {
    sl.items = append(sl.items, item)
}

slay (sl @SimpleList[T]) get(index normie) T {
    damn sl.items[index]
}

slay (sl @SimpleList[T]) size() normie {
    damn len(sl.items)
}

slay test_generic_interface() {
    sus list SimpleList[normie]
    sus container Container[normie] = @list
    
    container.add(10)
    container.add(20)
    container.add(30)
    
    assert_eq_int(container.size(), 3)
    assert_eq_int(container.get(0), 10)
    assert_eq_int(container.get(2), 30)
}

// Empty interface (any type)
be_like Any collab {
}

slay test_empty_interface() {
    sus values []Any = []Any{42, "hello", based}
    
    assert_eq_int(len(values), 3)
    
    // Type assertions on empty interface
    sus int_val, ok = values[0].(normie)
    assert_true(ok)
    assert_eq_int(int_val, 42)
    
    sus str_val, ok = values[1].(tea)
    assert_true(ok)
    assert_eq_string(str_val, "hello")
    
    sus bool_val, ok = values[2].(lit)
    assert_true(ok)
    assert_true(bool_val)
}

// Method set tests
be_like Counter collab {
    increment()
    decrement()
    value() normie
}

be_like SimpleCounter squad {
    count normie
}

slay (sc @SimpleCounter) increment() {
    sc.count++
}

slay (sc @SimpleCounter) decrement() {
    sc.count--
}

slay (sc @SimpleCounter) value() normie {
    damn sc.count
}

slay test_method_set() {
    sus counter SimpleCounter = SimpleCounter{count: 0}
    sus c Counter = @counter
    
    c.increment()
    c.increment()
    c.increment()
    c.decrement()
    
    assert_eq_int(c.value(), 2)
}

// Interface satisfaction checks
slay process_greeter(g Greeter) tea {
    damn g.greet("Interface Test")
}

slay test_interface_satisfaction() {
    sus person Person = Person{name: "Eve", age: 28}
    sus robot Robot = Robot{model: "HAL", version: 9000}
    
    sus result1 = process_greeter(person)
    sus result2 = process_greeter(robot)
    
    assert_eq_string(result1, "Hello Interface Test, I'm Eve")
    assert_eq_string(result2, "HELLO Interface Test. I AM HAL VERSION 9000")
}

// Test driver
test_start("Interface Basic")
test_interface_basic()
print_test_summary()

test_start("Interface Multiple Implementations")
test_interface_multiple_implementations()
print_test_summary()

test_start("Interface Composition")
test_interface_composition()
print_test_summary()

test_start("Type Assertion")
test_type_assertion()
print_test_summary()

test_start("Generic Interface")
test_generic_interface()
print_test_summary()

test_start("Empty Interface")
test_empty_interface()
print_test_summary()

test_start("Method Set")
test_method_set()
print_test_summary()

test_start("Interface Satisfaction")
test_interface_satisfaction()
print_test_summary()

vibez.spill("All interface tests completed!")
