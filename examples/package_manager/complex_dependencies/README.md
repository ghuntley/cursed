# Complex Dependencies Web Server

An advanced web server demonstrating complex dependency management, optional features, and build configurations in CURSED.

## Features

- **Web Framework**: HTTP server with middleware support
- **Database Integration**: Async database connectivity
- **Authentication** (optional): JWT-based auth with bcrypt
- **Caching** (optional): Redis and in-memory caching
- **Monitoring** (optional): Metrics collection
- **Configuration Management**: TOML-based configuration
- **Structured Logging**: Async logging with multiple levels

## Dependencies Overview

This project demonstrates:

### Runtime Dependencies (8 packages)
- `web_framework`: Core HTTP server functionality
- `database_driver`: Database connectivity
- `json_utils`: JSON serialization/parsing
- `log_manager`: Structured logging
- `config_parser`: Configuration file parsing
- `auth_provider`: Authentication (optional)
- `cache_manager`: Caching layer (optional)  
- `metrics_collector`: Monitoring (optional)

### Development Dependencies (4 packages)
- `test_framework`: Unit and integration testing
- `http_client`: HTTP client for tests
- `mock_server`: Mock services for testing
- `benchmark_suite`: Performance benchmarking

### Build Dependencies (2 packages)
- `api_codegen`: API endpoint code generation
- `migration_generator`: Database migration tools

## Feature Flags

Build with different feature combinations:

```bash
# Basic server (minimal features)
cursed-pkg build

# Development build (auth + caching)
cursed-pkg build --features=dev

# Production build (all features)
cursed-pkg build --features=production

# Specific features
cursed-pkg build --features="auth,monitoring"

# All features
cursed-pkg build --all-features
```

## Installation and Setup

```bash
# Clone the example
git clone https://github.com/cursed-lang/examples
cd examples/package_manager/complex_dependencies

# Install dependencies
cursed-pkg build

# Create configuration file
cp config.example.toml config.toml
# Edit config.toml with your settings

# Run database migrations
cursed-pkg run --bin db-migrate -- --config config.toml

# Start the server
cursed-pkg run --bin web-server -- config.toml
```

## Configuration

Create a `config.toml` file:

```toml
[server]
host = "localhost"
port = 8080

[database]
url = "postgres://user:password@localhost/webserver_demo"

[log]
level = "info"

# Optional: Authentication settings
[auth]
jwt_secret = "your-secret-key-here"

# Optional: Caching settings
[cache]
redis_url = "redis://localhost:6379"
```

## Usage Examples

### Start Basic Server
```bash
cursed-pkg run --bin web-server -- config.toml
```

### Start with Authentication
```bash
cursed-pkg build --features=auth
cursed-pkg run --bin web-server -- config.toml
```

### Start with All Features
```bash
cursed-pkg build --features=full
cursed-pkg run --bin web-server -- config.toml
```

### Run Database Migrations
```bash
cursed-pkg run --bin db-migrate -- --config config.toml
```

## API Endpoints

### Health Check
```http
GET /health
```

Response:
```json
{
    "status": "healthy",
    "timestamp": 1642345678,
    "metrics": {...}  // if monitoring enabled
}
```

### List Users
```http
GET /api/users
Authorization: Bearer <token>  // if auth enabled
```

### Create User
```http
POST /api/users
Authorization: Bearer <token>  // if auth enabled
Content-Type: application/json

{
    "name": "John Doe",
    "email": "john@example.com"
}
```

## Testing

```bash
# Run unit tests
cursed-pkg test

# Run integration tests
cursed-pkg test --test integration

# Run benchmarks
cursed-pkg bench

# Test specific features
cursed-pkg test --features=auth
```

## Build Profiles

Different optimization levels for different scenarios:

```bash
# Development build (fast compilation)
cursed-pkg build

# Release build (optimized)
cursed-pkg build --release

# Production build (size optimized)
cursed-pkg build --profile=production
```

## Dependency Tree

View the complex dependency relationships:

```bash
# Show full dependency tree
cursed-pkg tree

# Show dependencies with features
cursed-pkg tree --features=full

# Show only duplicate dependencies
cursed-pkg tree --duplicates
```

Example output:
```
web-server-demo v0.2.0
├── web_framework v3.1.2
│   ├── http_parser v1.8.0
│   ├── async_runtime v2.1.0
│   └── middleware_core v1.0.0
├── database_driver v2.0.1
│   ├── connection_pool v1.5.0
│   └── query_builder v2.3.0
├── auth_provider v1.8.0 (*)
│   ├── jwt_utils v3.2.0
│   └── bcrypt v0.14.0
└── [dev-dependencies]
    ├── test_framework v2.0.0
    └── http_client v1.3.0
```

## Performance Considerations

This example demonstrates:

### Dependency Impact
- **Binary size**: ~15MB with all features
- **Compilation time**: ~45 seconds with all features
- **Memory usage**: ~50MB base + features

### Optimization Strategies
- Use feature flags to reduce bloat
- Pin critical dependencies to exact versions
- Use build profiles for different scenarios
- Cache dependencies for faster builds

## Learning Objectives

This example teaches:

1. **Complex dependency management** with multiple packages
2. **Optional dependencies** using feature flags
3. **Build-time dependencies** for code generation
4. **Development vs production** dependency separation
5. **Feature-based compilation** for modularity
6. **Build profiles** for optimization
7. **Dependency tree analysis** and conflict resolution
8. **Performance impact** of dependency choices

## Package Manager Commands

### Dependency Management
```bash
# Add runtime dependency
cursed-pkg add web_framework --features="async,middleware"

# Add optional dependency
cursed-pkg add auth_provider --optional --features="jwt"

# Add development dependency
cursed-pkg add test_framework --dev

# Remove dependency
cursed-pkg remove old_dependency

# Update all dependencies
cursed-pkg update

# Update specific dependency
cursed-pkg update web_framework
```

### Feature Management
```bash
# Build with features
cursed-pkg build --features="auth,caching"

# Test with features
cursed-pkg test --features="full"

# List available features
cursed-pkg metadata features
```

### Analysis and Debugging
```bash
# Analyze dependency tree
cursed-pkg tree --depth=3

# Check for security vulnerabilities
cursed-pkg audit

# Verify lock file
cursed-pkg verify

# Show package metadata
cursed-pkg show web_framework
```

## Common Issues and Solutions

### Build Errors
```bash
# Clean and rebuild
cursed-pkg clean
cursed-pkg build

# Check feature requirements
cursed-pkg check --features=full
```

### Dependency Conflicts
```bash
# Show conflicting versions
cursed-pkg tree --duplicates

# Force update to resolve conflicts
cursed-pkg update --aggressive
```

### Performance Issues
```bash
# Build with minimal features
cursed-pkg build --no-default-features --features="basic"

# Use release profile
cursed-pkg build --release
```

## Next Steps

After mastering complex dependencies, explore:

1. **workspace_example** - Multi-package workspace management
2. **performance_benchmark** - Handling many dependencies efficiently
3. **Package publishing** - Publishing your own packages with dependencies
