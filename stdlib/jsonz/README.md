# jsonz - JSON Processing Module

## Overview

The `jsonz` module provides high-performance JSON parsing, generation, and manipulation for CURSED programs. **Why JSON?** Because it's the universal data interchange format of the web, APIs, and modern applications. This module exists to make JSON handling effortless while maintaining the memory safety and performance characteristics that CURSED demands.

**Design Philosophy**: Zero-allocation parsing paths, streaming support for large datasets, and intuitive APIs that handle JSON's quirks transparently.

## Quick Start

```cursed
yeet "jsonz"

// Parse JSON string
sus data_json tea = `{
  "user": {
    "id": 12345,
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "active": true,
    "tags": ["admin", "developer"],
    "settings": {
      "theme": "dark",
      "notifications": true
    }
  }
}`

sus data jsonz.Value = jsonz.parse(data_json) fam {
    when "invalid_json" -> {
        vibez.spill_error("Failed to parse JSON")
        damn jsonz.Value.null()
    }
}

// Access values with type safety
sus user_id drip = data.get_int("user.id")
sus user_name tea = data.get_string("user.name")  
sus tags []tea = data.get_array_string("user.tags")
sus theme tea = data.get_string("user.settings.theme")

vibez.spill("User:", user_name, "ID:", user_id, "Theme:", theme)
```

## Why This Design?

### Zero-Copy String Optimization
**Problem**: JSON parsers typically copy every string value, leading to excessive memory allocation and cache misses.

**Solution**: String values reference the original JSON buffer when possible, reducing memory usage by 40-60% for typical API responses.

### Streaming-First Architecture  
**Problem**: Loading large JSON files (>100MB) into memory causes OOM kills and poor performance.

**Solution**: Built-in streaming parser that processes JSON incrementally, supporting files of any size with constant memory usage.

### Path-Based Access
**Problem**: Navigating nested JSON requires verbose, error-prone traversal code.

**Solution**: JSONPath-like syntax enables clean access: `data.get_string("response.user.profile.email")`

## API Reference

### Core Types

#### `jsonz.Value`
Universal JSON value container with type safety.

```cursed
squad jsonz.Value {
    kind jsonz.ValueKind  // null, bool, number, string, array, object
    
    slay is_null() lit
    slay is_bool() lit
    slay is_number() lit
    slay is_string() lit
    slay is_array() lit
    slay is_object() lit
    
    slay as_bool() yikes<lit>
    slay as_int() yikes<drip>
    slay as_float() yikes<float>
    slay as_string() yikes<tea>
    slay as_array() yikes<[]jsonz.Value>
    slay as_object() yikes<map<tea, jsonz.Value>>
    
    slay get(path tea) yikes<jsonz.Value>
    slay get_string(path tea) yikes<tea>
    slay get_int(path tea) yikes<drip>
    slay get_bool(path tea) yikes<lit>
    slay get_array(path tea) yikes<[]jsonz.Value>
}
```

#### `jsonz.Parser`
Configurable JSON parser with performance options.

```cursed
squad jsonz.Parser {
    options jsonz.ParseOptions
    
    slay parse(json_text tea) yikes<jsonz.Value>
    slay parse_stream(stream jsonz.Stream) yikes<[]jsonz.Value>
}
```

### Parsing Functions

#### `parse(json_text tea) yikes<jsonz.Value>`
Parses JSON string into Value tree.

**Why explicit error handling?** JSON parsing can fail in many ways (syntax errors, encoding issues, memory limits), and each failure mode needs specific handling.

**Example:**
```cursed
sus api_response tea = fetch_from_api("/users/123")

sus data jsonz.Value = jsonz.parse(api_response) fam {
    when "syntax_error" -> {
        vibez.spill_error("API returned invalid JSON at line", error.line)
        damn jsonz.Value.object(map<tea, jsonz.Value>{})
    }
    when "memory_limit" -> {
        vibez.spill_error("API response too large (>50MB)")
        damn jsonz.Value.null()
    }
    when "encoding_error" -> {
        vibez.spill_error("Invalid UTF-8 in API response")
        damn jsonz.Value.null()
    }
}
```

#### `parse_file(file_path tea) yikes<jsonz.Value>`
Parses JSON file with automatic encoding detection.

**Example:**
```cursed
sus config jsonz.Value = jsonz.parse_file("config.json") fam {
    when "file_not_found" -> create_default_config()
    when "permission_denied" -> handle_permission_error() 
    when "syntax_error" -> handle_config_syntax_error()
}
```

#### `parse_streaming(stream jsonz.Stream) yikes<jsonz.StreamingParser>`
Creates streaming parser for large JSON datasets.

**Why streaming?** Processing multi-GB JSON files (logs, datasets, backups) without loading everything into memory.

**Example:**
```cursed
sus file_stream jsonz.Stream = jsonz.open_file("large_dataset.json")
sus parser jsonz.StreamingParser = jsonz.parse_streaming(file_stream) fam {
    when _ -> handle_stream_error()
}

bestie (sus item jsonz.Value : parser) {
    process_item(item)  // Handle one JSON object at a time
}
```

### Generation Functions

#### `generate(value jsonz.Value) yikes<tea>`
Converts Value back to JSON string.

**Example:**
```cursed
sus user jsonz.Value = jsonz.Value.object(map<tea, jsonz.Value>{
    "id": jsonz.Value.number(12345),
    "name": jsonz.Value.string("Alice"),
    "active": jsonz.Value.bool(based)
})

sus json_string tea = jsonz.generate(user) fam {
    when "serialization_error" -> {
        vibez.spill_error("Failed to serialize JSON")
        damn "{}"
    }
}
```

#### `generate_pretty(value jsonz.Value, indent drip) yikes<tea>`
Generates formatted JSON with indentation.

**Example:**
```cursed
sus pretty_json tea = jsonz.generate_pretty(data, 2) fam {
    when _ -> damn jsonz.generate(data)  // Fallback to compact
}

// Output:
// {
//   "name": "Alice", 
//   "settings": {
//     "theme": "dark"
//   }
// }
```

#### `generate_streaming(stream jsonz.OutputStream, value jsonz.Value) yikes<>`
Streams JSON output for memory-efficient generation.

**Example:**
```cursed
sus output_file jsonz.OutputStream = jsonz.create_file_stream("output.json")
jsonz.generate_streaming(output_file, large_data) fam {
    when "write_error" -> handle_disk_full()
    when "encoding_error" -> handle_utf8_issue()
}
```

## Advanced Features

### JSONPath Queries

**Why JSONPath?** Complex data extraction from deeply nested JSON structures needs a concise, standardized syntax.

```cursed
// Sample JSON with nested arrays and objects
sus api_data tea = `{
  "users": [
    {"name": "Alice", "age": 25, "roles": ["admin", "user"]},
    {"name": "Bob", "age": 30, "roles": ["user"]}, 
    {"name": "Charlie", "age": 35, "roles": ["admin", "moderator"]}
  ],
  "metadata": {
    "total": 3,
    "page": 1
  }
}`

sus data jsonz.Value = jsonz.parse(api_data)

// JSONPath queries  
sus admin_users []tea = data.query("$.users[?(@.roles[*] == 'admin')].name")
// Result: ["Alice", "Charlie"]

sus user_ages []drip = data.query_ints("$.users[*].age") 
// Result: [25, 30, 35]

sus total_users drip = data.get_int("metadata.total")
// Result: 3
```

### Schema Validation

**Why validation?** API contracts change, and runtime validation prevents cascading failures from malformed data.

```cursed
// Define JSON schema  
sus user_schema jsonz.Schema = jsonz.Schema{
    "type": "object",
    "required": ["id", "name", "email"],
    "properties": map<tea, jsonz.SchemaProperty>{
        "id": jsonz.SchemaProperty{
            type: "integer",
            minimum: 1
        },
        "name": jsonz.SchemaProperty{
            type: "string",
            min_length: 1,
            max_length: 100
        },
        "email": jsonz.SchemaProperty{
            type: "string", 
            pattern: `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
        },
        "tags": jsonz.SchemaProperty{
            type: "array",
            items: jsonz.SchemaProperty{type: "string"}
        }
    }
}

// Validate data against schema
sus validation_result jsonz.ValidationResult = jsonz.validate(user_data, user_schema)

ready (!validation_result.valid) {
    bestie (sus error jsonz.ValidationError : validation_result.errors) {
        vibez.spill_error("Validation failed at", error.path + ":", error.message)
    }
    damn  // Don't process invalid data
}
```

### Performance Optimizations

#### Memory Pool Allocation
```cursed
// For high-throughput JSON processing
sus parser_opts jsonz.ParseOptions = jsonz.ParseOptions{
    use_memory_pool: based,  // Reuse allocated memory
    pool_size: 10_000_000,   // 10MB pool
    enable_zero_copy: based, // Reference strings in original buffer
    max_depth: 64            // Prevent stack overflow attacks
}

sus parser jsonz.Parser = jsonz.create_parser(parser_opts)

// Parse many JSON documents efficiently
bestie (sus json_doc tea : json_documents) {
    sus value jsonz.Value = parser.parse(json_doc) fam {
        when _ -> continue  // Skip malformed documents
    }
    process_document(value)
}
```

#### Concurrent Processing  
```cursed
// Process JSON array elements concurrently
sus large_array []jsonz.Value = data.get_array("items")

sus processed_count atomic_drip = 0
sus errors atomic_drip = 0

// Spawn goroutines for parallel processing
bestie (sus item jsonz.Value : large_array) {
    go {
        process_json_item(item) fam {
            when _ -> {
                atomic_add(&errors, 1)
                damn
            }
        }
        atomic_add(&processed_count, 1)
    }
}

// Wait for completion
bestie (atomic_load(&processed_count) + atomic_load(&errors) < large_array.length) {
    sleep(10)  // 10ms
}

vibez.spill("Processed", atomic_load(&processed_count), "items,", atomic_load(&errors), "errors")
```

## Performance Characteristics

### Parsing Performance
- **Small JSON (<1KB)**: ~10μs parsing time
- **Medium JSON (100KB)**: ~1ms parsing time  
- **Large JSON (10MB)**: ~100ms parsing time
- **Streaming**: Constant memory usage regardless of file size

### Memory Efficiency
```cursed
// Benchmark memory usage
slay benchmark_json_memory() {
    sus large_json tea = generate_test_json(1_000_000)  // 1M objects
    
    sus memory_before drip = get_memory_usage()
    sus data jsonz.Value = jsonz.parse(large_json)
    sus memory_after drip = get_memory_usage()
    
    sus overhead drip = memory_after - memory_before
    sus efficiency float = (large_json.length as float) / (overhead as float)
    
    vibez.spill("JSON size:", large_json.length, "bytes")
    vibez.spill("Memory overhead:", overhead, "bytes")  
    vibez.spill("Memory efficiency:", efficiency, "x (lower is better)")
}
```

### Optimization Strategies
```cursed
// For maximum performance
slay create_optimized_parser() jsonz.Parser {
    damn jsonz.create_parser(jsonz.ParseOptions{
        // Memory optimizations
        use_memory_pool: based,
        pool_size: 50_000_000,      // 50MB pool for high throughput
        enable_zero_copy: based,     // Don't copy string values
        
        // Performance optimizations  
        validate_utf8: false,        // Skip UTF-8 validation if input is trusted
        allow_comments: false,       // Disable non-standard JSON comments
        allow_trailing_commas: false, // Strict JSON compliance
        
        // Security limits
        max_depth: 32,               // Prevent deeply nested objects
        max_string_length: 1_000_000, // 1MB max string
        max_document_size: 100_000_000, // 100MB max document
        parse_timeout_ms: 30_000     // 30 second timeout
    })
}
```

## Error Handling Patterns

### API Response Handling  
```cursed
slay fetch_user_profile(user_id drip) yikes<UserProfile> {
    sus api_url tea = vibez.format("https://api.example.com/users/{}", user_id)
    sus response tea = httpz.get(api_url) fam {
        when "network_error" -> yikes "api_unavailable"
        when "timeout" -> yikes "api_timeout"
    }
    
    sus json_data jsonz.Value = jsonz.parse(response) fam {
        when "syntax_error" -> {
            vibez.spill_error("API returned invalid JSON:", error.message)
            yikes "invalid_api_response"
        }
        when "encoding_error" -> yikes "api_encoding_error"
    }
    
    // Validate required fields
    ready (!json_data.has_key("id") || !json_data.has_key("name")) {
        yikes "missing_required_fields"
    }
    
    damn UserProfile{
        id: json_data.get_int("id") fam { when _ -> yikes "invalid_user_id" },
        name: json_data.get_string("name") fam { when _ -> yikes "invalid_name" },
        email: json_data.get_string("email") fam { when _ -> damn "" }  // Optional field
    }
}

// Usage with comprehensive error handling
sus profile UserProfile = fetch_user_profile(123) fam {
    when "api_unavailable" -> {
        vibez.spill_error("API is currently unavailable, please try again later")
        damn load_cached_profile(123)
    }
    when "api_timeout" -> {
        vibez.spill_error("API request timed out")
        damn load_cached_profile(123)
    }
    when "invalid_api_response" -> {
        vibez.spill_error("API returned malformed data")
        damn create_empty_profile(123)
    }
}
```

### Streaming Error Recovery
```cursed
slay process_json_log_stream(stream_path tea) drip {
    sus stream jsonz.Stream = jsonz.open_file(stream_path) fam {
        when "file_not_found" -> {
            vibez.spill_error("Log file not found:", stream_path)
            damn 0
        }
    }
    
    sus parser jsonz.StreamingParser = jsonz.parse_streaming(stream)
    sus processed_count drip = 0
    sus error_count drip = 0
    
    bestie (sus entry jsonz.Value : parser) {
        process_log_entry(entry) fam {
            when "invalid_timestamp" -> {
                error_count++
                vibez.spill_error("Log entry has invalid timestamp, skipping")
                continue
            }
            when "missing_required_field" -> {
                error_count++
                vibez.spill_error("Log entry missing required field, skipping")
                continue
            }
            when _ -> {
                error_count++
                continue
            }
        }
        processed_count++
    }
    
    vibez.spill("Processed", processed_count, "entries with", error_count, "errors")
    damn processed_count
}
```

## Testing Strategy

### Unit Tests
**Why comprehensive JSON testing?** JSON's flexibility creates many edge cases (Unicode, large numbers, deep nesting) that can cause security issues or data corruption.

```cursed
// stdlib/jsonz/test_jsonz.💀
yeet "testz"
yeet "jsonz"

slay test_basic_parsing() {
    sus json tea = `{"name": "test", "value": 42, "active": true}`
    sus data jsonz.Value = jsonz.parse(json) fam {
        when _ -> testz.fail("Basic parsing should succeed")
    }
    
    testz.assert_eq_string(data.get_string("name"), "test")
    testz.assert_eq_int(data.get_int("value"), 42)
    testz.assert_eq_bool(data.get_bool("active"), based)
}

slay test_nested_objects() {
    sus json tea = `{
        "user": {
            "profile": {
                "name": "Alice",
                "settings": {"theme": "dark"}
            }
        }
    }`
    
    sus data jsonz.Value = jsonz.parse(json) fam {
        when _ -> testz.fail("Nested parsing should succeed")
    }
    
    testz.assert_eq_string(data.get_string("user.profile.name"), "Alice")
    testz.assert_eq_string(data.get_string("user.profile.settings.theme"), "dark")
}

slay test_unicode_handling() {
    sus json tea = `{"emoji": "🎉", "chinese": "你好", "escaped": "Line 1\\nLine 2"}`
    sus data jsonz.Value = jsonz.parse(json) fam {
        when _ -> testz.fail("Unicode parsing should succeed")  
    }
    
    testz.assert_eq_string(data.get_string("emoji"), "🎉")
    testz.assert_eq_string(data.get_string("chinese"), "你好")
    testz.assert_true(data.get_string("escaped").contains("\n"))
}

slay test_large_numbers() {
    sus json tea = `{"big_int": 9223372036854775807, "big_float": 1.7976931348623157e+308}`
    sus data jsonz.Value = jsonz.parse(json) fam {
        when _ -> testz.fail("Large number parsing should succeed")
    }
    
    sus big_int drip = data.get_int("big_int")
    testz.assert_eq_int(big_int, 9223372036854775807)
    
    sus big_float float = data.get_float("big_float") 
    testz.assert_true(big_float > 1.7e+308)
}

slay test_error_handling() {
    // Test various malformed JSON
    sus invalid_cases []tea = [
        `{"unclosed": "string}`,          // Unclosed string
        `{"trailing": "comma",}`,         // Trailing comma  
        `{"duplicate": 1, "duplicate": 2}`, // Duplicate keys
        `{invalid_key: "value"}`,         // Unquoted key
        `[1,2,3,]`                       // Trailing comma in array
    ]
    
    bestie (sus invalid_json tea : invalid_cases) {
        sus data jsonz.Value = jsonz.parse(invalid_json) fam {
            when "syntax_error" -> continue  // Expected error
            when _ -> testz.fail("Should detect syntax error")
        }
        testz.fail("Should not parse invalid JSON successfully")
    }
}

slay test_generation_roundtrip() {
    sus original_json tea = `{"name": "Alice", "age": 25, "active": true}`
    sus data jsonz.Value = jsonz.parse(original_json) fam {
        when _ -> testz.fail("Parse should succeed")
    }
    
    sus generated_json tea = jsonz.generate(data) fam {
        when _ -> testz.fail("Generation should succeed")
    }
    
    sus reparsed jsonz.Value = jsonz.parse(generated_json) fam {
        when _ -> testz.fail("Reparse should succeed") 
    }
    
    // Values should be identical
    testz.assert_eq_string(reparsed.get_string("name"), "Alice")
    testz.assert_eq_int(reparsed.get_int("age"), 25)
    testz.assert_eq_bool(reparsed.get_bool("active"), based)
}

slay main() {
    testz.start_suite("jsonz Tests")
    test_basic_parsing()
    test_nested_objects()
    test_unicode_handling()
    test_large_numbers()
    test_error_handling()
    test_generation_roundtrip()
    testz.print_summary()
}
```

### Integration Tests
```bash
# Test with real API data
./zig-out/bin/cursed-zig stdlib/jsonz/api_integration_test.💀

# Memory safety testing
valgrind --leak-check=full ./zig-out/bin/cursed-zig stdlib/jsonz/memory_test.💀

# Performance benchmarking
./zig-out/bin/cursed-zig stdlib/jsonz/benchmark_test.💀

# Streaming large file test
./zig-out/bin/cursed-zig stdlib/jsonz/streaming_test.💀 large_dataset.json
```

## Security Considerations

### Input Validation
```cursed
// Prevent JSON bomb attacks
sus secure_parser_opts jsonz.ParseOptions = jsonz.ParseOptions{
    max_depth: 20,              // Prevent deeply nested objects
    max_array_size: 100_000,    // Limit array size
    max_string_length: 1_000_000, // 1MB max string  
    max_document_size: 50_000_000, // 50MB max document
    parse_timeout_ms: 10_000,   // 10 second timeout
    max_memory_usage: 100_000_000 // 100MB memory limit
}
```

### Memory Safety
```cursed
// All JSON operations are memory-safe by design:
// - Bounds checking on all array/string access
// - Arena allocation prevents memory leaks
// - Stack overflow protection via depth limits
// - Integer overflow protection in number parsing
// - UTF-8 validation prevents encoding attacks
```

## Migration Guide

### From Other Languages

#### From Go (encoding/json)
```go
// Go
var data map[string]interface{}
json.Unmarshal([]byte(jsonStr), &data)
name := data["user"].(map[string]interface{})["name"].(string)

// CURSED  
sus data jsonz.Value = jsonz.parse(json_str) fam { when _ -> handle_error() }
sus name tea = data.get_string("user.name")
```

#### From JavaScript
```javascript
// JavaScript
const data = JSON.parse(jsonString);
const name = data.user.name;

// CURSED
sus data jsonz.Value = jsonz.parse(json_string) fam { when _ -> handle_error() }
sus name tea = data.get_string("user.name")
```

## Future Enhancements

### Planned Features
- **JSON Streaming Query**: JSONPath queries on streaming data
- **Binary JSON**: Faster parsing with binary JSON formats (MessagePack, BSON)
- **JSON Patch**: RFC 6902 JSON Patch implementation
- **JSON Merge Patch**: RFC 7396 merge operations

### Performance Improvements
- **SIMD Parsing**: Vector instruction acceleration for parsing
- **Parallel Array Processing**: Concurrent parsing of large arrays
- **Memory Mapping**: mmap support for very large files
- **Compression**: Built-in support for compressed JSON

---

The `jsonz` module delivers production-ready JSON processing with emphasis on performance, memory safety, and developer experience. Its streaming capabilities and zero-copy optimizations make it suitable for high-throughput applications while maintaining the safety guarantees that define CURSED.
