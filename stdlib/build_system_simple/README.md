# build_system_simple

A simple yet comprehensive build system module for CURSED projects. Provides essential build, test, and package management functionality with a clean command-line interface.

## Overview

The `build_system_simple` module provides core functionality for:
- Building CURSED projects from configuration
- Running tests and validation
- Managing project dependencies
- Package installation and management
- Project cleanup and rebuilding

## Core Functions

### Project Management

#### `create_simple_config() -> map[tea]interface{}`
Creates a default build configuration with standard settings:
- Project name: "simple_project"
- Version: "1.0.0"
- Source directories: ["src"]
- Output directory: "build"
- Targets: ["main"]

#### `parse_config(config_path: tea) -> map[tea]interface{}`
Parses build configuration from a TOML file. Currently returns default config for demo purposes.

**Parameters:**
- `config_path`: Path to the configuration file

**Returns:** Configuration map with project settings

### Build Operations

#### `build_project_simple(config_path: tea) -> lit`
Builds a CURSED project using the specified configuration.

**Parameters:**
- `config_path`: Path to the build configuration file

**Returns:** `based` if build succeeds, `cringe` if it fails

#### `clean_project_simple(config: map[tea]interface{}) -> lit`
Cleans build artifacts from the project output directory.

**Parameters:**
- `config`: Project configuration map

**Returns:** `based` when cleanup completes

#### `run_tests_simple(config: map[tea]interface{}) -> lit`
Executes the project test suite.

**Parameters:**
- `config`: Project configuration map

**Returns:** `based` if all tests pass, `cringe` if any fail

### Package Management

#### `install_package_simple(name: tea, version: tea) -> lit`
Installs a package with the specified version.

**Parameters:**
- `name`: Package name to install
- `version`: Version specification

**Returns:** `based` if installation succeeds

#### `list_packages_simple() -> []tea`
Lists all installed packages in the current environment.

**Returns:** Array of package names

### Command Line Interface

#### `build_system_main_simple(args: []tea) -> normie`
Main entry point for the build system command-line interface.

**Supported Commands:**
- `build`: Build the project
- `test`: Run tests
- `clean`: Clean build artifacts
- `rebuild`: Clean and build
- `install <package> <version>`: Install a package
- `list`: List installed packages

**Parameters:**
- `args`: Command line arguments

**Returns:** Exit code (0 for success, 1 for failure)

## Usage Examples

### Basic Build Workflow

```cursed
yeet "build_system_simple"

// Build a project
sus success lit = build_project_simple("CursedBuild.toml")
lowkey success {
    vibez.spill("Build completed successfully!")
} yikes {
    vibez.spill("Build failed!")
}

// Run tests
sus config map[tea]interface{} = parse_config("CursedBuild.toml")
sus test_success lit = run_tests_simple(config)
```

### Package Management

```cursed
// Install a package
install_package_simple("stringz", "1.0.0")

// List all packages
sus packages []tea = list_packages_simple()
bestie _, package := iterate packages {
    vibez.spill("Installed: " + package)
}
```

### Command Line Usage

```bash
# Build the project
cursed_build build

# Run tests
cursed_build test

# Clean and rebuild
cursed_build rebuild

# Install a package
cursed_build install testz 1.0.0

# List packages
cursed_build list
```

## Configuration

The build system expects a `CursedBuild.toml` configuration file with the following structure:

```toml
name = "my_project"
version = "1.0.0"
targets = ["main"]
source_dirs = ["src"]
output_dir = "build"
```

## Testing

The module includes comprehensive test coverage in `test_build_system_simple.csd`:

```bash
# Run module tests
zig build test
./zig-out/bin/cursed-zig stdlib/build_system_simple/test_build_system_simple.csd
```

## Dependencies

- `testz`: For testing framework
- `vibez`: For output and logging

## Architecture

The build system follows a simple configuration-driven approach:

1. **Configuration Parsing**: Reads project settings from TOML files
2. **Build Pipeline**: Executes build steps based on configuration
3. **Test Integration**: Runs tests as part of the build process
4. **Package Management**: Handles dependency installation and tracking
5. **CLI Interface**: Provides user-friendly command-line access

## Error Handling

All functions return appropriate status codes:
- `based`: Operation succeeded
- `cringe`: Operation failed
- Exit codes: 0 for success, 1 for failure

Error messages are displayed to help diagnose build issues.

## Extension Points

The build system can be extended with:
- Custom build targets
- Additional package repositories
- Integration with external tools
- Advanced configuration options
- Plugin system support

## Performance

- Fast configuration parsing
- Incremental build support (planned)
- Parallel test execution (planned)
- Efficient package caching

## Compatibility

- Works with all CURSED project structures
- Compatible with standard TOML configuration files
- Cross-platform build support
- Integration with existing toolchains
