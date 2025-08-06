# CURSED Production Deployment Pipeline

This directory contains the complete production deployment infrastructure for the CURSED programming language compiler.

## 🚀 Overview

The deployment pipeline provides:

- **Automated Release Builds** - Cross-platform compilation with optimization
- **Distribution Packaging** - tar.gz, zip, and native installers for all platforms
- **CI/CD Integration** - GitHub Actions workflows for automated deployment
- **Performance Profiling** - Comprehensive benchmarking and optimization analysis
- **Security Scanning** - Vulnerability detection and security validation
- **Monitoring Setup** - Production monitoring with Prometheus, Grafana, and Alertmanager
- **Version Management** - Semantic versioning and automated release tagging

## 📁 Directory Structure

```
deploy/
├── production_pipeline.sh      # Main deployment pipeline script
├── release_automation.py       # Version management and release automation
├── security_scanner.py         # Security vulnerability scanner
├── performance_profiler.py     # Performance benchmarking system
├── package_builder.py          # Multi-platform package builder
├── monitoring_setup.py         # Production monitoring configuration
└── README.md                   # This file
```

## 🛠️ Quick Start

### 1. Prerequisites

Ensure you have the required tools installed:

```bash
# Required tools
- zig (0.13.0+)
- git
- tar/gzip
- python3

# Optional tools (for enhanced features)
- hyperfine (performance benchmarking)
- valgrind (memory analysis)
- docker (monitoring stack)
```

### 2. Run Full Production Pipeline

```bash
# Set version and run complete pipeline
export VERSION=1.0.0
export BUILD_NUMBER=$(date +%Y%m%d%H%M%S)
./production_pipeline.sh
```

This will:
- Build optimized binaries for all platforms
- Run comprehensive testing
- Perform security and performance analysis
- Create distribution packages
- Generate deployment reports

### 3. Individual Operations

#### Security Scanning
```bash
python security_scanner.py --output security-report.json
```

#### Performance Benchmarking
```bash
python performance_profiler.py --runs 10 --graphs
```

#### Package Building
```bash
python package_builder.py --version 1.0.0 --platforms linux-x64 macos-arm64
```

#### Release Management
```bash
# Bump version
python release_automation.py bump --type minor

# Create release
python release_automation.py release --type patch

# Deploy to staging
python release_automation.py deploy
```

#### Monitoring Setup
```bash
python monitoring_setup.py --environment production
cd monitoring/
./start_monitoring.sh
```

## 🏗️ Architecture

### Build Pipeline Flow

```
1. Prerequisites Check
2. Clean Build Environment
3. Cross-Platform Compilation
4. Test Suite Execution
5. Security Scanning
6. Performance Benchmarking
7. Package Creation
8. Checksum Generation
9. Release Documentation
10. Deployment to Staging
```

### Supported Platforms

- **Linux x64** (primary development target)
- **Linux ARM64** (server deployment)
- **macOS x64** (Intel Macs)
- **macOS ARM64** (Apple Silicon)
- **Windows x64** (native Windows support)
- **WebAssembly** (browser/edge deployment)

### Package Formats

- **tar.gz** - Unix/Linux/macOS standard
- **zip** - Windows standard
- **deb** - Debian/Ubuntu packages
- **rpm** - RedHat/CentOS packages (planned)
- **msi** - Windows installers (planned)
- **pkg** - macOS installers (planned)

## 🔒 Security

### Security Scanning

The security scanner checks for:

- Hardcoded secrets and credentials
- Weak cryptographic algorithms
- Unsafe function calls
- Path traversal vulnerabilities
- Command injection risks
- Binary security features (stack protection, RELRO, PIE)

### Security Best Practices

- All binaries are built with security hardening
- Dependencies are scanned for known vulnerabilities
- Static analysis is performed on all code
- Memory safety is verified with Valgrind
- Secrets are managed through environment variables

## 📊 Performance

### Benchmarking Suite

The performance profiler measures:

- **Compilation Speed** - Time to compile various program sizes
- **Execution Speed** - Runtime performance of compiled programs
- **Memory Usage** - Peak memory consumption during compilation
- **Optimization Effectiveness** - Impact of compiler optimizations

### Performance Targets

- Compilation: ~100k lines of code per second
- Memory: <50MB for typical projects
- Binary size: ~2MB for stripped release builds
- Startup time: <100ms for small programs

## 🚀 CI/CD Integration

### GitHub Actions Workflows

- **`.github/workflows/production-deploy.yml`** - Main deployment pipeline
- **`.github/workflows/cross-platform.yml`** - Cross-platform builds

### Workflow Triggers

- **Tag Push** (v*) - Automatic production release
- **Manual Dispatch** - Controlled deployment to staging/production
- **Pull Request** - Build verification and testing

### Environment Variables

Required secrets in GitHub:
- `GITHUB_TOKEN` - For release creation
- `SLACK_WEBHOOK` - For deployment notifications (optional)

## 📈 Monitoring

### Production Monitoring Stack

- **Prometheus** - Metrics collection and alerting
- **Grafana** - Visualization and dashboards
- **Alertmanager** - Alert routing and notifications
- **Node Exporter** - System metrics

### Key Metrics

- Compilation rate and success rate
- Error rates by type
- Memory and CPU usage
- Response times and latency
- System health and availability

### Alerts

- Compiler service down
- High compilation latency (>10s)
- High error rate (>10%)
- Memory usage high (>80%)
- Disk space low (<10%)

## 🔧 Configuration

### Environment Variables

```bash
# Version and build info
VERSION=1.0.0
BUILD_NUMBER=20240101120000

# Deployment settings
DEPLOY_ENV=production
CURSED_METRICS_PORT=8080

# Monitoring
CURSED_ALERT_EMAIL=alerts@cursed-lang.org
CURSED_SLACK_WEBHOOK=https://hooks.slack.com/...

# Security
CURSED_SIGNING_KEY=/path/to/signing.key
```

### Build Options

```bash
# Optimization levels
-Doptimize=ReleaseFast    # Maximum performance
-Doptimize=ReleaseSmall   # Minimum size
-Doptimize=ReleaseSafe    # Safety checks enabled

# Additional options
-Dstatic=true            # Static linking
-Dtarget=x86_64-linux    # Cross-compilation target
-Dcoverage=true          # Code coverage
```

## 📚 Documentation

### Generated Documentation

The pipeline automatically generates:

- API reference documentation
- Performance benchmarks report
- Security scan results
- Cross-platform compatibility matrix
- Installation and usage guides

### Release Artifacts

Each release includes:

- Platform-specific binaries
- Source code archive
- Documentation package
- SHA256 checksums
- Release notes
- Installation scripts

## 🔄 Release Process

### Automated Release (Recommended)

1. Create and push a version tag:
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```

2. GitHub Actions automatically:
   - Runs full test suite
   - Performs security and performance analysis
   - Builds packages for all platforms
   - Creates GitHub release with assets
   - Deploys to staging environment

### Manual Release

1. Version bump:
   ```bash
   python release_automation.py bump --type minor
   ```

2. Run deployment pipeline:
   ```bash
   ./production_pipeline.sh
   ```

3. Create release:
   ```bash
   python release_automation.py release
   ```

## 🐛 Troubleshooting

### Common Issues

#### Build Failures
- Ensure all prerequisites are installed
- Check Zig version compatibility (0.13.0+)
- Verify LLVM installation and paths

#### Test Failures
- Run tests individually to isolate issues
- Check for platform-specific test failures
- Review test output logs in artifacts/

#### Security Scan Failures
- Review security report for critical issues
- Update dependencies with known vulnerabilities
- Fix hardcoded secrets or unsafe code patterns

#### Performance Regressions
- Compare with previous benchmark results
- Profile specific test cases that regressed
- Check for memory leaks or infinite loops

### Getting Help

- Check the [main project README](../README.md)
- Review existing [GitHub issues](https://github.com/ghuntley/cursed/issues)
- Create a new issue with deployment logs and error details

## 🤝 Contributing

### Adding New Platforms

1. Add platform configuration to `package_builder.py`
2. Update cross-compilation targets in `build.zig`
3. Add platform-specific build steps to CI workflows
4. Test end-to-end deployment pipeline

### Improving Performance

1. Add new benchmark cases to `performance_profiler.py`
2. Update performance targets and alerts
3. Profile and optimize bottlenecks
4. Document performance improvements

### Security Enhancements

1. Add new security checks to `security_scanner.py`
2. Update hardening flags in build configuration
3. Review and update dependency scanning
4. Document security best practices

## 📄 License

This deployment infrastructure is part of the CURSED project and is licensed under the MIT License.

---

**Made with ❤️ for the CURSED community**

For more information, visit: https://github.com/ghuntley/cursed
