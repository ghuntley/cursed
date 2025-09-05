fr fr CURSED Standard Library Documentation Demo
fr fr Showcases documentation for stdlib modules

fr fr/ Standard Library Documentation Demonstration
fr fr/ 
fr fr/ This module demonstrates comprehensive documentation
fr fr/ for CURSED's standard library functions and modules.
fr fr/ 
fr fr/ @module stdlib_demo
fr fr/ @author CURSED Team
fr fr/ @version 1.0.0
slay main_character() {
    println("📚 CURSED Standard Library Documentation Demo")
    
    demonstrate_math_functions()
    demonstrate_string_operations()
    demonstrate_collections()
    demonstrate_io_operations()
    demonstrate_crypto_functions()
}

fr fr/ Mathematics Functions Documentation
fr fr/ 
fr fr/ The CURSED math library provides comprehensive mathematical
fr fr/ operations with Gen Z flair and robust error handling.
fr fr/ 
fr fr/ @example Basic math operations
fr fr/ ```cursed
fr fr/ yeet "stdlib::math"
fr fr/ 
fr fr/ facts result = math::sqrt(16.0)  // returns 4.0
fr fr/ facts power = math::pow(2.0, 3.0)  // returns 8.0
fr fr/ ```
fr fr/ 
fr fr/ @see math::basic
fr fr/ @see math::advanced
fr fr/ @since 1.0.0
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

fr fr/ String Operations Documentation
fr fr/ 
fr fr/ CURSED's string library provides powerful text manipulation
fr fr/ with Unicode support and performance optimization.
fr fr/ 
fr fr/ @example String manipulation
fr fr/ ```cursed
fr fr/ yeet "stdlib::string"
fr fr/ 
fr fr/ facts greeting = string::concat("Hello", " World")
fr fr/ facts upper = string::to_uppercase("cursed")
fr fr/ ```
fr fr/ 
fr fr/ @performance O(n) for most operations
fr fr/ @unicode Full Unicode support
fr fr/ @since 1.0.0
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

fr fr/ Collections Documentation
fr fr/ 
fr fr/ CURSED provides comprehensive collection types including
fr fr/ arrays, hash maps, sets, and specialized data structures.
fr fr/ 
fr fr/ @example Working with collections
fr fr/ ```cursed
fr fr/ yeet "stdlib::collections"
fr fr/ 
fr fr/ sus list = Array::new()
fr fr/ list.push("item1")
fr fr/ list.push("item2")
fr fr/ 
fr fr/ sus map = HashMap::new()
fr fr/ map.insert("key", "value")
fr fr/ ```
fr fr/ 
fr fr/ @performance Optimized for common operations
fr fr/ @memory Automatic memory management
fr fr/ @thread_safety Some collections are thread-safe
fr fr/ @since 1.0.0
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

fr fr/ I/O Operations Documentation
fr fr/ 
fr fr/ CURSED's I/O library provides file operations, console I/O,
fr fr/ and network communication with async support.
fr fr/ 
fr fr/ @example File operations
fr fr/ ```cursed
fr fr/ yeet "stdlib::io"
fr fr/ 
fr fr/ facts content = io::read_file("example.txt")?
fr fr/ io::write_file("output.txt", content)?
fr fr/ ```
fr fr/ 
fr fr/ @async Supports async operations
fr fr/ @errors Comprehensive error handling
fr fr/ @buffered Buffered I/O for performance
fr fr/ @since 1.0.0
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

fr fr/ Cryptography Functions Documentation
fr fr/ 
fr fr/ CURSED's crypto library provides secure cryptographic operations
fr fr/ including hashing, encryption, and key management.
fr fr/ 
fr fr/ @example Hashing
fr fr/ ```cursed
fr fr/ yeet "stdlib::crypto"
fr fr/ 
fr fr/ facts hash = crypto::sha256("password123")
fr fr/ facts secure_hash = crypto::argon2("password", salt)
fr fr/ ```
fr fr/ 
fr fr/ @security Constant-time operations where applicable
fr fr/ @algorithms Industry-standard cryptographic algorithms
fr fr/ @performance Optimized implementations
fr fr/ @since 1.0.0
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

fr fr/ Network Communication Documentation
fr fr/ 
fr fr/ CURSED provides high-level networking APIs for HTTP,
fr fr/ WebSocket, and TCP communication.
fr fr/ 
fr fr/ @example HTTP client
fr fr/ ```cursed
fr fr/ yeet "stdlib::http"
fr fr/ 
fr fr/ facts response = http::get("https://api.example.com")?
fr fr/ facts data = response.json()?
fr fr/ ```
fr fr/ 
fr fr/ @async All operations are async
fr fr/ @tls TLS support built-in
fr fr/ @performance Connection pooling and keep-alive
fr fr/ @since 1.0.0
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

fr fr/ Concurrency and Parallelism Documentation
fr fr/ 
fr fr/ CURSED supports goroutines, channels, and async/await
fr fr/ for concurrent and parallel programming.
fr fr/ 
fr fr/ @example Goroutines
fr fr/ ```cursed
fr fr/ stan background_task()  // spawn goroutine
fr fr/ 
fr fr/ facts result = damn async_operation()  // await result
fr fr/ ```
fr fr/ 
fr fr/ @goroutines Lightweight threads
fr fr/ @channels Message passing between goroutines
fr fr/ @async_await Modern async programming model
fr fr/ @since 1.0.0
slay demonstrate_concurrency() {
    println("⚡ Concurrency Operations:")
    
    // Spawn goroutines
    stan background_worker("Task 1")
    stan background_worker("Task 2")
    stan background_worker("Task 3")
    
    // Channel communication
    facts channel = Channel::new()
    stan producer(channel)
    facts result = damn consumer(channel)
    
    println("Concurrent operation result: " + result)
}

fr fr/ Background worker goroutine
fr fr/ 
fr fr/ @param task_name Name of the task to execute
slay background_worker(task_name: string) {
    println("🔄 Executing: " + task_name)
    // Simulate work
    sleep(100)  // 100ms
    println("✅ Completed: " + task_name)
}

fr fr/ Producer goroutine for channel demo
fr fr/ 
fr fr/ @param channel Channel to send data to
slay producer(channel: Channel<string>) {
    channel.send("Hello from producer!")
    channel.send("Data packet 1")
    channel.send("Data packet 2")
    channel.close()
}

fr fr/ Consumer function for channel demo
fr fr/ 
fr fr/ @param channel Channel to receive data from
fr fr/ @return Combined result string
slay consumer(channel: Channel<string>) -> string {
    sus results = Array::new()
    
    periodt (facts message = channel.receive()) {
        results.push(message)
    }
    
    return results.join(", ")
}

fr fr/ Advanced Features Documentation
fr fr/ 
fr fr/ CURSED includes advanced features like reflection,
fr fr/ macros, and compile-time evaluation.
fr fr/ 
fr fr/ @example Reflection
fr fr/ ```cursed
fr fr/ facts type_info = reflect::type_of(my_object)
fr fr/ facts methods = type_info.methods()
fr fr/ ```
fr fr/ 
fr fr/ @reflection Runtime type information
fr fr/ @macros Compile-time code generation
fr fr/ @compile_time Compile-time evaluation
fr fr/ @since 1.0.0
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

fr fr/ Error Handling Best Practices
fr fr/ 
fr fr/ CURSED provides comprehensive error handling with
fr fr/ Result types, Option types, and panic recovery.
fr fr/ 
fr fr/ @example Error handling
fr fr/ ```cursed
fr fr/ facts result = risky_operation()?
fr fr/ 
fr fr/ lowkey (maybe_value.is_some()) {
fr fr/     facts value = maybe_value.unwrap()
fr fr/     // use value
fr fr/ }
fr fr/ ```
fr fr/ 
fr fr/ @result_type Rust-style Result<T, E>
fr fr/ @option_type Rust-style Option<T>
fr fr/ @panic_recovery Panic recovery mechanisms
fr fr/ @since 1.0.0
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

fr fr/ Safe division operation
fr fr/ 
fr fr/ @param numerator Number to divide
fr fr/ @param denominator Number to divide by
fr fr/ @return Result with quotient or error message
slay safe_divide(numerator: f64, denominator: f64) -> Result<f64, string> {
    lowkey (denominator == 0.0) {
        return Err("Division by zero is not allowed")
    }
    
    return Ok(numerator / denominator)
}

fr fr/ Parse string to number
fr fr/ 
fr fr/ @param input String to parse
fr fr/ @return Optional number if parsing succeeds
slay parse_number(input: string) -> Option<i32> {
    // Simulated parsing logic
    lowkey (input == "42") {
        return Some(42)
    }
    
    return None
}

fr fr/ Chain multiple operations that can fail
fr fr/ 
fr fr/ @return Result of chained operations
slay chain_operations() -> Result<string, string> {
    facts first = safe_divide(20.0, 4.0)?
    facts second = safe_divide(first, 2.0)?
    facts third = safe_divide(second, 1.0)?
    
    return Ok("Final result: " + third.to_string())
}

fr fr/ Memory Management Documentation
fr fr/ 
fr fr/ CURSED provides automatic memory management with
fr fr/ garbage collection and smart pointers.
fr fr/ 
fr fr/ @example Smart pointers
fr fr/ ```cursed
fr fr/ facts shared_ptr = Rc::new(data)
fr fr/ facts weak_ref = Rc::downgrade(shared_ptr)
fr fr/ ```
fr fr/ 
fr fr/ @garbage_collection Automatic memory management
fr fr/ @smart_pointers Reference counting and weak references
fr fr/ @memory_safety Memory safety guarantees
fr fr/ @since 1.0.0
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
