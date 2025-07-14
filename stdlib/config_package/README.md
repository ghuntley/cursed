# Configuration and Package Management Module

Enterprise-grade configuration and package management module implemented in pure CURSED.

## Features

### Configuration Parsing
- **Multi-format Support**: JSON, YAML, TOML configuration parsing
- **Auto-detection**: Automatic format detection based on file extensions
- **Validation**: Comprehensive configuration validation
- **Type Safety**: Strongly-typed configuration structures

### Package Management
- **Dependency Resolution**: Recursive dependency resolution with circular detection
- **Version Management**: Semantic versioning with compatibility checking
- **Registry Integration**: Package search, installation, and publishing
- **Lock Files**: Deterministic dependency locking

### Build Automation
- **Build Scripts**: Configurable build pipeline execution
- **Test Commands**: Automated test execution
- **CI/CD Integration**: Support for continuous integration workflows

## Data Structures

### Config
```cursed
be_like Config = struct {
    package_name tea,
    version tea,
    description tea,
    author tea,
    dependencies []PackageDep,
    build_scripts []tea,
    test_commands []tea,
    metadata map[tea]tea
}
```

### PackageDep
```cursed
be_like PackageDep = struct {
    name tea,
    version tea,
    source tea,
    dependencies []tea
}
```

### PackageManager
```cursed
be_like PackageManager = struct {
    config Config,
    cache_dir tea,
    registry_url tea,
    installed_packages []PackageDep
}
```

## Core Functions

### Configuration Management
- `parse_json_config(config_data tea) Config` - Parse JSON configuration
- `parse_yaml_config(config_data tea) Config` - Parse YAML configuration
- `parse_toml_config(config_data tea) Config` - Parse TOML configuration
- `load_config_file(file_path tea) Config` - Load configuration from file
- `validate_config(config Config) lit` - Validate configuration structure

### Version Management
- `compare_versions(version1 tea, version2 tea) VersionCompare` - Compare semantic versions
- `is_compatible_version(required tea, available tea) lit` - Check version compatibility
- `get_latest_version(package_name tea) tea` - Get latest available version
- `validate_semver(version tea) lit` - Validate semantic version format

### Dependency Resolution
- `resolve_dependency(dep PackageDep) lit` - Resolve single dependency
- `resolve_dependencies(deps []PackageDep) lit` - Resolve multiple dependencies
- `check_circular_dependencies(deps []PackageDep) lit` - Detect circular dependencies
- `build_dependency_tree(config Config) []PackageDep` - Build dependency tree

### Package Management
- `create_package_manager(config_path tea) PackageManager` - Create package manager
- `install_package(manager *PackageManager, package_name tea, version tea) lit` - Install package
- `uninstall_package(manager *PackageManager, package_name tea) lit` - Uninstall package
- `update_package(manager *PackageManager, package_name tea) lit` - Update package
- `list_installed_packages(manager PackageManager) []tea` - List installed packages

### Build Automation
- `run_build_script(script tea) lit` - Execute build script
- `run_test_command(command tea) lit` - Execute test command
- `execute_build_pipeline(config Config) lit` - Run complete build pipeline

### Registry Operations
- `search_packages(query tea) []tea` - Search package registry
- `publish_package(manager PackageManager) lit` - Publish package to registry
- `get_package_info(package_name tea) PackageDep` - Get package information

### Utilities
- `generate_lock_file(manager PackageManager) tea` - Generate dependency lock file
- `clean_cache(manager *PackageManager) lit` - Clean package cache
- `validate_package_manager(manager PackageManager) lit` - Validate package manager

## Usage Examples

### Basic Configuration Loading
```cursed
yeet "config_package"

# Load configuration from file
sus config Config = load_config_file("project.json")
vibez.spill("Loaded project: " + config.package_name)
vibez.spill("Version: " + config.version)
```

### Package Management
```cursed
yeet "config_package"

# Create package manager
sus manager PackageManager = create_package_manager("cursed.toml")

# Install dependencies
install_package(&manager, "stdlib", "2.5.0")
install_package(&manager, "web-framework", "1.2.0")

# List installed packages
sus packages []tea = list_installed_packages(manager)
bestie i := 0; i < len(packages); i++ {
    vibez.spill("Installed: " + packages[i])
}
```

### Build Pipeline
```cursed
yeet "config_package"

# Load configuration
sus config Config = load_config_file("build.toml")

# Execute build pipeline
if execute_build_pipeline(config) {
    vibez.spill("Build pipeline completed successfully")
} else {
    vibez.spill("Build pipeline failed")
}
```

### Dependency Resolution
```cursed
yeet "config_package"

# Create dependencies
sus deps []PackageDep = []
sus dep PackageDep
dep.name = "cursed-stdlib"
dep.version = "2.5.0"
deps = append(deps, dep)

# Resolve dependencies
if resolve_dependencies(deps) {
    vibez.spill("Dependencies resolved successfully")
} else {
    vibez.spill("Dependency resolution failed")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/config_package/test_config_package.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/config_package/test_config_package.csd
./test_config_package
```

## Implementation Details

- **Pure CURSED**: Implemented without FFI dependencies for maximum portability
- **Memory Safe**: Uses CURSED's garbage collection and type system
- **Enterprise Ready**: Comprehensive error handling and validation
- **Performance Optimized**: Efficient algorithms for dependency resolution
- **Test Coverage**: 60+ comprehensive test cases covering all functionality

## Configuration File Examples

### JSON Configuration (cursed.json)
```json
{
  "package_name": "my-project",
  "version": "1.0.0",
  "description": "My CURSED project",
  "author": "Developer Name",
  "dependencies": [
    {
      "name": "stdlib",
      "version": "2.5.0",
      "source": "registry"
    }
  ],
  "build_scripts": ["make build"],
  "test_commands": ["make test"]
}
```

### YAML Configuration (cursed.yaml)
```yaml
package_name: my-project
version: 1.0.0
description: My CURSED project
author: Developer Name
dependencies:
  - name: stdlib
    version: 2.5.0
    source: registry
build_scripts:
  - make build
test_commands:
  - make test
```

### TOML Configuration (cursed.toml)
```toml
package_name = "my-project"
version = "1.0.0"
description = "My CURSED project"
author = "Developer Name"

[[dependencies]]
name = "stdlib"
version = "2.5.0"
source = "registry"

build_scripts = ["make build"]
test_commands = ["make test"]
```

## Best Practices

1. **Version Pinning**: Pin dependency versions for reproducible builds
2. **Lock Files**: Commit lock files to ensure consistent environments
3. **Semantic Versioning**: Follow semantic versioning for package releases
4. **Circular Dependencies**: Avoid circular dependencies in package design
5. **Build Scripts**: Use idempotent build scripts for reliability
6. **Cache Management**: Regularly clean package cache to free disk space

## Enterprise Features

- **Multi-format Support**: Support for JSON, YAML, and TOML configurations
- **Dependency Resolution**: Advanced dependency resolution with conflict detection
- **Build Automation**: Comprehensive build pipeline automation
- **Registry Integration**: Full package registry integration
- **Lock File Generation**: Deterministic dependency locking
- **Validation**: Comprehensive configuration and package validation
- **Performance**: Optimized for large-scale enterprise deployments
