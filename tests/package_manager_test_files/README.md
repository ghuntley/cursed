# Package Manager Test Files

This directory contains test files, sample packages, and configuration examples for the CURSED package manager testing infrastructure.

## Directory Structure

```
tests/package_manager_test_files/
├── README.md                           # This file
├── sample_packages/                    # Sample CURSED packages
│   ├── basic_package/                  # Simple package example
│   ├── complex_package/                # Complex package with dependencies
│   ├── circular_deps/                  # Circular dependency examples
│   └── large_framework/                # Large package for performance testing
├── configurations/                    # Package manager configurations
│   ├── default.toml                    # Default configuration
│   ├── custom_registry.toml            # Custom registry configuration
│   └── development.toml                # Development configuration
├── dependency_graphs/                  # Example dependency scenarios
│   ├── simple_chain.json               # Simple dependency chain
│   ├── diamond_dependency.json         # Diamond dependency pattern
│   └── complex_web_app.json            # Complex web application dependencies
└── test_scenarios/                     # Specific test scenarios
    ├── version_conflicts.json           # Version conflict scenarios
    ├── performance_tests.json           # Performance test configurations
    └── error_conditions.json            # Error condition simulations
```

## Sample Packages

### Basic Package (`sample_packages/basic_package/`)
A minimal CURSED package with no dependencies, used for basic functionality testing.

### Complex Package (`sample_packages/complex_package/`)
A package with multiple dependencies, dev dependencies, and various configuration options.

### Circular Dependencies (`sample_packages/circular_deps/`)
Examples of packages that create circular dependency scenarios for testing resolution algorithms.

### Large Framework (`sample_packages/large_framework/`)
A comprehensive package with many dependencies for performance and stress testing.

## Test Configurations

### Default Configuration (`configurations/default.toml`)
Standard package manager configuration for testing.

### Custom Registry (`configurations/custom_registry.toml`)
Configuration for testing with custom package registries.

### Development Configuration (`configurations/development.toml`)
Configuration optimized for development and testing environments.

## Dependency Graph Examples

### Simple Chain (`dependency_graphs/simple_chain.json`)
A linear dependency chain: A → B → C → D

### Diamond Dependency (`dependency_graphs/diamond_dependency.json`)
Diamond pattern: A → B,C → D (where B and C both depend on D)

### Complex Web App (`dependency_graphs/complex_web_app.json`)
Realistic web application dependency graph with multiple frameworks and utilities.

## Test Scenarios

### Version Conflicts (`test_scenarios/version_conflicts.json`)
Predefined scenarios that create version conflicts for testing resolution strategies.

### Performance Tests (`test_scenarios/performance_tests.json`)
Large-scale dependency scenarios for performance benchmarking.

### Error Conditions (`test_scenarios/error_conditions.json`)
Various error conditions for testing error handling and recovery.

## Usage

These test files are automatically used by the package manager test suite. You can also use them manually:

```bash
# Run tests with specific configuration
cargo test package_manager_integration_test -- --test-config tests/package_manager_test_files/configurations/development.toml

# Test with specific dependency graph
cargo test dependency_resolution_test -- --dependency-graph tests/package_manager_test_files/dependency_graphs/complex_web_app.json

# Performance test with large framework
cargo test performance_test -- --package tests/package_manager_test_files/sample_packages/large_framework
```

## Adding New Test Cases

To add new test scenarios:

1. **Sample Packages**: Create a new directory under `sample_packages/` with a valid `CursedPackage.toml`
2. **Configurations**: Add new TOML files under `configurations/` 
3. **Dependency Graphs**: Create JSON files describing dependency relationships
4. **Test Scenarios**: Add JSON files with specific test parameters

### Example Package Structure

```
sample_packages/my_new_package/
├── CursedPackage.toml          # Package metadata
├── src/                        # Source code
│   ├── main.csd               # Main entry point
│   └── lib/                    # Library modules
└── tests/                      # Package tests
    └── basic_test.csd         # Test files
```

### Example CursedPackage.toml

```toml
[package]
name = "my-new-package"
version = "1.0.0"
description = "Example package for testing"
authors = ["Test Author <test@example.com>"]
license = "MIT"
repository = "https://github.com/example/my-new-package"

[dependencies]
web-framework = "^1.0.0"
database-driver = { version = "2.1.0", features = ["async"] }

[dev-dependencies]
test-framework = "1.0.0"
```

## Maintenance

These test files should be updated when:
- New package manager features are added
- Dependency resolution algorithms change
- New error conditions need to be tested
- Performance requirements change

Always ensure that sample packages have valid CURSED syntax and realistic dependency patterns.
