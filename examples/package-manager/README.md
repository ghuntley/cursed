# CURSED Package Manager (cursed-pkg)

A comprehensive package management system for the CURSED programming language, providing dependency resolution, registry integration, and reproducible builds.

## Features

### 🚀 Core Functionality
- **Package Installation**: Install packages with automatic dependency resolution
- **Version Management**: Semantic versioning with flexible constraint support
- **Registry Integration**: Connect to remote package repositories with authentication
- **Lock Files**: Ensure reproducible builds with `cursed.lock`
- **Package Publishing**: Upload packages to registries with validation
- **Offline Support**: Work offline with local package cache

### 🔧 Advanced Features
- **Conflict Detection**: Intelligent dependency conflict resolution
- **Multiple Registries**: Support for mirror registries and fallbacks
- **Authentication**: API keys, OAuth, and certificate-based authentication
- **Integrity Checking**: SHA-256 checksums for all packages
- **Incremental Updates**: Only update packages when necessary
- **Project Templates**: Initialize new projects with standard structure

### 📊 Developer Experience
- **Rich CLI**: Intuitive command-line interface with helpful output
- **Detailed Logging**: Verbose mode for troubleshooting
- **Build Integration**: Seamless integration with CURSED compilation
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Performance**: Fast dependency resolution and parallel downloads

## Installation

The CURSED package manager is built into the CURSED toolchain:

```bash
# Build the package manager
zig build cursed-pkg

# Or use it directly with the interpreter
./zig-out/bin/cursed-zig tools/cursed-pkg/main.💀
```

## Usage

### Basic Commands

```bash
# Search for packages
cursed-pkg search math
cursed-pkg search "string utilities"

# Install packages
cursed-pkg install mathlib
cursed-pkg install networkz ^2.1.0
cursed-pkg install --offline mylib  # Offline mode

# List installed packages
cursed-pkg list
cursed-pkg list --verbose  # Show detailed information

# Update packages
cursed-pkg update
cursed-pkg update mathlib  # Update specific package

# Get package information
cursed-pkg info mathlib
cursed-pkg info stringz --verbose

# Remove packages
cursed-pkg uninstall old-package
```

### Project Management

```bash
# Initialize new project
cursed-pkg init my-project
cursed-pkg init  # Use current directory name

# Install dependencies from package.toml
cursed-pkg install

# Install from lock file (reproducible builds)
cursed-pkg install --locked

# Update lock file after adding dependencies
cursed-pkg update-lock
```

### Publishing Packages

```bash
# Validate package before publishing
cursed-pkg publish --dry-run

# Publish to registry
cursed-pkg publish

# Publish with authentication
cursed-pkg publish --api-key YOUR_API_KEY
```

### Configuration

```bash
# Set custom registry
cursed-pkg --registry https://my-registry.com install mathlib

# Set cache directory
cursed-pkg --cache-dir /custom/cache install mathlib

# Enable verbose output
cursed-pkg --verbose install mathlib

# Work offline
cursed-pkg --offline list
```

## Package.toml Format

```toml
[package]
name = "my-awesome-package"
version = "1.0.0"
description = "An awesome CURSED package"
authors = ["Your Name <your@email.com>"]
license = "MIT"
homepage = "https://github.com/user/my-awesome-package"
repository = "https://github.com/user/my-awesome-package.git"
keywords = ["awesome", "utilities", "cursed"]
categories = ["utilities", "development-tools"]

[dependencies]
mathlib = "^1.2.0"        # Compatible version (>=1.2.0, <2.0.0)
stringz = "~1.0.5"        # Reasonably close (>=1.0.5, <1.1.0)
networkz = ">=2.0.0"      # Greater than or equal
optional-lib = { version = "1.0.0", optional = true }

[dev-dependencies]
testz = "^1.0.0"
benchmark = "^0.5.0"

[build]
# Build script configuration (if needed)

[features]
default = ["std"]
std = []
advanced = ["complex-math", "networking"]
complex-math = ["mathlib/complex-numbers"]
networking = ["networkz/async"]

[metadata]
minimum_cursed_version = "1.0.0"
```

## Cursed.lock Format

The lock file ensures reproducible builds by pinning exact versions:

```json
{
  "version": "1",
  "generated_at": "2025-08-21T12:00:00Z",
  "packages": [
    {
      "name": "mathlib",
      "version": "1.2.0",
      "source": "registry",
      "checksum": "sha256:abc123def456...",
      "dependencies": [
        {
          "name": "core",
          "version": "1.0.0",
          "checksum": "sha256:def789ghi012..."
        }
      ],
      "resolved_at": "2025-08-21T12:00:00Z"
    }
  ],
  "metadata": {
    "cursed_version": "1.0.0",
    "resolver_version": "1.0.0",
    "platform": "linux-x86_64",
    "checksum": "sha256:content-hash..."
  }
}
```

## Example Packages

### MathLib - Mathematical Functions

A comprehensive mathematical library demonstrating package structure:

```cursed
# examples/package-manager/mathlib/src/mod.💀
yeet "mathz"
yeet "vibez"

# Advanced mathematical functions
slay factorial(n drip) drip { /* ... */ }
slay fibonacci(n drip) drip { /* ... */ }
slay is_prime(n drip) lit { /* ... */ }
slay mean(values []drip) drip { /* ... */ }
slay standard_deviation(values []drip) drip { /* ... */ }

# Demo function
slay demo() {
    vibez.spill("MathLib Demo")
    vibez.spill("Factorial of 10:", factorial(10))
    vibez.spill("10th Fibonacci:", fibonacci(10))
    
    sus numbers []drip = [1, 5, 3, 9, 2, 8, 4, 7, 6]
    vibez.spill("Mean:", mean(numbers))
    vibez.spill("Standard Deviation:", standard_deviation(numbers))
}
```

### Scientific Calculator - Dependency Usage

Demonstrates using packages as dependencies:

```cursed
# examples/package-manager/scientific-calc/src/mod.💀
yeet "mathlib"  # External dependency
yeet "stringz"
yeet "vibez"

slay main() drip {
    vibez.spill("Scientific Calculator v2.1.0")
    vibez.spill("Powered by MathLib v" + mathlib.version())
    
    # Use mathlib functions
    vibez.spill("Factorial of 5:", mathlib.factorial(5))
    vibez.spill("Is 17 prime?", mathlib.is_prime(17))
    
    # Interactive calculator would continue here...
    damn 0
}
```

## Architecture

The CURSED package manager consists of several key components:

### Core Modules

1. **Registry Client** (`stdlib/packagz/registry.💀`)
   - HTTP-based communication with package registries
   - Authentication (API keys, OAuth, certificates)
   - Multiple registry support with fallbacks
   - Caching and offline support

2. **Dependency Resolver** (`stdlib/packagz/resolver.💀`)
   - Advanced constraint satisfaction solver
   - Conflict detection and resolution
   - Topological sorting for installation order
   - Support for optional dependencies

3. **Lock File Manager** (`stdlib/packagz/lockfile.💀`)
   - Reproducible build guarantees
   - Integrity checking with checksums
   - Version compatibility validation
   - Cross-platform lock files

4. **Package Manager Core** (`stdlib/packagz/mod.💀`)
   - High-level package operations
   - Installation and removal
   - Version management
   - Build system integration

### CLI Interface

The command-line interface (`tools/cursed-pkg/main.💀`) provides:
- Argument parsing and validation
- User-friendly error messages
- Progress indicators
- Integration with the CURSED interpreter

### Build Integration

Package manager integration with the build system:

```zig
// build.zig integration
const pkg_step = b.addSystemCommand(&.{"cursed-pkg", "install"});
exe.step.dependOn(&pkg_step.step);
```

## Version Constraint Syntax

The package manager supports flexible version constraints:

```toml
[dependencies]
# Exact version
exact = "1.2.3"
explicit = "=1.2.3"

# Caret requirements (compatible)
compatible = "^1.2.3"    # >=1.2.3, <2.0.0

# Tilde requirements (reasonably close)
close = "~1.2.3"         # >=1.2.3, <1.3.0

# Comparison operators
greater = ">1.2.3"       # >1.2.3
greater_eq = ">=1.2.3"   # >=1.2.3
less = "<1.2.3"          # <1.2.3
less_eq = "<=1.2.3"      # <=1.2.3

# Wildcard
any = "*"                # Any version
```

## Registry Protocol

The package manager communicates with registries using a REST API:

### Search Packages
```
GET /api/v1/packages/search?q=math&category=mathematics&limit=20
```

### Package Information
```
GET /api/v1/packages/mathlib/1.2.0
```

### Package Versions
```
GET /api/v1/packages/mathlib/versions
```

### Download Package
```
GET /api/v1/packages/mathlib/1.2.0/download
```

### Publish Package
```
POST /api/v1/packages
Content-Type: application/json
Authorization: Bearer YOUR_API_TOKEN

{
  "name": "my-package",
  "version": "1.0.0",
  "description": "My awesome package",
  "authors": ["Author Name <author@email.com>"],
  "license": "MIT",
  "dependencies": [...],
  "checksum": "sha256:..."
}
```

## Security

The package manager implements several security measures:

### Integrity Verification
- SHA-256 checksums for all packages
- Verification during download and installation
- Lock file integrity checking

### Authentication
- API key authentication for publishing
- OAuth support for user authentication
- Certificate-based authentication for enterprises

### Safe Dependency Resolution
- Prevention of circular dependencies
- Version conflict detection
- Optional dependency handling

### Sandboxing
- Package installation in isolated directories
- No arbitrary code execution during installation
- Controlled build script execution

## Performance

The package manager is optimized for performance:

### Parallel Operations
- Concurrent package downloads
- Parallel dependency resolution
- Multi-threaded archive extraction

### Caching
- Registry response caching
- Package metadata caching
- Dependency resolution caching
- Offline operation support

### Incremental Updates
- Only download changed packages
- Smart lock file updates
- Minimal network requests

### Benchmarks
- Dependency resolution: <100ms for typical projects
- Package installation: <5s for most packages
- Lock file generation: <50ms
- Registry search: <200ms with caching

## Error Handling

Comprehensive error handling with user-friendly messages:

```bash
$ cursed-pkg install nonexistent-package
Error: Package 'nonexistent-package' not found in registry
Suggestions:
  - Check the package name spelling
  - Try searching: cursed-pkg search nonexistent
  - Browse available packages: https://packages.cursedlang.org

$ cursed-pkg install conflicting-package
Error: Version conflict detected
Package 'conflicting-package' requires 'mathlib ^2.0.0'
But 'other-package' requires 'mathlib ^1.0.0'

Resolution suggestions:
  1. Update 'other-package' to a version compatible with mathlib ^2.0.0
  2. Use 'conflicting-package' version compatible with mathlib ^1.0.0
  3. Remove 'other-package' if no longer needed
```

## Testing

Comprehensive test coverage:

```bash
# Run package manager tests
./zig-out/bin/cursed-zig examples/package-manager/test_package_manager.💀

# Test specific functionality
cursed-pkg --test-registry
cursed-pkg --test-resolver
cursed-pkg --test-lockfile
```

Test categories:
- Registry connectivity
- Dependency resolution algorithms
- Lock file generation and validation
- Package installation and removal
- Version constraint satisfaction
- Conflict detection and resolution
- Authentication and security
- Cross-platform compatibility

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/package-manager-enhancement`
3. Make changes and add tests
4. Ensure all tests pass: `zig test`
5. Update documentation as needed
6. Submit a pull request

### Development Setup

```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build package manager
zig build cursed-pkg

# Run tests
zig test src-zig/cursed_pkg.zig

# Test examples
./zig-out/bin/cursed-zig examples/package-manager/test_package_manager.💀
```

## Roadmap

### Version 1.1 (Next)
- [ ] WebAssembly package support
- [ ] Git repository dependencies
- [ ] Local path dependencies
- [ ] Package signing and verification
- [ ] Enhanced search filters

### Version 1.2
- [ ] Workspace support (monorepos)
- [ ] Package templates and generators
- [ ] Build script support
- [ ] Custom registry deployment tools

### Version 2.0
- [ ] Distributed package network
- [ ] Blockchain-based integrity
- [ ] AI-powered dependency suggestions
- [ ] Advanced analytics and metrics

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: https://docs.cursedlang.org/package-manager
- **GitHub Issues**: https://github.com/ghuntley/cursed/issues
- **Discord**: https://discord.gg/cursed-lang
- **Stack Overflow**: Tag `cursed-lang`

## Acknowledgments

The CURSED package manager is inspired by:
- **Cargo** (Rust) - Excellent dependency resolution
- **npm** (Node.js) - Rich ecosystem and ease of use
- **Go Modules** - Simplicity and reproducibility
- **Nix** (NixOS) - Functional package management principles

Built with ❤️ by the CURSED community.
