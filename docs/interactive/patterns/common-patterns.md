# Common Design Patterns in CURSED

This guide covers essential design patterns and best practices for writing idiomatic, maintainable CURSED code.

## 🎯 Table of Contents

1. [Error Handling Patterns](#error-handling-patterns)
2. [Concurrency Patterns](#concurrency-patterns)
3. [Data Processing Patterns](#data-processing-patterns)
4. [Resource Management Patterns](#resource-management-patterns)
5. [Testing Patterns](#testing-patterns)
6. [Performance Patterns](#performance-patterns)

## 🚨 Error Handling Patterns

### Pattern 1: Result Type Pattern

**Problem**: Handling operations that can fail gracefully.

**Solution**: Use CURSED's `yikes` error system for explicit error handling.

```cursed
yeet "vibez"
yeet "filez"

# Define custom error types
squad FileError {
    message tea
    code drip
}

# Function that can fail
slay read_config(filename tea) yikes<FileError> {
    ready (!filez.exists(filename)) {
        yikes FileError{
            message: "File not found",
            code: 404
        }
    }
    
    sus content tea = filez.read_string(filename) fam {
        when _ -> yikes FileError{
            message: "Failed to read file",
            code: 500
        }
    }
    
    damn content
}

# Usage with proper error handling
slay main() {
    sus config tea = read_config("config.json") fam {
        when FileError{code: 404} -> {
            vibez.spill("Using default configuration")
            damn "{\"default\": true}"
        }
        when FileError{code: 500} -> {
            vibez.spill("IO error, exiting")
            damn ""
        }
        when _ -> {
            vibez.spill("Unknown error")
            damn ""
        }
    }
    
    vibez.spill("Config:", config)
}
```

<interactive-editor>
yeet "vibez"

# Simplified error handling example
slay divide(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

slay safe_divide(a drip, b drip) drip {
    sus result drip = divide(a, b) fam {
        when "division by zero" -> {
            vibez.spill("Warning: Division by zero, returning 0")
            damn 0
        }
        when _ -> {
            vibez.spill("Unknown error")
            damn 0
        }
    }
    damn result
}

vibez.spill("Safe division:", safe_divide(10, 2))
vibez.spill("Safe division by zero:", safe_divide(10, 0))
</interactive-editor>

### Pattern 2: Chain of Responsibility for Error Recovery

```cursed
# Multiple fallback strategies
slay fetch_data_with_fallbacks(url tea) tea {
    # Try primary source
    sus result tea = fetch_from_primary(url) fam {
        when NetworkError -> {
            # Fallback to cache
            fetch_from_cache(url) fam {
                when CacheError -> {
                    # Final fallback
                    damn "default_data"
                }
            }
        }
        when _ -> damn "unknown_error"
    }
    damn result
}
```

## 🚀 Concurrency Patterns

### Pattern 1: Worker Pool Pattern

**Problem**: Process many tasks concurrently with limited resources.

**Solution**: Use goroutines with channels for work distribution.

```cursed
yeet "concurrenz"
yeet "vibez"

# Worker pool for processing tasks
slay worker_pool<T, R>(
    tasks []T,
    worker_count drip,
    processor slay(T) R
) []R {
    sus task_chan chan<T> = concurrenz.make_channel()
    sus result_chan chan<R> = concurrenz.make_channel()
    
    # Start workers
    bestie (worker_id drip: 0..worker_count) {
        go {
            bestie (based) {
                sus task T = <-task_chan
                ready (task == nil) {
                    shook  # Exit worker
                }
                sus result R = processor(task)
                result_chan <- result
            }
        }
    }
    
    # Send tasks
    go {
        bestie (task T: tasks) {
            task_chan <- task
        }
        # Signal completion
        bestie (i drip: 0..worker_count) {
            task_chan <- nil
        }
    }
    
    # Collect results
    sus results []R = []
    bestie (i drip: 0..len(tasks)) {
        sus result R = <-result_chan
        results = append(results, result)
    }
    
    damn results
}

# Example usage
slay process_number(n drip) drip {
    # Simulate work
    damn n * n
}

slay main() {
    sus numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus squares []drip = worker_pool(numbers, 3, process_number)
    vibez.spill("Squares:", squares)
}
```

<interactive-editor>
yeet "concurrenz"
yeet "vibez"
yeet "timez"

# Simplified worker example
slay process_task(task_id drip, result_chan chan<tea>) {
    timez.sleep(100)  # Simulate work
    result_chan <- "Task " + task_id + " completed"
}

slay main() {
    sus result_chan chan<tea> = concurrenz.make_channel()
    sus worker_count drip = 3
    
    # Start workers
    bestie (i drip: 0..worker_count) {
        go { process_task(i + 1, result_chan) }
    }
    
    # Collect results
    bestie (i drip: 0..worker_count) {
        sus result tea = <-result_chan
        vibez.spill(result)
    }
}

main()
</interactive-editor>

### Pattern 2: Fan-Out/Fan-In Pattern

```cursed
# Fan-out: distribute work to multiple workers
# Fan-in: collect results from multiple workers
slay fan_out_fan_in<T>(input []T, workers drip) []T {
    sus input_chan chan<T> = concurrenz.make_channel()
    sus output_chans []chan<T> = []
    
    # Create worker channels (fan-out)
    bestie (i drip: 0..workers) {
        sus worker_chan chan<T> = concurrenz.make_channel()
        output_chans = append(output_chans, worker_chan)
        
        go {
            bestie (item T: input_chan) {
                # Process item
                worker_chan <- process_item(item)
            }
        }
    }
    
    # Merge results (fan-in)
    sus result_chan chan<T> = merge_channels(output_chans)
    
    # Collect all results
    sus results []T = []
    bestie (item T: result_chan) {
        results = append(results, item)
    }
    
    damn results
}
```

### Pattern 3: Pipeline Pattern

```cursed
# Processing pipeline with stages
slay create_pipeline<T>() {
    sus stage1_chan chan<T> = concurrenz.make_channel()
    sus stage2_chan chan<T> = concurrenz.make_channel()
    sus output_chan chan<T> = concurrenz.make_channel()
    
    # Stage 1: Input processing
    go {
        bestie (item T: stage1_chan) {
            sus processed T = process_stage1(item)
            stage2_chan <- processed
        }
    }
    
    # Stage 2: Further processing
    go {
        bestie (item T: stage2_chan) {
            sus processed T = process_stage2(item)
            output_chan <- processed
        }
    }
    
    damn Pipeline<T>{
        input: stage1_chan,
        output: output_chan
    }
}
```

## 📊 Data Processing Patterns

### Pattern 1: Builder Pattern for Complex Objects

**Problem**: Creating complex objects with many optional parameters.

**Solution**: Use builder pattern with method chaining.

```cursed
yeet "vibez"

squad DatabaseConfig {
    host tea
    port drip
    username tea
    password tea
    database tea
    pool_size drip
    timeout drip
    ssl_enabled lit
}

squad DatabaseConfigBuilder {
    config DatabaseConfig
}

slay new_database_config() DatabaseConfigBuilder {
    damn DatabaseConfigBuilder{
        config: DatabaseConfig{
            host: "localhost",
            port: 5432,
            pool_size: 10,
            timeout: 30,
            ssl_enabled: cap
        }
    }
}

slay with_host(self DatabaseConfigBuilder, host tea) DatabaseConfigBuilder {
    self.config.host = host
    damn self
}

slay with_port(self DatabaseConfigBuilder, port drip) DatabaseConfigBuilder {
    self.config.port = port
    damn self
}

slay with_credentials(self DatabaseConfigBuilder, username tea, password tea) DatabaseConfigBuilder {
    self.config.username = username
    self.config.password = password
    damn self
}

slay with_ssl(self DatabaseConfigBuilder, enabled lit) DatabaseConfigBuilder {
    self.config.ssl_enabled = enabled
    damn self
}

slay build(self DatabaseConfigBuilder) DatabaseConfig {
    damn self.config
}

# Usage
slay main() {
    sus config DatabaseConfig = new_database_config()
        .with_host("production.db.com")
        .with_port(5432)
        .with_credentials("admin", "secret")
        .with_ssl(based)
        .build()
    
    vibez.spill("Database config:", config.host, config.port)
}
```

<interactive-editor>
yeet "vibez"

# Simplified builder example
squad HttpRequest {
    url tea
    method tea
    headers []tea
    timeout drip
}

squad RequestBuilder {
    request HttpRequest
}

slay new_request() RequestBuilder {
    damn RequestBuilder{
        request: HttpRequest{
            method: "GET",
            headers: [],
            timeout: 30
        }
    }
}

slay url(self RequestBuilder, url tea) RequestBuilder {
    self.request.url = url
    damn self
}

slay method(self RequestBuilder, method tea) RequestBuilder {
    self.request.method = method
    damn self
}

slay timeout(self RequestBuilder, timeout drip) RequestBuilder {
    self.request.timeout = timeout
    damn self
}

slay build(self RequestBuilder) HttpRequest {
    damn self.request
}

sus request HttpRequest = new_request()
    .url("https://api.example.com")
    .method("POST")
    .timeout(60)
    .build()

vibez.spill("Request:", request.method, request.url)
</interactive-editor>

### Pattern 2: Visitor Pattern for Data Processing

```cursed
# Process different data types uniformly
collab DataProcessor<T> {
    process(data T) T
}

squad JsonProcessor {}
squad XmlProcessor {}
squad CsvProcessor {}

slay process(self JsonProcessor, data tea) tea {
    # JSON-specific processing
    damn "processed_json: " + data
}

slay process(self XmlProcessor, data tea) tea {
    # XML-specific processing
    damn "processed_xml: " + data
}

slay process(self CsvProcessor, data tea) tea {
    # CSV-specific processing
    damn "processed_csv: " + data
}

# Generic processing function
slay process_data<T>(processor DataProcessor<T>, data T) T {
    damn processor.process(data)
}
```

### Pattern 3: Functional Pipeline Pattern

```cursed
yeet "arrayz"

# Chain operations together
slay pipeline<T>(data []T) []T {
    damn data
        .filter(slay(x T) lit { damn is_valid(x) })
        .map(slay(x T) T { damn transform(x) })
        .sort(slay(a T, b T) lit { damn compare(a, b) })
}

# Example with numbers
slay process_numbers(numbers []drip) []drip {
    damn numbers
        .filter(slay(n drip) lit { damn n > 0 })
        .map(slay(n drip) drip { damn n * 2 })
        .sort(slay(a drip, b drip) lit { damn a < b })
}
```

## 🛠️ Resource Management Patterns

### Pattern 1: RAII (Resource Acquisition Is Initialization)

**Problem**: Ensuring resources are properly cleaned up.

**Solution**: Use defer statements for automatic cleanup.

```cursed
yeet "filez"
yeet "vibez"

slay process_file(filename tea) yikes<tea> {
    sus file filez.File = filez.open(filename) fam {
        when _ -> yikes "failed to open file"
    }
    
    # Ensure file is closed when function exits
    defer { filez.close(file) }
    
    sus content tea = filez.read_all(file) fam {
        when _ -> yikes "failed to read file"
    }
    
    # Process content
    damn process_content(content)
}

slay with_database_transaction<T>(action slay() T) T {
    sus tx DatabaseTransaction = database.begin_transaction()
    defer {
        ready (had_error()) {
            database.rollback(tx)
        } otherwise {
            database.commit(tx)
        }
    }
    
    damn action()
}
```

<interactive-editor>
yeet "vibez"

# Simplified resource management
slay with_resource<T>(resource_name tea, action slay() T) T {
    vibez.spill("Acquiring resource:", resource_name)
    
    defer {
        vibez.spill("Releasing resource:", resource_name)
    }
    
    damn action()
}

slay do_work() tea {
    damn "Work completed"
}

sus result tea = with_resource("database_connection", do_work)
vibez.spill("Result:", result)
</interactive-editor>

### Pattern 2: Object Pool Pattern

```cursed
yeet "concurrenz"

squad ObjectPool<T> {
    objects chan<T>
    factory slay() T
    max_size drip
}

slay new_object_pool<T>(factory slay() T, max_size drip) ObjectPool<T> {
    sus pool ObjectPool<T> = ObjectPool<T>{
        objects: concurrenz.make_buffered_channel(max_size),
        factory: factory,
        max_size: max_size
    }
    
    # Pre-populate pool
    bestie (i drip: 0..max_size) {
        pool.objects <- factory()
    }
    
    damn pool
}

slay acquire<T>(pool ObjectPool<T>) T {
    damn <-pool.objects
}

slay release<T>(pool ObjectPool<T>, obj T) {
    # Reset object state if needed
    reset_object(obj)
    
    # Return to pool
    pool.objects <- obj
}

# Usage with automatic release
slay with_pooled_object<T, R>(pool ObjectPool<T>, action slay(T) R) R {
    sus obj T = acquire(pool)
    defer { release(pool, obj) }
    damn action(obj)
}
```

## 🧪 Testing Patterns

### Pattern 1: Table-Driven Tests

**Problem**: Testing multiple scenarios efficiently.

**Solution**: Use data-driven test cases.

```cursed
yeet "testz"
yeet "vibez"

squad TestCase<Input, Expected> {
    name tea
    input Input
    expected Expected
}

slay test_calculator() {
    sus test_cases []TestCase<[]drip, drip> = [
        TestCase{name: "addition", input: [2, 3], expected: 5},
        TestCase{name: "subtraction", input: [5, 3], expected: 2},
        TestCase{name: "multiplication", input: [4, 3], expected: 12},
        TestCase{name: "division", input: [10, 2], expected: 5}
    ]
    
    bestie (test_case TestCase<[]drip, drip>: test_cases) {
        sus result drip = calculate(test_case.input[0], test_case.input[1])
        testz.assert_eq(result, test_case.expected, test_case.name)
    }
}

slay calculate(a drip, b drip) drip {
    # Simplified calculator for demo
    damn a + b
}
```

<interactive-editor>
yeet "testz"
yeet "vibez"

# Simplified testing example
squad TestCase {
    name tea
    input drip
    expected drip
}

slay double(x drip) drip {
    damn x * 2
}

slay test_double() {
    sus cases []TestCase = [
        TestCase{name: "positive", input: 5, expected: 10},
        TestCase{name: "negative", input: -3, expected: -6},
        TestCase{name: "zero", input: 0, expected: 0}
    ]
    
    bestie (case TestCase: cases) {
        sus result drip = double(case.input)
        ready (result == case.expected) {
            vibez.spill("✅", case.name, "passed")
        } otherwise {
            vibez.spill("❌", case.name, "failed: expected", case.expected, "got", result)
        }
    }
}

test_double()
</interactive-editor>

### Pattern 2: Dependency Injection for Testing

```cursed
# Define interfaces for dependencies
collab DatabaseRepository {
    save(data tea) yikes<tea>
    find(id drip) tea
}

collab Logger {
    log(message tea)
}

# Service that depends on external services
squad UserService {
    db DatabaseRepository
    logger Logger
}

slay create_user(self UserService, name tea) yikes<tea> {
    self.logger.log("Creating user: " + name)
    damn self.db.save(name)
}

# Mock implementations for testing
squad MockDatabase {
    saved_data []tea
}

slay save(self MockDatabase, data tea) yikes<tea> {
    self.saved_data = append(self.saved_data, data)
    damn ""
}

slay find(self MockDatabase, id drip) tea {
    damn "mock_user"
}

squad MockLogger {
    messages []tea
}

slay log(self MockLogger, message tea) {
    self.messages = append(self.messages, message)
}

# Test with mocks
slay test_user_service() {
    sus mock_db MockDatabase = MockDatabase{saved_data: []}
    sus mock_logger MockLogger = MockLogger{messages: []}
    
    sus service UserService = UserService{
        db: mock_db,
        logger: mock_logger
    }
    
    service.create_user("Alice")
    
    testz.assert_eq(len(mock_db.saved_data), 1)
    testz.assert_eq(len(mock_logger.messages), 1)
}
```

## 🚀 Performance Patterns

### Pattern 1: Lazy Initialization

**Problem**: Expensive resources that might not be needed.

**Solution**: Initialize only when first accessed.

```cursed
yeet "concurrenz"

squad LazyValue<T> {
    value T
    initialized lit
    mutex concurrenz.Mutex
    initializer slay() T
}

slay new_lazy<T>(initializer slay() T) LazyValue<T> {
    damn LazyValue<T>{
        initialized: cap,
        mutex: concurrenz.new_mutex(),
        initializer: initializer
    }
}

slay get<T>(self LazyValue<T>) T {
    ready (self.initialized) {
        damn self.value
    }
    
    concurrenz.lock(self.mutex)
    defer { concurrenz.unlock(self.mutex) }
    
    # Double-check pattern
    ready (self.initialized) {
        damn self.value
    }
    
    self.value = self.initializer()
    self.initialized = based
    damn self.value
}

# Usage
sus expensive_resource LazyValue<tea> = new_lazy(slay() tea {
    # Expensive initialization
    damn "expensive_data"
})
```

<interactive-editor>
yeet "vibez"

# Simplified lazy loading
squad LazyConfig {
    config tea
    loaded lit
}

slay load_config() tea {
    vibez.spill("Loading configuration...")
    damn "config_data"
}

slay get_config(self LazyConfig) tea {
    ready (!self.loaded) {
        self.config = load_config()
        self.loaded = based
    }
    damn self.config
}

sus lazy_config LazyConfig = LazyConfig{loaded: cap}

vibez.spill("First access:", get_config(lazy_config))
vibez.spill("Second access:", get_config(lazy_config))
</interactive-editor>

### Pattern 2: Memory Pool for Allocations

```cursed
# Pre-allocate memory to avoid garbage collection pressure
squad MemoryPool<T> {
    pool []T
    size drip
    available chan<drip>
}

slay new_memory_pool<T>(size drip, factory slay() T) MemoryPool<T> {
    sus pool []T = []
    sus available chan<drip> = concurrenz.make_buffered_channel(size)
    
    bestie (i drip: 0..size) {
        pool = append(pool, factory())
        available <- i
    }
    
    damn MemoryPool<T>{
        pool: pool,
        size: size,
        available: available
    }
}

slay allocate<T>(pool MemoryPool<T>) T {
    sus index drip = <-pool.available
    damn pool.pool[index]
}

slay deallocate<T>(pool MemoryPool<T>, index drip) {
    # Reset object if needed
    reset_object(pool.pool[index])
    pool.available <- index
}
```

### Pattern 3: Batch Processing

```cursed
# Process items in batches for better performance
slay process_in_batches<T>(
    items []T,
    batch_size drip,
    processor slay([]T) yikes<tea>
) yikes<tea> {
    sus batch []T = []
    
    bestie (item T: items) {
        batch = append(batch, item)
        
        ready (len(batch) >= batch_size) {
            processor(batch) fam {
                when _ -> yikes "batch processing failed"
            }
            batch = []  # Reset batch
        }
    }
    
    # Process remaining items
    ready (len(batch) > 0) {
        processor(batch) fam {
            when _ -> yikes "final batch processing failed"
        }
    }
    
    damn ""
}
```

## 📋 Pattern Summary

| Pattern | Use Case | Key Benefit |
|---------|----------|-------------|
| Result Type | Error handling | Explicit error propagation |
| Worker Pool | Concurrent processing | Resource management |
| Builder | Complex object creation | Flexible configuration |
| RAII | Resource management | Automatic cleanup |
| Table-Driven Tests | Testing multiple cases | Maintainable tests |
| Lazy Initialization | Expensive resources | Performance optimization |
| Pipeline | Data transformation | Composable operations |
| Dependency Injection | Testable code | Loose coupling |

## 🔗 Next Steps

- [Advanced Concurrency Patterns](advanced-concurrency.md)
- [Performance Optimization Guide](../performance/optimization.md)
- [Testing Best Practices](../testing/best-practices.md)
- [Code Review Guidelines](../community/code-review.md)

---

**Remember**: These patterns are guidelines, not strict rules. Choose the right pattern for your specific use case and don't over-engineer simple solutions.
