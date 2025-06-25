# CURSED Package Manager Integration Demo

This document demonstrates how the integrated package manager works with CURSED's build system and compilation pipeline.

## Overview

The CURSED Package Manager provides:

1. **Package Discovery**: Search and find packages in the registry
2. **Dependency Resolution**: Automatically resolve and install dependencies
3. **Build Integration**: Seamless integration with the compilation pipeline
4. **Cache Management**: Efficient caching of downloaded packages
5. **Import Resolution**: Automatic import path resolution for external packages

## Basic Usage

### 1. Initialize a New Project

```bash
# Create a new CURSED project with package management
make pkg-init
cursed-pkg init my-web-app --description "A web application in CURSED"
cd my-web-app
```

This creates:
- `CursedPackage.toml` - Package manifest
- `src/main.csd` - Main source file
- Basic project structure

### 2. Add Dependencies

Search for available packages:

```bash
make pkg-search PACKAGE=http
make pkg-info PACKAGE=http
```

Add dependencies to `CursedPackage.toml`:

```toml
[package]
name = "my-web-app"
version = "0.1.0"
description = "A web application in CURSED"
authors = ["Your Name <your.email@example.com>"]

[dependencies]
http = "1.0.0"
json = "1.2.0"
crypto = "0.5.0"

[dev-dependencies]
testing = "1.0.1"
```

### 3. Install Dependencies

```bash
# Install all dependencies
make pkg-install

# Or install specific packages
cursed-pkg install http --version 1.0.0
```

### 4. Build with Dependencies

```bash
# Build project with automatic dependency resolution
make build-with-packages

# Test with dependencies
make test-with-packages
```

## Advanced Integration Examples

### 1. Using External Packages in Code

Once dependencies are installed, use them in your CURSED code:

```cursed
vibe my_web_app;

// Import external packages
yeet "http"
yeet "json"
yeet "crypto"

slay main() {
    // Create HTTP client
    sus client = http.Client();
    
    // Make a request
    sus response = client.get("https://api.example.com/data");
    
    // Parse JSON response
    sus data = json.parse(response.body);
    
    // Hash some data
    sus hash = crypto.sha256("important data");
    
    capicola("Server response: {}", data);
    capicola("Data hash: {}", hash);
}
```

### 2. Workspace with Multiple Packages

Create a workspace with multiple CURSED packages:

```
my-workspace/
├── CursedPackage.toml
├── web-server/
│   ├── src/main.csd
│   └── CursedPackage.toml
├── shared-utils/
│   ├── src/lib.csd
│   └── CursedPackage.toml
└── cli-tool/
    ├── src/main.csd
    └── CursedPackage.toml
```

The main `CursedPackage.toml` can specify workspace members:

```toml
[workspace]
members = ["web-server", "shared-utils", "cli-tool"]

[workspace.dependencies]
http = "1.0.0"
json = "1.2.0"
```

### 3. Development Dependencies

Use different dependencies for development and production:

```toml
[dependencies]
http = "1.0.0"
json = "1.2.0"

[dev-dependencies]
testing = "1.0.1"
mock-server = "0.3.0"
benchmark = "2.1.0"
```

Build with development dependencies:

```bash
make build-with-packages
# Or explicitly include dev dependencies
cursed-pkg install --dev
```

## Compilation Pipeline Integration

### How It Works

1. **Dependency Analysis**: The compiler reads `CursedPackage.toml` and identifies required packages
2. **Package Resolution**: Dependencies are resolved and downloaded if not cached
3. **Import Path Setup**: Package sources are made available for import resolution
4. **Separate Compilation**: Each package is compiled separately in dependency order
5. **Linking**: All compiled modules are linked together

### Configuration

Configure compilation behavior in `CursedPackage.toml`:

```toml
[package]
name = "my-app"
version = "1.0.0"

[build]
optimization-level = 2
parallel-compilation = true
include-dev-dependencies = false

[dependencies]
http = { version = "1.0.0", features = ["tls", "json"] }
database = { version = "2.1.0", optional = true }
```

### Custom Build Scripts

For complex build requirements, create build scripts:

```bash
#!/bin/bash
# build.sh - Custom build script with package management

set -e

echo "🔧 Custom build with package management"

# Install dependencies
make pkg-install

# Run pre-build tasks
echo "Running pre-build tasks..."

# Custom compilation with specific options
cursed-pkg build --optimization-level 3 --features production

# Post-build tasks
echo "Running post-build tasks..."
```

## Cache Management

### Cache Structure

```
~/.cache/cursed/
├── packages/
│   ├── http/
│   │   ├── 1.0.0/
│   │   └── 1.1.0/
│   └── json/
│       └── 1.2.0/
├── registry/
│   └── index.json
└── config.toml
```

### Cache Commands

```bash
# Check cache status
cursed-pkg cache info

# Clean cache
make pkg-clean

# Clear specific package
cursed-pkg cache remove http

# Update registry index
make pkg-update
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build with Package Manager

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup CURSED environment
      run: |
        # Install CURSED and package manager
        curl -sSf https://install.cursed-lang.org | sh
        source ~/.cursed/env
    
    - name: Cache packages
      uses: actions/cache@v3
      with:
        path: ~/.cache/cursed
        key: cursed-packages-${{ hashFiles('**/CursedPackage.toml') }}
    
    - name: Install dependencies
      run: make pkg-install
    
    - name: Build with packages
      run: make build-with-packages
    
    - name: Test with packages
      run: make test-with-packages
```

### Docker Integration

```dockerfile
# Dockerfile with package manager
FROM cursed:latest

WORKDIR /app

# Copy package manifest
COPY CursedPackage.toml .

# Install dependencies
RUN cursed-pkg install

# Copy source code
COPY src/ src/

# Build application
RUN make build-with-packages

CMD ["./target/my-app"]
```

## Performance Considerations

### Parallel Compilation

The package manager supports parallel compilation:

```bash
# Enable parallel compilation (default)
cursed-pkg build --parallel

# Control parallelism
cursed-pkg build --jobs 4
```

### Incremental Builds

Only recompile changed packages:

```bash
# Incremental build (default)
make build-with-packages

# Force full rebuild
cursed-pkg build --clean
```

### Caching Strategies

- **Local Cache**: Packages cached locally for offline usage
- **Registry Cache**: Registry index cached and updated incrementally
- **Build Cache**: Compiled artifacts cached for faster rebuilds

## Troubleshooting

### Common Issues

1. **Dependency Conflicts**:
   ```bash
   make pkg-check --conflicts
   cursed-pkg resolve --verbose
   ```

2. **Cache Issues**:
   ```bash
   make pkg-clean
   cursed-pkg cache verify
   ```

3. **Build Failures**:
   ```bash
   cursed-pkg build --verbose --no-cache
   ```

### Debug Mode

Enable verbose logging:

```bash
CURSED_LOG=debug make build-with-packages
cursed-pkg --verbose install http
```

## Future Enhancements

The package manager is designed to support future features:

- **Private Registries**: Support for private package registries
- **Package Publishing**: Publish packages to registries
- **Security Auditing**: Vulnerability scanning for dependencies
- **Workspace Management**: Enhanced multi-package workspace support
- **Build Scripts**: Custom build and test scripts
- **Platform Targets**: Cross-compilation for different platforms

## Examples Repository

See the `examples/package_manager/` directory for complete working examples:

- `simple-app/` - Basic application with external dependencies
- `web-server/` - Web server using HTTP and JSON packages
- `workspace/` - Multi-package workspace example
- `library/` - Reusable library package
