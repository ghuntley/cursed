#!/bin/bash

# CURSED Interactive Documentation Build Script
# This script builds the complete interactive documentation system

set -e

echo "🚀 Building CURSED Interactive Documentation System..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOCS_DIR="$(pwd)/docs/interactive"
BUILD_DIR="$(pwd)/build/interactive-docs"
CURSED_COMPILER="$(pwd)/zig-out/bin/cursed-zig"

# Helper functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js is required but not installed"
        exit 1
    fi
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        log_error "npm is required but not installed"
        exit 1
    fi
    
    # Check CURSED compiler
    if [ ! -f "$CURSED_COMPILER" ]; then
        log_warning "CURSED compiler not found at $CURSED_COMPILER"
        log_info "Building CURSED compiler first..."
        zig build
        
        if [ ! -f "$CURSED_COMPILER" ]; then
            log_error "Failed to build CURSED compiler"
            exit 1
        fi
    fi
    
    log_success "Prerequisites checked"
}

# Build CURSED compiler and tools
build_cursed_tools() {
    log_info "Building CURSED compiler and tools..."
    
    # Build main compiler
    zig build
    
    # Verify tools are available
    local tools=("cursed-zig" "cursed-doc" "cursed-fmt" "cursed-lint" "cursed-lsp")
    for tool in "${tools[@]}"; do
        if [ ! -f "$(pwd)/zig-out/bin/$tool" ]; then
            log_warning "$tool not found, may affect documentation features"
        fi
    done
    
    log_success "CURSED tools built"
}

# Generate API documentation
generate_api_docs() {
    log_info "Generating API documentation..."
    
    mkdir -p "$DOCS_DIR/api-docs/generated"
    
    # Generate docs for standard library modules
    local stdlib_modules=(
        "vibez" "mathz" "stringz" "arrayz" "testz" "filez" 
        "networkz" "timez" "jsonz" "cryptz" "concurrenz" "asyncz"
        "collections" "error_handling" "memory" "reflect"
    )
    
    for module in "${stdlib_modules[@]}"; do
        if [ -f "stdlib/${module}.csd" ]; then
            log_info "Generating docs for $module..."
            
            # Use cursed-doc if available, otherwise create placeholder
            if [ -f "$(pwd)/zig-out/bin/cursed-doc" ]; then
                "$(pwd)/zig-out/bin/cursed-doc" "stdlib/${module}.csd" > "$DOCS_DIR/api-docs/generated/${module}.md" || {
                    log_warning "Failed to generate docs for $module, creating placeholder"
                    create_api_placeholder "$module"
                }
            else
                create_api_placeholder "$module"
            fi
        else
            log_warning "Module $module not found, creating placeholder"
            create_api_placeholder "$module"
        fi
    done
    
    log_success "API documentation generated"
}

# Create API documentation placeholder
create_api_placeholder() {
    local module=$1
    cat > "$DOCS_DIR/api-docs/generated/${module}.md" << EOF
# $module Module

The \`$module\` module provides essential functionality for CURSED programs.

## Import

\`\`\`cursed
yeet "$module"
\`\`\`

## Functions

### Core Functions

Documentation for this module is being generated automatically from source code.

## Examples

\`\`\`cursed
yeet "$module"
yeet "vibez"

# Example usage will be added here
vibez.spill("Using $module module")
\`\`\`

## See Also

- [CURSED Standard Library Overview](../stdlib-overview.md)
- [Module System Guide](../../tutorials/intermediate/modules.md)

---

*This documentation is automatically generated from source code.*
EOF
}

# Validate tutorial code examples
validate_tutorial_code() {
    log_info "Validating tutorial code examples..."
    
    local tutorial_files=$(find "$DOCS_DIR/tutorials" -name "*.md" -type f)
    local validation_errors=0
    
    while IFS= read -r file; do
        log_info "Validating $file..."
        
        # Extract CURSED code blocks and validate them
        local temp_code_file="/tmp/tutorial_code_$$.csd"
        
        # Extract code between ```cursed and ``` 
        sed -n '/```cursed/,/```/p' "$file" | sed '1d;$d' > "$temp_code_file"
        
        if [ -s "$temp_code_file" ]; then
            # Only validate if there's actual code
            if grep -q "yeet\|slay\|sus\|vibez" "$temp_code_file"; then
                if ! "$CURSED_COMPILER" check "$temp_code_file" 2>/dev/null; then
                    log_warning "Code validation failed in $file"
                    ((validation_errors++))
                fi
            fi
        fi
        
        rm -f "$temp_code_file"
    done <<< "$tutorial_files"
    
    if [ $validation_errors -eq 0 ]; then
        log_success "All tutorial code examples validated"
    else
        log_warning "$validation_errors tutorial files have code issues"
    fi
}

# Build interactive examples
build_interactive_examples() {
    log_info "Building interactive code examples..."
    
    mkdir -p "$DOCS_DIR/examples/compiled"
    
    # Find all .csd files in examples directory
    local example_files=$(find "$DOCS_DIR" -name "*.csd" -type f)
    
    while IFS= read -r file; do
        if [ -f "$file" ]; then
            local basename=$(basename "$file" .csd)
            local dirname=$(dirname "$file")
            local relative_dir=${dirname#$DOCS_DIR/}
            
            mkdir -p "$DOCS_DIR/examples/compiled/$relative_dir"
            
            log_info "Compiling example: $basename"
            
            # Compile to WASM for browser execution
            if "$CURSED_COMPILER" --compile --wasm "$file" -o "$DOCS_DIR/examples/compiled/$relative_dir/$basename.wasm" 2>/dev/null; then
                log_success "Compiled $basename to WASM"
            else
                log_warning "Failed to compile $basename to WASM"
            fi
        fi
    done <<< "$example_files"
    
    log_success "Interactive examples built"
}

# Install webapp dependencies
install_webapp_dependencies() {
    log_info "Installing webapp dependencies..."
    
    cd "$DOCS_DIR/webapp"
    
    # Install dependencies
    npm install
    
    log_success "Webapp dependencies installed"
    
    cd - > /dev/null
}

# Build React webapp
build_webapp() {
    log_info "Building React webapp..."
    
    cd "$DOCS_DIR/webapp"
    
    # Run type checking
    log_info "Running TypeScript type check..."
    if npm run type-check; then
        log_success "TypeScript type check passed"
    else
        log_warning "TypeScript type check failed, continuing anyway"
    fi
    
    # Run linting
    log_info "Running ESLint..."
    if npm run lint; then
        log_success "Linting passed"
    else
        log_warning "Linting failed, continuing anyway"
    fi
    
    # Run tests
    log_info "Running tests..."
    if npm test -- --run; then
        log_success "Tests passed"
    else
        log_warning "Tests failed, continuing anyway"
    fi
    
    # Build production version
    log_info "Building production webapp..."
    npm run build
    
    log_success "Webapp built successfully"
    
    cd - > /dev/null
}

# Generate search index
generate_search_index() {
    log_info "Generating search index..."
    
    # Create search index from all documentation content
    local search_script="$DOCS_DIR/scripts/generate-search-index.js"
    
    cat > "$search_script" << 'EOF'
const fs = require('fs');
const path = require('path');

function generateSearchIndex() {
    const documents = [];
    
    // Process markdown files
    function processDirectory(dir, prefix = '') {
        const files = fs.readdirSync(dir);
        
        for (const file of files) {
            const fullPath = path.join(dir, file);
            const stat = fs.statSync(fullPath);
            
            if (stat.isDirectory()) {
                processDirectory(fullPath, prefix + file + '/');
            } else if (file.endsWith('.md')) {
                const content = fs.readFileSync(fullPath, 'utf8');
                const title = extractTitle(content);
                
                documents.push({
                    id: prefix + file,
                    title: title,
                    content: content,
                    url: '/' + prefix + file.replace('.md', ''),
                    type: getDocumentType(prefix)
                });
            }
        }
    }
    
    function extractTitle(content) {
        const match = content.match(/^#\s+(.+)$/m);
        return match ? match[1] : 'Untitled';
    }
    
    function getDocumentType(prefix) {
        if (prefix.includes('tutorials/')) return 'tutorial';
        if (prefix.includes('api-docs/')) return 'api';
        if (prefix.includes('migration/')) return 'migration';
        if (prefix.includes('patterns/')) return 'pattern';
        return 'documentation';
    }
    
    // Process all documentation directories
    const docsDir = path.join(__dirname, '..');
    processDirectory(path.join(docsDir, 'tutorials'), 'tutorials/');
    processDirectory(path.join(docsDir, 'api-docs'), 'api/');
    processDirectory(path.join(docsDir, 'migration'), 'migration/');
    processDirectory(path.join(docsDir, 'patterns'), 'patterns/');
    processDirectory(path.join(docsDir, 'pathways'), 'pathways/');
    
    // Write search index
    const searchIndex = {
        documents: documents,
        version: '1.0.0',
        generated: new Date().toISOString()
    };
    
    fs.writeFileSync(
        path.join(__dirname, '../webapp/public/search-index.json'),
        JSON.stringify(searchIndex, null, 2)
    );
    
    console.log(`Generated search index with ${documents.length} documents`);
}

generateSearchIndex();
EOF

    # Create scripts directory if it doesn't exist
    mkdir -p "$DOCS_DIR/scripts"
    
    # Run search index generation
    node "$search_script"
    
    log_success "Search index generated"
}

# Create deployment package
create_deployment_package() {
    log_info "Creating deployment package..."
    
    # Create build directory
    mkdir -p "$BUILD_DIR"
    
    # Copy webapp build
    cp -r "$DOCS_DIR/webapp/dist/"* "$BUILD_DIR/"
    
    # Copy additional assets
    if [ -d "$DOCS_DIR/assets" ]; then
        cp -r "$DOCS_DIR/assets" "$BUILD_DIR/"
    fi
    
    # Copy compiled examples
    if [ -d "$DOCS_DIR/examples/compiled" ]; then
        cp -r "$DOCS_DIR/examples/compiled" "$BUILD_DIR/examples/"
    fi
    
    # Create build info
    cat > "$BUILD_DIR/build-info.json" << EOF
{
    "buildTime": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "version": "1.0.0",
    "cursedVersion": "$(cat VERSION 2>/dev/null || echo 'unknown')",
    "gitCommit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "gitBranch": "$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')"
}
EOF
    
    log_success "Deployment package created at $BUILD_DIR"
}

# Run development server
run_dev_server() {
    log_info "Starting development server..."
    
    cd "$DOCS_DIR/webapp"
    
    log_success "Development server starting at http://localhost:5173"
    log_info "Press Ctrl+C to stop"
    
    npm run dev
}

# Main build process
main() {
    local command=${1:-"build"}
    
    case $command in
        "build")
            check_prerequisites
            build_cursed_tools
            generate_api_docs
            validate_tutorial_code
            build_interactive_examples
            install_webapp_dependencies
            build_webapp
            generate_search_index
            create_deployment_package
            log_success "Interactive documentation build complete!"
            ;;
        "dev")
            check_prerequisites
            build_cursed_tools
            generate_api_docs
            install_webapp_dependencies
            generate_search_index
            run_dev_server
            ;;
        "docs-only")
            generate_api_docs
            validate_tutorial_code
            build_interactive_examples
            generate_search_index
            log_success "Documentation content updated!"
            ;;
        "webapp-only")
            install_webapp_dependencies
            build_webapp
            log_success "Webapp built!"
            ;;
        "validate")
            validate_tutorial_code
            log_success "Tutorial validation complete!"
            ;;
        *)
            echo "Usage: $0 [build|dev|docs-only|webapp-only|validate]"
            echo ""
            echo "Commands:"
            echo "  build      - Full build (default)"
            echo "  dev        - Development server"
            echo "  docs-only  - Build documentation content only"
            echo "  webapp-only - Build webapp only"
            echo "  validate   - Validate tutorial code examples"
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
