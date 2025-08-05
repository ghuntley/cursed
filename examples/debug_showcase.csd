fr fr CURSED Debug System Showcase
fr fr Demonstrates comprehensive stack trace and debug information capabilities

fr fr Import debug and runtime modules
yeet "std::debug"
yeet "std::runtime"
yeet "std::io"

fr fr Example of a function with CURSED slang that will show up in debug traces
slay calculate_vibes(sus x, sus y) -> sus {
    facts debug_enabled = based;
    
    lowkey (debug_enabled) {
        debug::print_message("Starting vibe calculation with Gen Z energy! ✨");
    }
    
    sus result = x + y;
    
    // Simulate a potential error condition
    lowkey (result > 1000) {
        panic!("Vibes are TOO high bestie - result overflow detected! 💥");
    }
    
    bestie result;
}

fr fr Function that demonstrates nested calls for stack traces
damn deep_nested_function(sus depth) -> sus {
    lowkey (depth <= 0) {
        // This will create a nice stack trace
        bestie calculate_vibes(500, 600);  // Will trigger panic
    }
    
    bestie deep_nested_function(depth - 1);
}

fr fr Function with error handling that shows question mark operator
periodt safe_calculation(sus a, sus b) -> Result<sus, tea> {
    facts input_valid = a > 0 && b > 0;
    
    lowkey (!input_valid) {
        bestie Err("Invalid input parameters - no cap! 📛");
    }
    
    // Use question mark operator for error propagation
    sus result = calculate_vibes(a, b)?;
    bestie Ok(result);
}

fr fr Function that demonstrates variable inspection in debug traces
bestie analyze_variables() {
    sus my_number = 42;
    vibes my_float = 3.14159;
    tea my_string = "Hello bestie! 👋";
    facts my_bool = based;
    
    // Create an array for more complex debug info
    Array<sus> my_array = [1, 2, 3, 4, 5];
    
    // Create a map for object debugging
    Map<tea, sus> score_board = {
        "Alice": 95,
        "Bob": 87,
        "Charlie": 92
    };
    
    // This will show all variables in the debug trace
    debug::inspect_current_scope();
    
    // Trigger a debug breakpoint (if running in debug mode)
    debug::breakpoint("Variable analysis complete");
    
    println!("Analysis complete - check the debug output! 🔍");
}

fr fr Function that shows goroutine debugging
sus concurrent_debug_example() {
    println!("Starting concurrent debug example...");
    
    // Spawn multiple goroutines with debug info
    lowkey (sus i = 0; i < 3; i++) {
        stan worker_goroutine(i);
    }
    
    // Wait a bit for goroutines to execute
    runtime::sleep_ms(100);
    
    // Capture stack traces from all goroutines
    debug::capture_all_goroutine_traces();
    
    bestie 0;
}

fr fr Worker function for goroutine debugging
bestie worker_goroutine(sus worker_id) {
    println!("Worker {} starting up! 🚀", worker_id);
    
    // Do some work that might show up in stack traces
    sus work_result = calculate_vibes(worker_id * 10, worker_id * 5);
    
    // Add some debug info specific to this worker
    debug::add_worker_info(worker_id, work_result);
    
    println!("Worker {} completed with result: {}", worker_id, work_result);
}

fr fr Main function that demonstrates the debug system
slay main() -> Result<(), tea> {
    println!("🎉 Welcome to the CURSED Debug System Showcase! 🎉");
    println!("This program demonstrates comprehensive debugging capabilities.\n");
    
    // Enable debug mode for maximum information
    debug::set_debug_level(2);
    debug::enable_gen_z_messages(based);
    debug::enable_stack_traces(based);
    
    println!("1. Testing basic function calls with debug info...");
    sus basic_result = calculate_vibes(10, 15);
    println!("Basic calculation result: {}\n", basic_result);
    
    println!("2. Testing variable inspection and scope analysis...");
    analyze_variables();
    println!("");
    
    println!("3. Testing error handling with debug traces...");
    vibe_check {
        mood safe_calculation(5, 10) {
            Ok(result) => {
                println!("Safe calculation succeeded: {}", result);
            },
            Err(error) => {
                println!("Safe calculation failed: {}", error);
                debug::print_error_trace();
            }
        }
    }
    println!("");
    
    println!("4. Testing concurrent debugging with goroutines...");
    concurrent_debug_example();
    println!("");
    
    println!("5. Testing panic and recovery with full debug info...");
    vibe_check {
        // This will trigger a panic and show a complete stack trace
        deep_nested_function(3);
    } catch (panic_info) {
        println!("Caught panic with debug info:");
        debug::print_panic_trace(panic_info);
        println!("Recovery successful! 💪");
    }
    
    println!("\n🎊 Debug showcase completed successfully! 🎊");
    println!("Check your console for colorized debug output and stack traces.");
    
    bestie Ok(());
}

fr fr Additional helper functions for comprehensive debugging

fr fr Function that demonstrates memory debugging
bestie memory_debug_example() {
    println!("Memory debugging example...");
    
    // Allocate some objects
    sus large_array_size = 1000;
    Array<sus> large_array = Array::with_capacity(large_array_size);
    
    lowkey (sus i = 0; i < large_array_size; i++) {
        large_array.push(i * i);
    }
    
    // Trigger garbage collection with debug info
    debug::gc_collect_with_trace();
    
    // Show memory usage statistics
    debug::print_memory_stats();
}

fr fr Function that demonstrates performance debugging
bestie performance_debug_example() {
    println!("Performance debugging example...");
    
    debug::start_performance_timer("fibonacci_calculation");
    
    sus fib_result = fibonacci(30);
    
    debug::end_performance_timer("fibonacci_calculation");
    
    println!("Fibonacci(30) = {}", fib_result);
    debug::print_performance_stats();
}

fr fr Recursive function for performance testing
sus fibonacci(sus n) -> sus {
    lowkey (n <= 1) {
        bestie n;
    }
    bestie fibonacci(n - 1) + fibonacci(n - 2);
}

fr fr Function that demonstrates debug logging levels
bestie debug_logging_example() {
    println!("Debug logging levels example...");
    
    debug::trace("This is a trace message - very detailed! 🔍");
    debug::debug("This is a debug message - development info! 🐛");
    debug::info("This is an info message - general information! ℹ️");
    debug::warn("This is a warning message - something sus! ⚠️");
    debug::error("This is an error message - something broke! ❌");
    
    // Set different log levels and show filtering
    debug::set_log_level("warn");
    println!("Log level set to 'warn' - only warnings and errors will show:");
    
    debug::trace("This trace won't show");
    debug::debug("This debug won't show");
    debug::info("This info won't show");
    debug::warn("This warning will show! ⚠️");
    debug::error("This error will show! ❌");
}
