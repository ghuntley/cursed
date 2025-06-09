# CursedPackage.toml Specification 📋

The `CursedPackage.toml` file is the manifest for CURSED packages, containing all the metadata, dependencies, and configuration needed to build and distribute your package. This document provides the complete specification for this file format.

## File Format 📝

`CursedPackage.toml` is written in [TOML](https://toml.io) format, which is human-readable and easy to edit. The file must be located in the root directory of your package.

## Basic Structure 🏗️

```toml
[package]
# Package metadata

[dependencies]  
# Runtime dependencies

[dev-dependencies]
# Development-only dependencies

[build-dependencies]
# Build-time dependencies

[features]
# Optional features

[[bin]]
# Binary targets

[[lib]]
# Library targets

[scripts]
# Build scripts

[profiles]
# Build profiles
```

## Package Section 📦

The `[package]` section contains metadata about your package.

### Required Fields

```toml
[package]
name = "my-awesome-package"
version = "1.2.3"
```

- **`name`** (string): Package name, must be unique in registry
  - Format: lowercase letters, numbers, underscores, hyphens
  - Length: 3-64 characters
  - Must not start with numbers or special characters

- **`version`** (string): Package version following [Semantic Versioning](https://semver.org)
  - Format: `MAJOR.MINOR.PATCH` (e.g., "1.2.3")
  - Pre-release: `1.2.3-alpha.1`
  - Build metadata: `1.2.3+20231201`

### Optional Fields

```toml
[package]
name = "web-framework"
version = "2.1.0"
authors = ["Jane Doe <jane@example.com>", "John Smith <john@example.com>"]
description = "A blazingly fast web framework for CURSED"
documentation = "https://docs.example.com/web-framework"
homepage = "https://example.com/web-framework" 
repository = "https://github.com/example/web-framework"
license = "MIT OR Apache-2.0"
license-file = "LICENSE.txt"
readme = "README.md"
keywords = ["web", "framework", "http", "server"]
categories = ["web-programming", "development-tools"]
edition = "2024"
exclude = [
    "tests/fixtures/*",
    "benchmarks/data/*",
    ".github/*",
    "*.tmp"
]
include = [
    "src/**/*",
    "examples/**/*",
    "README.md",
    "LICENSE*"
]
```

#### Detailed Field Descriptions

- **`authors`** (array of strings): Package authors with optional email
  - Format: `"Name <email@domain.com>"` or just `"Name"`

- **`description`** (string): One-line description of the package
  - Length: 10-300 characters
  - Used in search results and package listings

- **`documentation`** (string): URL to package documentation
  - Must be HTTPS URL
  - Can point to docs.cursed-lang.org, GitHub pages, etc.

- **`homepage`** (string): URL to package homepage/website
  - Must be HTTPS URL

- **`repository`** (string): URL to source code repository
  - Supports GitHub, GitLab, Bitbucket, etc.
  - Format: `https://github.com/user/repo`

- **`license`** (string): License identifier or expression
  - Use SPDX license identifiers: `MIT`, `Apache-2.0`, `GPL-3.0`
  - Multiple licenses: `MIT OR Apache-2.0`
  - Custom license: use `license-file` instead

- **`license-file`** (string): Path to license file
  - Use when license is not a standard SPDX identifier
  - Relative to package root

- **`readme`** (string): Path to README file
  - Default: `README.md`
  - Displayed on package registry page

- **`keywords`** (array of strings): Search keywords
  - 1-10 keywords maximum
  - Each keyword: 2-20 characters
  - Used for package discovery

- **`categories`** (array of strings): Package categories
  - Predefined categories (see Categories section)
  - Used for package classification

- **`edition`** (string): CURSED language edition
  - Current: `"2024"`
  - Determines available language features

- **`exclude`** (array of strings): Files to exclude from package
  - Glob patterns relative to package root
  - Applied before `include` patterns

- **`include`** (array of strings): Files to include in package
  - Glob patterns relative to package root
  - If specified, only these files are included
  - Applied after `exclude` patterns

### Package Categories 📂

Predefined categories for package classification:

- **`algorithms`**: Algorithms and data structures
- **`api-bindings`**: Bindings to external APIs
- **`authentication`**: Authentication and authorization
- **`caching`**: Caching implementations
- **`command-line-interface`**: CLI tools and utilities
- **`concurrency`**: Concurrency and parallelism
- **`cryptography`**: Cryptographic functions
- **`data-structures`**: Data structure implementations
- **`database`**: Database drivers and ORMs
- **`date-and-time`**: Date and time handling
- **`development-tools`**: Development and build tools
- **`encoding`**: Text and binary encoding/decoding
- **`filesystem`**: File system operations
- **`graphics`**: Graphics and image processing
- **`gui`**: Graphical user interfaces
- **`hardware-support`**: Hardware-specific functionality
- **`internationalization`**: i18n and localization
- **`mathematics`**: Mathematical functions
- **`memory-management`**: Memory allocation and GC
- **`network-programming`**: Networking protocols
- **`no-std`**: No standard library packages
- **`os`**: Operating system interfaces
- **`parser-implementations`**: Parsers and parsing
- **`rendering`**: Rendering engines
- **`science`**: Scientific computing
- **`simulation`**: Simulation engines
- **`template-engine`**: Template processors
- **`text-processing`**: Text manipulation
- **`value-formatting`**: Value serialization
- **`web-programming`**: Web development
- **`websocket`**: WebSocket implementations

## Dependencies Section 🔗

Dependencies are other packages your package needs to function.

### Basic Dependencies

```toml
[dependencies]
# Latest compatible version
json_utils = "2.1.0"

# Version constraint
math_utils = "^1.5.0"  # >=1.5.0, <2.0.0
string_utils = "~1.2.3"  # >=1.2.3, <1.3.0

# Exact version  
crypto_utils = "=3.0.1"

# Version range
http_client = ">=2.0.0, <4.0.0"
```

### Advanced Dependencies

```toml
[dependencies]
# Optional dependency (only included with feature)
advanced_crypto = { version = "1.0", optional = true }

# From specific registry
private_lib = { version = "2.1.0", registry = "company" }

# From git repository
dev_lib = { git = "https://github.com/org/lib", tag = "v1.0.0" }
experimental = { git = "https://github.com/org/exp", branch = "main" }
feature_branch = { git = "https://github.com/org/fb", rev = "abc123" }

# From local path (development)
local_lib = { path = "../local-lib" }

# With specific features enabled
web_framework = { version = "3.0", features = ["async", "tls"] }

# Renamed import
xml = { package = "xml_parser", version = "2.0" }

# Platform-specific dependencies
[target.'cfg(unix)'.dependencies]
unix_utils = "1.0"

[target.'cfg(windows)'.dependencies]  
windows_utils = "1.0"
```

### Development Dependencies

Dependencies only used during development and testing:

```toml
[dev-dependencies]
test_framework = "2.0"
mock_server = "1.5"
benchmark_tools = "0.8"
```

### Build Dependencies

Dependencies used during the build process:

```toml
[build-dependencies]
code_generator = "1.2"
protobuf_compiler = "3.0"
```

## Version Constraints 📏

CURSED uses [Semantic Versioning](https://semver.org) with these constraint operators:

| Constraint | Description | Example | Matches |
|------------|-------------|---------|---------|
| `1.2.3` | Compatible (default `^`) | `1.2.3` | `>=1.2.3, <2.0.0` |
| `^1.2.3` | Compatible updates | `^1.2.3` | `>=1.2.3, <2.0.0` |
| `~1.2.3` | Patch updates only | `~1.2.3` | `>=1.2.3, <1.3.0` |
| `=1.2.3` | Exact version | `=1.2.3` | `1.2.3` only |
| `>=1.2.0` | Greater or equal | `>=1.2.0` | `1.2.0` and above |
| `>1.2.0` | Greater than | `>1.2.0` | Above `1.2.0` |
| `<=1.2.0` | Less or equal | `<=1.2.0` | `1.2.0` and below |
| `<1.2.0` | Less than | `<1.2.0` | Below `1.2.0` |
| `*` | Any version | `*` | Any version |
| `1.*` | Any patch version | `1.*` | `>=1.0.0, <2.0.0` |
| `1.2.*` | Any patch version | `1.2.*` | `>=1.2.0, <1.3.0` |

### Multiple Constraints

```toml
[dependencies]
# Both constraints must be satisfied
my_lib = ">=1.2.0, <2.0.0"

# Complex constraint
stable_lib = ">=1.0.0, <2.0.0, !=1.5.0"
```

## Features Section 🎛️

Features enable conditional compilation and optional dependencies.

```toml
[features]
# Default features (enabled by default)
default = ["std", "serde"]

# Feature definitions
std = []  # Empty feature
crypto = ["crypto_utils", "rand"]  # Enables dependencies
network = ["http_client", "tls"]
async = ["tokio", "futures"]

# Feature combinations
full = ["crypto", "network", "async"]

# Weak dependencies (don't enable the dependency by default)
json = ["json_utils?/serde"]
```

### Feature Dependencies

```toml
[dependencies]
# Normal dependency
base_lib = "1.0"

# Optional dependency (only with feature)
crypto_utils = { version = "2.0", optional = true }
http_client = { version = "1.5", optional = true }

[features]
default = []
crypto = ["crypto_utils"]  # Enables crypto_utils dependency
network = ["http_client"]  # Enables http_client dependency
```

### Using Features in Code

```cursed
vibe my_package

#[cfg(feature = "crypto")]
import "crypto_utils"

export slay process_data(data tea) tea {
    #[cfg(feature = "crypto")]
    {
        cap crypto_utils.encrypt(data)
    }
    
    #[cfg(not(feature = "crypto"))]
    {
        cap data
    }
}
```

## Binary and Library Targets 🎯

### Library Target

Most packages have one library target (default):

```toml
[lib]
name = "my_awesome_lib"  # Default: package name
path = "src/lib.csd"     # Default: src/lib.csd
```

### Binary Targets

Packages can have multiple binary targets:

```toml
# Single binary
[[bin]]
name = "my-tool"
path = "src/bin/tool.csd"

# Multiple binaries
[[bin]]
name = "client"
path = "src/bin/client.csd"

[[bin]]
name = "server"  
path = "src/bin/server.csd"

# Binary with specific features
[[bin]]
name = "advanced-tool"
path = "src/bin/advanced.csd"
required-features = ["crypto", "network"]
```

## Scripts Section 🛠️

Build scripts run at specific points in the build process:

```toml
[scripts]
prebuild = "scripts/generate_code.sh"
postbuild = "scripts/validate_output.sh"
pretest = "scripts/setup_test_data.sh"
posttest = "scripts/cleanup_test_data.sh"
prepack = "scripts/prepare_package.sh"
postpack = "scripts/sign_package.sh"
```

Script types:
- **`prebuild`**: Before compilation starts
- **`postbuild`**: After compilation completes
- **`pretest`**: Before running tests
- **`posttest`**: After running tests  
- **`prepack`**: Before creating package archive
- **`postpack`**: After creating package archive

## Build Profiles 🏗️

Profiles define compilation settings for different scenarios:

```toml
[profiles.dev]
optimization = "none"
debug = true
debug-assertions = true

[profiles.release]
optimization = "max"
debug = false
debug-assertions = false
strip = true
lto = true

[profiles.test]
optimization = "fast"
debug = true
debug-assertions = true

[profiles.bench]
optimization = "max"
debug = true
debug-assertions = false

# Custom profile
[profiles.production]
inherits = "release"
optimization = "size"
panic = "abort"
```

Profile settings:
- **`optimization`**: `"none"`, `"fast"`, `"max"`, `"size"`
- **`debug`**: Include debug information (boolean)
- **`debug-assertions`**: Enable debug assertions (boolean)
- **`strip`**: Strip symbols from binary (boolean)
- **`lto`**: Enable link-time optimization (boolean)
- **`panic`**: Panic strategy: `"unwind"`, `"abort"`
- **`inherits`**: Inherit from another profile

## Target-Specific Configuration 🎯

Configure dependencies and settings for specific platforms:

```toml
# Unix-specific dependencies
[target.'cfg(unix)'.dependencies]
unix_socket = "1.0"

# Windows-specific dependencies
[target.'cfg(windows)'.dependencies]
winapi = "2.0"

# Architecture-specific
[target.'cfg(target_arch = "x86_64")'.dependencies]
simd_utils = "1.0"

# OS and architecture combination
[target.'cfg(all(target_os = "linux", target_arch = "x86_64"))'.dependencies]
linux_x64_optimized = "1.0"

# Custom target
[target.wasm32-unknown-unknown.dependencies]
wasm_utils = "1.0"
```

### Target Predicates

Common target predicates:

- **Operating System**: `target_os = "linux"`, `"windows"`, `"macos"`, `"freebsd"`
- **Architecture**: `target_arch = "x86_64"`, `"aarch64"`, `"arm"`, `"wasm32"`
- **Environment**: `target_env = "gnu"`, `"msvc"`, `"musl"`
- **Pointer width**: `target_pointer_width = "64"`, `"32"`
- **Endianness**: `target_endian = "little"`, `"big"`
- **Family**: `target_family = "unix"`, `"windows"`

### Combining Predicates

```toml
# All conditions must be true
[target.'cfg(all(unix, target_arch = "x86_64"))'.dependencies]

# Any condition can be true  
[target.'cfg(any(target_os = "linux", target_os = "freebsd"))'.dependencies]

# Negation
[target.'cfg(not(windows))'.dependencies]
```

## Workspace Configuration 🏢

For multi-package projects:

```toml
[workspace]
members = [
    "packages/core",
    "packages/web",
    "packages/cli"
]

exclude = [
    "packages/experimental/*"
]

# Shared dependencies across workspace
[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

# Shared package metadata
[workspace.package]
authors = ["Team Name <team@company.com>"]
edition = "2024"
license = "MIT"
repository = "https://github.com/company/project"

# Resolver version
resolver = "2"
```

## Metadata Section 📊

Additional metadata for tools and registries:

```toml
[package.metadata]
# Custom metadata for your tools
docs-rs = { all-features = true }
playground = { features = ["std", "demo"] }

[package.metadata.registry]
# Registry-specific metadata
featured = true
maintained-by = "core-team"
stability = "stable"
```

## Complete Example 📋

Here's a comprehensive example showing all sections:

```toml
[package]
name = "web-framework"
version = "2.1.0"
authors = ["Jane Doe <jane@example.com>"]
description = "A blazingly fast web framework for CURSED"
license = "MIT OR Apache-2.0"
repository = "https://github.com/example/web-framework"
documentation = "https://docs.example.com/web-framework"
homepage = "https://example.com/web-framework"
readme = "README.md"
keywords = ["web", "framework", "http", "async"]
categories = ["web-programming", "network-programming"]
edition = "2024"
exclude = ["tests/fixtures/*", "benchmarks/data/*"]

[dependencies]
http = "1.0"
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", optional = true }
tls = { version = "2.0", optional = true }

[dev-dependencies]
test-framework = "2.0"
mock-server = "1.5"

[build-dependencies]
code-generator = "1.2"

[features]
default = ["std"]
std = []
async = ["tokio"]
tls-support = ["tls", "crypto"]
full = ["async", "tls-support"]

[[bin]]
name = "web-server"
path = "src/bin/server.csd"
required-features = ["async"]

[lib]
name = "web_framework"
path = "src/lib.csd"

[scripts]
prebuild = "scripts/generate_version.sh"
postbuild = "scripts/validate_build.sh"

[profiles.release]
optimization = "max"
debug = false
strip = true
lto = true

[target.'cfg(unix)'.dependencies]
unix-socket = "1.0"

[package.metadata.docs-rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Validation and Best Practices ✅

### Validation Rules

The package manager validates your `CursedPackage.toml`:

1. **Required fields** must be present
2. **Version** must follow semantic versioning
3. **Package name** must be valid and unique
4. **Dependencies** must exist and be accessible
5. **Features** must reference valid dependencies
6. **File paths** must exist and be readable
7. **URLs** must be valid HTTPS URLs
8. **License** must be valid SPDX identifier or file

### Best Practices

1. **Use semantic versioning** consistently
2. **Write clear descriptions** (10-100 words)
3. **Include comprehensive keywords** for discoverability
4. **Specify appropriate categories** 
5. **Keep dependencies minimal** and up-to-date
6. **Use features** for optional functionality
7. **Document breaking changes** in version updates
8. **Include license information** 
9. **Provide repository and documentation** links
10. **Test with different feature combinations**

### Common Mistakes to Avoid

❌ **Don't**:
- Use generic names like "utils" or "helper"
- Include build artifacts in package
- Forget to update version on changes
- Use unstable dependencies in stable packages
- Include sensitive information in metadata

✅ **Do**:
- Use descriptive, specific names
- Exclude unnecessary files
- Follow semantic versioning
- Pin dependency versions appropriately
- Review generated package before publishing

That's the complete specification for `CursedPackage.toml`! Use this as your reference for creating perfectly configured packages! 📋✨
