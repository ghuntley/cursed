# Migrating from Rust to CURSED

Coming from Rust to CURSED involves shifting from manual memory management to garbage collection, while maintaining strong type safety and performance. CURSED offers a more approachable syntax with similar safety guarantees.

## Table of Contents

1. [Philosophy Differences](#philosophy-differences)
2. [Memory Management](#memory-management)
3. [Type System Comparison](#type-system-comparison)
4. [Syntax Translation](#syntax-translation)
5. [Concurrency Models](#concurrency-models)
6. [Error Handling](#error-handling)
7. [Pattern Matching](#pattern-matching)
8. [Migration Strategy](#migration-strategy)
9. [Common Pitfalls](#common-pitfalls)
10. [Working Examples](#working-examples)

## Philosophy Differences

### Rust Philosophy
- **Zero-cost abstractions**: Performance at compile time
- **Memory safety**: Through ownership and borrowing
- **Fearless concurrency**: Safe concurrency through type system
- **Explicit control**: Manual memory management

### CURSED Philosophy
- **Productive abstractions**: Performance through simplicity
- **Memory safety**: Through garbage collection and type safety
- **Approachable concurrency**: Safe concurrency through runtime
- **Implicit control**: Automatic memory management

## Memory Management

### Ownership vs Garbage Collection

**Rust:**
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved, no longer valid
    // println!("{}", s1); // Error: value borrowed here after move
    println!("{}", s2);
}
```

**CURSED:**
```cursed
slay main() {
    sus s1 tea = "hello"
    sus s2 tea = s1  // s1 is still valid, GC handles memory
    vibez.spill(s1)  // Still works
    vibez.spill(s2)  // Also works
}
```

### References and Borrowing

**Rust:**
```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}
```

**CURSED:**
```cursed
slay calculate_length(s tea) normie {
    damn s.len()
}

slay main() {
    sus s1 tea = "hello"
    len := calculate_length(s1)  // No explicit borrowing needed
    vibez.spill("Length of '", s1, "' is ", len)
}
```

### Mutable References

**Rust:**
```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}
```

**CURSED:**
```cursed
slay change(s *tea) {
    *s = *s + ", world"
}

slay main() {
    sus s tea = "hello"
    change(&s)  // Pass reference for mutation
    vibez.spill(s)
}
```

## Type System Comparison

### Primitive Types

| Rust Type | CURSED Type | Description |
|-----------|-------------|-------------|
| `i8` | `smol` | 8-bit signed integer |
| `i16` | `mid` | 16-bit signed integer |
| `i32` | `normie` | 32-bit signed integer |
| `i64` | `thicc` | 64-bit signed integer |
| `u8` | `byte` | 8-bit unsigned integer |
| `f32` | `drip` | 32-bit float |
| `f64` | `meal` | 64-bit float |
| `String` | `tea` | String type |
| `bool` | `lit` | Boolean type |
| `char` | `sip` | Character type |

### Option Types

**Rust:**
```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match divide(10.0, 3.0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Cannot divide by zero"),
    }
}
```

**CURSED:**
```cursed
slay divide(a meal, b meal) meal {
    lowkey b == 0.0 {
        yikes "division by zero"
    }
    damn a / b
}

slay main() {
    shook {
        result := divide(10.0, 3.0)
        vibez.spill("Result: ", result)
    } fam err {
        vibez.spill("Cannot divide by zero")
    }
}
```

### Structs and Enums

**Rust:**
```rust
struct Person {
    name: String,
    age: u32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}
```

**CURSED:**
```cursed
vibes Person struct {
    name tea
    age normie
}

vibes Message enum {
    Quit
    Move { x normie, y normie }
    Write(tea)
}

slay (p Person) new(name tea, age normie) Person {
    damn Person { name: name, age: age }
}

slay (p Person) greet() {
    vibez.spill("Hello, I'm ", p.name)
}
```

### Traits vs Interfaces

**Rust:**
```rust
trait Display {
    fn fmt(&self) -> String;
}

trait Debug {
    fn debug(&self) -> String;
}

impl Display for Person {
    fn fmt(&self) -> String {
        format!("{} ({})", self.name, self.age)
    }
}

impl Debug for Person {
    fn debug(&self) -> String {
        format!("Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}
```

**CURSED:**
```cursed
vibes Display interface {
    fmt() tea
}

vibes Debug interface {
    debug() tea
}

slay (p Person) fmt() tea {
    damn p.name + " (" + p.age.to_string() + ")"
}

slay (p Person) debug() tea {
    damn "Person { name: " + p.name + ", age: " + p.age.to_string() + " }"
}
```

## Syntax Translation

### Variables and Mutability

**Rust:**
```rust
let x = 5;              // Immutable
let mut y = 10;         // Mutable
y = 15;                 // OK

const MAX_SIZE: usize = 100;  // Constant
```

**CURSED:**
```cursed
sus x normie = 5        // Immutable by default
sus y normie = 10       // Can be reassigned
y = 15                  // OK

facts MAX_SIZE normie = 100  // Constant
```

### Functions

**Rust:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_hello() {
    println!("Hello!");
}

fn main() {
    let result = add(5, 3);
    println!("Result: {}", result);
    print_hello();
}
```

**CURSED:**
```cursed
slay add(a normie, b normie) normie {
    damn a + b
}

slay print_hello() {
    vibez.spill("Hello!")
}

slay main() {
    result := add(5, 3)
    vibez.spill("Result: ", result)
    print_hello()
}
```

### Control Flow

**Rust:**
```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
    
    // Loop
    for i in 0..5 {
        println!("Number: {}", i);
    }
    
    // While loop
    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}
```

**CURSED:**
```cursed
slay main() {
    number := 6
    
    lowkey number % 4 == 0 {
        vibez.spill("number is divisible by 4")
    } sus lowkey number % 3 == 0 {
        vibez.spill("number is divisible by 3")
    } sus {
        vibez.spill("number is not divisible by 4 or 3")
    }
    
    // Loop
    bestie i := 0; i < 5; i++ {
        vibez.spill("Number: ", i)
    }
    
    // While loop
    sus counter normie = 0
    bestie counter < 3 {
        vibez.spill("Counter: ", counter)
        counter++
    }
}
```

## Concurrency Models

### Threads vs Goroutines

**Rust:**
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}
```

**CURSED:**
```cursed
yeet "timez"

slay main() {
    yolo {
        bestie i := 1; i < 10; i++ {
            vibez.spill("hi number ", i, " from spawned goroutine!")
            timez.sleep(timez.millisecond)
        }
    }
    
    bestie i := 1; i < 5; i++ {
        vibez.spill("hi number ", i, " from main goroutine!")
        timez.sleep(timez.millisecond)
    }
    
    timez.sleep(timez.second)  // Wait for goroutine to finish
}
```

### Channels

**Rust:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

**CURSED:**
```cursed
slay main() {
    ch := make(chan tea)
    
    yolo {
        val := "hi"
        ch <- val
    }
    
    received := <-ch
    vibez.spill("Got: ", received)
}
```

### Message Passing

**Rust:**
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}
```

**CURSED:**
```cursed
yeet "timez"

slay main() {
    ch := make(chan tea)
    
    yolo {
        vals := ["hi", "from", "the", "goroutine"]
        
        bestie val := range vals {
            ch <- val
            timez.sleep(timez.second)
        }
        close(ch)
    }
    
    bestie received := range ch {
        vibez.spill("Got: ", received)
    }
}
```

## Error Handling

### Result Types vs Enhanced Error Handling

**Rust:**
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}
```

**CURSED:**
```cursed
yeet "dropz"

slay read_file(filename tea) tea {
    shook {
        file := dropz.open(filename)
        contents := file.read_all()
        file.close()
        damn contents
    } fam err {
        yikes err
    }
}

slay main() {
    shook {
        contents := read_file("hello.txt")
        vibez.spill("File contents: ", contents)
    } fam error {
        vibez.spill("Error reading file: ", error)
    }
}
```

### Panic vs Yikes

**Rust:**
```rust
fn main() {
    let v = vec![1, 2, 3];
    
    // This will panic
    v[99];
}
```

**CURSED:**
```cursed
slay main() {
    v := [1, 2, 3]
    
    // This will trigger error handling
    lowkey len(v) <= 99 {
        yikes "index out of bounds"
    }
    
    vibez.spill(v[99])
}
```

## Pattern Matching

### Match vs Switch

**Rust:**
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value: {}", value_in_cents(coin));
}
```

**CURSED:**
```cursed
vibes Coin enum {
    Penny
    Nickel
    Dime
    Quarter
}

slay value_in_cents(coin Coin) byte {
    match coin {
        Coin.Penny => damn 1
        Coin.Nickel => damn 5
        Coin.Dime => damn 10
        Coin.Quarter => damn 25
    }
}

slay main() {
    coin := Coin.Dime
    vibez.spill("Value: ", value_in_cents(coin))
}
```

### Option Pattern Matching

**Rust:**
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}", six);
    println!("{:?}", none);
}
```

**CURSED:**
```cursed
slay plus_one(x Option[normie]) Option[normie] {
    match x {
        cringe => damn cringe
        Some(i) => damn Some(i + 1)
    }
}

slay main() {
    five := Some(5)
    six := plus_one(five)
    none := plus_one(cringe)
    
    vibez.spill(six)
    vibez.spill(none)
}
```

## Migration Strategy

### Phase 1: Conceptual Mapping
1. **Identify ownership patterns**: Map Rust ownership to CURSED GC semantics
2. **Convert error handling**: Replace `Result<T, E>` with `yikes`/`shook`/`fam`
3. **Simplify lifetimes**: Remove lifetime annotations (GC handles this)
4. **Map concurrency**: Convert threads to goroutines, channels remain similar

### Phase 2: Syntax Translation
1. **Keywords**: `fn` → `slay`, `let` → `sus`, `return` → `damn`
2. **Types**: `i32` → `normie`, `String` → `tea`, `bool` → `lit`
3. **Control flow**: `if` → `lowkey`, `for` → `bestie`, `match` → `match`
4. **Concurrency**: `thread::spawn` → `yolo`, keep channel syntax

### Phase 3: Leverage CURSED Features
1. **Simplified memory management**: Remove explicit borrowing
2. **Enhanced error handling**: Use CURSED's error propagation
3. **Comprehensive stdlib**: Use CURSED's extensive standard library
4. **Performance**: Leverage LLVM optimizations

## Common Pitfalls

### 1. Over-thinking Memory Management
**Problem:** Trying to manually manage memory like in Rust
```rust
// Rust thinking (unnecessary in CURSED)
let s = String::from("hello");
let s_ref = &s;  // Explicit borrowing
```

**Solution:** Trust the GC
```cursed
// CURSED approach
sus s tea = "hello"
sus s_copy tea = s  // GC handles everything
```

### 2. Overly Complex Error Handling
**Problem:** Trying to use Result-like patterns
```rust
// Rust approach
match result {
    Ok(value) => println!("{}", value),
    Err(e) => println!("Error: {}", e),
}
```

**Solution:** Use CURSED's enhanced error handling
```cursed
// CURSED approach
shook {
    value := operation()
    vibez.spill(value)
} fam e {
    vibez.spill("Error: ", e)
}
```

### 3. Lifetime Confusion
**Problem:** Trying to specify lifetimes (not needed in CURSED)
```rust
// Rust (with lifetimes)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**Solution:** CURSED doesn't need lifetimes
```cursed
// CURSED (no lifetimes needed)
slay longest(x tea, y tea) tea {
    lowkey x.len() > y.len() {
        damn x
    } sus {
        damn y
    }
}
```

### 4. Ownership Transfer Confusion
**Problem:** Expecting move semantics
```rust
// Rust behavior
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved
// println!("{}", s1);  // Error
```

**Solution:** CURSED uses copying/references
```cursed
// CURSED behavior
sus s1 tea = "hello"
sus s2 tea = s1  // s1 is still usable
vibez.spill(s1)  // Works fine
```

## Working Examples

### Example 1: Web Server

**Rust:**
```rust
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
```

**CURSED:**
```cursed
yeet "vibe_net"

slay handle_connection(stream vibe_net.TcpStream) {
    buffer := make([]byte, 1024)
    stream.read(buffer)
    
    response := "HTTP/1.1 200 OK\r\n\r\nHello from CURSED!"
    stream.write(response.as_bytes())
    stream.flush()
}

slay main() {
    listener := vibe_net.listen("127.0.0.1:7878")
    
    bestie stream := range listener.incoming() {
        handle_connection(stream)
    }
}
```

### Example 2: Concurrent Calculator

**Rust:**
```rust
use std::sync::mpsc;
use std::thread;

enum Operation {
    Add(i32, i32),
    Multiply(i32, i32),
    Quit,
}

fn calculator(rx: mpsc::Receiver<Operation>) {
    loop {
        match rx.recv().unwrap() {
            Operation::Add(a, b) => println!("Add: {}", a + b),
            Operation::Multiply(a, b) => println!("Multiply: {}", a * b),
            Operation::Quit => break,
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        calculator(rx);
    });
    
    tx.send(Operation::Add(10, 20)).unwrap();
    tx.send(Operation::Multiply(5, 6)).unwrap();
    tx.send(Operation::Quit).unwrap();
}
```

**CURSED:**
```cursed
vibes Operation enum {
    Add(normie, normie)
    Multiply(normie, normie)
    Quit
}

slay calculator(rx <-chan Operation) {
    bestie {
        ready {
            op := <-rx:
                match op {
                    Operation.Add(a, b) => vibez.spill("Add: ", a + b)
                    Operation.Multiply(a, b) => vibez.spill("Multiply: ", a * b)
                    Operation.Quit => damn
                }
        }
    }
}

slay main() {
    ch := make(chan Operation)
    
    yolo calculator(ch)
    
    ch <- Operation.Add(10, 20)
    ch <- Operation.Multiply(5, 6)
    ch <- Operation.Quit
}
```

### Example 3: File Processing

**Rust:**
```rust
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn process_file(filename: &str) -> Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);
    }
    
    Ok(())
}

fn main() {
    match process_file("input.txt") {
        Ok(_) => println!("File processed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
```

**CURSED:**
```cursed
yeet "dropz"

slay process_file(filename tea) {
    shook {
        file := dropz.open(filename)
        defer file.close()
        
        lines := file.read_lines()
        bestie line := range lines {
            vibez.spill("Line: ", line)
        }
    } fam err {
        yikes err
    }
}

slay main() {
    shook {
        process_file("input.txt")
        vibez.spill("File processed successfully")
    } fam e {
        vibez.spill("Error: ", e)
    }
}
```

## Testing Your Migration

### Running Examples

```bash
# Test basic syntax
cargo run --bin cursed examples/basic_syntax.csd

# Test memory management
cargo run --bin cursed examples/memory_demo.csd

# Test concurrency
cargo run --bin cursed examples/concurrent_calculator.csd

# Test error handling
cargo run --bin cursed examples/error_handling.csd

# Compile to native
cargo run --bin cursed -- compile examples/web_server.csd
./web_server
```

### Performance Comparison

```bash
# Rust build
cargo build --release

# CURSED build
cargo run --bin cursed -- compile --optimize program.csd

# Compare execution times
time ./rust_program
time ./cursed_program
```

## Next Steps

1. **Embrace GC**: Don't fight the garbage collector, leverage it
2. **Simplify error handling**: Use CURSED's enhanced error system
3. **Explore stdlib**: CURSED has a comprehensive standard library
4. **Performance tuning**: Use LLVM optimizations for performance
5. **Community**: Join the CURSED community for Rust migration tips

The migration from Rust to CURSED involves trading some performance control for development velocity and simplicity, while maintaining strong type safety and modern language features.
