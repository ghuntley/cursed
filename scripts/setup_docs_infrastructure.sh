#!/bin/bash

# CURSED Documentation Infrastructure Setup Script
#
# This script sets up the complete documentation infrastructure
# including server, registry, and testing environment.

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_DIR="$PROJECT_ROOT/examples/doc_configs"
DOCS_DIR="$PROJECT_ROOT/docs"
REGISTRY_DIR="$PROJECT_ROOT/registry"
LOGS_DIR="$PROJECT_ROOT/logs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
SETUP_SERVER=true
SETUP_REGISTRY=true
SETUP_TESTING=false
PORT=8080
VERBOSE=false

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
CURSED Documentation Infrastructure Setup Script

Usage: $0 [OPTIONS]

OPTIONS:
    --server               Setup documentation server (default: true)
    --no-server           Skip server setup
    --registry            Setup documentation registry (default: true)
    --no-registry         Skip registry setup
    --testing             Setup testing infrastructure (default: false)
    --port PORT           Server port (default: 8080)
    --verbose             Enable verbose output
    -h, --help            Show this help message

EXAMPLES:
    # Setup complete infrastructure
    $0

    # Setup only server on custom port
    $0 --no-registry --port 3000

    # Setup everything including testing
    $0 --testing

    # Setup only registry
    $0 --no-server --registry

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --server)
                SETUP_SERVER=true
                shift
                ;;
            --no-server)
                SETUP_SERVER=false
                shift
                ;;
            --registry)
                SETUP_REGISTRY=true
                shift
                ;;
            --no-registry)
                SETUP_REGISTRY=false
                shift
                ;;
            --testing)
                SETUP_TESTING=true
                shift
                ;;
            --port)
                PORT="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Validate requirements
validate_requirements() {
    log_info "Validating setup requirements..."

    # Check if cursed CLI is available
    if ! command -v cursed &> /dev/null; then
        log_error "CURSED CLI not found. Please install CURSED first."
        exit 1
    fi

    # Check if project directory exists
    if [[ ! -d "$PROJECT_ROOT" ]]; then
        log_error "Project root directory not found: $PROJECT_ROOT"
        exit 1
    fi

    # Check write permissions
    if [[ ! -w "$PROJECT_ROOT" ]]; then
        log_error "No write permission for project directory: $PROJECT_ROOT"
        exit 1
    fi

    log_success "Requirements validation passed"
}

# Create necessary directories
create_directories() {
    log_info "Creating necessary directories..."

    local dirs=(
        "$DOCS_DIR"
        "$REGISTRY_DIR"
        "$LOGS_DIR"
        "$CONFIG_DIR"
    )

    for dir in "${dirs[@]}"; do
        if [[ ! -d "$dir" ]]; then
            mkdir -p "$dir"
            log_info "Created directory: $dir"
        fi
    done

    log_success "Directories created successfully"
}

# Setup documentation registry
setup_registry() {
    if [[ "$SETUP_REGISTRY" != "true" ]]; then
        return 0
    fi

    log_info "Setting up documentation registry..."

    cd "$PROJECT_ROOT"

    # Initialize registry
    if ! cursed doc registry init --data-dir "$REGISTRY_DIR"; then
        log_error "Failed to initialize documentation registry"
        exit 1
    fi

    # Create registry configuration if it doesn't exist
    local registry_config="$REGISTRY_DIR/config.toml"
    if [[ ! -f "$registry_config" ]]; then
        cat > "$registry_config" << EOF
# CURSED Documentation Registry Configuration
data_dir = "$REGISTRY_DIR"
index_file = "$REGISTRY_DIR/index.json"
cache_size = 1000
refresh_interval = 3600
auto_resolve_deps = true
max_dependency_depth = 10
EOF
        log_info "Created registry configuration: $registry_config"
    fi

    log_success "Documentation registry setup completed"
}

# Setup documentation server
setup_server() {
    if [[ "$SETUP_SERVER" != "true" ]]; then
        return 0
    fi

    log_info "Setting up documentation server..."

    # Create server configuration if it doesn't exist
    local server_config="$CONFIG_DIR/server_runtime.toml"
    if [[ ! -f "$server_config" ]]; then
        cat > "$server_config" << EOF
# CURSED Documentation Server Runtime Configuration
bind_address = "127.0.0.1:$PORT"
document_root = "$DOCS_DIR"
enable_https = false

[cors_config]
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "OPTIONS"]
allowed_headers = ["Content-Type", "Authorization"]
allow_credentials = false

[rate_limiting]
requests_per_minute = 60
burst_capacity = 10
enabled = true

[cache_config]
enabled = true
static_cache_duration = 3600
api_cache_duration = 300
max_cache_size = 104857600

[search_config]
enabled = true
max_results = 100
index_refresh_interval = 300
full_text_search = true

[analytics_config]
enabled = true
retention_days = 30
track_page_views = true
track_search_queries = true
track_downloads = true
EOF
        log_info "Created server configuration: $server_config"
    fi

    # Validate server configuration
    if ! cursed doc validate --config "$server_config" --config-type server; then
        log_error "Server configuration validation failed"
        exit 1
    fi

    log_success "Documentation server setup completed"
}

# Setup testing infrastructure
setup_testing() {
    if [[ "$SETUP_TESTING" != "true" ]]; then
        return 0
    fi

    log_info "Setting up testing infrastructure..."

    # Create testing configuration if it doesn't exist
    local testing_config="$CONFIG_DIR/testing_runtime.toml"
    if [[ ! -f "$testing_config" ]]; then
        cat > "$testing_config" << EOF
# CURSED Documentation Testing Runtime Configuration
check_links = true
verify_examples = true
check_completeness = true
check_accessibility = true
request_timeout = 30
max_concurrent_requests = 10
retry_attempts = 3
example_timeout = 60
min_coverage_percentage = 70.0

[link_checking]
enabled = true
follow_redirects = true
check_external_links = true
check_internal_links = true
timeout = 30
max_redirects = 5

[example_verification]
enabled = true
compile_examples = true
run_examples = false
sandbox_execution = true
memory_limit_mb = 256
execution_timeout = 30

[completeness_analysis]
enabled = true
require_function_docs = true
require_struct_docs = true
require_enum_docs = true
require_module_docs = true
min_doc_length = 10

[accessibility_checking]
enabled = true
check_alt_text = true
check_heading_structure = true
check_color_contrast = true
wcag_level = "AA"

[output]
verbose = false
progress_bars = true
colored_output = true
save_report = true
report_format = "html"
EOF
        log_info "Created testing configuration: $testing_config"
    fi

    # Validate testing configuration
    if ! cursed doc validate --config "$testing_config" --config-type testing; then
        log_error "Testing configuration validation failed"
        exit 1
    fi

    log_success "Testing infrastructure setup completed"
}

# Generate sample documentation
generate_sample_docs() {
    log_info "Generating sample documentation..."

    cd "$PROJECT_ROOT"

    # Generate documentation if source exists
    if [[ -d "$PROJECT_ROOT/src" ]]; then
        if ! cursed doc generate --source-dir src --output-dir "$DOCS_DIR"; then
            log_warning "Failed to generate documentation from source"
        else
            log_success "Sample documentation generated"
        fi
    else
        # Create minimal sample documentation
        mkdir -p "$DOCS_DIR"
        cat > "$DOCS_DIR/index.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2rem; }
        .header { background: #f8f9fa; padding: 2rem; border-radius: 8px; }
        .content { margin: 2rem 0; }
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Documentation</h1>
        <p>Welcome to the CURSED programming language documentation!</p>
    </div>
    <div class="content">
        <h2>Getting Started</h2>
        <p>This is a sample documentation page. Use the CURSED CLI to generate real documentation from your source code.</p>
        
        <h3>Commands</h3>
        <ul>
            <li><code>cursed doc generate</code> - Generate documentation</li>
            <li><code>cursed doc serve</code> - Start documentation server</li>
            <li><code>cursed doc test</code> - Test documentation</li>
        </ul>
    </div>
</body>
</html>
EOF
        log_info "Created sample documentation"
    fi
}

# Create startup scripts
create_startup_scripts() {
    log_info "Creating startup scripts..."

    # Create server startup script
    if [[ "$SETUP_SERVER" == "true" ]]; then
        cat > "$PROJECT_ROOT/start_docs_server.sh" << EOF
#!/bin/bash
# Start CURSED Documentation Server

cd "\$(dirname "\$0")"

echo "Starting CURSED documentation server..."
echo "Server will be available at: http://localhost:$PORT"
echo "Press Ctrl+C to stop the server"

cursed doc serve \\
    --bind "127.0.0.1:$PORT" \\
    --document-root "$DOCS_DIR" \\
    --config "$CONFIG_DIR/server_runtime.toml"
EOF
        chmod +x "$PROJECT_ROOT/start_docs_server.sh"
        log_info "Created server startup script: start_docs_server.sh"
    fi

    # Create testing script
    if [[ "$SETUP_TESTING" == "true" ]]; then
        cat > "$PROJECT_ROOT/test_docs.sh" << EOF
#!/bin/bash
# Test CURSED Documentation

cd "\$(dirname "\$0")"

echo "Testing CURSED documentation..."

cursed doc test \\
    --config "$CONFIG_DIR/testing_runtime.toml" \\
    --format html \\
    --output "$LOGS_DIR/test_report.html"

echo "Test report saved to: $LOGS_DIR/test_report.html"
EOF
        chmod +x "$PROJECT_ROOT/test_docs.sh"
        log_info "Created testing script: test_docs.sh"
    fi

    # Create deployment script
    cat > "$PROJECT_ROOT/deploy_docs.sh" << 'EOF'
#!/bin/bash
# Deploy CURSED Documentation

cd "$(dirname "$0")"

echo "Deploying CURSED documentation..."

# Run the main deployment script
./scripts/deploy_docs.sh "$@"
EOF
    chmod +x "$PROJECT_ROOT/deploy_docs.sh"
    log_info "Created deployment script: deploy_docs.sh"
}

# Create monitoring script
create_monitoring_script() {
    log_info "Creating monitoring script..."

    cat > "$PROJECT_ROOT/monitor_docs.sh" << EOF
#!/bin/bash
# Monitor CURSED Documentation Infrastructure

cd "\$(dirname "\$0")"

echo "CURSED Documentation Infrastructure Status"
echo "========================================"

# Check if server is running
if curl -s http://localhost:$PORT/api/health > /dev/null 2>&1; then
    echo "✅ Documentation server is running (port $PORT)"
else
    echo "❌ Documentation server is not responding"
fi

# Check registry status
if [[ -f "$REGISTRY_DIR/index.json" ]]; then
    local package_count=\$(cursed doc registry list --format json 2>/dev/null | jq length 2>/dev/null || echo "0")
    echo "📦 Registry contains \$package_count packages"
else
    echo "❌ Registry not found or not initialized"
fi

# Check documentation directory
if [[ -d "$DOCS_DIR" && -f "$DOCS_DIR/index.html" ]]; then
    echo "📚 Documentation is available"
else
    echo "❌ Documentation not found"
fi

# Check disk usage
local docs_size=\$(du -sh "$DOCS_DIR" 2>/dev/null | cut -f1 || echo "0B")
local registry_size=\$(du -sh "$REGISTRY_DIR" 2>/dev/null | cut -f1 || echo "0B")

echo "💾 Documentation size: \$docs_size"
echo "💾 Registry size: \$registry_size"

# Check log files
if [[ -d "$LOGS_DIR" ]]; then
    local log_count=\$(find "$LOGS_DIR" -name "*.log" | wc -l)
    echo "📝 Log files: \$log_count"
fi

echo ""
echo "URLs:"
echo "  Documentation: http://localhost:$PORT/"
echo "  Server API: http://localhost:$PORT/api/"
echo "  Health check: http://localhost:$PORT/api/health"
EOF
    chmod +x "$PROJECT_ROOT/monitor_docs.sh"
    log_info "Created monitoring script: monitor_docs.sh"
}

# Print setup summary
print_summary() {
    log_success "Documentation infrastructure setup completed!"
    echo ""
    echo "📋 Setup Summary:"
    echo "=================="
    
    if [[ "$SETUP_REGISTRY" == "true" ]]; then
        echo "✅ Documentation registry initialized in: $REGISTRY_DIR"
    fi
    
    if [[ "$SETUP_SERVER" == "true" ]]; then
        echo "✅ Documentation server configured on port: $PORT"
    fi
    
    if [[ "$SETUP_TESTING" == "true" ]]; then
        echo "✅ Testing infrastructure configured"
    fi
    
    echo "✅ Documentation directory: $DOCS_DIR"
    echo "✅ Configuration files: $CONFIG_DIR"
    echo "✅ Startup scripts created"
    echo ""
    echo "🚀 Next Steps:"
    echo "=============="
    
    if [[ "$SETUP_SERVER" == "true" ]]; then
        echo "1. Start the documentation server:"
        echo "   ./start_docs_server.sh"
        echo ""
        echo "2. Open your browser to:"
        echo "   http://localhost:$PORT"
        echo ""
    fi
    
    echo "3. Generate documentation from your source:"
    echo "   cursed doc generate --source-dir src --output-dir docs"
    echo ""
    
    if [[ "$SETUP_TESTING" == "true" ]]; then
        echo "4. Test your documentation:"
        echo "   ./test_docs.sh"
        echo ""
    fi
    
    echo "5. Deploy your documentation:"
    echo "   ./deploy_docs.sh --target local"
    echo ""
    echo "6. Monitor the infrastructure:"
    echo "   ./monitor_docs.sh"
    echo ""
    echo "📖 For more information, see the documentation at:"
    echo "   https://docs.cursed.dev/documentation/"
}

# Main setup workflow
main() {
    log_info "Starting CURSED documentation infrastructure setup"

    parse_args "$@"
    validate_requirements
    create_directories
    setup_registry
    setup_server
    setup_testing
    generate_sample_docs
    create_startup_scripts
    create_monitoring_script
    print_summary
}

# Run main function with all arguments
main "$@"
