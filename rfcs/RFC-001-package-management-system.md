# RFC #001: CURSED Package Management System

**Status**: Draft  
**Author**: CURSED Core Team  
**Created**: 2025-08-21  
**Target Release**: v1.1  

## Summary

This RFC proposes a comprehensive package management system for CURSED v1.1, including package registry, dependency resolution, and distribution infrastructure.

## Motivation

CURSED v1.0 established a solid foundation with 50+ standard library modules, but lacks a robust ecosystem for third-party packages. To accelerate adoption and community growth, we need:

1. **Easy package discovery** and installation
2. **Semantic versioning** and dependency management
3. **Secure package distribution** with integrity verification
4. **Publishing workflow** for package authors
5. **Private package registry** support for enterprises

## Design Goals

### Core Principles
- **Developer Experience**: Installation should be one command (`cursed add packagename`)
- **Security First**: All packages cryptographically signed and verified
- **Performance**: Sub-second dependency resolution and caching
- **Compatibility**: Seamless integration with existing build system
- **Decentralization**: Support for multiple registries and mirrors

### Success Criteria
- [ ] Install 95% of packages in <10 seconds
- [ ] Resolve complex dependency graphs in <2 seconds  
- [ ] Support 1000+ concurrent package downloads
- [ ] Zero successful supply chain attacks in first year
- [ ] 90% developer satisfaction score

## Detailed Design

### Package Manifest Format

```toml
# Package.toml
[package]
name = "awesome-http-client"
version = "1.2.3"
authors = ["developer@example.com"]
description = "A fast HTTP client for CURSED"
license = "MIT"
repository = "https://github.com/user/awesome-http-client"
documentation = "https://docs.example.com/awesome-http-client"
keywords = ["http", "networking", "async"]
categories = ["network-programming", "web-programming"]

[dependencies]
jsonz = "^2.1.0"
networkz = "^1.0.0"
asyncz = { version = "^0.9.0", features = ["tls"] }

[dev-dependencies]
testz = "^1.0.0"
benchmarkz = "^0.5.0"

[build]
entry = "src/lib.💀"
binary = "src/main.💀"  # Optional: for executable packages

[features]
default = ["tls", "compression"]
tls = ["cryptz/tls"]
compression = ["compressz"]
experimental = []

[target.linux]
dependencies = { systemz = "^1.0.0" }

[target.windows]
dependencies = { windowsz = "^1.0.0" }
```

### Package Structure

```
awesome-http-client/
├── Package.toml              # Package manifest
├── README.md                # Documentation
├── LICENSE                  # License file
├── CHANGELOG.md             # Version history
├── src/
│   ├── lib.💀             # Library entry point
│   ├── client.💀          # Core implementation
│   └── utils.💀           # Helper utilities
├── tests/
│   ├── integration_test.💀
│   └── unit_test.💀
├── examples/
│   └── quick_start.💀
└── docs/                   # Additional documentation
    └── api.md
```

### Command Line Interface

```bash
# Package Management Commands
cursed init [package-name]           # Initialize new package
cursed add <package>[@version]       # Add dependency
cursed remove <package>              # Remove dependency
cursed update [package]              # Update dependencies
cursed list                          # List installed packages
cursed search <query>                # Search package registry
cursed info <package>                # Show package information

# Publishing Commands
cursed login                         # Authenticate with registry
cursed publish                       # Publish package to registry
cursed unpublish <package>[@version] # Remove package version
cursed owner add/remove <user>       # Manage package ownership

# Workspace Commands
cursed workspace init               # Create multi-package workspace
cursed workspace add <path>         # Add package to workspace
cursed workspace build              # Build all workspace packages
cursed workspace test               # Test all workspace packages

# Registry Commands
cursed registry add <name> <url>    # Add custom registry
cursed registry remove <name>       # Remove registry
cursed registry list                # List configured registries
```

### Dependency Resolution Algorithm

#### Version Resolution Strategy
1. **SemVer Compatibility**: Use semantic versioning for compatibility checks
2. **Minimum Version Selection**: Choose minimum versions that satisfy constraints
3. **Diamond Dependency**: Resolve conflicting versions using latest compatible
4. **Pre-release Handling**: Never auto-select pre-release versions
5. **Yanked Versions**: Skip yanked versions unless explicitly requested

#### Resolution Process
```
1. Parse Package.toml dependencies
2. Query registries for available versions
3. Build dependency graph with version constraints
4. Resolve conflicts using minimum compatible versions
5. Validate circular dependency detection
6. Generate lock file with exact versions
7. Download and verify packages
8. Extract to local cache
```

### Package Registry Architecture

#### Registry API Endpoints
```
GET  /api/v1/packages                    # List packages (paginated)
GET  /api/v1/packages/{name}             # Package metadata
GET  /api/v1/packages/{name}/{version}   # Specific version info
GET  /api/v1/packages/{name}/versions    # All versions
POST /api/v1/packages/{name}/publish     # Publish new version
DELETE /api/v1/packages/{name}/{version} # Unpublish version
GET  /api/v1/search?q={query}            # Search packages
```

#### Package Storage
- **Metadata Storage**: PostgreSQL for searchable package information
- **Artifact Storage**: S3-compatible storage for package archives
- **CDN Distribution**: Global CDN for fast package downloads
- **Integrity Verification**: SHA256 checksums for all packages
- **Digital Signatures**: Ed25519 signatures for authenticity

#### Registry Features
- **Search and Discovery**: Full-text search with filters
- **Usage Analytics**: Download counts and popularity metrics
- **Security Scanning**: Automated vulnerability detection
- **Namespace Management**: Verified publisher accounts
- **API Rate Limiting**: DDoS protection and fair usage

### Security Model

#### Package Signing
```toml
# Registry maintains public keys for publishers
[security]
signatures = [
    { algorithm = "Ed25519", public_key = "abcd1234...", keyid = "publisher-key-1" }
]
checksums = [
    { algorithm = "SHA256", hash = "ef567890..." }
]
```

#### Client Verification Process
1. Download package archive and signature
2. Verify Ed25519 signature against publisher public key
3. Validate SHA256 checksum matches registry metadata
4. Check package contents against manifest
5. Scan for known vulnerabilities before installation

#### Supply Chain Protection
- **Publisher Verification**: GitHub/email identity verification required
- **Two-Factor Authentication**: Required for publishing
- **Audit Logging**: All publish/unpublish events logged
- **Vulnerability Database**: Integration with security advisory feeds
- **Reproducible Builds**: Encourage reproducible package builds

### Build System Integration

#### Package.toml Integration
```zig
// build.zig automatically reads Package.toml
const std = @import("std");
const cursed = @import("cursed");

pub fn build(b: *std.build.Builder) void {
    const package = cursed.Package.fromToml(b.allocator, "Package.toml");
    
    const exe = b.addExecutable(package.name, package.entry);
    
    // Automatically link dependencies
    for (package.dependencies.items) |dep| {
        exe.linkSystemLibrary(dep.name);
        exe.addPackagePath(dep.name, dep.import_path);
    }
    
    exe.install();
}
```

#### Dependency Caching
- **Global Cache**: `~/.cursed/packages` for shared dependencies
- **Project Cache**: `./cursed-packages` for project-specific versions
- **Content-Addressed Storage**: Avoid duplicate downloads
- **Cache Validation**: Verify integrity on each build
- **Cache Cleanup**: Automatic cleanup of unused versions

### Private Registry Support

#### Enterprise Features
- **Private Registries**: Self-hosted registries for proprietary packages
- **Access Control**: Organization-based package permissions
- **Mirroring**: Mirror public packages to private registries
- **Backup and Disaster Recovery**: Enterprise-grade data protection
- **Analytics Dashboard**: Usage tracking and security monitoring

#### Configuration
```toml
# ~/.cursed/config.toml
[registries]
default = "https://packages.cursedlang.org"
enterprise = "https://packages.corp.example.com"

[auth]
"packages.corp.example.com" = { token = "[REDACTED:enterprise-token]" }

[mirrors]
"packages.cursedlang.org" = [
    "https://mirror1.cursedlang.org",
    "https://mirror2.cursedlang.org"
]
```

## Implementation Plan

### Phase 1: Core Infrastructure (Weeks 1-4)
- [ ] Package manifest format and parsing
- [ ] Basic dependency resolution algorithm
- [ ] Local package cache implementation
- [ ] Command line interface skeleton

### Phase 2: Registry Backend (Weeks 5-8)
- [ ] Package registry API server
- [ ] Database schema for package metadata
- [ ] Package upload and storage system
- [ ] Basic search and discovery

### Phase 3: Security and Signing (Weeks 9-10)
- [ ] Digital signature implementation
- [ ] Publisher verification system
- [ ] Package integrity verification
- [ ] Vulnerability scanning integration

### Phase 4: Advanced Features (Weeks 11-12)
- [ ] Workspace support for multi-package projects
- [ ] Private registry support
- [ ] Advanced dependency resolution (features, targets)
- [ ] Performance optimizations

### Phase 5: Polish and Documentation (Weeks 13-14)
- [ ] Comprehensive documentation
- [ ] Migration guide from manual dependency management  
- [ ] Community testing and feedback
- [ ] Performance benchmarking

## Alternatives Considered

### Alternative 1: Git-based Dependencies
**Pros**: Simple, uses existing infrastructure
**Cons**: No version resolution, security concerns, performance issues

### Alternative 2: Vendor Directory Approach
**Pros**: Reproducible builds, offline support
**Cons**: Repository bloat, merge conflicts, poor UX

### Alternative 3: Operating System Packages
**Pros**: Mature ecosystem, system integration
**Cons**: OS-specific, slow updates, version conflicts

## Open Questions

1. **Monorepo Support**: How should we handle packages in monorepos?
2. **Binary Dependencies**: Should we support platform-specific binaries?
3. **Workspace Inheritance**: How should workspace packages inherit dependencies?
4. **Registry Federation**: Should we support federated registries?
5. **License Compliance**: How do we help users track license obligations?

## Backward Compatibility

This RFC introduces new functionality without breaking existing code:
- Existing projects continue to work without Package.toml
- Standard library imports remain unchanged
- Build system maintains backward compatibility
- Migration is opt-in, not required

## Community Input

We encourage community feedback on:
- Package manifest format design
- Command line interface ergonomics  
- Security model requirements
- Enterprise feature priorities
- Alternative implementation approaches

### How to Provide Feedback
- **Discord**: #rfc-discussion channel
- **GitHub**: Comment on RFC issue #1
- **Email**: rfc@cursedlang.org
- **Community Calls**: Weekly RFC review meetings

## Success Metrics

### Adoption Metrics
- [ ] 100 packages published in first month
- [ ] 1,000 packages by end of v1.1 release cycle
- [ ] 50% of new projects use package management
- [ ] 25+ enterprise private registries deployed

### Performance Metrics
- [ ] Average dependency resolution: <2 seconds
- [ ] Package download success rate: >99.9%
- [ ] Registry uptime: >99.95%
- [ ] Average package install time: <10 seconds

### Security Metrics
- [ ] Zero successful supply chain attacks
- [ ] 100% of packages cryptographically signed
- [ ] <24 hour response to security vulnerabilities
- [ ] 90% of publishers use 2FA

---

**Timeline**: Submit for review by August 28, 2025  
**Implementation Start**: September 1, 2025  
**Target Completion**: November 30, 2025  
**Release Target**: CURSED v1.1 (December 2025)
