# pure_cursed_runtime

Pure CURSED runtime bridge providing essential runtime functions through minimal C shims. Offers a clean interface to system functionality while maintaining CURSED's pure implementation philosophy.

## Overview

The `pure_cursed_runtime` module provides:
- Essential I/O operations through minimal C shims
- String manipulation functions
- File system operations
- Time and timing functions
- Cryptographic primitives
- Clean bridge between CURSED and system functionality

## Design Philosophy

This module serves as a **bridge layer** that:
- Provides essential runtime functions not implementable in pure CURSED
- Uses minimal C shims instead of full FFI for better performance
- Maintains clean, simple interfaces
- Enables pure CURSED implementations to access system resources

## Core Functions

### I/O Operations

#### `print(message: tea) -> lit`
Prints message to standard output through minimal C shim.

**Parameters:**
- `message`: Text to print

**Returns:** `based` if successful, `cringe` if failed

**Example:**
```cursed
yeet "pure_cursed_runtime"

lowkey print("Hello, CURSED!") {
    // Print succeeded
} yikes {
    // Print failed
}
```

#### `println(message: tea) -> lit`
Prints message with newline to standard output.

**Parameters:**
- `message`: Text to print

**Returns:** Success status

**Example:**
```cursed
println("This line ends with newline")
println("So does this one")
```

#### `read_line() -> tea`
Reads a line of input from standard input.

**Returns:** Input line as string

**Example:**
```cursed
print("Enter your name: ")
sus name tea = read_line()
println("Hello, " + name + "!")
```

### String Operations

#### `string_length(s: tea) -> normie`
Returns length of string through efficient C shim.

**Parameters:**
- `s`: String to measure

**Returns:** Length in characters

**Example:**
```cursed
sus text tea = "Hello, World!"
sus length normie = string_length(text)
println("Length: " + string(length))  // Length: 13
```

#### `string_concat(a: tea, b: tea) -> tea`
Concatenates two strings efficiently.

**Parameters:**
- `a`: First string
- `b`: Second string

**Returns:** Concatenated string

**Example:**
```cursed
sus first tea = "Hello, "
sus second tea = "World!"
sus result tea = string_concat(first, second)
println(result)  // Hello, World!
```

### File System Operations

#### `file_exists(path: tea) -> lit`
Checks if file exists at specified path.

**Parameters:**
- `path`: File path to check

**Returns:** `based` if file exists, `cringe` otherwise

**Example:**
```cursed
lowkey file_exists("config.txt") {
    println("Configuration file found")
} yikes {
    println("Configuration file missing")
}
```

#### `file_read(path: tea) -> tea`
Reads entire file contents as string.

**Parameters:**
- `path`: File path to read

**Returns:** File contents or empty string on error

**Example:**
```cursed
sus content tea = file_read("data.txt")
lowkey string_length(content) > 0 {
    println("File content: " + content)
} yikes {
    println("Could not read file")
}
```

#### `file_write(path: tea, content: tea) -> lit`
Writes content to file, creating or overwriting as needed.

**Parameters:**
- `path`: File path to write
- `content`: Data to write

**Returns:** `based` if successful

**Example:**
```cursed
sus data tea = "Hello, file system!"
lowkey file_write("output.txt", data) {
    println("File written successfully")
} yikes {
    println("Failed to write file")
}
```

### Time and Timing

#### `time_now_ms() -> normie`
Gets current time in milliseconds since epoch.

**Returns:** Current timestamp

**Example:**
```cursed
sus start_time normie = time_now_ms()
// ... do some work ...
sus end_time normie = time_now_ms()
sus duration normie = end_time - start_time
println("Operation took " + string(duration) + " ms")
```

#### `sleep_ms(ms: normie)`
Sleeps for specified number of milliseconds.

**Parameters:**
- `ms`: Milliseconds to sleep

**Example:**
```cursed
println("Starting countdown...")
bestie i := 3; i > 0; i = i - 1 {
    println(string(i))
    sleep_ms(1000)  // Sleep for 1 second
}
println("Go!")
```

### Cryptographic Primitives

#### `sha256(data: tea) -> tea`
Computes SHA-256 hash of input data.

**Parameters:**
- `data`: Input data to hash

**Returns:** SHA-256 hash as hexadecimal string

**Example:**
```cursed
sus message tea = "Hello, CURSED!"
sus hash tea = sha256(message)
println("SHA-256: " + hash)
```

#### `random_bytes(length: normie) -> tea`
Generates cryptographically secure random bytes.

**Parameters:**
- `length`: Number of random bytes to generate

**Returns:** Random bytes as string

**Example:**
```cursed
sus random_data tea = random_bytes(16)
sus hex_random tea = bytes_to_hex(random_data)
println("Random: " + hex_random)
```

## Usage Examples

### Basic I/O Application

```cursed
yeet "pure_cursed_runtime"

slay main() {
    println("=== CURSED Runtime Demo ===")
    
    // Get user input
    print("Enter your message: ")
    sus message tea = read_line()
    
    // Process message
    sus length normie = string_length(message)
    println("Your message has " + string(length) + " characters")
    
    // Create hash
    sus hash tea = sha256(message)
    println("SHA-256 hash: " + hash)
    
    // Save to file
    sus filename tea = "user_message.txt"
    sus full_content tea = string_concat("Message: ", message)
    full_content = string_concat(full_content, "\nHash: ")
    full_content = string_concat(full_content, hash)
    
    lowkey file_write(filename, full_content) {
        println("Message saved to " + filename)
    } yikes {
        println("Failed to save message")
    }
}
```

### File Processing Application

```cursed
slay process_files() {
    sus files []tea = []tea{"input1.txt", "input2.txt", "input3.txt"}
    sus output_file tea = "combined_output.txt"
    sus combined_content tea = ""
    
    // Process each input file
    bestie i := 0; i < len(files); i = i + 1 {
        sus filename tea = files[i]
        
        lowkey file_exists(filename) {
            println("Processing: " + filename)
            
            sus content tea = file_read(filename)
            sus timestamp tea = string(time_now_ms())
            
            // Add content with timestamp
            combined_content = string_concat(combined_content, "=== ")
            combined_content = string_concat(combined_content, filename)
            combined_content = string_concat(combined_content, " (")
            combined_content = string_concat(combined_content, timestamp)
            combined_content = string_concat(combined_content, ") ===\n")
            combined_content = string_concat(combined_content, content)
            combined_content = string_concat(combined_content, "\n\n")
        } yikes {
            println("Warning: " + filename + " not found")
        }
    }
    
    // Write combined output
    lowkey file_write(output_file, combined_content) {
        println("Combined output written to " + output_file)
    }
}
```

### Performance Timing

```cursed
slay benchmark_operation() {
    println("Benchmarking string operations...")
    
    sus iterations normie = 10000
    sus start_time normie = time_now_ms()
    
    // Perform string operations
    sus result tea = ""
    bestie i := 0; i < iterations; i = i + 1 {
        sus temp tea = "iteration_"
        temp = string_concat(temp, string(i))
        result = string_concat(result, temp)
        
        // Add some delay periodically
        lowkey i % 1000 == 0 {
            sleep_ms(1)
        }
    }
    
    sus end_time normie = time_now_ms()
    sus duration normie = end_time - start_time
    sus final_length normie = string_length(result)
    
    println("Completed " + string(iterations) + " iterations")
    println("Duration: " + string(duration) + " ms")
    println("Final string length: " + string(final_length))
    println("Operations per second: " + string(iterations * 1000 / duration))
}
```

### Cryptographic Utilities

```cursed
slay crypto_demo() {
    println("=== Cryptographic Demo ===")
    
    // Generate random salt
    sus salt tea = random_bytes(16)
    println("Generated salt (" + string(string_length(salt)) + " bytes)")
    
    // Hash with salt
    sus password tea = "user_password_123"
    sus salted_password tea = string_concat(password, salt)
    sus password_hash tea = sha256(salted_password)
    
    println("Password hash: " + password_hash)
    
    // Verify password
    print("Enter password to verify: ")
    sus input_password tea = read_line()
    sus input_salted tea = string_concat(input_password, salt)
    sus input_hash tea = sha256(input_salted)
    
    lowkey input_hash == password_hash {
        println("✅ Password correct!")
    } yikes {
        println("❌ Password incorrect!")
    }
}
```

## C Shim Implementation

The module relies on minimal C shims that provide system functionality:

### C Function Signatures

```c
// I/O operations
int cursed_print(const char* message);
int cursed_println(const char* message);
char* cursed_read_line(void);

// String operations
int cursed_string_length(const char* s);
char* cursed_string_concat(const char* a, const char* b);

// File operations
int cursed_file_exists(const char* path);
char* cursed_file_read(const char* path);
int cursed_file_write(const char* path, const char* content);

// Time operations
long cursed_time_now_ms(void);
void cursed_time_sleep_ms(int ms);

// Crypto operations
char* cursed_crypto_sha256(const char* data);
char* cursed_crypto_random_bytes(int length);
```

### Memory Management

The C shims handle memory management automatically:
- String returns are managed by the runtime
- No explicit memory deallocation needed from CURSED
- Automatic cleanup on application exit

## Performance Characteristics

### Function Performance

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| print/println | O(n) | Where n is string length |
| string_length | O(1) | C strlen optimization |
| string_concat | O(n+m) | Where n,m are string lengths |
| file_read | O(n) | Where n is file size |
| file_write | O(n) | Where n is content size |
| sha256 | O(n) | Where n is input size |
| random_bytes | O(n) | Where n is byte count |

### Memory Usage

- **Minimal overhead**: C shims use standard library functions
- **No dynamic allocation in CURSED**: All handled by runtime
- **Efficient string operations**: Direct C implementation
- **Small runtime footprint**: < 50KB for all shims

## Error Handling

### Error Conventions

- **Boolean returns**: `based` for success, `cringe` for failure
- **String returns**: Empty string indicates error
- **Integer returns**: -1 or 0 typically indicates error

### Robust Error Handling

```cursed
slay safe_file_operation(filename tea) {
    // Check existence before reading
    lowkey !file_exists(filename) {
        println("Error: File does not exist: " + filename)
        damn
    }
    
    // Attempt to read
    sus content tea = file_read(filename)
    lowkey string_length(content) == 0 {
        println("Error: Could not read file or file is empty")
        damn
    }
    
    // Process content safely
    println("File content (" + string(string_length(content)) + " bytes):")
    println(content)
}
```

## Testing

### Unit Tests

```cursed
// Test basic runtime functions
slay test_runtime_functions() {
    test_start("Runtime Functions")
    
    // Test string operations
    sus test_str tea = "test"
    assert_eq_int(string_length(test_str), 4)
    
    sus concatenated tea = string_concat("hello", "world")
    assert_eq_int(string_length(concatenated), 10)
    
    // Test I/O operations
    assert_true(print("Test print"))
    assert_true(println("Test println"))
    
    // Test file operations
    sus test_content tea = "test file content"
    assert_true(file_write("test.tmp", test_content))
    assert_true(file_exists("test.tmp"))
    
    sus read_content tea = file_read("test.tmp")
    assert_eq_string(read_content, test_content)
    
    print_test_summary()
}
```

### Integration Tests

```bash
# Run runtime bridge tests
zig build test
./zig-out/bin/cursed-zig stdlib/pure_cursed_runtime/test_pure_cursed_runtime.csd
```

## Dependencies

**External Dependencies:**
- Minimal C runtime library (libc)
- System calls for I/O operations

**CURSED Dependencies:**
- None - this module provides foundation functionality

## Architecture

### Bridge Design

1. **CURSED Interface Layer**: Clean, typed function signatures
2. **Bridge Layer**: Minimal translation between CURSED and C
3. **C Shim Layer**: Efficient system call wrappers
4. **System Layer**: Operating system functionality

### Design Principles

- **Minimal Interface**: Only essential functions
- **Clean Abstractions**: Hide system complexity
- **Performance Focus**: Direct C implementations
- **Error Transparency**: Clear error reporting

### Extension Points

```cursed
// Framework for adding new runtime functions
slay add_runtime_function(name tea, implementation tea) {
    // Register new C shim
    // Update CURSED interface
    // Provide documentation
}
```

## Integration Examples

### Application Startup

```cursed
// Main application entry point
slay main() {
    // Runtime is automatically available
    println("CURSED Application Starting...")
    
    // Use runtime functions throughout application
    run_application()
    
    println("Application Completed")
}
```

### Library Integration

```cursed
// Other modules can use runtime functions
yeet "pure_cursed_runtime"

slay log_message(level tea, message tea) {
    sus timestamp tea = string(time_now_ms())
    sus log_entry tea = string_concat("[", timestamp)
    log_entry = string_concat(log_entry, "] ")
    log_entry = string_concat(log_entry, level)
    log_entry = string_concat(log_entry, ": ")
    log_entry = string_concat(log_entry, message)
    
    println(log_entry)
    
    // Also write to log file
    file_write("application.log", log_entry)
}
```

## Future Enhancements

### Planned Additions

1. **Network Operations**: Basic socket functionality
2. **Process Management**: Process spawning and control
3. **Memory Operations**: Direct memory manipulation
4. **Advanced I/O**: Non-blocking I/O operations

### Architecture Evolution

The module is designed to evolve from basic shims to a comprehensive runtime system while maintaining backwards compatibility and performance.

The `pure_cursed_runtime` module provides the essential bridge between CURSED's pure implementation philosophy and the practical need for system functionality, enabling robust applications while maintaining clean abstractions.
