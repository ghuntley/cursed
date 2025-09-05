# CURSED Compiler Distribution Guide

This guide provides comprehensive instructions for packaging, distributing, and deploying the CURSED compiler for enterprise environments.

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Build System](#build-system)
4. [Package Formats](#package-formats)
5. [Container Deployment](#container-deployment)
6. [Package Manager Integration](#package-manager-integration)
7. [Release Automation](#release-automation)
8. [Enterprise Deployment](#enterprise-deployment)
9. [Security Considerations](#security-considerations)
10. [Troubleshooting](#troubleshooting)

## Overview

The CURSED compiler distribution system provides:

- **Multi-platform builds**: Linux, macOS, Windows, ARM64, WebAssembly
- **Native packages**: DEB, RPM, MSI, PKG formats
- **Container images**: Docker, Podman support with Alpine and Ubuntu bases
- **Package managers**: Homebrew, APT, YUM integration
- **Automated releases**: CI/CD pipelines with GitHub Actions
- **Enterprise features**: GPG signing, repository management, security scanning

## Quick Start

### Building All Platforms

```bash
# Make the build script executable
chmod +x packaging/build-release.sh

# Build for all supported platforms
./packaging/build-release.sh

# Check the generated artifacts
ls -la dist/
```

### Creating a Single Platform Package

```bash
# Build for Linux x86_64 only
zig build -Dtarget=x86_64-unknown-linux-gnu -Doptimize=ReleaseFast

# Build for macOS ARM64
zig build -Dtarget=aarch64-apple-darwin -Doptimize=ReleaseFast

# Build for Windows
zig build -Dtarget=x86_64-pc-windows-gnu -Doptimize=ReleaseFast
```

### Docker Quick Start

```bash
# Build the Docker image
docker build -t cursed/compiler:latest -f packaging/docker/Dockerfile .

# Run the compiler
docker run --rm cursed/compiler:latest --version

# Interactive development environment
docker-compose -f packaging/docker/docker-compose.yml up cursed-dev
```

## Build System

### Architecture

The build system consists of several components:

```
packaging/
├── build-release.sh           # Main build script
├── ci/
│   └── github-actions.yml     # CI/CD pipeline
├── docker/
│   ├── Dockerfile             # Multi-stage container build
│   ├── Dockerfile.alpine      # Alpine Linux runtime
│   ├── Dockerfile.ubuntu      # Ubuntu LTS runtime
│   └── docker-compose.yml     # Development environment
├── package-managers/
│   ├── homebrew/             # macOS package manager
│   ├── apt/                  # Debian/Ubuntu repositories
│   └── yum/                  # RedHat/CentOS repositories
└── automation/
    └── release-automation.sh  # Automated release management
```

### Build Configuration

The build system supports various configuration options:

```bash
# Environment variables
export BUILD_MODE=release                    # release, debug
export ENABLE_LTO=true                      # Link-time optimization
export ENABLE_STRIP=true                    # Strip debug symbols
export PARALLEL_JOBS=8                      # Parallel compilation jobs

# Build targets
export TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
    "wasm32-wasi"
)
```

### Cross-Compilation

The build system supports cross-compilation for all target platforms:

```bash
# Linux ARM64 from x86_64
zig build -Dtarget=aarch64-unknown-linux-gnu

# Windows from Linux
zig build -Dtarget=x86_64-pc-windows-gnu

# macOS from Linux (requires macOS SDK)
zig build -Dtarget=x86_64-apple-darwin

# WebAssembly
zig build -Dtarget=wasm32-wasi -Doptimize=ReleaseSmall
```

## Package Formats

### Debian Packages (DEB)

```bash
# Create DEB package
./packaging/build-release.sh

# Install the package
sudo dpkg -i dist/cursed_1.0.0_amd64.deb

# Verify installation
cursed-zig --version
```

Package details:
- **Architecture**: amd64, arm64
- **Dependencies**: libc6, libllvm18
- **Install location**: `/usr/bin/`, `/usr/share/cursed/`
- **Cache directory**: `/var/cache/cursed`

### RPM Packages

```bash
# Install on RHEL/CentOS/Fedora
sudo rpm -i dist/cursed-1.0.0-1.x86_64.rpm

# Or using package manager
sudo dnf install dist/cursed-1.0.0-1.x86_64.rpm
```

Package details:
- **Architecture**: x86_64, aarch64
- **Dependencies**: glibc, llvm
- **Install location**: `/usr/bin/`, `/usr/share/cursed/`

### Windows MSI

```powershell
# Install using Windows Installer
msiexec /i cursed-1.0.0.msi /quiet

# Verify installation
cursed-zig.exe --version
```

### macOS PKG

```bash
# Install the package
sudo installer -pkg cursed-1.0.0.pkg -target /

# Verify installation
cursed-zig --version
```

## Container Deployment

### Docker Images

We provide multiple Docker image variants:

#### Alpine Linux (Minimal)

```bash
# Pull the Alpine image (smallest size)
docker pull cursed/compiler:1.0.0-alpine

# Run with volume mount
docker run --rm -v $(pwd):/workspace cursed/compiler:1.0.0-alpine /workspace/main.💀
```

#### Ubuntu LTS (Compatibility)

```bash
# Pull the Ubuntu image (better compatibility)
docker pull cursed/compiler:1.0.0-ubuntu

# Interactive development
docker run -it --rm -v $(pwd):/workspace cursed/compiler:1.0.0-ubuntu bash
```

### Docker Compose

```bash
# Development environment
docker-compose -f packaging/docker/docker-compose.yml up cursed-dev

# Production compiler service
docker-compose -f packaging/docker/docker-compose.yml up cursed-compiler

# LSP server for editors
docker-compose -f packaging/docker/docker-compose.yml up cursed-lsp
```

### Kubernetes Deployment

```yaml
# Example Kubernetes deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cursed-compiler
spec:
  replicas: 3
  selector:
    matchLabels:
      app: cursed-compiler
  template:
    metadata:
      labels:
        app: cursed-compiler
    spec:
      containers:
      - name: cursed-compiler
        image: cursed/compiler:1.0.0
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "1Gi"
            cpu: "1000m"
        volumeMounts:
        - name: source-code
          mountPath: /workspace
      volumes:
      - name: source-code
        persistentVolumeClaim:
          claimName: source-pvc
```

## Package Manager Integration

### Homebrew (macOS)

```bash
# Add the tap
brew tap cursed/tap

# Install CURSED
brew install cursed

# Update to latest version
brew upgrade cursed
```

**Maintainers**: To update the Homebrew formula:

```bash
# Update the formula with new version and checksums
./packaging/automation/release-automation.sh --homebrew-update
```

### APT Repository (Debian/Ubuntu)

#### Setup Repository

```bash
# Run the repository setup script
sudo ./packaging/package-managers/apt/setup-repository.sh
```

#### Client Installation

```bash
# Add repository
curl -fsSL https://packages.cursed.dev/apt/install-cursed-repo.sh | sudo bash

# Install CURSED
sudo apt install cursed
```

#### Repository Management

```bash
# Add a new package
./add-package.sh cursed_1.0.1_amd64.deb

# Remove a package
./remove-package.sh cursed

# Update repository metadata
./update-repository.sh
```

### YUM Repository (RHEL/CentOS/Fedora)

#### Setup Repository

```bash
# Run the repository setup script
sudo ./packaging/package-managers/yum/setup-repository.sh
```

#### Client Installation

```bash
# Add repository
curl -fsSL https://packages.cursed.dev/yum/install-cursed-repo.sh | sudo bash

# Install CURSED
sudo dnf install cursed        # Fedora/RHEL 8+
sudo yum install cursed        # RHEL 7
```

#### Repository Management

```bash
# Add a new package
./add-package.sh el9 cursed-1.0.1-1.x86_64.rpm

# Remove a package
./remove-package.sh el9 cursed

# Update repository metadata
./update-repository.sh
```

## Release Automation

### Automated Releases

The release automation system handles:

- **Version management**: Semantic versioning with automatic increment
- **Build orchestration**: Multi-platform builds with testing
- **Package creation**: All supported package formats
- **Git operations**: Tagging, commit, and push
- **GitHub releases**: Automated release creation with artifacts
- **Notifications**: Slack, Discord, email notifications

### Manual Release

```bash
# Patch release (1.0.0 -> 1.0.1)
./packaging/automation/release-automation.sh

# Minor release (1.0.1 -> 1.1.0)
./packaging/automation/release-automation.sh --type minor

# Major release (1.1.0 -> 2.0.0)
./packaging/automation/release-automation.sh --type major

# Prerelease (1.0.0 -> 1.0.1-alpha.0)
./packaging/automation/release-automation.sh --type prerelease

# Dry run (show what would happen)
./packaging/automation/release-automation.sh --dry-run
```

### CI/CD Pipeline

The GitHub Actions workflow automatically:

1. **Triggers**: On tag push (`v*`) or manual dispatch
2. **Builds**: All supported platforms in parallel
3. **Tests**: Comprehensive test suite on all platforms
4. **Packages**: Creates native packages (DEB, RPM, PKG, MSI)
5. **Containers**: Builds and pushes Docker images
6. **Releases**: Creates GitHub release with all artifacts
7. **Notifications**: Sends notifications to configured channels

### Environment Variables

Configure the automation system with these environment variables:

```bash
# Git configuration
export GIT_REMOTE=origin
export GIT_BRANCH=main

# Package managers
export HOMEBREW_TAP_REPO=cursed/homebrew-tap
export DOCKER_REGISTRY=cursed

# Notifications
export SLACK_WEBHOOK_URL=https://hooks.slack.com/...
export DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...
export EMAIL_RECIPIENTS=dev@cursed.dev

# Security
export GPG_KEY_ID=ABC123DEF456
export DOCKER_USERNAME=cursed
export DOCKER_PASSWORD=secret
```

## Enterprise Deployment

### Security Requirements

For enterprise deployments, ensure:

1. **GPG Signing**: All packages are signed with valid GPG keys
2. **Checksum Verification**: SHA256 checksums for all artifacts
3. **TLS Encryption**: HTTPS for all repository communication
4. **Access Control**: Authentication for package repositories
5. **Vulnerability Scanning**: Regular security audits

### High Availability

Deploy package repositories with:

```bash
# Load balancer configuration
upstream cursed_packages {
    server packages1.cursed.dev:443;
    server packages2.cursed.dev:443;
    server packages3.cursed.dev:443;
}

server {
    listen 443 ssl http2;
    server_name packages.cursed.dev;
    
    location / {
        proxy_pass https://cursed_packages;
        proxy_set_header Host $host;
        proxy_ssl_verify off;
    }
}
```

### Monitoring

Monitor package repositories with:

```bash
# Health check endpoints
curl -f https://packages.cursed.dev/health

# Metrics collection
curl https://packages.cursed.dev/metrics

# Log aggregation
tail -f /var/log/nginx/cursed-packages-access.log
```

### Backup and Recovery

```bash
# Backup repository data
rsync -av /srv/apt/cursed/ backup-server:/backups/apt/
rsync -av /srv/yum/cursed/ backup-server:/backups/yum/

# GPG key backup
gpg --export-secret-keys > cursed-gpg-secret.key
gpg --export > cursed-gpg-public.key
```

## Security Considerations

### Package Signing

All packages are signed with GPG keys:

```bash
# Generate GPG key
gpg --gen-key

# Sign a package
rpmsign --addsign package.rpm
dpkg-sig --sign builder package.deb

# Verify signature
rpm --checksig package.rpm
dpkg-sig --verify package.deb
```

### Repository Security

1. **HTTPS Only**: All repository traffic uses TLS 1.2+
2. **GPG Verification**: Package signatures are verified automatically
3. **Access Logs**: All access is logged and monitored
4. **Rate Limiting**: Prevents abuse and DDoS attacks

### Container Security

```dockerfile
# Security best practices in containers
RUN adduser -D -s /bin/sh cursed  # Non-root user
USER cursed                       # Switch to non-root
COPY --chown=cursed:cursed ...    # Correct ownership
HEALTHCHECK CMD cursed --version  # Health monitoring
```

### Supply Chain Security

1. **Source Verification**: All dependencies are verified
2. **Reproducible Builds**: Builds are deterministic
3. **Vulnerability Scanning**: Regular security audits
4. **SBOM Generation**: Software Bill of Materials included

## Troubleshooting

### Common Build Issues

#### LLVM Not Found

```bash
# Install LLVM development packages
sudo apt install llvm-18-dev libllvm18  # Ubuntu/Debian
sudo dnf install llvm-devel              # Fedora
sudo yum install llvm-devel              # RHEL/CentOS

# Set LLVM path manually
export LLVM_CONFIG_PATH=/usr/bin/llvm-config-18
```

#### Cross-Compilation Failures

```bash
# Install cross-compilation tools
sudo apt install gcc-aarch64-linux-gnu  # ARM64 cross-compiler
sudo apt install mingw-w64               # Windows cross-compiler

# Use native builds if cross-compilation fails
./packaging/build-release.sh --native-only
```

#### Memory Issues

```bash
# Reduce parallel jobs
export PARALLEL_JOBS=2

# Enable swap if low memory
sudo swapon /swapfile

# Use release build (smaller memory footprint)
zig build -Doptimize=ReleaseFast
```

### Package Installation Issues

#### APT Repository Problems

```bash
# Update GPG keyring
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys <KEY_ID>

# Clear APT cache
sudo apt clean
sudo apt update

# Check repository configuration
cat /etc/apt/sources.list.d/cursed.list
```

#### YUM Repository Problems

```bash
# Clear YUM cache
sudo dnf clean all  # Fedora
sudo yum clean all  # RHEL/CentOS

# Rebuild cache
sudo dnf makecache
sudo yum makecache fast

# Check repository configuration
cat /etc/yum.repos.d/cursed.repo
```

### Container Issues

#### Docker Build Failures

```bash
# Clear Docker cache
docker system prune -a

# Build with no cache
docker build --no-cache -t cursed/compiler .

# Check resource limits
docker system df
docker stats
```

#### Permission Issues

```bash
# Fix file permissions
sudo chown -R $USER:$USER dist/

# Run container as current user
docker run --user $(id -u):$(id -g) cursed/compiler
```

### Performance Issues

#### Slow Builds

```bash
# Increase parallel jobs
export PARALLEL_JOBS=$(nproc)

# Use SSD storage
export TMPDIR=/fast/tmp

# Enable compiler cache
export ZIG_CACHE_DIR=/fast/zig-cache
```

#### Large Package Sizes

```bash
# Use ReleaseSmall optimization
zig build -Doptimize=ReleaseSmall

# Strip debug symbols
export ENABLE_STRIP=true

# Enable compression
export ENABLE_COMPRESSION=true
```

## Support and Resources

### Documentation

- [CURSED Language Guide](https://docs.cursed.dev/guide/)
- [API Reference](https://docs.cursed.dev/api/)
- [Standard Library](https://docs.cursed.dev/stdlib/)

### Community

- [GitHub Issues](https://github.com/ghuntley/cursed/issues)
- [Discussions](https://github.com/ghuntley/cursed/discussions)
- [Discord](https://discord.gg/cursed)

### Enterprise Support

For enterprise support, contact: enterprise@cursed.dev

## Contributing

To contribute to the distribution system:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test on multiple platforms
5. Submit a pull request

### Testing Changes

```bash
# Test build system
./packaging/build-release.sh --dry-run

# Test package creation
./packaging/test-packages.sh

# Test container builds
docker build -f packaging/docker/Dockerfile.test .
```

---

This distribution guide is maintained by the CURSED Development Team. For questions or suggestions, please open an issue on GitHub.
