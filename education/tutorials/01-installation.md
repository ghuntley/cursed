# Installing CURSED

Get CURSED up and running on your system in just a few minutes.

## Quick Install (Recommended)

```bash
# Install CURSED with automatic installer
curl -sSf https://install.cursedlang.org | sh

# Add to PATH (add to your .bashrc/.zshrc)
export PATH="$HOME/.cursed/bin:$PATH"

# Verify installation
cursed-zig --version
```

## Manual Installation

### Prerequisites
- Zig 0.11+ (for building from source)
- LLVM 16+ (for compilation features)
- Git (for cloning repository)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the compiler
zig build

# Test installation
./zig-out/bin/cursed-zig --version
```

### Add to PATH

```bash
# Add CURSED tools to your PATH
echo 'export PATH="'$(pwd)'/zig-out/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Platform-Specific Instructions

### Linux (Ubuntu/Debian)
```bash
# Install dependencies
sudo apt update
sudo apt install build-essential llvm-16-dev libclang-16-dev

# Install Zig
wget https://ziglang.org/builds/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /opt/zig
echo 'export PATH="/opt/zig:$PATH"' >> ~/.bashrc
```

### macOS
```bash
# Install dependencies with Homebrew
brew install llvm@16 zig

# Set LLVM path
echo 'export PATH="/opt/homebrew/opt/llvm@16/bin:$PATH"' >> ~/.zshrc
```

### Windows
```powershell
# Install with Chocolatey
choco install zig llvm

# Or download manually from:
# - Zig: https://ziglang.org/download/
# - LLVM: https://github.com/llvm/llvm-project/releases
```

## Verify Installation

Create a test file to verify everything works:

```bash
# Create hello.csd
echo 'yeet "vibez"; vibez.spill("Hello, CURSED!")' > hello.csd

# Run with interpreter
cursed-zig hello.csd

# Compile to binary
cursed-zig --compile hello.csd
./hello
```

**Expected output:**
```
Hello, CURSED!
```

## IDE Setup

### VS Code (Recommended)
```bash
code --install-extension cursed-lang.cursed-vscode
```

### Vim/Neovim
```vim
" Add to your .vimrc
Plug 'cursed-lang/vim-cursed'
```

## Next Steps

✅ Installation complete!  
👉 Continue to [CURSED in 30 Minutes](./02-quick-start.md)

## Troubleshooting

**Build fails with "undefined symbol"?**
```bash
# Clean rebuild
rm -rf zig-cache/ zig-out/
zig build clean && zig build
```

**LLVM linking issues?**
```bash
# Set LLVM path explicitly
export LLVM_SYS_160_PREFIX=/usr/lib/llvm-16
zig build
```

**Permission errors?**
```bash
# Don't use sudo with curl installer
# Install to user directory instead
curl -sSf https://install.cursedlang.org | sh -s -- --no-sudo
```

Need help? Join our [Discord community](../community/discord.md)!
