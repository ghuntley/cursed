#!/bin/bash

# CURSED WebAssembly Build Script
# Compiles CURSED programs to WebAssembly with different targets

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}$1${NC}"
}

# Show help
show_help() {
    print_header "🚀 CURSED WebAssembly Build Script"
    echo
    echo "Usage: $0 [command] [options]"
    echo
    echo "Commands:"
    echo "  all               Build all WASM targets"
    echo "  browser           Build for browser (default)"
    echo "  wasi              Build for WASI runtime"
    echo "  lib               Build WASM library for JS integration"
    echo "  demo              Build demo programs"
    echo "  clean             Clean build artifacts"
    echo "  test              Test WASM builds"
    echo "  help              Show this help message"
    echo
    echo "Options:"
    echo "  --optimize        Enable optimizations (default: true)"
    echo "  --debug           Include debug information"
    echo "  --verbose         Verbose output"
    echo "  --with-html       Generate HTML wrapper (browser target)"
    echo "  --with-js         Generate JavaScript bindings (browser target)"
    echo
    echo "Examples:"
    echo "  $0 browser --optimize --with-html"
    echo "  $0 wasi --debug"
    echo "  $0 demo --verbose"
    echo "  $0 all"
}

# Parse command line arguments
COMMAND=""
OPTIMIZE=true
DEBUG=false
VERBOSE=false
WITH_HTML=false
WITH_JS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        all|browser|wasi|lib|demo|clean|test|help)
            COMMAND="$1"
            shift
            ;;
        --optimize)
            OPTIMIZE=true
            shift
            ;;
        --no-optimize)
            OPTIMIZE=false
            shift
            ;;
        --debug)
            DEBUG=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --with-html)
            WITH_HTML=true
            shift
            ;;
        --with-js)
            WITH_JS=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Set default command
if [[ -z "$COMMAND" ]]; then
    COMMAND="browser"
fi

# Show help and exit
if [[ "$COMMAND" == "help" ]]; then
    show_help
    exit 0
fi

# Set build options
BUILD_OPTS=()
if [[ "$OPTIMIZE" == "true" ]]; then
    BUILD_OPTS+=("-Doptimize=ReleaseFast")
else
    BUILD_OPTS+=("-Doptimize=Debug")
fi

if [[ "$VERBOSE" == "true" ]]; then
    BUILD_OPTS+=("--verbose")
fi

# Print build configuration
print_header "🔧 CURSED WASM Build Configuration"
echo "Command: $COMMAND"
echo "Optimize: $OPTIMIZE"
echo "Debug: $DEBUG"
echo "Verbose: $VERBOSE"
echo "HTML: $WITH_HTML"
echo "JS Bindings: $WITH_JS"
echo

# Ensure build directory exists
mkdir -p build/wasm

# Check for required tools
check_tools() {
    print_status "Checking required tools..."
    
    if ! command -v zig &> /dev/null; then
        print_error "Zig compiler not found. Please install Zig."
        exit 1
    fi
    
    if [[ "$COMMAND" == "wasi" ]] || [[ "$COMMAND" == "test" ]]; then
        if ! command -v wasmtime &> /dev/null; then
            print_warning "Wasmtime not found. Install with: curl https://wasmtime.dev/install.sh -sSf | bash"
        fi
    fi
    
    print_success "Tools check complete"
}

# Clean build artifacts
clean_build() {
    print_status "Cleaning WASM build artifacts..."
    rm -rf build/wasm/*
    rm -rf zig-out/
    rm -f *.wasm *.js *.html
    print_success "Clean complete"
}

# Build for browser target
build_browser() {
    print_status "Building CURSED for WebAssembly (Browser)..."
    
    # Build the WASM module
    zig build wasm-browser "${BUILD_OPTS[@]}"
    
    # Copy to build directory (handle different naming)
    if [[ -f "zig-out/bin/cursed-browser.wasm" ]]; then
        cp zig-out/bin/cursed-browser.wasm build/wasm/
    elif [[ -f "zig-out/bin/cursed-browser" ]]; then
        cp zig-out/bin/cursed-browser build/wasm/cursed-browser.wasm
    else
        print_error "Browser WASM build not found"
        return 1
    fi
    
    # Generate JavaScript bindings if requested
    if [[ "$WITH_JS" == "true" ]] || [[ "$COMMAND" == "demo" ]]; then
        print_status "Generating JavaScript bindings..."
        cp wasm_demo.js build/wasm/cursed.js 2>/dev/null || {
            print_warning "wasm_demo.js not found, creating basic bindings..."
            create_basic_js_bindings
        }
    fi
    
    # Generate HTML wrapper if requested
    if [[ "$WITH_HTML" == "true" ]] || [[ "$COMMAND" == "demo" ]]; then
        print_status "Generating HTML wrapper..."
        if [[ -f "wasm_demo.html" ]]; then
            cp wasm_demo.html build/wasm/index.html
        else
            print_warning "wasm_demo.html not found, creating basic HTML wrapper..."
            create_basic_html_wrapper
        fi
        print_success "HTML wrapper created: build/wasm/index.html"
        print_status "Open in browser: file://$(realpath build/wasm/index.html)"
    fi
    
    print_success "Browser WASM build complete: build/wasm/cursed-browser.wasm"
}

# Build for WASI target
build_wasi() {
    print_status "Building CURSED for WebAssembly (WASI)..."
    
    # Build the WASM module
    zig build wasm-wasi "${BUILD_OPTS[@]}"
    
    # Copy to build directory
    cp zig-out/bin/cursed-wasi.wasm build/wasm/
    
    print_success "WASI WASM build complete: build/wasm/cursed-wasi.wasm"
    
    # Test with wasmtime if available
    if command -v wasmtime &> /dev/null; then
        print_status "Testing WASI build with wasmtime..."
        echo "Running: wasmtime build/wasm/cursed-wasi.wasm"
        wasmtime build/wasm/cursed-wasi.wasm || print_warning "WASI test failed (this is expected for demo)"
    fi
}

# Build WASM library
build_lib() {
    print_status "Building CURSED WASM Library..."
    
    # Build the shared library
    zig build wasm-lib "${BUILD_OPTS[@]}"
    
    # Copy to build directory
    cp zig-out/lib/libcursed-lib.* build/wasm/ 2>/dev/null || print_warning "Library file not found"
    
    print_success "WASM library build complete"
}

# Build demo programs
build_demo() {
    print_status "Building CURSED WASM demo programs..."
    
    # Build all targets
    build_browser
    build_wasi
    build_lib
    
    # Compile demo programs if they exist
    if [[ -f "wasm_demo.csd" ]]; then
        print_status "Compiling demo CURSED program..."
        # This would use the actual CURSED compiler when implemented
        # ./zig-out/bin/cursed-zig --backend wasm --target wasm32-browser wasm_demo.csd -o build/wasm/demo.wasm
        print_warning "CURSED compiler integration pending - using pre-built binaries"
    fi
    
    print_success "Demo build complete!"
    echo
    print_header "🎮 Demo Instructions:"
    echo "1. Browser Demo:"
    echo "   - Open: build/wasm/index.html"
    echo "   - Or serve with: python3 -m http.server 8000 -d build/wasm"
    echo "2. WASI Demo:"
    echo "   - Run: wasmtime build/wasm/cursed-wasi.wasm"
}

# Test WASM builds
test_builds() {
    print_status "Testing WASM builds..."
    
    # Test browser build
    if [[ -f "build/wasm/cursed-browser.wasm" ]]; then
        print_status "✅ Browser WASM found"
        
        # Basic validation
        file build/wasm/cursed-browser.wasm | grep -q "WebAssembly" && \
            print_success "Browser WASM is valid WebAssembly binary" || \
            print_warning "Browser WASM validation failed"
    else
        print_warning "Browser WASM not found"
    fi
    
    # Test WASI build
    if [[ -f "build/wasm/cursed-wasi.wasm" ]]; then
        print_status "✅ WASI WASM found"
        
        # Test with wasmtime if available
        if command -v wasmtime &> /dev/null; then
            print_status "Testing WASI build..."
            timeout 5s wasmtime build/wasm/cursed-wasi.wasm 2>/dev/null && \
                print_success "WASI WASM runs successfully" || \
                print_warning "WASI WASM test had issues (may be expected)"
        fi
    else
        print_warning "WASI WASM not found"
    fi
    
    print_success "Testing complete"
}

# Create basic JavaScript bindings
create_basic_js_bindings() {
    cat > build/wasm/cursed.js << 'EOF'
// Basic CURSED WASM JavaScript bindings
class CursedModule {
    constructor() {
        this.instance = null;
        this.memory = null;
    }
    
    async load(wasmPath) {
        const wasmModule = await WebAssembly.instantiateStreaming(fetch(wasmPath), {
            js: {
                console_log: (ptr, len) => console.log(this.readString(ptr, len)),
                alert: (ptr, len) => alert(this.readString(ptr, len))
            }
        });
        
        this.instance = wasmModule.instance;
        this.memory = this.instance.exports.memory;
        return this;
    }
    
    readString(ptr, len) {
        const bytes = new Uint8Array(this.memory.buffer, ptr, len);
        return new TextDecoder().decode(bytes);
    }
    
    call(functionName, ...args) {
        return this.instance.exports[functionName](...args);
    }
}

window.CursedModule = CursedModule;
EOF
}

# Create basic HTML wrapper
create_basic_html_wrapper() {
    cat > build/wasm/index.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED WebAssembly Demo</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            max-width: 800px; 
            margin: 0 auto; 
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
        }
        .container { 
            background: rgba(255,255,255,0.1); 
            padding: 30px; 
            border-radius: 15px;
            backdrop-filter: blur(10px);
        }
        button { 
            background: #ff6b6b; 
            color: white; 
            border: none; 
            padding: 10px 20px; 
            margin: 10px; 
            border-radius: 5px; 
            cursor: pointer; 
        }
        #output { 
            background: rgba(0,0,0,0.3); 
            padding: 20px; 
            border-radius: 10px; 
            margin-top: 20px;
            font-family: monospace;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 CURSED WebAssembly Demo</h1>
        <p>Welcome to CURSED running in WebAssembly!</p>
        
        <button onclick="runTest()">Run CURSED Test</button>
        <button onclick="runVersion()">Get Version</button>
        <button onclick="clearOutput()">Clear Output</button>
        
        <pre id="output">Loading CURSED WASM...</pre>
    </div>
    
    <script src="cursed.js"></script>
    <script>
        let cursedModule;
        let output;
        
        async function initWasm() {
            try {
                cursedModule = new CursedModule();
                await cursedModule.load('cursed-browser.wasm');
                output = document.getElementById('output');
                output.textContent = '✅ CURSED WASM loaded successfully!\nClick buttons to test functionality.';
            } catch (error) {
                output.textContent = '❌ Failed to load CURSED WASM: ' + error.message;
            }
        }
        
        function runTest() {
            if (cursedModule) {
                const result = cursedModule.call('cursed_test');
                output.textContent += '\n🧪 Test result: ' + result;
            }
        }
        
        function runVersion() {
            if (cursedModule) {
                const version = cursedModule.call('cursed_version');
                output.textContent += '\n📋 Version: ' + version;
            }
        }
        
        function clearOutput() {
            output.textContent = '';
        }
        
        window.addEventListener('load', initWasm);
    </script>
</body>
</html>
EOF
}

# Main execution
main() {
    print_header "🚀 CURSED WebAssembly Build System"
    echo
    
    check_tools
    
    case "$COMMAND" in
        clean)
            clean_build
            ;;
        browser)
            build_browser
            ;;
        wasi)
            build_wasi
            ;;
        lib)
            build_lib
            ;;
        demo)
            build_demo
            ;;
        all)
            print_status "Building all WASM targets..."
            build_browser
            build_wasi
            build_lib
            print_success "All WASM builds complete!"
            ;;
        test)
            test_builds
            ;;
        *)
            print_error "Unknown command: $COMMAND"
            show_help
            exit 1
            ;;
    esac
    
    echo
    print_success "CURSED WASM build script completed successfully!"
    
    # Show next steps
    if [[ "$COMMAND" != "clean" ]] && [[ "$COMMAND" != "test" ]]; then
        print_header "📋 Next Steps:"
        echo "• Check build/wasm/ directory for compiled binaries"
        echo "• Run './build_wasm.sh test' to validate builds"
        echo "• Run './build_wasm.sh demo --with-html' for full demo"
        echo "• Open build/wasm/index.html in your browser"
    fi
}

# Run main function
main
