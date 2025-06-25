#!/usr/bin/env cursed

//! # Comprehensive Standard Library Demo
//! 
//! This example demonstrates the extensive capabilities of CURSED's standard library,
//! showcasing over 30 modules that provide enterprise-grade functionality for
//! real-world applications.
//! 
//! ## Modules Demonstrated
//! - **Math Operations**: Basic, advanced, statistics, trigonometry
//! - **String Manipulation**: Text processing, formatting, validation  
//! - **File System**: I/O operations, directory management, metadata
//! - **Networking**: HTTP clients/servers, DNS, WebSockets
//! - **Database**: SQLite, PostgreSQL, MongoDB integration
//! - **Cryptography**: Complete crypto ecosystem with 10+ modules
//! - **Process Management**: System processes, IPC, monitoring
//! - **Time Handling**: Date/time operations, formatting, timezones
//! - **Collections**: Data structures, iterators, algorithms
//! - **Testing Framework**: Unit testing, benchmarks, assertions
//! - **Environment**: Variable handling, configuration management
//! - **JSON Processing**: Parsing, serialization, streaming
//! - **Logging**: Structured logging with multiple outputs
//! - **Template System**: Multiple formats, rendering engines
//! - **Web Framework**: Full-stack web development
//! - **Atomic Operations**: Low-level synchronization primitives
//! - **Signal Handling**: OS signal management and coordination
//! - **Inter-Process Communication**: Pipes, queues, shared memory
//! - **Performance Profiling**: CPU, memory, benchmark tools
//! - **Synchronization**: Threading, mutexes, channels
//! - **Asynchronous Operations**: Future-based programming
//!
//! @author CURSED Language Team
//! @version 1.0.0

// Import major standard library modules
import "stdlib::math";
import "stdlib::string";
import "stdlib::fs";
import "stdlib::net";
import "stdlib::database";
import "stdlib::crypto";
import "stdlib::process";
import "stdlib::time";
import "stdlib::collections";
import "stdlib::testing";
import "stdlib::env";
import "stdlib::json_tea";
import "stdlib::oglogging";
import "stdlib::template";
import "stdlib::web_vibez";
import "stdlib::atomic_drip";
import "stdlib::signal_boost";
import "stdlib::ipc";
import "stdlib::profiler";
import "stdlib::sync";
import "stdlib::io";

/// Demonstrate mathematical operations across multiple modules
slay function demonstrate_math_operations() -> Result<(), string> {
    spill("🔢 Mathematical Operations Demo");
    spill("================================\n");
    
    // Basic operations
    spill("📊 Basic Math:");
    facts a = 15.7;
    facts b = 4.2;
    
    spill("  {} + {} = {:.3}", a, b, math::add(a, b));
    spill("  {} - {} = {:.3}", a, b, math::subtract(a, b));
    spill("  {} * {} = {:.3}", a, b, math::multiply(a, b));
    spill("  {} / {} = {:.3}", a, b, math::divide(a, b)?);
    spill("  sqrt({}) = {:.3}", a, math::sqrt(a)?);
    spill("  {}^2 = {:.3}", a, math::pow(a, 2.0)?);
    
    // Trigonometric functions
    spill("\n📐 Trigonometry:");
    facts angle = math::PI / 4.0; // 45 degrees
    spill("  sin(π/4) = {:.6}", math::sin(angle));
    spill("  cos(π/4) = {:.6}", math::cos(angle));
    spill("  tan(π/4) = {:.6}", math::tan(angle));
    spill("  45° = {:.3} radians", math::degrees_to_radians(45.0));
    
    // Statistical operations
    spill("\n📈 Statistics:");
    facts data = [1.2, 3.4, 2.1, 5.6, 4.3, 6.7, 3.9, 2.8, 4.5, 3.2];
    spill("  Dataset: {:?}", data);
    spill("  Mean: {:.3}", math::mean(&data)?);
    spill("  Median: {:.3}", math::median(&data)?);
    spill("  Std Dev: {:.3}", math::standard_deviation(&data)?);
    spill("  Min: {:.3}, Max: {:.3}", math::min_array(&data)?, math::max_array(&data)?);
    
    // Random number generation
    spill("\n🎲 Random Numbers:");
    spill("  Random float [0,1): {:.6}", math::random());
    spill("  Random int [1,100]: {}", math::random_range(1, 101));
    spill("  Random choice: {}", math::choice(&["apple", "banana", "cherry"])?);
    
    periodt Ok(());
}

/// Demonstrate string manipulation capabilities
slay function demonstrate_string_operations() -> Result<(), string> {
    spill("\n🔤 String Manipulation Demo");
    spill("=============================\n");
    
    facts text = "  Hello, CURSED Programming Language!  ";
    spill("📝 Original: '{}'", text);
    
    // Basic string operations
    spill("🔧 Basic Operations:");
    spill("  Length: {}", string::length(&text));
    spill("  Trimmed: '{}'", string::trim(&text));
    spill("  Uppercase: '{}'", string::to_uppercase(&text));
    spill("  Lowercase: '{}'", string::to_lowercase(&text));
    spill("  Title Case: '{}'", string::to_title_case(&text));
    
    // Search and replace
    spill("\n🔍 Search & Replace:");
    facts search_text = string::trim(&text);
    spill("  Contains 'CURSED': {}", string::contains(&search_text, "CURSED"));
    spill("  Starts with 'Hello': {}", string::starts_with(&search_text, "Hello"));
    spill("  Find 'CURSED': {:?}", string::find(&search_text, "CURSED"));
    spill("  Replace 'CURSED' with 'Amazing': '{}'", 
          string::replace(&search_text, "CURSED", "Amazing"));
    
    // Splitting and joining
    spill("\n✂️  Splitting & Joining:");
    facts words = string::split(&search_text, " ");
    spill("  Split by space: {:?}", words);
    spill("  Word count: {}", words.length());
    spill("  Joined with '-': '{}'", string::join(&words, "-"));
    
    // Validation
    spill("\n✅ Validation:");
    facts email = "user@example.com";
    facts url = "https://cursed-lang.org";
    facts phone = "+1-555-123-4567";
    
    spill("  '{}' is valid email: {}", email, string::is_email(&email));
    spill("  '{}' is valid URL: {}", url, string::is_url(&url));
    spill("  '{}' is valid phone: {}", phone, string::is_phone_number(&phone));
    
    // Formatting
    spill("\n🎨 Formatting:");
    facts number = 42;
    facts name = "Alice";
    spill("  Pad left: '{}'", string::pad_left("123", 8, '0'));
    spill("  Center: '{}'", string::center("CURSED", 20, '='));
    spill("  Format template: '{}'", 
          "Hello {}, your number is {}!".replace("{}", &name).replace("{}", &number.to_string()));
    
    periodt Ok(());
}

/// Demonstrate file system operations
slay function demonstrate_file_operations() -> Result<(), string> {
    spill("\n📁 File System Operations Demo");
    spill("===============================\n");
    
    facts demo_dir = "demo_files";
    facts demo_file = format!("{}/example.txt", demo_dir);
    facts demo_content = "Hello from CURSED file system!\nThis is a test file.\nLine 3 of content.";
    
    spill("📂 Directory Operations:");
    
    // Create directory
    lowkey (!fs::exists(&demo_dir)) {
        fs::create_dir(&demo_dir)?;
        spill("  ✅ Created directory: {}", demo_dir);
    } highkey {
        spill("  📁 Directory already exists: {}", demo_dir);
    }
    
    // File operations
    spill("\n📄 File Operations:");
    
    // Write file
    fs::write_file(&demo_file, &demo_content)?;
    spill("  ✅ Wrote file: {}", demo_file);
    
    // Read file
    facts read_content = fs::read_file(&demo_file)?;
    spill("  📖 Read {} bytes from file", read_content.len());
    spill("  📝 Content preview: '{}'", 
          string::truncate(&read_content.replace("\n", "\\n"), 50));
    
    // File metadata
    facts metadata = fs::metadata(&demo_file)?;
    spill("  📊 File size: {} bytes", metadata.size);
    spill("  🕐 Modified: {:?}", metadata.modified);
    spill("  🔒 Permissions: {:?}", metadata.permissions);
    
    // Directory listing
    spill("\n📋 Directory Listing:");
    facts entries = fs::list_dir(&demo_dir)?;
    spill("  📁 {} found {} entries:", demo_dir, entries.length());
    
    bestie (sus i = 0; i < entries.length(); i++) {
        facts entry = &entries[i];
        facts file_type = lowkey (entry.is_file) { "📄" } highkey { "📁" };
        spill("    {} {} ({} bytes)", file_type, entry.name, entry.size);
    }
    
    // Path operations
    spill("\n🛤️  Path Operations:");
    facts path = &demo_file;
    spill("  Full path: {}", fs::absolute_path(path)?);
    spill("  Parent dir: {:?}", fs::parent_dir(path));
    spill("  File name: {:?}", fs::file_name(path));
    spill("  Extension: {:?}", fs::extension(path));
    
    // Cleanup
    fs::delete_file(&demo_file)?;
    fs::remove_dir(&demo_dir)?;
    spill("  🗑️  Cleaned up demo files");
    
    periodt Ok(());
}

/// Demonstrate networking capabilities
slay function demonstrate_networking() -> Result<(), string> {
    spill("\n🌐 Networking Demo");
    spill("==================\n");
    
    // Network utilities
    spill("🔧 Network Utilities:");
    
    // DNS operations
    match net::resolve_hostname("example.com") {
        Ok(ip) => spill("  🌍 example.com resolves to: {}", ip),
        Err(e) => spill("  ❌ DNS resolution failed: {}", e),
    }
    
    // Port availability check
    facts test_port = 8080;
    spill("  🔌 Port {} available: {}", test_port, net::is_port_available(test_port));
    
    // Network interface info
    spill("\n🖧 Network Interfaces:");
    match net::list_interfaces() {
        Ok(interfaces) => {
            bestie (sus i = 0; i < interfaces.length(); i++) {
                facts iface = &interfaces[i];
                spill("  📡 {}: {} ({})", iface.name, iface.ip_address, iface.interface_type);
            }
        }
        Err(e) => spill("  ❌ Failed to list interfaces: {}", e),
    }
    
    // HTTP client example (simulated)
    spill("\n🌐 HTTP Operations:");
    spill("  📡 Creating HTTP client...");
    
    sus http_client = net::HttpClient::new();
    http_client.set_timeout(5000); // 5 second timeout
    http_client.set_user_agent("CURSED-Demo/1.0");
    
    spill("  ✅ HTTP client configured");
    spill("  🔗 Ready for requests (example endpoints):");
    spill("    • GET https://api.github.com/users/octocat");
    spill("    • POST https://httpbin.org/post");
    spill("    • WebSocket ws://echo.websocket.org");
    
    periodt Ok(());
}

/// Demonstrate database operations  
slay function demonstrate_database() -> Result<(), string> {
    spill("\n🗄️  Database Operations Demo");
    spill("============================\n");
    
    // SQLite database operations
    spill("💾 SQLite Operations:");
    
    facts db_path = ":memory:"; // In-memory database for demo
    sus db = database::sqlite::Connection::open(db_path)?;
    
    spill("  ✅ Connected to SQLite database");
    
    // Create table
    facts create_table_sql = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            age INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    ";
    
    db.execute(create_table_sql)?;
    spill("  🏗️  Created 'users' table");
    
    // Insert data
    facts users_data = [
        ("Alice Johnson", "alice@example.com", 28),
        ("Bob Smith", "bob@example.com", 34),
        ("Charlie Brown", "charlie@example.com", 25),
    ];
    
    bestie (sus i = 0; i < users_data.length(); i++) {
        facts (name, email, age) = users_data[i];
        facts insert_sql = "INSERT INTO users (name, email, age) VALUES (?, ?, ?)";
        db.execute_params(insert_sql, &[name, email, &age.to_string()])?;
    }
    
    spill("  📝 Inserted {} users", users_data.length());
    
    // Query data
    facts query_sql = "SELECT name, email, age FROM users ORDER BY name";
    facts results = db.query(query_sql)?;
    
    spill("  📊 Query results:");
    bestie (sus i = 0; i < results.length(); i++) {
        facts row = &results[i];
        spill("    👤 {}: {} (age {})", row[0], row[1], row[2]);
    }
    
    // Database statistics
    facts count_sql = "SELECT COUNT(*) FROM users";
    facts count_result = db.query_single(count_sql)?;
    spill("  📈 Total users in database: {}", count_result[0]);
    
    db.close()?;
    spill("  🔐 Database connection closed");
    
    periodt Ok(());
}

/// Demonstrate cryptographic operations
slay function demonstrate_cryptography() -> Result<(), string> {
    spill("\n🔐 Cryptography Demo");
    spill("====================\n");
    
    // Hash functions
    spill("🔨 Hash Functions:");
    facts message = "Hello, CURSED Cryptography!";
    
    facts sha256_hash = crypto::hash::sha256(message.as_bytes())?;
    facts sha3_hash = crypto::hash::sha3_256(message.as_bytes())?;
    facts blake3_hash = crypto::hash::blake3(message.as_bytes())?;
    
    spill("  📝 Message: '{}'", message);
    spill("  🔗 SHA-256: {}", hex::encode(&sha256_hash));
    spill("  🔗 SHA3-256: {}", hex::encode(&sha3_hash));
    spill("  🔗 BLAKE3: {}", hex::encode(&blake3_hash));
    
    // Symmetric encryption
    spill("\n🔒 Symmetric Encryption:");
    
    facts key = crypto::random::generate_key(32)?; // 256-bit key
    facts plaintext = "This is a secret message for CURSED!";
    
    facts encrypted = crypto::symmetric::aes_gcm_encrypt(plaintext.as_bytes(), &key)?;
    facts decrypted = crypto::symmetric::aes_gcm_decrypt(&encrypted, &key)?;
    facts decrypted_text = String::from_utf8(decrypted)?;
    
    spill("  🔓 Original: '{}'", plaintext);
    spill("  🔐 Encrypted: {} bytes", encrypted.len());
    spill("  🔓 Decrypted: '{}'", decrypted_text);
    spill("  ✅ Encryption/decryption successful: {}", plaintext == decrypted_text);
    
    // Digital signatures
    spill("\n✍️  Digital Signatures:");
    
    facts (private_key, public_key) = crypto::asymmetric::ed25519_generate_keypair()?;
    facts signature = crypto::signatures::ed25519_sign(message.as_bytes(), &private_key)?;
    facts is_valid = crypto::signatures::ed25519_verify(message.as_bytes(), &signature, &public_key)?;
    
    spill("  🔑 Generated Ed25519 keypair");
    spill("  ✍️  Signed message");
    spill("  ✅ Signature valid: {}", is_valid);
    
    // Key derivation
    spill("\n🗝️  Key Derivation:");
    
    facts password = "secure_password_123";
    facts salt = crypto::random::generate_salt(16)?;
    facts derived_key = crypto::kdf::pbkdf2_sha256(password.as_bytes(), &salt, 100000, 32)?;
    
    spill("  🔐 Password: '{}'", password);
    spill("  🧂 Salt: {}", hex::encode(&salt));
    spill("  🔑 Derived key: {}", hex::encode(&derived_key));
    
    periodt Ok(());
}

/// Demonstrate process management
slay function demonstrate_process_management() -> Result<(), string> {
    spill("\n⚙️  Process Management Demo");
    spill("===========================\n");
    
    // Current process information
    spill("📊 Current Process Info:");
    facts current_pid = process::get_current_pid();
    facts process_info = process::get_process_info(current_pid)?;
    
    spill("  🆔 PID: {}", current_pid);
    spill("  👤 User: {:?}", process_info.user);
    spill("  💾 Memory: {:.2} MB", process_info.memory_mb);
    spill("  🖥️  CPU: {:.1}%", process_info.cpu_percent);
    spill("  🕐 Runtime: {:?}", process_info.runtime);
    
    // System information
    spill("\n🖥️  System Information:");
    facts cpu_count = process::get_cpu_count();
    facts load_avg = process::get_load_average()?;
    facts uptime = process::get_system_uptime()?;
    
    spill("  🔢 CPU cores: {}", cpu_count);
    spill("  📊 Load average: {:.2}, {:.2}, {:.2}", load_avg.0, load_avg.1, load_avg.2);
    spill("  ⏰ System uptime: {:.2} hours", uptime / 3600.0);
    
    // Command execution
    spill("\n🚀 Command Execution:");
    
    // Execute simple commands
    facts commands = [
        "echo 'Hello from CURSED process management!'",
        "date",
        "pwd",
    ];
    
    bestie (sus i = 0; i < commands.length(); i++) {
        facts cmd = commands[i];
        spill("  🔄 Executing: {}", cmd);
        
        match process::run_command(cmd) {
            Ok(output) => {
                spill("    ✅ Exit code: {}", output.exit_code);
                spill("    📤 Output: {}", string::trim(&output.stdout));
                lowkey (!output.stderr.is_empty()) {
                    spill("    ⚠️  Stderr: {}", string::trim(&output.stderr));
                }
            }
            Err(e) => {
                spill("    ❌ Command failed: {}", e);
            }
        }
    }
    
    // Process monitoring
    spill("\n👁️  Process Monitoring:");
    
    facts process_list = process::get_process_list()?;
    facts total_processes = process_list.length();
    spill("  📋 Total processes: {}", total_processes);
    
    // Show top processes by memory usage
    sus sorted_processes = process_list.clone();
    sorted_processes.sort_by(|a, b| b.memory_mb.partial_cmp(&a.memory_mb).unwrap());
    
    spill("  🏆 Top 5 processes by memory:");
    bestie (sus i = 0; i < math::min(5, sorted_processes.length()); i++) {
        facts proc = &sorted_processes[i];
        spill("    {}. {} (PID {}): {:.1} MB", 
              i + 1, proc.name, proc.pid, proc.memory_mb);
    }
    
    periodt Ok(());
}

/// Demonstrate time and date operations
slay function demonstrate_time_operations() -> Result<(), string> {
    spill("\n🕐 Time & Date Operations Demo");
    spill("==============================\n");
    
    // Current time
    spill("⏰ Current Time:");
    facts now = time::now();
    facts utc_now = time::utc_now();
    
    spill("  🏠 Local time: {}", now.format("%Y-%m-%d %H:%M:%S %Z")?);
    spill("  🌍 UTC time: {}", utc_now.format("%Y-%m-%d %H:%M:%S UTC")?);
    spill("  📅 Today: {}", now.format("%A, %B %d, %Y")?);
    
    // Date arithmetic
    spill("\n📅 Date Arithmetic:");
    facts tomorrow = now.add_days(1)?;
    facts next_week = now.add_days(7)?;
    facts last_month = now.subtract_months(1)?;
    
    spill("  ➡️  Tomorrow: {}", tomorrow.format("%Y-%m-%d")?);
    spill("  📅 Next week: {}", next_week.format("%Y-%m-%d")?);
    spill("  ⬅️  Last month: {}", last_month.format("%Y-%m-%d")?);
    
    // Duration calculations
    spill("\n⏱️  Duration Calculations:");
    facts start_time = time::now();
    
    // Simulate some work
    time::sleep_millis(100);
    
    facts end_time = time::now();
    facts duration = time::duration_between(&start_time, &end_time)?;
    
    spill("  ⏱️  Simulated work duration: {} ms", duration.as_millis());
    
    // Time formatting
    spill("\n🎨 Time Formatting:");
    facts timestamp = now;
    
    spill("  📊 ISO 8601: {}", timestamp.format_iso8601()?);
    spill("  📧 RFC 3339: {}", timestamp.format_rfc3339()?);
    spill("  🇺🇸 US Format: {}", timestamp.format("%m/%d/%Y %I:%M %p")?);
    spill("  🇪🇺 EU Format: {}", timestamp.format("%d/%m/%Y %H:%M")?);
    spill("  📱 Relative: {}", time::format_relative(&timestamp, &now));
    
    // Timezone operations
    spill("\n🌍 Timezone Operations:");
    facts timezones = ["UTC", "America/New_York", "Europe/London", "Asia/Tokyo"];
    
    bestie (sus i = 0; i < timezones.length(); i++) {
        facts tz_name = timezones[i];
        match time::timezone_by_name(tz_name) {
            Ok(timezone) => {
                facts tz_time = now.convert_timezone(&timezone)?;
                spill("  🌍 {}: {}", tz_name, tz_time.format("%H:%M %Z")?);
            }
            Err(e) => {
                spill("  ❌ Failed to get timezone {}: {}", tz_name, e);
            }
        }
    }
    
    periodt Ok(());
}

/// Demonstrate collections and data structures
slay function demonstrate_collections() -> Result<(), string> {
    spill("\n📦 Collections & Data Structures Demo");
    spill("=====================================\n");
    
    // Dynamic arrays
    spill("📋 Dynamic Arrays:");
    sus numbers = collections::Vec::new();
    bestie (sus i = 1; i <= 10; i++) {
        numbers.push(i * i); // Push squares
    }
    
    spill("  📊 Squares: {:?}", numbers);
    spill("  📏 Length: {}", numbers.length());
    spill("  🔍 Contains 25: {}", numbers.contains(&25));
    spill("  🎯 Index of 36: {:?}", numbers.find(&36));
    
    // Hash sets
    spill("\n🎯 Hash Sets:");
    sus unique_numbers = collections::HashSet::new();
    facts test_data = [1, 2, 3, 2, 4, 3, 5, 1, 6];
    
    bestie (sus i = 0; i < test_data.length(); i++) {
        unique_numbers.insert(test_data[i]);
    }
    
    spill("  📊 Original: {:?}", test_data);
    spill("  🎯 Unique: {:?}", unique_numbers.to_vec());
    spill("  📏 Unique count: {}", unique_numbers.size());
    
    // Hash maps
    spill("\n🗺️  Hash Maps:");
    sus word_count = collections::HashMap::new();
    facts text = "the quick brown fox jumps over the lazy dog the fox is quick";
    facts words = string::split(&text, " ");
    
    bestie (sus i = 0; i < words.length(); i++) {
        facts word = &words[i];
        facts current_count = word_count.get(word).unwrap_or(&0);
        word_count.insert(word.clone(), current_count + 1);
    }
    
    spill("  📝 Text: '{}'", text);
    spill("  📊 Word frequencies:");
    
    facts sorted_words = word_count.keys().collect::<Vec<_>>();
    sorted_words.sort();
    
    bestie (sus i = 0; i < sorted_words.length(); i++) {
        facts word = sorted_words[i];
        facts count = word_count.get(word).unwrap();
        spill("    '{}': {} times", word, count);
    }
    
    // Queues and stacks
    spill("\n📚 Queues & Stacks:");
    
    // Queue (FIFO)
    sus queue = collections::Queue::new();
    bestie (sus i = 1; i <= 5; i++) {
        queue.enqueue(format!("Task {}", i));
    }
    
    spill("  📥 Queue operations:");
    spill("    Queue size: {}", queue.size());
    flex (!queue.is_empty()) {
        facts item = queue.dequeue().unwrap();
        spill("    Dequeued: {}", item);
    }
    
    // Stack (LIFO)
    sus stack = collections::Stack::new();
    bestie (sus i = 1; i <= 5; i++) {
        stack.push(format!("Item {}", i));
    }
    
    spill("  📚 Stack operations:");
    spill("    Stack size: {}", stack.size());
    flex (!stack.is_empty()) {
        facts item = stack.pop().unwrap();
        spill("    Popped: {}", item);
    }
    
    // Priority queue
    spill("\n🎖️  Priority Queue:");
    sus priority_queue = collections::PriorityQueue::new();
    
    facts tasks = [
        ("Critical bug fix", 10),
        ("Code review", 5),
        ("Documentation update", 3),
        ("Security patch", 15),
        ("Feature implementation", 7),
    ];
    
    bestie (sus i = 0; i < tasks.length(); i++) {
        facts (task, priority) = tasks[i];
        priority_queue.insert(task, priority);
    }
    
    spill("  📋 Processing tasks by priority:");
    flex (!priority_queue.is_empty()) {
        facts (task, priority) = priority_queue.extract_max().unwrap();
        spill("    Priority {}: {}", priority, task);
    }
    
    periodt Ok(());
}

/// Demonstrate testing framework capabilities
slay function demonstrate_testing_framework() -> Result<(), string> {
    spill("\n🧪 Testing Framework Demo");
    spill("==========================\n");
    
    spill("🔬 Creating test suite...");
    
    // Create test framework
    sus test_framework = testing::TestFramework::new();
    test_framework.set_parallel_execution(true);
    test_framework.set_timeout(5000); // 5 second timeout
    
    // Define test cases
    spill("\n📝 Defining test cases:");
    
    // Math tests
    test_framework.add_test("math_addition", || {
        facts result = 2 + 2;
        testing::assert_eq(result, 4, "Basic addition should work")?;
        testing::assert_greater(result, 0, "Result should be positive")?;
        Ok(())
    });
    
    test_framework.add_test("math_division", || {
        facts result = 10.0 / 2.0;
        testing::assert_close_to(result, 5.0, 0.001, "Division should be accurate")?;
        Ok(())
    });
    
    // String tests
    test_framework.add_test("string_operations", || {
        facts text = "Hello, World!";
        testing::assert_true(string::contains(&text, "World"), "Should contain 'World'")?;
        testing::assert_eq(string::length(&text), 13, "Length should be 13")?;
        testing::assert_starts_with(&text, "Hello", "Should start with 'Hello'")?;
        Ok(())
    });
    
    // Collection tests
    test_framework.add_test("collection_operations", || {
        sus vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        testing::assert_eq(vec.length(), 3, "Vector should have 3 elements")?;
        testing::assert_contains_element(&vec, &2, "Vector should contain 2")?;
        testing::assert_not_empty(&vec, "Vector should not be empty")?;
        Ok(())
    });
    
    // Error handling test
    test_framework.add_test("error_handling", || {
        facts result = math::divide(10.0, 0.0);
        testing::assert_error(&result, "Division by zero should return error")?;
        Ok(())
    });
    
    // Benchmark test
    test_framework.add_benchmark("string_concatenation", || {
        sus result = String::new();
        bestie (sus i = 0; i < 1000; i++) {
            result += "a";
        }
        result
    });
    
    spill("  ✅ Added {} test cases", test_framework.test_count());
    spill("  ✅ Added {} benchmarks", test_framework.benchmark_count());
    
    // Run tests
    spill("\n🚀 Running test suite:");
    facts test_results = test_framework.run_all_tests()?;
    
    spill("  📊 Test Results:");
    spill("    Total tests: {}", test_results.total_tests);
    spill("    Passed: {}", test_results.passed_tests);
    spill("    Failed: {}", test_results.failed_tests);
    spill("    Execution time: {} ms", test_results.execution_time_ms);
    
    // Show individual test results
    bestie (sus i = 0; i < test_results.individual_results.length(); i++) {
        facts result = &test_results.individual_results[i];
        facts status_icon = lowkey (result.passed) { "✅" } highkey { "❌" };
        spill("    {} {}: {} ms", status_icon, result.test_name, result.execution_time_ms);
        
        lowkey (!result.passed) {
            spill("      Error: {}", result.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
        }
    }
    
    // Run benchmarks
    spill("\n⚡ Running benchmarks:");
    facts benchmark_results = test_framework.run_all_benchmarks()?;
    
    bestie (sus i = 0; i < benchmark_results.length(); i++) {
        facts bench = &benchmark_results[i];
        spill("  🏃 {}: {:.2} ops/sec", bench.name, bench.operations_per_second);
        spill("    Average time: {:.3} ms", bench.average_time_ms);
        spill("    Min/Max: {:.3}/{:.3} ms", bench.min_time_ms, bench.max_time_ms);
    }
    
    periodt Ok(());
}

/// Demonstrate environment and configuration management
slay function demonstrate_environment() -> Result<(), string> {
    spill("\n🌍 Environment & Configuration Demo");
    spill("===================================\n");
    
    // Environment variables
    spill("🔧 Environment Variables:");
    
    // Set some demo environment variables
    env::set_env("CURSED_DEMO_VAR", "demo_value")?;
    env::set_env("CURSED_DEBUG", "true")?;
    env::set_env("CURSED_PORT", "8080")?;
    env::set_env("CURSED_FEATURES", "crypto,web,database")?;
    
    spill("  ✅ Set demo environment variables");
    
    // Read environment variables
    facts demo_var = env::get_env("CURSED_DEMO_VAR").unwrap_or("not_set".to_string());
    facts debug_mode = env::get_bool_env("CURSED_DEBUG").unwrap_or(false);
    facts port = env::get_int_env("CURSED_PORT").unwrap_or(3000);
    facts features = env::get_env_list("CURSED_FEATURES", ",").unwrap_or(Vec::new());
    
    spill("  📊 Environment values:");
    spill("    CURSED_DEMO_VAR: {}", demo_var);
    spill("    CURSED_DEBUG: {}", debug_mode);
    spill("    CURSED_PORT: {}", port);
    spill("    CURSED_FEATURES: {:?}", features);
    
    // System environment
    spill("\n🖥️  System Environment:");
    facts current_dir = env::get_current_dir()?;
    facts home_dir = env::get_home_dir().unwrap_or("unknown".to_string());
    facts temp_dir = env::get_temp_dir();
    facts username = env::get_username().unwrap_or("unknown".to_string());
    facts hostname = env::get_hostname().unwrap_or("unknown".to_string());
    
    spill("  📁 Current directory: {}", current_dir);
    spill("  🏠 Home directory: {}", home_dir);
    spill("  🗂️  Temp directory: {}", temp_dir);
    spill("  👤 Username: {}", username);
    spill("  🖥️  Hostname: {}", hostname);
    
    // PATH environment variable
    lowkey (facts path_env = env::get_path_env()) {
        spill("\n🛤️  PATH Analysis:");
        spill("    Total PATH entries: {}", path_env.length());
        spill("    First 5 PATH entries:");
        
        bestie (sus i = 0; i < math::min(5, path_env.length()); i++) {
            spill("      {}. {}", i + 1, path_env[i]);
        }
    }
    
    // Environment variable expansion
    spill("\n🔄 Variable Expansion:");
    facts template = "User ${CURSED_DEMO_VAR} is running on port ${CURSED_PORT}";
    facts expanded = env::expand_env_vars(&template)?;
    spill("  📝 Template: {}", template);
    spill("  🔄 Expanded: {}", expanded);
    
    // Configuration parsing
    spill("\n⚙️  Configuration Parsing:");
    facts config_text = "
        database_url=sqlite:///app.db
        api_key=secret_key_123
        max_connections=100
        enable_logging=true
        allowed_origins=localhost,127.0.0.1,example.com
    ";
    
    facts config = env::parse_env_config(&config_text)?;
    spill("  📋 Parsed configuration:");
    
    for (key, value) in config {
        spill("    {}: {}", key, value);
    }
    
    // Cleanup demo variables
    env::remove_env("CURSED_DEMO_VAR")?;
    env::remove_env("CURSED_DEBUG")?;
    env::remove_env("CURSED_PORT")?;
    env::remove_env("CURSED_FEATURES")?;
    spill("  🗑️  Cleaned up demo environment variables");
    
    periodt Ok(());
}

/// Demonstrate JSON processing capabilities
slay function demonstrate_json_processing() -> Result<(), string> {
    spill("\n📄 JSON Processing Demo");
    spill("========================\n");
    
    // JSON serialization
    spill("📝 JSON Serialization:");
    
    facts data = json_tea::JsonObject::new();
    data.insert("name", json_tea::JsonValue::String("CURSED Language".to_string()));
    data.insert("version", json_tea::JsonValue::String("1.0.0".to_string()));
    data.insert("features", json_tea::JsonValue::Array([
        json_tea::JsonValue::String("type_safety".to_string()),
        json_tea::JsonValue::String("concurrency".to_string()),
        json_tea::JsonValue::String("performance".to_string()),
    ].to_vec()));
    data.insert("active", json_tea::JsonValue::Bool(true));
    data.insert("priority", json_tea::JsonValue::Number(100.0));
    
    facts json_value = json_tea::JsonValue::Object(data);
    facts json_string = json_tea::marshal_indent(&json_value, "  ")?;
    
    spill("  📊 Generated JSON:");
    facts lines: Vec<&str> = json_string.lines().collect();
    bestie (sus i = 0; i < lines.length(); i++) {
        spill("    {}", lines[i]);
    }
    
    // JSON parsing
    spill("\n📖 JSON Parsing:");
    facts parsed_value = json_tea::unmarshal(&json_string)?;
    
    match parsed_value {
        json_tea::JsonValue::Object(obj) => {
            spill("  ✅ Successfully parsed JSON object");
            spill("  📊 Object properties:");
            
            for (key, value) in &obj {
                match value {
                    json_tea::JsonValue::String(s) => spill("    {}: '{}' (string)", key, s),
                    json_tea::JsonValue::Number(n) => spill("    {}: {} (number)", key, n),
                    json_tea::JsonValue::Bool(b) => spill("    {}: {} (boolean)", key, b),
                    json_tea::JsonValue::Array(arr) => spill("    {}: array with {} items", key, arr.length()),
                    json_tea::JsonValue::Object(_) => spill("    {}: nested object", key),
                    json_tea::JsonValue::Null => spill("    {}: null", key),
                }
            }
        }
        _ => spill("  ❌ Expected object at root level"),
    }
    
    // JSON validation
    spill("\n✅ JSON Validation:");
    facts test_json_inputs = [
        r#"{"valid": true, "number": 42}"#,
        r#"{"name": "test"}"#,
        r#"{"invalid": json}"#,  // Invalid JSON
        r#"[1, 2, 3, 4, 5]"#,
    ];
    
    bestie (sus i = 0; i < test_json_inputs.length(); i++) {
        facts input = test_json_inputs[i];
        facts is_valid = json_tea::valid(input);
        facts status = lowkey (is_valid) { "✅" } highkey { "❌" };
        
        spill("  {} '{}' -> {}", status, 
              string::truncate(input, 30), 
              lowkey (is_valid) { "valid" } highkey { "invalid" });
    }
    
    // Streaming JSON processing
    spill("\n🌊 Streaming JSON:");
    spill("  📡 Creating JSON stream processor...");
    
    sus encoder = json_tea::new_encoder();
    encoder.set_indent("  ");
    encoder.set_escape_html(false);
    
    spill("  ✅ JSON encoder configured");
    spill("  🔄 Ready for streaming large JSON datasets");
    
    periodt Ok(());
}

/// Main demonstration orchestrator
slay function main() -> Result<(), string> {
    spill("🎊 CURSED Standard Library Comprehensive Demo");
    spill("=============================================\n");
    
    spill("This demo showcases the extensive capabilities of CURSED's");
    spill("standard library with over 30 modules providing enterprise-grade");
    spill("functionality for real-world applications.\n");
    
    // Run all demonstrations
    demonstrate_math_operations()?;
    demonstrate_string_operations()?;
    demonstrate_file_operations()?;
    demonstrate_networking()?;
    demonstrate_database()?;
    demonstrate_cryptography()?;
    demonstrate_process_management()?;
    demonstrate_time_operations()?;
    demonstrate_collections()?;
    demonstrate_testing_framework()?;
    demonstrate_environment()?;
    demonstrate_json_processing()?;
    
    spill("\n🎉 Comprehensive Standard Library Demo Complete!");
    spill("================================================\n");
    
    spill("🏆 CURSED Standard Library Summary:");
    spill("  • ✅ Mathematical operations (basic, advanced, statistics)");
    spill("  • ✅ String manipulation and validation");
    spill("  • ✅ File system operations and metadata");
    spill("  • ✅ Networking (HTTP, DNS, WebSockets)");
    spill("  • ✅ Database integration (SQLite, PostgreSQL, MongoDB)");
    spill("  • ✅ Cryptography (hashing, encryption, signatures, KDF)");
    spill("  • ✅ Process management and system monitoring");
    spill("  • ✅ Time/date operations and timezone handling");
    spill("  • ✅ Collections and data structures");
    spill("  • ✅ Testing framework with benchmarks");
    spill("  • ✅ Environment and configuration management");
    spill("  • ✅ JSON processing and streaming");
    
    spill("\n💡 Additional Available Modules:");
    spill("  • 🔐 Complete cryptography ecosystem (10+ modules)");
    spill("  • 🌐 Web framework (web_vibez) for full-stack development");
    spill("  • 🏗️  Template system with multiple format support");
    spill("  • ⚛️  Atomic operations for low-level synchronization");
    spill("  • 📡 Signal handling and OS integration");
    spill("  • 🔗 Inter-process communication (IPC)");
    spill("  • 📊 Performance profiling and monitoring");
    spill("  • 🧵 Threading and synchronization primitives");
    spill("  • 🔄 Asynchronous programming support");
    spill("  • 📝 Comprehensive logging framework");
    
    spill("\n🚀 CURSED proves that programming languages can be:");
    spill("  💅 Fun and engaging with Gen Z slang syntax");
    spill("  🏭 Production-ready with enterprise features");
    spill("  🔒 Safe with strong typing and memory management");
    spill("  ⚡ Fast with LLVM-based compilation");
    spill("  🌍 Comprehensive with extensive standard library");
    
    periodt Ok(());
}
