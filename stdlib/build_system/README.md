# CURSED Build System

A complete build system for CURSED projects, written entirely in CURSED itself. This self-hosting build system provides comprehensive project management, dependency resolution, and build orchestration capabilities.

## Features

- **Project Configuration Management**: TOML-based configuration with sensible defaults
- **Dependency Resolution**: Automatic resolution of local and remote dependencies
- **Multi-Target Builds**: Support for multiple build targets and outputs
- **Parallel Builds**: Efficient parallel compilation for faster build times
- **Testing Integration**: Built-in test discovery and execution
- **Package Management**: Install and manage external packages
- **Build Caching**: Intelligent caching to avoid unnecessary rebuilds
- **Watch Mode**: Automatic rebuilds on file changes
- **Multiple Targets**: Native compilation, interpretation, and WebAssembly support

## Installation

The build system is included with the CURSED compiler. To use the CLI tool:

```bash
# Build the CURSED build tool
cargo build --bin cursed_build

# Or use it directly
cargo run --bin cursed_build -- <command>
```

## Configuration

Create a `CursedBuild.toml` file in your project root:

```toml
[project]
name = "my_project"
version = "1.0.0"
authors = ["Your Name"]
description = "My CURSED project"

[build]
targets = ["main", "cli"]
source_dirs = ["src", "lib"]
output_dir = "build"
optimization_level = "2"
parallel_builds = true
cache_enabled = true
build_mode = "native"  # "native", "interpret", or "wasm"

[dependencies]
json_parser = "1.0.0"
http_client = "2.1.0"
crypto = "1.5.0"

[build_dependencies]
code_generator = "1.0.0"

[testing]
test_patterns = ["test_*.💀", "*_test.💀", "tests/*.💀"]
parallel_tests = true
```

## Commands

### Build Project

```bash
cursed_build build
```

Builds all targets specified in the configuration.

### Run Tests

```bash
cursed_build test
```

Discovers and runs all test files matching the configured patterns.

### Clean Build Artifacts

```bash
cursed_build clean
```

Removes all build artifacts and optionally clears the cache.

### Rebuild Project

```bash
cursed_build rebuild
```

Performs a clean followed by a build.

### Package Management

```bash
# Install a package
cursed_build install json_parser 1.0.0

# List installed packages
cursed_build list
```

### Watch Mode

```bash
cursed_build watch
```

Watches for file changes and automatically rebuilds the project.

### Custom Configuration

```bash
cursed_build build --config=custom.toml
```

Use a custom configuration file instead of the default `CursedBuild.toml`.

## Project Structure

The build system expects a standard project structure:

```
my_project/
├── CursedBuild.toml          # Build configuration
├── src/                      # Source files
│   ├── main.💀             # Main target
│   └── lib.💀              # Library code
├── tests/                    # Test files
│   ├── test_main.💀        # Tests for main
│   └── integration_test.💀  # Integration tests
├── build/                    # Build output (generated)
└── docs/                     # Documentation
```

## Build Targets

The build system supports multiple targets:

- **Native Compilation**: Compiles to native executables using LLVM
- **Interpretation**: Runs code directly in the CURSED interpreter
- **WebAssembly**: Compiles to WebAssembly for web deployment

## Dependency Management

Dependencies are resolved in the following order:

1. **Local stdlib**: Check the CURSED standard library
2. **Package cache**: Check the local package cache (`~/.cursed/packages`)
3. **Remote packages**: Download from the package repository

### Dependency Specification

```toml
[dependencies]
# Simple version
json_parser = "1.0.0"

# Git repository
http_client = { git = "https://github.com/cursed/http_client", tag = "v2.1.0" }

# Local path
utils = { path = "../shared_utils" }
```

## Caching

The build system includes intelligent caching:

- **Build Cache**: Skips rebuilding targets that haven't changed
- **Dependency Cache**: Caches resolved dependencies
- **Test Cache**: Caches test results for unchanged files

Cache files are stored in `~/.cursed/cache/`.

## Parallel Builds

Enable parallel builds for faster compilation:

```toml
[build]
parallel_builds = true
```

The build system will automatically determine the optimal number of parallel jobs based on your system's CPU cores.

## Testing

The build system includes a comprehensive testing framework:

### Test Discovery

Tests are automatically discovered based on file patterns:

- `test_*.💀` - Test files starting with "test_"
- `*_test.💀` - Test files ending with "_test"
- `tests/*.💀` - All files in the tests directory

### Test Execution

```bash
# Run all tests
cursed_build test

# Run tests with verbose output
cursed_build test --verbose

# Run specific test patterns
cursed_build test --filter math
```

## Advanced Configuration

### Optimization Levels

```toml
[build]
optimization_level = "0"  # No optimization
optimization_level = "1"  # Basic optimization
optimization_level = "2"  # Standard optimization (default)
optimization_level = "3"  # Aggressive optimization
```

### Custom Build Scripts

```toml
[scripts]
pre_build = "echo 'Starting build...'"
post_build = "echo 'Build complete!'"
pre_test = "echo 'Running tests...'"
post_test = "echo 'Tests complete!'"
```

### Environment Variables

```toml
[env]
DEBUG = "1"
LOG_LEVEL = "info"
```

## API Reference

### Build System Functions

```cursed
# Core build operations
build_project(config_path tea) lit
clean_project(config map[tea]interface{}) lit
rebuild_project(config_path tea) lit

# Configuration management
parse_build_config(config_path tea) map[tea]interface{}
merge_configs(default_config map[tea]interface{}, user_config map[tea]interface{}) map[tea]interface{}

# Dependency management
resolve_dependencies(config map[tea]interface{}) []tea
install_package(name tea, version tea) lit
list_packages() []tea

# Testing
run_tests(config map[tea]interface{}) lit
find_test_files(patterns []tea, source_dirs []tea) []tea

# Caching
check_build_cache(target tea, dependencies []tea) lit
update_build_cache(target tea)

# Build orchestration
build_targets_parallel(targets []tea, config map[tea]interface{}, dependencies []tea) lit
build_single_target(target tea, config map[tea]interface{}, dependencies []tea) lit
```

## Examples

### Simple Project

```toml
# CursedBuild.toml
[project]
name = "hello_world"
version = "1.0.0"

[build]
targets = ["main"]
```

```cursed
# src/main.💀
vibez.spill("Hello, CURSED!")
```

```bash
cursed_build build
./build/main
```

### Library Project

```toml
# CursedBuild.toml
[project]
name = "math_utils"
version = "2.0.0"
type = "library"

[build]
targets = ["lib"]
source_dirs = ["src", "utils"]

[dependencies]
testz = "1.0.0"
```

### Web Application

```toml
# CursedBuild.toml
[project]
name = "web_app"
version = "1.0.0"

[build]
targets = ["server", "client"]
build_mode = "native"

[targets.client]
build_mode = "wasm"
output_dir = "web/static"

[dependencies]
web_framework = "3.0.0"
json_parser = "2.0.0"
```

## Troubleshooting

### Common Issues

1. **Build Fails**: Check that all dependencies are available and the configuration is valid
2. **Tests Not Found**: Ensure test files match the configured patterns
3. **Cache Issues**: Clear the cache with `cursed_build clean --cache`
4. **Permission Errors**: Check file permissions in the project directory

### Debug Mode

```bash
CURSED_DEBUG=1 cursed_build build
```

Enables verbose logging for debugging build issues.

## Contributing

The build system is written in CURSED and can be extended easily:

1. **Add New Commands**: Extend the `build_system_main` function
2. **Custom Build Steps**: Add functions to the build orchestration pipeline
3. **New Target Types**: Extend the target compilation system
4. **Package Sources**: Add new dependency resolution mechanisms

## Performance

The CURSED build system is designed for performance:

- **Parallel Compilation**: Utilizes all available CPU cores
- **Intelligent Caching**: Avoids unnecessary rebuilds
- **Incremental Builds**: Only rebuilds changed components
- **Efficient Dependency Resolution**: Caches dependency information

Typical build times:
- Small projects (< 10 files): < 1 second
- Medium projects (< 100 files): < 10 seconds
- Large projects (< 1000 files): < 60 seconds

## License

The CURSED Build System is part of the CURSED language project and follows the same licensing terms.
