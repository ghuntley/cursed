fr fr CURSED Debug Integration Demo
fr fr This file demonstrates comprehensive debug information generation
fr fr for the CURSED programming language compiler

fr fr Simple function with debug information
slay main(sus argc, tea argv) -> sus {
    // Variable declarations with debug info
    sus result = 42;
    facts debug_enabled = based;
    vibes calculation = 3.14159;
    tea message = "Debug information works!";
    
    // Control flow with debug tracking
    lowkey (debug_enabled) {
        // Loop with debug locations
        sus i = 0;
        periodt (i < 5) {
            result = result + i;
            i = i + 1;
            yolo; // Yield point for concurrent debugging
        }
    } highkey {
        result = 0;
    }
    
    // Function call with debug info
    sus final_result = calculate(result, 10);
    
    bestie; // Return with debug location
    final_result;
}

fr fr Helper function with parameter debug info
slay calculate(sus x, sus y) -> sus {
    sus temp = x * 2;
    vibes factor = 1.5;
    
    // Conditional with debug tracking
    lowkey (x > y) {
        bestie temp + y;
    } highkey {
        bestie x + y;
    }
}

fr fr Struct with debug information
squad Person {
    tea name;
    sus age;
    facts is_active;
}

fr fr Interface with debug information
collab Drawable {
    yolo draw() -> void;
    yolo get_area() -> vibes;
}

fr fr Error handling with debug info
slay risky_operation() -> Result<sus, tea> {
    sus value = 42;
    
    // Error propagation with debug tracking
    sus result = might_fail(value)?;
    
    Ok(result)
}

fr fr Concurrent function with debug support
slay concurrent_task() -> void {
    // Channel operations with debug info
    chan<sus> ch = make_chan<sus>(10);
    
    // Goroutine spawn with debug tracking
    stan send_data(ch);
    stan receive_data(ch);
    
    // Synchronization with debug info
    ch.close();
}

fr fr Generic function with debug information
slay generic_function<T>(value: T) -> T {
    // Type constraints with debug tracking
    bestie value;
}

fr fr Function with complex control flow
slay complex_function(sus input) -> sus {
    sus result = 0;
    
    // Nested blocks with debug scope tracking
    {
        sus local_var = input * 2;
        
        // Switch statement with debug info
        vibe_check input {
            mood 1 -> {
                result = local_var + 10;
            }
            mood 2 -> {
                result = local_var + 20;
            }
            basic -> {
                result = local_var;
            }
        }
    }
    
    // Loop with break/continue debug tracking
    periodt (result < 100) {
        lowkey (result % 2 == 0) {
            flex; // Continue with debug location
        }
        
        result = result + 1;
        
        lowkey (result > 50) {
            period; // Break with debug location
        }
    }
    
    bestie result;
}

fr fr Error handling function
slay error_demo() -> void {
    // Panic with debug information
    panic("This is a debug-tracked panic!");
}

fr fr Memory management with debug tracking
slay memory_demo() -> void {
    // Allocation with debug info
    sus* ptr = allocate<sus>(100);
    
    // Pointer operations with debug tracking
    *ptr = 42;
    
    // Deallocation with debug info
    deallocate(ptr);
}
