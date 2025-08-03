fr fr Comprehensive type system test for CURSED compiler
fr fr Tests parsing, type checking, and code generation of complex types

yeet "testz"

fr fr Test basic CURSED types
test_start("Basic CURSED Types")

sus x normie = 42
sus name tea = "Alice"
sus flag lit = based
sus small smol = 127
sus big thicc = 999999999
sus pi meal = 3.14159
sus tiny snack = 2.5
sus char sip = 'A'
sus data byte = 255

vibez.spillf("Values: {} {} {} {} {} {} {} {} {}", 
    x, name, flag, small, big, pi, tiny, char, data)

assert_eq_int(x, 42)
assert_eq_string(name, "Alice")
assert_true(flag)

fr fr Test complex types
test_start("Complex Types")

fr fr Array types
sus numbers []normie = [1, 2, 3, 4, 5]
sus texts []tea = ["hello", "world", "cursed"]

fr fr Map types
sus ages map[tea]normie = {"Alice": 25, "Bob": 30}

fr fr Channel types
sus ch dm<normie> = make_channel<normie>()

fr fr Pointer types
sus ptr *normie = &x

vibez.spillf("Array length: {}, Map size: {}", numbers.len(), ages.len())

assert_eq_int(numbers.len(), 5)
assert_eq_int(ages.len(), 2)

fr fr Test struct with complex types
squad Person {
    spill name tea
    spill age normie
    spill scores []normie
    spill contacts map[tea]tea
}

test_start("Struct with Complex Types")

sus person Person = Person{
    name: "Alice",
    age: 25,
    scores: [95, 87, 92],
    contacts: {"email": "alice@example.com", "phone": "123-456-7890"}
}

vibez.spillf("Person: {} age {}", person.name, person.age)
assert_eq_string(person.name, "Alice")
assert_eq_int(person.age, 25)
assert_eq_int(person.scores.len(), 3)

fr fr Test generic types
test_start("Generic Types")

squad Container<T> {
    spill value T
    spill values []T
}

slay create_container<T>(initial_value T) Container<T> {
    damn Container<T>{
        value: initial_value,
        values: [initial_value]
    }
}

sus int_container Container<normie> = create_container<normie>(42)
sus string_container Container<tea> = create_container<tea>("hello")

assert_eq_int(int_container.value, 42)
assert_eq_string(string_container.value, "hello")

fr fr Test interfaces with complex type constraints
test_start("Interface Type Checking")

collab Serializable<T> {
    slay serialize() tea
    slay deserialize(data tea) T
    slay get_type_name() tea
}

collab Container_Interface<T> where T: Serializable<T> {
    slay store(value T)
    slay retrieve() T
    slay size() normie
}

squad DataStore<T> where T: Serializable<T> {
    spill items []T
    spill metadata map[tea]tea
}

flex DataStore<T> => Container_Interface<T> {
    slay store(value T) {
        items.push(value)
    }
    
    slay retrieve() T {
        if items.len() > 0 {
            damn items.pop()
        }
        damn default_value<T>()
    }
    
    slay size() normie {
        damn items.len()
    }
}

fr fr Test function types
test_start("Function Types")

sus processor (normie) -> tea = slay(x normie) tea {
    damn "processed: " + x.to_string()
}

sus result tea = processor(42)
assert_eq_string(result, "processed: 42")

fr fr Test tuple types
test_start("Tuple Types")

sus coordinate (normie, normie) = (10, 20)
sus (x_pos, y_pos) = coordinate

assert_eq_int(x_pos, 10)
assert_eq_int(y_pos, 20)

sus named_tuple (name: tea, age: normie, active: lit) = (
    name: "Bob", 
    age: 30, 
    active: based
)

assert_eq_string(named_tuple.name, "Bob")
assert_eq_int(named_tuple.age, 30)
assert_true(named_tuple.active)

fr fr Test type aliases and advanced patterns
test_start("Type Aliases and Patterns")

type UserID = normie
type Username = tea
type UserData = map[tea]tea

sus user_id UserID = 12345
sus username Username = "alice123"
sus user_data UserData = {"role": "admin", "department": "engineering"}

assert_eq_int(user_id, 12345)
assert_eq_string(username, "alice123")

fr fr Test pattern matching with types
sus value normie = 42
sus type_result tea = match value {
    x normie if x > 0 => "positive integer",
    x normie if x < 0 => "negative integer", 
    0 => "zero",
    _ => "unknown"
}

assert_eq_string(type_result, "positive integer")

fr fr Test concurrency with typed channels
test_start("Typed Channels")

sus number_channel dm<normie> = make_channel<normie>()
sus string_channel dm<tea> = make_channel<tea>()

stan {
    dm_send(number_channel, 100)
    dm_send(string_channel, "concurrent")
}

sus received_number normie = dm_recv(number_channel)
sus received_string tea = dm_recv(string_channel)

assert_eq_int(received_number, 100)
assert_eq_string(received_string, "concurrent")

fr fr Test error handling with types
test_start("Error Handling with Types")

slay divide_safe(a normie, b normie) Result<normie, tea> {
    if b == 0 {
        damn Err("division by zero")
    }
    damn Ok(a / b)
}

sus division_result = divide_safe(10, 2)
match division_result {
    Ok(value) => assert_eq_int(value, 5),
    Err(error) => vibez.spill("Unexpected error:", error)
}

fr fr Test memory management with complex types
test_start("Memory Management")

sus large_data [][]normie = []
bestie i := 0; i < 100; i = i + 1 {
    sus row []normie = []
    bestie j := 0; j < 100; j = j + 1 {
        row.push(i * j)
    }
    large_data.push(row)
}

assert_eq_int(large_data.len(), 100)
assert_eq_int(large_data[50].len(), 100)
assert_eq_int(large_data[10][10], 100)

print_test_summary()

vibez.spill("Type system comprehensive test completed successfully!")
