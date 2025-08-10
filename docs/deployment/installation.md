# CURSED Installation Guide

Complete installation and deployment guide for production environments.

## System Requirements

### Minimum Requirements
- **RAM**: 4GB (8GB recommended for large projects)
- **Storage**: 2GB free space
- **CPU**: x86_64 or ARM64 architecture
- **OS**: Linux (Ubuntu 20.04+), macOS (10.15+), Windows 10+ (via WSL2)

### Development Requirements
- **Zig**: 0.11.0 or later
- **LLVM**: 15.0+ (for native compilation)
- **Git**: For source code management
- **Valgrind**: For memory safety testing (Linux only)

## Quick Installation

### Pre-built Binaries (Recommended)

```bash
# Download latest release
curl -L https://github.com/ghuntley/cursed/releases/latest/download/cursed-linux-x64.tar.gz | tar xz

# Install to system path
sudo mv cursed-zig /usr/local/bin/
sudo mv cursed-stable /usr/local/bin/

# Verify installation
cursed-zig --version
```

### Package Managers

#### Homebrew (macOS/Linux)
```bash
brew tap ghuntley/cursed
brew install cursed
```

#### Snap (Linux)
```bash
sudo snap install cursed --classic
```

#### Chocolatey (Windows)
```powershell
choco install cursed
```

## Source Installation

### Prerequisites Setup

#### Ubuntu/Debian
```bash
# Install Zig
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /opt/zig
echo 'export PATH="/opt/zig:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Install LLVM
sudo apt update
sudo apt install llvm-15-dev libllvm15 llvm-15-runtime
sudo apt install build-essential git valgrind

# Install development dependencies
sudo apt install pkg-config libssl-dev
```

#### macOS
```bash
# Install Zig via Homebrew
brew install zig

# Install LLVM
brew install llvm@15

# Add LLVM to PATH
echo 'export PATH="/opt/homebrew/opt/llvm@15/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows (WSL2)
```bash
# Install WSL2 Ubuntu
wsl --install -d Ubuntu

# Follow Ubuntu instructions above in WSL2 environment
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build compiler
zig build

# Install system-wide
sudo cp zig-out/bin/cursed-zig /usr/local/bin/
sudo cp zig-out/bin/cursed-stable /usr/local/bin/

# Verify installation
cursed-zig --version
cursed-stable --version
```

### Build Validation

```bash
# Test basic functionality
echo 'vibez.spill("Hello, CURSED!")' > hello.csd
cursed-zig hello.csd

# Test memory safety
valgrind cursed-zig hello.csd

# Test compilation
cursed-zig --compile hello.csd
./hello

# Test standard library
echo 'yeet "mathz"; vibez.spill(abs_normie(-42))' > math_test.csd
cursed-zig math_test.csd
```

## Development Environment Setup

### DevEnv.sh (Recommended)

```bash
# Install devenv.sh
curl -L https://github.com/cachix/devenv/releases/latest/download/devenv-linux-x86_64 -o devenv
chmod +x devenv
sudo mv devenv /usr/local/bin/

# Setup development environment
cd cursed
direnv allow
devenv shell

# All dependencies now available
zig build
```

### Docker Development Environment

```dockerfile
# Dockerfile.dev
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl wget build-essential git valgrind \
    llvm-15-dev libllvm15 llvm-15-runtime \
    pkg-config libssl-dev

# Install Zig
RUN wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz \
    && tar xf zig-linux-x86_64-0.11.0.tar.xz \
    && mv zig-linux-x86_64-0.11.0 /opt/zig \
    && ln -s /opt/zig/zig /usr/local/bin/zig

WORKDIR /workspace
COPY . .
RUN zig build

CMD ["bash"]
```

```bash
# Build and run development container
docker build -f Dockerfile.dev -t cursed-dev .
docker run -it -v $(pwd):/workspace cursed-dev
```

## Production Deployment

### Container Deployment

#### Multi-stage Dockerfile
```dockerfile
# Dockerfile
FROM ubuntu:22.04 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl wget build-essential git \
    llvm-15-dev libllvm15 llvm-15-runtime

# Install Zig
RUN wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz \
    && tar xf zig-linux-x86_64-0.11.0.tar.xz \
    && mv zig-linux-x86_64-0.11.0 /opt/zig \
    && ln -s /opt/zig/zig /usr/local/bin/zig

# Build CURSED
COPY . /src
WORKDIR /src
RUN zig build -Doptimize=ReleaseFast

# Production image
FROM ubuntu:22.04

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libllvm15 \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binaries
COPY --from=builder /src/zig-out/bin/cursed-zig /usr/local/bin/
COPY --from=builder /src/zig-out/bin/cursed-stable /usr/local/bin/
COPY --from=builder /src/stdlib /usr/local/lib/cursed/stdlib

# Create non-root user
RUN useradd -m -s /bin/bash cursed
USER cursed
WORKDIR /home/cursed

ENTRYPOINT ["cursed-zig"]
```

#### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'

services:
  cursed-compiler:
    build: .
    image: cursed:latest
    volumes:
      - ./projects:/workspace
    working_dir: /workspace
    command: ["--help"]
    
  cursed-server:
    image: cursed:latest
    ports:
      - "8080:8080"
    volumes:
      - ./server:/app
    working_dir: /app
    command: ["server.csd"]
    environment:
      - CURSED_ENV=production
      - CURSED_LOG_LEVEL=info
```

### Kubernetes Deployment

```yaml
# k8s-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cursed-compiler
  labels:
    app: cursed-compiler
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
      - name: cursed
        image: cursed:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        env:
        - name: CURSED_ENV
          value: "production"
        - name: CURSED_LOG_LEVEL
          value: "info"
        volumeMounts:
        - name: cursed-stdlib
          mountPath: /usr/local/lib/cursed/stdlib
          readOnly: true
      volumes:
      - name: cursed-stdlib
        configMap:
          name: cursed-stdlib-config

---
apiVersion: v1
kind: Service
metadata:
  name: cursed-compiler-service
spec:
  selector:
    app: cursed-compiler
  ports:
    - protocol: TCP
      port: 80
      targetPort: 8080
  type: LoadBalancer
```

### System Service Installation

#### Systemd Service (Linux)
```ini
# /etc/systemd/system/cursed-compiler.service
[Unit]
Description=CURSED Compiler Service
After=network.target

[Service]
Type=forking
User=cursed
Group=cursed
WorkingDirectory=/opt/cursed
ExecStart=/usr/local/bin/cursed-zig server --daemon
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable cursed-compiler
sudo systemctl start cursed-compiler
sudo systemctl status cursed-compiler
```

#### LaunchD Service (macOS)
```xml
<!-- ~/Library/LaunchAgents/com.cursed.compiler.plist -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.cursed.compiler</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/cursed-zig</string>
        <string>server</string>
        <string>--daemon</string>
    </array>
    <key>WorkingDirectory</key>
    <string>/opt/cursed</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

```bash
# Load service
launchctl load ~/Library/LaunchAgents/com.cursed.compiler.plist
launchctl start com.cursed.compiler
```

## Configuration

### Environment Variables

```bash
# Core configuration
export CURSED_HOME="/opt/cursed"
export CURSED_STDLIB_PATH="/usr/local/lib/cursed/stdlib"
export CURSED_CACHE_DIR="$HOME/.cursed/cache"
export CURSED_LOG_LEVEL="info"  # debug, info, warn, error

# Performance tuning
export CURSED_PARALLEL_JOBS="$(nproc)"
export CURSED_MEMORY_LIMIT="4GB"
export CURSED_ENABLE_LTO="true"

# Security settings
export CURSED_SANDBOX_MODE="strict"
export CURSED_ALLOWED_SYSCALLS="read,write,open,close"
export CURSED_NETWORK_ACCESS="false"
```

### Configuration File

```toml
# ~/.cursed/config.toml
[compiler]
default_target = "native"
optimization_level = "fast"
enable_debug_info = false
parallel_compilation = true
max_parallel_jobs = 8

[runtime]
garbage_collector = "incremental"
stack_size = "1MB"
heap_size = "100MB"
enable_profiling = false

[security]
sandbox_mode = "strict"
memory_protection = true
stack_protection = true
allow_unsafe_code = false

[logging]
level = "info"
output = "stderr"
format = "json"
enable_colors = true

[stdlib]
search_paths = [
    "/usr/local/lib/cursed/stdlib",
    "~/.cursed/packages"
]
auto_import = ["vibez"]

[performance]
compilation_cache = true
cache_directory = "~/.cursed/cache"
cache_size_limit = "1GB"
enable_lto = true
profile_guided_optimization = false
```

## Network Installation

### Remote Installation Script

```bash
#!/bin/bash
# install.sh - Remote installation script

set -euo pipefail

CURSED_VERSION="${CURSED_VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

# Map architecture names
case "$ARCH" in
    x86_64) ARCH="x64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

# Download URL
if [ "$CURSED_VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/ghuntley/cursed/releases/latest/download/cursed-${OS}-${ARCH}.tar.gz"
else
    DOWNLOAD_URL="https://github.com/ghuntley/cursed/releases/download/${CURSED_VERSION}/cursed-${OS}-${ARCH}.tar.gz"
fi

echo "Installing CURSED for ${OS}-${ARCH}"
echo "Download URL: $DOWNLOAD_URL"

# Create temporary directory
TEMP_DIR="$(mktemp -d)"
trap "rm -rf $TEMP_DIR" EXIT

# Download and extract
cd "$TEMP_DIR"
curl -fsSL "$DOWNLOAD_URL" | tar xz

# Install binaries
sudo mkdir -p "$INSTALL_DIR"
sudo cp cursed-zig "$INSTALL_DIR/"
sudo cp cursed-stable "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/cursed-zig" "$INSTALL_DIR/cursed-stable"

# Install standard library
sudo mkdir -p /usr/local/lib/cursed
sudo cp -r stdlib /usr/local/lib/cursed/

echo "CURSED installed successfully!"
echo "Run 'cursed-zig --version' to verify installation"
```

Usage:
```bash
# Install latest version
curl -fsSL https://install.cursed.dev | bash

# Install specific version
curl -fsSL https://install.cursed.dev | CURSED_VERSION=v1.0.0 bash

# Install to custom directory
curl -fsSL https://install.cursed.dev | INSTALL_DIR=/opt/cursed/bin bash
```

## Verification and Testing

### Installation Verification

```bash
#!/bin/bash
# verify-installation.sh

echo "=== CURSED Installation Verification ==="

# Check binary availability
if command -v cursed-zig &> /dev/null; then
    echo "✓ cursed-zig found: $(which cursed-zig)"
    cursed-zig --version
else
    echo "✗ cursed-zig not found in PATH"
    exit 1
fi

if command -v cursed-stable &> /dev/null; then
    echo "✓ cursed-stable found: $(which cursed-stable)"
    cursed-stable --version
else
    echo "✗ cursed-stable not found in PATH"
    exit 1
fi

# Test basic functionality
echo "=== Testing Basic Functionality ==="
echo 'vibez.spill("Installation test successful!")' > /tmp/install_test.csd

if cursed-zig /tmp/install_test.csd 2>/dev/null | grep -q "Installation test successful"; then
    echo "✓ Basic execution test passed"
else
    echo "✗ Basic execution test failed"
    exit 1
fi

# Test standard library
echo "=== Testing Standard Library ==="
echo 'yeet "mathz"; vibez.spill("Math test:", abs_normie(-42))' > /tmp/stdlib_test.csd

if cursed-zig /tmp/stdlib_test.csd 2>/dev/null | grep -q "Math test: 42"; then
    echo "✓ Standard library test passed"
else
    echo "✗ Standard library test failed"
    exit 1
fi

# Test compilation
echo "=== Testing Compilation ==="
if cursed-zig --compile /tmp/install_test.csd 2>/dev/null && [ -x /tmp/install_test ]; then
    echo "✓ Compilation test passed"
    /tmp/install_test
    rm -f /tmp/install_test
else
    echo "✗ Compilation test failed"
    exit 1
fi

# Memory safety test (if valgrind available)
if command -v valgrind &> /dev/null; then
    echo "=== Testing Memory Safety ==="
    if valgrind --error-exitcode=1 --quiet cursed-zig /tmp/install_test.csd &>/dev/null; then
        echo "✓ Memory safety test passed"
    else
        echo "✗ Memory safety test failed"
        exit 1
    fi
fi

# Cleanup
rm -f /tmp/install_test.csd /tmp/stdlib_test.csd

echo "=== All Tests Passed! ==="
echo "CURSED is properly installed and ready to use."
```

### Performance Benchmarks

```bash
#!/bin/bash
# benchmark-installation.sh

echo "=== CURSED Performance Benchmarks ==="

# Compilation speed test
echo "Testing compilation speed..."
time cursed-zig --compile examples/large_program.csd

# Runtime performance test
echo "Testing runtime performance..."
time ./large_program

# Memory usage test
echo "Testing memory usage..."
/usr/bin/time -v cursed-zig examples/memory_test.csd

# Concurrent performance test
echo "Testing concurrency performance..."
cursed-zig examples/goroutine_benchmark.csd
```

## Troubleshooting Installation

### Common Issues

#### LLVM Not Found
```bash
# Ubuntu/Debian
sudo apt install llvm-15-dev libllvm15

# macOS
brew install llvm@15
export PATH="/opt/homebrew/opt/llvm@15/bin:$PATH"

# Set environment variables
export LLVM_CONFIG="llvm-config-15"
export LLVM_SYS_150_PREFIX="/usr/lib/llvm-15"
```

#### Zig Version Conflicts
```bash
# Check Zig version
zig version

# Update to correct version
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /opt/zig
sudo ln -sf /opt/zig/zig /usr/local/bin/zig
```

#### Permission Issues
```bash
# Fix binary permissions
sudo chmod +x /usr/local/bin/cursed-zig
sudo chmod +x /usr/local/bin/cursed-stable

# Fix stdlib permissions
sudo chown -R root:root /usr/local/lib/cursed
sudo chmod -R 755 /usr/local/lib/cursed
```

#### Memory Issues During Build
```bash
# Increase swap space
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Build with reduced parallelism
zig build -j1
```

For additional support, see the [Troubleshooting Guide](../support/troubleshooting.md) or visit our [GitHub Issues](https://github.com/ghuntley/cursed/issues).

## Next Steps

- [Getting Started Guide](../user-guide/getting-started.md)
- [Performance Optimization](performance.md)
- [Security Configuration](security.md)
- [Monitoring Setup](monitoring.md)
