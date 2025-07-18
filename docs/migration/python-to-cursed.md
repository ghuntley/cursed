# Migrating from Python to CURSED

Moving from Python to CURSED involves transitioning from dynamic typing to static typing, from interpreted to compiled execution, and from duck typing to explicit interfaces. CURSED offers significant performance improvements while maintaining code clarity.

## Table of Contents

1. [Philosophy Differences](#philosophy-differences)
2. [Type System Transformation](#type-system-transformation)
3. [Syntax Translation](#syntax-translation)
4. [Performance Implications](#performance-implications)
5. [Object-Oriented Programming](#object-oriented-programming)
6. [Error Handling](#error-handling)
7. [Concurrency Models](#concurrency-models)
8. [Standard Library](#standard-library)
9. [Migration Strategy](#migration-strategy)
10. [Common Pitfalls](#common-pitfalls)
11. [Working Examples](#working-examples)

## Philosophy Differences

### Python Philosophy (The Zen of Python)
- **Beautiful is better than ugly**: Code aesthetics matter
- **Explicit is better than implicit**: Clear over clever
- **Simple is better than complex**: Avoid unnecessary complexity
- **Readability counts**: Code should be readable
- **Duck typing**: "If it walks like a duck and quacks like a duck, it's a duck"

### CURSED Philosophy
- **Vibes over verbosity**: Modern, expressive syntax
- **Static over dynamic**: Type safety at compile time
- **Performance over convenience**: Fast execution with clear code
- **Compile-time safety**: Catch errors before runtime
- **Structured concurrency**: Safe concurrent programming

## Type System Transformation

### Dynamic vs Static Typing

**Python:**
```python
# Dynamic typing - types determined at runtime
x = 42
x = "hello"  # OK - can change type
x = [1, 2, 3]  # OK - can change type again

def add(a, b):
    return a + b  # Works with any type that supports +

result = add(5, 3)        # Returns 8
result = add("hi", "bye") # Returns "hibye"
```

**CURSED:**
```cursed
// Static typing - types determined at compile time
sus x normie = 42
// x = "hello"  // Error - cannot change type

slay add(a normie, b normie) normie {
    damn a + b
}

slay add_strings(a tea, b tea) tea {
    damn a + b
}

result := add(5, 3)              // Returns 8
result2 := add_strings("hi", "bye") // Returns "hibye"
```

### Type Annotations

**Python (with type hints):**
```python
from typing import List, Dict, Optional

def process_data(items: List[int]) -> Dict[str, int]:
    result: Dict[str, int] = {}
    for item in items:
        result[str(item)] = item * 2
    return result

def find_user(user_id: int) -> Optional[str]:
    users = {1: "Alice", 2: "Bob"}
    return users.get(user_id)
```

**CURSED:**
```cursed
yeet "stringz"

slay process_data(items []normie) map[tea]normie {
    result := make(map[tea]normie)
    bestie item := range items {
        result[stringz.from_int(item)] = item * 2
    }
    damn result
}

slay find_user(user_id normie) tea {
    users := map[normie]tea{1: "Alice", 2: "Bob"}
    lowkey value, exists := users[user_id]; exists {
        damn value
    }
    yikes "user not found"
}
```

### None vs Nil/Error Handling

**Python:**
```python
def divide(a, b):
    if b == 0:
        return None
    return a / b

result = divide(10, 2)
if result is not None:
    print(f"Result: {result}")
else:
    print("Division by zero")
```

**CURSED:**
```cursed
slay divide(a meal, b meal) meal {
    lowkey b == 0 {
        yikes "division by zero"
    }
    damn a / b
}

slay main() {
    shook {
        result := divide(10, 2)
        vibez.spill("Result: ", result)
    } fam err {
        vibez.spill("Division by zero")
    }
}
```

## Syntax Translation

### Variables and Basic Operations

**Python:**
```python
# Variables
name = "Alice"
age = 25
height = 5.6
is_student = True

# Basic operations
full_name = name + " Smith"
next_year = age + 1
bmi = weight / (height ** 2)

# List operations
numbers = [1, 2, 3, 4, 5]
numbers.append(6)
first = numbers[0]
```

**CURSED:**
```cursed
// Variables
sus name tea = "Alice"
sus age normie = 25
sus height meal = 5.6
sus is_student lit = based

// Basic operations
full_name := name + " Smith"
next_year := age + 1
bmi := weight / (height * height)

// Array operations
numbers := [1, 2, 3, 4, 5]
numbers = append(numbers, 6)
first := numbers[0]
```

### Functions

**Python:**
```python
def greet(name, greeting="Hello"):
    return f"{greeting}, {name}!"

def calculate_area(length, width):
    return length * width

def process_numbers(*args):
    return sum(args)

# Function calls
message = greet("Alice")
custom_message = greet("Bob", "Hi")
area = calculate_area(5, 3)
total = process_numbers(1, 2, 3, 4)
```

**CURSED:**
```cursed
slay greet(name tea, greeting tea) tea {
    lowkey greeting == "" {
        greeting = "Hello"
    }
    damn greeting + ", " + name + "!"
}

slay calculate_area(length meal, width meal) meal {
    damn length * width
}

slay process_numbers(args ...normie) normie {
    total := 0
    bestie arg := range args {
        total += arg
    }
    damn total
}

// Function calls
message := greet("Alice", "Hello")
custom_message := greet("Bob", "Hi")
area := calculate_area(5, 3)
total := process_numbers(1, 2, 3, 4)
```

### Classes vs Structs

**Python:**
```python
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
        self.friends = []
    
    def add_friend(self, friend):
        self.friends.append(friend)
    
    def greet(self):
        return f"Hello, I'm {self.name}"
    
    def __str__(self):
        return f"Person(name={self.name}, age={self.age})"

# Usage
person = Person("Alice", 25)
person.add_friend("Bob")
print(person.greet())
print(person)
```

**CURSED:**
```cursed
vibes Person struct {
    name tea
    age normie
    friends []tea
}

slay new_person(name tea, age normie) Person {
    damn Person{
        name: name,
        age: age,
        friends: make([]tea, 0),
    }
}

slay (p *Person) add_friend(friend tea) {
    p.friends = append(p.friends, friend)
}

slay (p Person) greet() tea {
    damn "Hello, I'm " + p.name
}

slay (p Person) to_string() tea {
    damn "Person(name=" + p.name + ", age=" + stringz.from_int(p.age) + ")"
}

// Usage
person := new_person("Alice", 25)
person.add_friend("Bob")
vibez.spill(person.greet())
vibez.spill(person.to_string())
```

### Control Flow

**Python:**
```python
# If statements
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")

# For loops
for i in range(5):
    print(f"Number: {i}")

for item in items:
    print(f"Item: {item}")

# While loops
count = 0
while count < 5:
    print(f"Count: {count}")
    count += 1

# List comprehensions
squares = [x**2 for x in range(10)]
evens = [x for x in numbers if x % 2 == 0]
```

**CURSED:**
```cursed
// If statements
lowkey age >= 18 {
    vibez.spill("Adult")
} sus lowkey age >= 13 {
    vibez.spill("Teenager")
} sus {
    vibez.spill("Child")
}

// For loops
bestie i := 0; i < 5; i++ {
    vibez.spill("Number: ", i)
}

bestie item := range items {
    vibez.spill("Item: ", item)
}

// While loops
count := 0
bestie count < 5 {
    vibez.spill("Count: ", count)
    count++
}

// Array processing (no direct comprehension, use functions)
squares := make([]normie, 10)
bestie i := 0; i < 10; i++ {
    squares[i] = i * i
}

evens := make([]normie, 0)
bestie x := range numbers {
    lowkey x % 2 == 0 {
        evens = append(evens, x)
    }
}
```

## Performance Implications

### Execution Model

**Python:**
```python
# Interpreted execution
import time

def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

start = time.time()
result = fibonacci(35)
end = time.time()
print(f"Fibonacci(35) = {result}, Time: {end - start:.2f}s")
```

**CURSED:**
```cursed
yeet "timez"

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n-1) + fibonacci(n-2)
}

slay main() {
    start := timez.now()
    result := fibonacci(35)
    end := timez.now()
    duration := end.sub(start)
    vibez.spill("Fibonacci(35) = ", result, ", Time: ", duration.seconds(), "s")
}
```

### Memory Usage

**Python:**
```python
# Python - automatic memory management, higher overhead
import sys

data = [i for i in range(1000000)]
print(f"Memory usage: {sys.getsizeof(data)} bytes")

# Dictionary with dynamic typing
user_data = {
    "name": "Alice",
    "age": 25,
    "scores": [85, 92, 78]
}
```

**CURSED:**
```cursed
yeet "runtime"

slay main() {
    // CURSED - garbage collected, lower overhead
    data := make([]normie, 1000000)
    bestie i := 0; i < 1000000; i++ {
        data[i] = i
    }
    
    memory_usage := runtime.memory_usage()
    vibez.spill("Memory usage: ", memory_usage, " bytes")
    
    // Struct with static typing
    vibes UserData struct {
        name tea
        age normie
        scores []normie
    }
    
    user_data := UserData{
        name: "Alice",
        age: 25,
        scores: [85, 92, 78],
    }
}
```

## Object-Oriented Programming

### Classes vs Structs and Interfaces

**Python:**
```python
from abc import ABC, abstractmethod

class Animal(ABC):
    def __init__(self, name):
        self.name = name
    
    @abstractmethod
    def make_sound(self):
        pass
    
    def describe(self):
        return f"This is {self.name}"

class Dog(Animal):
    def make_sound(self):
        return "Woof!"

class Cat(Animal):
    def make_sound(self):
        return "Meow!"

# Usage
animals = [Dog("Buddy"), Cat("Whiskers")]
for animal in animals:
    print(f"{animal.describe()}: {animal.make_sound()}")
```

**CURSED:**
```cursed
vibes Animal interface {
    make_sound() tea
    describe() tea
}

vibes Dog struct {
    name tea
}

vibes Cat struct {
    name tea
}

slay (d Dog) make_sound() tea {
    damn "Woof!"
}

slay (d Dog) describe() tea {
    damn "This is " + d.name
}

slay (c Cat) make_sound() tea {
    damn "Meow!"
}

slay (c Cat) describe() tea {
    damn "This is " + c.name
}

slay main() {
    animals := []Animal{
        Dog{name: "Buddy"},
        Cat{name: "Whiskers"},
    }
    
    bestie animal := range animals {
        vibez.spill(animal.describe(), ": ", animal.make_sound())
    }
}
```

### Inheritance vs Composition

**Python:**
```python
class Vehicle:
    def __init__(self, brand):
        self.brand = brand
    
    def start(self):
        return f"{self.brand} starting..."

class Car(Vehicle):
    def __init__(self, brand, doors):
        super().__init__(brand)
        self.doors = doors
    
    def drive(self):
        return f"Driving {self.brand} with {self.doors} doors"

class ElectricCar(Car):
    def __init__(self, brand, doors, battery_capacity):
        super().__init__(brand, doors)
        self.battery_capacity = battery_capacity
    
    def charge(self):
        return f"Charging {self.brand} battery ({self.battery_capacity}kWh)"
```

**CURSED:**
```cursed
vibes Vehicle interface {
    start() tea
    get_brand() tea
}

vibes Car struct {
    brand tea
    doors normie
}

vibes ElectricCar struct {
    Car                    // Composition
    battery_capacity normie
}

slay (c Car) start() tea {
    damn c.brand + " starting..."
}

slay (c Car) get_brand() tea {
    damn c.brand
}

slay (c Car) drive() tea {
    damn "Driving " + c.brand + " with " + stringz.from_int(c.doors) + " doors"
}

slay (e ElectricCar) charge() tea {
    damn "Charging " + e.brand + " battery (" + stringz.from_int(e.battery_capacity) + "kWh)"
}

slay main() {
    car := Car{brand: "Toyota", doors: 4}
    electric := ElectricCar{
        Car: Car{brand: "Tesla", doors: 4},
        battery_capacity: 75,
    }
    
    vibez.spill(car.start())
    vibez.spill(car.drive())
    vibez.spill(electric.start())
    vibez.spill(electric.charge())
}
```

## Error Handling

### Try/Except vs Yikes/Shook/Fam

**Python:**
```python
import json

def parse_json_file(filename):
    try:
        with open(filename, 'r') as file:
            data = json.load(file)
            return data
    except FileNotFoundError:
        print(f"File {filename} not found")
        return None
    except json.JSONDecodeError as e:
        print(f"Invalid JSON: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None

# Usage
data = parse_json_file("config.json")
if data:
    print(f"Loaded config: {data}")
else:
    print("Failed to load config")
```

**CURSED:**
```cursed
yeet "dropz"
yeet "encode_mood"

slay parse_json_file(filename tea) map[tea]interface{} {
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        content := file.read_all()
        data := encode_mood.json_decode(content)
        damn data
    } fam err {
        match err.type() {
            "FileNotFoundError" => {
                vibez.spill("File ", filename, " not found")
                yikes "file not found"
            }
            "JSONDecodeError" => {
                vibez.spill("Invalid JSON: ", err.message())
                yikes "invalid json"
            }
            basic => {
                vibez.spill("Unexpected error: ", err.message())
                yikes err.message()
            }
        }
    }
}

slay main() {
    shook {
        data := parse_json_file("config.json")
        vibez.spill("Loaded config: ", data)
    } fam err {
        vibez.spill("Failed to load config: ", err.message())
    }
}
```

## Concurrency Models

### Threading vs Goroutines

**Python:**
```python
import threading
import time
import queue

def worker(q, worker_id):
    while True:
        item = q.get()
        if item is None:
            break
        print(f"Worker {worker_id} processing {item}")
        time.sleep(1)
        q.task_done()

def main():
    q = queue.Queue()
    threads = []
    
    # Start worker threads
    for i in range(3):
        t = threading.Thread(target=worker, args=(q, i))
        t.start()
        threads.append(t)
    
    # Add tasks
    for i in range(10):
        q.put(f"task_{i}")
    
    # Wait for all tasks to complete
    q.join()
    
    # Stop workers
    for i in range(3):
        q.put(None)
    
    for t in threads:
        t.join()

if __name__ == "__main__":
    main()
```

**CURSED:**
```cursed
yeet "timez"

slay worker(tasks <-chan tea, worker_id normie) {
    bestie task := range tasks {
        vibez.spill("Worker ", worker_id, " processing ", task)
        timez.sleep(timez.second)
    }
}

slay main() {
    tasks := make(chan tea, 10)
    
    // Start worker goroutines
    bestie i := 0; i < 3; i++ {
        yolo worker(tasks, i)
    }
    
    // Add tasks
    bestie i := 0; i < 10; i++ {
        tasks <- "task_" + stringz.from_int(i)
    }
    
    // Close channel to signal completion
    close(tasks)
    
    // Wait for goroutines to finish
    timez.sleep(2 * timez.second)
}
```

### Asyncio vs Channels

**Python:**
```python
import asyncio
import aiohttp

async def fetch_url(session, url):
    async with session.get(url) as response:
        return await response.text()

async def main():
    urls = [
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2",
        "https://httpbin.org/delay/3",
    ]
    
    async with aiohttp.ClientSession() as session:
        tasks = [fetch_url(session, url) for url in urls]
        results = await asyncio.gather(*tasks)
        
        for i, result in enumerate(results):
            print(f"URL {i+1}: {len(result)} characters")

if __name__ == "__main__":
    asyncio.run(main())
```

**CURSED:**
```cursed
yeet "vibe_net"

slay fetch_url(url tea, results chan<- tea) {
    response := vibe_net.http_get(url)
    results <- response
}

slay main() {
    urls := [
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/2", 
        "https://httpbin.org/delay/3",
    ]
    
    results := make(chan tea, len(urls))
    
    // Start concurrent requests
    bestie i, url := range urls {
        yolo fetch_url(url, results)
    }
    
    // Collect results
    bestie i := 0; i < len(urls); i++ {
        result := <-results
        vibez.spill("URL ", i+1, ": ", len(result), " characters")
    }
}
```

## Standard Library

### Common Modules Comparison

| Python Module | CURSED Module | Description |
|---------------|---------------|-------------|
| `json` | `encode_mood` | JSON encoding/decoding |
| `os` | `vibe_life` | Operating system interface |
| `sys` | `sys_core` | System-specific parameters |
| `datetime` | `timez` | Date and time handling |
| `re` | `regex` | Regular expressions |
| `http` | `vibe_net` | HTTP client/server |
| `threading` | Built-in goroutines | Concurrency |
| `collections` | `collections` | Specialized containers |
| `itertools` | `iter_tools` | Iterator utilities |
| `functools` | `func_tools` | Higher-order functions |

### File Operations

**Python:**
```python
import os
import shutil

# Read file
with open('input.txt', 'r') as f:
    content = f.read()

# Write file
with open('output.txt', 'w') as f:
    f.write("Hello, World!")

# File operations
os.makedirs('new_dir', exist_ok=True)
shutil.copy('source.txt', 'dest.txt')
os.remove('temp.txt')
```

**CURSED:**
```cursed
yeet "dropz"
yeet "vibe_life"

slay main() {
    // Read file
    shook {
        file := dropz.open("input.txt")
        defer file.close()
        content := file.read_all()
        vibez.spill("Content: ", content)
    } fam err {
        vibez.spill("Error reading file: ", err.message())
    }
    
    // Write file
    shook {
        file := dropz.create("output.txt")
        defer file.close()
        file.write("Hello, World!")
    } fam err {
        vibez.spill("Error writing file: ", err.message())
    }
    
    // File operations
    vibe_life.mkdir("new_dir")
    vibe_life.copy("source.txt", "dest.txt")
    vibe_life.remove("temp.txt")
}
```

## Migration Strategy

### Phase 1: Type System Migration
1. **Add type annotations**: Start with Python type hints
2. **Identify dynamic patterns**: Find code that relies on dynamic typing
3. **Define data structures**: Convert classes to structs and interfaces
4. **Handle None values**: Replace with explicit error handling

### Phase 2: Syntax Translation
1. **Convert basic syntax**: Variables, functions, control flow
2. **Replace print statements**: Use `vibez.spill` instead of `print`
3. **Update imports**: Change `import` to `yeet`
4. **Fix string formatting**: Use concatenation or formatting functions

### Phase 3: Performance Optimization
1. **Leverage static typing**: Use compile-time optimizations
2. **Replace dynamic structures**: Use typed arrays instead of lists
3. **Optimize hot paths**: Use efficient algorithms and data structures
4. **Add concurrency**: Use goroutines for parallel processing

### Phase 4: CURSED-Specific Features
1. **Enhanced error handling**: Use `yikes`/`shook`/`fam` pattern
2. **Structured concurrency**: Replace threads with goroutines
3. **Comprehensive stdlib**: Use CURSED's extensive standard library
4. **Compilation optimizations**: Leverage LLVM backend

## Common Pitfalls

### 1. Dynamic Typing Habits
**Problem:** Trying to use variables with different types
```python
# Python (OK)
x = 42
x = "hello"  # Type changes at runtime
```

**Solution:** Use static typing
```cursed
// CURSED (must be consistent)
sus x normie = 42
sus y tea = "hello"  // Different variable for different type
```

### 2. List Mutability Assumptions
**Problem:** Assuming lists are always mutable
```python
# Python (mutable)
numbers = [1, 2, 3]
numbers.append(4)  # Modifies original list
```

**Solution:** Use explicit allocation
```cursed
// CURSED (use append for growth)
numbers := [1, 2, 3]
numbers = append(numbers, 4)  // Creates new slice
```

### 3. Exception Handling Patterns
**Problem:** Using try/except for control flow
```python
# Python (not recommended but works)
try:
    value = dictionary[key]
except KeyError:
    value = default_value
```

**Solution:** Use explicit checking
```cursed
// CURSED (explicit checking)
lowkey value, exists := dictionary[key]; exists {
    // use value
} sus {
    value = default_value
}
```

### 4. Duck Typing Expectations
**Problem:** Assuming duck typing works
```python
# Python (duck typing)
def process(obj):
    return obj.method()  # Works if obj has method()
```

**Solution:** Use interfaces
```cursed
// CURSED (explicit interfaces)
vibes Processor interface {
    method() tea
}

slay process(obj Processor) tea {
    damn obj.method()
}
```

## Working Examples

### Example 1: Web API Server

**Python:**
```python
from flask import Flask, jsonify, request
import json

app = Flask(__name__)

users = [
    {"id": 1, "name": "Alice", "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "email": "bob@example.com"},
]

@app.route('/users', methods=['GET'])
def get_users():
    return jsonify(users)

@app.route('/users/<int:user_id>', methods=['GET'])
def get_user(user_id):
    user = next((u for u in users if u['id'] == user_id), None)
    if user:
        return jsonify(user)
    return jsonify({"error": "User not found"}), 404

@app.route('/users', methods=['POST'])
def create_user():
    data = request.get_json()
    new_user = {
        "id": len(users) + 1,
        "name": data['name'],
        "email": data['email']
    }
    users.append(new_user)
    return jsonify(new_user), 201

if __name__ == '__main__':
    app.run(debug=True)
```

**CURSED:**
```cursed
yeet "vibe_net"
yeet "encode_mood"

vibes User struct {
    id normie `json:"id"`
    name tea `json:"name"`
    email tea `json:"email"`
}

sus users []User = [
    User{id: 1, name: "Alice", email: "alice@example.com"},
    User{id: 2, name: "Bob", email: "bob@example.com"},
]

slay get_users(w vibe_net.ResponseWriter, r *vibe_net.Request) {
    w.header().set("Content-Type", "application/json")
    encode_mood.json_encode(w, users)
}

slay get_user(w vibe_net.ResponseWriter, r *vibe_net.Request) {
    user_id := vibe_net.get_path_param(r, "user_id")
    
    bestie user := range users {
        lowkey user.id == user_id {
            w.header().set("Content-Type", "application/json")
            encode_mood.json_encode(w, user)
            damn
        }
    }
    
    w.write_header(404)
    encode_mood.json_encode(w, map[tea]tea{"error": "User not found"})
}

slay create_user(w vibe_net.ResponseWriter, r *vibe_net.Request) {
    shook {
        body := r.read_body()
        sus data map[tea]tea
        encode_mood.json_decode(body, &data)
        
        new_user := User{
            id: len(users) + 1,
            name: data["name"],
            email: data["email"],
        }
        
        users = append(users, new_user)
        
        w.write_header(201)
        w.header().set("Content-Type", "application/json")
        encode_mood.json_encode(w, new_user)
    } fam err {
        w.write_header(400)
        encode_mood.json_encode(w, map[tea]tea{"error": err.message()})
    }
}

slay main() {
    mux := vibe_net.new_mux()
    mux.handle_func("/users", get_users, "GET")
    mux.handle_func("/users", create_user, "POST")
    mux.handle_func("/users/{user_id}", get_user, "GET")
    
    vibez.spill("Server starting on :8080")
    vibe_net.listen_and_serve(":8080", mux)
}
```

### Example 2: Data Processing Pipeline

**Python:**
```python
import csv
import json
from datetime import datetime
from typing import List, Dict

def read_csv(filename: str) -> List[Dict]:
    data = []
    with open(filename, 'r') as file:
        reader = csv.DictReader(file)
        for row in reader:
            data.append(row)
    return data

def process_data(data: List[Dict]) -> List[Dict]:
    processed = []
    for item in data:
        processed_item = {
            'id': int(item['id']),
            'name': item['name'].strip().title(),
            'score': float(item['score']),
            'processed_at': datetime.now().isoformat()
        }
        processed.append(processed_item)
    return processed

def write_json(data: List[Dict], filename: str):
    with open(filename, 'w') as file:
        json.dump(data, file, indent=2)

def main():
    try:
        # Read CSV data
        raw_data = read_csv('input.csv')
        
        # Process data
        processed_data = process_data(raw_data)
        
        # Write JSON output
        write_json(processed_data, 'output.json')
        
        print(f"Processed {len(processed_data)} records")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
```

**CURSED:**
```cursed
yeet "dropz"
yeet "encode_mood"
yeet "stringz"
yeet "timez"

vibes RawRecord struct {
    id tea
    name tea
    score tea
}

vibes ProcessedRecord struct {
    id normie `json:"id"`
    name tea `json:"name"`
    score meal `json:"score"`
    processed_at tea `json:"processed_at"`
}

slay read_csv(filename tea) []RawRecord {
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        content := file.read_all()
        lines := stringz.split(content, "\n")
        
        records := make([]RawRecord, 0)
        bestie i := 1; i < len(lines); i++ {  // Skip header
            fields := stringz.split(lines[i], ",")
            lowkey len(fields) >= 3 {
                record := RawRecord{
                    id: fields[0],
                    name: fields[1],
                    score: fields[2],
                }
                records = append(records, record)
            }
        }
        damn records
    } fam err {
        yikes err.message()
    }
}

slay process_data(data []RawRecord) []ProcessedRecord {
    processed := make([]ProcessedRecord, 0)
    
    bestie item := range data {
        shook {
            processed_item := ProcessedRecord{
                id: stringz.to_int(item.id),
                name: stringz.title(stringz.trim(item.name)),
                score: stringz.to_float(item.score),
                processed_at: timez.now().rfc3339(),
            }
            processed = append(processed, processed_item)
        } fam err {
            vibez.spill("Error processing record: ", err.message())
            simp  // Skip this record
        }
    }
    
    damn processed
}

slay write_json(data []ProcessedRecord, filename tea) {
    shook {
        file := dropz.create(filename)
        defer file.close()
        
        json_data := encode_mood.json_encode_pretty(data)
        file.write(json_data)
    } fam err {
        yikes err.message()
    }
}

slay main() {
    shook {
        // Read CSV data
        raw_data := read_csv("input.csv")
        
        // Process data
        processed_data := process_data(raw_data)
        
        // Write JSON output
        write_json(processed_data, "output.json")
        
        vibez.spill("Processed ", len(processed_data), " records")
    } fam err {
        vibez.spill("Error: ", err.message())
    }
}
```

### Example 3: Concurrent File Processor

**Python:**
```python
import os
import threading
import time
from concurrent.futures import ThreadPoolExecutor
from queue import Queue

def process_file(filename):
    """Process a single file"""
    print(f"Processing {filename}...")
    time.sleep(1)  # Simulate processing
    
    with open(filename, 'r') as f:
        content = f.read()
    
    processed = content.upper()
    output_filename = f"processed_{filename}"
    
    with open(output_filename, 'w') as f:
        f.write(processed)
    
    print(f"Finished processing {filename}")
    return output_filename

def main():
    # Get list of files to process
    files = [f for f in os.listdir('.') if f.endswith('.txt')]
    
    # Process files concurrently
    with ThreadPoolExecutor(max_workers=4) as executor:
        results = list(executor.map(process_file, files))
    
    print(f"Processed {len(results)} files")
    for result in results:
        print(f"Created: {result}")

if __name__ == "__main__":
    main()
```

**CURSED:**
```cursed
yeet "dropz"
yeet "vibe_life"
yeet "stringz"
yeet "timez"

slay process_file(filename tea, results chan<- tea) {
    vibez.spill("Processing ", filename, "...")
    timez.sleep(timez.second)  // Simulate processing
    
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        content := file.read_all()
        processed := stringz.to_upper(content)
        
        output_filename := "processed_" + filename
        output_file := dropz.create(output_filename)
        defer output_file.close()
        
        output_file.write(processed)
        
        vibez.spill("Finished processing ", filename)
        results <- output_filename
    } fam err {
        vibez.spill("Error processing ", filename, ": ", err.message())
        results <- ""
    }
}

slay main() {
    // Get list of files to process
    files := vibe_life.glob("*.txt")
    
    // Create results channel
    results := make(chan tea, len(files))
    
    // Process files concurrently
    bestie filename := range files {
        yolo process_file(filename, results)
    }
    
    // Collect results
    processed_files := make([]tea, 0)
    bestie i := 0; i < len(files); i++ {
        result := <-results
        lowkey result != "" {
            processed_files = append(processed_files, result)
        }
    }
    
    vibez.spill("Processed ", len(processed_files), " files")
    bestie result := range processed_files {
        vibez.spill("Created: ", result)
    }
}
```

## Testing Your Migration

### Running Examples

```bash
# Test basic syntax
cargo run --bin cursed examples/basic_syntax.csd

# Test type safety
cargo run --bin cursed examples/type_safety.csd

# Test performance
cargo run --bin cursed examples/performance_test.csd

# Test concurrency
cargo run --bin cursed examples/concurrent_processing.csd

# Compile to native
cargo run --bin cursed -- compile examples/web_api.csd
./web_api
```

### Performance Comparison

```bash
# Python execution
time python3 fibonacci.py

# CURSED interpretation
time cargo run --bin cursed fibonacci.csd

# CURSED compilation
cargo run --bin cursed -- compile fibonacci.csd
time ./fibonacci
```

## Next Steps

1. **Embrace static typing**: Use the type system to catch errors early
2. **Leverage compilation**: Take advantage of native performance
3. **Learn concurrency**: Master goroutines and channels
4. **Explore stdlib**: Use CURSED's comprehensive standard library
5. **Optimize performance**: Use LLVM optimizations and profiling

The migration from Python to CURSED involves a significant paradigm shift but offers substantial benefits in performance, type safety, and concurrent programming capabilities.
