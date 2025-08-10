# CURSED Compiler Architecture

Comprehensive guide to the CURSED compiler implementation architecture in Zig.

## System Overview

The CURSED compiler is implemented in Zig and follows a traditional multi-pass compiler architecture with modern optimizations and memory safety features.

```
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Source Code   │───▶│    Lexer     │───▶│     Parser      │
│    (.csd)       │    │  (Tokenize)  │    │  (AST Build)    │
└─────────────────┘    └──────────────┘    └─────────────────┘
                                                     │
                                                     ▼
┌─────────────────┐    ┌──────────────┐    ┌─────────────────┐
│   Native Code   │◀───│ LLVM Codegen │◀───│ Type Checker    │
│   Executable    │    │   Backend    │    │ & Semantic      │
└─────────────────┘    └──────────────┘    └─────────────────┘
```

## Core Components

### 1. Lexical Analysis (`src-zig/lexer.zig`)

**Purpose**: Converts source code into tokens for parsing.

**Key Features**:
- Zero-copy tokenization for performance
- Unicode support for identifiers and string literals
- Gen Z slang keyword recognition
- Comment handling and preservation

**Token Types**:
```zig
const TokenType = enum {
    // Literals
    IDENTIFIER,
    INTEGER,
    FLOAT,
    STRING,
    CHAR,
    
    // Keywords
    SUS,        // Variable declaration
    SPILL,      // Struct field
    SLAY,       // Function definition
    DAMN,       // Return
    READY,      // If condition
    OTHERWISE,  // Else
    BESTIE,     // While loop
    SICK,       // Pattern match
    WHEN,       // Match case
    BASED,      // True
    CRINGE,     // False
    YEET,       // Import
    SQUAD,      // Struct
    COLLAB,     // Interface
    VIBE,       // Goroutine
    
    // Operators
    PLUS, MINUS, MULTIPLY, DIVIDE,
    ASSIGN, EQUALS, NOT_EQUALS,
    LESS_THAN, GREATER_THAN,
    
    // Delimiters
    LPAREN, RPAREN,
    LBRACE, RBRACE,
    LBRACKET, RBRACKET,
    SEMICOLON, COMMA,
    DOT, ARROW,
};
```

**Performance Characteristics**:
- Linear O(n) scanning
- Zero heap allocations during tokenization
- Memory-mapped file reading for large sources

### 2. Parser (`src-zig/parser.zig`)

**Purpose**: Builds Abstract Syntax Tree (AST) from tokens.

**Architecture**:
- Recursive descent parser
- Operator precedence handling
- Error recovery mechanisms
- Memory arena allocation for AST nodes

**AST Node Types**:
```zig
const AstNode = union(enum) {
    // Expressions
    literal: Literal,
    identifier: []const u8,
    binary_op: BinaryOp,
    unary_op: UnaryOp,
    function_call: FunctionCall,
    array_access: ArrayAccess,
    struct_access: StructAccess,
    
    // Statements
    variable_decl: VariableDecl,
    function_decl: FunctionDecl,
    if_stmt: IfStatement,
    while_loop: WhileLoop,
    return_stmt: ReturnStatement,
    
    // Declarations
    struct_decl: StructDecl,
    enum_decl: EnumDecl,
    import_decl: ImportDecl,
};
```

**Error Recovery**:
- Synchronization points at statement boundaries
- Panic mode recovery for malformed expressions
- Multiple error reporting in single pass

### 3. Type System (`src-zig/type_system_runtime.zig`)

**Purpose**: Type checking, inference, and semantic analysis.

**Type Representation**:
```zig
const Type = union(enum) {
    // Primitive types
    int: IntType,
    float: FloatType,
    bool: void,
    string: void,
    char: void,
    
    // Composite types
    array: *Type,
    slice: *Type,
    map: struct { key: *Type, value: *Type },
    function: FunctionType,
    
    // User-defined types
    struct_type: StructType,
    enum_type: EnumType,
    interface: InterfaceType,
    
    // Generic types
    generic: GenericType,
    type_param: []const u8,
};
```

**Type Inference Algorithm**:
1. **Bottom-up inference** for expressions
2. **Constraint solving** for generic types
3. **Unification** for type parameter resolution
4. **Subtyping** for interface compliance

**Features**:
- Hindley-Milner type inference
- Generic type instantiation
- Interface satisfaction checking
- Memory safety verification

### 4. LLVM Code Generation (`src-zig/advanced_codegen.zig`)

**Purpose**: Generates optimized LLVM IR and native machine code.

**Code Generation Pipeline**:
```
AST → LLVM IR → Optimization Passes → Machine Code
```

**Key Components**:
- **Function Lowering**: CURSED functions → LLVM functions
- **Memory Management**: Automatic insertion of allocation/deallocation
- **Control Flow**: CURSED control structures → LLVM basic blocks
- **Runtime Integration**: Calls to CURSED runtime system

**Optimization Passes**:
- Dead code elimination
- Constant folding and propagation
- Function inlining
- Loop optimizations
- Memory access optimization

**LLVM Integration**:
```zig
const CodeGenerator = struct {
    context: *llvm.Context,
    module: *llvm.Module,
    builder: *llvm.Builder,
    
    fn generateFunction(self: *Self, func: *FunctionDecl) !*llvm.Function {
        // Create LLVM function
        const llvm_func = llvm.addFunction(self.module, func.name, func_type);
        
        // Generate function body
        const entry = llvm.appendBasicBlock(llvm_func, "entry");
        llvm.positionBuilderAtEnd(self.builder, entry);
        
        // Generate statements
        for (func.body.statements) |stmt| {
            try self.generateStatement(stmt);
        }
        
        return llvm_func;
    }
};
```

### 5. Runtime System (`src-zig/concurrency.zig`)

**Purpose**: Provides runtime support for concurrency, memory management, and standard library.

**Components**:

#### Goroutine Scheduler
```zig
const Scheduler = struct {
    work_queue: ConcurrentQueue(Goroutine),
    thread_pool: []Thread,
    
    fn schedule(self: *Self, goroutine: Goroutine) void {
        self.work_queue.push(goroutine);
        self.wakeWorker();
    }
    
    fn workerThread(self: *Self) void {
        while (true) {
            const goroutine = self.work_queue.pop() orelse {
                self.parkWorker();
                continue;
            };
            goroutine.run();
        }
    }
};
```

#### Channel Implementation
```zig
fn Channel(comptime T: type) type {
    return struct {
        buffer: RingBuffer(T),
        senders: WaitQueue,
        receivers: WaitQueue,
        mutex: Mutex,
        
        fn send(self: *Self, value: T) void {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            if (self.buffer.isFull()) {
                self.senders.park();
                return;
            }
            
            self.buffer.push(value);
            self.receivers.wake();
        }
        
        fn receive(self: *Self) T {
            self.mutex.lock();
            defer self.mutex.unlock();
            
            if (self.buffer.isEmpty()) {
                self.receivers.park();
                return self.receive(); // Retry after wake
            }
            
            const value = self.buffer.pop();
            self.senders.wake();
            return value;
        }
    };
}
```

### 6. Memory Management (`src-zig/gc.zig`)

**Purpose**: Automatic memory management with minimal runtime overhead.

**Strategies**:
1. **Arena Allocation**: For compiler data structures
2. **Reference Counting**: For shared data
3. **Escape Analysis**: Stack vs heap allocation decisions
4. **Incremental GC**: Low-latency garbage collection

```zig
const MemoryManager = struct {
    arena: ArenaAllocator,
    gc_heap: GcHeap,
    
    fn allocate(self: *Self, size: usize, alignment: usize) []u8 {
        // Use escape analysis to choose allocation strategy
        if (self.escapesToHeap(size)) {
            return self.gc_heap.allocate(size, alignment);
        } else {
            return self.arena.allocate(size, alignment);
        }
    }
    
    fn collectGarbage(self: *Self) void {
        // Mark and sweep collection
        self.markRoots();
        self.sweep();
    }
};
```

## Build System Architecture

### Build Configuration (`build.zig`)

The build system supports multiple compilation targets and optimization levels:

```zig
const Build = @import("std").Build;

pub fn build(b: *Build) void {
    // Primary compiler executable
    const cursed_exe = b.addExecutable(.{
        .name = "cursed-zig",
        .root_source_file = .{ .path = "src-zig/main_unified.zig" },
        .target = target,
        .optimize = optimize,
    });
    
    // Add LLVM dependency
    cursed_exe.linkSystemLibrary("llvm-15");
    cursed_exe.addIncludePath(.{ .path = "/usr/include/llvm-15" });
    
    // Stable minimal compiler
    const stable_exe = b.addExecutable(.{
        .name = "cursed-stable",
        .root_source_file = .{ .path = "src-zig/minimal_main.zig" },
        .target = target,
        .optimize = .ReleaseFast,
    });
    
    // Install executables
    b.installArtifact(cursed_exe);
    b.installArtifact(stable_exe);
}
```

### Cross-Compilation Support

```zig
// Supported targets
const targets = [_]Target{
    .{ .cpu_arch = .x86_64, .os_tag = .linux },
    .{ .cpu_arch = .x86_64, .os_tag = .macos },
    .{ .cpu_arch = .x86_64, .os_tag = .windows },
    .{ .cpu_arch = .aarch64, .os_tag = .linux },
    .{ .cpu_arch = .aarch64, .os_tag = .macos },
    .{ .cpu_arch = .wasm32, .os_tag = .freestanding },
};
```

## Module System Architecture

### Standard Library Structure

```
stdlib/
├── vibez/          # I/O operations
├── mathz/          # Mathematical functions
├── stringz/        # String utilities
├── arrayz/         # Array operations
├── testz/          # Testing framework
├── cryptz/         # Cryptographic functions
├── concurrenz/     # Concurrency primitives
├── filez/          # File system operations
├── httpz/          # HTTP client/server
├── jsonz/          # JSON parsing
└── timez/          # Time and date utilities
```

### Module Loading System

```zig
const ModuleLoader = struct {
    cache: HashMap([]const u8, *Module),
    search_paths: [][]const u8,
    
    fn loadModule(self: *Self, name: []const u8) !*Module {
        // Check cache first
        if (self.cache.get(name)) |module| {
            return module;
        }
        
        // Search for module file
        const path = try self.findModulePath(name);
        
        // Parse and compile module
        const source = try fs.readFileAlloc(allocator, path);
        const module = try self.compileModule(name, source);
        
        // Cache result
        try self.cache.put(name, module);
        return module;
    }
};
```

## Performance Architecture

### Compilation Performance

**Design Goals**:
- Sub-second compilation for most programs
- Incremental compilation support
- Parallel compilation of modules

**Optimizations**:
- Memory arena allocation (no malloc/free overhead)
- Zero-copy string handling
- Parallel parsing of independent modules
- Cached type checking results

### Runtime Performance

**Features**:
- LLVM-based optimization pipeline
- Profile-guided optimization support
- Link-time optimization (LTO)
- Runtime profiling hooks

```zig
// Performance monitoring hooks
const PerformanceMonitor = struct {
    compile_times: HashMap([]const u8, u64),
    memory_usage: AtomicU64,
    
    fn recordCompileTime(self: *Self, module: []const u8, time_ns: u64) void {
        self.compile_times.put(module, time_ns) catch {};
    }
    
    fn getAverageCompileTime(self: *Self) u64 {
        var total: u64 = 0;
        var count: u64 = 0;
        
        var iter = self.compile_times.iterator();
        while (iter.next()) |entry| {
            total += entry.value_ptr.*;
            count += 1;
        }
        
        return if (count > 0) total / count else 0;
    }
};
```

## Error Handling Architecture

### Diagnostic System

```zig
const Diagnostic = struct {
    level: Level,
    message: []const u8,
    location: SourceLocation,
    suggestions: []Suggestion,
    
    const Level = enum { error, warning, info, hint };
    
    fn format(self: *const Self, writer: anytype) !void {
        try writer.print("{s}:{d}:{d}: {s}: {s}\n",
            .{ self.location.file, self.location.line, self.location.column,
               @tagName(self.level), self.message });
        
        // Print source line with highlighting
        try self.printSourceLine(writer);
        
        // Print suggestions
        for (self.suggestions) |suggestion| {
            try suggestion.format(writer);
        }
    }
};
```

### Error Recovery Strategies

1. **Parser Recovery**: Synchronize at statement boundaries
2. **Type Error Recovery**: Continue with assumed types
3. **Code Generation Recovery**: Generate runtime error checks
4. **Incremental Recovery**: Minimize recompilation on fixes

## Testing Architecture

### Unit Testing Framework

```zig
// Built-in testing support
test "lexer tokenization" {
    const source = "sus x drip = 42";
    var lexer = Lexer.init(source);
    
    try expectEqual(TokenType.SUS, lexer.nextToken().type);
    try expectEqual(TokenType.IDENTIFIER, lexer.nextToken().type);
    try expectEqual(TokenType.DRIP, lexer.nextToken().type);
    try expectEqual(TokenType.ASSIGN, lexer.nextToken().type);
    try expectEqual(TokenType.INTEGER, lexer.nextToken().type);
}
```

### Integration Testing

```bash
# Automated test suite
zig test src-zig/lexer.zig
zig test src-zig/parser.zig  
zig test src-zig/type_system_runtime.zig
zig test src-zig/advanced_codegen.zig

# End-to-end compilation tests
./test_suite/run_e2e_tests.sh
```

## Security Architecture

### Memory Safety

- **Bounds Checking**: Array access validation
- **Use-After-Free Prevention**: Automatic lifetime management
- **Buffer Overflow Protection**: Stack canaries and guard pages
- **Integer Overflow Detection**: Runtime checks in debug mode

### Sandboxing

```zig
const Sandbox = struct {
    allowed_syscalls: HashSet(u32),
    memory_limit: usize,
    time_limit: u64,
    
    fn createSandboxedExecution(self: *Self) !Process {
        const process = try Process.spawn(.{
            .seccomp_filter = self.createSeccompFilter(),
            .memory_limit = self.memory_limit,
            .time_limit = self.time_limit,
        });
        
        return process;
    }
};
```

This architecture provides a solid foundation for the CURSED compiler with emphasis on performance, safety, and maintainability. The modular design allows for easy extension and optimization of individual components.

## Next Steps

- [Build System Guide](build-system.md)
- [Contributing Guidelines](contributing.md)
- [Performance Tuning](../deployment/performance.md)
- [Debugging Guide](debugging.md)
