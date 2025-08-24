# CURSED Standard Library Documentation

## Complete Reference for All Stdlib Modules

Welcome to the comprehensive documentation for the CURSED programming language standard library. This documentation covers all 50+ implemented modules with detailed API references, usage guides, and examples.

## 📚 Documentation Structure

Each module documentation includes:
- **Module Overview** - Purpose, key features, getting started
- **API Reference** - All functions with parameters, return values, examples
- **Usage Guide** - Common patterns, best practices, troubleshooting
- **Performance Notes** - Optimization tips, complexity analysis
- **Integration Examples** - How modules work together
- **Migration Guide** - From other languages, upgrade paths

## 🚀 Quick Start

```cursed
# Basic I/O - Print to console
yeet "vibez"
vibez.spill("Hello, CURSED!")

# Mathematical operations
yeet "mathz"
sus result drip = mathz.sqrt(16)  # result = 4.0

# Array processing
yeet "arrayz" 
sus numbers []drip = [1, 2, 3, 4, 5]
sus total drip = arrayz.sum_array(numbers)

# Testing your code
yeet "testz"
testz.test_start("my_test")
testz.assert_eq_int(2 + 2, 4)
testz.print_test_summary()
```

## 📋 Module Categories

### Core Modules (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [vibez](core/vibez.md) | I/O operations, printing, formatting | ✅ Production |
| [mathz](core/mathz.md) | Mathematical functions, constants, algorithms | ✅ Production |
| [stringz](core/stringz.md) | String manipulation, parsing, formatting | ✅ Production |
| [arrayz](core/arrayz.md) | Array operations, algorithms, utilities | ✅ Production |
| [testz](core/testz.md) | Testing framework with assertions and benchmarks | ✅ Production |

### System & Platform (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [filez](system/filez.md) | File system operations, path manipulation | ✅ Production |
| [networkz](system/networkz.md) | Network programming, HTTP client/server | ✅ Production |
| [timez](system/timez.md) | Date/time handling, timers, scheduling | ✅ Production |
| [platformz](system/platformz.md) | Platform-specific operations | ✅ Production |
| [procesz](system/procesz.md) | Process management, signals, pipes | ✅ Production |

### Data & Serialization (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [jsonz](data/jsonz.md) | JSON parsing and generation | ✅ Production |
| [xmlz](data/xmlz.md) | XML processing | ✅ Production |
| [csvz](data/csvz.md) | CSV reading and writing | ✅ Production |
| [yamlz](data/yamlz.md) | YAML support | ✅ Production |
| [tomlz](data/tomlz.md) | TOML configuration files | ✅ Production |

### Cryptography & Security (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [cryptz](security/cryptz.md) | Cryptographic primitives, hashing | ✅ Production |
| [tlsz](security/tlsz.md) | TLS/SSL support | ✅ Production |
| [authz](security/authz.md) | Authentication systems | ✅ Production |
| [hashz](security/hashz.md) | Hash functions and utilities | ✅ Production |

### Concurrency & Async (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [concurrenz](concurrency/concurrenz.md) | Goroutines, channels, synchronization | ✅ Production |
| [asyncz](concurrency/asyncz.md) | Async/await primitives | ⚠️ Beta |
| [streamz](concurrency/streamz.md) | Reactive streams and event handling | ⚠️ Beta |
| [schedulz](concurrency/schedulz.md) | Task scheduling and execution | ⚠️ Beta |

### Database & Storage (Beta Status ⚠️)
| Module | Purpose | Status |
|--------|---------|--------|
| [dbz](database/dbz.md) | Database abstraction layer | ⚠️ Beta |
| [sqlz](database/sqlz.md) | SQL query building and execution | ⚠️ Beta |
| [redisz](database/redisz.md) | Redis client | ⚠️ Beta |
| [mongoz](database/mongoz.md) | MongoDB support | ⚠️ Beta |

### Graphics & UI (Beta Status ⚠️)
| Module | Purpose | Status |
|--------|---------|--------|
| [windowz](ui/windowz.md) | Window management | ⚠️ Beta |
| [drawz](ui/drawz.md) | 2D graphics primitives | ⚠️ Beta |
| [uiz](ui/uiz.md) | UI framework components | ⚠️ Beta |
| [gamez](ui/gamez.md) | Game development utilities | ⚠️ Beta |

### Advanced Utilities (Mixed Status)
| Module | Purpose | Status |
|--------|---------|--------|
| [reflectz](advanced/reflectz.md) | Runtime reflection and introspection | ✅ Production |
| [memoryz](advanced/memoryz.md) | Memory management utilities | ✅ Production |
| [debugz](advanced/debugz.md) | Debugging and profiling tools | ✅ Production |
| [packz](advanced/packz.md) | Package management utilities | ⚠️ Beta |
| [buildz](advanced/buildz.md) | Build system integration | ⚠️ Beta |
| [deployz](advanced/deployz.md) | Deployment and packaging tools | ⚠️ Beta |
| [metricz](advanced/metricz.md) | Performance monitoring and profiling | ⚠️ Beta |

### Specialized Domains (Production Ready ✅)
| Module | Purpose | Status |
|--------|---------|--------|
| [compressionz](specialized/compressionz.md) | Compression algorithms (GZIP, DEFLATE, LZ4) | ✅ Production |
| [audioz](specialized/audioz.md) | Audio processing and format handling | ✅ Production |
| [imagez](specialized/imagez.md) | Image processing and manipulation | ✅ Production |
| [mlz](specialized/mlz.md) | Machine learning primitives | ⚠️ Beta |
| [scientificz](specialized/scientificz.md) | Scientific computing | ⚠️ Beta |
| [blockchainz](specialized/blockchainz.md) | Blockchain and cryptocurrency | ⚠️ Beta |

## 🎯 Getting Started by Use Case

### **Web Development**
```cursed
yeet "networkz"    # HTTP server/client
yeet "jsonz"       # JSON handling
yeet "htmlz"       # HTML generation
yeet "tlsz"        # HTTPS support
yeet "authz"       # Authentication
```

### **System Programming**
```cursed
yeet "filez"       # File operations
yeet "procesz"     # Process management
yeet "memoryz"     # Memory management
yeet "concurrenz"  # Concurrency
yeet "platformz"   # OS interaction
```

### **Data Processing**
```cursed
yeet "csvz"        # CSV processing
yeet "jsonz"       # JSON parsing
yeet "xmlz"        # XML processing
yeet "compressionz" # Compression
yeet "dbz"         # Database access
```

### **Game Development**
```cursed
yeet "gamez"       # Game utilities
yeet "drawz"       # 2D graphics
yeet "audioz"      # Audio processing
yeet "windowz"     # Window management
yeet "timez"       # Game timing
```

### **CLI Tools**
```cursed
yeet "vibez"       # Console I/O
yeet "filez"       # File operations
yeet "stringz"     # Text processing
yeet "procesz"     # Process control
yeet "testz"       # Testing
```

### **Data Science**
```cursed
yeet "mathz"       # Mathematical functions
yeet "arrayz"      # Array operations
yeet "scientificz" # Scientific computing
yeet "mlz"         # Machine learning
yeet "csvz"        # Data import/export
```

## 🔧 Common Integration Patterns

### **HTTP API Server**
```cursed
yeet "networkz"
yeet "jsonz"
yeet "authz"

slay create_api_server() {
    sus server Server = networkz.create_server("0.0.0.0", 8080)
    
    networkz.handle_route(server, "GET", "/api/users", slay(request HttpRequest) HttpResponse {
        # Authenticate request
        sus user User = authz.authenticate(request.headers)
        
        # Process and return JSON
        sus users []User = get_users()
        sus json_data tea = jsonz.marshal(users)
        
        damn networkz.json_response(json_data)
    })
    
    networkz.start_server(server)
}
```

### **Data Processing Pipeline**
```cursed
yeet "filez"
yeet "csvz"
yeet "arrayz"
yeet "mathz"

slay process_data_file(filename tea) {
    # Read CSV data
    sus data [][]tea = csvz.read_file(filename)
    
    # Process numeric columns
    sus numbers []drip = arrayz.map(data, slay(row []tea) drip {
        damn stringz.to_float(row[1])
    })
    
    # Calculate statistics
    sus average drip = mathz.mean(numbers)
    sus std_dev drip = mathz.standard_deviation(numbers)
    
    vibez.spill("Average:", average, "Std Dev:", std_dev)
}
```

### **Concurrent Processing**
```cursed
yeet "concurrenz"
yeet "testz"

slay concurrent_test() {
    sus results chan<drip> = concurrenz.make_channel()
    sus workers drip = 4
    
    # Start workers
    bestie (i drip = 0; i < workers; i += 1) {
        go {
            sus result drip = heavy_computation(i)
            results <- result
        }
    }
    
    # Collect results
    sus total drip = 0
    bestie (i drip = 0; i < workers; i += 1) {
        total += <-results
    }
    
    testz.assert_true(total > 0)
}
```

## 📊 Performance Characteristics

### **Module Performance Tiers**

**Tier 1 - Optimized (Sub-microsecond)**
- `mathz` - SIMD optimized mathematical operations
- `stringz` - Efficient string operations with SSO
- `arrayz` - Cache-friendly array algorithms

**Tier 2 - Fast (1-10 microseconds)**
- `vibez` - Buffered I/O operations
- `concurrenz` - Lock-free channel operations
- `memoryz` - Arena-based allocation

**Tier 3 - Standard (10-100 microseconds)**
- `jsonz` - Streaming JSON parser
- `networkz` - HTTP request handling
- `filez` - Cached file operations

**Tier 4 - Heavy (100+ microseconds)**
- `cryptz` - Cryptographic operations
- `compressionz` - Compression algorithms
- `dbz` - Database queries

## 🚀 Migration Guides

### **From Go**
```cursed
# Go: fmt.Println("Hello")
vibez.spill("Hello")

# Go: math.Sqrt(16)
mathz.sqrt(16)

# Go: json.Marshal(data)
jsonz.marshal(data)
```

### **From Rust**
```cursed
# Rust: println!("Hello")
vibez.spill("Hello")

# Rust: vec![1, 2, 3]
sus array []drip = [1, 2, 3]

# Rust: std::thread::spawn
go { /* concurrent code */ }
```

### **From Python**
```cursed
# Python: print("Hello")
vibez.spill("Hello")

# Python: math.sqrt(16)
mathz.sqrt(16)

# Python: json.dumps(data)
jsonz.marshal(data)
```

## 🔍 Advanced Topics

### **Memory Management**
All stdlib modules use CURSED's advanced memory management:
- **Arena Allocators**: Fast bulk allocation/deallocation
- **Zero-Copy Operations**: Minimize memory overhead
- **Automatic Cleanup**: RAII-style resource management

### **Error Handling**
Consistent error handling across all modules:
```cursed
sus result tea = filez.read_file("config.json") fam {
    when "file_not_found" -> {
        vibez.spill("Config file missing, using defaults")
        damn "{}"
    }
    when "permission_denied" -> {
        yikes "insufficient_permissions"
    }
}
```

### **Testing Integration**
All modules integrate with the testz framework:
```cursed
testz.test_start("module_test")
testz.assert_eq_string(stringz.trim("  hello  "), "hello")
testz.benchmark_start("string_ops")
# Performance testing code
testz.benchmark_end()
testz.print_test_summary()
```

## 📈 Version History

- **v1.0.0** - Production release with 20 core modules
- **v0.9.0** - Beta release with advanced features
- **v0.8.0** - Alpha release with basic functionality

## 🤝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on:
- Adding new stdlib modules
- Improving existing modules  
- Writing tests and documentation
- Performance optimization
- Security considerations

---

**© 2025 CURSED Language Project | Production Ready 🚀**
