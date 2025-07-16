# WASM Mood - WebAssembly Support for CURSED

Complete WebAssembly compilation and runtime support for CURSED programs. Compile CURSED code to WebAssembly and execute WASM modules within CURSED applications.

## Features

- **CURSED to WASM Compilation**: Direct compilation of CURSED source to WebAssembly bytecode
- **WebAssembly Runtime**: Embedded WASM runtime for executing WebAssembly modules
- **Memory Management**: Safe and efficient memory handling for WASM modules
- **Import/Export System**: Seamless function and memory sharing between CURSED and WASM
- **Multiple Formats**: Support for both binary (.wasm) and text (.wat) formats
- **Optimization**: Multiple optimization levels for performance tuning

## Quick Start

```cursed
yeet "wasm_mood"

# Compile CURSED source to WebAssembly
sus source := "slay add(a normie, b normie) normie { damn a + b }"
sus options WasmCompileOptions
options.optimization_level = 2
options.target_format = "wasm"

sus module := wasm_compile_from_source(source, options)

# Create runtime and load module
sus config WasmRuntimeConfig
config.max_memory = 1048576  # 1MB
sus runtime := wasm_create_runtime(config)
sus instance := wasm_load_module(runtime, module)

# Call WebAssembly function
sus arg1 WasmValue
arg1.value_type = "i32"
arg1.int_value = 10

sus arg2 WasmValue  
arg2.value_type = "i32"
arg2.int_value = 32

sus args := []WasmValue{arg1, arg2}
sus result := wasm_call_function(instance, "add", args)
vibez.spill("Result: ", result.int_value)  # Output: Result: 42
```

## Core Types

### WasmModule
Represents a compiled WebAssembly module.

```cursed
be_like WasmModule = struct {
    bytecode []byte,        # Compiled WASM bytecode
    exports map[tea]normie, # Exported functions
    imports map[tea]normie, # Imported functions
    memory_size normie,     # Memory size in bytes
    validated lit,          # Module validation status
    functions []WasmFunction,
    memory_pages normie
}
```

### WasmRuntime
WebAssembly execution runtime environment.

```cursed
be_like WasmRuntime = struct {
    instances []WasmInstance,    # Loaded module instances
    memory_pool []WasmMemory,    # Memory management
    config WasmRuntimeConfig,    # Runtime configuration
    active lit                   # Runtime status
}
```

### WasmInstance
A loaded and executable WebAssembly module instance.

```cursed
be_like WasmInstance = struct {
    module WasmModule,                    # Source module
    memory WasmMemory,                    # Instance memory
    function_table map[tea]WasmFunction,  # Function lookup table
    globals map[tea]normie,               # Global variables
    exports map[tea]normie,               # Exported items
    running lit                           # Execution status
}
```

## Compilation Functions

### wasm_compile_from_source
Compile CURSED source code to WebAssembly.

```cursed
slay wasm_compile_from_source(source tea, options WasmCompileOptions) WasmModule

# Example
sus source := """
    slay fibonacci(n normie) normie {
        yikes n <= 1 {
            damn n
        }
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
"""

sus options WasmCompileOptions
options.optimization_level = 3
options.enable_debug = cap
options.enable_bounds_check = based

sus module := wasm_compile_from_source(source, options)
```

### wasm_compile_from_file
Compile CURSED source file to WebAssembly.

```cursed
slay wasm_compile_from_file(filepath tea, options WasmCompileOptions) WasmModule

# Example
sus module := wasm_compile_from_file("math_functions.csd", options)
```

### wasm_optimize_module
Apply optimization passes to a WebAssembly module.

```cursed
slay wasm_optimize_module(module WasmModule, level normie) WasmModule

# Optimization levels:
# 0 - No optimization
# 1 - Basic optimization  
# 2 - Intermediate optimization
# 3 - Aggressive optimization

sus optimized := wasm_optimize_module(module, 3)
```

## Runtime Functions

### wasm_create_runtime
Create a new WebAssembly runtime environment.

```cursed
slay wasm_create_runtime(config WasmRuntimeConfig) WasmRuntime

# Example
sus config WasmRuntimeConfig
config.max_memory = 2097152      # 2MB max memory
config.max_instances = 10        # Max 10 instances
config.enable_jit = based        # Enable JIT compilation
config.stack_size = 65536        # 64KB stack
config.timeout_ms = 10000        # 10 second timeout

sus runtime := wasm_create_runtime(config)
```

### wasm_load_module
Load a WebAssembly module into the runtime.

```cursed
slay wasm_load_module(runtime WasmRuntime, module WasmModule) WasmInstance

# Example
sus instance := wasm_load_module(runtime, module)
yikes !instance.running {
    vibez.spill("Failed to load module")
}
```

### wasm_call_function
Call a function in a WebAssembly instance.

```cursed
slay wasm_call_function(instance WasmInstance, func_name tea, args []WasmValue) WasmValue

# Example - calling a math function
sus x_val WasmValue
x_val.value_type = "i32"
x_val.int_value = 25

sus args := []WasmValue{x_val}
sus result := wasm_call_function(instance, "square_root", args)
vibez.spill("Square root of 25: ", result.int_value)
```

## Memory Management

### wasm_alloc_memory
Allocate WebAssembly linear memory.

```cursed
slay wasm_alloc_memory(size normie) WasmMemory

# Example - allocate 128KB
sus memory := wasm_alloc_memory(131072)
```

### wasm_write_memory / wasm_read_memory
Write to and read from WebAssembly memory.

```cursed
slay wasm_write_memory(memory WasmMemory, offset normie, data []byte) lit
slay wasm_read_memory(memory WasmMemory, offset normie, size normie) []byte

# Example - store and retrieve data
sus data := []byte{0x48, 0x65, 0x6C, 0x6C, 0x6F}  # "Hello"
wasm_write_memory(memory, 1000, data)

sus retrieved := wasm_read_memory(memory, 1000, 5)
# retrieved now contains "Hello"
```

## Import/Export System

### Function Exports
Export CURSED functions to WebAssembly modules.

```cursed
# Export a CURSED function to WASM
sus math_func WasmFunction
math_func.name = "multiply"
math_func.signature = "i32, i32 -> i32"
math_func.param_count = 2
math_func.return_count = 1

wasm_add_export(module, "multiply", math_func)
```

### Function Imports
Import functions from WebAssembly modules.

```cursed
# Import a function from WASM to CURSED
sus imported_func WasmFunction
imported_func.name = "external_calculation"
imported_func.signature = "f64 -> f64"

wasm_add_import(module, "external_calculation", imported_func)
```

### List Imports and Exports
Inspect module imports and exports.

```cursed
sus exports := wasm_list_exports(module)
bestie export_name := range exports {
    vibez.spill("Export: ", export_name)
}

sus imports := wasm_list_imports(module)  
bestie import_name := range imports {
    vibez.spill("Import: ", import_name)
}
```

## WebAssembly Text Format

### Convert to WAT
Convert binary WebAssembly to text format.

```cursed
slay wasm_module_to_wat(module WasmModule) tea

# Example
sus wat_text := wasm_module_to_wat(module)
vibez.spill("WAT Format:")
vibez.spill(wat_text)
```

### Parse WAT
Parse WebAssembly text format.

```cursed
slay wasm_wat_to_module(wat tea) WasmModule

# Example
sus wat := """
(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add)
  (export "add" (func $add)))
"""

sus module := wasm_wat_to_module(wat)
```

## Error Handling

WebAssembly operations use CURSED's error handling system:

```cursed
sus result := wasm_compile_from_source(source, options)
yikes !result.validated {
    vibez.spill("Compilation failed")
    damn
}

# Memory operations with bounds checking
sus write_ok := wasm_write_memory(memory, offset, data)
yikes !write_ok {
    vibez.spill("Memory write failed - out of bounds")
}
```

## Performance Optimization

### Optimization Levels
- **Level 0**: No optimization, fastest compilation
- **Level 1**: Basic optimizations, balanced performance
- **Level 2**: Intermediate optimizations, good performance
- **Level 3**: Aggressive optimizations, best runtime performance

### JIT Compilation
Enable Just-In-Time compilation for better runtime performance:

```cursed
sus config WasmRuntimeConfig
config.enable_jit = based  # Enable JIT compilation
config.stack_size = 131072 # Larger stack for JIT
```

### Memory Tuning
Optimize memory usage:

```cursed
# Use appropriate memory sizes
sus config WasmRuntimeConfig
config.max_memory = 1048576  # 1MB for small programs
config.max_memory = 16777216 # 16MB for larger programs

# Allocate memory in page-aligned sizes
sus memory := wasm_alloc_memory(65536 * 4)  # 4 pages (256KB)
```

## Advanced Examples

### Complete Web Application Backend
```cursed
yeet "wasm_mood"
yeet "web_vibez"

# Compile web handler to WASM
sus handler_source := """
    slay handle_request(method normie, path tea) tea {
        ready method {
            1 -> damn handle_get(path)     # GET
            2 -> damn handle_post(path)    # POST
            basic -> damn "404 Not Found"
        }
    }
    
    slay handle_get(path tea) tea {
        yikes path == "/api/status" {
            damn "OK"
        }
        damn "Not Found"
    }
"""

sus options WasmCompileOptions
options.optimization_level = 3
options.target_format = "wasm"

sus web_module := wasm_compile_from_source(handler_source, options)
sus runtime := wasm_create_runtime(WasmRuntimeConfig{
    max_memory: 2097152,
    enable_jit: based
})
sus instance := wasm_load_module(runtime, web_module)

# Use in web server
slay process_request(method normie, path tea) tea {
    sus method_val WasmValue
    method_val.value_type = "i32"
    method_val.int_value = method
    
    sus path_val WasmValue
    path_val.value_type = "string"  # Custom string type
    
    sus args := []WasmValue{method_val, path_val}
    sus result := wasm_call_function(instance, "handle_request", args)
    
    damn result.string_value
}
```

### Mathematical Computing Engine
```cursed
# Compile mathematical functions to WASM for performance
sus math_source := """
    slay matrix_multiply(a [][]normie, b [][]normie, rows normie, cols normie) [][]normie {
        sus result := make([][]normie, rows)
        bestie i := 0; i < rows; i++ {
            result[i] = make([]normie, cols)
            bestie j := 0; j < cols; j++ {
                sus sum := 0
                bestie k := 0; k < cols; k++ {
                    sum += a[i][k] * b[k][j]
                }
                result[i][j] = sum
            }
        }
        damn result
    }
"""

sus math_module := wasm_compile_from_source(math_source, WasmCompileOptions{
    optimization_level: 3,
    enable_bounds_check: cap  # Disable for performance
})

# Use for high-performance computing
sus matrix_instance := wasm_load_module(runtime, math_module)
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/wasm_mood/test_wasm_mood.csd
```

Test compilation mode:
```bash
cargo run --bin cursed -- compile stdlib/wasm_mood/test_wasm_mood.csd
./test_wasm_mood
```

## Integration with CURSED Ecosystem

### Build System Integration
```cursed
# In CursedBuild.toml
[targets.wasm]
type = "wasm"
optimization_level = 2
output = "dist/app.wasm"
```

### Package Dependencies
```cursed
# In CursedPackage.toml
[dependencies]
wasm_mood = "^1.0.0"
web_vibez = "^2.0.0"
```

### Performance Monitoring
```cursed
yeet "wasm_mood"
yeet "performance"

# Monitor WASM execution performance
sus start_time := performance.now()
sus result := wasm_call_function(instance, "compute_intensive", args)
sus end_time := performance.now()
vibez.spill("WASM execution time: ", end_time - start_time, "ms")
```

## Security

WebAssembly modules execute in a secure sandbox:
- Memory isolation between instances
- Controlled import/export system
- Bounds checking for memory operations
- Resource limits (memory, execution time)
- No direct access to host system

## Browser Compatibility

Generated WebAssembly is compatible with:
- All modern browsers (Chrome, Firefox, Safari, Edge)
- Node.js with WebAssembly support
- Deno runtime
- Any WASM-compatible runtime environment

## Performance Notes

- JIT compilation provides 2-5x performance improvement
- Memory operations are bounds-checked by default
- Optimization level 3 provides best runtime performance
- Use appropriate memory sizes to avoid GC pressure
- Consider disabling bounds checking for trusted code

---

The `wasm_mood` module provides production-ready WebAssembly support for CURSED, enabling high-performance code compilation and execution while maintaining the language's safety guarantees.
