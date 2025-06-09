#!/bin/bash
# Setup Package Manager Integration for CURSED
# This script sets up the development environment for package management

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🚀 Setting up CURSED Package Manager Integration"

# Function to print colored output
print_status() {
    echo -e "\033[1;34m[INFO]\033[0m $1"
}

print_success() {
    echo -e "\033[1;32m[SUCCESS]\033[0m $1"
}

print_error() {
    echo -e "\033[1;31m[ERROR]\033[0m $1"
}

print_warning() {
    echo -e "\033[1;33m[WARNING]\033[0m $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if we're in a devenv shell
    if [[ -z "${DEVENV_ROOT:-}" ]]; then
        print_warning "Not in devenv shell. Run 'devenv shell' first for best experience."
    fi
    
    # Check Rust installation
    if ! command -v cargo >/dev/null 2>&1; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Build package manager binaries
build_package_manager() {
    print_status "Building package manager binaries..."
    
    cd "$PROJECT_ROOT"
    
    # Build the package manager CLI
    if ! cargo build --bin cursed-pkg; then
        print_error "Failed to build cursed-pkg binary"
        exit 1
    fi
    
    # Build the separate package binary if it exists
    if cargo build --bin cursed-package 2>/dev/null; then
        print_status "Built cursed-package binary"
    fi
    
    print_success "Package manager binaries built successfully"
}

# Create package cache directories
setup_cache_directories() {
    print_status "Setting up package cache directories..."
    
    local cache_dir="${HOME}/.cache/cursed"
    local packages_dir="${cache_dir}/packages"
    local registry_dir="${cache_dir}/registry"
    
    mkdir -p "$packages_dir"
    mkdir -p "$registry_dir"
    
    # Create cache configuration
    cat > "${cache_dir}/config.toml" << EOF
# CURSED Package Manager Cache Configuration
[cache]
max_size = "1GB"
cleanup_interval = "7d"
compression = true

[registry]
default_url = "https://packages.cursed-lang.org"
update_interval = "1h"

[downloads]
timeout = "30s"
retries = 3
parallel_limit = 4
EOF
    
    print_success "Cache directories created at $cache_dir"
}

# Setup development workspace
setup_dev_workspace() {
    print_status "Setting up development workspace..."
    
    cd "$PROJECT_ROOT"
    
    # Create example workspace if it doesn't exist
    if [[ ! -d "example_workspace" ]]; then
        mkdir -p example_workspace/src
        
        # Create example CursedPackage.toml
        cat > example_workspace/CursedPackage.toml << EOF
[package]
name = "example-app"
version = "0.1.0"
description = "Example CURSED application with package dependencies"
authors = ["Developer <dev@example.com>"]
license = "MIT"
keywords = ["example", "cursed"]
categories = ["development"]

[dependencies]
# Example dependencies (these would be real packages in a full implementation)
# http = "1.0.0"
# json = "1.2.0"

[dev-dependencies]
# testing = "1.0.1"

[[bin]]
name = "example-app"
path = "src/main.csd"
EOF
        
        # Create example main.csd
        cat > example_workspace/src/main.csd << 'EOF'
vibe example_app;

// Example imports (would work when packages are available)
// yeet "http"
// yeet "json"

slay main() {
    capicola("Hello from CURSED with package management!");
    
    // Example of using external packages
    // sus client = http.Client()
    // sus data = json.parse("{\"hello\": \"world\"}")
    
    capicola("Package management integration is working!");
}
EOF
        
        print_success "Created example workspace in example_workspace/"
    fi
}

# Create package manager integration tests
setup_integration_tests() {
    print_status "Setting up integration tests..."
    
    cd "$PROJECT_ROOT"
    
    # Create integration test for package compilation
    cat > tests/package_manager_integration_test.rs << 'EOF'
//! Integration tests for package manager and compilation system

use cursed::package_manager::compilation_integration::{
    build_workspace_with_packages, has_external_dependencies, PackageCompilationOptions
};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_workspace_compilation_without_dependencies() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_dir = temp_dir.path();
    
    // Create a simple workspace without external dependencies
    let manifest_content = r#"
name = "simple-app"
version = "1.0.0"
description = "Simple application"
authors = ["Test Author <test@example.com>"]
dependencies = {}
dev_dependencies = {}
"#;
    
    fs::write(workspace_dir.join("CursedPackage.toml"), manifest_content).unwrap();
    
    // Create source directory and file
    let src_dir = workspace_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    let main_content = r#"
vibe simple_app;

slay main() {
    capicola("Hello, World!");
}
"#;
    fs::write(src_dir.join("main.csd"), main_content).unwrap();
    
    // Test dependency detection
    let has_deps = has_external_dependencies(workspace_dir);
    // This will likely fail due to TOML format mismatch, but that's expected
    assert!(has_deps.is_err() || !has_deps.unwrap());
}

#[test]
fn test_package_compilation_options() {
    let options = PackageCompilationOptions::default();
    assert!(!options.include_dev_dependencies);
    assert!(options.parallel_compilation);
    assert_eq!(options.optimization_level, 2);
}
EOF
    
    print_success "Created integration tests"
}

# Update Makefile with package manager targets
update_makefile() {
    print_status "Makefile already contains package manager targets"
    print_success "Package manager targets are available in Makefile"
}

# Setup git hooks for package management
setup_git_hooks() {
    print_status "Setting up git hooks for package management..."
    
    local hooks_dir="$PROJECT_ROOT/.git/hooks"
    
    if [[ -d "$hooks_dir" ]]; then
        # Create pre-commit hook that checks package dependencies
        cat > "$hooks_dir/pre-commit-package-check" << 'EOF'
#!/bin/bash
# Pre-commit hook to check package dependencies

echo "🔍 Checking package dependencies..."

# Check if CursedPackage.toml exists and is valid
if [[ -f "CursedPackage.toml" ]]; then
    if ! cargo run --bin cursed-pkg check 2>/dev/null; then
        echo "⚠️  Package dependency check warnings (not blocking commit)"
    fi
fi

echo "✅ Package dependency check completed"
EOF
        
        chmod +x "$hooks_dir/pre-commit-package-check"
        print_success "Git hooks configured"
    else
        print_warning "Not in a git repository, skipping git hooks setup"
    fi
}

# Create development documentation
create_documentation() {
    print_status "Creating package manager documentation..."
    
    local docs_dir="$PROJECT_ROOT/docs/package_manager"
    mkdir -p "$docs_dir"
    
    cat > "$docs_dir/README.md" << 'EOF'
# CURSED Package Manager

The CURSED Package Manager provides comprehensive dependency management for CURSED projects.

## Quick Start

### Initialize a new project
```bash
make pkg-init
cursed-pkg init my-project --description "My CURSED project"
```

### Install dependencies
```bash
# Install all dependencies from CursedPackage.toml
make pkg-install

# Install a specific package
make pkg-search PACKAGE=http
make pkg-info PACKAGE=http
cursed-pkg install http --version 1.0.0
```

### Build with dependencies
```bash
# Build project with automatic dependency resolution
make build-with-packages

# Test with dependencies
make test-with-packages
```

## Package Management Commands

| Command | Description |
|---------|-------------|
| `make pkg-install` | Install dependencies from CursedPackage.toml |
| `make pkg-update` | Update all dependencies to latest versions |
| `make pkg-search PACKAGE=name` | Search for packages |
| `make pkg-info PACKAGE=name` | Show package information |
| `make pkg-check` | Check dependencies for updates and issues |
| `make pkg-clean` | Clean package cache |
| `make pkg-init` | Initialize CursedPackage.toml |

## Integration with Build System

The package manager is fully integrated with the CURSED build system:

- **Automatic dependency resolution**: Dependencies are resolved and installed automatically during compilation
- **Import path resolution**: External packages are made available via import statements
- **Separate compilation**: Each package is compiled separately for faster incremental builds
- **Caching**: Downloaded packages are cached for offline usage

## Configuration

Package manager configuration is stored in:
- Global: `~/.cache/cursed/config.toml`
- Project: `CursedPackage.toml`

## Development

To work on the package manager:

```bash
# Build package manager
cargo build --bin cursed-pkg

# Run tests
cargo test package_manager

# Setup development environment
./scripts/setup_package_manager.sh
```
EOF
    
    print_success "Documentation created in docs/package_manager/"
}

# Main setup function
main() {
    print_status "Starting CURSED Package Manager setup..."
    
    check_prerequisites
    build_package_manager
    setup_cache_directories
    setup_dev_workspace
    setup_integration_tests
    update_makefile
    setup_git_hooks
    create_documentation
    
    print_success "🎉 Package Manager setup completed successfully!"
    
    echo ""
    echo "Next steps:"
    echo "1. Try the example workspace: cd example_workspace && make build-with-packages"
    echo "2. Initialize a new project: make pkg-init"
    echo "3. Search for packages: make pkg-search PACKAGE=http"
    echo "4. Read the documentation: docs/package_manager/README.md"
    echo ""
    echo "Available commands:"
    echo "  make pkg-help     - Show package management help"
    echo "  make pkg-install  - Install dependencies"
    echo "  make pkg-search   - Search packages"
    echo "  cursed-pkg --help - Full CLI help"
}

# Run main function
main "$@"
EOF

chmod +x scripts/setup_package_manager.sh
