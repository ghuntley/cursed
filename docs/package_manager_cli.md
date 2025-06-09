# Package Manager CLI Reference 🎮

Complete command-line reference for the CURSED package manager (`cursed-pkg`). This guide covers every command, option, and flag available in the package manager.

## Global Options 🌐

These options work with any command:

| Option | Short | Description | Example |
|--------|-------|-------------|---------|
| `--verbose` | `-v` | Enable verbose output | `cursed-pkg -v build` |
| `--quiet` | `-q` | Suppress output except errors | `cursed-pkg -q test` |
| `--color` | | Control color output: auto, always, never | `cursed-pkg --color=never build` |
| `--config` | | Specify config file path | `cursed-pkg --config=dev.toml build` |
| `--manifest-path` | | Path to CursedPackage.toml | `cursed-pkg --manifest-path=sub/CursedPackage.toml build` |
| `--offline` | | Run without network access | `cursed-pkg --offline build` |
| `--help` | `-h` | Show help message | `cursed-pkg --help` |
| `--version` | `-V` | Show version information | `cursed-pkg --version` |

## Package Creation Commands 📦

### `cursed-pkg new`

Create a new package project.

```bash
cursed-pkg new [OPTIONS] <name>
```

**Options:**
- `--bin` Create a binary (application) package
- `--lib` Create a library package (default)
- `--name <name>` Set package name (default: directory name)
- `--edition <edition>` CURSED edition (default: 2024)
- `--vcs <vcs>` Initialize version control: git, hg, pijul, fossil, none

**Examples:**
```bash
# Create new library
cursed-pkg new math-utils

# Create new binary application
cursed-pkg new --bin web-server

# Create without git initialization
cursed-pkg new --vcs none my-tool

# Create with specific edition
cursed-pkg new --edition 2024 future-lib
```

### `cursed-pkg init`

Initialize a package in an existing directory.

```bash
cursed-pkg init [OPTIONS] [path]
```

**Options:**
- `--bin` Create binary package
- `--lib` Create library package (default)
- `--name <name>` Set package name
- `--edition <edition>` CURSED edition
- `--vcs <vcs>` Version control system

**Examples:**
```bash
# Initialize in current directory
cursed-pkg init

# Initialize specific directory
cursed-pkg init ./my-existing-project

# Initialize as binary package
cursed-pkg init --bin --name my-tool
```

## Build Commands 🔨

### `cursed-pkg build`

Compile the current package.

```bash
cursed-pkg build [OPTIONS]
```

**Options:**
- `--release` Build in release mode with optimizations
- `--profile <profile>` Build with specific profile
- `--target <target>` Build for specific target platform
- `--features <features>` Space/comma separated list of features
- `--all-features` Activate all available features
- `--no-default-features` Do not activate default features
- `--package <package>` Build specific package in workspace
- `--workspace` Build all packages in workspace
- `--exclude <package>` Exclude packages from workspace build
- `--jobs <jobs>` Number of parallel jobs (default: # of CPUs)
- `--keep-going` Build remaining packages after failure
- `--offline` Run without accessing network
- `--frozen` Require CursedPackage.lock to be up-to-date
- `--locked` Require CursedPackage.lock to exist and be up-to-date

**Examples:**
```bash
# Basic build
cursed-pkg build

# Release build with optimizations
cursed-pkg build --release

# Build with specific features
cursed-pkg build --features "crypto json"

# Build all features
cursed-pkg build --all-features

# Build for specific target
cursed-pkg build --target wasm32-unknown-unknown

# Build specific package in workspace
cursed-pkg build --package web-utils

# Build entire workspace
cursed-pkg build --workspace

# Parallel build with 8 jobs
cursed-pkg build --jobs 8
```

### `cursed-pkg check`

Check package for errors without building.

```bash
cursed-pkg check [OPTIONS]
```

Uses same options as `build` but doesn't generate output files.

**Examples:**
```bash
# Quick syntax check
cursed-pkg check

# Check with all features
cursed-pkg check --all-features

# Check specific package
cursed-pkg check --package core
```

### `cursed-pkg clean`

Remove build artifacts.

```bash
cursed-pkg clean [OPTIONS]
```

**Options:**
- `--package <package>` Clean specific package
- `--target-dir <dir>` Clean specific target directory
- `--profile <profile>` Clean specific profile
- `--target <target>` Clean specific target

**Examples:**
```bash
# Clean all build artifacts
cursed-pkg clean

# Clean specific package
cursed-pkg clean --package web-utils

# Clean release artifacts only
cursed-pkg clean --profile release
```

## Dependency Management Commands 🔗

### `cursed-pkg add`

Add dependency to package.

```bash
cursed-pkg add [OPTIONS] <package>[@version]
```

**Options:**
- `--dev` Add as development dependency
- `--build` Add as build dependency
- `--optional` Add as optional dependency
- `--no-default-features` Disable default features
- `--features <features>` Enable specific features
- `--rename <name>` Rename the dependency
- `--registry <registry>` Use specific registry
- `--path <path>` Add local path dependency
- `--git <url>` Add git dependency
- `--branch <branch>` Specify git branch
- `--tag <tag>` Specify git tag
- `--rev <rev>` Specify git revision
- `--dry-run` Show what would be added without changing files

**Examples:**
```bash
# Add latest version
cursed-pkg add json-utils

# Add specific version
cursed-pkg add json-utils@2.1.0

# Add with version constraint
cursed-pkg add "json-utils@^2.0.0"

# Add development dependency
cursed-pkg add --dev test-framework

# Add with specific features
cursed-pkg add web-framework --features "async tls"

# Add optional dependency
cursed-pkg add crypto-utils --optional

# Add from git repository
cursed-pkg add experimental --git https://github.com/org/experimental

# Add local path dependency
cursed-pkg add my-lib --path ../my-lib

# Add from private registry
cursed-pkg add private-lib --registry https://registry.company.com

# Rename dependency
cursed-pkg add xml-parser --rename xml
```

### `cursed-pkg remove`

Remove dependency from package.

```bash
cursed-pkg remove [OPTIONS] <package>
```

**Options:**
- `--dev` Remove from dev-dependencies
- `--build` Remove from build-dependencies
- `--dry-run` Show what would be removed

**Examples:**
```bash
# Remove dependency
cursed-pkg remove old-library

# Remove dev dependency
cursed-pkg remove --dev old-test-lib

# Preview removal
cursed-pkg remove --dry-run unused-lib
```

### `cursed-pkg update`

Update dependencies to latest compatible versions.

```bash
cursed-pkg update [OPTIONS] [package]
```

**Options:**
- `--package <package>` Update specific package in workspace
- `--workspace` Update all packages in workspace
- `--aggressive` Update to latest versions (may break compatibility)
- `--dry-run` Show what would be updated
- `--precise <version>` Update to exact version

**Examples:**
```bash
# Update all dependencies
cursed-pkg update

# Update specific dependency
cursed-pkg update json-utils

# Update workspace packages
cursed-pkg update --workspace

# Preview updates
cursed-pkg update --dry-run

# Aggressive update (may break compatibility)
cursed-pkg update --aggressive
```

### `cursed-pkg list`

List installed packages.

```bash
cursed-pkg list [OPTIONS]
```

**Options:**
- `--depth <depth>` Dependency tree depth
- `--format <format>` Output format: human, json, toml
- `--tree` Show as dependency tree

**Examples:**
```bash
# List direct dependencies
cursed-pkg list

# Show dependency tree
cursed-pkg list --tree

# Limit tree depth
cursed-pkg list --tree --depth 2

# JSON output
cursed-pkg list --format json
```

### `cursed-pkg tree`

Display dependency tree.

```bash
cursed-pkg tree [OPTIONS]
```

**Options:**
- `--package <package>` Show tree for specific package
- `--depth <depth>` Maximum display depth
- `--prefix-depth` Show prefix for each depth level
- `--no-dedupe` Don't deduplicate repeated dependencies
- `--duplicates` Show only duplicate dependencies
- `--edges <edges>` Edge kinds: normal, dev, build, all
- `--target <target>` Show dependencies for specific target
- `--format <format>` Output format: human, json

**Examples:**
```bash
# Show full dependency tree
cursed-pkg tree

# Limit depth
cursed-pkg tree --depth 3

# Show duplicates only
cursed-pkg tree --duplicates

# Include dev dependencies
cursed-pkg tree --edges all

# JSON format
cursed-pkg tree --format json
```

## Testing Commands 🧪

### `cursed-pkg test`

Run package tests.

```bash
cursed-pkg test [OPTIONS] [testname]
```

**Options:**
- `--lib` Test library code only
- `--bins` Test binary targets only
- `--examples` Test example code
- `--tests` Test integration tests only
- `--benches` Test benchmark code
- `--all-targets` Test all targets
- `--doc` Test documentation examples
- `--package <package>` Test specific package
- `--workspace` Test all packages in workspace
- `--exclude <package>` Exclude packages from testing
- `--release` Run tests in release mode
- `--features <features>` Test with specific features
- `--all-features` Test with all features
- `--no-default-features` Test without default features
- `--target <target>` Test for specific target
- `--jobs <jobs>` Number of parallel jobs
- `--` Arguments passed to test binary

**Examples:**
```bash
# Run all tests
cursed-pkg test

# Test specific function
cursed-pkg test test_addition

# Test with all features
cursed-pkg test --all-features

# Test specific package
cursed-pkg test --package core

# Test workspace
cursed-pkg test --workspace

# Test in release mode
cursed-pkg test --release

# Pass arguments to test binary
cursed-pkg test -- --nocapture --test-threads=1
```

### `cursed-pkg bench`

Run benchmarks.

```bash
cursed-pkg bench [OPTIONS] [benchname]
```

**Options:**
Similar to `test` command but for benchmarks.

**Examples:**
```bash
# Run all benchmarks
cursed-pkg bench

# Run specific benchmark
cursed-pkg bench sorting_benchmark

# Benchmark with all features
cursed-pkg bench --all-features
```

## Package Information Commands ℹ️

### `cursed-pkg show`

Display package information.

```bash
cursed-pkg show [OPTIONS] <package>
```

**Options:**
- `--registry <registry>` Use specific registry
- `--version <version>` Show specific version
- `--json` Output in JSON format

**Examples:**
```bash
# Show package info
cursed-pkg show json-utils

# Show specific version
cursed-pkg show json-utils --version 2.1.0

# JSON output
cursed-pkg show json-utils --json
```

### `cursed-pkg search`

Search for packages.

```bash
cursed-pkg search [OPTIONS] <query>
```

**Options:**
- `--limit <limit>` Maximum number of results
- `--registry <registry>` Search specific registry
- `--json` Output in JSON format

**Examples:**
```bash
# Search for packages
cursed-pkg search json

# Limit results
cursed-pkg search web --limit 10

# Search private registry
cursed-pkg search utils --registry company
```

### `cursed-pkg info`

Show detailed package metadata.

```bash
cursed-pkg info [OPTIONS] <package>
```

**Examples:**
```bash
# Show detailed info
cursed-pkg info web-framework

# Show as JSON
cursed-pkg info web-framework --json
```

## Publishing Commands 📢

### `cursed-pkg publish`

Publish package to registry.

```bash
cursed-pkg publish [OPTIONS]
```

**Options:**
- `--dry-run` Perform dry run without uploading
- `--allow-dirty` Allow publishing with uncommitted changes
- `--registry <registry>` Publish to specific registry
- `--token <token>` Use specific authentication token
- `--index <index>` Use specific registry index
- `--target <target>` Build for specific target before publishing

**Examples:**
```bash
# Publish to default registry
cursed-pkg publish

# Dry run (preview)
cursed-pkg publish --dry-run

# Publish to private registry
cursed-pkg publish --registry https://registry.company.com

# Allow dirty working directory
cursed-pkg publish --allow-dirty
```

### `cursed-pkg package`

Create package archive without publishing.

```bash
cursed-pkg package [OPTIONS]
```

**Options:**
- `--list` List files included in package
- `--allow-dirty` Allow packaging with uncommitted changes
- `--target-dir <dir>` Directory for output

**Examples:**
```bash
# Create package archive
cursed-pkg package

# List files that would be included
cursed-pkg package --list

# Allow uncommitted changes
cursed-pkg package --allow-dirty
```

### `cursed-pkg yank`

Remove published version from index.

```bash
cursed-pkg yank [OPTIONS] <package> <version>
```

**Options:**
- `--undo` Undo a yank
- `--registry <registry>` Use specific registry
- `--token <token>` Authentication token

**Examples:**
```bash
# Yank version
cursed-pkg yank my-package 1.2.0

# Undo yank
cursed-pkg yank --undo my-package 1.2.0
```

## Registry Commands 🏪

### `cursed-pkg login`

Login to registry.

```bash
cursed-pkg login [OPTIONS] [registry]
```

**Options:**
- `--registry <registry>` Registry URL
- `--token <token>` Authentication token

**Examples:**
```bash
# Login to default registry
cursed-pkg login

# Login to specific registry
cursed-pkg login https://registry.company.com

# Login with token
cursed-pkg login --token abc123def456
```

### `cursed-pkg logout`

Logout from registry.

```bash
cursed-pkg logout [OPTIONS] [registry]
```

**Examples:**
```bash
# Logout from default registry
cursed-pkg logout

# Logout from specific registry
cursed-pkg logout https://registry.company.com
```

### `cursed-pkg whoami`

Show current user.

```bash
cursed-pkg whoami [OPTIONS]
```

**Options:**
- `--registry <registry>` Check specific registry

**Examples:**
```bash
# Show current user
cursed-pkg whoami

# Check specific registry
cursed-pkg whoami --registry company
```

## Workspace Commands 🏢

### `cursed-pkg workspace`

Workspace management commands.

```bash
cursed-pkg workspace <subcommand>
```

#### `cursed-pkg workspace new`

Create new workspace.

```bash
cursed-pkg workspace new <name>
```

#### `cursed-pkg workspace add`

Add package to workspace.

```bash
cursed-pkg workspace add <path>
```

#### `cursed-pkg workspace remove`

Remove package from workspace.

```bash
cursed-pkg workspace remove <package>
```

#### `cursed-pkg workspace list`

List workspace members.

```bash
cursed-pkg workspace list
```

**Examples:**
```bash
# Create workspace
cursed-pkg workspace new my-project

# Add package to workspace
cursed-pkg workspace add packages/core

# Remove package from workspace
cursed-pkg workspace remove core

# List workspace members
cursed-pkg workspace list
```

## Version Management Commands 🏷️

### `cursed-pkg version`

Manage package version.

```bash
cursed-pkg version [OPTIONS] [level]
```

**Options:**
- `--exact` Set exact version

**Version levels:**
- `major` Increment major version (1.0.0 → 2.0.0)
- `minor` Increment minor version (1.0.0 → 1.1.0)
- `patch` Increment patch version (1.0.0 → 1.0.1)

**Examples:**
```bash
# Increment patch version
cursed-pkg version patch

# Increment minor version
cursed-pkg version minor

# Set exact version
cursed-pkg version --exact 2.1.0
```

## Configuration Commands ⚙️

### `cursed-pkg config`

Manage configuration.

```bash
cursed-pkg config <subcommand>
```

#### `cursed-pkg config get`

Get configuration value.

```bash
cursed-pkg config get <key>
```

#### `cursed-pkg config set`

Set configuration value.

```bash
cursed-pkg config set <key> <value>
```

#### `cursed-pkg config unset`

Remove configuration value.

```bash
cursed-pkg config unset <key>
```

#### `cursed-pkg config list`

List all configuration.

```bash
cursed-pkg config list
```

**Examples:**
```bash
# Get registry URL
cursed-pkg config get registry.default

# Set default registry
cursed-pkg config set registry.default https://my-registry.com

# List all config
cursed-pkg config list

# Remove config value
cursed-pkg config unset build.target
```

## Security Commands 🔒

### `cursed-pkg audit`

Audit dependencies for security vulnerabilities.

```bash
cursed-pkg audit [OPTIONS]
```

**Options:**
- `--fix` Automatically update vulnerable dependencies
- `--ignore <advisory>` Ignore specific advisory
- `--format <format>` Output format: human, json
- `--database <db>` Use specific vulnerability database

**Examples:**
```bash
# Basic security audit
cursed-pkg audit

# Auto-fix vulnerabilities
cursed-pkg audit --fix

# JSON format
cursed-pkg audit --format json

# Ignore specific advisory
cursed-pkg audit --ignore CURSED-2024-001
```

### `cursed-pkg verify`

Verify package integrity.

```bash
cursed-pkg verify [OPTIONS] [package]
```

**Options:**
- `--checksum` Verify checksums only
- `--signature` Verify signatures only

**Examples:**
```bash
# Verify all dependencies
cursed-pkg verify

# Verify specific package
cursed-pkg verify json-utils

# Verify checksums only
cursed-pkg verify --checksum
```

## Lock File Commands 🔒

### `cursed-pkg lock`

Generate/update lock file.

```bash
cursed-pkg lock [OPTIONS]
```

**Options:**
- `--update` Update specific dependency

**Examples:**
```bash
# Generate lock file
cursed-pkg lock

# Update specific dependency in lock file
cursed-pkg lock --update json-utils
```

### `cursed-pkg generate-lockfile`

Generate lock file (alias for `lock`).

```bash
cursed-pkg generate-lockfile
```

## Documentation Commands 📚

### `cursed-pkg doc`

Generate documentation.

```bash
cursed-pkg doc [OPTIONS]
```

**Options:**
- `--open` Open documentation in browser
- `--no-deps` Don't build dependencies
- `--package <package>` Document specific package
- `--workspace` Document entire workspace
- `--lib` Document library code only
- `--bins` Document binary targets
- `--target <target>` Build for specific target

**Examples:**
```bash
# Generate docs
cursed-pkg doc

# Generate and open
cursed-pkg doc --open

# Document workspace
cursed-pkg doc --workspace

# Document without dependencies
cursed-pkg doc --no-deps
```

## Utility Commands 🛠️

### `cursed-pkg install`

Install binary from registry.

```bash
cursed-pkg install [OPTIONS] <package>
```

**Options:**
- `--version <version>` Install specific version
- `--git <url>` Install from git
- `--path <path>` Install from local path
- `--bin <name>` Install specific binary
- `--bins` Install all binaries
- `--force` Force installation over existing
- `--target <target>` Install for specific target

**Examples:**
```bash
# Install latest version
cursed-pkg install web-tool

# Install specific version
cursed-pkg install web-tool@2.1.0

# Install from git
cursed-pkg install --git https://github.com/org/tool

# Force reinstall
cursed-pkg install --force web-tool
```

### `cursed-pkg uninstall`

Uninstall installed binary.

```bash
cursed-pkg uninstall <package>
```

### `cursed-pkg run`

Run binary target.

```bash
cursed-pkg run [OPTIONS] [--bin <name>] [-- <args>]
```

**Options:**
- `--bin <name>` Run specific binary
- `--example <name>` Run example
- `--package <package>` Run binary from specific package
- `--release` Run release build
- `--features <features>` Run with features
- `--` Arguments passed to binary

**Examples:**
```bash
# Run default binary
cursed-pkg run

# Run specific binary
cursed-pkg run --bin server

# Run example
cursed-pkg run --example demo

# Pass arguments
cursed-pkg run --bin client -- --host localhost --port 8080

# Run with features
cursed-pkg run --features network --bin server
```

### `cursed-pkg fmt`

Format source code.

```bash
cursed-pkg fmt [OPTIONS]
```

**Options:**
- `--check` Check formatting without changing files
- `--package <package>` Format specific package

**Examples:**
```bash
# Format all code
cursed-pkg fmt

# Check formatting
cursed-pkg fmt --check
```

### `cursed-pkg fix`

Fix lint warnings.

```bash
cursed-pkg fix [OPTIONS]
```

**Options:**
- `--package <package>` Fix specific package
- `--workspace` Fix entire workspace
- `--broken-code` Allow fixing broken code
- `--edition` Apply edition-specific fixes

**Examples:**
```bash
# Fix all warnings
cursed-pkg fix

# Fix workspace
cursed-pkg fix --workspace

# Allow fixing broken code
cursed-pkg fix --broken-code
```

## Output Formats 📊

Many commands support different output formats:

### Human Format (Default)
Easy to read, colorized output for terminal use.

### JSON Format
Machine-readable JSON for scripts and tools:
```bash
cursed-pkg list --format json
cursed-pkg show package --json
cursed-pkg audit --format json
```

### TOML Format
TOML format for configuration compatibility:
```bash
cursed-pkg list --format toml
```

## Exit Codes 🚪

Standard exit codes for scripting:

- **0**: Success
- **1**: General error
- **2**: Invalid usage/arguments
- **101**: Build failed
- **102**: Test failed
- **103**: Dependency resolution failed
- **104**: Network error
- **105**: Authentication failed

## Environment Variables 🌍

Configure behavior with environment variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `CURSED_HOME` | CURSED installation directory | `/usr/local/cursed` |
| `CURSED_REGISTRY` | Default registry URL | `https://registry.company.com` |
| `CURSED_TARGET_DIR` | Build output directory | `./target` |
| `CURSED_HTTP_TIMEOUT` | HTTP timeout in seconds | `30` |
| `CURSED_TERM_COLOR` | Terminal color: auto, always, never | `always` |
| `CURSED_TERM_VERBOSE` | Enable verbose output | `true` |
| `CURSED_BUILD_JOBS` | Number of parallel build jobs | `4` |
| `CURSED_NET_OFFLINE` | Disable network access | `true` |

## Shell Completion 🐚

Generate shell completion scripts:

```bash
# Bash
cursed-pkg completion bash > ~/.local/share/bash-completion/completions/cursed-pkg

# Zsh
cursed-pkg completion zsh > ~/.zfunc/_cursed-pkg

# Fish
cursed-pkg completion fish > ~/.config/fish/completions/cursed-pkg.fish

# PowerShell
cursed-pkg completion powershell > cursed-pkg.ps1
```

That's the complete CLI reference! Every command, option, and flag you need to master the CURSED package manager! 🎮✨
