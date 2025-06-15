# CURSED Build System Documentation

## Overview

The CURSED programming language uses an optimized build system built on top of Cargo and Make, providing comprehensive build, test, and development workflows. The system is designed for performance, reliability, and developer productivity.

## Architecture

### Core Components

1. **Main Makefile** - Primary build system with organized targets
2. **Optimization System** (`Makefile.optimization`) - Advanced optimization and profiling
3. **Linking Fix** (`fix_linking.sh`) - Nix environment compatibility
4. **Build Scripts** (`scripts/`) - Automation and validation tools

### Integration Points

- **Cargo** - Rust build system and package manager
- **LLVM** - Code generation and optimization
- **DevEnv/Nix** - Development environment management
- **CI/CD** - Automated testing and deployment

## Quick Start

### Basic Usage

```bash
# Build the project
make build

# Run tests
make test

# Format code
make fmt

# Check system health
make health-check

# Get help
make help
```

### Development Workflow

```bash
# Complete development cycle
make dev

# Watch for changes and rebuild
make dev-watch

# Run CI pipeline locally
make ci

# Quick validation
make ci-quick
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `VERBOSE` | Enable verbose output | `0` |
| `WORKERS` | Number of parallel workers | auto-detected |
| `BUILD_TYPE` | Build type (debug/release) | `debug` |
| `PROFILE` | Build profile | `dev` |

### Usage Examples

```bash
# Verbose build with 8 workers
make build VERBOSE=1 WORKERS=8

# Release build
make build BUILD_TYPE=release

# Parallel testing
make test WORKERS=4
```

## Target Categories

### Core Build Targets

| Target | Description |
|--------|-------------|
| `build` | Build project in debug mode |
| `build-release` | Build project in release mode |
| `clean` | Clean all build artifacts |
| `check` | Quick syntax and type check |
| `install` | Install compiler system-wide |

### Testing Targets

| Target | Description |
|--------|-------------|
| `test` | Run all tests |
| `test-unit` | Run unit tests only |
| `test-integration` | Run integration tests only |
| `test-ignored` | Run ignored tests |
| `test-coverage` | Generate coverage report |
| `test-filter` | Run filtered tests (requires `TEST_PATTERN`) |
| `test-file` | Run specific test file (requires `TEST_FILE`) |
| `test-name` | Run specific test (requires `TEST_NAME`) |

### Code Quality Targets

| Target | Description |
|--------|-------------|
| `lint` | Run clippy linting |
| `lint-fix` | Fix linting issues automatically |
| `fmt` | Format all source files |
| `fmt-check` | Check formatting without changes |
| `fmt-cursed-fix` | Format CURSED (.csd) files |
| `rust-fmt-fix` | Format Rust files |

### Module Testing

| Target | Description |
|--------|-------------|
| `math-test` | Test all math modules |
| `crypto-test` | Test cryptographic modules |
| `gc-test` | Test garbage collection |
| `collections-test` | Test data structures |
| `type-system-test` | Test type system |

### Development Targets

| Target | Description |
|--------|-------------|
| `dev` | Complete development workflow |
| `dev-watch` | Watch for changes and rebuild |
| `debug` | Build with debug symbols |
| `docs` | Generate documentation |
| `docs-open` | Generate and open docs |
| `example` | Run default example |
| `examples` | List available examples |

### CI/CD Targets

| Target | Description |
|--------|-------------|
| `ci` | Full CI pipeline |
| `ci-quick` | Quick CI validation |
| `validate` | Full validation suite |
| `pre-commit` | Pre-commit hook tasks |
| `health-check` | Check build system health |

### Benchmarking

| Target | Description |
|--------|-------------|
| `bench` | Run default benchmarks |
| `bench-math` | Math function benchmarks |
| `bench-crypto` | Crypto benchmarks |
| `bench-gc` | Garbage collection benchmarks |

## Optimization System

The optimization system provides advanced performance analysis and optimization capabilities.

### Optimization Targets

| Target | Description |
|--------|-------------|
| `opt-build` | Build optimization system |
| `opt-test` | Run optimization tests |
| `opt-analyze` | Performance analysis |
| `opt-profile` | Compilation profiling |
| `opt-benchmark` | Run optimization benchmarks |
| `opt-workflow` | Complete optimization workflow |

### Usage Examples

```bash
# Performance analysis
make opt-analyze OPT_PROFILE=release

# Benchmark with specific optimization level
make opt-benchmark OPT_LEVEL=3

# Complete optimization workflow
make opt-workflow OPT_WORKERS=8

# Profile parsing performance
make opt-profile-parsing

# Cache management
make opt-cache-stats
make opt-cache-clear
```

### Configuration Options

| Variable | Description | Default |
|----------|-------------|---------|
| `OPT_WORKERS` | Optimization workers | auto |
| `OPT_LEVEL` | Optimization level (0-3,s,z) | `2` |
| `OPT_PROFILE` | Profile type | `development` |
| `OPT_SESSION` | Session name | `default` |

## Advanced Features

### Parallel Builds

The build system automatically detects available CPU cores and uses parallel builds where safe:

```bash
# Override worker count
make test WORKERS=16

# Disable parallel builds
make test WORKERS=1
```

### Build Caching

Build artifacts are cached for improved performance:

```bash
# Check cache status
make status

# Clean all caches
make clean-all
```

### Cross-Platform Support

The build system works on Linux, macOS, and Windows:

- **Linux**: Full feature support
- **macOS**: Full feature support with minor path differences
- **Windows**: Core functionality (some advanced features may be limited)

### Nix Environment Integration

For Nix users, the build system integrates with the linking fix:

```bash
# Automatic linking fix usage
make build  # Uses fix_linking.sh automatically

# Direct script usage
./fix_linking.sh cargo build
```

## Troubleshooting

### Common Issues

#### Build Failures

```bash
# Check system health
make health-check

# Verify environment
make status

# Clean and rebuild
make clean build
```

#### Test Failures

```bash
# Run with verbose output
make test VERBOSE=1

# Run specific test
make test-name TEST_NAME=my_test

# Check test infrastructure
make test-unit
```

#### Linking Issues (Nix)

```bash
# Verify linking fix
./fix_linking.sh echo "test"

# Check library paths
echo $LIBRARY_PATH

# Rebuild with linking fix
make clean build
```

### Debug Information

```bash
# System information
make info

# Build system status
make status

# Comprehensive health check
make health-check

# Tool versions
rustc --version
cargo --version
make --version
```

### Getting Help

```bash
# General help
make help

# Optimization help
make opt-help

# Specific module help
make math-help
make crypto-help
```

## Migration Guide

### From Old Build System

If migrating from an older build system:

```bash
# Run migration script
./scripts/migrate_build_system.sh

# Validate migration
make health-check

# Test basic functionality
make build test
```

### Rollback

If issues occur, rollback is possible:

```bash
# Rollback to previous version
./scripts/migrate_build_system.sh --rollback

# Or restore from backup
cp tmp_backup/Makefile.backup.* Makefile
```

## Best Practices

### Development Workflow

1. **Start with health check**: `make health-check`
2. **Use development workflow**: `make dev`
3. **Watch for changes**: `make dev-watch`
4. **Run quick validation**: `make ci-quick`
5. **Full validation before commit**: `make validate`

### Testing Strategy

1. **Run unit tests frequently**: `make test-unit`
2. **Integration tests before commit**: `make test-integration`
3. **Module-specific testing**: `make crypto-test`, `make math-test`
4. **Coverage analysis**: `make test-coverage`

### Performance Optimization

1. **Profile compilation**: `make opt-profile`
2. **Benchmark regularly**: `make opt-benchmark`
3. **Analyze performance**: `make opt-analyze`
4. **Monitor regression**: `make opt-regression`

### Code Quality

1. **Format before commit**: `make fmt`
2. **Fix linting issues**: `make lint-fix`
3. **Check documentation**: `make docs`
4. **Validate CI locally**: `make ci`

## Integration with IDEs

### VS Code

Add to `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "CURSED: Build",
            "type": "shell",
            "command": "make",
            "args": ["build"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always"
            }
        },
        {
            "label": "CURSED: Test",
            "type": "shell",
            "command": "make",
            "args": ["test"],
            "group": "test"
        }
    ]
}
```

### Other IDEs

Most IDEs can be configured to use Make targets. Consult your IDE documentation for specific integration steps.

## Contributing

### Adding New Targets

1. Add target to appropriate section in Makefile
2. Follow naming conventions
3. Include help text with `##` comment
4. Test target functionality
5. Update documentation

### Build System Development

1. Test changes with `--dry-run`
2. Validate with health check
3. Ensure backward compatibility
4. Update tests and documentation

## Support

For build system issues:

1. Run `make health-check` for diagnostics
2. Check `make status` for current state
3. Review documentation and examples
4. Use `make help` for available targets
5. Check GitHub issues for known problems

## Appendix

### Complete Target List

Run `make help` for the most current list of available targets.

### Configuration Reference

See the Makefile header for all configuration variables and their defaults.

### Performance Benchmarks

The optimization system provides detailed performance benchmarks. Run `make opt-benchmark` for current system performance metrics.
