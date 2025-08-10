# CURSED Compiler - Production Packaging System

> **Enterprise-ready distribution and deployment for the CURSED Programming Language Compiler**

This directory contains a comprehensive production packaging and distribution system designed for enterprise deployment of the CURSED compiler across multiple platforms and package managers.

## 🚀 Quick Start

```bash
# Build all platform packages
./packaging/build-release.sh

# Run automated release
./packaging/automation/release-automation.sh

# Verify packages
./packaging/scripts/package-verification.sh

# Deploy containers
docker-compose -f packaging/docker/docker-compose.yml up
```

## 📁 Directory Structure

```
packaging/
├── 📄 README.md                          # This file
├── 📄 VERSION                            # Current version
├── 🔧 build-release.sh                   # Main build script
├── 📁 ci/
│   └── 📄 github-actions.yml             # CI/CD pipeline
├── 📁 docker/
│   ├── 📄 Dockerfile                     # Multi-stage build
│   ├── 📄 Dockerfile.alpine              # Alpine runtime
│   ├── 📄 Dockerfile.ubuntu              # Ubuntu runtime
│   └── 📄 docker-compose.yml             # Development environment
├── 📁 package-managers/
│   ├── 📁 homebrew/
│   │   └── 📄 cursed.rb                  # Homebrew formula
│   ├── 📁 apt/
│   │   └── 🔧 setup-repository.sh        # APT repository setup
│   └── 📁 yum/
│       └── 🔧 setup-repository.sh        # YUM repository setup
├── 📁 automation/
│   └── 🔧 release-automation.sh          # Automated releases
├── 📁 scripts/
│   └── 🔧 package-verification.sh        # Package testing
├── 📁 templates/
│   └── 📄 RELEASE_NOTES.md.template      # Release notes template
└── 📁 docs/
    └── 📄 DISTRIBUTION_GUIDE.md          # Comprehensive guide
```

## 🎯 Features

### Multi-Platform Support
- ✅ **Linux**: x86_64, ARM64 (DEB, RPM, tar.gz)
- ✅ **macOS**: Intel, Apple Silicon (PKG, tar.gz)
- ✅ **Windows**: x86_64 (MSI, ZIP)
- ✅ **WebAssembly**: WASI runtime (WASM)

### Package Formats
- 📦 **Native Packages**: DEB, RPM, MSI, PKG
- 📁 **Archives**: tar.gz, ZIP
- 🐳 **Containers**: Docker (Alpine, Ubuntu)
- 📋 **Package Managers**: Homebrew, APT, YUM

### Enterprise Features
- 🔒 **Security**: GPG signing, checksum verification
- 🔄 **Automation**: CI/CD pipelines, automated releases
- 📊 **Monitoring**: Health checks, metrics collection
- 🛠️ **Testing**: Comprehensive package validation
- 📚 **Documentation**: Complete deployment guides

## 🛠️ Build System

### Quick Build

```bash
# Build for all platforms
./packaging/build-release.sh

# Build specific platform
zig build -Dtarget=x86_64-unknown-linux-gnu -Doptimize=ReleaseFast

# Build with custom options
BUILD_MODE=release ENABLE_LTO=true ./packaging/build-release.sh
```

### Configuration Options

```bash
# Environment variables
export BUILD_MODE=release                    # release, debug
export ENABLE_LTO=true                      # Link-time optimization
export ENABLE_STRIP=true                    # Strip debug symbols
export PARALLEL_JOBS=8                      # Parallel compilation

# Supported targets
TARGETS=(
    "x86_64-unknown-linux-gnu"    # Linux x64
    "aarch64-unknown-linux-gnu"   # Linux ARM64
    "x86_64-apple-darwin"         # macOS Intel
    "aarch64-apple-darwin"        # macOS Apple Silicon
    "x86_64-pc-windows-gnu"       # Windows x64
    "wasm32-wasi"                 # WebAssembly
)
```

### Build Outputs

After running the build script, artifacts are created in the `dist/` directory:

```
dist/
├── cursed-1.0.0-x86_64-unknown-linux-gnu.tar.gz
├── cursed-1.0.0-aarch64-apple-darwin.tar.gz
├── cursed-1.0.0-x86_64-pc-windows-gnu.zip
├── cursed_1.0.0_amd64.deb
├── cursed-1.0.0-1.x86_64.rpm
├── cursed-1.0.0.pkg
└── SHA256SUMS
```

## 🐳 Container Deployment

### Docker Images

Three optimized container variants:

```bash
# Alpine Linux (minimal size)
docker pull cursed/compiler:1.0.0-alpine
docker run --rm cursed/compiler:1.0.0-alpine --version

# Ubuntu LTS (compatibility)
docker pull cursed/compiler:1.0.0-ubuntu
docker run -it cursed/compiler:1.0.0-ubuntu bash

# Development environment
docker pull cursed/compiler:1.0.0-dev
docker run -it -v $(pwd):/workspace cursed/compiler:1.0.0-dev
```

### Docker Compose

```bash
# Development environment
docker-compose -f packaging/docker/docker-compose.yml up cursed-dev

# Production compiler service
docker-compose -f packaging/docker/docker-compose.yml up cursed-compiler

# LSP server for IDE integration
docker-compose -f packaging/docker/docker-compose.yml up cursed-lsp
```

### Container Features

- 🏔️ **Alpine**: 50MB runtime image
- 🐧 **Ubuntu**: Enterprise compatibility
- 👤 **Non-root**: Security-hardened execution
- 🏥 **Health checks**: Built-in monitoring
- 📁 **Volumes**: Persistent cache and projects

## 📦 Package Managers

### Homebrew (macOS)

```bash
# Install from tap
brew tap cursed/tap
brew install cursed

# Direct formula
brew install https://raw.githubusercontent.com/cursed/homebrew-tap/main/cursed.rb
```

**Formula Features:**
- 🍎 Universal binaries (Intel + Apple Silicon)
- 📚 Standard library integration
- 🔧 Environment setup
- ✅ Comprehensive testing

### APT Repository (Debian/Ubuntu)

```bash
# Setup repository
curl -fsSL https://packages.cursed.dev/apt/install-cursed-repo.sh | sudo bash

# Install CURSED
sudo apt update
sudo apt install cursed
```

**Repository Features:**
- 🔒 GPG signed packages
- 🏗️ Multiple architectures (amd64, arm64)
- 📊 Repository metrics
- 🔄 Automated updates

### YUM Repository (RHEL/CentOS/Fedora)

```bash
# Setup repository
curl -fsSL https://packages.cursed.dev/yum/install-cursed-repo.sh | sudo bash

# Install CURSED
sudo dnf install cursed  # Fedora/RHEL 8+
sudo yum install cursed  # RHEL 7
```

**Repository Features:**
- 🏢 Enterprise Linux support
- 📦 RPM package management
- 🔐 Package signing
- 📈 Usage analytics

## 🔄 Release Automation

### Automated Releases

The release system handles complete end-to-end releases:

```bash
# Patch release (1.0.0 → 1.0.1)
./packaging/automation/release-automation.sh

# Minor release (1.0.1 → 1.1.0)
./packaging/automation/release-automation.sh --type minor

# Major release (1.1.0 → 2.0.0)
./packaging/automation/release-automation.sh --type major

# Prerelease (1.0.0 → 1.0.1-alpha.0)
./packaging/automation/release-automation.sh --type prerelease
```

### Automation Features

- 📊 **Version Management**: Semantic versioning
- 🏗️ **Multi-platform Builds**: Parallel compilation
- 🧪 **Testing**: Comprehensive test suites
- 📝 **Documentation**: Automated release notes
- 🏷️ **Git Integration**: Tagging and pushing
- 🐙 **GitHub Releases**: Artifact uploads
- 📢 **Notifications**: Slack, Discord, email

### CI/CD Pipeline

GitHub Actions workflow provides:

1. **Trigger**: Tag push or manual dispatch
2. **Build**: All platforms in parallel
3. **Test**: Cross-platform validation
4. **Package**: Native package creation
5. **Container**: Docker image builds
6. **Release**: GitHub release with artifacts
7. **Deploy**: Package repository updates

## 🔒 Security

### Package Signing

All packages are cryptographically signed:

```bash
# Verify DEB package
apt-key fingerprint cursed

# Verify RPM package
rpm --checksig cursed-1.0.0-1.x86_64.rpm

# Verify checksums
sha256sum -c SHA256SUMS
```

### Container Security

- 👤 **Non-root execution**: All containers run as unprivileged user
- 🔒 **Minimal attack surface**: Alpine Linux base
- 🏥 **Health monitoring**: Built-in health checks
- 📊 **Security scanning**: Vulnerability assessments

### Supply Chain Security

- ✅ **Reproducible builds**: Deterministic compilation
- 📋 **SBOM**: Software Bill of Materials
- 🔍 **Dependency scanning**: Regular audits
- 📦 **Signed artifacts**: GPG verification

## 🧪 Testing & Validation

### Package Verification

```bash
# Test all packages
./packaging/scripts/package-verification.sh

# Test specific platform
./packaging/scripts/package-verification.sh --platform x86_64-unknown-linux-gnu

# Test specific format
./packaging/scripts/package-verification.sh --format deb
```

### Test Coverage

- ✅ **Archive integrity**: Format validation
- 🔧 **Binary execution**: Functionality testing
- 📦 **Package installation**: Native package managers
- 🐳 **Container execution**: Docker environments
- 🔒 **Security verification**: Signature validation

### Test Environments

- 🐧 **Linux**: Ubuntu 20.04+, Debian 11+, CentOS 7+
- 🍎 **macOS**: 10.15+, Intel + Apple Silicon
- 🪟 **Windows**: Windows 10+, Server 2019+
- 🐳 **Containers**: Alpine, Ubuntu, CentOS, Fedora

## 📊 Monitoring & Analytics

### Health Checks

```bash
# Repository health
curl https://packages.cursed.dev/health

# Package metrics
curl https://packages.cursed.dev/metrics

# Container health
docker run --rm cursed/compiler:latest --version
```

### Monitoring Endpoints

- 🏥 **Health**: `/health` - Service status
- 📊 **Metrics**: `/metrics` - Prometheus metrics
- 📈 **Analytics**: `/analytics` - Usage statistics
- 📋 **Status**: `/status` - System information

## 🛠️ Development

### Local Development

```bash
# Setup development environment
docker-compose -f packaging/docker/docker-compose.yml up cursed-dev

# Test build locally
./packaging/build-release.sh --dry-run

# Verify packages
./packaging/scripts/package-verification.sh --skip-containers
```

### Contributing

1. **Fork** the repository
2. **Create** feature branch
3. **Test** on multiple platforms
4. **Verify** package integrity
5. **Submit** pull request

### Testing Changes

```bash
# Test build system
./packaging/build-release.sh --dry-run

# Test automation
./packaging/automation/release-automation.sh --dry-run

# Validate packages
./packaging/scripts/package-verification.sh
```

## 📚 Documentation

- 📖 **[Distribution Guide](docs/DISTRIBUTION_GUIDE.md)**: Comprehensive deployment documentation
- 🔧 **[Build System](docs/BUILD_SYSTEM.md)**: Build configuration and troubleshooting
- 🐳 **[Container Guide](docs/CONTAINER_GUIDE.md)**: Docker deployment best practices
- 🔒 **[Security Guide](docs/SECURITY_GUIDE.md)**: Security considerations and best practices

## 🆘 Troubleshooting

### Common Issues

#### Build Failures

```bash
# LLVM not found
sudo apt install llvm-18-dev libllvm18

# Cross-compilation issues
./packaging/build-release.sh --native-only

# Memory issues
export PARALLEL_JOBS=2
```

#### Package Installation

```bash
# APT repository issues
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys <KEY_ID>
sudo apt update

# YUM repository issues
sudo dnf clean all
sudo dnf makecache
```

#### Container Problems

```bash
# Docker build failures
docker system prune -a
docker build --no-cache

# Permission issues
docker run --user $(id -u):$(id -g)
```

### Getting Help

- 🐛 **Issues**: [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/ghuntley/cursed/discussions)
- 📞 **Discord**: [Community Server](https://discord.gg/cursed)
- 📧 **Enterprise**: enterprise@cursed.dev

## 📈 Roadmap

### Current Features (v1.0)
- ✅ Multi-platform builds
- ✅ Native packages (DEB, RPM, MSI, PKG)
- ✅ Container deployment
- ✅ Package manager integration
- ✅ Automated releases
- ✅ Security signing

### Upcoming Features (v1.1)
- 🔄 **Chocolatey**: Windows package manager
- 📱 **Snap**: Universal Linux packages
- 🏗️ **Flatpak**: Desktop application packaging
- ☁️ **Cloud**: AWS, GCP, Azure deployment
- 📊 **Telemetry**: Usage analytics
- 🔧 **Tools**: Package manager tooling

### Future Roadmap (v2.0)
- 🏢 **Enterprise**: LDAP integration, audit logging
- 🔐 **Security**: SLSA compliance, provenance
- 📦 **Registry**: Private package registry
- 🌐 **CDN**: Global distribution network
- 🤖 **AI**: Intelligent deployment optimization

## 🤝 Support

### Community Support
- 📖 [Documentation](https://docs.cursed.dev)
- 💬 [Discord Community](https://discord.gg/cursed)
- 🐛 [Issue Tracker](https://github.com/ghuntley/cursed/issues)

### Enterprise Support
- 📞 **Priority Support**: 24/7 enterprise support
- 🎓 **Training**: Team training and onboarding
- 🏗️ **Custom Deployment**: Tailored solutions
- 📧 **Contact**: enterprise@cursed.dev

---

## 📄 License

This packaging system is part of the CURSED compiler project and is licensed under the same terms.

**Maintained by the CURSED Development Team** | [cursed.dev](https://cursed.dev)

---

*Ready to deploy CURSED at enterprise scale? Start with our [Distribution Guide](docs/DISTRIBUTION_GUIDE.md)!*
