//! CURSED LSP Features Demo
//! This file demonstrates all the enhanced LSP features implemented for CURSED

use "stdlib::io"
use "stdlib::math"
use "stdlib::collections"

// Function demonstration with enhanced completion and hover
slay calculate_fibonacci(n: int) -> int {
    lowkey n <= 1 {
        bounce n
    }
    
    // Local variable with type inference
    sus result = calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    bounce result
}

// Struct definition with semantic highlighting
squad Person {
    name: string,
    age: int,
    email: string,
}

// Interface definition for semantic tokens
collab Drawable {
    draw() -> void,
    get_area() -> float,
}

// Async function with goroutine spawning
yolo process_data(data: []int) -> Result<int, Error> {
    sus total = 0
    
    // Enhanced completion should suggest array methods here
    bestie item in data {
        // Type-aware member completion for int
        total += item.abs()
        
        // Error propagation operator for semantic highlighting
        facts processed = try_process(item)?
        total += processed
        
        // Yield point for cooperative scheduling
        yolo
    }
    
    bounce Ok(total)
}

// Complex function for extract function refactoring demo
slay complex_calculation(x: float, y: float) -> float {
    // This block could be extracted as a function
    sus temp1 = sqrt(x * x + y * y)
    sus temp2 = pow(temp1, 2.0)
    sus temp3 = temp2 / (x + y)
    
    // This expression could be extracted as a variable
    facts complex_result = temp3 * sin(x) + cos(y) * log(temp1)
    
    bounce complex_result
}

// Goroutine demonstration
slay worker_goroutine(id: int, ch: chan string) {
    periodt true {
        // Channel operations with semantic highlighting
        facts message = recv(ch)
        lowkey message == "stop" {
            println(format("Worker {} stopping", id))
            skrr // Break from loop
        }
        
        println(format("Worker {} processing: {}", id, message))
        sleep(1000) // Sleep for 1 second
    }
}

// Main function demonstrating various language features
slay main() {
    println("CURSED LSP Features Demo")
    
    // Variable declarations with different types
    facts pi = 3.14159
    sus counter = 0
    facts name = "CURSED Language"
    
    // Array with type-aware completions
    sus numbers = [1, 2, 3, 4, 5]
    
    // Member access with intelligent completion
    println(format("Array length: {}", numbers.len()))
    numbers.push(6)
    numbers.sort()
    
    // Map with enhanced member completions
    sus scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
    scores.set("David", 88)
    
    lowkey scores.contains_key("Alice") {
        facts alice_score = scores.get("Alice")
        println(format("Alice's score: {}", alice_score))
    }
    
    // Function calls with parameter hints
    facts fib_result = calculate_fibonacci(10)
    println(format("Fibonacci(10) = {}", fib_result))
    
    // Struct instantiation and member access
    facts person = Person {
        name: "John Doe",
        age: 30,
        email: "john@example.com",
    }
    
    // Type-aware member completion for struct
    println(format("Person: {} ({})", person.name, person.age))
    
    // Channel creation and goroutine spawning
    facts work_channel = make_channel(10)
    
    // Spawn multiple worker goroutines
    bestie i in 0..3 {
        stan worker_goroutine(i, work_channel)
    }
    
    // Send work to goroutines
    bestie i in 0..10 {
        send(work_channel, format("Task {}", i))
    }
    
    // Stop all workers
    bestie i in 0..3 {
        send(work_channel, "stop")
    }
    
    // Error handling demonstration
    facts result = try_operation()
    vibe_check result {
        mood Ok(value):
            println(format("Success: {}", value))
        mood Err(error):
            println(format("Error: {}", error))
        basic:
            println("Unknown result")
    }
    
    // Async operation
    yolo async_demo()
}

// Function with potential refactoring opportunities
slay try_operation() -> Result<int, string> {
    // This could be extracted as a variable
    sus random_value = rand() % 100
    
    lowkey random_value > 50 {
        bounce Ok(random_value)
    } highkey {
        bounce Err("Value too low")
    }
}

// Async function demonstration
yolo async_demo() {
    println("Starting async operation...")
    
    // Spawn async task
    facts future = stan async_task()
    
    // Do other work while waiting
    println("Doing other work...")
    sleep(500)
    
    // Await the result
    facts result = await future
    println(format("Async result: {}", result))
}

yolo async_task() -> string {
    sleep(1000) // Simulate async work
    bounce "Async operation completed"
}

// Demonstration of various operators and expressions
slay operator_showcase() {
    sus x = 10
    sus y = 5
    
    // Arithmetic operators
    facts sum = x + y
    facts difference = x - y
    facts product = x * y
    facts quotient = x / y
    facts remainder = x % y
    
    // Comparison operators
    facts is_equal = x == y
    facts is_not_equal = x != y
    facts is_greater = x > y
    facts is_less_equal = x <= y
    
    // Logical operators
    facts and_result = (x > 0) && (y > 0)
    facts or_result = (x > 0) || (y < 0)
    facts not_result = !(x == y)
    
    // Bitwise operations
    facts bitwise_and = x & y
    facts bitwise_or = x | y
    facts bitwise_xor = x ^ y
    facts left_shift = x << 1
    facts right_shift = x >> 1
    
    println("Operator showcase completed")
}

// Complex data structures for enhanced completion
squad DatabaseConfig {
    host: string,
    port: int,
    username: string,
    password: string,
    database_name: string,
    connection_pool_size: int,
}

collab Database {
    connect() -> Result<Connection, Error>,
    execute_query(query: string) -> Result<QueryResult, Error>,
    close() -> void,
}

// Generic function demonstration
slay generic_function<T>(items: []T, predicate: (T) -> bool) -> []T {
    sus filtered = []T{}
    
    bestie item in items {
        lowkey predicate(item) {
            filtered.push(item)
        }
    }
    
    bounce filtered
}

// This function has potential code smells that diagnostics should catch
slay function_with_issues() {
    // Unused variable - should trigger warning
    sus unused_variable = 42
    
    // Potential null pointer access - should trigger warning
    sus nullable_value: string? = nil
    facts length = nullable_value.len() // Unsafe access
    
    // Unreachable code - should trigger warning
    bounce
    println("This will never execute")
}

// Pattern matching demonstration
slay pattern_matching_demo(value: any) {
    vibe_check value {
        mood string s:
            println(format("String value: {}", s))
        mood int i when i > 0:
            println(format("Positive integer: {}", i))
        mood int i when i < 0:
            println(format("Negative integer: {}", i))
        mood int 0:
            println("Zero")
        mood float f:
            println(format("Float value: {}", f))
        mood nil:
            println("Nil value")
        basic:
            println("Unknown type")
    }
}
