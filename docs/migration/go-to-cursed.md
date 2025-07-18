# Migrating from Go to CURSED

CURSED is heavily inspired by Go's syntax and concepts, making this one of the most natural transitions. However, CURSED adds modern Gen Z terminology and several enhancements while maintaining Go's philosophy of simplicity and readability.

## Table of Contents

1. [Quick Syntax Reference](#quick-syntax-reference)
2. [Core Language Features](#core-language-features)
3. [Type System Differences](#type-system-differences)
4. [Concurrency Model](#concurrency-model)
5. [Package System](#package-system)
6. [Error Handling](#error-handling)
7. [Migration Strategy](#migration-strategy)
8. [Common Pitfalls](#common-pitfalls)
9. [Working Examples](#working-examples)

## Quick Syntax Reference

### Basic Syntax Translation

| Go Concept | Go Syntax | CURSED Syntax | CURSED Keyword |
|------------|-----------|---------------|----------------|
| Variable declaration | `var x int` | `sus x normie` | `sus` |
| Short declaration | `x := 42` | `x := 42` | Same |
| Function | `func name() {}` | `slay name() {}` | `slay` |
| Return | `return value` | `damn value` | `damn` |
| If statement | `if condition {}` | `lowkey condition {}` | `lowkey` |
| For loop | `for i := 0; i < 10; i++` | `bestie i := 0; i < 10; i++` | `bestie` |
| Print | `fmt.Println()` | `vibez.spill()` | `vibez.spill` |
| Import | `import "fmt"` | `yeet "fmt"` | `yeet` |
| Boolean true | `true` | `based` | `based` |
| Boolean false | `false` | `cap` | `cap` |
| String type | `string` | `tea` | `tea` |
| Integer type | `int` | `normie` | `normie` |
| Boolean type | `bool` | `lit` | `lit` |

### Side-by-Side Comparison

**Go:**
```go
package main

import "fmt"

func main() {
    var message string = "Hello, World!"
    count := 42
    
    if count > 0 {
        fmt.Println(message)
    }
    
    for i := 0; i < count; i++ {
        fmt.Printf("Count: %d\n", i)
    }
}
```

**CURSED:**
```cursed
vibe main

yeet "vibez"

slay main() {
    sus message tea = "Hello, World!"
    count := 42
    
    lowkey count > 0 {
        vibez.spill(message)
    }
    
    bestie i := 0; i < count; i++ {
        vibez.spill("Count: ", i)
    }
}
```

## Core Language Features

### 1. Variables and Types

**Go:**
```go
var x int = 42
var name string = "Alice"
var isActive bool = true
var price float64 = 19.99

// Short declaration
y := 100
```

**CURSED:**
```cursed
sus x normie = 42
sus name tea = "Alice"
sus isActive lit = based
sus price meal = 19.99

// Short declaration (same)
y := 100
```

### 2. Functions

**Go:**
```go
func add(a int, b int) int {
    return a + b
}

func greet(name string) {
    fmt.Printf("Hello, %s!\n", name)
}
```

**CURSED:**
```cursed
slay add(a normie, b normie) normie {
    damn a + b
}

slay greet(name tea) {
    vibez.spill("Hello, ", name, "!")
}
```

### 3. Structs and Methods

**Go:**
```go
type Person struct {
    Name string
    Age  int
}

func (p Person) Greet() {
    fmt.Printf("Hi, I'm %s\n", p.Name)
}

func (p *Person) SetAge(age int) {
    p.Age = age
}
```

**CURSED:**
```cursed
vibes Person struct {
    Name tea
    Age  normie
}

slay (p Person) Greet() {
    vibez.spill("Hi, I'm ", p.Name)
}

slay (p *Person) SetAge(age normie) {
    p.Age = age
}
```

### 4. Interfaces

**Go:**
```go
type Writer interface {
    Write([]byte) (int, error)
}

type Speaker interface {
    Speak() string
}
```

**CURSED:**
```cursed
vibes Writer interface {
    Write([byte]) (normie, error)
}

vibes Speaker interface {
    Speak() tea
}
```

## Type System Differences

### Primitive Types

| Go Type | CURSED Type | Description |
|---------|-------------|-------------|
| `int` | `normie` | 32-bit signed integer |
| `int8` | `smol` | 8-bit signed integer |
| `int16` | `mid` | 16-bit signed integer |
| `int64` | `thicc` | 64-bit signed integer |
| `uint8` | `byte` | 8-bit unsigned integer |
| `float32` | `drip` | 32-bit float |
| `float64` | `meal` | 64-bit float |
| `string` | `tea` | String type |
| `bool` | `lit` | Boolean type |
| `rune` | `sip` | Character type |

### Type Assertions

**Go:**
```go
var i interface{} = 42
value, ok := i.(int)
if ok {
    fmt.Println("Value:", value)
}
```

**CURSED:**
```cursed
sus i interface{} = 42
value, ok := i.(normie)
lowkey ok {
    vibez.spill("Value:", value)
}
```

### Arrays and Slices

**Go:**
```go
var arr [5]int
var slice []int = make([]int, 5)
slice = append(slice, 10)
```

**CURSED:**
```cursed
sus arr [5]normie
sus slice []normie = make([]normie, 5)
slice = append(slice, 10)
```

## Concurrency Model

### Goroutines vs Yolo-routines

**Go:**
```go
func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        fmt.Printf("Worker %d processing job %d\n", id, j)
        results <- j * 2
    }
}

func main() {
    jobs := make(chan int, 100)
    results := make(chan int, 100)
    
    // Start workers
    for w := 1; w <= 3; w++ {
        go worker(w, jobs, results)
    }
    
    // Send jobs
    for j := 1; j <= 5; j++ {
        jobs <- j
    }
    close(jobs)
    
    // Collect results
    for r := 1; r <= 5; r++ {
        <-results
    }
}
```

**CURSED:**
```cursed
slay worker(id normie, jobs <-chan normie, results chan<- normie) {
    bestie j := range jobs {
        vibez.spill("Worker ", id, " processing job ", j)
        results <- j * 2
    }
}

slay main() {
    jobs := make(chan normie, 100)
    results := make(chan normie, 100)
    
    // Start workers
    bestie w := 1; w <= 3; w++ {
        yolo worker(w, jobs, results)  // yolo instead of go
    }
    
    // Send jobs
    bestie j := 1; j <= 5; j++ {
        jobs <- j
    }
    close(jobs)
    
    // Collect results
    bestie r := 1; r <= 5; r++ {
        <-results
    }
}
```

### Select Statements

**Go:**
```go
select {
case msg1 := <-ch1:
    fmt.Println("Received:", msg1)
case msg2 := <-ch2:
    fmt.Println("Received:", msg2)
default:
    fmt.Println("No message received")
}
```

**CURSED:**
```cursed
ready {
    msg1 := <-ch1:
        vibez.spill("Received:", msg1)
    msg2 := <-ch2:
        vibez.spill("Received:", msg2)
    basic:
        vibez.spill("No message received")
}
```

## Package System

### Import and Package Declaration

**Go:**
```go
package main

import (
    "fmt"
    "strings"
    "net/http"
)
```

**CURSED:**
```cursed
vibe main

yeet (
    "vibez"
    "stringz"
    "net/http"
)
```

### Module Definition

**Go:**
```go
package mypackage

func PublicFunction() {
    fmt.Println("This is public")
}

func privateFunction() {
    fmt.Println("This is private")
}
```

**CURSED:**
```cursed
vibe mypackage

slay PublicFunction() {
    vibez.spill("This is public")
}

slay privateFunction() {
    vibez.spill("This is private")
}
```

## Error Handling

### Traditional Go Error Handling

**Go:**
```go
func divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, fmt.Errorf("division by zero")
    }
    return a / b, nil
}

func main() {
    result, err := divide(10, 0)
    if err != nil {
        fmt.Println("Error:", err)
        return
    }
    fmt.Println("Result:", result)
}
```

**CURSED (Traditional Style):**
```cursed
slay divide(a meal, b meal) (meal, error) {
    lowkey b == 0 {
        damn 0, fmt.Errorf("division by zero")
    }
    damn a / b, cringe
}

slay main() {
    result, err := divide(10, 0)
    lowkey err != cringe {
        vibez.spill("Error:", err)
        damn
    }
    vibez.spill("Result:", result)
}
```

### CURSED Enhanced Error Handling

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
        result := divide(10, 0)
        vibez.spill("Result:", result)
    } fam err {
        vibez.spill("Error:", err)
    }
}
```

## Migration Strategy

### Phase 1: Syntax Translation
1. **Replace keywords**: Use find/replace for basic keyword conversion
2. **Update imports**: Change `import` to `yeet`
3. **Update types**: Replace primitive types with CURSED equivalents
4. **Update functions**: Change `func` to `slay`, `return` to `damn`

### Phase 2: Leverage CURSED Features
1. **Enhanced error handling**: Use `yikes`/`shook`/`fam` for better error flow
2. **Improved concurrency**: Use `yolo` for goroutines, `ready` for select
3. **Modern stdlib**: Leverage CURSED's comprehensive standard library

### Phase 3: Optimization
1. **Memory management**: Take advantage of CURSED's GC improvements
2. **Performance**: Use CURSED's LLVM backend optimizations
3. **Tooling**: Adopt CURSED's development tools

## Common Pitfalls

### 1. Keyword Confusion
**Problem:** Forgetting to use CURSED keywords
```go
// Go (old)
if x > 0 {
    return x
}
```

**Solution:** Use CURSED keywords
```cursed
// CURSED (correct)
lowkey x > 0 {
    damn x
}
```

### 2. Type Name Confusion
**Problem:** Using Go type names
```go
// Go (old)
var count int
var message string
```

**Solution:** Use CURSED type names
```cursed
// CURSED (correct)
sus count normie
sus message tea
```

### 3. Boolean Literal Confusion
**Problem:** Using true/false
```go
// Go (old)
var isActive bool = true
```

**Solution:** Use based/cap
```cursed
// CURSED (correct)
sus isActive lit = based
```

### 4. Print Function Confusion
**Problem:** Using fmt.Println
```go
// Go (old)
fmt.Println("Hello")
```

**Solution:** Use vibez.spill
```cursed
// CURSED (correct)
vibez.spill("Hello")
```

## Working Examples

### Example 1: HTTP Server

**Go:**
```go
package main

import (
    "fmt"
    "net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
}

func main() {
    http.HandleFunc("/", handler)
    fmt.Println("Server starting on :8080")
    http.ListenAndServe(":8080", nil)
}
```

**CURSED:**
```cursed
vibe main

yeet (
    "vibez"
    "net/http"
)

slay handler(w http.ResponseWriter, r *http.Request) {
    vibez.spillf(w, "Hello, %s!", r.URL.Path[1:])
}

slay main() {
    http.HandleFunc("/", handler)
    vibez.spill("Server starting on :8080")
    http.ListenAndServe(":8080", cringe)
}
```

### Example 2: Worker Pool

**Go:**
```go
package main

import (
    "fmt"
    "time"
)

func worker(id int, jobs <-chan int, results chan<- int) {
    for j := range jobs {
        fmt.Printf("Worker %d started job %d\n", id, j)
        time.Sleep(time.Second)
        fmt.Printf("Worker %d finished job %d\n", id, j)
        results <- j * 2
    }
}

func main() {
    const numJobs = 5
    const numWorkers = 3
    
    jobs := make(chan int, numJobs)
    results := make(chan int, numJobs)
    
    for w := 1; w <= numWorkers; w++ {
        go worker(w, jobs, results)
    }
    
    for j := 1; j <= numJobs; j++ {
        jobs <- j
    }
    close(jobs)
    
    for r := 1; r <= numJobs; r++ {
        <-results
    }
}
```

**CURSED:**
```cursed
vibe main

yeet (
    "vibez"
    "timez"
)

slay worker(id normie, jobs <-chan normie, results chan<- normie) {
    bestie j := range jobs {
        vibez.spill("Worker ", id, " started job ", j)
        timez.sleep(timez.second)
        vibez.spill("Worker ", id, " finished job ", j)
        results <- j * 2
    }
}

slay main() {
    numJobs := 5
    numWorkers := 3
    
    jobs := make(chan normie, numJobs)
    results := make(chan normie, numJobs)
    
    bestie w := 1; w <= numWorkers; w++ {
        yolo worker(w, jobs, results)
    }
    
    bestie j := 1; j <= numJobs; j++ {
        jobs <- j
    }
    close(jobs)
    
    bestie r := 1; r <= numJobs; r++ {
        <-results
    }
}
```

### Example 3: JSON API

**Go:**
```go
package main

import (
    "encoding/json"
    "fmt"
    "net/http"
)

type User struct {
    ID   int    `json:"id"`
    Name string `json:"name"`
}

func getUser(w http.ResponseWriter, r *http.Request) {
    user := User{ID: 1, Name: "Alice"}
    
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(user)
}

func main() {
    http.HandleFunc("/user", getUser)
    fmt.Println("Server starting on :8080")
    http.ListenAndServe(":8080", nil)
}
```

**CURSED:**
```cursed
vibe main

yeet (
    "encode_mood"
    "vibez"
    "net/http"
)

vibes User struct {
    ID   normie `json:"id"`
    Name tea    `json:"name"`
}

slay getUser(w http.ResponseWriter, r *http.Request) {
    user := User{ID: 1, Name: "Alice"}
    
    w.Header().Set("Content-Type", "application/json")
    encode_mood.json_encode(w, user)
}

slay main() {
    http.HandleFunc("/user", getUser)
    vibez.spill("Server starting on :8080")
    http.ListenAndServe(":8080", cringe)
}
```

## Testing Your Migration

### Running Examples

```bash
# Test basic syntax
cargo run --bin cursed examples/basic_syntax.csd

# Test concurrency
cargo run --bin cursed examples/worker_pool.csd

# Test HTTP server
cargo run --bin cursed examples/http_server.csd

# Compile to native
cargo run --bin cursed -- compile examples/basic_syntax.csd
./basic_syntax
```

### Verification Checklist

- [ ] All Go keywords replaced with CURSED equivalents
- [ ] Type names updated to CURSED types
- [ ] Import statements use `yeet` instead of `import`
- [ ] Functions use `slay` instead of `func`
- [ ] Return statements use `damn` instead of `return`
- [ ] Conditional statements use `lowkey` instead of `if`
- [ ] Loop statements use `bestie` instead of `for`
- [ ] Goroutines use `yolo` instead of `go`
- [ ] Select statements use `ready` instead of `select`
- [ ] Boolean literals use `based`/`cap` instead of `true`/`false`
- [ ] Print statements use `vibez.spill` instead of `fmt.Println`

## Next Steps

1. **Explore CURSED stdlib**: Check out the comprehensive standard library
2. **Advanced features**: Learn about CURSED's enhanced error handling and pattern matching
3. **Performance optimization**: Leverage LLVM backend optimizations
4. **Community**: Join the CURSED community for support and best practices

The transition from Go to CURSED should be smooth due to the similar syntax and concepts. The main differences are keyword changes and some enhanced features that make CURSED even more powerful and expressive!
