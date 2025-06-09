# Package Publishing Guide 📢

Ready to share your CURSED package with the world? This comprehensive guide covers everything from preparing your package for publication to maintaining it in the registry. Let's make your package absolutely iconic! ✨

## Pre-Publishing Checklist ✅

### Package Quality Standards

Before publishing, ensure your package meets these quality standards:

#### 1. Code Quality
```bash
# Format your code
cursed-fmt --write src/

# Run linter
cursed-lint src/

# Fix any warnings
cursed-pkg fix
```

#### 2. Testing Coverage
```bash
# Run all tests
cursed-pkg test --all-features

# Run benchmarks if available
cursed-pkg bench

# Test examples
cursed-pkg test --examples
```

#### 3. Documentation
```bash
# Generate and check documentation
cursed-pkg doc --open

# Test documentation examples
cursed-pkg test --doc
```

#### 4. Security Audit
```bash
# Check for vulnerabilities
cursed-pkg audit

# Verify package contents
cursed-pkg package --list
```

### Complete Package Metadata

Ensure your `CursedPackage.toml` has complete metadata:

```toml
[package]
name = "my-awesome-package"
version = "1.0.0"
authors = ["Your Name <you@example.com>"]
description = "A clear, concise description of what your package does"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/my-awesome-package"
documentation = "https://docs.yourdomain.com/my-awesome-package"
homepage = "https://yourdomain.com/my-awesome-package"
readme = "README.md"
keywords = ["web", "framework", "async", "http"]
categories = ["web-programming", "development-tools"]
edition = "2024"

# Include only necessary files
include = [
    "src/**/*",
    "examples/**/*",
    "README.md",
    "LICENSE*",
    "CHANGELOG.md"
]

# Exclude unnecessary files
exclude = [
    "tests/fixtures/**/*",
    "benchmarks/data/**/*",
    ".github/**/*",
    "*.tmp",
    "target/**/*"
]
```

### Required Documentation

Your package should include these documentation files:

#### README.md Example
```markdown
# My Awesome Package

A blazingly fast web framework for CURSED that makes building web applications a breeze!

## Features

- 🚀 Lightning-fast performance
- 🔒 Built-in security features
- 🎯 Type-safe routing
- 📦 Minimal dependencies
- 🌐 WebSocket support

## Quick Start

Add to your `CursedPackage.toml`:

```toml
[dependencies]
my-awesome-package = "1.0.0"
```

Basic usage:

```cursed
vibe main

import "my-awesome-package" as web

slay main() {
    sus app = web.new()
    
    app.get("/", |req, res| {
        res.send("Hello, World! 🌍")
    })
    
    app.listen(":8080")
}
```

## Documentation

Full documentation is available at [docs.yourdomain.com](https://docs.yourdomain.com/my-awesome-package).

## Examples

Check out the [examples directory](examples/) for more usage examples.

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md).

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
```

#### CHANGELOG.md Example
```markdown
# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2024-01-15

### Added
- Initial release
- Core web framework functionality
- WebSocket support
- Built-in security middleware

### Changed
- N/A (initial release)

### Fixed
- N/A (initial release)

## [Unreleased]

### Added
- Nothing yet!
```

#### LICENSE Files
Include appropriate license files. For dual-licensing:

**LICENSE-MIT**:
```
MIT License

Copyright (c) 2024 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
```

**LICENSE-APACHE**:
```
Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION...
```

## Version Strategy 📊

### Semantic Versioning

Follow [Semantic Versioning](https://semver.org) strictly:

- **MAJOR** (1.0.0 → 2.0.0): Breaking changes
- **MINOR** (1.0.0 → 1.1.0): New features, backward compatible
- **PATCH** (1.0.0 → 1.0.1): Bug fixes, backward compatible

### Pre-Release Versions

Use pre-release versions for testing:

```bash
# Alpha release (early testing)
cursed-pkg version 1.0.0-alpha.1

# Beta release (feature complete, testing)
cursed-pkg version 1.0.0-beta.1

# Release candidate (ready for production)
cursed-pkg version 1.0.0-rc.1

# Final release
cursed-pkg version 1.0.0
```

### Version Management

```bash
# Increment version automatically
cursed-pkg version patch    # 1.0.0 → 1.0.1
cursed-pkg version minor    # 1.0.0 → 1.1.0
cursed-pkg version major    # 1.0.0 → 2.0.0

# Set specific version
cursed-pkg version 2.1.0

# Pre-release versions
cursed-pkg version 2.0.0-beta.1
```

## Publishing Process 🚀

### Step 1: Registry Setup

#### Create Account
```bash
# Sign up at https://registry.cursed-lang.org
# Then login locally
cursed-pkg login
```

#### Verify Authentication
```bash
# Check you're logged in
cursed-pkg whoami

# Should show your username
```

### Step 2: Pre-Publication Testing

#### Dry Run
```bash
# Preview what will be published
cursed-pkg publish --dry-run

# This shows:
# - Files that will be included
# - Package size
# - Validation results
# - Any warnings or errors
```

#### Package Inspection
```bash
# Create package archive locally
cursed-pkg package

# List included files
cursed-pkg package --list

# Check package size
ls -lh target/package/my-awesome-package-1.0.0.csd
```

### Step 3: Publication

#### First Publication
```bash
# Publish to default registry
cursed-pkg publish

# Publish to specific registry
cursed-pkg publish --registry https://registry.company.com

# Allow dirty working directory (not recommended)
cursed-pkg publish --allow-dirty
```

#### Success Indicators
After successful publication, you'll see:
```
📦 Packaging my-awesome-package v1.0.0 (/path/to/project)
🔍 Verifying my-awesome-package v1.0.0 (/path/to/project)
📤 Uploading my-awesome-package v1.0.0 (/path/to/project)
✅ Published my-awesome-package v1.0.0
🌐 Available at: https://registry.cursed-lang.org/packages/my-awesome-package
```

## Registry Management 🏪

### Default Registry

The official CURSED registry is the default:
- **URL**: `https://registry.cursed-lang.org`
- **Web interface**: `https://cursed-lang.org/packages`
- **Documentation**: Automatically generated at `https://docs.cursed-lang.org/packages/your-package`

### Private Registries

For internal or proprietary packages:

#### Configuration
```bash
# Add private registry
cursed-pkg config set registries.company https://registry.company.com

# Login to private registry
cursed-pkg login --registry company

# Publish to private registry
cursed-pkg publish --registry company
```

#### Registry in CursedPackage.toml
```toml
[dependencies]
# Public package from default registry
json_utils = "2.1.0"

# Private package from company registry
internal_lib = { version = "1.0.0", registry = "company" }
```

### Multiple Registry Strategy

Organize packages across registries:

```toml
# ~/.cursed/config.toml
[registries]
default = "https://registry.cursed-lang.org"
company = "https://registry.company.com"
experimental = "https://dev-registry.company.com"

[registries.company]
token = "company-auth-token"

[registries.experimental]  
token = "dev-auth-token"
```

## Package Maintenance 🔧

### Updating Packages

#### Patch Updates (Bug Fixes)
```bash
# Fix bugs in your code
# Update tests
# Update CHANGELOG.md

# Increment patch version
cursed-pkg version patch

# Publish update
cursed-pkg publish
```

#### Minor Updates (New Features)
```bash
# Add new features
# Ensure backward compatibility
# Add tests for new features
# Update documentation
# Update CHANGELOG.md

# Increment minor version
cursed-pkg version minor

# Publish update
cursed-pkg publish
```

#### Major Updates (Breaking Changes)
```bash
# Make breaking changes
# Update migration guide
# Update examples
# Comprehensive testing
# Update CHANGELOG.md with migration notes

# Increment major version
cursed-pkg version major

# Publish update
cursed-pkg publish
```

### Version Yanking

Remove problematic versions from the index:

```bash
# Yank a broken version
cursed-pkg yank my-package 1.2.3

# Undo a yank (if fixed)
cursed-pkg yank --undo my-package 1.2.3
```

**When to yank:**
- Critical security vulnerabilities
- Data corruption bugs
- Completely broken functionality
- Accidental publication of sensitive data

**Don't yank for:**
- Minor bugs (publish a patch instead)
- Performance issues (publish an improvement)
- API improvements (publish a new version)

### Deprecation Strategy

For breaking changes, provide migration path:

#### 1. Deprecation Release
```cursed
// In version 1.5.0
#[deprecated(since = "1.5.0", note = "Use new_function instead")]
export slay old_function(x normie) normie {
    // Emit deprecation warning
    eprintln!("Warning: old_function is deprecated, use new_function instead")
    cap new_function(x)
}

export slay new_function(x normie) normie {
    // New implementation
}
```

#### 2. Migration Guide
Document the migration path:

```markdown
## Migration Guide: v1.x to v2.0

### Breaking Changes

1. `old_function` has been removed, use `new_function` instead:

```cursed
// Before (v1.x)
let result = my_package.old_function(42)

// After (v2.0)
let result = my_package.new_function(42)
```

2. Configuration format changed:

```toml
# Before (v1.x)
[config]
setting = "value"

# After (v2.0)
[config.settings]
setting = "value"
```
```

#### 3. Breaking Change Release
```bash
# Remove deprecated functionality
# Update major version
cursed-pkg version major
cursed-pkg publish
```

## Quality and Trust Metrics 📊

### Package Statistics

The registry tracks various metrics:

#### Download Statistics
- Total downloads
- Recent downloads (30 days)
- Download trends

#### Quality Metrics
- Documentation completeness
- Test coverage
- Dependency freshness
- Security audit status

#### Trust Indicators
- Publisher verification
- Package signing status
- Community ratings
- Maintenance activity

### Improving Package Ranking

#### 1. Documentation Quality
```bash
# Comprehensive documentation
cursed-pkg doc --all-features

# Include examples in docs
cursed-pkg test --doc

# Keep README updated
```

#### 2. Active Maintenance
- Regular updates
- Quick issue responses
- Security patch releases
- Dependency updates

#### 3. Community Engagement
- Respond to issues promptly
- Accept valuable contributions
- Provide support channels
- Maintain compatibility

#### 4. Testing and Quality
```bash
# Comprehensive test suite
cursed-pkg test --all-features

# Regular security audits
cursed-pkg audit

# Code quality tools
cursed-fmt --check
cursed-lint
```

## Advanced Publishing Topics 🎯

### Automated Publishing

#### GitHub Actions
```yaml
# .github/workflows/publish.yml
name: Publish Package
on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup CURSED
        uses: cursed-lang/setup-cursed@v1
        
      - name: Build and test
        run: |
          cursed-pkg build --release
          cursed-pkg test --all-features
          
      - name: Publish to registry
        run: cursed-pkg publish
        env:
          CURSED_TOKEN: ${{ secrets.CURSED_TOKEN }}
```

#### Release Automation
```bash
# Create release script
#!/bin/bash
# release.sh

set -e

VERSION=${1:-patch}

echo "🚀 Starting release process..."

# Update version
cursed-pkg version $VERSION
NEW_VERSION=$(grep '^version = ' CursedPackage.toml | sed 's/version = "\(.*\)"/\1/')

echo "📝 Version updated to $NEW_VERSION"

# Update changelog
echo "## [$NEW_VERSION] - $(date +%Y-%m-%d)" >> CHANGELOG.tmp
echo "" >> CHANGELOG.tmp
echo "### Added" >> CHANGELOG.tmp
echo "- " >> CHANGELOG.tmp
echo "" >> CHANGELOG.tmp
cat CHANGELOG.md >> CHANGELOG.tmp
mv CHANGELOG.tmp CHANGELOG.md

echo "📋 Please update CHANGELOG.md with release notes"
read -p "Press enter when changelog is updated..."

# Commit and tag
git add .
git commit -m "Release v$NEW_VERSION"
git tag "v$NEW_VERSION"

# Build and test
cursed-pkg build --release
cursed-pkg test --all-features

# Publish
cursed-pkg publish

# Push to git
git push origin main
git push origin "v$NEW_VERSION"

echo "✅ Release v$NEW_VERSION completed!"
```

### Cross-Platform Publishing

#### Multiple Targets
```toml
# CursedPackage.toml
[package.metadata.targets]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc", 
    "x86_64-apple-darwin",
    "aarch64-apple-darwin"
]
```

#### Build Matrix
```yaml
# .github/workflows/build-all.yml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    target: 
      - x86_64-unknown-linux-gnu
      - x86_64-pc-windows-msvc
      - x86_64-apple-darwin
      - aarch64-apple-darwin
```

### Package Signing and Verification

#### Package Signing
```bash
# Generate signing key
cursed-pkg key generate --name "My Package Key"

# Sign package during publish
cursed-pkg publish --sign

# Verify signed package
cursed-pkg verify my-package@1.0.0 --signature
```

#### Trust Chain
```bash
# Trust a publisher's key
cursed-pkg trust add publisher@cursed-lang.org

# Verify package with trusted keys only
cursed-pkg install my-package --verify-trust
```

## Troubleshooting Publishing 🔧

### Common Publishing Errors

#### Authentication Errors
```bash
# Error: Authentication failed
# Solution: Re-login to registry
cursed-pkg logout
cursed-pkg login

# Check token validity
cursed-pkg whoami
```

#### Package Size Errors
```bash
# Error: Package too large (>10MB)
# Solution: Exclude unnecessary files
echo "target/" >> .cursedignore
echo "*.tmp" >> .cursedignore
echo "test-data/" >> .cursedignore

# Check package size
cursed-pkg package --list | wc -l
```

#### Validation Errors
```bash
# Error: Missing required metadata
# Solution: Complete CursedPackage.toml
[package]
description = "Add a clear description"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/repo"

# Error: Invalid version
# Solution: Use semantic versioning
version = "1.0.0"  # Not "v1.0" or "1.0"
```

#### Dependency Errors
```bash
# Error: Circular dependency detected
# Solution: Restructure dependencies or use features

# Error: Dependency not found
# Solution: Verify dependency names and versions
cursed-pkg search dependency-name
```

### Publishing Checklist ✅

Before each publish:

- [ ] Code is formatted (`cursed-fmt --check`)
- [ ] No lint warnings (`cursed-lint`)
- [ ] All tests pass (`cursed-pkg test --all-features`)
- [ ] Documentation is up-to-date (`cursed-pkg doc`)
- [ ] CHANGELOG.md is updated
- [ ] Version is incremented appropriately
- [ ] No sensitive data in package (`cursed-pkg package --list`)
- [ ] Dependencies are up-to-date (`cursed-pkg audit`)
- [ ] Package builds cleanly (`cursed-pkg build --release`)
- [ ] Examples work (`cursed-pkg test --examples`)

## Package Ecosystem Best Practices 🌟

### Naming Conventions

#### Package Names
- Use lowercase with underscores: `web_utils`, `json_parser`
- Be descriptive: `http_client` not `client`
- Avoid generic names: `string_utils` not `utils`
- Consider namespacing: `company_web_utils`

#### Example Names
```toml
# ✅ Good names
json_parser = "1.0.0"      # Clear purpose
http_client = "2.1.0"      # Specific functionality
cursed_web = "1.5.0"       # Language-specific prefix

# ❌ Avoid these names
utils = "1.0.0"            # Too generic
helper = "2.0.0"           # Unclear purpose
lib = "1.0.0"              # Meaningless
```

### Compatibility Strategy

#### Edition Compatibility
```toml
[package]
edition = "2024"           # Use latest stable edition

# Support multiple editions if needed
[package.metadata.editions]
supported = ["2024", "2023"]
minimum = "2023"
```

#### Feature Flag Strategy
```toml
[features]
default = ["std"]

# Stable features
std = []
json = ["serde_json"]
http = ["reqwest"]

# Experimental features (clearly marked)
experimental_async = []
unstable_simd = []

# Version compatibility
compat_v1 = []             # v1.x compatibility mode
```

### Community Guidelines

#### Issue Management
- Use issue templates
- Label issues appropriately
- Respond within reasonable time
- Provide reproduction steps
- Document common problems

#### Contributing Guidelines
Create CONTRIBUTING.md:

```markdown
# Contributing to My Awesome Package

## Development Setup

1. Clone the repository
2. Install dependencies: `cursed-pkg build`
3. Run tests: `cursed-pkg test`

## Submitting Changes

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Code Style

- Use `cursed-fmt` for formatting
- Run `cursed-lint` before submitting
- Follow existing code patterns
- Add documentation for public APIs

## Release Process

Only maintainers can publish releases. We use semantic versioning.
```

That's everything you need to know about publishing CURSED packages! Your packages are about to be absolutely iconic in the registry! 📢✨

For more technical details, check out the [Registry Protocol Specification](package_registry_protocol.md) and [Package Manager API Documentation](package_manager_api.md).
