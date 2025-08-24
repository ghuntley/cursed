# Standard Library Getting Started Guide

## Overview

The CURSED standard library provides 50+ modules covering everything from basic I/O to advanced cryptography. This guide helps you get started with the most commonly used modules.

## Essential Modules Quick Start

### vibez - Basic I/O and Output

The most fundamental module for any CURSED program.

```cursed
yeet "vibez"

// Basic output
vibez.spill("Hello, CURSED!")

// Formatted output  
vibez.spill("User:", "Alice", "Age:", 25)

// Colored output
vibez.spill(vibez.color("Success!", "green"))
vibez.spill(vibez.color("Warning:", "yellow") + " Check your input")

// Input
vibez.spill_no_newline("Enter your name: ")
sus name tea = vibez.read_line()
vibez.spill("Hello,", name)
```

**Key Functions:**
- `spill(...)` - Print with newline
- `spill_no_newline(...)` - Print without newline
- `spill_error(...)` - Print to stderr
- `read_line()` - Read user input
- `color(text, color)` - Apply colors
- `format(template, ...)` - String formatting

### mathz - Mathematical Operations

Comprehensive math functions for calculations and algorithms.

```cursed
yeet "mathz"

// Basic operations
sus result drip = mathz.abs(-5)        // 5
sus maximum drip = mathz.max(10, 20)   // 20
sus power drip = mathz.pow(2, 3)       // 8

// Trigonometry
sus sine drip = mathz.sin(mathz.PI / 2)  // 1.0
sus angle drip = mathz.degrees(mathz.PI) // 180.0

// Statistics
sus data []drip = [1, 2, 3, 4, 5]
sus average drip = mathz.mean(data)      // 3.0
sus total drip = mathz.sum(data)         // 15.0

// Random numbers
mathz.set_random_seed(12345)
sus random_int drip = mathz.random_int(1, 10)
sus random_float drip = mathz.random()   // 0.0 to 1.0
```

**Key Functions:**
- Basic: `abs()`, `min()`, `max()`, `clamp()`
- Powers: `pow()`, `sqrt()`, `cbrt()`
- Trig: `sin()`, `cos()`, `tan()`, `degrees()`, `radians()`
- Stats: `mean()`, `sum()`, `std_dev()`, `median()`
- Random: `random()`, `random_int()`, `set_random_seed()`

### stringz - String Processing

Unicode-aware string manipulation and processing.

```cursed
yeet "stringz"

sus text tea = "Hello, CURSED World!"

// Basic operations
sus length drip = stringz.length(text)           // 20
sus upper tea = stringz.to_upper(text)           // "HELLO, CURSED WORLD!"
sus lower tea = stringz.to_lower(text)           // "hello, cursed world!"

// Searching and checking
sus contains lit = stringz.contains(text, "CURSED")  // based
sus starts lit = stringz.starts_with(text, "Hello")  // based
sus position drip = stringz.find(text, "World")      // 14

// Splitting and joining
sus words []tea = stringz.split(text, " ")
// ["Hello,", "CURSED", "World!"]

sus joined tea = stringz.join(words, "-")
// "Hello,-CURSED-World!"

// Trimming and padding
sus padded tea = stringz.pad_left("42", 5, "0")     // "00042"
sus trimmed tea = stringz.trim("  hello  ")          // "hello"

// Replacement
sus replaced tea = stringz.replace(text, "World", "Universe")
// "Hello, CURSED Universe!"
```

**Key Functions:**
- Info: `length()`, `is_empty()`, `contains()`, `starts_with()`, `ends_with()`
- Transform: `to_upper()`, `to_lower()`, `trim()`, `replace()`
- Split/Join: `split()`, `join()`, `lines()`
- Format: `pad_left()`, `pad_right()`, `format_number()`

### testz - Testing Framework

Comprehensive testing for reliable code development.

```cursed
yeet "testz"
yeet "mathz"

// Basic test structure
slay test_basic_math() {
    testz.start_test("Basic Addition")
    
    sus result drip = 2 + 3
    testz.assert_eq_int(result, 5, "2 + 3 should equal 5")
    
    testz.end_test()
}

slay test_string_operations() {
    testz.start_test("String Length")
    
    sus text tea = "Hello"
    testz.assert_eq_int(text.length(), 5)
    testz.assert_true(text.contains("ell"))
    
    testz.end_test()
}

// Error testing
slay test_division_by_zero() {
    testz.start_test("Division Error Handling")
    
    sus result = safe_divide(10, 0)
    testz.assert_error(result, "division by zero")
    
    testz.end_test()
}

slay main() {
    testz.start_suite("Basic Operations Tests")
    
    test_basic_math()
    test_string_operations() 
    test_division_by_zero()
    
    testz.print_summary()
}
```

**Key Functions:**
- Setup: `start_suite()`, `start_test()`, `end_test()`, `print_summary()`
- Assertions: `assert_true()`, `assert_eq_int()`, `assert_eq_string()`, `assert_error()`
- Advanced: `benchmark()`, `property_test()`, `assert_panic()`

### concurrenz - Concurrency and Channels

Powerful concurrent programming with goroutines and channels.

```cursed
yeet "concurrenz"

// Basic goroutine
go {
    vibez.spill("Running concurrently!")
    timez.sleep(1000)  // 1 second
    vibez.spill("Goroutine finished")
}

// Channel communication
sus ch chan<drip> = make_channel()

// Producer goroutine
go {
    bestie (sus i drip = 1; i <= 5; i++) {
        ch <- i
        timez.sleep(500)
    }
    close(ch)
}

// Consumer
ready (lit) {
    sus value drip, sus ok lit = <-ch
    ready (!ok) {
        vibez.spill("Channel closed")
        break
    }
    vibez.spill("Received:", value)
}

// Buffered channels
sus buffered chan<tea> = make_buffered_channel(3)
buffered <- "message1"
buffered <- "message2"
buffered <- "message3"  // Won't block

// Worker pool pattern
sus jobs chan<drip> = make_channel()
sus results chan<drip> = make_channel()

// Start 3 workers
bestie (sus w drip = 1; w <= 3; w++) {
    go {
        ready (lit) {
            sus job drip, sus ok lit = <-jobs
            ready (!ok) { break }
            
            // Process job
            sus result drip = job * job
            results <- result
        }
    }
}

// Send jobs
bestie (sus j drip = 1; j <= 9; j++) {
    jobs <- j
}
close(jobs)

// Collect results
bestie (sus r drip = 1; r <= 9; r++) {
    sus result drip = <-results
    vibez.spill("Result:", result)
}
```

**Key Concepts:**
- `go { ... }` - Spawn goroutine
- `make_channel<T>()` - Create channel
- `channel <- value` - Send to channel
- `value := <-channel` - Receive from channel
- `close(channel)` - Close channel
- `select { ... }` - Multi-channel operations

### filez - File System Operations

File and directory operations with error handling.

```cursed
yeet "filez"

// Read file
sus content tea = filez.read_file("config.txt") fam {
    when "file not found" -> {
        vibez.spill_error("Config file missing")
        damn ""
    }
    when _ -> {
        vibez.spill_error("Failed to read config file")
        damn ""
    }
}

// Write file
filez.write_file("output.txt", "Hello, World!") fam {
    when "permission denied" -> {
        vibez.spill_error("Cannot write to file")
    }
    when _ -> {
        vibez.spill_error("Write failed")
    }
}

// File information
ready (filez.exists("data.json")) {
    sus info FileInfo = filez.get_info("data.json") fam {
        when _ -> damn FileInfo{}
    }
    
    vibez.spill("File size:", info.size, "bytes")
    vibez.spill("Modified:", info.modified_time)
    vibez.spill("Is directory:", info.is_directory)
}

// Directory operations
filez.create_directory("logs") fam {
    when "already exists" -> vibez.spill("Directory exists")
    when _ -> vibez.spill_error("Failed to create directory")
}

sus files []tea = filez.list_directory(".")
bestie (sus file tea : files) {
    vibez.spill("File:", file)
}

// Path operations
sus absolute tea = filez.absolute_path("../config")
sus joined tea = filez.join_path("home", "user", "documents")
sus extension tea = filez.get_extension("document.pdf")  // "pdf"
```

**Key Functions:**
- File I/O: `read_file()`, `write_file()`, `append_file()`
- Info: `exists()`, `get_info()`, `get_size()`
- Directories: `list_directory()`, `create_directory()`, `remove_directory()`
- Paths: `join_path()`, `absolute_path()`, `get_extension()`, `get_directory()`

## Working with Multiple Modules

### Web Server Example
```cursed
yeet "vibez"
yeet "networkz" 
yeet "jsonz"
yeet "filez"

struct APIResponse {
    status drip,
    message tea,
    data map[tea]normie
}

slay handle_users(req HTTPRequest) HTTPResponse {
    ready (req.method == "GET") {
        // Read user data from file
        sus users_json tea = filez.read_file("users.json") fam {
            when _ -> damn create_error_response(500, "Failed to read users")
        }
        
        sus users []User = jsonz.parse(users_json) fam {
            when _ -> damn create_error_response(400, "Invalid user data")
        }
        
        sus response APIResponse = APIResponse{
            status: 200,
            message: "Users retrieved successfully",
            data: {"users": users}
        }
        
        damn HTTPResponse{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: jsonz.stringify(response)
        }
    } 
    
    damn create_error_response(405, "Method not allowed")
}

slay create_error_response(status drip, message tea) HTTPResponse {
    sus error_response APIResponse = APIResponse{
        status: status,
        message: message,
        data: {}
    }
    
    damn HTTPResponse{
        status: status,
        headers: {"Content-Type": "application/json"},
        body: jsonz.stringify(error_response)
    }
}

slay main() {
    vibez.spill("Starting web server...")
    
    sus server HTTPServer = networkz.create_server("localhost", 8080)
    server.route("/api/users", handle_users)
    
    vibez.spill("Server running on http://localhost:8080")
    server.listen()
}
```

### Data Processing Pipeline
```cursed
yeet "filez"
yeet "stringz"
yeet "mathz"
yeet "csvz"

struct SalesRecord {
    date tea,
    product tea,
    amount drip,
    region tea
}

slay process_sales_data() {
    // Read CSV file
    sus csv_content tea = filez.read_file("sales.csv") fam {
        when _ -> {
            vibez.spill_error("Failed to read sales data")
            damn
        }
    }
    
    // Parse CSV
    sus records []SalesRecord = csvz.parse(csv_content, SalesRecord) fam {
        when _ -> {
            vibez.spill_error("Failed to parse CSV")
            damn
        }
    }
    
    // Process data
    sus total_sales drip = 0
    sus regional_sales map[tea]drip = {}
    
    bestie (sus record SalesRecord : records) {
        total_sales += record.amount
        
        ready (regional_sales.has_key(record.region)) {
            regional_sales[record.region] += record.amount
        } otherwise {
            regional_sales[record.region] = record.amount
        }
    }
    
    // Generate report
    vibez.spill("Sales Report")
    vibez.spill("=" + repeat("=", 40))
    vibez.spill("Total Sales: $" + mathz.format_currency(total_sales))
    vibez.spill("")
    vibez.spill("Regional Breakdown:")
    
    bestie (sus region tea, sus amount drip : regional_sales) {
        sus percentage drip = (amount * 100) / total_sales
        vibez.spill("  " + region + ": $" + mathz.format_currency(amount) + 
                   " (" + mathz.round(percentage, 1).(tea) + "%)")
    }
    
    // Save summary
    sus summary tea = generate_summary_report(total_sales, regional_sales)
    filez.write_file("sales_summary.txt", summary)
    vibez.spill("")
    vibez.spill("Summary saved to sales_summary.txt")
}
```

### Concurrent Data Fetcher
```cursed
yeet "networkz"
yeet "concurrenz"
yeet "jsonz"

struct APIResult {
    url tea,
    data tea,
    error tea,
    duration drip
}

slay fetch_multiple_apis(urls []tea) []APIResult {
    sus results chan<APIResult> = make_channel()
    sus wg WaitGroup = WaitGroup{}
    
    // Launch goroutine for each URL
    bestie (sus url tea : urls) {
        wg.add(1)
        go {
            shook wg.done()
            
            sus start drip = timez.get_time_microseconds()
            sus data tea = networkz.get(url) fam {
                when error -> {
                    sus duration drip = timez.get_time_microseconds() - start
                    results <- APIResult{url, "", error, duration}
                    damn
                }
            }
            
            sus duration drip = timez.get_time_microseconds() - start
            results <- APIResult{url, data, "", duration}
        }
    }
    
    // Close results channel when all goroutines finish
    go {
        wg.wait()
        close(results)
    }
    
    // Collect results
    sus all_results []APIResult = []
    ready (lit) {
        sus result APIResult, sus ok lit = <-results
        ready (!ok) { break }
        all_results = append(all_results, result)
    }
    
    damn all_results
}

slay main() {
    sus urls []tea = [
        "https://api.github.com/users/octocat",
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://httpbin.org/json"
    ]
    
    vibez.spill("Fetching data from", urls.length(), "APIs...")
    
    sus results []APIResult = fetch_multiple_apis(urls)
    
    bestie (sus result APIResult : results) {
        ready (result.error == "") {
            vibez.spill(vibez.color("✅ " + result.url, "green"))
            vibez.spill("  Duration: " + (result.duration / 1000).(tea) + "ms")
            vibez.spill("  Data length: " + result.data.length().(tea) + " bytes")
        } otherwise {
            vibez.spill(vibez.color("❌ " + result.url, "red"))
            vibez.spill("  Error: " + result.error)
        }
        vibez.spill("")
    }
}
```

## Best Practices

### Error Handling
```cursed
// Always handle errors explicitly
sus result = risky_operation() fam {
    when "specific error" -> {
        // Handle specific case
        damn default_value
    }
    when _ -> {
        // Handle all other errors
        vibez.spill_error("Unexpected error:", error)
        damn default_value
    }
}
```

### Resource Management
```cursed
// Use 'shook' for cleanup
slay process_file(filename tea) {
    sus file File = filez.open(filename) fam {
        when _ -> damn
    }
    shook file.close()  // Ensures file is closed
    
    // Process file...
}
```

### Testing
```cursed
// Write tests for every module interaction
slay test_file_processing() {
    testz.start_test("File Processing")
    
    // Create test file
    filez.write_file("test.txt", "test data")
    
    // Test processing
    sus result = process_file("test.txt")
    testz.assert_eq_string(result, expected_output)
    
    // Cleanup
    filez.remove_file("test.txt")
    
    testz.end_test()
}
```

## Next Steps

1. **Read the full documentation** for each module in `stdlib/*/README.md`
2. **Run the examples** in the `examples/` directory
3. **Write tests** using the `testz` framework
4. **Check the migration guide** if coming from other languages
5. **Join the community** for help and discussions

Each standard library module includes comprehensive documentation, examples, and test suites to help you understand and use them effectively.
