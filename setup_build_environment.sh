#!/bin/bash

# CURSED Build Environment Setup Script
# Fixes C compiler/toolchain setup issues and ensures proper LLVM configuration

set -e

echo "🔧 Setting up CURSED build environment..."

# 1. Install required system packages
echo "📦 Installing system dependencies..."
sudo apt update
sudo apt install -y \
    llvm-18-dev \
    libllvm18 \
    llvm-18-tools \
    clang-18 \
    clang-tools-18 \
    libc6-dev \
    build-essential \
    pkg-config \
    crossbuild-essential-arm64 \
    crossbuild-essential-amd64 \
    mingw-w64 \
    mingw-w64-tools

# 2. Create symlinks for LLVM tools
echo "🔗 Creating LLVM tool symlinks..."
sudo ln -sf /usr/bin/llvm-config-18 /usr/bin/llvm-config
sudo ln -sf /usr/bin/llvm-ar-18 /usr/bin/llvm-ar

# 3. Set up environment variables
echo "🌍 Setting up environment variables..."
export LLVM_SYS_181_PREFIX="/usr/lib/llvm-18"
export LLVM_CONFIG_PATH="/usr/bin/llvm-config-18"
export CC="/usr/bin/gcc"
export CXX="/usr/bin/g++"
export AR="/usr/bin/ar"
export RANLIB="/usr/bin/ranlib"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig"

# 4. Write environment setup to file for sourcing
cat > ~/.cursed_env << 'EOF'
# CURSED Build Environment Variables
export LLVM_SYS_181_PREFIX="/usr/lib/llvm-18"
export LLVM_CONFIG_PATH="/usr/bin/llvm-config-18"
export CC="/usr/bin/gcc"
export CXX="/usr/bin/g++"
export AR="/usr/bin/ar" 
export RANLIB="/usr/bin/ranlib"
export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig"
export PATH="/usr/bin:$PATH"

# Cross-compilation environment
export CC_aarch64_unknown_linux_gnu="aarch64-linux-gnu-gcc"
export CXX_aarch64_unknown_linux_gnu="aarch64-linux-gnu-g++"
export AR_aarch64_unknown_linux_gnu="aarch64-linux-gnu-ar"
export CC_x86_64_pc_windows_gnu="x86_64-w64-mingw32-gcc"
export CXX_x86_64_pc_windows_gnu="x86_64-w64-mingw32-g++"
export AR_x86_64_pc_windows_gnu="x86_64-w64-mingw32-ar"

echo "✅ CURSED build environment loaded"
EOF

# 5. Source the environment
source ~/.cursed_env

# 6. Verify installation
echo "🔍 Verifying installation..."
echo "LLVM version: $(llvm-config --version)"
echo "GCC version: $(gcc --version | head -1)"
echo "Clang version: $(clang-18 --version | head -1)"
echo "Zig version: $(zig version)"

# 7. Test build
echo "🔨 Testing CURSED build..."
zig build

# 8. Test basic CURSED execution
echo "🧪 Testing CURSED execution..."
echo 'vibez.spill("Build environment setup complete!")' > test_env_setup.csd
./zig-out/bin/cursed test_env_setup.csd
rm test_env_setup.csd

echo ""
echo "✅ CURSED build environment setup complete!"
echo ""
echo "To use this environment in the future, run:"
echo "  source ~/.cursed_env"
echo ""
echo "Or add this line to your ~/.bashrc:"
echo "  source ~/.cursed_env"
