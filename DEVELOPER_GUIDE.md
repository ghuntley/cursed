# CURSED Developer Guide

## Contributing to the CURSED Programming Language

Welcome to the CURSED developer community! This guide provides everything you need to build, test, contribute to, and extend the CURSED programming language ecosystem.

## Table of Contents

- [Development Environment Setup](#development-environment-setup)
- [Building the Compiler](#building-the-compiler)
- [Testing Framework](#testing-framework)
- [Contributing Guidelines](#contributing-guidelines)
- [Extending the Standard Library](#extending-the-standard-library)
- [IDE Integration Development](#ide-integration-development)
- [Advanced Development Topics](#advanced-development-topics)

## Development Environment Setup

### Prerequisites

**Required Tools:**
- **Zig 0.13.0+**: Primary build tool and implementation language
- **LLVM 16+**: Backend for code generation
- **Git**: Version control
- **Valgrind**: Memory leak detection (Linux/macOS)
- **GDB/LLDB**: Debugging support

**Platform-Specific Requirements:**

**Linux (Ubuntu/Debian):**
```bash
# Install dependencies
sudo apt update
sudo apt install -y build-essential llvm-16-dev libclang-16-dev
sudo apt install -y valgrind gdb git curl

# Install Zig
curl -L https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz | tar -xJ
sudo mv zig-linux-x86_64-0.13.0 /opt/zig
echo 'export PATH="/opt/zig:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**macOS:**
```bash
# Using Homebrew
brew install zig llvm@16 valgrind git

# Add LLVM to PATH
echo 'export PATH="/opt/homebrew/opt/llvm@16/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

**Windows:**
```bash
# Using Chocolatey
choco install zig llvm git

# Or use WSL2 with Linux setup
```

### Repository Setup

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Verify Zig installation
zig version

# Build the compiler
zig build

# Verify installation
./zig-out/bin/cursed-zig --version
```

### Development Environment Configuration

**Editor Setup:**

1. **VS Code** (Recommended for CURSED development):
```json
// .vscode/settings.json
{
    "zig.zls.enable": true,
    "zig.initialSetupDone": true,
    "files.associations": {
        "*.csd": "cursed",
        "*.zig": "zig"
    },
    "editor.formatOnSave": true
}
```

2. **Vim/Neovim:**
```lua
-- init.lua
require('lspconfig').zls.setup{}
require('lspconfig').cursed_lsp.setup{}
```

**Git Configuration:**
```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Set up git hooks
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Building the Compiler

### Standard Build Process

```bash
# Clean build (recommended for development)
zig build clean
zig build

# Development build with debug information
zig build -Doptimize=Debug

# Release build with optimizations
zig build -Doptimize=ReleaseFast

# Cross-compilation examples
zig build -Dtarget=x86_64-linux
zig build -Dtarget=aarch64-macos
zig build -Dtarget=x86_64-windows
```

### Build Targets

The CURSED build system produces multiple executables:

| Target | Purpose | Build Command |
|--------|---------|---------------|
| `cursed-zig` | Main compiler/interpreter | `zig build` |
| `cursed-lsp` | Language server | `zig build cursed-lsp` |
| `cursed-fmt` | Code formatter | `zig build cursed-fmt` |
| `cursed-lint` | Static analyzer | `zig build cursed-lint` |
| `cursed-debug` | Interactive debugger | `zig build cursed-debug` |
| `cursed-pkg` | Package manager | `zig build cursed-pkg` |
| `cursed-doc` | Documentation generator | `zig build cursed-doc` |

### Build Configuration

**build.zig Configuration:**
```zig
// Customize build options
const optimize = b.standardOptimizeOption(.{});
const target = b.standardTargetOptions(.{});

// Add custom build steps
const custom_step = b.step("custom", "Custom build step");
```

**Environment Variables:**
```bash
# Set LLVM path if not in standard location
export LLVM_SYS_160_PREFIX=/usr/lib/llvm-16

# Enable debug logging
export CURSED_DEBUG=1

# Set custom stack size for compilation
export CURSED_STACK_SIZE=16777216
```

### Build Troubleshooting

**Common Issues and Solutions:**

1. **LLVM Linking Errors:**
```bash
# Ensure LLVM is properly installed
llvm-config --version
export LLVM_SYS_160_PREFIX=$(llvm-config --prefix)
zig build clean && zig build
```

2. **Memory Issues During Build:**
```bash
# Use debug build to avoid optimization memory usage
zig build -Doptimize=Debug

# Increase stack size
ulimit -s unlimited
```

3. **Cross-Compilation Hanging:**
```bash
# Install target toolchain
sudo apt install gcc-aarch64-linux-gnu
zig build -Dtarget=aarch64-linux --verbose
```

## Testing Framework

### Running Tests

**Core Test Suite:**
```bash
# Run all Zig unit tests
zig build test

# Run CURSED language tests
./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd

# Run specific component tests
zig test src-zig/lexer.zig
zig test src-zig/parser.zig
zig test src-zig/type_system_runtime.zig
```

**Test Categories:**

1. **Unit Tests** (Zig tests for internal components):
```zig
// In src-zig/lexer.zig
test "lexer tokenizes basic types" {
    const allocator = testing.allocator;
    var lexer = Lexer.init("sus x drip = 42", allocator);
    defer lexer.deinit();
    
    const tokens = try lexer.tokenize();
    try testing.expectEqual(tokens.len, 5);
    try testing.expectEqual(tokens[0].type, .Sus);
}
```

2. **Integration Tests** (CURSED programs):
```cursed
fr fr test_suite/basic_syntax.csd
sus result drip = 2 + 2
ready (result != 4) {
    vibez.error("Basic arithmetic failed!")
    exit(1)
}
vibez.spill("✅ Basic syntax test passed")
```

3. **Standard Library Tests**:
```cursed
yeet "testz"

slay test_stringz_operations() {
    yeet "stringz"
    
    testz.assert_eq(stringz.upper("hello"), "HELLO", "String uppercase")
    testz.assert_eq(stringz.length("test"), 4, "String length")
    testz.assert_true(stringz.contains("hello world", "world"), "String contains")
}

testz.run_test("String Operations", test_stringz_operations)
testz.print_summary()
```

### Writing Tests

**Unit Test Guidelines:**
```zig
// Always use testing.allocator for memory management
test "memory safe operations" {
    const allocator = testing.allocator;
    var list = ArrayList(i32).init(allocator);
    defer list.deinit();
    
    try list.append(42);
    try testing.expectEqual(list.items[0], 42);
}

// Test error conditions
test "error handling" {
    try testing.expectError(error.DivisionByZero, divide(10, 0));
}

// Use descriptive test names
test "parser handles complex expressions with nested function calls" {
    // Test implementation
}
```

**CURSED Test Programs:**
```cursed
fr fr Use testz framework for systematic testing
yeet "testz"

slay test_array_operations() {
    yeet "arrayz"
    
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus doubled []drip = arrayz.map(numbers, slay(x) { damn x * 2 })
    
    testz.assert_eq(doubled.length(), 5, "Array length preserved")
    testz.assert_eq(doubled[0], 2, "First element doubled")
    testz.assert_eq(doubled[4], 10, "Last element doubled")
}

testz.run_test("Array Operations", test_array_operations)
```

### Performance Testing

**Benchmark Tests:**
```bash
# Run performance benchmarks
./scripts/benchmark_performance.sh

# Profile compilation speed
time zig build
hyperfine 'zig build clean && zig build'

# Profile runtime performance
./zig-out/bin/cursed-zig --benchmark benchmarks/array_ops.csd
./zig-out/bin/cursed-zig --profile benchmarks/concurrency.csd
```

**Memory Testing:**
```bash
# Memory leak detection
valgrind --leak-check=full --error-exitcode=1 \
  ./zig-out/bin/cursed-zig test_suite/memory_test.csd

# Address sanitizer
zig build -Doptimize=Debug -fsanitize=address
```

### Continuous Integration

**GitHub Actions Workflow:**
```yaml
# .github/workflows/test.yml
name: Test Suite
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Zig
        uses: goto-bus-stop/setup-zig@v2
        with:
          version: 0.13.0
      - name: Build
        run: zig build
      - name: Test
        run: |
          zig build test
          ./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd
      - name: Memory Test
        run: |
          valgrind --error-exitcode=1 \
            ./zig-out/bin/cursed-zig test_suite/basic_syntax.csd
```

## Contributing Guidelines

### Code Style and Standards

**Zig Code Style:**
```zig
// Use camelCase for variables and functions
const maxRetries = 10;
fn parseExpression() !void { }

// Use PascalCase for types
const TokenType = enum { ... };
const Parser = struct { ... };

// Use snake_case for file names
// src-zig/type_system_runtime.zig

// Use descriptive names
fn generateLLVMCodeForFunctionCall() !void { }  // Good
fn genCode() !void { }                          // Avoid
```

**CURSED Code Style:**
```cursed
fr fr Use descriptive variable names
sus user_count drip = 0                  fr fr Good
sus n drip = 0                          fr fr Avoid

fr fr Format function definitions consistently
slay calculate_total_price(items []Item) normie {
    sus total normie = 0.0
    bestie item in items {
        total += item.price
    }
    damn total
}

fr fr Use consistent indentation (4 spaces)
ready (condition) {
    fr fr 4 spaces indentation
    do_something()
}
```

### Commit Guidelines

**Commit Message Format:**
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (no logic changes)
- `refactor`: Code refactoring
- `test`: Adding or fixing tests
- `perf`: Performance improvements

**Examples:**
```bash
git commit -m "feat(parser): add support for generic function parsing"
git commit -m "fix(codegen): resolve LLVM IR generation for interface calls"
git commit -m "docs(stdlib): add comprehensive examples for stringz module"
```

### Pull Request Process

1. **Fork and Branch:**
```bash
git clone https://github.com/yourusername/cursed.git
git checkout -b feature/new-feature-name
```

2. **Development Process:**
```bash
# Make changes
# Write tests
zig build test
./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd

# Format code
./zig-out/bin/cursed-fmt src-zig/your-file.zig
```

3. **Quality Checks:**
```bash
# Run linter
./zig-out/bin/cursed-lint src-zig/
zig fmt --check src-zig/

# Memory safety check
valgrind --error-exitcode=1 ./zig-out/bin/cursed-zig test_program.csd

# Performance check
./scripts/benchmark_performance.sh
```

4. **Pull Request Checklist:**
- [ ] All tests pass
- [ ] New features have tests
- [ ] Documentation updated
- [ ] Memory safety verified
- [ ] Performance impact assessed
- [ ] Code formatted correctly

### Issue Reporting

**Bug Report Template:**
```markdown
## Bug Description
Brief description of the bug.

## Steps to Reproduce
1. Create file `test.csd` with content: ...
2. Run `cursed-zig test.csd`
3. Observe error: ...

## Expected Behavior
What should happen instead.

## Environment
- CURSED Version: 1.0.0
- Platform: Ubuntu 22.04
- Zig Version: 0.13.0

## Additional Context
Any additional information or context.
```

**Feature Request Template:**
```markdown
## Feature Description
Clear description of the proposed feature.

## Use Case
Specific use case or problem this would solve.

## Proposed Implementation
If applicable, how this could be implemented.

## Alternatives Considered
Other approaches you've considered.
```

## Extending the Standard Library

### Creating New Modules

**Module Structure:**
```
stdlib/
├── your_module/
│   ├── mod.csd              # Main module file
│   ├── README.md            # Documentation
│   ├── examples/            # Usage examples
│   └── tests/              # Module-specific tests
```

**Module Template:**
```cursed
fr fr stdlib/your_module/mod.csd
fr fr Your Module - Brief description of functionality
fr fr
fr fr This module provides [description of capabilities]

fr fr Export public functions
slay public_function(param tea) tea {
    damn "result"
}

fr fr Private helper functions
slay private_helper() {
    fr fr Implementation details
}

fr fr Module constants
sus MODULE_VERSION tea = "1.0.0"
sus DEFAULT_TIMEOUT drip = 5000

fr fr Module types
squad ModuleConfig {
    option1 tea,
    option2 drip,
    enabled lit
}

fr fr Module initialization
slay init() {
    fr fr Setup code
}

slay cleanup() {
    fr fr Cleanup code
}
```

**Documentation Requirements:**
```markdown
# Module Name

Brief description of what the module does.

## Functions

### `function_name(param: type) -> return_type`
Description of what the function does.

**Parameters:**
- `param`: Description of parameter

**Returns:**
- Description of return value

**Example:**
```cursed
yeet "your_module"
sus result = your_module.function_name("example")
```

### Integration Process

1. **Add to Build System:**
```zig
// In build.zig
const your_module_step = b.addExecutable(.{
    .name = "test_your_module",
    .root_source_file = .{ .path = "stdlib/your_module/tests/test_main.csd" },
    .target = target,
    .optimize = optimize,
});
```

2. **Add to Standard Library Index:**
```cursed
fr fr In stdlib/mod.csd
yeet "your_module"
```

3. **Write Comprehensive Tests:**
```cursed
fr fr stdlib/your_module/tests/test_main.csd
yeet "testz"
yeet "your_module"

slay test_basic_functionality() {
    sus result = your_module.function_name("test")
    testz.assert_eq(result, "expected", "Basic functionality")
}

slay test_error_handling() {
    sus result = your_module.error_prone_function("") fam {
        when "ExpectedError" -> {
            testz.pass("Error handled correctly")
            damn
        }
        when _ -> {
            testz.fail("Unexpected error")
            damn
        }
    }
}

testz.run_test("Basic Functionality", test_basic_functionality)
testz.run_test("Error Handling", test_error_handling)
testz.print_summary()
```

### Best Practices for Module Development

1. **Error Handling:**
```cursed
fr fr Use specific error types
enum ModuleError {
    InvalidInput,
    NetworkTimeout,
    ConfigurationError(tea)
}

slay module_function(input tea) yikes<ModuleError> {
    ready (input.length() == 0) {
        yikes ModuleError.InvalidInput
    }
    fr fr Process input
}
```

2. **Resource Management:**
```cursed
squad ResourceManager {
    handle drip,
    
    slay init(config ModuleConfig) ResourceManager {
        fr fr Initialize resource
        damn ResourceManager{ handle: create_handle() }
    }
    
    slay cleanup(self) {
        fr fr Clean up resources
        close_handle(self.handle)
    }
}
```

3. **Performance Considerations:**
```cursed
fr fr Use efficient algorithms and data structures
fr fr Profile your module with the benchmarking framework
slay benchmark_module_function() {
    sus start_time = timez.now()
    
    bestie i := 0; i < 10000; i++ {
        module_function("test input")
    }
    
    sus end_time = timez.now()
    sus duration = end_time - start_time
    vibez.spill("Function executed 10000 times in", duration, "ms")
}
```

## IDE Integration Development

### Language Server Protocol (LSP)

**LSP Server Architecture:**
```zig
// src-zig/lsp/server.zig
const LspServer = struct {
    allocator: Allocator,
    workspace: WorkspaceManager,
    parser: Parser,
    
    pub fn init(allocator: Allocator) LspServer {
        return LspServer{
            .allocator = allocator,
            .workspace = WorkspaceManager.init(allocator),
            .parser = Parser.init(allocator),
        };
    }
    
    pub fn handleRequest(self: *LspServer, request: JsonRpc) !JsonRpc {
        switch (request.method) {
            "textDocument/completion" => return self.handleCompletion(request),
            "textDocument/hover" => return self.handleHover(request),
            "textDocument/definition" => return self.handleDefinition(request),
            else => return error.MethodNotFound,
        }
    }
};
```

**Adding LSP Features:**

1. **Code Completion:**
```zig
fn handleCompletion(self: *LspServer, request: JsonRpc) !JsonRpc {
    const params = request.params.textDocument;
    const document = try self.workspace.getDocument(params.uri);
    const position = params.position;
    
    // Parse document and find completion context
    const completions = try self.getCompletions(document, position);
    
    return JsonRpc{
        .id = request.id,
        .result = completions,
    };
}
```

2. **Error Diagnostics:**
```zig
fn publishDiagnostics(self: *LspServer, uri: []const u8) !void {
    const document = try self.workspace.getDocument(uri);
    const diagnostics = try self.parser.validateDocument(document);
    
    const notification = JsonRpc{
        .method = "textDocument/publishDiagnostics",
        .params = .{
            .uri = uri,
            .diagnostics = diagnostics,
        },
    };
    
    try self.sendNotification(notification);
}
```

### VS Code Extension Development

**Extension Structure:**
```
vscode-cursed-extension/
├── package.json
├── src/
│   ├── extension.ts
│   ├── language-configuration.json
│   └── cursed.tmLanguage.json
├── syntaxes/
└── snippets/
```

**Syntax Highlighting Grammar:**
```json
{
    "name": "CURSED",
    "scopeName": "source.cursed",
    "patterns": [
        {
            "name": "keyword.control.cursed",
            "match": "\\b(ready|otherwise|bestie|sick|when|go|select)\\b"
        },
        {
            "name": "storage.type.cursed",
            "match": "\\b(sus|slay|squad|collab|enum)\\b"
        },
        {
            "name": "constant.language.cursed",
            "match": "\\b(based|cap|damn)\\b"
        }
    ]
}
```

**Extension Activation:**
```typescript
// src/extension.ts
import * as vscode from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';

export function activate(context: vscode.ExtensionContext) {
    // Start LSP client
    const serverOptions = {
        command: 'cursed-lsp',
        args: ['--stdio'],
    };
    
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'cursed' }],
    };
    
    const client = new LanguageClient(
        'cursed-lsp',
        'CURSED Language Server',
        serverOptions,
        clientOptions
    );
    
    client.start();
    context.subscriptions.push(client);
}
```

### Debugger Integration

**DWARF Debug Information:**
```zig
// Generate debug info during compilation
fn generateDebugInfo(self: *CodeGen, function: *Function) !void {
    const debug_info = try self.llvm_context.createDISubprogram(.{
        .name = function.name,
        .linkage_name = function.linkage_name,
        .file = self.debug_file,
        .line = function.line_number,
        .type = function.debug_type,
        .scope = self.debug_scope,
    });
    
    function.llvm_function.setSubprogram(debug_info);
}
```

**GDB Pretty Printers:**
```python
# scripts/gdb_cursed_printers.py
import gdb

class CursedStringPrinter:
    def __init__(self, val):
        self.val = val
    
    def to_string(self):
        return f"tea \"{self.val['data'].string()}\""

def register_cursed_printers():
    pp = gdb.printing.RegexpCollectionPrettyPrinter("cursed")
    pp.add_printer('tea', '^cursed::String$', CursedStringPrinter)
    gdb.printing.register_pretty_printer(gdb.current_objfile(), pp)

register_cursed_printers()
```

## Advanced Development Topics

### Custom LLVM Passes

**Implementing Optimization Passes:**
```zig
// src-zig/optimization/cursed_passes.zig
const CursedOptimizationPass = struct {
    pub fn run(module: *llvm.Module) !void {
        const functions = module.getFunctions();
        
        for (functions) |function| {
            try optimizeFunction(function);
        }
    }
    
    fn optimizeFunction(function: *llvm.Function) !void {
        // CURSED-specific optimizations
        try eliminateDeadGoroutines(function);
        try optimizeChannelOperations(function);
        try inlineSmallFunctions(function);
    }
};
```

### Memory Pool Implementation

**Custom Allocators:**
```zig
// src-zig/memory/pool_allocator.zig
const PoolAllocator = struct {
    const Pool = struct {
        memory: []u8,
        free_list: ?*FreeNode,
        block_size: usize,
    };
    
    pools: std.HashMap(usize, Pool),
    
    pub fn alloc(self: *PoolAllocator, size: usize) ![]u8 {
        const pool = self.pools.getPtr(size) orelse {
            const new_pool = try self.createPool(size);
            try self.pools.put(size, new_pool);
            return new_pool.allocate();
        };
        
        return pool.allocate();
    }
};
```

### Profiling Integration

**Built-in Profiler:**
```zig
// src-zig/profiling/profiler.zig
const Profiler = struct {
    samples: std.ArrayList(Sample),
    start_time: i64,
    
    const Sample = struct {
        function_name: []const u8,
        duration_ns: u64,
        memory_allocated: usize,
    };
    
    pub fn startProfiling(self: *Profiler, function_name: []const u8) void {
        self.current_sample = Sample{
            .function_name = function_name,
            .start_time = std.time.nanoTimestamp(),
        };
    }
    
    pub fn endProfiling(self: *Profiler) void {
        self.current_sample.duration_ns = std.time.nanoTimestamp() - self.current_sample.start_time;
        self.samples.append(self.current_sample) catch {};
    }
};
```

### Cross-Compilation Support

**Adding New Targets:**
```zig
// build.zig
const supported_targets = [_]std.Target.Query{
    .{ .cpu_arch = .x86_64, .os_tag = .linux },
    .{ .cpu_arch = .x86_64, .os_tag = .macos },
    .{ .cpu_arch = .x86_64, .os_tag = .windows },
    .{ .cpu_arch = .aarch64, .os_tag = .linux },
    .{ .cpu_arch = .aarch64, .os_tag = .macos },
    .{ .cpu_arch = .wasm32, .os_tag = .wasi },
    // Add new target here
    .{ .cpu_arch = .riscv64, .os_tag = .linux },
};
```

**Platform-Specific Code:**
```zig
// src-zig/platform/platform.zig
pub const Platform = switch (builtin.target.os.tag) {
    .linux => @import("linux.zig"),
    .macos => @import("macos.zig"),
    .windows => @import("windows.zig"),
    else => @compileError("Unsupported platform"),
};
```

## Development Workflow Best Practices

### Daily Development Cycle

```bash
# 1. Update and build
git pull origin main
zig build

# 2. Make changes
# Edit files...

# 3. Test changes
zig build test
./zig-out/bin/cursed-zig comprehensive_stdlib_test.csd

# 4. Memory safety check
valgrind --error-exitcode=1 ./zig-out/bin/cursed-zig your_test.csd

# 5. Format and lint
zig fmt src-zig/
./zig-out/bin/cursed-lint src-zig/

# 6. Commit changes
git add .
git commit -m "feat(component): description of changes"
git push origin your-branch
```

### Performance Development

**Benchmarking New Features:**
```bash
# Before implementing feature
./scripts/benchmark_performance.sh > before.txt

# Implement feature
# ...

# After implementing feature
./scripts/benchmark_performance.sh > after.txt

# Compare results
diff before.txt after.txt
```

**Memory Profiling:**
```bash
# Profile memory usage
valgrind --tool=massif ./zig-out/bin/cursed-zig large_program.csd

# Analyze memory usage
ms_print massif.out.* | less
```

### Documentation Development

**API Documentation:**
```zig
/// Calculate the factorial of a number
/// 
/// This function computes the factorial using iterative approach
/// for better performance compared to recursive implementation.
/// 
/// Arguments:
///   n: The number to calculate factorial for (must be >= 0)
/// 
/// Returns:
///   The factorial of n, or error.Overflow if result is too large
/// 
/// Example:
///   const result = try factorial(5); // returns 120
pub fn factorial(n: u32) !u64 {
    // Implementation...
}
```

**Module Documentation:**
```cursed
fr fr Module: mathz - Advanced Mathematical Operations
fr fr
fr fr The mathz module provides comprehensive mathematical functions
fr fr including basic arithmetic, trigonometry, statistics, and
fr fr advanced numerical operations.
fr fr
fr fr Quick Start:
fr fr   yeet "mathz"
fr fr   sus result = mathz.sqrt(25.0)  // 5.0
fr fr
fr fr Performance Notes:
fr fr   - All functions are optimized for performance
fr fr   - Uses LLVM's math intrinsics when available
fr fr   - Memory allocation is minimized
```

## Community and Support

### Getting Help

1. **GitHub Discussions**: Ask questions and share ideas
2. **Discord Server**: Real-time chat with developers
3. **Stack Overflow**: Tag questions with `cursed-lang`
4. **Documentation**: Comprehensive guides and API reference

### Contribution Recognition

**Recognition System:**
- **Core Contributors**: Major language features and stdlib modules
- **Maintainers**: Ongoing project maintenance and reviews  
- **Community Leaders**: Documentation, tutorials, community building
- **Bug Hunters**: Finding and reporting critical issues
- **Performance Engineers**: Optimization and benchmarking

**Acknowledgments:**
Contributors are recognized in:
- Release notes
- Contributors file
- Project documentation
- Conference presentations

## Future Development

### Roadmap Participation

**Short-term Goals (Q4 2025):**
- Performance optimization passes
- Additional standard library modules
- Enhanced IDE features
- Package registry improvements

**Medium-term Goals (2026):**
- Language feature extensions
- Mobile and embedded targets
- Advanced debugging tools
- Enterprise monitoring features

**Long-term Vision (2027+):**
- Self-hosting compiler improvements
- Advanced type system features
- Research collaborations
- Educational platform integration

### Research Areas

**Active Research Topics:**
1. **Compile-time Computation**: Advanced meta-programming capabilities
2. **Concurrency Models**: Actor systems and software transactional memory
3. **Memory Management**: Region-based allocation and linear types
4. **Performance**: Profile-guided optimization and machine learning integration
5. **Verification**: Formal verification and property-based testing

---

**Welcome to the CURSED development community!** 

Whether you're fixing bugs, implementing new features, optimizing performance, or improving documentation, your contributions help make CURSED better for everyone. Join us in building the future of systems programming! 🚀

**Next Steps:**
1. Set up your development environment
2. Build the compiler and run tests
3. Pick an issue from our GitHub repository
4. Join our Discord community
5. Make your first contribution!

---

**Documentation Version**: 1.0.0  
**Last Updated**: August 21, 2025  
**Community**: [Discord](https://discord.gg/cursed-lang) | [GitHub](https://github.com/ghuntley/cursed) | [Discussions](https://github.com/ghuntley/cursed/discussions)
