# CURSED Package Manager (Getting Your Dependencies Together) 📦

The CURSED package manager (`cursed-pkg`) is your one-stop shop for managing dependencies, publishing packages, and building epic projects! No more manual dependency juggling - this tool keeps your project organized and your dependencies fresh, periodt! 💅

## Installation (Getting That Package Manager Glow-Up) 🔧

### Install from Source (DIY Energy)
```bash
# Clone the repo if you haven't already
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the package manager
make build-pkg

# Install globally (recommended)
cargo install --path . --bin cursed-pkg
```

### Download Binary (Quick and Easy)
```bash
# Download from releases (when available)
curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-pkg-linux -o cursed-pkg
chmod +x cursed-pkg
sudo mv cursed-pkg /usr/local/bin/
```

## Getting Started (Your First Package Project) 🚀

### Creating a New Package
```bash
# Create a new package project
cursed-pkg new my-awesome-package
cd my-awesome-package

# This creates:
# - CursedPackage.toml  (package manifest)
# - src/               (source code directory)
# - examples/          (example code)
# - tests/             (test directory)
# - README.md          (documentation)
```

### Your First CursedPackage.toml
When you create a new package, you get a starter manifest:

```toml
[package]
name = "my-awesome-package"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "An absolutely iconic CURSED package"
license = "MIT"
repository = "https://github.com/yourusername/my-awesome-package"
documentation = "https://docs.yourdomain.com/my-awesome-package"
keywords = ["cursed", "awesome", "package"]
categories = ["utility", "development"]
homepage = "https://yourdomain.com/my-awesome-package"

[dependencies]
# Your dependencies go here
# Example: math_utils = "1.0.0"

[dev-dependencies]  
# Development-only dependencies
# Example: test_utils = "0.5.0"

[build-dependencies]
# Build-time dependencies
# Example: code_generator = "2.1.0"

[features]
default = []
advanced = ["crypto", "network"]
crypto = []
network = []

[[bin]]
name = "my-tool"
path = "src/bin/my_tool.csd"
```

## Command Reference (All the CLI Magic) 🎮

### Package Creation and Management

| Command | Description | Example |
|---------|-------------|---------|
| `cursed-pkg new <name>` | Create new package | `cursed-pkg new web-utils` |
| `cursed-pkg init` | Initialize package in existing directory | `cursed-pkg init` |
| `cursed-pkg build` | Build the current package | `cursed-pkg build` |
| `cursed-pkg check` | Check package without building | `cursed-pkg check` |
| `cursed-pkg test` | Run tests | `cursed-pkg test` |
| `cursed-pkg clean` | Clean build artifacts | `cursed-pkg clean` |

### Dependency Management

| Command | Description | Example |
|---------|-------------|---------|
| `cursed-pkg add <package>` | Add dependency | `cursed-pkg add json_utils@1.2.0` |
| `cursed-pkg remove <package>` | Remove dependency | `cursed-pkg remove old_package` |
| `cursed-pkg update` | Update all dependencies | `cursed-pkg update` |
| `cursed-pkg list` | List installed packages | `cursed-pkg list` |
| `cursed-pkg tree` | Show dependency tree | `cursed-pkg tree` |

### Publishing and Registry

| Command | Description | Example |
|---------|-------------|---------|
| `cursed-pkg publish` | Publish to registry | `cursed-pkg publish` |
| `cursed-pkg login` | Login to registry | `cursed-pkg login` |
| `cursed-pkg search <query>` | Search packages | `cursed-pkg search json` |
| `cursed-pkg show <package>` | Show package info | `cursed-pkg show math_utils` |

### Workspace Management

| Command | Description | Example |
|---------|-------------|---------|
| `cursed-pkg workspace new` | Create new workspace | `cursed-pkg workspace new my-project` |
| `cursed-pkg workspace add` | Add package to workspace | `cursed-pkg workspace add ./my-lib` |
| `cursed-pkg workspace build` | Build entire workspace | `cursed-pkg workspace build` |

## Working with Dependencies (Getting Your Imports Right) 🔗

### Adding Dependencies

```bash
# Add latest version
cursed-pkg add math_utils

# Add specific version
cursed-pkg add math_utils@2.1.0

# Add with version constraint
cursed-pkg add "math_utils@>=2.0.0,<3.0.0"

# Add development dependency
cursed-pkg add --dev test_framework

# Add build dependency
cursed-pkg add --build code_generator

# Add with features
cursed-pkg add web_utils --features="http,json"
```

### Using Dependencies in Code

Once added to your `CursedPackage.toml`, use them in your CURSED code:

```cursed
vibe main

import "math_utils"
import "json_utils" as json
import "web_utils/http"

slay main() {
    sus result = math_utils.add(5, 3)
    printfr("Result: {}", result)
    
    sus data = json.parse("{\"name\": \"cursed\"}")
    printfr("Parsed: {:?}", data)
}
```

### Version Constraints

| Constraint | Meaning | Example |
|------------|---------|---------|
| `1.2.3` | Exact version | `math_utils = "1.2.3"` |
| `^1.2.3` | Compatible version (default) | `math_utils = "^1.2.3"` |
| `~1.2.3` | Patch updates only | `math_utils = "~1.2.3"` |
| `>=1.2.0` | Greater than or equal | `math_utils = ">=1.2.0"` |
| `1.2.*` | Wildcard patch | `math_utils = "1.2.*"` |
| `1.*` | Wildcard minor | `math_utils = "1.*"` |

### Dependency Resolution

The package manager uses semantic versioning and resolves conflicts automatically:

```mermaid
graph TD
    A[Your Package] --> B[math_utils@^2.0.0]
    A --> C[json_utils@^1.5.0]
    C --> D[math_utils@^1.8.0]
    
    B --> E[math_utils@2.1.0]
    D --> E
    
    F[Resolution: math_utils@2.1.0] --> E
```

## Package Structure (Keeping It Organized) 📁

### Standard Package Layout

```
my-package/
├── CursedPackage.toml    # Package manifest
├── README.md             # Package documentation
├── LICENSE               # License file
├── .gitignore           # Git ignore rules
├── src/                 # Source code
│   ├── lib.csd         # Main library file
│   ├── utils/          # Utility modules
│   │   ├── math.csd
│   │   └── string.csd
│   └── bin/            # Binary targets
│       └── cli.csd
├── tests/              # Test files
│   ├── integration.csd
│   └── unit.csd
├── examples/           # Example code
│   ├── basic.csd
│   └── advanced.csd
├── docs/               # Additional documentation
├── benchmarks/         # Performance benchmarks
└── scripts/            # Build/utility scripts
```

### Library Package Example

**src/lib.csd** (main library file):
```cursed
vibe math_utils

fr fr Mathematical utility functions for CURSED programs

export slay add(a normie, b normie) normie {
    cap a + b
}

export slay multiply(a normie, b normie) normie {
    cap a * b
}

export slay factorial(n normie) normie {
    issa n <= 1 {
        cap 1
    }
    cap n * factorial(n - 1)
}
```

### Binary Package Example

**src/bin/calculator.csd**:
```cursed
vibe calculator

import "../lib"

slay main(args []tea) {
    issa len(args) < 3 {
        printfr("Usage: calculator <op> <a> <b>")
        cap 1
    }
    
    sus op = args[0]
    sus a = parse_int(args[1])
    sus b = parse_int(args[2])
    
    sus result = vibe_check op {
        mood "add" -> math_utils.add(a, b)
        mood "multiply" -> math_utils.multiply(a, b)
        basic -> {
            printfr("Unknown operation: {}", op)
            cap 1
        }
    }
    
    printfr("Result: {}", result)
    cap 0
}
```

## Workspaces (Managing Multiple Packages) 🏗️

Workspaces let you manage multiple related packages together:

### Workspace Structure

```
my-workspace/
├── CursedWorkspace.toml  # Workspace manifest
├── packages/
│   ├── core/            # Core library
│   │   ├── CursedPackage.toml
│   │   └── src/
│   ├── web/             # Web utilities
│   │   ├── CursedPackage.toml
│   │   └── src/
│   └── cli/             # Command-line tool
│       ├── CursedPackage.toml
│       └── src/
└── examples/            # Workspace-wide examples
```

### CursedWorkspace.toml

```toml
[workspace]
name = "my-awesome-workspace"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]

# Package locations
members = [
    "packages/core",
    "packages/web", 
    "packages/cli"
]

# Exclude certain directories
exclude = [
    "packages/experimental/*"
]

# Shared dependencies for all workspace members
[workspace.dependencies]
json_utils = "2.1.0"
test_framework = "1.5.0"

# Shared metadata
[workspace.package]
edition = "2024"
license = "MIT"
repository = "https://github.com/yourusername/my-awesome-workspace"
```

### Working with Workspaces

```bash
# Create new workspace
cursed-pkg workspace new my-project
cd my-project

# Add a new package to workspace
cursed-pkg workspace add packages/new-package

# Build all packages
cursed-pkg workspace build

# Test all packages
cursed-pkg workspace test

# Build specific package
cursed-pkg build -p core

# Run binary from workspace
cursed-pkg run -p cli -- --help
```

## Features and Conditional Compilation 🎛️

Features allow optional functionality and conditional compilation:

### Defining Features

In `CursedPackage.toml`:

```toml
[features]
# Default features (enabled by default)
default = ["std", "basic_crypto"]

# Feature definitions
std = []
crypto = ["crypto_utils", "rand"]
network = ["http_client", "websockets"]
advanced = ["crypto", "network", "threading"]

# Optional dependencies (only included if feature is enabled)
[dependencies]
crypto_utils = { version = "1.0", optional = true }
http_client = { version = "2.1", optional = true }
```

### Using Features in Code

```cursed
vibe my_package

#[cfg(feature = "crypto")]
import "crypto_utils"

#[cfg(feature = "network")]
import "http_client"

export slay process_data(data tea) tea {
    #[cfg(feature = "crypto")]
    {
        sus encrypted = crypto_utils.encrypt(data)
        cap encrypted
    }
    
    #[cfg(not(feature = "crypto"))]
    {
        cap data  // Return data unchanged
    }
}

#[cfg(feature = "network")]
export slay fetch_data(url tea) Result<tea, tea> {
    cap http_client.get(url)
}
```

### Building with Features

```bash
# Build with default features
cursed-pkg build

# Build with specific features
cursed-pkg build --features="crypto,network"

# Build with all features
cursed-pkg build --all-features

# Build with no default features
cursed-pkg build --no-default-features

# Add dependency with features
cursed-pkg add web_utils --features="http,json"
```

## Publishing Packages (Sharing the Vibes) 📢

### Preparing for Publication

1. **Complete your package metadata**:
```toml
[package]
name = "my-awesome-lib"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "An absolutely iconic CURSED library for doing awesome things"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/my-awesome-lib"
documentation = "https://docs.yourdomain.com/my-awesome-lib"
homepage = "https://yourdomain.com/my-awesome-lib"
keywords = ["cursed", "awesome", "utility", "productivity"]
categories = ["development-tools", "data-structures"]
readme = "README.md"
exclude = [
    "tests/fixtures/*",
    "benchmarks/data/*",
    ".github/*"
]
```

2. **Write good documentation**:
```bash
# Generate docs
cursed-pkg doc

# Check documentation links
cursed-pkg doc --check
```

3. **Run quality checks**:
```bash
# Run all tests
cursed-pkg test

# Check formatting
cursed-fmt --check src/

# Run linter
cursed-lint src/

# Check for security issues
cursed-pkg audit
```

### Publishing Process

```bash
# Login to registry (first time only)
cursed-pkg login

# Dry run (see what would be published)
cursed-pkg publish --dry-run

# Publish for real
cursed-pkg publish

# Publish with specific registry
cursed-pkg publish --registry https://my-private-registry.com
```

### Version Management

Follow semantic versioning:
- **Patch** (1.0.1): Bug fixes, no breaking changes
- **Minor** (1.1.0): New features, backward compatible
- **Major** (2.0.0): Breaking changes

```bash
# Bump version automatically
cursed-pkg version patch   # 1.0.0 -> 1.0.1
cursed-pkg version minor   # 1.0.0 -> 1.1.0  
cursed-pkg version major   # 1.0.0 -> 2.0.0

# Set specific version
cursed-pkg version 2.1.0
```

## Registry Configuration (Where Packages Live) 🏪

### Default Registry

CURSED packages are published to the official registry by default:
- **URL**: `https://registry.cursed-lang.org`
- **Documentation**: `https://docs.cursed-lang.org`
- **Search**: `https://cursed-lang.org/packages`

### Private Registries

Configure private registries in `~/.cursed/config.toml`:

```toml
[registries]
# Default registry
default = "https://registry.cursed-lang.org"

# Private company registry
company = "https://packages.mycompany.com"

# Local development registry
local = "http://localhost:8080"

[registries.company]
index = "https://packages.mycompany.com/index"
token = "your-auth-token"

[registries.local]
index = "http://localhost:8080/index"
```

### Using Private Registries

```bash
# Publish to specific registry
cursed-pkg publish --registry company

# Install from specific registry
cursed-pkg add my-private-lib --registry company

# Search specific registry
cursed-pkg search json --registry company
```

In `CursedPackage.toml`:
```toml
[dependencies]
# From default registry
public_lib = "1.0.0"

# From specific registry
private_lib = { version = "2.1.0", registry = "company" }

# From git repository
dev_lib = { git = "https://github.com/myorg/dev-lib", branch = "main" }

# From local path (development)
local_lib = { path = "../local-lib" }
```

## Security and Trust (Keeping It Safe) 🔒

### Package Verification

All packages are automatically verified:
- **Checksum verification**: Ensures package integrity
- **Signature verification**: Validates publisher identity
- **License validation**: Checks license compatibility
- **Dependency scanning**: Identifies security vulnerabilities

### Security Commands

```bash
# Audit dependencies for security issues
cursed-pkg audit

# Show detailed security report
cursed-pkg audit --verbose

# Update to fix security issues
cursed-pkg audit --fix

# Check specific package
cursed-pkg audit math_utils
```

### Trust Management

```bash
# List trusted publishers
cursed-pkg trust list

# Trust a publisher
cursed-pkg trust add publisher@cursed-lang.org

# Revoke trust
cursed-pkg trust remove publisher@cursed-lang.org

# Show package trust status
cursed-pkg trust show math_utils
```

### Lock Files

Lock files ensure reproducible builds:

**CursedPackage.lock** (automatically generated):
```toml
# This file is automatically generated. Do not edit.
[[package]]
name = "math_utils"
version = "2.1.0"
checksum = "abc123def456..."
dependencies = [
    "number_types 1.0.0"
]

[[package]]
name = "number_types"
version = "1.0.0"
checksum = "def456ghi789..."
dependencies = []
```

Lock file commands:
```bash
# Generate/update lock file
cursed-pkg lock

# Build from lock file (CI/production)
cursed-pkg build --locked

# Verify lock file is up to date
cursed-pkg verify
```

## Best Practices (Staying Iconic) 💎

### Package Naming
- Use lowercase with underscores: `web_utils`, `json_parser`
- Be descriptive but concise: `http_client` not `h` or `super_awesome_http_client_library`
- Avoid generic names: `utils` → `string_utils`
- Consider namespacing: `mycompany_web_utils`

### Versioning Strategy
- Start with `0.1.0` for initial development
- Use `1.0.0` when API is stable
- Follow semantic versioning strictly
- Document breaking changes clearly

### Documentation
- Write clear package descriptions
- Include usage examples in README
- Document all public APIs
- Provide migration guides for breaking changes

### Testing
- Test all public functionality
- Include integration tests
- Test with different feature combinations
- Use continuous integration

### Dependencies
- Keep dependencies minimal
- Pin major versions for stability
- Regularly update and audit dependencies
- Consider dependency licensing

## Troubleshooting (When Things Get Sus) 🔧

### Common Issues

**❌ "Package not found"**
```bash
# Check package name and registry
cursed-pkg search partial_name

# Verify registry configuration
cursed-pkg config list
```

**❌ "Version conflict"**
```bash
# Show dependency tree
cursed-pkg tree

# Update dependencies
cursed-pkg update

# Use specific versions
cursed-pkg add package@=1.2.3
```

**❌ "Build failed"**
```bash
# Clean and rebuild
cursed-pkg clean
cursed-pkg build

# Check for missing dependencies
cursed-pkg check

# Verbose build output
cursed-pkg build --verbose
```

**❌ "Authentication failed"**
```bash
# Re-login to registry
cursed-pkg login

# Check token validity
cursed-pkg whoami

# Use specific registry
cursed-pkg login --registry https://my-registry.com
```

### Getting Help

```bash
# Show help for any command
cursed-pkg help
cursed-pkg help build
cursed-pkg help publish

# Show version and debug info
cursed-pkg --version
cursed-pkg --verbose build

# Check configuration
cursed-pkg config show
```

## Advanced Usage (For Power Users) ⚡

### Custom Build Scripts

Add build scripts in `CursedPackage.toml`:

```toml
[package.scripts]
prebuild = "scripts/generate_code.sh"
postbuild = "scripts/validate_output.sh"
pretest = "scripts/setup_test_data.sh"
```

### Cross-Platform Building

```bash
# Build for different targets
cursed-pkg build --target linux-x64
cursed-pkg build --target windows-x64
cursed-pkg build --target macos-arm64

# List available targets
cursed-pkg targets list
```

### Optimization Profiles

```toml
[profiles.dev]
optimization = "none"
debug = true

[profiles.release]
optimization = "max"
debug = false
strip = true

[profiles.bench]
optimization = "max"
debug = true
```

That's everything you need to know about the CURSED package manager! Your dependencies are about to be absolutely organized! 📦✨

For more technical details, check out the [Package Manager API Documentation](package_manager_api.md) and [Registry Protocol Specification](package_registry_protocol.md).
