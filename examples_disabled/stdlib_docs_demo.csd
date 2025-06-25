// CURSED Standard Library Documentation Demo
// Showcases documentation for stdlib modules

/// Standard Library Documentation Demonstration
/// 
/// This module demonstrates comprehensive documentation
/// for CURSED's standard library functions and modules.
/// 
/// @module stdlib_demo
/// @author CURSED Team
/// @version 1.0.0
slay main() {
    println("📚 CURSED Standard Library Documentation Demo")
    
    demonstrate_math_functions()
    demonstrate_string_operations()
    demonstrate_collections()
    demonstrate_io_operations()
    demonstrate_crypto_functions()
}

/// Mathematics Functions Documentation
/// 
/// The CURSED math library provides comprehensive mathematical
/// operations with Gen Z flair and robust error handling.
/// 
/// @example Basic math operations
/// ```cursed
/// import "stdlib::math"
/// 
/// facts result = math::sqrt(16.0)  // returns 4.0
/// facts power = math::pow(2.0, 3.0)  // returns 8.0
/// ```
/// 
/// @see math::basic
/// @see math::advanced
/// @since 1.0.0
slay demonstrate_math_functions() {
    println("🔢 Math Functions:")
    
    // Basic arithmetic
    facts sum = math::add(5.0, 3.0)
    facts product = math::multiply(4.0, 7.0)
    println("5 + 3 = " + sum.to_string())
    println("4 * 7 = " + product.to_string())
    
    // Advanced functions
    facts sqrt_result = math::sqrt(25.0)
    facts log_result = math::ln(math::E)
    println("√25 = " + sqrt_result.to_string())
    println("ln(e) = " + log_result.to_string())
    
    // Trigonometric functions
    facts sin_result = math::sin(math::PI / 2.0)
    facts cos_result = math::cos(0.0)
    println("sin(π/2) = " + sin_result.to_string())
    println("cos(0) = " + cos_result.to_string())
}

/// String Operations Documentation
/// 
/// CURSED's string library provides powerful text manipulation
/// with Unicode support and performance optimization.
/// 
/// @example String manipulation
/// ```cursed
/// import "stdlib::string"
/// 
/// facts greeting = string::concat("Hello", " World")
/// facts upper = string::to_uppercase("cursed")
/// ```
/// 
/// @performance O(n) for most operations
/// @unicode Full Unicode support
/// @since 1.0.0
slay demonstrate_string_operations() {
    println("🔤 String Operations:")
    
    // String concatenation
    facts hello = "Hello"
    facts world = "World"
    facts greeting = string::concat(hello, " " + world + "!")
    println("Concatenated: " + greeting)
    
    // String transformations
    facts upper = string::to_uppercase("cursed language")
    facts lower = string::to_lowercase("GEN Z VIBES")
    println("Uppercase: " + upper)
    println("Lowercase: " + lower)
    
    // String searching
    facts text = "CURSED is absolutely iconic"
    facts position = string::find(text, "iconic")
    lowkey (position >= 0) {
        println("Found 'iconic' at position: " + position.to_string())
    }
    
    // String splitting
    facts parts = string::split("apple,banana,cherry", ",")
    println("Split result has " + parts.length().to_string() + " parts")
}

/// Collections Documentation
/// 
/// CURSED provides comprehensive collection types including
/// arrays, hash maps, sets, and specialized data structures.
/// 
/// @example Working with collections
/// ```cursed
/// import "stdlib::collections"
/// 
/// sus list = Array::new()
/// list.push("item1")
/// list.push("item2")
/// 
/// sus map = HashMap::new()
/// map.insert("key", "value")
/// ```
/// 
/// @performance Optimized for common operations
/// @memory Automatic memory management
/// @thread_safety Some collections are thread-safe
/// @since 1.0.0
slay demonstrate_collections() {
    println("📦 Collections:")
    
    // Array operations
    sus numbers = Array::new()
    numbers.push(1)
    numbers.push(2)
    numbers.push(3)
    println("Array length: " + numbers.length().to_string())
    
    // HashMap operations
    sus person_ages = HashMap::new()
    person_ages.insert("Alice", 25)
    person_ages.insert("Bob", 30)
    person_ages.insert("Charlie", 22)
    
    lowkey (person_ages.contains_key("Alice")) {
        facts alice_age = person_ages.get("Alice").unwrap()
        println("Alice's age: " + alice_age.to_string())
    }
    
    // Set operations
    sus unique_tags = HashSet::new()
    unique_tags.insert("gen-z")
    unique_tags.insert("programming")
    unique_tags.insert("cursed")
    unique_tags.insert("gen-z")  // Duplicate, won't be added
    println("Unique tags count: " + unique_tags.size().to_string())
}

/// I/O Operations Documentation
/// 
/// CURSED's I/O library provides file operations, console I/O,
/// and network communication with async support.
/// 
/// @example File operations
/// ```cursed
/// import "stdlib::io"
/// 
/// facts content = io::read_file("example.txt")?
/// io::write_file("output.txt", content)?
/// ```
/// 
/// @async Supports async operations
/// @errors Comprehensive error handling
/// @buffered Buffered I/O for performance
/// @since 1.0.0
slay demonstrate_io_operations() {
    println("💾 I/O Operations:")
    
    // Console I/O
    println("This is console output! 📝")
    io::print("Print without newline")
    io::println(" - now with newline!")
    
    // File operations (simulated)
    facts sample_data = "Hello from CURSED! 🔥"
    
    // In real implementation, this would write to file
    println("Would write to file: " + sample_data)
    
    // Directory operations
    facts current_dir = io::current_directory()
    println("Current directory: " + current_dir)
    
    // Path operations
    facts file_path = path::join("docs", "readme.md")
    facts extension = path::extension(file_path)
    println("File extension: " + extension)
}

/// Cryptography Functions Documentation
/// 
/// CURSED's crypto library provides secure cryptographic operations
/// including hashing, encryption, and key management.
/// 
/// @example Hashing
/// ```cursed
/// import "stdlib::crypto"
/// 
/// facts hash = crypto::sha256("password123")
/// facts secure_hash = crypto::argon2("password", salt)
/// ```
/// 
/// @security Constant-time operations where applicable
/// @algorithms Industry-standard cryptographic algorithms
/// @performance Optimized implementations
/// @since 1.0.0
slay demonstrate_crypto_functions() {
    println("🔐 Cryptography Functions:")
    
    // Hashing operations
    facts message = "Hello CURSED!"
    facts sha256_hash = crypto::sha256(message)
    facts blake3_hash = crypto::blake3(message)
    
    println("Original: " + message)
    println("SHA256: " + sha256_hash)
    println("BLAKE3: " + blake3_hash)
    
    // Random number generation
    facts random_bytes = crypto::random_bytes(16)
    facts random_number = crypto::random_i32(1, 100)
    
    println("Random number (1-100): " + random_number.to_string())
    println("Random bytes length: " + random_bytes.length().to_string())
    
    // Key derivation
    facts password = "super_secure_password"
    facts salt = crypto::generate_salt()
    facts derived_key = crypto::pbkdf2(password, salt, 10000)
    
    println("Derived key length: " + derived_key.length().to_string())
}

/// Network Communication Documentation
/// 
/// CURSED provides high-level networking APIs for HTTP,
/// WebSocket, and TCP communication.
/// 
/// @example HTTP client
/// ```cursed
/// import "stdlib::http"
/// 
/// facts response = http::get("https://api.example.com")?
/// facts data = response.json()?
/// ```
/// 
/// @async All operations are async
/// @tls TLS support built-in
/// @performance Connection pooling and keep-alive
/// @since 1.0.0
slay demonstrate_network_operations() {
    println("🌐 Network Operations:")
    
    // HTTP operations (simulated)
    println("Would make HTTP GET request to API")
    println("Would parse JSON response")
    println("Would handle network errors gracefully")
    
    // WebSocket operations (simulated)
    println("Would establish WebSocket connection")
    println("Would send/receive real-time messages")
    
    // TCP operations (simulated)
    println("Would create TCP server on port 8080")
    println("Would handle incoming connections")
}

/// Concurrency and Parallelism Documentation
/// 
/// CURSED supports goroutines, channels, and async/await
/// for concurrent and parallel programming.
/// 
/// @example Goroutines
/// ```cursed
/// stan background_task()  // spawn goroutine
/// 
/// facts result = yolo async_operation()  // await result
/// ```
/// 
/// @goroutines Lightweight threads
/// @channels Message passing between goroutines
/// @async_await Modern async programming model
/// @since 1.0.0
slay demonstrate_concurrency() {
    println("⚡ Concurrency Operations:")
    
    // Spawn goroutines
    stan background_worker("Task 1")
    stan background_worker("Task 2")
    stan background_worker("Task 3")
    
    // Channel communication
    facts channel = Channel::new()
    stan producer(channel)
    facts result = yolo consumer(channel)
    
    println("Concurrent operation result: " + result)
}

/// Background worker goroutine
/// 
/// @param task_name Name of the task to execute
slay background_worker(task_name: string) {
    println("🔄 Executing: " + task_name)
    // Simulate work
    sleep(100)  // 100ms
    println("✅ Completed: " + task_name)
}

/// Producer goroutine for channel demo
/// 
/// @param channel Channel to send data to
slay producer(channel: Channel<string>) {
    channel.send("Hello from producer!")
    channel.send("Data packet 1")
    channel.send("Data packet 2")
    channel.close()
}

/// Consumer function for channel demo
/// 
/// @param channel Channel to receive data from
/// @return Combined result string
slay consumer(channel: Channel<string>) -> string {
    sus results = Array::new()
    
    periodt (facts message = channel.receive()) {
        results.push(message)
    }
    
    return results.join(", ")
}

/// Advanced Features Documentation
/// 
/// CURSED includes advanced features like reflection,
/// macros, and compile-time evaluation.
/// 
/// @example Reflection
/// ```cursed
/// facts type_info = reflect::type_of(my_object)
/// facts methods = type_info.methods()
/// ```
/// 
/// @reflection Runtime type information
/// @macros Compile-time code generation
/// @compile_time Compile-time evaluation
/// @since 1.0.0
slay demonstrate_advanced_features() {
    println("🚀 Advanced Features:")
    
    // Reflection (simulated)
    facts person = Person::new("Alex", 25, 8.5)
    facts type_name = reflect::type_name(person)
    facts field_count = reflect::field_count(person)
    
    println("Object type: " + type_name)
    println("Field count: " + field_count.to_string())
    
    // Compile-time features (simulated)
    facts build_time = compile_time::timestamp()
    facts version = compile_time::version()
    
    println("Built at: " + build_time)
    println("Version: " + version)
}

/// Error Handling Best Practices
/// 
/// CURSED provides comprehensive error handling with
/// Result types, Option types, and panic recovery.
/// 
/// @example Error handling
/// ```cursed
/// facts result = risky_operation()?
/// 
/// lowkey (maybe_value.is_some()) {
///     facts value = maybe_value.unwrap()
///     // use value
/// }
/// ```
/// 
/// @result_type Rust-style Result<T, E>
/// @option_type Rust-style Option<T>
/// @panic_recovery Panic recovery mechanisms
/// @since 1.0.0
slay demonstrate_error_handling() {
    println("⚠️ Error Handling:")
    
    // Result type handling
    facts divide_result = safe_divide(10.0, 2.0)
    lowkey (divide_result.is_ok()) {
        println("Division result: " + divide_result.unwrap().to_string())
    } highkey {
        println("Division error: " + divide_result.error())
    }
    
    // Option type handling
    facts maybe_number = parse_number("42")
    lowkey (maybe_number.is_some()) {
        println("Parsed number: " + maybe_number.unwrap().to_string())
    } highkey {
        println("Failed to parse number")
    }
    
    // Error propagation with ?
    facts chained_result = chain_operations()?
    println("Chained operations result: " + chained_result)
}

/// Safe division operation
/// 
/// @param numerator Number to divide
/// @param denominator Number to divide by
/// @return Result with quotient or error message
slay safe_divide(numerator: f64, denominator: f64) -> Result<f64, string> {
    lowkey (denominator == 0.0) {
        return Err("Division by zero is not allowed")
    }
    
    return Ok(numerator / denominator)
}

/// Parse string to number
/// 
/// @param input String to parse
/// @return Optional number if parsing succeeds
slay parse_number(input: string) -> Option<i32> {
    // Simulated parsing logic
    lowkey (input == "42") {
        return Some(42)
    }
    
    return None
}

/// Chain multiple operations that can fail
/// 
/// @return Result of chained operations
slay chain_operations() -> Result<string, string> {
    facts first = safe_divide(20.0, 4.0)?
    facts second = safe_divide(first, 2.0)?
    facts third = safe_divide(second, 1.0)?
    
    return Ok("Final result: " + third.to_string())
}

/// Memory Management Documentation
/// 
/// CURSED provides automatic memory management with
/// garbage collection and smart pointers.
/// 
/// @example Smart pointers
/// ```cursed
/// facts shared_ptr = Rc::new(data)
/// facts weak_ref = Rc::downgrade(shared_ptr)
/// ```
/// 
/// @garbage_collection Automatic memory management
/// @smart_pointers Reference counting and weak references
/// @memory_safety Memory safety guarantees
/// @since 1.0.0
slay demonstrate_memory_management() {
    println("🧠 Memory Management:")
    
    // Reference counting
    facts shared_data = Rc::new("Shared data")
    facts reference1 = Rc::clone(shared_data)
    facts reference2 = Rc::clone(shared_data)
    
    println("Reference count: " + Rc::strong_count(shared_data).to_string())
    
    // Weak references
    facts weak_ref = Rc::downgrade(shared_data)
    lowkey (weak_ref.upgrade().is_some()) {
        println("Weak reference is still valid")
    }
    
    // Memory usage information
    facts heap_size = memory::heap_size()
    facts gc_stats = memory::gc_statistics()
    
    println("Current heap size: " + heap_size.to_string() + " bytes")
    println("GC collections: " + gc_stats.collections.to_string())
}
