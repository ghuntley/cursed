# CURSED WebAssembly Deployment Guide

This comprehensive guide covers compiling CURSED programs to WebAssembly (WASM) and deploying them in various environments.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Compilation Targets](#compilation-targets)
- [Browser Deployment](#browser-deployment)
- [WASI Deployment](#wasi-deployment)
- [JavaScript Interop](#javascript-interop)
- [Performance Optimization](#performance-optimization)
- [Troubleshooting](#troubleshooting)
- [Advanced Usage](#advanced-usage)

## Overview

CURSED supports compilation to WebAssembly, enabling CURSED programs to run in:

- **Web Browsers** - Interactive web applications with JavaScript integration
- **WASI Environments** - Command-line tools and serverless functions
- **Embedded Systems** - Lightweight WASM runtime environments
- **Cross-Platform Applications** - Universal binary format

### Key Benefits

✅ **Universal Compatibility** - Runs on any platform with WASM support  
✅ **Near-Native Performance** - Compiled to efficient WASM bytecode  
✅ **Memory Safety** - Sandboxed execution environment  
✅ **Small Binary Size** - Optimized for web deployment  
✅ **JavaScript Integration** - Seamless interop with web APIs  

## Quick Start

### 1. Install Requirements

```bash
# Zig compiler (latest)
curl https://ziglang.org/download/0.12.0/zig-linux-x86_64-0.12.0.tar.xz | tar -xJ
export PATH=$PATH:./zig-linux-x86_64-0.12.0

# Optional: WASI runtime for testing
curl https://wasmtime.dev/install.sh -sSf | bash
```

### 2. Build WASM Targets

```bash
# Build all WASM targets
./build_wasm.sh all

# Or build specific targets
./build_wasm.sh browser --with-html
./build_wasm.sh wasi --debug
```

### 3. Test Your Build

```bash
# Test all builds
./build_wasm.sh test

# Run browser demo
python3 -m http.server 8000 -d build/wasm
# Open http://localhost:8000

# Run WASI demo
wasmtime build/wasm/cursed-wasi.wasm
```

## Compilation Targets

### Browser Target (wasm32-browser)

**Use Case**: Interactive web applications with DOM access and JavaScript interop.

```bash
# Compile CURSED program for browser
cursed-zig --backend wasm --target wasm32-browser app.csd -o app.wasm

# Or using build system
zig build wasm-browser -Doptimize=ReleaseFast
```

**Features**:
- JavaScript function imports/exports
- DOM manipulation via JS bindings
- Browser API access (localStorage, fetch, etc.)
- Automatic memory management
- Size-optimized builds

**Generated Files**:
- `app.wasm` - WebAssembly binary
- `app.js` - JavaScript bindings (optional)
- `app.html` - HTML wrapper (optional)

### WASI Target (wasm32-wasi)

**Use Case**: Command-line tools, serverless functions, and server-side applications.

```bash
# Compile CURSED program for WASI
cursed-zig --backend wasm --target wasm32-wasi cli.csd -o cli.wasm

# Or using build system
zig build wasm-wasi -Doptimize=ReleaseFast
```

**Features**:
- File system access
- Command-line arguments
- Environment variables
- Standard I/O operations
- Network access (runtime dependent)

**Runtime Support**:
- Wasmtime
- Wasmer
- Node.js (with WASI polyfill)
- Deno
- Cloud Functions (AWS Lambda, Cloudflare Workers)

### Freestanding Target (wasm32-freestanding)

**Use Case**: Embedded systems and minimal runtime environments.

```bash
# Compile for freestanding WASM
cursed-zig --backend wasm --target wasm32-freestanding minimal.csd -o minimal.wasm
```

**Features**:
- No host dependencies
- Minimal binary size
- Custom memory management
- Embedded system compatible

## Browser Deployment

### Basic HTML Integration

Create an HTML file to load your CURSED WASM module:

```html
<!DOCTYPE html>
<html>
<head>
    <title>CURSED WASM App</title>
</head>
<body>
    <div id="output"></div>
    <button onclick="runCursed()">Run CURSED Code</button>
    
    <script>
        let cursedModule;
        
        // Load WASM module
        WebAssembly.instantiateStreaming(fetch('app.wasm'), {
            js: {
                console_log: (ptr, len) => {
                    const str = readString(ptr, len);
                    document.getElementById('output').innerHTML += str + '<br>';
                }
            }
        }).then(result => {
            cursedModule = result.instance;
            console.log('CURSED WASM loaded!');
        });
        
        // Helper to read strings from WASM memory
        function readString(ptr, len) {
            const bytes = new Uint8Array(cursedModule.exports.memory.buffer, ptr, len);
            return new TextDecoder().decode(bytes);
        }
        
        // Call CURSED functions
        function runCursed() {
            if (cursedModule) {
                cursedModule.exports.main();
            }
        }
    </script>
</body>
</html>
```

### Using Generated JavaScript Bindings

The CURSED compiler can generate JavaScript bindings automatically:

```bash
./build_wasm.sh browser --with-js --with-html
```

This creates a `CursedModule` class:

```javascript
// Load and use CURSED module
const cursed = new CursedModule();
await cursed.load('app.wasm');

// Call exported functions
const result = cursed.call('calculate', 42, 13);
console.log('Result:', result);

// Access all exports
console.log('Available functions:', cursed.getExports());
```

### Advanced Browser Integration

#### DOM Manipulation

CURSED code can interact with the DOM through JavaScript imports:

```cursed
// CURSED code (app.csd)
yeet "domz"  // DOM manipulation module

slay create_button(text tea) {
    sus btn domz.Element = domz.create_element("button")
    domz.set_text(btn, text)
    domz.add_event_listener(btn, "click", button_clicked)
    domz.append_to_body(btn)
}

slay button_clicked() {
    vibez.spill("Button clicked from CURSED!")
}
```

#### Fetch API Integration

```cursed
// CURSED code with HTTP requests
yeet "httpz"  // HTTP client module

slay fetch_data() {
    sus response tea = httpz.get("https://api.example.com/data") fam {
        when httpz.NetworkError -> {
            vibez.spill("Network error occurred")
            damn ""
        }
    }
    damn response
}
```

### Deployment to CDN

Deploy CURSED WASM apps to any static hosting:

```bash
# Build optimized for production
./build_wasm.sh browser --optimize

# Deploy to various platforms
# Netlify
netlify deploy --prod --dir=build/wasm

# Vercel
vercel build/wasm

# GitHub Pages
git add build/wasm && git commit -m "Deploy WASM app"
git push origin main
```

## WASI Deployment

### Local Development

```bash
# Build and run locally
./build_wasm.sh wasi --debug
wasmtime build/wasm/cursed-wasi.wasm --help

# With command line arguments
wasmtime build/wasm/cursed-wasi.wasm hello Alice
wasmtime build/wasm/cursed-wasi.wasm file README.md
```

### Serverless Functions

#### AWS Lambda with WASI

```yaml
# serverless.yml
service: cursed-wasi-lambda

provider:
  name: aws
  runtime: provided.al2
  
functions:
  cursed-function:
    handler: bootstrap
    events:
      - httpApi:
          path: /api/{proxy+}
          method: ANY

# Deploy with WASM runtime layer
plugins:
  - serverless-wasm
```

#### Cloudflare Workers

```javascript
// worker.js
import { WASI } from '@cloudflare/wasi';

export default {
  async fetch(request, env) {
    const wasi = new WASI();
    const wasmModule = await WebAssembly.instantiateStreaming(
      fetch('/cursed-app.wasm'), 
      wasi.getImports()
    );
    
    wasi.start(wasmModule);
    return new Response('CURSED WASM executed!');
  }
};
```

#### Deno Deploy

```typescript
// main.ts
import { serve } from "https://deno.land/std/http/server.ts";

const wasmModule = await WebAssembly.instantiateStreaming(
  fetch(new URL("./cursed-app.wasm", import.meta.url))
);

serve((req) => {
  // Execute CURSED WASM code
  const result = wasmModule.instance.exports.handle_request();
  return new Response(`CURSED result: ${result}`);
});
```

### Container Deployment

```dockerfile
# Dockerfile
FROM wasmtime/wasmtime:latest

COPY build/wasm/cursed-wasi.wasm /app/
WORKDIR /app

ENTRYPOINT ["wasmtime", "cursed-wasi.wasm"]
```

```bash
# Build and run container
docker build -t cursed-wasi .
docker run -it cursed-wasi hello world
```

## JavaScript Interop

### Calling JavaScript from CURSED

```cursed
// Import JavaScript functions
extern slay js_alert(message tea)
extern slay js_fetch(url tea) tea
extern slay js_local_storage_set(key tea, value tea)

slay show_notification() {
    js_alert("Hello from CURSED!")
}

slay save_data(data tea) {
    js_local_storage_set("cursed-data", data)
}
```

### Calling CURSED from JavaScript

```javascript
// JavaScript side
const wasmImports = {
  js: {
    alert: (ptr, len) => {
      const message = readString(ptr, len);
      alert(message);
    },
    fetch: async (ptr, len) => {
      const url = readString(ptr, len);
      const response = await fetch(url);
      return writeString(await response.text());
    },
    local_storage_set: (keyPtr, keyLen, valuePtr, valueLen) => {
      const key = readString(keyPtr, keyLen);
      const value = readString(valuePtr, valueLen);
      localStorage.setItem(key, value);
    }
  }
};

// Load WASM with imports
WebAssembly.instantiateStreaming(fetch('app.wasm'), wasmImports)
  .then(result => {
    // Call CURSED functions
    result.instance.exports.show_notification();
    result.instance.exports.save_data();
  });
```

### Type Marshalling

CURSED provides automatic marshalling for common types:

| CURSED Type | WASM Type | JavaScript Type |
|-------------|-----------|-----------------|
| `drip` (integer) | `i32` | `number` |
| `tea` (string) | `i32, i32` | `string` |
| `lit` (boolean) | `i32` | `boolean` |
| `[]drip` (array) | `i32, i32` | `Int32Array` |

## Performance Optimization

### Build-Time Optimizations

```bash
# Size optimization (for web)
./build_wasm.sh browser --optimize
zig build wasm-browser -Doptimize=ReleaseSmall

# Speed optimization (for compute-heavy tasks)
zig build wasm-browser -Doptimize=ReleaseFast

# Debug builds (for development)
./build_wasm.sh browser --debug --verbose
```

### Runtime Optimizations

#### Memory Management

```cursed
// Use memory pools for frequent allocations
yeet "memoryz"

slay process_data(data []tea) {
    sus pool memoryz.Pool = memoryz.create_pool(1024 * 1024) // 1MB pool
    defer memoryz.destroy_pool(pool)
    
    // Use pool for temporary allocations
    sus results []drip = memoryz.alloc_array(pool, drip, data.len)
    
    // Process data efficiently
    bestie (i drip = 0; i < data.len; i = i + 1) {
        results[i] = process_item(data[i])
    }
}
```

#### SIMD Optimization

```cursed
// Enable SIMD for parallel processing
slay vector_add(a []drip, b []drip) []drip {
    sus result []drip = make_array(drip, a.len)
    
    // Compiler will auto-vectorize when possible
    bestie (i drip = 0; i < a.len; i = i + 1) {
        result[i] = a[i] + b[i]
    }
    
    damn result
}
```

#### Threading (where supported)

```cursed
// Web Workers integration for multi-threading
yeet "threadz"

slay parallel_processing(data []tea) {
    sus workers []threadz.Worker = threadz.create_workers(4)
    defer threadz.destroy_workers(workers)
    
    sus results []drip = threadz.parallel_map(workers, data, process_item)
    damn results
}
```

### Bundle Size Optimization

1. **Use Tree Shaking**: Only import needed modules
2. **Minimize Exports**: Export only necessary functions
3. **Optimize Strings**: Use string interning for repeated strings
4. **Remove Debug Info**: Use release builds for production

```bash
# Measure bundle size
./build_wasm.sh browser --optimize
wc -c build/wasm/cursed-browser.wasm

# Analyze with tools
wasm-objdump -h build/wasm/cursed-browser.wasm
wasm-strip build/wasm/cursed-browser.wasm
```

## Troubleshooting

### Common Issues

#### 1. WASM Module Failed to Load

**Problem**: `WebAssembly.instantiateStreaming()` fails

**Solutions**:
```javascript
// Fallback for older browsers
if (!WebAssembly.instantiateStreaming) {
  const response = await fetch('app.wasm');
  const bytes = await response.arrayBuffer();
  const module = await WebAssembly.instantiate(bytes, imports);
}

// Check MIME type
// Ensure server sends: Content-Type: application/wasm
```

#### 2. Memory Access Violations

**Problem**: Invalid memory access errors

**Solutions**:
```cursed
// Check array bounds
slay safe_array_access(arr []drip, index drip) drip {
    ready (index >= 0 && index < arr.len) {
        damn arr[index]
    } otherwise {
        vibez.spill("Array index out of bounds:", index)
        damn 0
    }
}

// Use memory validation
sus memory_valid lit = validate_memory_access(ptr, size)
ready (!memory_valid) {
    vibez.spill("Invalid memory access attempted")
    damn error.MemoryViolation
}
```

#### 3. Stack Overflow

**Problem**: Call stack exceeded

**Solutions**:
```cursed
// Use iteration instead of recursion
slay fibonacci_iterative(n drip) drip {
    ready (n <= 1) damn n
    
    sus a drip = 0
    sus b drip = 1
    sus i drip = 2
    
    bestie (i <= n) {
        sus c drip = a + b
        a = b
        b = c
        i = i + 1
    }
    
    damn b
}
```

### Debugging Tools

#### Browser DevTools

1. **WebAssembly Tab**: Inspect WASM modules and memory
2. **Console**: View CURSED output and errors
3. **Network**: Check WASM loading times
4. **Performance**: Profile WASM execution

#### Command Line Tools

```bash
# Disassemble WASM
wasm-objdump -d build/wasm/cursed-browser.wasm

# Validate WASM
wasm-validate build/wasm/cursed-browser.wasm

# Profile with wasmtime
wasmtime --profile build/wasm/cursed-wasi.wasm
```

### Performance Profiling

```javascript
// Browser profiling
performance.mark('cursed-start');
cursedModule.exports.main();
performance.mark('cursed-end');
performance.measure('cursed-execution', 'cursed-start', 'cursed-end');

console.log(performance.getEntriesByName('cursed-execution')[0].duration);
```

## Advanced Usage

### Custom WASM Runtime

Create custom runtime environments:

```rust
// Custom Rust runtime
use wasmtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "cursed-app.wasm")?;
    
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    
    // Call CURSED main function
    let main = instance.get_typed_func::<(), i32>(&mut store, "main")?;
    let result = main.call(&mut store, ())?;
    
    println!("CURSED result: {}", result);
    Ok(())
}
```

### WebAssembly Interfaces

Define custom interfaces for CURSED modules:

```wit
// interface.wit (WebAssembly Interface Types)
interface cursed-api {
  enum log-level {
    info,
    warning,
    error
  }
  
  record config {
    debug: bool,
    optimize: bool
  }
  
  log: func(level: log-level, message: string)
  compile: func(source: string, config: config) -> result<string, string>
}
```

### Module Federation

Share CURSED modules across applications:

```javascript
// Module host
const cursedModules = new Map();

async function loadCursedModule(name, url) {
  const module = await WebAssembly.instantiateStreaming(fetch(url));
  cursedModules.set(name, module);
  return module;
}

// Module consumer
const mathModule = cursedModules.get('cursed-math');
const result = mathModule.instance.exports.fibonacci(10);
```

## Deployment Checklist

### Pre-deployment

- [ ] Build with optimizations enabled
- [ ] Test in target environment
- [ ] Validate WASM binary
- [ ] Check bundle size
- [ ] Verify all exports work
- [ ] Test error handling
- [ ] Profile performance

### Browser Deployment

- [ ] Configure proper MIME types
- [ ] Enable HTTPS (required for SharedArrayBuffer)
- [ ] Test Cross-Origin isolation
- [ ] Verify CSP compatibility
- [ ] Test on multiple browsers
- [ ] Implement fallbacks
- [ ] Add loading indicators

### WASI Deployment

- [ ] Test with target runtime
- [ ] Configure file system permissions
- [ ] Set up environment variables
- [ ] Test command-line arguments
- [ ] Verify network access
- [ ] Configure logging
- [ ] Set up monitoring

## Best Practices

### Development

1. **Start Small**: Begin with simple functions before complex applications
2. **Test Early**: Use `./build_wasm.sh test` frequently
3. **Profile Often**: Monitor performance throughout development
4. **Use Types**: Leverage CURSED's type system for safety
5. **Handle Errors**: Implement proper error handling

### Production

1. **Optimize Builds**: Always use optimized builds for production
2. **Monitor Performance**: Set up performance monitoring
3. **Cache Aggressively**: Use proper caching headers for WASM files
4. **Update Regularly**: Keep WASM runtime and toolchain updated
5. **Security First**: Follow WASM security best practices

### Maintenance

1. **Version Management**: Use semantic versioning for WASM modules
2. **Backwards Compatibility**: Maintain API compatibility
3. **Documentation**: Keep API documentation updated
4. **Testing**: Maintain comprehensive test suites
5. **Monitoring**: Set up error tracking and performance monitoring

---

## Conclusion

CURSED's WebAssembly support enables powerful, cross-platform applications that run efficiently in browsers, servers, and embedded systems. With proper optimization and deployment practices, CURSED WASM applications can achieve near-native performance while maintaining the safety and expressiveness of the CURSED language.

For more advanced topics and examples, see the [CURSED WASM Examples Repository](https://github.com/cursed-lang/wasm-examples) and [Performance Guide](WASM_PERFORMANCE.md).

---

*This guide is part of the CURSED Programming Language documentation. For updates and community support, visit [cursedlang.org](https://cursedlang.org).*
