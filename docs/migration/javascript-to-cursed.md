# Migrating from JavaScript to CURSED

Transitioning from JavaScript to CURSED involves moving from dynamic, loosely-typed scripting to static, strongly-typed compiled programming. CURSED provides type safety, performance, and structured development while maintaining code expressiveness.

## Table of Contents

1. [Paradigm Shift](#paradigm-shift)
2. [Type System Evolution](#type-system-evolution)
3. [Syntax Translation](#syntax-translation)
4. [Asynchronous Programming](#asynchronous-programming)
5. [Object-Oriented vs Struct-Based](#object-oriented-vs-struct-based)
6. [Module Systems](#module-systems)
7. [Error Handling](#error-handling)
8. [Performance Considerations](#performance-considerations)
9. [Migration Strategy](#migration-strategy)
10. [Common Pitfalls](#common-pitfalls)
11. [Working Examples](#working-examples)

## Paradigm Shift

### JavaScript Philosophy
- **Dynamic typing**: Types determined at runtime
- **Prototype-based**: Objects inherit from other objects
- **Event-driven**: Asynchronous, non-blocking operations
- **Flexible syntax**: Multiple ways to accomplish tasks
- **Runtime flexibility**: Code can be modified at runtime

### CURSED Philosophy
- **Static typing**: Types checked at compile time
- **Struct-based**: Composition over inheritance
- **Concurrent programming**: Goroutines and channels
- **Consistent syntax**: Clear, structured approach
- **Compile-time safety**: Errors caught before deployment

## Type System Evolution

### Dynamic vs Static Variables

**JavaScript:**
```javascript
// Dynamic typing - no type declarations
let x = 42;
x = "hello";        // OK - type changes at runtime
x = [1, 2, 3];      // OK - type changes again
x = {a: 1, b: 2};   // OK - type changes again

// Functions with no type information
function add(a, b) {
    return a + b;   // Works with numbers, strings, etc.
}

console.log(add(5, 3));        // 8
console.log(add("hi", "bye")); // "hibye"
```

**CURSED:**
```cursed
// Static typing - explicit type declarations
sus x normie = 42
// x = "hello"  // Error - cannot change type

// Functions with explicit types
slay add(a normie, b normie) normie {
    damn a + b
}

slay add_strings(a tea, b tea) tea {
    damn a + b
}

vibez.spill(add(5, 3))              // 8
vibez.spill(add_strings("hi", "bye")) // "hibye"
```

### Type Annotations and Interfaces

**JavaScript (with TypeScript-like annotations):**
```javascript
// TypeScript-style type annotations
interface User {
    id: number;
    name: string;
    email: string;
}

function getUser(id: number): User | null {
    const users: User[] = [
        { id: 1, name: "Alice", email: "alice@example.com" },
        { id: 2, name: "Bob", email: "bob@example.com" }
    ];
    
    return users.find(u => u.id === id) || null;
}

// Optional parameters
function greet(name: string, greeting: string = "Hello"): string {
    return `${greeting}, ${name}!`;
}
```

**CURSED:**
```cursed
// Explicit struct and interface definitions
vibes User struct {
    id normie
    name tea
    email tea
}

slay get_user(id normie) User {
    users := []User{
        User{id: 1, name: "Alice", email: "alice@example.com"},
        User{id: 2, name: "Bob", email: "bob@example.com"},
    }
    
    bestie user := range users {
        lowkey user.id == id {
            damn user
        }
    }
    
    yikes "user not found"
}

// Optional parameters using function overloading
slay greet(name tea) tea {
    damn greet_with_greeting(name, "Hello")
}

slay greet_with_greeting(name tea, greeting tea) tea {
    damn greeting + ", " + name + "!"
}
```

### Undefined vs Nil/Error Handling

**JavaScript:**
```javascript
// Undefined and null handling
function divide(a, b) {
    if (b === 0) {
        return undefined;
    }
    return a / b;
}

const result = divide(10, 2);
if (result !== undefined) {
    console.log(`Result: ${result}`);
} else {
    console.log("Division by zero");
}

// Null coalescing
const username = user?.name ?? "Anonymous";
```

**CURSED:**
```cursed
// Explicit error handling
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
    
    // Default value handling
    username := user.name
    lowkey username == "" {
        username = "Anonymous"
    }
}
```

## Syntax Translation

### Variables and Constants

**JavaScript:**
```javascript
// Variable declarations
let mutableVar = 42;
const immutableVar = "hello";
var oldStyleVar = true;

// Destructuring
const [first, second] = [1, 2];
const {name, age} = {name: "Alice", age: 25};

// Template literals
const message = `Hello, ${name}! You are ${age} years old.`;
```

**CURSED:**
```cursed
// Variable declarations
sus mutableVar normie = 42
facts immutableVar tea = "hello"
sus oldStyleVar lit = based

// Tuple destructuring
first, second := 1, 2
// Struct destructuring requires explicit access
person := Person{name: "Alice", age: 25}
name := person.name
age := person.age

// String concatenation (no template literals)
message := "Hello, " + name + "! You are " + stringz.from_int(age) + " years old."
```

### Functions

**JavaScript:**
```javascript
// Function declarations
function regularFunction(a, b) {
    return a + b;
}

// Arrow functions
const arrowFunction = (a, b) => a + b;

// Higher-order functions
const numbers = [1, 2, 3, 4, 5];
const doubled = numbers.map(x => x * 2);
const evens = numbers.filter(x => x % 2 === 0);
const sum = numbers.reduce((acc, x) => acc + x, 0);

// Async functions
async function fetchData(url) {
    const response = await fetch(url);
    return await response.json();
}
```

**CURSED:**
```cursed
// Function declarations
slay regular_function(a normie, b normie) normie {
    damn a + b
}

// No direct arrow function equivalent, use regular functions
slay arrow_function(a normie, b normie) normie {
    damn a + b
}

// Higher-order functions (manual implementation)
slay map_double(numbers []normie) []normie {
    result := make([]normie, len(numbers))
    bestie i, x := range numbers {
        result[i] = x * 2
    }
    damn result
}

slay filter_evens(numbers []normie) []normie {
    result := make([]normie, 0)
    bestie x := range numbers {
        lowkey x % 2 == 0 {
            result = append(result, x)
        }
    }
    damn result
}

slay reduce_sum(numbers []normie) normie {
    sum := 0
    bestie x := range numbers {
        sum += x
    }
    damn sum
}

// Async functions using goroutines
slay fetch_data(url tea, result chan<- tea) {
    response := vibe_net.http_get(url)
    result <- response
}
```

### Control Flow

**JavaScript:**
```javascript
// If statements
if (age >= 18) {
    console.log("Adult");
} else if (age >= 13) {
    console.log("Teenager");
} else {
    console.log("Child");
}

// Switch statements
switch (day) {
    case "Monday":
        console.log("Start of work week");
        break;
    case "Friday":
        console.log("TGIF!");
        break;
    default:
        console.log("Regular day");
}

// Loops
for (let i = 0; i < 5; i++) {
    console.log(`Number: ${i}`);
}

for (const item of items) {
    console.log(`Item: ${item}`);
}

// While loops
let count = 0;
while (count < 5) {
    console.log(`Count: ${count}`);
    count++;
}
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

// Switch statements
match day {
    "Monday" => vibez.spill("Start of work week")
    "Friday" => vibez.spill("TGIF!")
    basic => vibez.spill("Regular day")
}

// Loops
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
```

## Asynchronous Programming

### Promises vs Goroutines

**JavaScript:**
```javascript
// Promises
function fetchUserData(userId) {
    return fetch(`/api/users/${userId}`)
        .then(response => response.json())
        .then(data => {
            console.log("User data:", data);
            return data;
        })
        .catch(error => {
            console.error("Error:", error);
            throw error;
        });
}

// Async/await
async function processUser(userId) {
    try {
        const userData = await fetchUserData(userId);
        const processedData = await processData(userData);
        await saveData(processedData);
        console.log("User processed successfully");
    } catch (error) {
        console.error("Processing failed:", error);
    }
}
```

**CURSED:**
```cursed
// Goroutines and channels
slay fetch_user_data(user_id normie, result chan<- User) {
    shook {
        url := "/api/users/" + stringz.from_int(user_id)
        response := vibe_net.http_get(url)
        user := encode_mood.json_decode(response)
        vibez.spill("User data: ", user)
        result <- user
    } fam error {
        vibez.spill("Error: ", error.message())
        result <- User{} // Send empty user to indicate error
    }
}

slay process_user(user_id normie) {
    user_channel := make(chan User)
    processed_channel := make(chan User)
    saved_channel := make(chan lit)
    
    // Fetch user data
    yolo fetch_user_data(user_id, user_channel)
    
    // Process in pipeline
    yolo {
        user := <-user_channel
        processed := process_data(user)
        processed_channel <- processed
    }
    
    yolo {
        processed := <-processed_channel
        save_data(processed)
        saved_channel <- based
    }
    
    // Wait for completion
    <-saved_channel
    vibez.spill("User processed successfully")
}
```

### Event Handling vs Channel Communication

**JavaScript:**
```javascript
// Event emitter pattern
class EventEmitter {
    constructor() {
        this.events = {};
    }
    
    on(event, callback) {
        if (!this.events[event]) {
            this.events[event] = [];
        }
        this.events[event].push(callback);
    }
    
    emit(event, data) {
        if (this.events[event]) {
            this.events[event].forEach(callback => callback(data));
        }
    }
}

// Usage
const emitter = new EventEmitter();

emitter.on('user-login', (user) => {
    console.log(`User ${user.name} logged in`);
});

emitter.on('user-logout', (user) => {
    console.log(`User ${user.name} logged out`);
});

emitter.emit('user-login', {name: 'Alice'});
```

**CURSED:**
```cursed
// Channel-based communication
vibes Event struct {
    event_type tea
    data interface{}
}

slay event_emitter(events <-chan Event) {
    bestie event := range events {
        match event.event_type {
            "user-login" => {
                user := event.data.(User)
                vibez.spill("User ", user.name, " logged in")
            }
            "user-logout" => {
                user := event.data.(User)
                vibez.spill("User ", user.name, " logged out")
            }
            basic => {
                vibez.spill("Unknown event: ", event.event_type)
            }
        }
    }
}

slay main() {
    events := make(chan Event)
    
    // Start event processor
    yolo event_emitter(events)
    
    // Send events
    events <- Event{
        event_type: "user-login",
        data: User{name: "Alice"},
    }
    
    events <- Event{
        event_type: "user-logout", 
        data: User{name: "Alice"},
    }
    
    close(events)
}
```

### Callbacks vs Channels

**JavaScript:**
```javascript
// Callback pattern
function processFiles(files, callback) {
    let processedCount = 0;
    const results = [];
    
    files.forEach((file, index) => {
        processFile(file, (err, result) => {
            if (err) {
                callback(err, null);
                return;
            }
            
            results[index] = result;
            processedCount++;
            
            if (processedCount === files.length) {
                callback(null, results);
            }
        });
    });
}

// Usage
processFiles(['file1.txt', 'file2.txt'], (err, results) => {
    if (err) {
        console.error('Error:', err);
    } else {
        console.log('Results:', results);
    }
});
```

**CURSED:**
```cursed
// Channel pattern
slay process_files(files []tea) []tea {
    results := make([]tea, len(files))
    done := make(chan lit)
    
    bestie i, file := range files {
        yolo {
            result := process_file(file)
            results[i] = result
            done <- based
        }
    }
    
    // Wait for all to complete
    bestie i := 0; i < len(files); i++ {
        <-done
    }
    
    damn results
}

slay main() {
    files := []tea{"file1.txt", "file2.txt"}
    
    shook {
        results := process_files(files)
        vibez.spill("Results: ", results)
    } fam err {
        vibez.spill("Error: ", err.message())
    }
}
```

## Object-Oriented vs Struct-Based

### Classes vs Structs

**JavaScript:**
```javascript
// Class-based approach
class User {
    constructor(name, email) {
        this.name = name;
        this.email = email;
        this.loginCount = 0;
    }
    
    login() {
        this.loginCount++;
        console.log(`${this.name} logged in (${this.loginCount} times)`);
    }
    
    getProfile() {
        return {
            name: this.name,
            email: this.email,
            loginCount: this.loginCount
        };
    }
    
    static createGuest() {
        return new User("Guest", "guest@example.com");
    }
}

// Inheritance
class AdminUser extends User {
    constructor(name, email, permissions) {
        super(name, email);
        this.permissions = permissions;
    }
    
    deleteUser(userId) {
        if (this.permissions.includes('delete')) {
            console.log(`Admin ${this.name} deleted user ${userId}`);
        } else {
            console.log('Permission denied');
        }
    }
}

// Usage
const user = new User("Alice", "alice@example.com");
user.login();

const admin = new AdminUser("Bob", "bob@example.com", ['delete', 'edit']);
admin.deleteUser(123);
```

**CURSED:**
```cursed
// Struct-based approach
vibes User struct {
    name tea
    email tea
    login_count normie
}

slay new_user(name tea, email tea) User {
    damn User{
        name: name,
        email: email,
        login_count: 0,
    }
}

slay (u *User) login() {
    u.login_count++
    vibez.spill(u.name, " logged in (", u.login_count, " times)")
}

slay (u User) get_profile() map[tea]interface{} {
    damn map[tea]interface{}{
        "name": u.name,
        "email": u.email,
        "login_count": u.login_count,
    }
}

slay create_guest() User {
    damn new_user("Guest", "guest@example.com")
}

// Composition instead of inheritance
vibes AdminUser struct {
    User                    // Embedded struct
    permissions []tea
}

slay new_admin_user(name tea, email tea, permissions []tea) AdminUser {
    damn AdminUser{
        User: new_user(name, email),
        permissions: permissions,
    }
}

slay (a AdminUser) delete_user(user_id normie) {
    has_permission := cap
    bestie permission := range a.permissions {
        lowkey permission == "delete" {
            has_permission = based
            ghosted
        }
    }
    
    lowkey has_permission {
        vibez.spill("Admin ", a.name, " deleted user ", user_id)
    } sus {
        vibez.spill("Permission denied")
    }
}

slay main() {
    user := new_user("Alice", "alice@example.com")
    user.login()
    
    admin := new_admin_user("Bob", "bob@example.com", []tea{"delete", "edit"})
    admin.delete_user(123)
}
```

### Prototypes vs Interfaces

**JavaScript:**
```javascript
// Prototype-based inheritance
function Animal(name) {
    this.name = name;
}

Animal.prototype.speak = function() {
    console.log(`${this.name} makes a sound`);
};

function Dog(name, breed) {
    Animal.call(this, name);
    this.breed = breed;
}

Dog.prototype = Object.create(Animal.prototype);
Dog.prototype.constructor = Dog;

Dog.prototype.speak = function() {
    console.log(`${this.name} barks`);
};

// Usage
const dog = new Dog("Buddy", "Golden Retriever");
dog.speak(); // "Buddy barks"
```

**CURSED:**
```cursed
// Interface-based approach
vibes Animal interface {
    speak() tea
    get_name() tea
}

vibes Dog struct {
    name tea
    breed tea
}

slay (d Dog) speak() tea {
    damn d.name + " barks"
}

slay (d Dog) get_name() tea {
    damn d.name
}

vibes Cat struct {
    name tea
    color tea
}

slay (c Cat) speak() tea {
    damn c.name + " meows"
}

slay (c Cat) get_name() tea {
    damn c.name
}

slay make_animal_speak(animal Animal) {
    vibez.spill(animal.speak())
}

slay main() {
    dog := Dog{name: "Buddy", breed: "Golden Retriever"}
    cat := Cat{name: "Whiskers", color: "Orange"}
    
    make_animal_speak(dog)
    make_animal_speak(cat)
}
```

## Module Systems

### CommonJS/ES6 vs CURSED Modules

**JavaScript (CommonJS):**
```javascript
// math.js
function add(a, b) {
    return a + b;
}

function multiply(a, b) {
    return a * b;
}

module.exports = {
    add,
    multiply
};

// main.js
const math = require('./math');
// or
const { add, multiply } = require('./math');

console.log(math.add(2, 3));
console.log(multiply(4, 5));
```

**JavaScript (ES6 Modules):**
```javascript
// math.js
export function add(a, b) {
    return a + b;
}

export function multiply(a, b) {
    return a * b;
}

// main.js
import { add, multiply } from './math.js';
// or
import * as math from './math.js';

console.log(add(2, 3));
console.log(multiply(4, 5));
```

**CURSED:**
```cursed
// math/mod.csd
vibe math

slay add(a normie, b normie) normie {
    damn a + b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

// main.csd
vibe main

yeet "math"
yeet "vibez"

slay main() {
    vibez.spill(math.add(2, 3))
    vibez.spill(math.multiply(4, 5))
}
```

### NPM vs Package Management

**JavaScript (package.json):**
```json
{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21"
  },
  "scripts": {
    "start": "node index.js",
    "test": "jest"
  }
}
```

**CURSED (CursedPackage.toml):**
```toml
[package]
name = "my-app"
version = "1.0.0"

[dependencies]
web_vibez = "1.0.0"
collection_tools = "2.1.0"

[scripts]
start = "cursed run main.csd"
test = "cursed test"
```

## Error Handling

### Try/Catch vs Yikes/Shook/Fam

**JavaScript:**
```javascript
// Traditional error handling
function parseJSON(jsonString) {
    try {
        const data = JSON.parse(jsonString);
        return data;
    } catch (error) {
        console.error('JSON parse error:', error.message);
        return null;
    }
}

// Async error handling
async function fetchData(url) {
    try {
        const response = await fetch(url);
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Fetch error:', error.message);
        throw error;
    }
}

// Usage
fetchData('https://api.example.com/data')
    .then(data => console.log(data))
    .catch(error => console.error('Failed to fetch:', error));
```

**CURSED:**
```cursed
// CURSED error handling
slay parse_json(json_string tea) interface{} {
    shook {
        data := encode_mood.json_decode(json_string)
        damn data
    } fam error {
        vibez.spill("JSON parse error: ", error.message())
        yikes "json parse failed"
    }
}

slay fetch_data(url tea) interface{} {
    shook {
        response := vibe_net.http_get(url)
        
        lowkey response.status_code != 200 {
            yikes "HTTP error! status: " + stringz.from_int(response.status_code)
        }
        
        data := encode_mood.json_decode(response.body)
        damn data
    } fam error {
        vibez.spill("Fetch error: ", error.message())
        yikes error.message()
    }
}

slay main() {
    shook {
        data := fetch_data("https://api.example.com/data")
        vibez.spill(data)
    } fam error {
        vibez.spill("Failed to fetch: ", error.message())
    }
}
```

## Performance Considerations

### Interpretation vs Compilation

**JavaScript:**
```javascript
// JavaScript - JIT compiled at runtime
function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

console.time('fibonacci');
const result = fibonacci(35);
console.timeEnd('fibonacci');
console.log(`Result: ${result}`);
```

**CURSED:**
```cursed
// CURSED - compiled to native code
yeet "timez"

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay main() {
    start := timez.now()
    result := fibonacci(35)
    end := timez.now()
    
    duration := end.sub(start)
    vibez.spill("Time: ", duration.milliseconds(), "ms")
    vibez.spill("Result: ", result)
}
```

### Memory Management

**JavaScript:**
```javascript
// JavaScript - automatic garbage collection
function createLargeArray() {
    const largeArray = new Array(1000000).fill(0);
    return largeArray.map((_, index) => index);
}

// Memory usage is managed automatically
const data = createLargeArray();
console.log(`Array length: ${data.length}`);
```

**CURSED:**
```cursed
// CURSED - garbage collected with better performance
slay create_large_array() []normie {
    large_array := make([]normie, 1000000)
    bestie i := 0; i < 1000000; i++ {
        large_array[i] = i
    }
    damn large_array
}

slay main() {
    data := create_large_array()
    vibez.spill("Array length: ", len(data))
}
```

## Migration Strategy

### Phase 1: Type System Adoption
1. **Identify data structures**: Map JavaScript objects to CURSED structs
2. **Define interfaces**: Convert prototype chains to interfaces
3. **Type function signatures**: Add explicit parameter and return types
4. **Handle undefined/null**: Replace with explicit error handling

### Phase 2: Syntax Conversion
1. **Convert basic syntax**: Variables, functions, control flow
2. **Replace console.log**: Use `vibez.spill` instead
3. **Update imports**: Change `import`/`require` to `yeet`
4. **Fix string templating**: Use concatenation instead of template literals

### Phase 3: Async Programming
1. **Convert Promises**: Replace with goroutines and channels
2. **Handle callbacks**: Use channel communication patterns
3. **Event systems**: Replace event emitters with channels
4. **Error propagation**: Use CURSED's error handling system

### Phase 4: Performance Optimization
1. **Leverage compilation**: Use native code generation
2. **Optimize data structures**: Use efficient types and algorithms
3. **Concurrent processing**: Use goroutines for parallel work
4. **Memory efficiency**: Take advantage of GC improvements

## Common Pitfalls

### 1. Type Coercion Assumptions
**Problem:** Expecting JavaScript's loose type coercion
```javascript
// JavaScript (type coercion)
console.log(5 + "3");    // "53"
console.log("5" - 3);    // 2
console.log(true + 1);   // 2
```

**Solution:** Use explicit type conversion
```cursed
// CURSED (explicit conversion)
vibez.spill(stringz.from_int(5) + "3")  // "53"
vibez.spill(stringz.to_int("5") - 3)    // 2
vibez.spill(1 + 1)                      // 2 (boolean to int conversion)
```

### 2. Undefined/Null Handling
**Problem:** Expecting undefined/null behavior
```javascript
// JavaScript (undefined handling)
const obj = {};
console.log(obj.nonExistent);  // undefined
console.log(obj.nonExistent?.property);  // undefined
```

**Solution:** Use explicit error handling
```cursed
// CURSED (explicit checking)
obj := make(map[tea]interface{})
lowkey value, exists := obj["nonExistent"]; exists {
    vibez.spill(value)
} sus {
    vibez.spill("key not found")
}
```

### 3. Dynamic Property Access
**Problem:** Expecting dynamic property access
```javascript
// JavaScript (dynamic properties)
const obj = {name: "Alice", age: 25};
const key = "name";
console.log(obj[key]);  // "Alice"
```

**Solution:** Use maps for dynamic access
```cursed
// CURSED (use maps for dynamic access)
obj := map[tea]interface{}{
    "name": "Alice",
    "age": 25,
}
key := "name"
vibez.spill(obj[key])
```

### 4. Hoisting Assumptions
**Problem:** Expecting JavaScript hoisting behavior
```javascript
// JavaScript (hoisting)
console.log(x);  // undefined (not error)
var x = 5;
```

**Solution:** Declare variables before use
```cursed
// CURSED (must declare before use)
sus x normie = 5
vibez.spill(x)  // Must declare first
```

## Working Examples

### Example 1: REST API Server

**JavaScript (Express.js):**
```javascript
const express = require('express');
const app = express();

app.use(express.json());

const users = [
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' }
];

app.get('/users', (req, res) => {
    res.json(users);
});

app.get('/users/:id', (req, res) => {
    const id = parseInt(req.params.id);
    const user = users.find(u => u.id === id);
    
    if (user) {
        res.json(user);
    } else {
        res.status(404).json({ error: 'User not found' });
    }
});

app.post('/users', (req, res) => {
    const { name, email } = req.body;
    
    if (!name || !email) {
        return res.status(400).json({ error: 'Name and email required' });
    }
    
    const newUser = {
        id: users.length + 1,
        name,
        email
    };
    
    users.push(newUser);
    res.status(201).json(newUser);
});

app.listen(3000, () => {
    console.log('Server running on port 3000');
});
```

**CURSED:**
```cursed
yeet "vibe_net"
yeet "encode_mood"
yeet "stringz"

vibes User struct {
    id normie `json:"id"`
    name tea `json:"name"`
    email tea `json:"email"`
}

sus users []User = []User{
    User{id: 1, name: "Alice", email: "alice@example.com"},
    User{id: 2, name: "Bob", email: "bob@example.com"},
}

slay get_users(w vibe_net.ResponseWriter, r *vibe_net.Request) {
    w.header().set("Content-Type", "application/json")
    encode_mood.json_encode(w, users)
}

slay get_user(w vibe_net.ResponseWriter, r *vibe_net.Request) {
    id_str := vibe_net.get_path_param(r, "id")
    id := stringz.to_int(id_str)
    
    bestie user := range users {
        lowkey user.id == id {
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
        sus user_data map[tea]tea
        encode_mood.json_decode(body, &user_data)
        
        name := user_data["name"]
        email := user_data["email"]
        
        lowkey name == "" || email == "" {
            w.write_header(400)
            encode_mood.json_encode(w, map[tea]tea{"error": "Name and email required"})
            damn
        }
        
        new_user := User{
            id: len(users) + 1,
            name: name,
            email: email,
        }
        
        users = append(users, new_user)
        
        w.write_header(201)
        w.header().set("Content-Type", "application/json")
        encode_mood.json_encode(w, new_user)
    } fam err {
        w.write_header(500)
        encode_mood.json_encode(w, map[tea]tea{"error": err.message()})
    }
}

slay main() {
    mux := vibe_net.new_mux()
    
    mux.handle_func("/users", get_users, "GET")
    mux.handle_func("/users", create_user, "POST")
    mux.handle_func("/users/{id}", get_user, "GET")
    
    vibez.spill("Server running on port 3000")
    vibe_net.listen_and_serve(":3000", mux)
}
```

### Example 2: Real-time Chat Application

**JavaScript (Socket.io):**
```javascript
const express = require('express');
const http = require('http');
const socketIo = require('socket.io');

const app = express();
const server = http.createServer(app);
const io = socketIo(server);

const users = new Map();
const rooms = new Map();

io.on('connection', (socket) => {
    console.log(`User connected: ${socket.id}`);
    
    socket.on('join-room', (data) => {
        const { username, room } = data;
        
        socket.join(room);
        users.set(socket.id, { username, room });
        
        if (!rooms.has(room)) {
            rooms.set(room, new Set());
        }
        rooms.get(room).add(socket.id);
        
        socket.to(room).emit('user-joined', {
            username,
            message: `${username} joined the room`
        });
    });
    
    socket.on('message', (data) => {
        const user = users.get(socket.id);
        if (user) {
            io.to(user.room).emit('message', {
                username: user.username,
                message: data.message,
                timestamp: new Date().toISOString()
            });
        }
    });
    
    socket.on('disconnect', () => {
        const user = users.get(socket.id);
        if (user) {
            socket.to(user.room).emit('user-left', {
                username: user.username,
                message: `${user.username} left the room`
            });
            
            rooms.get(user.room)?.delete(socket.id);
            users.delete(socket.id);
        }
    });
});

server.listen(3000, () => {
    console.log('Chat server running on port 3000');
});
```

**CURSED:**
```cursed
yeet "vibe_net"
yeet "encode_mood"
yeet "timez"

vibes User struct {
    id tea
    username tea
    room tea
}

vibes Message struct {
    message_type tea
    username tea
    message tea
    timestamp tea
}

sus users map[tea]User = make(map[tea]User)
sus rooms map[tea][]tea = make(map[tea][]tea)

slay handle_websocket(conn vibe_net.WebSocketConnection) {
    user_id := conn.id()
    
    bestie {
        message := conn.read_message()
        sus msg Message
        encode_mood.json_decode(message, &msg)
        
        match msg.message_type {
            "join-room" => {
                user := User{
                    id: user_id,
                    username: msg.username,
                    room: msg.message, // room name in message field
                }
                
                users[user_id] = user
                
                // Add to room
                lowkey room_users, exists := rooms[user.room]; exists {
                    rooms[user.room] = append(room_users, user_id)
                } sus {
                    rooms[user.room] = []tea{user_id}
                }
                
                // Broadcast to room
                broadcast_message := Message{
                    message_type: "user-joined",
                    username: user.username,
                    message: user.username + " joined the room",
                    timestamp: timez.now().rfc3339(),
                }
                
                broadcast_to_room(user.room, broadcast_message, user_id)
            }
            
            "message" => {
                lowkey user, exists := users[user_id]; exists {
                    chat_message := Message{
                        message_type: "message",
                        username: user.username,
                        message: msg.message,
                        timestamp: timez.now().rfc3339(),
                    }
                    
                    broadcast_to_room(user.room, chat_message, "")
                }
            }
        }
    }
    
    // Handle disconnect
    lowkey user, exists := users[user_id]; exists {
        // Remove from room
        lowkey room_users, room_exists := rooms[user.room]; room_exists {
            new_users := make([]tea, 0)
            bestie id := range room_users {
                lowkey id != user_id {
                    new_users = append(new_users, id)
                }
            }
            rooms[user.room] = new_users
        }
        
        // Broadcast leave message
        leave_message := Message{
            message_type: "user-left",
            username: user.username,
            message: user.username + " left the room",
            timestamp: timez.now().rfc3339(),
        }
        
        broadcast_to_room(user.room, leave_message, user_id)
        delete(users, user_id)
    }
}

slay broadcast_to_room(room tea, message Message, exclude_id tea) {
    lowkey room_users, exists := rooms[room]; exists {
        message_data := encode_mood.json_encode(message)
        
        bestie user_id := range room_users {
            lowkey user_id != exclude_id {
                // Send to user (implementation depends on WebSocket library)
                vibe_net.send_to_user(user_id, message_data)
            }
        }
    }
}

slay main() {
    mux := vibe_net.new_mux()
    
    mux.handle_websocket("/chat", handle_websocket)
    
    vibez.spill("Chat server running on port 3000")
    vibe_net.listen_and_serve(":3000", mux)
}
```

### Example 3: Task Queue System

**JavaScript (Bull Queue):**
```javascript
const Queue = require('bull');
const Redis = require('ioredis');

const redis = new Redis();
const emailQueue = new Queue('email', {
    redis: { port: 6379, host: '127.0.0.1' }
});

// Job processor
emailQueue.process('send-email', async (job) => {
    const { to, subject, body } = job.data;
    
    console.log(`Sending email to ${to}...`);
    
    // Simulate email sending
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    console.log(`Email sent to ${to}`);
    
    return { success: true, timestamp: new Date().toISOString() };
});

// Add jobs to queue
function addEmailJob(emailData) {
    return emailQueue.add('send-email', emailData, {
        attempts: 3,
        backoff: 'exponential'
    });
}

// Usage
async function main() {
    // Add some email jobs
    await addEmailJob({
        to: 'alice@example.com',
        subject: 'Welcome!',
        body: 'Welcome to our service!'
    });
    
    await addEmailJob({
        to: 'bob@example.com', 
        subject: 'Newsletter',
        body: 'Here is your weekly newsletter'
    });
    
    console.log('Email jobs added to queue');
}

main().catch(console.error);
```

**CURSED:**
```cursed
yeet "timez"
yeet "encode_mood"

vibes EmailJob struct {
    to tea
    subject tea
    body tea
}

vibes JobResult struct {
    success lit
    timestamp tea
}

slay email_worker(jobs <-chan EmailJob, results chan<- JobResult) {
    bestie job := range jobs {
        vibez.spill("Sending email to ", job.to, "...")
        
        // Simulate email sending
        timez.sleep(2 * timez.second)
        
        vibez.spill("Email sent to ", job.to)
        
        results <- JobResult{
            success: based,
            timestamp: timez.now().rfc3339(),
        }
    }
}

slay add_email_job(jobs chan<- EmailJob, email_data EmailJob) {
    // Add with retry logic
    bestie attempt := 0; attempt < 3; attempt++ {
        shook {
            jobs <- email_data
            damn
        } fam err {
            lowkey attempt == 2 {
                vibez.spill("Failed to add job after 3 attempts: ", err.message())
            } sus {
                vibez.spill("Retry attempt ", attempt + 1)
                timez.sleep(timez.second * 2) // Exponential backoff
            }
        }
    }
}

slay main() {
    jobs := make(chan EmailJob, 100)
    results := make(chan JobResult, 100)
    
    // Start workers
    bestie i := 0; i < 3; i++ {
        yolo email_worker(jobs, results)
    }
    
    // Add some email jobs
    email_jobs := []EmailJob{
        EmailJob{
            to: "alice@example.com",
            subject: "Welcome!",
            body: "Welcome to our service!",
        },
        EmailJob{
            to: "bob@example.com",
            subject: "Newsletter", 
            body: "Here is your weekly newsletter",
        },
    }
    
    bestie email_job := range email_jobs {
        yolo add_email_job(jobs, email_job)
    }
    
    vibez.spill("Email jobs added to queue")
    
    // Wait for results
    bestie i := 0; i < len(email_jobs); i++ {
        result := <-results
        vibez.spill("Job result: ", result.success, " at ", result.timestamp)
    }
}
```

## Testing Your Migration

### Running Examples

```bash
# Test basic syntax
cargo run --bin cursed examples/basic_syntax.csd

# Test type system
cargo run --bin cursed examples/type_system.csd

# Test async programming
cargo run --bin cursed examples/async_demo.csd

# Test API server
cargo run --bin cursed examples/api_server.csd

# Compile to native
cargo run --bin cursed -- compile examples/performance_test.csd
./performance_test
```

### Performance Comparison

```bash
# JavaScript execution
time node fibonacci.js

# CURSED interpretation
time cargo run --bin cursed fibonacci.csd

# CURSED compilation
cargo run --bin cursed -- compile fibonacci.csd
time ./fibonacci
```

## Next Steps

1. **Master static typing**: Learn to leverage compile-time type checking
2. **Embrace compilation**: Use native performance advantages
3. **Learn goroutines**: Master structured concurrency
4. **Explore stdlib**: Use CURSED's comprehensive standard library
5. **Performance optimization**: Leverage LLVM optimizations

The migration from JavaScript to CURSED involves learning static typing and structured programming, but provides significant benefits in performance, type safety, and maintainability.
