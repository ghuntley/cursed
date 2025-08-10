# 🦀 ➡️ 🔥 Migrating from Rust to CURSED

This guide helps Rust developers transition to CURSED, highlighting similarities and differences.

## 🎯 Quick Comparison

| Concept | Rust | CURSED |
|---------|------|--------|
| Variables | `let x = 5;` | `sus x drip = 5` |
| Mutable | `let mut x = 5;` | `sus x drip = 5` (mutable by default) |
| Immutable | `let x = 5;` | `lock x drip = 5` |
| Functions | `fn add(a: i32, b: i32) -> i32` | `slay add(a drip, b drip) drip` |
| Strings | `String`, `&str` | `tea` |
| Integers | `i32`, `i64`, `u32`, etc. | `drip` (64-bit signed) |
| Booleans | `bool`, `true`/`false` | `lit`, `based`/`cap` |
| Arrays | `Vec<T>`, `[T; N]` | `[]T` |
| Error handling | `Result<T, E>`, `?` | `yikes<E>`, `fam`, `?` |
| Pattern matching | `match` | `sick` |
| Comments | `//`, `/* */` | `fr`, `/* */` |

## 📦 Data Types Migration

### Basic Types

```rust
// Rust
let number: i32 = 42;
let float: f64 = 3.14;
let text: String = "Hello".to_string();
let flag: bool = true;
```

```cursed
fr CURSED
sus number drip = 42
sus float meal = 3.14
sus text tea = "Hello"
sus flag lit = based
```

### Collections

```rust
// Rust
let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("key".to_string(), 42);
```

```cursed
fr CURSED
sus numbers []drip = [1, 2, 3, 4, 5]
sus map map<tea, drip> = {"key": 42}
```

### Custom Types

```rust
// Rust
struct Person {
    name: String,
    age: u32,
}

enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}
```

```cursed
fr CURSED
squad Person {
    name tea
    age drip
}

sick Color {
    Red,
    Green,
    Blue,
    RGB(drip, drip, drip)
}
```

## ⚙️ Functions

### Basic Functions

```rust
// Rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```cursed
fr CURSED
slay greet(name tea) {
    vibez.spill("Hello,", name, "!")
}

slay add(a drip, b drip) drip {
    damn a + b
}
```

### Generics

```rust
// Rust
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Container<T> {
    value: T,
}
```

```cursed
fr CURSED
slay max<T: Comparable>(a T, b T) T {
    ready (a > b) { damn a }
    damn b
}

squad Container<T> {
    value T
}
```

### Closures

```rust
// Rust
let add_one = |x| x + 1;
let numbers: Vec<i32> = vec![1, 2, 3];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

```cursed
fr CURSED
sus add_one = slay(x drip) drip { damn x + 1 }
sus numbers []drip = [1, 2, 3]
sus doubled []drip = arrayz.map(numbers, slay(x drip) drip { damn x * 2 })
```

## 🔄 Control Flow

### Conditionals

```rust
// Rust
if age >= 18 {
    println!("Adult");
} else if age >= 13 {
    println!("Teen");
} else {
    println!("Child");
}

let result = if condition { "yes" } else { "no" };
```

```cursed
fr CURSED
ready (age >= 18) {
    vibez.spill("Adult")
} otherwise ready (age >= 13) {
    vibez.spill("Teen")
} otherwise {
    vibez.spill("Child")
}

sus result tea = condition ? "yes" : "no"
```

### Loops

```rust
// Rust
for i in 0..10 {
    println!("{}", i);
}

for item in &items {
    println!("{}", item);
}

let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}
```

```cursed
fr CURSED
bestie (i in 0..10) {
    vibez.spill(i)
}

bestie (item in items) {
    vibez.spill(item)
}

sus i drip = 0
bestie (i < 10) {
    vibez.spill(i)
    i = i + 1
}
```

### Pattern Matching

```rust
// Rust
match value {
    1 => println!("One"),
    2 => println!("Two"),
    3..=10 => println!("Between 3 and 10"),
    _ => println!("Something else"),
}

match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

```cursed
fr CURSED
sick value {
    1 -> vibez.spill("One")
    2 -> vibez.spill("Two")
    3..=10 -> vibez.spill("Between 3 and 10")
    _ -> vibez.spill("Something else")
}

sick result {
    Ok(value) -> vibez.spill("Success:", value)
    Error(error) -> vibez.spill("Error:", error)
}
```

## ⚠️ Error Handling

### Result Types

```rust
// Rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = divide(10, 2)?;
    println!("Result: {}", result);
    Ok(())
}
```

```cursed
fr CURSED
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "Division by zero"
    }
    damn a / b
}

slay main() yikes<tea> {
    sus result drip = divide(10, 2)?
    vibez.spill("Result:", result)
}
```

### Error Propagation and Handling

```rust
// Rust
match divide(10, 0) {
    Ok(result) => println!("Success: {}", result),
    Err(error) => println!("Error: {}", error),
}

// Using ? operator
let result = divide(10, 2)?;
```

```cursed
fr CURSED
sus result drip = divide(10, 0) fam {
    when "Division by zero" -> {
        vibez.spill("Cannot divide by zero!")
        damn 0
    }
    when error -> {
        vibez.spill("Error:", error)
        damn -1
    }
}

fr Using ? operator
sus result drip = divide(10, 2)?
```

## 📦 Modules and Crates

### Module System

```rust
// Rust
// In lib.rs or main.rs
mod math_utils;
use math_utils::add;

// In math_utils.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

use std::collections::HashMap;
use std::fs::File;
```

```cursed
fr CURSED
fr Import modules
yeet "math_utils"
yeet "math_utils" { add }

fr In math_utils.csd
slay pub add(a drip, b drip) drip {
    damn a + b
}

yeet "collections"
yeet "filez"
```

### Package Management

```toml
# Rust - Cargo.toml
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

```toml
# CURSED - CursedPackage.toml
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
jsonz = "1.0"
networkz = { version = "1.0", features = ["tls"] }
```

## 🔄 Concurrency

### Threads vs Goroutines

```rust
// Rust
use std::thread;
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("Hello from thread").unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received);
```

```cursed
fr CURSED
yeet "concurrenz"

sus ch chan<tea> = make_channel()

go {
    ch <- "Hello from goroutine"
}

sus received tea = <-ch
vibez.spill(received)
```

### Async/Await

```rust
// Rust
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{}", result);
}

async fn fetch_data() -> String {
    // Async operation
    "Data".to_string()
}
```

```cursed
fr CURSED
yeet "asyncz"

slay main() {
    sus result tea = await fetch_data()
    vibez.spill(result)
}

slay async fetch_data() Promise<tea> {
    fr Async operation
    damn "Data"
}
```

## 🛡️ Memory Management

### Ownership vs Garbage Collection

```rust
// Rust - Ownership and borrowing
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved, no longer valid
    
    let s3 = String::from("world");
    let len = calculate_length(&s3); // Borrowing
    println!("{} has length {}", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

```cursed
fr CURSED - Automatic memory management
slay main() {
    sus s1 tea = "hello"
    sus s2 tea = s1    # Both s1 and s2 are valid
    
    sus s3 tea = "world"
    sus len drip = calculate_length(s3)  # No borrowing needed
    vibez.spill(s3, "has length", len)
}

slay calculate_length(s tea) drip {
    damn len(s)
}
```

### RAII and Cleanup

```rust
// Rust
use std::fs::File;

fn read_file() -> std::io::Result<String> {
    let file = File::open("data.txt")?;
    // File automatically closed when going out of scope
    // ... read operations
    Ok(content)
}
```

```cursed
fr CURSED
yeet "filez"

slay read_file() yikes<tea> {
    sus file = filez.open("data.txt")?
    defer file.close()  # Explicit cleanup
    
    sus content tea = file.read()?
    damn content
}
```

## 🧪 Testing

### Unit Tests

```rust
// Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0).unwrap();
    }
}
```

```cursed
fr CURSED
yeet "testz"

slay test_add() {
    testz.assert_eq_int(add(2, 2), 4)
}

slay test_divide_by_zero() {
    fr Test should handle error gracefully
    sus result = divide(10, 0) fam {
        when _ -> damn 0
    }
    testz.assert_eq_int(result, 0)
}

testz.test_start("Math Tests")
test_add()
test_divide_by_zero()
testz.print_test_summary()
```

## 📊 Performance Considerations

### Compilation Speed

```bash
# Rust
cargo build          # 10-60 seconds for medium projects
cargo build --release

# CURSED
zig build            # 0.1-0.2 seconds for medium projects
cursed --compile --optimize=3
```

### Runtime Performance

| Aspect | Rust | CURSED |
|--------|------|--------|
| Memory safety | Zero-cost abstractions | GC + arena allocators |
| Execution speed | ~100% of C | ~80-90% of C |
| Memory usage | Minimal overhead | ~60-70% of C |
| Startup time | Fast | <10ms typical |
| Binary size | Small (with optimizations) | Small to medium |

## 🚀 Migration Strategy

### 1. Start with Simple Functions
Begin by converting pure functions that don't use advanced Rust features:

```rust
// Rust
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

```cursed
fr CURSED
slay fibonacci(n drip) drip {
    sick n {
        0 -> damn 0
        1 -> damn 1
        _ -> damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}
```

### 2. Convert Data Structures
Transform Rust structs and enums to CURSED equivalents:

```rust
// Rust
#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
    active: bool,
}

enum Status {
    Active,
    Inactive,
    Suspended(String),
}
```

```cursed
fr CURSED
squad User {
    id drip
    name tea
    email tea
    active lit
}

sick Status {
    Active,
    Inactive,
    Suspended(tea)
}
```

### 3. Replace Error Handling
Convert Rust's `Result` types to CURSED's error system:

```rust
// Rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}
```

```cursed
fr CURSED
yeet "stringz"

slay parse_number(s tea) yikes<tea> {
    damn stringz.parse_int(s)?
}
```

### 4. Update Concurrency Code
Replace Rust's async/await and channels:

```rust
// Rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    tokio::spawn(async move {
        tx.send("message").await.unwrap();
    });
    
    if let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}
```

```cursed
fr CURSED
yeet "concurrenz"

slay main() {
    sus ch chan<tea> = make_channel(100)
    
    go {
        ch <- "message"
    }
    
    sus msg tea = <-ch
    vibez.spill(msg)
}
```

## 🔧 Tool Equivalents

| Rust Tool | CURSED Equivalent | Purpose |
|-----------|-------------------|---------|
| `cargo` | `cursed` CLI | Package management |
| `rustc` | `cursed --compile` | Compilation |
| `cargo test` | `cursed test` | Testing |
| `cargo fmt` | `cursed format` | Code formatting |
| `cargo clippy` | `cursed lint` | Linting |
| `cargo doc` | `cursed doc` | Documentation |
| `rust-analyzer` | `cursed-lsp` | Language server |

## 📚 Learning Resources

1. **Hands-on Practice**: Start with [CURSED Examples](../../examples/)
2. **Language Guide**: Read the [Language Reference](../user-guide/language-reference.md)
3. **Standard Library**: Explore [API Documentation](../api/)
4. **Community**: Join [Discord](https://discord.gg/cursed-lang) for help

## 🎯 Common Pitfalls

### 1. Memory Management Mindset
- **Rust**: Think about ownership, borrowing, lifetimes
- **CURSED**: Trust the GC, use `defer` for cleanup

### 2. Error Handling
- **Rust**: `Result<T, E>` everywhere
- **CURSED**: `yikes<E>` for errors, `fam` for handling

### 3. String Types
- **Rust**: `String` vs `&str` distinction
- **CURSED**: Just `tea` - much simpler!

### 4. Concurrency
- **Rust**: Ownership-based thread safety
- **CURSED**: Channel-based communication, GC handles sharing

The migration from Rust to CURSED is straightforward for most code patterns. CURSED's simpler memory model and syntax make it easier to write and maintain code, while still providing the performance and safety features you expect from a modern language.
