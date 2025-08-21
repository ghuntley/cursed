# CURSED Programming Language Guide

## Welcome to CURSED - The Gen Z Programming Language 🔥

**CURSED** is a modern, high-performance systems programming language that combines the expressiveness of Gen Z slang with the power and safety of advanced language design. Built on Zig with LLVM backend, CURSED delivers blazing-fast compilation, near-C runtime performance, and a comprehensive standard library.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Syntax](#basic-syntax)
- [Data Types](#data-types)
- [Functions](#functions)
- [Control Flow](#control-flow)
- [Advanced Features](#advanced-features)
- [Standard Library](#standard-library)
- [Best Practices](#best-practices)
- [Migration Guides](#migration-guides)

## Getting Started

### Installation

```bash
# Install CURSED (Linux/macOS/Windows)
curl -sSf https://install.cursedlang.org | sh

# Verify installation
cursed-zig --version

# Create your first program
echo 'vibez.spill("Hello, world!")' > hello.csd
cursed-zig hello.csd
```

### IDE Support

**VS Code** (Recommended):
```bash
code --install-extension cursed-lang.cursed-vscode
```

**Other Editors**:
- Vim/Neovim: `vim-cursed` plugin with LSP support
- Emacs: `cursed-mode` with `lsp-mode` integration
- JetBrains: `cursed-intellij` plugin

## Basic Syntax

### Variables

CURSED uses Gen Z slang for type annotations:

```cursed
fr fr This is a comment - everything after "fr fr" is ignored

sus name tea = "CURSED Developer"      fr fr String (tea = text)
sus age drip = 25                      fr fr Integer (drip = number) 
sus height normie = 175.5             fr fr Float (normie = normal number)
sus active lit = based                 fr fr Boolean (lit = boolean, based = true)
sus inactive lit = cap                 fr fr Boolean (cap = false)
```

### Constants

```cursed
sus MAX_USERS drip = 1000              fr fr Immutable by default
```

### Variable Modifications

```cursed
sus score drip = 0
score = 100                            fr fr Reassignment
score += 50                            fr fr Compound assignment
score++                                fr fr Increment
score--                                fr fr Decrement
```

## Data Types

### Primitive Types

| CURSED Type | Traditional | Description | Example |
|-------------|-------------|-------------|---------|
| `tea` | String | Text data | `"Hello world"` |
| `drip` | Integer | Whole numbers | `42`, `-17` |
| `normie` | Float | Decimal numbers | `3.14`, `-0.5` |
| `lit` | Boolean | True/false | `based`, `cap` |

### Collections

#### Arrays
```cursed
sus numbers []drip = [1, 2, 3, 4, 5]
sus names []tea = ["Alice", "Bob", "Charlie"]
sus mixed []tea = []                   fr fr Empty array

fr fr Array operations
numbers.length()                       fr fr Get length: 5
numbers[0]                            fr fr Access element: 1
numbers.push(6)                       fr fr Add element
numbers.pop()                         fr fr Remove last element
```

#### Dynamic Arrays
```cursed
sus dynamic []drip = make_array<drip>()
dynamic.push(1)
dynamic.push(2)
dynamic.resize(10)
```

### Strings

```cursed
sus greeting tea = "Hello"
sus name tea = "World"
sus message tea = greeting + ", " + name + "!"

fr fr String methods
message.length()                       fr fr Length
message.upper()                        fr fr Convert to uppercase  
message.lower()                        fr fr Convert to lowercase
message.contains("Hello")              fr fr Check if contains substring
message.split(",")                     fr fr Split into array
```

### String Interpolation
```cursed
sus name tea = "CURSED"
sus version drip = 1
sus message tea = "Welcome to {name} v{version}!"
vibez.spill(message)                   fr fr Outputs: Welcome to CURSED v1!
```

## Functions

### Basic Functions

```cursed
slay greet(name tea) {
    vibez.spill("Hello,", name)
}

slay add(a drip, b drip) drip {
    damn a + b                         fr fr 'damn' returns a value
}

slay multiply(x drip, y drip) drip {
    damn x * y
}

fr fr Function calls
greet("World")                         fr fr Outputs: Hello, World
sus result drip = add(5, 3)           fr fr result = 8
```

### Functions with Multiple Return Values

```cursed
slay divide_with_remainder(a drip, b drip) (drip, drip) {
    sus quotient drip = a / b
    sus remainder drip = a % b
    damn (quotient, remainder)
}

sus (q, r) = divide_with_remainder(10, 3)  fr fr q = 3, r = 1
```

### Generic Functions

```cursed
slay max<T>(a T, b T) T {
    ready (a > b) {
        damn a
    }
    damn b
}

sus biggest drip = max<drip>(10, 20)      fr fr Type specified
sus largest normie = max(3.14, 2.71)     fr fr Type inferred
```

### Higher-Order Functions

```cursed
slay map<T, U>(arr []T, fn slay(T) U) []U {
    sus result []U = []
    bestie i := 0; i < arr.length(); i++ {
        result.push(fn(arr[i]))
    }
    damn result
}

slay double(x drip) drip {
    damn x * 2
}

sus numbers []drip = [1, 2, 3, 4, 5]
sus doubled []drip = map(numbers, double)  fr fr [2, 4, 6, 8, 10]
```

## Control Flow

### Conditional Statements

```cursed
sus age drip = 20

ready (age >= 18) {
    vibez.spill("You're an adult!")
} otherwise ready (age >= 13) {
    vibez.spill("You're a teenager!")
} otherwise {
    vibez.spill("You're a kid!")
}

fr fr Ternary operator
sus status tea = ready age >= 18 ? "adult" : "minor"
```

### Pattern Matching

```cursed
slay describe_number(n drip) tea {
    sick n {
        when 0 -> damn "zero"
        when 1, 2, 3 -> damn "small"
        when 4..10 -> damn "medium"  
        when x ready x > 10 -> damn "large"
        when _ -> damn "unknown"
    }
}

fr fr Advanced pattern matching with structs
sick person {
    when { name: "Alice", age: a ready a > 30 } -> {
        vibez.spill("Alice is over 30")
    }
    when { name: n, age: _ } -> {
        vibez.spill("Person named", n)
    }
}
```

### Loops

#### While Loops
```cursed
sus count drip = 0
bestie (count < 5) {
    vibez.spill("Count:", count)
    count++
}
```

#### For Loops (C-style)
```cursed
bestie i := 0; i < 10; i++ {
    vibez.spill("Iteration:", i)
}
```

#### For-Each Loops
```cursed
sus numbers []drip = [1, 2, 3, 4, 5]
bestie num in numbers {
    vibez.spill("Number:", num)
}

sus message tea = "Hello"
bestie char in message {
    vibez.spill("Character:", char)
}
```

#### Loop Control
```cursed
bestie i := 0; i < 100; i++ {
    ready (i % 2 == 0) {
        continue                       fr fr Skip even numbers
    }
    ready (i > 20) {
        break                          fr fr Exit loop
    }
    vibez.spill("Odd number:", i)
}
```

## Advanced Features

### Structs

```cursed
squad Person {
    name tea,
    age drip,
    email tea
}

fr fr Create instances
sus alice Person = Person{
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

fr fr Access fields
vibez.spill(alice.name)               fr fr Outputs: Alice
alice.age = 31                        fr fr Modify field
```

#### Struct Methods

```cursed
squad Person {
    name tea,
    age drip,
    
    slay greet(self) {
        vibez.spill("Hello, I'm", self.name)
    }
    
    slay birthday(self) {
        self.age++
        vibez.spill(self.name, "is now", self.age)
    }
    
    slay is_adult(self) lit {
        damn self.age >= 18
    }
}

sus person Person = Person{ name: "Bob", age: 25 }
person.greet()                        fr fr Outputs: Hello, I'm Bob  
person.birthday()                     fr fr Outputs: Bob is now 26
```

#### Struct Inheritance

```cursed
squad Animal {
    name tea,
    age drip,
    
    slay speak(self) {
        vibez.spill("*generic animal sound*")
    }
}

squad Dog extends Animal {
    breed tea,
    
    slay speak(self) {                fr fr Override parent method
        vibez.spill("Woof! I'm", self.name)
    }
    
    slay fetch(self) {
        vibez.spill(self.name, "is fetching!")
    }
}

sus dog Dog = Dog{
    name: "Buddy",
    age: 3,
    breed: "Golden Retriever"
}
dog.speak()                           fr fr Outputs: Woof! I'm Buddy
dog.fetch()                           fr fr Outputs: Buddy is fetching!
```

### Interfaces

```cursed
collab Drawable {
    slay draw(self)
    slay area(self) normie
}

squad Circle {
    radius normie,
    
    slay draw(self) {
        vibez.spill("Drawing circle with radius", self.radius)
    }
    
    slay area(self) normie {
        damn 3.14159 * self.radius * self.radius
    }
}

squad Rectangle {
    width normie,
    height normie,
    
    slay draw(self) {
        vibez.spill("Drawing rectangle", self.width, "x", self.height)
    }
    
    slay area(self) normie {
        damn self.width * self.height
    }
}

fr fr Polymorphism
sus shapes []Drawable = [
    Circle{ radius: 5.0 },
    Rectangle{ width: 10.0, height: 20.0 }
]

bestie shape in shapes {
    shape.draw()
    vibez.spill("Area:", shape.area())
}
```

### Enums

```cursed
enum Color {
    Red,
    Green,
    Blue,
    RGB(drip, drip, drip)
}

slay describe_color(c Color) tea {
    sick c {
        when Color.Red -> damn "red color"
        when Color.Green -> damn "green color"  
        when Color.Blue -> damn "blue color"
        when Color.RGB(r, g, b) -> damn "RGB({r}, {g}, {b})"
    }
}

sus my_color Color = Color.RGB(255, 128, 0)
vibez.spill(describe_color(my_color))     fr fr Outputs: RGB(255, 128, 0)
```

### Error Handling

CURSED uses `yikes`/`shook`/`fam` for error handling:

```cursed
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero is not allowed!"
    }
    damn a / b
}

fr fr Handle errors with 'fam' (catch)
sus result drip = divide(10, 2) fam {
    when "Division by zero is not allowed!" -> {
        vibez.spill("Error: Cannot divide by zero")
        damn 0
    }
    when _ -> {
        vibez.spill("An unexpected error occurred")
        damn -1
    }
}

fr fr Propagate errors with 'shook' (throw)
slay safe_divide(a drip, b drip) yikes<tea> {
    sus result drip = divide(a, b) fam {
        when e -> shook e              fr fr Re-throw the error
    }
    damn result
}
```

### Concurrency

#### Goroutines

```cursed
yeet "concurrenz"                     fr fr Import concurrency module

slay worker(id drip) {
    bestie i := 0; i < 5; i++ {
        vibez.spill("Worker", id, "iteration", i)
        concurrenz.sleep(100)          fr fr Sleep for 100ms
    }
}

fr fr Start goroutines
go worker(1)
go worker(2)
go worker(3)

concurrenz.sleep(1000)                fr fr Wait for workers to complete
```

#### Channels

```cursed
sus ch chan<drip> = concurrenz.make_channel<drip>()

fr fr Producer goroutine
go {
    bestie i := 0; i < 10; i++ {
        ch <- i                        fr fr Send to channel
        vibez.spill("Sent:", i)
    }
    concurrenz.close_channel(ch)
}

fr fr Consumer goroutine  
go {
    bestie based {
        sus (value, ok) = <-ch         fr fr Receive from channel
        ready (!ok) {
            break                      fr fr Channel closed
        }
        vibez.spill("Received:", value)
    }
}

concurrenz.sleep(2000)
```

#### Select Statements

```cursed
sus ch1 chan<drip> = concurrenz.make_channel<drip>()
sus ch2 chan<tea> = concurrenz.make_channel<tea>()

go {
    ch1 <- 42
}

go {
    ch2 <- "hello"
}

select {
    when value := <-ch1 -> {
        vibez.spill("Received number:", value)
    }
    when message := <-ch2 -> {
        vibez.spill("Received string:", message)
    }
    when timeout(1000) -> {
        vibez.spill("Timeout after 1 second")
    }
}
```

### Modules and Imports

```cursed
fr fr Import specific functions
yeet "mathz"
yeet "stringz"  
yeet "arrayz"

fr fr Use imported functions
sus result normie = mathz.sqrt(25.0)  fr fr result = 5.0
sus upper tea = stringz.upper("hello") fr fr upper = "HELLO"
sus sorted []drip = arrayz.sort([3, 1, 4, 1, 5])

fr fr Import with alias
yeet "networkz" as net
sus response tea = net.http_get("https://api.example.com")

fr fr Import specific items
yeet { PI, sqrt, pow } from "mathz"
sus circle_area normie = PI * pow(radius, 2)
```

## Standard Library

CURSED comes with a comprehensive standard library covering all major domains:

### Core I/O - `vibez`

```cursed
yeet "vibez"

fr fr Basic output
vibez.spill("Hello", "world")          fr fr Print with spaces
vibez.spilln("No newline")             fr fr Print without newline
vibez.error("Error message")           fr fr Print to stderr
vibez.debug("Debug info")              fr fr Debug output

fr fr Formatted output
vibez.printf("Name: %s, Age: %d", name, age)
vibez.sprintf("Formatted: %d", 42)     fr fr Return formatted string

fr fr Input operations
sus input tea = vibez.read_line()      fr fr Read line from stdin
sus number drip = vibez.read_int()     fr fr Read integer
```

### String Operations - `stringz`

```cursed
yeet "stringz"

sus text tea = "  Hello, World!  "

fr fr Basic operations
stringz.length(text)                   fr fr Get length
stringz.upper(text)                    fr fr Convert to uppercase
stringz.lower(text)                    fr fr Convert to lowercase
stringz.trim(text)                     fr fr Remove whitespace
stringz.contains(text, "World")        fr fr Check substring
stringz.starts_with(text, "Hello")     fr fr Check prefix
stringz.ends_with(text, "!")           fr fr Check suffix

fr fr Advanced operations
stringz.replace(text, "World", "CURSED")
stringz.split(text, ",")               fr fr Split into array
stringz.join(["Hello", "World"], " ")  fr fr Join array with delimiter
stringz.reverse(text)                  fr fr Reverse string
```

### Array Operations - `arrayz`

```cursed
yeet "arrayz"

sus numbers []drip = [5, 2, 8, 1, 9]

fr fr Basic operations
arrayz.length(numbers)                 fr fr Get length
arrayz.push(numbers, 3)                fr fr Add element
arrayz.pop(numbers)                    fr fr Remove last element
arrayz.insert(numbers, 2, 7)           fr fr Insert at index
arrayz.remove(numbers, 1)              fr fr Remove at index

fr fr Functional operations
arrayz.map(numbers, slay(x) { damn x * 2 })
arrayz.filter(numbers, slay(x) { damn x > 5 })
arrayz.reduce(numbers, slay(a, b) { damn a + b }, 0)
arrayz.sort(numbers)                   fr fr Sort in place
arrayz.reverse(numbers)                fr fr Reverse in place

fr fr Search operations
arrayz.find(numbers, 5)                fr fr Find first occurrence
arrayz.contains(numbers, 3)            fr fr Check if contains
arrayz.index_of(numbers, 8)            fr fr Get index of element
```

### Mathematics - `mathz`

```cursed
yeet "mathz"

fr fr Constants
mathz.PI                               fr fr 3.14159...
mathz.E                                fr fr 2.71828...

fr fr Basic operations
mathz.abs(-5)                          fr fr Absolute value: 5
mathz.min(3, 7)                        fr fr Minimum: 3
mathz.max(3, 7)                        fr fr Maximum: 7
mathz.clamp(15, 0, 10)                 fr fr Clamp to range: 10

fr fr Power and root functions
mathz.sqrt(25.0)                       fr fr Square root: 5.0
mathz.pow(2.0, 3.0)                    fr fr Power: 8.0
mathz.exp(1.0)                         fr fr e^x: 2.71828

fr fr Trigonometric functions
mathz.sin(mathz.PI / 2)                fr fr Sine: 1.0
mathz.cos(0.0)                         fr fr Cosine: 1.0  
mathz.tan(mathz.PI / 4)                fr fr Tangent: 1.0

fr fr Random numbers
mathz.random()                         fr fr Random float 0.0-1.0
mathz.random_int(1, 10)                fr fr Random integer 1-10
mathz.random_choice([1, 2, 3, 4, 5])   fr fr Random array element
```

### File Operations - `filez`

```cursed
yeet "filez"

fr fr Reading files
sus content tea = filez.read_text("config.txt")
sus bytes []drip = filez.read_bytes("image.png")
sus lines []tea = filez.read_lines("data.txt")

fr fr Writing files
filez.write_text("output.txt", "Hello, world!")
filez.write_bytes("binary.dat", [0x48, 0x65, 0x6C, 0x6C, 0x6F])
filez.append_text("log.txt", "New log entry\n")

fr fr File information
filez.exists("file.txt")               fr fr Check if file exists
filez.size("file.txt")                 fr fr Get file size
filez.is_dir("folder")                 fr fr Check if directory
filez.modified_time("file.txt")        fr fr Last modified timestamp

fr fr Directory operations
filez.list_dir(".")                    fr fr List directory contents
filez.create_dir("new_folder")         fr fr Create directory
filez.remove("old_file.txt")           fr fr Delete file
filez.copy("source.txt", "dest.txt")   fr fr Copy file
```

### Networking - `networkz`

```cursed
yeet "networkz"

fr fr HTTP client
sus response tea = networkz.http_get("https://api.github.com/users/octocat")
sus json_data tea = networkz.http_post(
    "https://httpbin.org/post",
    "application/json", 
    '{"message": "Hello!"}'
)

fr fr TCP server
sus server TCPServer = networkz.tcp_listen("127.0.0.1", 8080)
vibez.spill("Server listening on port 8080...")

bestie based {
    sus client TCPConnection = server.accept()
    go {
        sus message tea = client.read_line()
        vibez.spill("Received:", message)
        client.write("Echo: " + message + "\n")
        client.close()
    }
}

fr fr UDP communication
sus udp_socket UDPSocket = networkz.udp_bind("127.0.0.1", 9090)
udp_socket.send_to("127.0.0.1", 9091, "Hello UDP!")
sus (data, addr) = udp_socket.receive_from()
```

### JSON Processing - `jsonz`

```cursed
yeet "jsonz"

fr fr Parse JSON
sus json_text tea = '{"name": "Alice", "age": 30, "active": true}'
sus data JsonValue = jsonz.parse(json_text)

fr fr Access JSON data  
sus name tea = data["name"].as_string()       fr fr "Alice"
sus age drip = data["age"].as_int()          fr fr 30
sus active lit = data["active"].as_bool()    fr fr true

fr fr Generate JSON
sus person JsonValue = jsonz.object([
    ("name", jsonz.string("Bob")),
    ("age", jsonz.number(25)),
    ("hobbies", jsonz.array([
        jsonz.string("reading"),
        jsonz.string("coding")
    ]))
])
sus json_output tea = jsonz.stringify(person)
```

### Testing Framework - `testz`

```cursed
yeet "testz"

fr fr Test functions
slay test_addition() {
    testz.assert_eq(2 + 2, 4, "Basic addition")
    testz.assert_ne(2 + 2, 5, "Addition inequality")
    testz.assert_true(5 > 3, "Comparison test")
    testz.assert_false(1 > 5, "False comparison")
}

slay test_string_operations() {
    sus result tea = "hello".upper()
    testz.assert_eq(result, "HELLO", "String uppercase")
    
    testz.assert_contains("hello world", "world", "Substring check")
}

fr fr Run tests
testz.run_test("Addition Test", test_addition)
testz.run_test("String Test", test_string_operations)
testz.print_summary()
```

### Database Integration - `dbz`

```cursed
yeet "dbz"

fr fr Connect to database
sus db Database = dbz.connect("postgresql://user:pass@localhost/mydb")

fr fr Execute queries
sus users []User = db.query<User>("SELECT * FROM users WHERE age > ?", [18])
sus user_count drip = db.scalar<drip>("SELECT COUNT(*) FROM users")

fr fr Insert data
sus new_user User = User{
    name: "Charlie",
    email: "charlie@example.com", 
    age: 28
}
db.insert("users", new_user)

fr fr Transactions
db.begin_transaction()
db.execute("UPDATE users SET active = ? WHERE id = ?", [true, 123])
db.execute("INSERT INTO logs (message) VALUES (?)", ["User activated"])
db.commit()
```

### Cryptography - `cryptz`

```cursed
yeet "cryptz"

fr fr Hashing
sus password tea = "secret123"
sus hash tea = cryptz.sha256(password)
sus bcrypt_hash tea = cryptz.bcrypt(password, 10)

fr fr Symmetric encryption
sus key []drip = cryptz.generate_key(32)
sus plaintext tea = "Sensitive information"
sus encrypted []drip = cryptz.aes_encrypt(plaintext, key)
sus decrypted tea = cryptz.aes_decrypt(encrypted, key)

fr fr Digital signatures
sus (private_key, public_key) = cryptz.generate_keypair()
sus message tea = "Important message"
sus signature []drip = cryptz.sign(message, private_key)
sus valid lit = cryptz.verify(message, signature, public_key)
```

## Best Practices

### Code Organization

1. **Use descriptive variable names with Gen Z slang appropriately**:
```cursed
sus user_count drip = 0                fr fr Good
sus n drip = 0                         fr fr Avoid single letters
```

2. **Keep functions small and focused**:
```cursed
slay calculate_total_price(items []Item) normie {
    sus total normie = 0.0
    bestie item in items {
        total += item.price * (1.0 + item.tax_rate)
    }
    damn total
}
```

3. **Use meaningful struct and interface names**:
```cursed
squad DatabaseConnection { ... }       fr fr Clear intent
collab Serializable { ... }           fr fr Clear capability
```

### Error Handling

1. **Handle errors explicitly**:
```cursed
sus result drip = risky_operation() fam {
    when "NetworkError" -> {
        vibez.error("Network connection failed")
        damn -1
    }
    when "TimeoutError" -> {
        vibez.error("Operation timed out")
        damn -2  
    }
    when _ -> {
        vibez.error("Unknown error occurred")
        damn -3
    }
}
```

2. **Use specific error types**:
```cursed
enum DatabaseError {
    ConnectionFailed,
    QueryTimeout,
    ConstraintViolation(tea)
}

slay query_user(id drip) yikes<DatabaseError> {
    ready (!is_connected()) {
        yikes DatabaseError.ConnectionFailed
    }
    fr fr ... query logic
}
```

### Performance

1. **Use appropriate data structures**:
```cursed
fr fr For frequent insertions/deletions, use dynamic arrays
sus dynamic_list []tea = make_array<tea>()

fr fr For fixed-size collections, use regular arrays
sus fixed_buffer [1024]drip = [0; 1024]
```

2. **Minimize allocations in loops**:
```cursed
fr fr Pre-allocate outside the loop
sus results []tea = make_array_with_capacity<tea>(1000)
bestie i := 0; i < 1000; i++ {
    results.push(process_item(i))
}
```

3. **Use goroutines for I/O-bound operations**:
```cursed
sus results chan<Result> = make_channel<Result>()

bestie url in urls {
    go {
        sus response tea = networkz.http_get(url)
        results <- process_response(response)
    }
}
```

### Memory Management

1. **Prefer stack allocation when possible**:
```cursed
fr fr Stack allocated (fast)
sus buffer [256]drip = [0; 256]

fr fr Heap allocated when needed
sus large_buffer []drip = make_array_with_capacity<drip>(1000000)
```

2. **Use defer for cleanup**:
```cursed
slay process_file(filename tea) yikes<tea> {
    sus file File = filez.open(filename) fam {
        when e -> shook e
    }
    defer file.close()                 fr fr Ensures file is closed
    
    fr fr Process file...
}
```

## Migration Guides

### From Go

| Go Concept | CURSED Equivalent | Example |
|------------|-------------------|---------|
| `var x int` | `sus x drip` | `sus count drip = 0` |
| `func name() {}` | `slay name() {}` | `slay greet() { ... }` |
| `return value` | `damn value` | `damn result` |
| `if condition {}` | `ready (condition) {}` | `ready (x > 0) { ... }` |
| `for i := 0; i < 10; i++` | `bestie i := 0; i < 10; i++` | Same syntax |
| `go function()` | `go function()` | Same syntax |
| `make(chan int)` | `make_channel<drip>()` | Explicit type parameter |
| `fmt.Println()` | `vibez.spill()` | `vibez.spill("Hello")` |

### From Rust

| Rust Concept | CURSED Equivalent | Example |
|-------------|-------------------|---------|
| `let x: i32` | `sus x drip` | `sus count drip = 42` |
| `fn name() {}` | `slay name() {}` | `slay calculate() { ... }` |
| `return value` | `damn value` | `damn result * 2` |
| `if condition {}` | `ready (condition) {}` | `ready (valid) { ... }` |
| `for item in items` | `bestie item in items` | `bestie num in numbers` |
| `Result<T, E>` | `yikes<E>` | `slay divide() yikes<tea>` |
| `println!()` | `vibez.spill()` | `vibez.spill("Debug:", value)` |
| `Vec<T>` | `[]T` | `sus items []drip = []` |
| `String` | `tea` | `sus message tea = "hello"` |

### From Python

| Python Concept | CURSED Equivalent | Example |
|-------------|-------------------|---------|
| `x = 42` | `sus x drip = 42` | Explicit type annotation |
| `def name():` | `slay name() {` | Braces instead of indentation |  
| `return value` | `damn value` | `damn calculated_result` |
| `if condition:` | `ready (condition) {` | Parentheses and braces |
| `for item in items:` | `bestie item in items {` | Braces for body |
| `try/except` | `fam { when e -> }` | `divide(a, b) fam { when "ZeroDivision" -> ... }` |
| `print()` | `vibez.spill()` | `vibez.spill("Value:", x)` |
| `list` | `[]T` | `sus numbers []drip = [1, 2, 3]` |
| `str` | `tea` | `sus name tea = "Alice"` |

### From JavaScript

| JavaScript Concept | CURSED Equivalent | Example |
|-------------|-------------------|---------|
| `let x = 42` | `sus x drip = 42` | Explicit types |
| `function name() {}` | `slay name() {}` | Similar syntax |
| `return value` | `damn value` | `damn computed_value` |
| `if (condition) {}` | `ready (condition) {}` | `ready` instead of `if` |
| `for (let i = 0; i < 10; i++)` | `bestie i := 0; i < 10; i++` | `bestie` instead of `for` |
| `try/catch` | `fam { when e -> }` | `operation() fam { when e -> handle(e) }` |
| `console.log()` | `vibez.spill()` | `vibez.spill("Debug:", data)` |
| `[]` | `[]T` | `sus items []tea = []` |
| `string` | `tea` | `sus text tea = "content"` |

## Advanced Topics

### Memory Management Patterns

CURSED provides several memory management strategies:

```cursed
fr fr Arena allocation for temporary data
sus arena Arena = Arena.init()
defer arena.deinit()

sus temp_data []drip = arena.alloc_array<drip>(1000)
fr fr Automatically freed when arena is deinitialized

fr fr Pool allocation for frequent allocations
sus pool ObjectPool<User> = ObjectPool.init<User>()
sus user User = pool.acquire()
defer pool.release(user)
```

### Custom Operators

```cursed
fr fr Define custom operators for your types
squad Vector2 {
    x normie,
    y normie,
    
    fr fr Addition operator
    slay +(self, other Vector2) Vector2 {
        damn Vector2{ x: self.x + other.x, y: self.y + other.y }
    }
    
    fr fr Scalar multiplication
    slay *(self, scalar normie) Vector2 {
        damn Vector2{ x: self.x * scalar, y: self.y * scalar }
    }
}

sus v1 Vector2 = Vector2{ x: 1.0, y: 2.0 }
sus v2 Vector2 = Vector2{ x: 3.0, y: 4.0 }
sus result Vector2 = v1 + v2 * 2.0     fr fr Custom operators in action
```

### Reflection and Meta-programming

```cursed
yeet "reflectz"

slay describe_type<T>() {
    sus type_info TypeInfo = reflectz.type_of<T>()
    vibez.spill("Type name:", type_info.name)
    vibez.spill("Size:", type_info.size)
    
    ready (type_info.is_struct()) {
        bestie field in type_info.fields() {
            vibez.spill("Field:", field.name, "Type:", field.type_name)
        }
    }
}

describe_type<Person>()               fr fr Prints struct information
```

### Async/Await Pattern

```cursed
yeet "asyncz"

slay async fetch_user_data(user_id drip) asyncz.Future<User> {
    sus response tea = await networkz.async_http_get("/users/{user_id}")
    sus user User = await jsonz.async_parse<User>(response)
    damn user
}

slay async process_users() {
    sus user_ids []drip = [1, 2, 3, 4, 5]
    sus futures []asyncz.Future<User> = []
    
    bestie id in user_ids {
        futures.push(fetch_user_data(id))
    }
    
    sus users []User = await asyncz.all(futures)
    bestie user in users {
        vibez.spill("Processed user:", user.name)
    }
}
```

## Conclusion

CURSED combines the expressiveness of modern slang with the power of systems programming. Its comprehensive standard library, excellent performance characteristics, and rich feature set make it ideal for:

- **Systems Programming**: Operating systems, compilers, and low-level tools
- **Web Development**: High-performance APIs and microservices
- **Game Development**: Real-time applications and game engines
- **Data Processing**: High-throughput data pipelines and analytics
- **Machine Learning**: Performance-critical ML model serving
- **Education**: Teaching modern programming concepts with engaging syntax

**Key Advantages:**
- ⚡ **Blazing Fast**: Sub-second compilation, near-C runtime performance
- 🛡️ **Memory Safe**: Zero memory leaks with comprehensive safety guarantees
- 🌐 **Cross-Platform**: Single codebase targeting all major platforms
- 🔧 **Rich Tooling**: Professional IDE integration and developer experience
- 📚 **Complete Ecosystem**: Comprehensive standard library covering all domains
- 🎯 **Modern Design**: Clean syntax that's both fun and functional

Start building with CURSED today and experience the future of systems programming! 🚀

---

**Documentation Version**: 1.0.0  
**Last Updated**: August 21, 2025  
**For More Information**: Visit [cursedlang.org](https://cursedlang.org) or join our [Discord community](https://discord.gg/cursed-lang)
