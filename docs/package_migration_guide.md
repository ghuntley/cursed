# Package Manager Migration Guide 🚀

This guide helps you migrate existing CURSED projects to use the new package manager system. Whether you're converting from manual dependency management or upgrading from an older system, this guide covers all migration scenarios.

## Migration Overview 📋

### What Changes

| Before | After |
|--------|-------|
| Manual import paths | Package-based imports |
| Manual dependency management | Automatic resolution |
| Ad-hoc build scripts | Standardized build system |
| Inconsistent versioning | Semantic versioning |
| No dependency locking | Lock files for reproducibility |

### Migration Benefits

- **Automated dependency resolution** - No more manual version juggling
- **Reproducible builds** - Lock files ensure consistent environments
- **Security auditing** - Automatic vulnerability scanning
- **Simplified distribution** - Package publishing and discovery
- **Better tooling** - Integrated build, test, and documentation tools

## Pre-Migration Assessment 🔍

### Inventory Your Project

Before starting migration, assess your current project:

#### 1. Identify External Dependencies

```bash
# Find all import statements
grep -r "import" src/ --include="*.csd"

# Common patterns to look for:
# import "./local/path"
# import "../relative/path"  
# import "/absolute/path"
# import "git://github.com/user/repo"
```

#### 2. Analyze Project Structure

```bash
# Document current structure
find . -name "*.csd" -type f | head -20

# Look for:
# - Source code organization
# - Build scripts and tools
# - Configuration files
# - Documentation
# - Test files
```

#### 3. Check Version Information

```bash
# Look for version information
grep -r "version" . --include="*.toml" --include="*.json" --include="*.yaml"

# Document:
# - Current project version
# - Dependency versions (if any)
# - Compatibility requirements
```

### Migration Readiness Checklist

- [ ] Project uses CURSED 2024 edition or later
- [ ] All source code is in a predictable structure
- [ ] Dependencies are identifiable and available as packages
- [ ] No hardcoded absolute paths in imports
- [ ] Version control is set up (Git recommended)
- [ ] Backup of current project created

## Migration Scenarios 🛤️

### Scenario 1: Simple Project with Local Modules

**Before:** Project with local modules using relative imports

```
my-project/
├── main.csd
├── utils/
│   ├── math.csd
│   └── string.csd
└── helpers/
    └── file.csd
```

**main.csd:**
```cursed
vibe main

import "./utils/math"
import "./utils/string" 
import "./helpers/file"

slay main() {
    printfr("Hello from {}", math.add(1, 2))
}
```

#### Migration Steps

1. **Initialize package structure:**
```bash
cd my-project
cursed-pkg init --name my-project --lib
```

2. **Reorganize source code:**
```bash
mkdir -p src
mv main.csd src/
mv utils src/
mv helpers src/
```

3. **Update imports to use modules:**

**src/main.csd:**
```cursed
vibe main

// Local modules now use module paths
import "utils/math"
import "utils/string"
import "helpers/file"

slay main() {
    printfr("Hello from {}", math.add(1, 2))
}
```

4. **Create package manifest:**

**CursedPackage.toml:**
```toml
[package]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "My CURSED project"
edition = "2024"

[[bin]]
name = "my-project"
path = "src/main.csd"
```

5. **Test the migration:**
```bash
cursed-pkg build
cursed-pkg run
```

### Scenario 2: Project with External Dependencies

**Before:** Project manually managing external dependencies

```
my-web-app/
├── main.csd
├── vendor/
│   ├── json_parser/
│   ├── http_client/
│   └── template_engine/
└── src/
    └── app.csd
```

**main.csd:**
```cursed
vibe main

import "./vendor/json_parser"
import "./vendor/http_client"
import "./vendor/template_engine"
import "./src/app"

slay main() {
    app.run()
}
```

#### Migration Steps

1. **Initialize package:**
```bash
cd my-web-app
cursed-pkg init --name my-web-app --bin
```

2. **Add external dependencies:**
```bash
# Replace vendor directory with package dependencies
cursed-pkg add json_parser@2.1.0
cursed-pkg add http_client@1.5.0
cursed-pkg add template_engine@3.0.0
```

3. **Remove vendor directory:**
```bash
rm -rf vendor/
```

4. **Update imports:**

**src/main.csd:**
```cursed
vibe main

// External packages use package names
import "json_parser"
import "http_client" 
import "template_engine"
import "./app"  // Local modules still use relative paths

slay main() {
    app.run()
}
```

5. **Update package manifest:**

**CursedPackage.toml:**
```toml
[package]
name = "my-web-app"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "My web application"
edition = "2024"

[dependencies]
json_parser = "2.1.0"
http_client = "1.5.0"
template_engine = "3.0.0"

[[bin]]
name = "my-web-app"
path = "src/main.csd"
```

### Scenario 3: Multi-Module Project to Workspace

**Before:** Large project with multiple related components

```
my-platform/
├── core/
├── web-ui/
├── api-server/
├── cli-tools/
└── shared-utils/
```

#### Migration Steps

1. **Create workspace structure:**
```bash
cd my-platform
cursed-pkg workspace new .
```

2. **Convert each component to a package:**
```bash
# Convert core library
cd core
cursed-pkg init --lib --name platform-core

# Convert web UI
cd ../web-ui
cursed-pkg init --bin --name platform-web

# Convert API server
cd ../api-server  
cursed-pkg init --bin --name platform-api

# Convert CLI tools
cd ../cli-tools
cursed-pkg init --bin --name platform-cli

# Convert shared utilities
cd ../shared-utils
cursed-pkg init --lib --name platform-utils
```

3. **Create workspace manifest:**

**CursedWorkspace.toml:**
```toml
[workspace]
members = [
    "core",
    "web-ui", 
    "api-server",
    "cli-tools",
    "shared-utils"
]

# Shared dependencies
[workspace.dependencies]
json_utils = "2.1.0"
log_manager = "1.5.0"
config_parser = "1.2.0"
```

4. **Update cross-package dependencies:**

In each package's `CursedPackage.toml`:
```toml
[dependencies]
# Use workspace member as dependency
platform-core = { path = "../core" }
platform-utils = { path = "../shared-utils" }

# Use workspace shared dependencies
json_utils = { workspace = true }
log_manager = { workspace = true }
```

5. **Build entire workspace:**
```bash
cursed-pkg workspace build
```

## Import Syntax Migration 📝

### Old vs New Import Patterns

#### File-Based Imports → Package Imports

```cursed
// ❌ Old: File-based imports
import "./vendor/math_utils/src/lib"
import "../../shared/utils"
import "/absolute/path/to/module"

// ✅ New: Package-based imports  
import "math_utils"
import "shared_utils"
import "absolute_module"
```

#### Relative Imports → Module Imports

```cursed
// ❌ Old: Deep relative paths
import "../../../utils/string/operations"
import "../../common/error_handling"

// ✅ New: Clean module paths
import "utils/string/operations"
import "common/error_handling"
```

#### Git URL Imports → Package Names

```cursed
// ❌ Old: Git URL imports
import "git://github.com/user/awesome-lib#v1.2.0"
import "https://github.com/company/internal-lib.git"

// ✅ New: Package names with versions in manifest
import "awesome_lib"        // Version in CursedPackage.toml
import "internal_lib"       // Version in CursedPackage.toml
```

### Import Migration Tool

Create a script to help migrate imports:

```bash
#!/bin/bash
# migrate-imports.sh - Automated import migration

echo "🔄 Migrating import statements..."

# Replace file-based imports with package imports
find src/ -name "*.csd" -exec sed -i \
    's|import "./vendor/\([^/]*\)/.*"|import "\1"|g' {} \;

# Replace relative imports with module imports  
find src/ -name "*.csd" -exec sed -i \
    's|import "\.\./\.\./\([^"]*\)"|import "\1"|g' {} \;

# Replace absolute imports
find src/ -name "*.csd" -exec sed -i \
    's|import "/[^"]*src/\([^"]*\)"|import "\1"|g' {} \;

echo "✅ Import migration complete"
echo "🔍 Review changes and test builds"
```

## Configuration Migration ⚙️

### Build Configuration

#### Old: Custom Build Scripts

```bash
#!/bin/bash
# build.sh (old approach)
cursed-compile --optimize --target=release src/main.csd -o bin/app
cursed-compile --target=debug src/test.csd -o bin/test
```

#### New: Package Manager Profiles

**CursedPackage.toml:**
```toml
[profiles.dev]
optimization = "none"
debug = true

[profiles.release]  
optimization = "max"
debug = false
strip = true

[profiles.test]
optimization = "fast"
debug = true
```

**Usage:**
```bash
# Instead of custom scripts
cursed-pkg build --profile=release
cursed-pkg test
```

### Dependency Configuration

#### Old: Manual Dependency Tracking

```text
# dependencies.txt (old approach)
json_parser v2.1.0 from github.com/user/json_parser
http_client v1.5.0 from gitlab.com/team/http_client  
math_utils v1.0.0 from local path ../math_utils
```

#### New: Declarative Dependencies

**CursedPackage.toml:**
```toml
[dependencies]
json_parser = "2.1.0"
http_client = { git = "https://gitlab.com/team/http_client", tag = "v1.5.0" }
math_utils = { path = "../math_utils" }
```

## Version Management Migration 📊

### Establishing Initial Version

Choose appropriate starting version:

```bash
# For new projects
cursed-pkg version 0.1.0

# For existing stable projects
cursed-pkg version 1.0.0

# For experimental projects
cursed-pkg version 0.0.1
```

### Versioning Strategy

#### Pre-Migration Version Audit

1. **Document current state:**
```bash
echo "# Pre-migration version inventory" > migration-versions.md
echo "Project: $(basename $(pwd))" >> migration-versions.md
echo "Date: $(date)" >> migration-versions.md
echo "" >> migration-versions.md

# Document any existing version info
git tag -l >> migration-versions.md
grep -r "version" . --include="*.toml" >> migration-versions.md
```

2. **Establish semantic versioning:**
```toml
[package]
version = "1.0.0"  # Stable API
# or
version = "0.1.0"  # Development API
```

### Lock File Migration

#### Generate Initial Lock File

```bash
# Create initial lock file
cursed-pkg lock

# Verify lock file is valid
cursed-pkg verify

# Commit lock file
git add CursedPackage.lock
git commit -m "Add initial dependency lock file"
```

## Testing Migration 🧪

### Migration Testing Strategy

#### 1. Parallel Build Testing

```bash
# Test old build system
./old-build.sh

# Test new package manager
cursed-pkg build

# Compare outputs
diff old-output/ target/debug/
```

#### 2. Dependency Resolution Testing

```bash
# Verify all dependencies resolve
cursed-pkg check

# Test dependency tree
cursed-pkg tree

# Check for conflicts
cursed-pkg tree --duplicates
```

#### 3. Feature Parity Testing

```bash
# Test all build profiles
cursed-pkg build --profile=dev
cursed-pkg build --profile=release
cursed-pkg build --profile=test

# Test all features (if using features)
cursed-pkg build --all-features
cursed-pkg build --no-default-features
```

### Migration Validation Checklist

- [ ] All source files compile successfully
- [ ] All tests pass with new build system
- [ ] Dependencies resolve without conflicts
- [ ] Build artifacts are functionally equivalent
- [ ] Performance is comparable or better
- [ ] All team members can build successfully
- [ ] CI/CD pipeline works with new system
- [ ] Documentation is updated

## Team Migration Strategy 👥

### Coordinated Migration Plan

#### Phase 1: Preparation (1-2 weeks)
1. **Assessment and planning**
   - Inventory current dependencies
   - Plan new project structure
   - Identify migration risks

2. **Tool setup**
   - Install package manager on all dev machines
   - Update CI/CD with package manager
   - Create migration documentation

3. **Pilot migration**
   - Migrate one small project first
   - Test the process and tools
   - Refine migration procedures

#### Phase 2: Migration (1-3 weeks)
1. **Create migration branch**
```bash
git checkout -b migrate-to-package-manager
```

2. **Execute migration**
   - Follow project-specific migration steps
   - Update all team documentation
   - Test thoroughly

3. **Team validation**
   - Have each team member test build
   - Validate CI/CD pipeline
   - Update development workflows

#### Phase 3: Completion (1 week)
1. **Merge and deploy**
```bash
# After thorough testing
git checkout main
git merge migrate-to-package-manager
```

2. **Update documentation**
   - Update README with new build instructions
   - Update contributing guidelines
   - Update deployment procedures

3. **Team training**
   - Package manager training session
   - Updated development guidelines
   - Q&A and troubleshooting session

### Communication Plan

#### Migration Announcement

```markdown
# 📦 Migration to CURSED Package Manager

We're migrating our project to use the new CURSED package manager for better dependency management and build processes.

## Timeline
- **Preparation**: Jan 1-14
- **Migration**: Jan 15-Feb 5  
- **Completion**: Feb 6-12

## What Changes
- Dependency management now handled automatically
- New build commands: `cursed-pkg build` instead of `./build.sh`
- Consistent versioning across all projects
- Simplified setup for new team members

## Action Required
1. Install CURSED package manager: [link to instructions]
2. Read migration guide: [link to this document]
3. Test pilot project: [link to pilot project]

## Support
- Slack channel: #package-manager-migration
- Office hours: Tuesdays 2-3pm
- Contact: dev-team@company.com
```

## Troubleshooting Common Issues 🔧

### Import Resolution Errors

**Error:** `Cannot resolve import "old/path"`
```cursed
// ❌ Problem
import "./vendor/json_utils/lib"
```

**Solution:** Update to package import
```cursed
// ✅ Solution
import "json_utils"
```

Add to `CursedPackage.toml`:
```toml
[dependencies]
json_utils = "2.1.0"
```

### Build Script Conflicts

**Error:** `Multiple build systems detected`

**Solution:** Remove old build scripts
```bash
# Remove old build artifacts
rm -rf build/
rm build.sh Makefile

# Use package manager exclusively
cursed-pkg build
```

### Version Conflicts

**Error:** `Dependency version conflict`

**Solution:** Check and resolve version constraints
```bash
# Identify conflicts
cursed-pkg tree --duplicates

# Update constraints in CursedPackage.toml
[dependencies]
package_a = "^2.0.0"  # Instead of exact version
package_b = "^1.5.0"  # Compatible with package_a's requirements
```

### Performance Regression

**Issue:** Slower build times after migration

**Solutions:**
```bash
# Enable parallel builds
cursed-pkg build --jobs 8

# Use appropriate profile
cursed-pkg build --profile=dev  # Faster for development

# Check dependency tree depth
cursed-pkg tree --depth=5

# Consider reducing dependencies
cursed-pkg audit --unused
```

### CI/CD Integration Issues

**Issue:** CI/CD pipeline fails with package manager

**Solution:** Update CI configuration
```yaml
# .github/workflows/build.yml
- name: Setup CURSED
  uses: cursed-lang/setup-cursed@v1
  
- name: Cache dependencies
  uses: actions/cache@v3
  with:
    path: ~/.cursed/cache
    key: cursed-${{ hashFiles('CursedPackage.lock') }}
    
- name: Build project
  run: cursed-pkg build --locked
```

## Post-Migration Best Practices ✅

### Dependency Hygiene

```bash
# Regular dependency maintenance
cursed-pkg audit         # Security check
cursed-pkg outdated      # Check for updates
cursed-pkg clean         # Clean build cache
```

### Lock File Management

```bash
# Always commit lock files
git add CursedPackage.lock
git commit -m "Update dependency lock file"

# Use locked builds in CI
cursed-pkg build --locked
```

### Documentation Updates

Update all project documentation:

- **README.md**: New build instructions
- **CONTRIBUTING.md**: Package manager workflow
- **DEPLOYMENT.md**: New deployment process
- **API.md**: Updated import examples

### Team Workflow

Establish new team practices:

```bash
# Daily workflow
cursed-pkg check        # Quick validation
cursed-pkg test         # Run tests
cursed-pkg build        # Build project

# Weekly maintenance
cursed-pkg audit        # Security check
cursed-pkg update       # Update dependencies

# Before releases
cursed-pkg build --release --all-features
cursed-pkg test --all-features
```

## Migration Success Metrics 📈

Track these metrics to measure migration success:

### Development Metrics
- **Build time**: Before vs after migration
- **Setup time**: New developer onboarding time
- **Error rate**: Build failures and dependency issues
- **Developer satisfaction**: Team feedback

### Project Metrics  
- **Dependency count**: Number of dependencies managed
- **Security issues**: Vulnerabilities found and fixed
- **Update frequency**: How often dependencies are updated
- **Binary size**: Impact on output size

### Maintenance Metrics
- **Time to resolve conflicts**: Dependency conflict resolution time
- **Documentation quality**: Up-to-date docs percentage
- **CI/CD reliability**: Build success rate
- **Support requests**: Package manager related issues

That's your comprehensive migration guide! Follow these steps and your CURSED project will be using the package manager like a pro! 🚀✨

For ongoing support, check out the [Package Manager User Guide](package_manager.md) and [CLI Reference](package_manager_cli.md).
