# CURSED Programming Language - Intermediate Tutorial

## Table of Contents
1. [Module System](#module-system)
2. [Structs and Methods](#structs-and-methods)
3. [Interfaces](#interfaces)
4. [Advanced Data Structures](#advanced-data-structures)
5. [Generics](#generics)
6. [Concurrency Basics](#concurrency-basics)
7. [File I/O](#file-io)
8. [Testing](#testing)
9. [Package Management](#package-management)

## Module System

### Creating Modules
Modules help organize code into reusable components.

**File: `math_utils.💀`**
```cursed
# Module for mathematical utilities
vibe math_utils

# Export functions using 'vibes' keyword
vibes {
    add_numbers,
    multiply_numbers,
    calculate_factorial
}

slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay multiply_numbers(a normie, b normie) normie {
    damn a * b
}

slay calculate_factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * calculate_factorial(n - 1)
}

# Private function (not exported)
slay internal_helper(x normie) normie {
    damn x * 2
}
```

### Importing Modules
```cursed
# Import specific functions
yeet "math_utils" { add_numbers, multiply_numbers }

# Import entire module
yeet "math_utils" as math

# Import from standard library
yeet "stringz"
yeet "collections"
yeet "io"

# Use imported functions
sus result normie = add_numbers(5, 3)
sus factorial normie = math.calculate_factorial(5)
```

### Package Structure
```
my_project/
├── main.💀
├── CursedPackage.toml
├── src/
│   ├── math/
│   │   ├── mod.💀
│   │   ├── calculator.💀
│   │   └── statistics.💀
│   └── utils/
│       ├── mod.💀
│       └── helpers.💀
```

## Structs and Methods

### Defining Structs
```cursed
# Define a struct
struct Person {
    name tea
    age normie
    email tea
    isActive lit
}

# Create struct instances
sus person1 Person = Person{
    name: "Alice",
    age: 30,
    email: "alice@example.com",
    isActive: based
}

# Shorthand initialization
sus person2 := Person{"Bob", 25, "bob@example.com", based}
```

### Methods
```cursed
# Define methods for structs
slay (p *Person) greet() {
    vibez.spill("Hello, I'm " + p.name)
}

slay (p *Person) get_age() normie {
    damn p.age
}

slay (p *Person) set_age(new_age normie) {
    p.age = new_age
}

slay (p *Person) is_adult() lit {
    damn p.age >= 18
}

# Method with validation
slay (p *Person) set_email(new_email tea) tea {
    lowkey new_email == "" {
        damn "Email cannot be empty"
    }
    p.email = new_email
    damn ""
}

# Using methods
person1.greet()
person1.set_age(31)
vibez.spill("Is adult: " + person1.is_adult())
```

### Embedded Structs
```cursed
struct Address {
    street tea
    city tea
    zip_code tea
}

struct Employee {
    Person          # Embedded struct
    Address         # Embedded struct
    employee_id normie
    salary meal
}

sus employee := Employee{
    Person: Person{"John", 28, "john@company.com", based},
    Address: Address{"123 Main St", "Anytown", "12345"},
    employee_id: 1001,
    salary: 75000.0
}

# Access embedded fields
vibez.spill("Employee name: " + employee.name)
vibez.spill("Employee city: " + employee.city)
employee.greet()  # Method from embedded Person
```

## Interfaces

### Defining Interfaces
```cursed
# Define an interface
interface Drawable {
    draw()
    get_area() meal
}

interface Movable {
    move(x normie, y normie)
    get_position() (normie, normie)
}

# Structs implementing interfaces
struct Circle {
    radius meal
    x normie
    y normie
}

slay (c *Circle) draw() {
    vibez.spill("Drawing circle with radius " + c.radius)
}

slay (c *Circle) get_area() meal {
    damn 3.14159 * c.radius * c.radius
}

slay (c *Circle) move(x normie, y normie) {
    c.x = x
    c.y = y
}

slay (c *Circle) get_position() (normie, normie) {
    damn c.x, c.y
}

struct Rectangle {
    width meal
    height meal
    x normie
    y normie
}

slay (r *Rectangle) draw() {
    vibez.spill("Drawing rectangle " + r.width + "x" + r.height)
}

slay (r *Rectangle) get_area() meal {
    damn r.width * r.height
}

slay (r *Rectangle) move(x normie, y normie) {
    r.x = x
    r.y = y
}

slay (r *Rectangle) get_position() (normie, normie) {
    damn r.x, r.y
}
```

### Using Interfaces
```cursed
# Function that accepts any Drawable
slay draw_shape(shape Drawable) {
    shape.draw()
    vibez.spill("Area: " + shape.get_area())
}

# Function that accepts any Movable
slay move_shape(shape Movable, x normie, y normie) {
    shape.move(x, y)
    sus (px, py) := shape.get_position()
    vibez.spill("Moved to: (" + px + ", " + py + ")")
}

# Create shapes
sus circle := Circle{radius: 5.0, x: 0, y: 0}
sus rectangle := Rectangle{width: 10.0, height: 8.0, x: 0, y: 0}

# Use interface functions
draw_shape(circle)
draw_shape(rectangle)
move_shape(circle, 10, 20)
```

### Interface Composition
```cursed
# Combine interfaces
interface Shape {
    Drawable
    Movable
}

slay process_shape(shape Shape) {
    shape.draw()
    vibez.spill("Area: " + shape.get_area())
    shape.move(100, 200)
}

process_shape(circle)
process_shape(rectangle)
```

## Advanced Data Structures

### Maps (Hash Tables)
```cursed
yeet "collections"

# Create and use maps
sus ages := collections.new_map[tea, normie]()

# Add items
ages.put("Alice", 30)
ages.put("Bob", 25)
ages.put("Charlie", 35)

# Get items
sus alice_age := ages.get("Alice")
vibez.spill("Alice's age: " + alice_age)

# Check existence
lowkey ages.contains("David") {
    vibez.spill("David found")
} vibes {
    vibez.spill("David not found")
}

# Iterate over map
bestie (key, value) in ages {
    vibez.spill(key + " is " + value + " years old")
}
```

### Lists (Dynamic Arrays)
```cursed
yeet "collections"

# Create list
sus numbers := collections.new_list[normie]()

# Add elements
numbers.append(1)
numbers.append(2)
numbers.append(3)

# Insert at specific position
numbers.insert(1, 10)  # Insert 10 at index 1

# Remove elements
numbers.remove(2)  # Remove element at index 2

# Access elements
vibez.spill("First element: " + numbers.get(0))
vibez.spill("List size: " + numbers.size())

# Iterate
bestie num in numbers {
    vibez.spill("Number: " + num)
}
```

### Sets
```cursed
yeet "collections"

# Create set
sus unique_numbers := collections.new_set[normie]()

# Add elements
unique_numbers.add(1)
unique_numbers.add(2)
unique_numbers.add(1)  # Duplicate, won't be added

# Check membership
lowkey unique_numbers.contains(1) {
    vibez.spill("Set contains 1")
}

# Set operations
sus other_set := collections.new_set[normie]()
other_set.add(2)
other_set.add(3)

sus union_set := unique_numbers.union(other_set)
sus intersection_set := unique_numbers.intersection(other_set)
```

## Generics

### Generic Functions
```cursed
# Generic function for any type
slay max[T](a T, b T) T {
    lowkey a > b {
        damn a
    }
    damn b
}

# Usage
sus max_int := max[normie](5, 3)
sus max_float := max[meal](3.14, 2.71)
sus max_string := max[tea]("apple", "banana")
```

### Generic Structs
```cursed
# Generic stack implementation
struct Stack[T] {
    items []T
    top normie
}

slay (s *Stack[T]) push(item T) {
    s.items = append(s.items, item)
    s.top++
}

slay (s *Stack[T]) pop() (T, lit) {
    lowkey s.top == 0 {
        sus zero T
        damn zero, cap
    }
    s.top--
    sus item := s.items[s.top]
    s.items = s.items[:s.top]
    damn item, based
}

slay (s *Stack[T]) is_empty() lit {
    damn s.top == 0
}

# Usage
sus int_stack := Stack[normie]{}
int_stack.push(1)
int_stack.push(2)
int_stack.push(3)

sus (value, ok) := int_stack.pop()
lowkey ok {
    vibez.spill("Popped: " + value)
}
```

### Generic Interfaces
```cursed
interface Comparable[T] {
    compare(other T) normie  # -1, 0, 1
}

interface Container[T] {
    add(item T)
    remove(index normie) (T, lit)
    size() normie
}

# Generic function using interface
slay sort[T](items []T) where T: Comparable[T] {
    # Simple bubble sort
    bestie i := 0; i < len(items); i++ {
        bestie j := 0; j < len(items)-1; j++ {
            lowkey items[j].compare(items[j+1]) > 0 {
                # Swap
                sus temp := items[j]
                items[j] = items[j+1]
                items[j+1] = temp
            }
        }
    }
}
```

## Concurrency Basics

### Goroutines
```cursed
yeet "concurrenz"

# Simple goroutine
slay say_hello(name tea) {
    bestie i := 0; i < 5; i++ {
        vibez.spill("Hello " + name + " " + i)
        concurrenz.sleep(1000)  # Sleep for 1 second
    }
}

# Launch goroutines
yolo say_hello("Alice")
yolo say_hello("Bob")

# Wait for goroutines to complete
concurrenz.sleep(6000)
```

### Channels
```cursed
yeet "concurrenz"

# Create a channel
sus messages := make(chan tea)

# Send messages from a goroutine
yolo slay() {
    messages <- "Hello"
    messages <- "World"
    close(messages)
}

# Receive messages
bestie msg := range messages {
    vibez.spill("Received: " + msg)
}
```

### Channel Operations
```cursed
yeet "concurrenz"

# Buffered channel
sus numbers := make(chan normie, 3)

# Send without blocking
numbers <- 1
numbers <- 2
numbers <- 3

# Receive
sus first := <-numbers
sus second := <-numbers
sus third := <-numbers

vibez.spill("Numbers: " + first + ", " + second + ", " + third)
```

### Select Statement
```cursed
yeet "concurrenz"

sus ch1 := make(chan tea)
sus ch2 := make(chan tea)

yolo slay() {
    concurrenz.sleep(1000)
    ch1 <- "from ch1"
}

yolo slay() {
    concurrenz.sleep(2000)
    ch2 <- "from ch2"
}

ready {
    msg1 := <-ch1:
        vibez.spill("Received " + msg1)
    msg2 := <-ch2:
        vibez.spill("Received " + msg2)
    timeout(3000):
        vibez.spill("Timeout!")
}
```

## File I/O

### Reading Files
```cursed
yeet "io"
yeet "dropz"

# Read entire file
sus content tea
sus error tea
(content, error) = dropz.read_file("example.txt")

lowkey error != "" {
    vibez.spill("Error reading file: " + error)
} vibes {
    vibez.spill("File content: " + content)
}

# Read file line by line
sus file := dropz.open("example.txt")
defer file.close()

bestie line := range file.lines() {
    vibez.spill("Line: " + line)
}
```

### Writing Files
```cursed
yeet "dropz"

# Write to file
sus content tea = "Hello, CURSED file I/O!"
sus error tea = dropz.write_file("output.txt", content)

lowkey error != "" {
    vibez.spill("Error writing file: " + error)
} vibes {
    vibez.spill("File written successfully")
}

# Append to file
sus file := dropz.open_append("log.txt")
defer file.close()

file.write("Log entry: " + current_time() + "\n")
```

### File Operations
```cursed
yeet "dropz"

# Check if file exists
lowkey dropz.exists("myfile.txt") {
    vibez.spill("File exists")
}

# Get file info
sus info := dropz.stat("myfile.txt")
vibez.spill("File size: " + info.size())
vibez.spill("Last modified: " + info.modified())

# Create directories
dropz.mkdir("new_directory")
dropz.mkdir_all("path/to/nested/directory")

# List directory contents
sus files := dropz.list_dir(".")
bestie file in files {
    vibez.spill("File: " + file.name())
}
```

## Testing

### Basic Testing
```cursed
yeet "testz"

# Test function
slay test_add() {
    testz.test_start("add function")
    
    sus result := add(2, 3)
    testz.assert_eq_int(result, 5)
    
    testz.test_end()
}

# Test with multiple assertions
slay test_string_operations() {
    testz.test_start("string operations")
    
    sus str1 tea = "hello"
    sus str2 tea = "world"
    sus combined tea = str1 + " " + str2
    
    testz.assert_eq_string(combined, "hello world")
    testz.assert_true(len(combined) > 0)
    testz.assert_false(combined == "")
    
    testz.test_end()
}

# Run tests
test_add()
test_string_operations()
testz.print_test_summary()
```

### Advanced Testing
```cursed
yeet "testz"

# Test fixture setup
struct TestFixture {
    db Database
    test_data []normie
}

slay setup_test_fixture() TestFixture {
    sus fixture := TestFixture{
        db: create_test_database(),
        test_data: [1, 2, 3, 4, 5]
    }
    damn fixture
}

slay teardown_test_fixture(fixture TestFixture) {
    fixture.db.close()
}

# Test with fixture
slay test_database_operations() {
    testz.test_start("database operations")
    
    sus fixture := setup_test_fixture()
    defer teardown_test_fixture(fixture)
    
    # Test database operations
    sus error := fixture.db.insert("test_table", "test_data")
    testz.assert_eq_string(error, "")
    
    sus count := fixture.db.count("test_table")
    testz.assert_eq_int(count, 1)
    
    testz.test_end()
}
```

### Benchmark Testing
```cursed
yeet "testz"

slay benchmark_sort() {
    testz.benchmark_start("sort performance")
    
    sus data := generate_random_data(10000)
    
    testz.benchmark_time_start()
    sort(data)
    testz.benchmark_time_end()
    
    testz.benchmark_end()
}

benchmark_sort()
```

## Package Management

### Creating a Package
**File: `CursedPackage.toml`**
```toml
[package]
name = "my_awesome_package"
version = "1.0.0"
authors = ["Your Name <your.email@example.com>"]
description = "An awesome CURSED package"
license = "MIT"
repository = "https://github.com/yourusername/my_awesome_package"

[dependencies]
stringz = "^1.0"
collections = "^2.0"
io = "^1.5"

[dev-dependencies]
testz = "^3.0"
```

### Using Dependencies
```cursed
# Import dependencies
yeet "stringz"
yeet "collections"

# Use in your code
sus words := stringz.split("hello world", " ")
sus word_set := collections.new_set[tea]()

bestie word in words {
    word_set.add(word)
}
```

### Publishing Packages
```bash
# Build package
cursed build

# Run tests
cursed test

# Publish to registry
cursed publish
```

## Practice Projects

### Project 1: Task Manager
Create a simple task management application:

```cursed
yeet "collections"
yeet "dropz"
yeet "timez"

struct Task {
    id normie
    title tea
    description tea
    completed lit
    created_at tea
}

struct TaskManager {
    tasks collections.Map[normie, Task]
    next_id normie
}

slay (tm *TaskManager) add_task(title tea, description tea) normie {
    sus task := Task{
        id: tm.next_id,
        title: title,
        description: description,
        completed: cap,
        created_at: timez.now()
    }
    
    tm.tasks.put(tm.next_id, task)
    tm.next_id++
    
    damn task.id
}

slay (tm *TaskManager) complete_task(id normie) lit {
    sus task := tm.tasks.get(id)
    lowkey task == cringe {
        damn cap
    }
    
    task.completed = based
    tm.tasks.put(id, task)
    damn based
}

slay (tm *TaskManager) list_tasks() {
    bestie (id, task) in tm.tasks {
        sus status tea = "[ ]"
        lowkey task.completed {
            status = "[x]"
        }
        vibez.spill(status + " " + task.title + " - " + task.description)
    }
}

# Usage
sus manager := TaskManager{
    tasks: collections.new_map[normie, Task](),
    next_id: 1
}

sus task_id := manager.add_task("Learn CURSED", "Complete the intermediate tutorial")
manager.add_task("Build project", "Create a real application")
manager.list_tasks()
manager.complete_task(task_id)
manager.list_tasks()
```

### Project 2: Web Server
Create a simple HTTP server:

```cursed
yeet "web_vibez"
yeet "json"

struct User {
    id normie
    name tea
    email tea
}

sus users := collections.new_map[normie, User]()
sus next_user_id := 1

slay handle_get_users(req web_vibez.Request) web_vibez.Response {
    sus user_list := collections.new_list[User]()
    
    bestie (id, user) in users {
        user_list.append(user)
    }
    
    sus json_data := json.marshal(user_list)
    damn web_vibez.json_response(json_data)
}

slay handle_create_user(req web_vibez.Request) web_vibez.Response {
    sus user User
    sus error := json.unmarshal(req.body, &user)
    
    lowkey error != "" {
        damn web_vibez.error_response(400, "Invalid JSON")
    }
    
    user.id = next_user_id
    next_user_id++
    
    users.put(user.id, user)
    
    sus json_data := json.marshal(user)
    damn web_vibez.json_response(json_data)
}

slay main() {
    sus server := web_vibez.new_server()
    
    server.get("/users", handle_get_users)
    server.post("/users", handle_create_user)
    
    vibez.spill("Server starting on port 8080")
    server.listen(8080)
}
```

## Next Steps

You've completed the intermediate tutorial! You now know:
- Module system and package management
- Structs, methods, and interfaces
- Advanced data structures and generics
- Basic concurrency with goroutines and channels
- File I/O operations
- Testing and benchmarking
- Building real applications

### Continue Learning
- [Advanced Tutorial](advanced.md) - Master advanced concurrency, performance optimization, and metaprogramming
- [API Documentation](../api/) - Comprehensive standard library reference
- [Best Practices](../best-practices.md) - Production-ready coding patterns
- [Performance Guide](../performance.md) - Optimization techniques

---

*Keep slaying with CURSED! 🚀*
