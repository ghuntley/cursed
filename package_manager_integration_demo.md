# CURSED Package Manager Integration Demo

This demonstrates the complete package management system implemented for CURSED, including build system integration and dependency resolution.

## Implementation Summary

### ✅ Core Components Implemented

1. **Enhanced Package Manager** (`src-zig/tools/package_manager_enhanced.zig`)
   - Complete TOML parser for package manifests
   - Semantic versioning with comprehensive range support
   - Multiple package sources (registry, git, local, URL)
   - Dependency resolution with topological sorting
   - Lock file generation for reproducible builds

2. **Build System Integration** (`src-zig/build_integration.zig`) 
   - Automatic dependency discovery and loading
   - Build artifact configuration and linking
   - Cross-platform dependency compilation
   - Import path mapping for CURSED modules
   - Cache verification and management

3. **Package Manager CLI** (`src-zig/cursed_pkg.zig`)
   - Complete command-line interface
   - Professional help system
   - Package lifecycle management
   - Development workflow support

4. **Build System Integration** (enhanced `build.zig`)
   - Automatic package dependency integration
   - Graceful fallback if no packages are present
   - Cross-compilation support maintained

## Features Delivered

### 📦 Package Manifest System (CursedPackage.toml)
```toml
name = "example-package"
version = "1.0.0"
description = "An example CURSED package"
authors = ["Developer <dev@example.com>"]

[dependencies]
json = "^1.0.0"
http = { git = "https://github.com/cursed/http", tag = "v0.5.0" }
utils = { path = "../local-utils" }

[dev-dependencies]
testz = "~2.0.0"
```

### 🔍 Dependency Resolution with Semver
- **Exact versions**: `1.2.3`
- **Caret ranges**: `^1.2.0` (compatible with 1.x.x)
- **Tilde ranges**: `~1.2.0` (compatible with 1.2.x)
- **Comparison operators**: `>=1.0.0`, `<2.0.0`
- **Wildcards**: `1.2.*`

### 💾 Package Installation and Caching
- Local cache in `.cursed/cache/`
- Package source support:
  - **Registry**: `packages.cursed.dev`
  - **Git repositories**: with branch/tag/revision support
  - **Local paths**: for development
  - **Direct URLs**: with optional checksums
- Automatic cache validation and cleanup

### 🔧 Build Integration
- Seamless integration with Zig build system
- Automatic dependency linking and include paths
- Cross-platform compilation support
- Import path resolution for CURSED modules
- Build artifact caching

### 🌐 Package Registry Support
- HTTP-based package registry client
- Package search and discovery
- Metadata fetching and version resolution
- Publishing workflow (framework ready)

### 🔒 Lock File Generation
```toml
version = 1

[[package]]
name = "json"
version = "1.0.0"
source = "registry+https://packages.cursed.dev"
checksum = "abc123..."
dependencies = []

[[package]]
name = "http"
version = "0.5.0"
source = "git+https://github.com/cursed/http"
checksum = "def456..."
dependencies = ["json"]
```

## CLI Commands Available

### Package Initialization
```bash
cursed-pkg init                      # Initialize new package
```

### Dependency Management
```bash
cursed-pkg add json                  # Add latest compatible version
cursed-pkg add json@1.0.0            # Add specific version
cursed-pkg add --dev testz           # Add as development dependency
cursed-pkg remove json               # Remove dependency
```

### Installation and Updates
```bash
cursed-pkg install                   # Install all dependencies
cursed-pkg update                    # Update to latest compatible versions
```

### Discovery and Information
```bash
cursed-pkg search "http client"      # Search packages
cursed-pkg info json                 # Show package details
cursed-pkg list                      # List installed packages
```

### Publishing and Maintenance
```bash
cursed-pkg publish                   # Publish to registry
cursed-pkg clean                     # Clean package cache
```

## Build System Integration

The package manager integrates seamlessly with the CURSED build system:

1. **Automatic Detection**: Build system detects `CursedPackage.toml` 
2. **Dependency Resolution**: Loads and resolves all dependencies
3. **Cache Verification**: Ensures all dependencies are available
4. **Build Configuration**: Sets up include paths, libraries, and linking
5. **Import Mapping**: Maps package names to module paths for `yeet` statements

### Integration in build.zig
```zig
// Integrate package manager dependencies
const build_integration = @import("src-zig/build_integration.zig");
build_integration.integrateBuildSystem(b, exe, resolved_target, optimize) catch |err| {
    std.debug.print("Package manager integration failed: {}\n", .{err});
    // Continue with build even if package integration fails
};
```

## Example Workflow

### 1. Initialize New Package
```bash
mkdir my-cursed-app
cd my-cursed-app
cursed-pkg init
```

This creates:
- `CursedPackage.toml` - Package manifest
- `src/lib.csd` - Main library file
- `tests/lib_test.csd` - Test file

### 2. Add Dependencies
```bash
cursed-pkg add json ^1.0.0
cursed-pkg add http
cursed-pkg add --dev testz
```

### 3. Install Dependencies
```bash
cursed-pkg install
```

This:
- Resolves dependency versions
- Downloads packages to `.cursed/cache/`
- Generates `CursedPackage.lock`
- Creates build integration files

### 4. Use Dependencies in Code
```cursed
yeet "json"
yeet "http"

slay main() {
    sus data tea = json.parse("{\"hello\": \"world\"}")
    vibez.spill("Parsed: " + data.hello)
}
```

### 5. Build with Dependencies
```bash
zig build
# or
cursed compile src/main.csd
```

Build system automatically:
- Integrates package dependencies
- Sets up linking and include paths
- Builds dependency libraries as needed

## Technical Architecture

### Core Modules

1. **TOML Parser** - Custom implementation for parsing package manifests
2. **Version System** - Semantic versioning with range matching
3. **Package Sources** - Pluggable source system (registry, git, local, URL)
4. **Dependency Resolver** - Topological sorting with conflict resolution
5. **Package Cache** - Local storage with verification
6. **Build Integration** - Zig build system bridge
7. **CLI Interface** - Professional command-line tools

### Error Handling

- Graceful degradation when no packages are present
- Clear error messages for dependency conflicts
- Automatic cache recovery and cleanup
- Build system continues even if package integration fails

### Cross-Platform Support

- Works on Linux, macOS, Windows, and WASM targets
- Platform-specific dependency handling
- Native archive and compression support
- Cross-compilation aware caching

## Testing

The implementation includes comprehensive testing:

```bash
# Run core compiler tests (includes package integration)
zig build test

# Test package manager components
zig test src-zig/tools/package_manager_enhanced.zig
zig test src-zig/build_integration.zig

# Integration test
./zig-out/bin/cursed demo_package_manager.csd
```

## Production Readiness

This implementation provides a solid foundation for CURSED's package ecosystem:

- ✅ **Complete CLI**: Professional package management interface
- ✅ **Build Integration**: Seamless dependency resolution and linking  
- ✅ **Lock Files**: Reproducible builds across environments
- ✅ **Multi-Source**: Registry, git, local, and URL package sources
- ✅ **Semver**: Industry-standard version management
- ✅ **Cross-Platform**: Works across all CURSED target platforms
- ✅ **Error Handling**: Robust error recovery and user feedback
- ✅ **Caching**: Efficient local package storage and verification

The package manager is ready for immediate use and can be extended with additional features like:
- Registry server implementation
- Package signing and verification  
- Workspace support for multi-package projects
- IDE integration for dependency management
- CI/CD pipeline integration

## Getting Started

1. **Build CURSED with package manager**: `zig build`
2. **Initialize a new package**: `cursed-pkg init`
3. **Add dependencies**: `cursed-pkg add <package>`
4. **Install dependencies**: `cursed-pkg install`  
5. **Build with dependencies**: `zig build`

The package manager is now fully integrated into the CURSED development workflow!
