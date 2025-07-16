# CURSED Package Manager

The CURSED Package Manager provides a comprehensive package ecosystem for the CURSED programming language, enabling developers to easily distribute, install, and manage CURSED packages.

## Features

- **Package Installation**: Install packages from registries with automatic dependency resolution
- **Version Management**: Support for semantic versioning with flexible version constraints
- **Workspace Support**: Manage multi-package projects with shared dependencies
- **Lock Files**: Ensure reproducible builds with lock file generation and validation
- **Package Publishing**: Publish packages to registries with validation and archiving
- **Registry Management**: Support for multiple package registries
- **Caching**: Efficient package caching for faster installations
- **CLI Interface**: Complete command-line interface for all package operations

## Installation

The package manager comes bundled with the CURSED compiler. To use it:

```bash
# Install the CURSED compiler (includes package manager)
cargo install cursed

# Use the package manager
cursed-pkg --help
```

## Quick Start

### Installing Packages

```bash
# Install the latest version of a package
cursed-pkg install math-utils

# Install a specific version
cursed-pkg install math-utils --version 1.2.3

# Install from a workspace
cursed-pkg workspace install
```

### Publishing Packages

```bash
# Initialize a new package
mkdir my-package
cd my-package

# Create package.toml and src/mod.csd files
# See "Package Structure" section below

# Publish the package (dry run first)
cursed-pkg publish --dry-run

# Publish for real
cursed-pkg publish
```

### Working with Workspaces

```bash
# Initialize a workspace
cursed-pkg workspace init --members package1,package2

# Add a new member
cursed-pkg workspace add new-package

# Install all workspace dependencies
cursed-pkg workspace install

# Build the entire workspace
cursed-pkg workspace build
```

## Package Structure

A CURSED package follows this structure:

```
my-package/
├── package.toml          # Package metadata and dependencies
├── src/
│   ├── mod.csd          # Main module (required)
│   └── lib/             # Additional modules
│       └── utils.csd
├── tests/               # Test files
│   └── integration_tests.csd
├── examples/            # Example code
│   └── basic_usage.csd
├── README.md           # Documentation
└── LICENSE             # License file
```

### package.toml Format

```toml
[package]
name = "my-package"
version = "1.0.0"
description = "A sample CURSED package"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
homepage = "https://github.com/yourname/my-package"
repository = "https://github.com/yourname/my-package"
keywords = ["utility", "math", "algorithms"]
categories = ["algorithms", "data-structures"]

[dependencies]
math-utils = "^1.0.0"
string-helpers = "~2.1.0"
crypto-lib = { version = "3.0.0", features = ["aes", "sha256"] }

[dev-dependencies]
test-framework = "1.0.0"

[features]
default = ["basic"]
basic = []
advanced = ["crypto-lib/advanced"]
```

### Version Specifications

CURSED uses semantic versioning (SemVer) with these constraint operators:

- `1.2.3` - Exact version
- `^1.2.3` - Compatible version (>=1.2.3, <2.0.0)
- `~1.2.3` - Reasonably close version (>=1.2.3, <1.3.0)
- `>=1.2.3` - Greater than or equal to
- `<2.0.0` - Less than
- `1.2.*` - Wildcard version

## CLI Commands

### Package Management

```bash
# Install a package
cursed-pkg install <package-name> [--version <version>]

# Uninstall a package
cursed-pkg uninstall <package-name>

# Update packages
cursed-pkg update [package-name]  # Update specific package or all packages

# List installed packages
cursed-pkg list

# Search for packages
cursed-pkg search <query>

# Show package information
cursed-pkg info <package-name>
```

### Publishing

```bash
# Publish a package
cursed-pkg publish [directory] [--dry-run]
```

### Workspace Management

```bash
# Initialize workspace
cursed-pkg workspace init [--members <member1,member2>]

# Install workspace dependencies
cursed-pkg workspace install

# Build workspace
cursed-pkg workspace build

# Add workspace member
cursed-pkg workspace add <name> [--path <path>]
```

### Lock File Management

```bash
# Generate lock file
cursed-pkg lock generate

# Validate lock file
cursed-pkg lock validate

# Update lock file
cursed-pkg lock update
```

### Registry Management

```bash
# Add a registry
cursed-pkg registry add <name> <url>

# Remove a registry
cursed-pkg registry remove <name>

# List registries
cursed-pkg registry list
```

### Maintenance

```bash
# Clean package cache
cursed-pkg clean
```

## Configuration

The package manager can be configured via a `cursed-pkg.toml` file in your home directory:

```toml
[package-manager]
# Default registry URL
registry-url = "https://packages.cursed-lang.org"

# Cache directory
cache-dir = "~/.cursed/cache"

# Maximum cache size (in bytes)
max-cache-size = 1073741824  # 1GB

# Timeout for network operations (in seconds)
timeout = 30

# Number of parallel downloads
parallel-downloads = 4

# Verify package signatures
verify-signatures = true

# Additional registries
[[registries]]
name = "my-registry"
url = "https://my-company.com/cursed-packages"
api-key = "your-api-key"
```

## Workspaces

Workspaces allow you to manage multiple related packages in a single repository. A workspace is defined by a `CursedWorkspace.toml` file:

```toml
[workspace]
members = [
    "packages/core",
    "packages/utils",
    "packages/web"
]

exclude = [
    "target",
    "temp"
]

[workspace.dependencies]
# Shared dependencies for all workspace members
testz = "3.0.0"
common-types = "1.0.0"
```

### Workspace Benefits

- **Shared Dependencies**: Dependencies are shared across workspace members
- **Unified Building**: Build all packages with a single command
- **Dependency Management**: Automatic dependency resolution across packages
- **Version Consistency**: Ensure consistent versions across related packages

## Lock Files

Lock files (`CursedPackage.lock`) ensure reproducible builds by recording exact versions of all dependencies:

```toml
# This file is generated automatically by cursed-pkg
# Do not edit this file manually

version = 1

[[package]]
name = "math-utils"
version = "1.2.3"
source = "registry+https://packages.cursed-lang.org"
checksum = "abc123def456..."
dependencies = [
    "basic-math 0.1.0",
]

[[package]]
name = "basic-math"
version = "0.1.0"
source = "registry+https://packages.cursed-lang.org"
checksum = "789ghi012jkl..."
dependencies = []
```

## Registry Protocol

The CURSED package registry follows a RESTful API:

### Endpoints

- `GET /packages/{name}` - Get package metadata
- `GET /packages/{name}/{version}` - Get specific version metadata
- `GET /packages/{name}/{version}/download` - Download package archive
- `POST /packages` - Publish new package
- `GET /search?q={query}` - Search packages
- `GET /categories` - List package categories

### Authentication

API authentication uses bearer tokens:

```bash
# Set API token
export CURSED_PKG_TOKEN="your-api-token"

# Or configure in cursed-pkg.toml
```

## Package Validation

Before publishing, packages are validated for:

- **Structure**: Required files (package.toml, src/mod.csd)
- **Metadata**: Valid package.toml format
- **Source Code**: Basic syntax validation of .csd files
- **Dependencies**: Valid dependency specifications
- **Size**: Package size limits
- **Security**: Malware scanning (if enabled)

## Caching

The package manager implements intelligent caching:

- **Package Archives**: Downloaded packages are cached
- **Metadata**: Package metadata is cached with TTL
- **Dependency Resolution**: Resolution results are cached
- **Compression**: Cache uses compression to save space
- **Cleanup**: Automatic cleanup of old cached data

## Security

Security features include:

- **Signature Verification**: Package signatures are verified
- **Checksum Validation**: Package integrity is verified
- **Secure Downloads**: HTTPS for all network operations
- **Sandboxing**: Package installation in isolated environment
- **Audit Trail**: All operations are logged

## Troubleshooting

### Common Issues

1. **Network Connectivity**
   ```bash
   # Test registry connectivity
   curl -I https://packages.cursed-lang.org/health
   
   # Use offline mode
   cursed-pkg install package-name --offline
   ```

2. **Cache Corruption**
   ```bash
   # Clear cache
   cursed-pkg clean
   
   # Rebuild cache
   cursed-pkg update
   ```

3. **Lock File Issues**
   ```bash
   # Regenerate lock file
   rm CursedPackage.lock
   cursed-pkg lock generate
   ```

4. **Workspace Conflicts**
   ```bash
   # Validate workspace
   cursed-pkg workspace validate
   
   # Clean and rebuild
   cursed-pkg workspace clean
   cursed-pkg workspace install
   ```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
export CURSED_PKG_LOG=debug
cursed-pkg install package-name
```

## Integration with CURSED Compiler

The package manager integrates seamlessly with the CURSED compiler:

```bash
# Compile with automatic dependency resolution
cursed compile main.csd

# The compiler automatically:
# 1. Reads package.toml
# 2. Resolves dependencies
# 3. Downloads missing packages
# 4. Compiles with resolved packages in scope
```

## Best Practices

1. **Versioning**: Use semantic versioning for your packages
2. **Documentation**: Include comprehensive README and examples
3. **Testing**: Write tests for your packages
4. **Dependencies**: Minimize dependencies and specify exact versions
5. **Security**: Keep dependencies up to date
6. **Performance**: Use workspaces for related packages
7. **CI/CD**: Automate package publishing in your CI pipeline

## Examples

### Simple Package

```cursed
// src/mod.csd
yeet "testz"

/// Calculate the factorial of a number
slay factorial(n drip) drip {
    lowkey n <= 1 {
        damn 1.0
    } else {
        damn n * factorial(n - 1.0)
    }
}

/// Run tests
test_start("factorial tests")
assert_eq_float(factorial(5.0), 120.0)
assert_eq_float(factorial(0.0), 1.0)
print_test_summary()
```

### Using Dependencies

```cursed
// src/mod.csd
yeet "math-utils"
yeet "string-helpers"

slay format_result(value drip) tea {
    sus formatted := math_utils.round(value, 2)
    damn string_helpers.format("Result: {}", formatted)
}
```

### Workspace Example

```
my-workspace/
├── CursedWorkspace.toml
├── packages/
│   ├── core/
│   │   ├── package.toml
│   │   └── src/mod.csd
│   ├── utils/
│   │   ├── package.toml
│   │   └── src/mod.csd
│   └── web/
│       ├── package.toml
│       └── src/mod.csd
└── examples/
    └── full_example.csd
```

## Contributing

To contribute to the CURSED package manager:

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

The CURSED package manager is licensed under the MIT License.
