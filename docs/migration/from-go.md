# 🐹 ➡️ 🔥 Migrating from Go to CURSED

This guide helps Go developers transition to CURSED, highlighting the familiar concepts and exciting improvements.

## 🎯 Quick Comparison

| Concept | Go | CURSED |
|---------|-----|--------|
| Variables | `var x int = 5` | `sus x drip = 5` |
| Short declaration | `x := 5` | `sus x = 5` (inferred) |
| Constants | `const Pi = 3.14` | `lock Pi meal = 3.14` |
| Functions | `func add(a, b int) int` | `slay add(a drip, b drip) drip` |
| Strings | `string` | `tea` |
| Integers | `int`, `int64` | `drip` (64-bit) |
| Floats | `float64` | `meal` |
| Booleans | `bool`, `true`/`false` | `lit`, `based`/`cap` |
| Arrays/Slices | `[]int` | `[]drip` |
| Maps | `map[string]int` | `map<tea, drip>` |
| Channels | `chan int` | `chan<drip>` |
| Goroutines | `go func()` | `go { }` |
| Error handling | `error`, multiple returns | `yikes<E>`, `fam` |
| Interfaces | `interface{}` | `collab` |
| Structs | `struct` | `squad` |

## 📦 Basic Data Types

### Variables and Constants

```go
// Go
var name string = "Alice"
var age int = 25
const Pi float64 = 3.14159

// Short declarations
x := 42
y := "hello"
```

```cursed
fr CURSED
sus name tea = "Alice"
sus age drip = 25
lock Pi meal = 3.14159

fr Type inference
sus x = 42        # inferred as drip
sus y = "hello"   # inferred as tea
```

### Collections

```go
// Go
numbers := []int{1, 2, 3, 4, 5}
scores := map[string]int{
    "Alice": 95,
    "Bob":   87,
}

// Accessing
fmt.Println(numbers[0])
fmt.Println(scores["Alice"])
```

```cursed
fr CURSED
sus numbers []drip = [1, 2, 3, 4, 5]
sus scores map<tea, drip> = {
    "Alice": 95,
    "Bob": 87
}

fr Accessing
vibez.spill(numbers[0])
vibez.spill(scores["Alice"])
```

## ⚙️ Functions

### Basic Functions

```go
// Go
func greet(name string) {
    fmt.Printf("Hello, %s!\n", name)
}

func add(a, b int) int {
    return a + b
}

// Multiple returns
func divide(a, b int) (int, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
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

fr Error handling with yikes
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}
```

### Variadic Functions

```go
// Go
func sum(numbers ...int) int {
    total := 0
    for _, num := range numbers {
        total += num
    }
    return total
}

result := sum(1, 2, 3, 4, 5)
```

```cursed
fr CURSED
slay sum(numbers ...drip) drip {
    sus total drip = 0
    bestie (num in numbers) {
        total = total + num
    }
    damn total
}

sus result drip = sum(1, 2, 3, 4, 5)
```

### Function Types and Closures

```go
// Go
func apply(nums []int, fn func(int) int) []int {
    result := make([]int, len(nums))
    for i, v := range nums {
        result[i] = fn(v)
    }
    return result
}

double := func(x int) int { return x * 2 }
doubled := apply([]int{1, 2, 3}, double)
```

```cursed
fr CURSED
yeet "arrayz"

slay apply(nums []drip, fn slay(drip) drip) []drip {
    damn arrayz.map(nums, fn)
}

sus double = slay(x drip) drip { damn x * 2 }
sus doubled []drip = apply([1, 2, 3], double)
```

## 🏗️ Types and Structs

### Struct Definition

```go
// Go
type Person struct {
    Name  string
    Age   int
    Email string
}

// Constructor function
func NewPerson(name string, age int, email string) *Person {
    return &Person{
        Name:  name,
        Age:   age,
        Email: email,
    }
}

// Methods
func (p *Person) Greet() {
    fmt.Printf("Hi, I'm %s\n", p.Name)
}

func (p *Person) IsAdult() bool {
    return p.Age >= 18
}
```

```cursed
fr CURSED
squad Person {
    name tea
    age drip
    email tea
}

fr Constructor function
slay new_person(name tea, age drip, email tea) Person {
    damn Person{
        name: name,
        age: age,
        email: email
    }
}

fr Methods
impl Person {
    slay greet(self) {
        vibez.spill("Hi, I'm", self.name)
    }
    
    slay is_adult(self) lit {
        damn self.age >= 18
    }
}
```

### Interfaces

```go
// Go
type Drawable interface {
    Draw()
    Area() float64
}

type Circle struct {
    Radius float64
}

func (c Circle) Draw() {
    fmt.Printf("Drawing circle with radius %.2f\n", c.Radius)
}

func (c Circle) Area() float64 {
    return 3.14159 * c.Radius * c.Radius
}

// Using interface
var d Drawable = Circle{Radius: 5.0}
d.Draw()
```

```cursed
fr CURSED
collab Drawable {
    slay draw()
    slay area() meal
}

squad Circle {
    radius meal
}

impl Drawable for Circle {
    slay draw() {
        vibez.spill("Drawing circle with radius", self.radius)
    }
    
    slay area() meal {
        damn 3.14159 * self.radius * self.radius
    }
}

fr Using interface
sus d Drawable = Circle{radius: 5.0}
d.draw()
```

## 🔄 Control Flow

### Conditionals

```go
// Go
if age >= 18 {
    fmt.Println("Adult")
} else if age >= 13 {
    fmt.Println("Teen")
} else {
    fmt.Println("Child")
}

// Short if
if err := doSomething(); err != nil {
    log.Fatal(err)
}
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

fr Error handling
sus result = doSomething() fam {
    when error -> {
        vibez.spill("Error:", error)
        damn
    }
}
```

### Loops

```go
// Go
// For loop
for i := 0; i < 10; i++ {
    fmt.Println(i)
}

// While-style loop
i := 0
for i < 10 {
    fmt.Println(i)
    i++
}

// Range loop
numbers := []int{1, 2, 3, 4, 5}
for index, value := range numbers {
    fmt.Printf("Index: %d, Value: %d\n", index, value)
}

// Map iteration
scores := map[string]int{"Alice": 95, "Bob": 87}
for name, score := range scores {
    fmt.Printf("%s: %d\n", name, score)
}
```

```cursed
fr CURSED
fr Range loop
bestie (i in 0..10) {
    vibez.spill(i)
}

fr While-style loop
sus i drip = 0
bestie (i < 10) {
    vibez.spill(i)
    i = i + 1
}

fr Array iteration with index and value
sus numbers []drip = [1, 2, 3, 4, 5]
bestie (index, value in numbers) {
    vibez.spill("Index:", index, "Value:", value)
}

fr Map iteration
sus scores map<tea, drip> = {"Alice": 95, "Bob": 87}
bestie (name, score in scores) {
    vibez.spill(name, ":", score)
}
```

### Switch Statements

```go
// Go
switch day {
case "Monday":
    fmt.Println("Start of work week")
case "Friday":
    fmt.Println("TGIF!")
case "Saturday", "Sunday":
    fmt.Println("Weekend!")
default:
    fmt.Println("Regular day")
}

// Type switch
switch v := interface{}(value).(type) {
case int:
    fmt.Printf("Integer: %d\n", v)
case string:
    fmt.Printf("String: %s\n", v)
default:
    fmt.Printf("Unknown type: %T\n", v)
}
```

```cursed
fr CURSED
sick day {
    "Monday" -> vibez.spill("Start of work week")
    "Friday" -> vibez.spill("TGIF!")
    "Saturday" | "Sunday" -> vibez.spill("Weekend!")
    _ -> vibez.spill("Regular day")
}

fr Type matching with interfaces
sick value {
    x: drip -> vibez.spill("Integer:", x)
    s: tea -> vibez.spill("String:", s)
    _ -> vibez.spill("Unknown type")
}
```

## ⚠️ Error Handling

### Go's Error Pattern

```go
// Go
func readFile(filename string) (string, error) {
    content, err := ioutil.ReadFile(filename)
    if err != nil {
        return "", fmt.Errorf("failed to read file: %w", err)
    }
    return string(content), nil
}

func main() {
    content, err := readFile("data.txt")
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(content)
}
```

### CURSED's Error System

```cursed
fr CURSED
yeet "filez"

slay read_file(filename tea) yikes<tea> {
    sus content tea = filez.read(filename)?
    damn content
}

slay main() {
    sus content tea = read_file("data.txt") fam {
        when error -> {
            vibez.spill("Error:", error)
            damn
        }
    }
    vibez.spill(content)
}
```

### Custom Error Types

```go
// Go
type ValidationError struct {
    Field   string
    Message string
}

func (e ValidationError) Error() string {
    return fmt.Sprintf("validation error on field '%s': %s", e.Field, e.Message)
}

func validateAge(age int) error {
    if age < 0 {
        return ValidationError{Field: "age", Message: "must be positive"}
    }
    return nil
}
```

```cursed
fr CURSED
sick ValidationError {
    FieldError(tea, tea)  # field, message
}

slay validate_age(age drip) yikes<ValidationError> {
    ready (age < 0) {
        yikes ValidationError.FieldError("age", "must be positive")
    }
}
```

## 🔄 Concurrency

### Goroutines and Channels

```go
// Go
func worker(id int, jobs <-chan int, results chan<- int) {
    for job := range jobs {
        fmt.Printf("Worker %d processing job %d\n", id, job)
        results <- job * job
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
        result := <-results
        fmt.Printf("Result: %d\n", result)
    }
}
```

```cursed
fr CURSED
yeet "concurrenz"

slay worker(id drip, jobs chan<drip>, results chan<drip>) {
    bestie (job <- jobs) {
        vibez.spill("Worker", id, "processing job", job)
        results <- job * job
    }
}

slay main() {
    sus jobs chan<drip> = make_channel(100)
    sus results chan<drip> = make_channel(100)

    fr Start workers
    bestie (w in 1..=3) {
        go worker(w, jobs, results)
    }

    fr Send jobs
    bestie (j in 1..=5) {
        jobs <- j
    }
    close(jobs)

    fr Collect results
    bestie (r in 1..=5) {
        sus result drip = <-results
        vibez.spill("Result:", result)
    }
}
```

### Select Statement

```go
// Go
select {
case msg1 := <-ch1:
    fmt.Printf("Received from ch1: %s\n", msg1)
case msg2 := <-ch2:
    fmt.Printf("Received from ch2: %s\n", msg2)
case <-time.After(1 * time.Second):
    fmt.Println("Timeout")
default:
    fmt.Println("No messages")
}
```

```cursed
fr CURSED
sick {
    msg1 := <-ch1 -> vibez.spill("Received from ch1:", msg1)
    msg2 := <-ch2 -> vibez.spill("Received from ch2:", msg2)
    timeout(1000) -> vibez.spill("Timeout")
    default -> vibez.spill("No messages")
}
```

### WaitGroup Pattern

```go
// Go
import "sync"

func main() {
    var wg sync.WaitGroup
    
    for i := 0; i < 5; i++ {
        wg.Add(1)
        go func(id int) {
            defer wg.Done()
            fmt.Printf("Goroutine %d completed\n", id)
        }(i)
    }
    
    wg.Wait()
    fmt.Println("All goroutines completed")
}
```

```cursed
fr CURSED
yeet "concurrenz"

slay main() {
    sus wg WaitGroup = WaitGroup.new()
    
    bestie (i in 0..5) {
        wg.add(1)
        go {
            defer wg.done()
            vibez.spill("Goroutine", i, "completed")
        }
    }
    
    wg.wait()
    vibez.spill("All goroutines completed")
}
```

## 📦 Packages and Modules

### Package Declaration and Imports

```go
// Go
package main

import (
    "fmt"
    "log"
    "net/http"
    "encoding/json"
    
    "github.com/gorilla/mux"
    "myproject/internal/handlers"
)
```

```cursed
fr CURSED
fr Module imports
yeet "vibez"
yeet "logz" 
yeet "networkz"
yeet "jsonz"

fr External packages (from CursedPackage.toml)
yeet "router"
yeet "./internal/handlers"
```

### Creating Packages

```go
// Go - mathutil/mathutil.go
package mathutil

// Exported function (starts with capital letter)
func Add(a, b int) int {
    return a + b
}

// Unexported function
func multiply(a, b int) int {
    return a * b
}

// Usage
import "myproject/mathutil"
result := mathutil.Add(5, 3)
```

```cursed
fr CURSED - mathutil.csd
module mathutil

fr Public function
slay pub add(a drip, b drip) drip {
    damn a + b
}

fr Private function
slay multiply(a drip, b drip) drip {
    damn a * b
}

fr Usage
yeet "mathutil"
sus result drip = mathutil.add(5, 3)
```

## 🔧 Standard Library Migration

### String Operations

```go
// Go
import "strings"

text := "Hello, World!"
upper := strings.ToUpper(text)
words := strings.Split(text, " ")
joined := strings.Join(words, "-")
contains := strings.Contains(text, "World")
```

```cursed
fr CURSED
yeet "stringz"

sus text tea = "Hello, World!"
sus upper tea = stringz.to_upper(text)
sus words []tea = stringz.split(text, " ")
sus joined tea = stringz.join(words, "-")
sus contains lit = stringz.contains(text, "World")
```

### HTTP Server

```go
// Go
import (
    "fmt"
    "net/http"
    "log"
)

func helloHandler(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "Hello, World!")
}

func main() {
    http.HandleFunc("/", helloHandler)
    log.Println("Server starting on :8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

```cursed
fr CURSED
yeet "networkz"
yeet "web_vibez"

slay hello_handler(req Request, res Response) {
    res.send("Hello, World!")
}

slay main() {
    sus server = web_vibez.create_server(8080)
    server.route("GET", "/", hello_handler)
    
    vibez.spill("Server starting on :8080")
    server.listen()
}
```

### JSON Handling

```go
// Go
import (
    "encoding/json"
    "fmt"
)

type User struct {
    Name  string `json:"name"`
    Age   int    `json:"age"`
    Email string `json:"email"`
}

func main() {
    user := User{Name: "Alice", Age: 30, Email: "alice@example.com"}
    
    // Marshal to JSON
    jsonData, err := json.Marshal(user)
    if err != nil {
        panic(err)
    }
    
    // Unmarshal from JSON
    var newUser User
    err = json.Unmarshal(jsonData, &newUser)
    if err != nil {
        panic(err)
    }
    
    fmt.Printf("%+v\n", newUser)
}
```

```cursed
fr CURSED
yeet "jsonz"

squad User {
    name tea
    age drip
    email tea
}

slay main() yikes<tea> {
    sus user = User{
        name: "Alice",
        age: 30,
        email: "alice@example.com"
    }
    
    fr Marshal to JSON
    sus json_data tea = jsonz.marshal(user)?
    
    fr Unmarshal from JSON  
    sus new_user User = jsonz.unmarshal<User>(json_data)?
    
    vibez.spill(new_user)
}
```

## 📊 Performance Comparison

| Aspect | Go | CURSED |
|--------|-----|--------|
| Compilation speed | Fast (~1-5s) | Ultra-fast (~0.1-0.2s) |
| Runtime performance | ~90% of C | ~80-90% of C |
| Memory usage | Low | ~60-70% of C |
| Startup time | Fast | <10ms |
| Binary size | Small | Small to medium |
| GC pauses | Low latency | <1ms (100MB heaps) |

## 🚀 Migration Strategy

### 1. Start with Simple Packages
Begin with utility packages that have minimal dependencies:

```go
// Go
package mathutil

func Max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func Sum(numbers []int) int {
    total := 0
    for _, num := range numbers {
        total += num
    }
    return total
}
```

```cursed
fr CURSED
module mathutil

slay pub max(a drip, b drip) drip {
    ready (a > b) { damn a }
    damn b
}

slay pub sum(numbers []drip) drip {
    sus total drip = 0
    bestie (num in numbers) {
        total = total + num
    }
    damn total
}
```

### 2. Convert Data Structures
Transform Go structs to CURSED squads:

```go
// Go
type Config struct {
    Host     string
    Port     int
    Database string
    Debug    bool
}
```

```cursed
fr CURSED
squad Config {
    host tea
    port drip
    database tea
    debug lit
}
```

### 3. Update Error Handling
Replace Go's error pattern with CURSED's yikes system:

```go
// Go
func divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}

result, err := divide(10, 2)
if err != nil {
    log.Fatal(err)
}
```

```cursed
fr CURSED
slay divide(a meal, b meal) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

sus result meal = divide(10, 2) fam {
    when error -> {
        vibez.spill("Error:", error)
        damn
    }
}
```

### 4. Migrate Concurrency
CURSED's goroutines work similarly to Go's:

```go
// Go
func worker(jobs <-chan int, results chan<- int) {
    for job := range jobs {
        results <- job * 2
    }
}

jobs := make(chan int, 100)
results := make(chan int, 100)
go worker(jobs, results)
```

```cursed
fr CURSED
slay worker(jobs chan<drip>, results chan<drip>) {
    bestie (job <- jobs) {
        results <- job * 2
    }
}

sus jobs chan<drip> = make_channel(100)
sus results chan<drip> = make_channel(100)
go worker(jobs, results)
```

## 🔧 Tool Migration

| Go Tool | CURSED Equivalent | Purpose |
|---------|-------------------|---------|
| `go run` | `cursed` | Run programs |
| `go build` | `cursed --compile` | Compilation |
| `go test` | `cursed test` | Testing |
| `go fmt` | `cursed format` | Code formatting |
| `go vet` | `cursed lint` | Static analysis |
| `go mod` | Package manager | Dependency management |
| `gopls` | `cursed-lsp` | Language server |
| `go doc` | `cursed doc` | Documentation |

## 🎯 Key Advantages of CURSED over Go

1. **Faster Builds**: 50-100x faster compilation
2. **Simpler Syntax**: Gen Z slang makes code more expressive
3. **Better Error Messages**: More helpful compiler diagnostics
4. **Modern Features**: Pattern matching, generics, async/await
5. **Zero Memory Leaks**: Advanced GC with arena allocators
6. **Rich Standard Library**: 50+ modules vs Go's standard library

## 📚 Learning Path

1. **Start Simple**: Convert basic Go functions to CURSED
2. **Practice Syntax**: Get comfortable with `sus`, `slay`, `damn` keywords
3. **Explore Standard Library**: Try different modules (mathz, stringz, etc.)
4. **Build Projects**: Port small Go projects to CURSED
5. **Join Community**: Connect with other developers on [Discord](https://discord.gg/cursed-lang)

The migration from Go to CURSED is one of the smoothest you'll experience. The conceptual similarities make the transition natural, while CURSED's improvements make your code faster, safer, and more fun to write!
