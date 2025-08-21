# CURSED v1.0.0-rc2 Bug Bash Setup Guide

## Quick Setup (5 Minutes)

### Installation Options

#### Option 1: Curl Script (Recommended)
```bash
# Install CURSED v1.0.0-rc2  
curl -sSf https://install.cursedlang.org/rc2 | sh
source ~/.bashrc  # or restart terminal

# Verify installation
cursed-zig --version
```

#### Option 2: Manual Build
```bash
# Prerequisites: Zig 0.11+ required
git clone https://github.com/ghuntley/cursed.git
cd cursed
zig build

# Add to PATH
echo 'export PATH="$(pwd)/zig-out/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Option 3: Container Setup
```bash
# Docker setup for isolated testing
docker run -it --rm -v $(pwd):/workspace cursedlang/cursed:v1.0.0-rc2
cd /workspace
```

### Verification Test
```bash
# Create test file
echo 'vibez.spill("CURSED Bug Bash Ready!")' > hello.csd

# Run interpreter mode (should always work)
cursed-zig hello.csd

# Test compilation mode (report if fails)
cursed-zig --compile hello.csd && ./hello
```

## Platform-Specific Setup

### Linux Setup
```bash
# Ubuntu/Debian dependencies
sudo apt update
sudo apt install build-essential llvm-16-dev libclang-16-dev

# Arch Linux dependencies  
sudo pacman -S base-devel llvm clang

# Verify setup
cursed-zig --version
ldd $(which cursed-zig)  # Check dynamic linking
```

### macOS Setup  
```bash
# Install Xcode command line tools
xcode-select --install

# Homebrew dependencies (if using Homebrew)
brew install llvm@16

# For ARM64 Macs
export PATH="/opt/homebrew/opt/llvm@16/bin:$PATH"

# For Intel Macs  
export PATH="/usr/local/opt/llvm@16/bin:$PATH"
```

### Windows Setup
```bash
# Using WSL2 (recommended)
wsl --install Ubuntu-22.04
# Then follow Linux setup inside WSL

# Native Windows (advanced)
# Install Visual Studio 2022 with C++ tools
# Install LLVM 16 from https://llvm.org/releases/
# Set PATH to include LLVM bin directory
```

## Test Environment Setup

### Bug Bash Test Suite
```bash
# Download community test scenarios
mkdir ~/cursed-bugbash
cd ~/cursed-bugbash

# Get test files
curl -O https://raw.githubusercontent.com/ghuntley/cursed/main/community_testing/test_scenarios.csd
curl -O https://raw.githubusercontent.com/ghuntley/cursed/main/community_testing/bug_report_template.md

# Run comprehensive test
cursed-zig test_scenarios.csd
```

### Performance Testing Setup
```bash
# Install benchmarking tools
sudo apt install hyperfine valgrind  # Linux
brew install hyperfine              # macOS

# Memory leak testing
valgrind --leak-check=full cursed-zig test_scenarios.csd

# Performance benchmarking  
hyperfine 'cursed-zig test_scenarios.csd'
hyperfine 'zig build clean && zig build'  # Compilation speed
```

### Cross-Platform Testing
```bash
# Setup cross-compilation targets
zig targets  # List available targets

# Test cross-compilation (advanced)
cursed-zig --compile --target=x86_64-linux test.csd
cursed-zig --compile --target=aarch64-macos test.csd  
cursed-zig --compile --target=x86_64-windows test.csd
cursed-zig --compile --target=wasm32-wasi test.csd
```

## Development Environment Integration

### VS Code Setup
```bash
# Install CURSED extension (if available)
code --install-extension cursed-lang.cursed-vscode

# Manual LSP setup
curl -O https://raw.githubusercontent.com/ghuntley/cursed/main/cursed-vscode/cursed-language-config.json
```

### Vim/Neovim Setup
```bash
# Install syntax highlighting
git clone https://github.com/cursed-lang/vim-cursed.git ~/.vim/pack/cursed/start/vim-cursed

# LSP integration (Neovim with nvim-lsp)
# Add to init.lua:
require'lspconfig'.cursed_lsp.setup{
    cmd = {'cursed-lsp', '--stdio'},
    filetypes = {'cursed'},
}
```

### Emacs Setup
```elisp
;; Add to .emacs or init.el
(add-to-list 'auto-mode-alist '("\\.csd\\'" . cursed-mode))

;; LSP setup with lsp-mode
(use-package lsp-mode
  :hook (cursed-mode . lsp)
  :commands lsp
  :config
  (add-to-list 'lsp-language-id-configuration '(cursed-mode . "cursed"))
  (lsp-register-client
   (make-lsp-client :new-connection (lsp-stdio-connection "cursed-lsp")
                    :activation-fn (lsp-activate-on "cursed")
                    :server-id 'cursed-lsp)))
```

## Testing Methodology

### Bug Hunting Checklist
- [ ] **Smoke Test**: Basic functionality works
- [ ] **Feature Test**: Specific language features
- [ ] **Edge Cases**: Boundary conditions and error paths  
- [ ] **Integration**: Multiple features working together
- [ ] **Performance**: Memory usage and execution speed
- [ ] **Platform**: OS-specific behavior differences

### Test Categories by Skill Level

#### Beginner (5-10 minutes)
```bash
# Run pre-written test scenarios
cursed-zig test_scenarios.csd

# Try basic syntax variations
echo 'sus x drip = 42; vibez.spill(x)' | cursed-zig
echo 'sus y tea = "test"; vibez.spill(y)' | cursed-zig
```

#### Intermediate (30-60 minutes)  
```bash
# Test standard library modules
echo 'yeet "mathz"; vibez.spill(sqrt(16))' | cursed-zig
echo 'yeet "stringz"; vibez.spill(to_upper("hello"))' | cursed-zig

# Test error conditions
echo 'sus arr []drip = [1,2,3]; vibez.spill(arr[10])' | cursed-zig
```

#### Advanced (2+ hours)
```bash
# Stress testing
cursed-zig --compile large_program.csd
valgrind ./large_program

# Cross-compilation testing
for target in x86_64-linux aarch64-linux x86_64-windows; do
    cursed-zig --compile --target=$target test.csd
done
```

## Troubleshooting Common Issues

### Build Failures
```bash
# Clean rebuild
rm -rf zig-cache/ zig-out/
zig build clean && zig build

# Check dependencies
zig version  # Should be 0.11+
llvm-config --version  # Should be 16+
```

### Runtime Crashes
```bash
# Debug mode compilation
zig build -Doptimize=Debug

# Core dump analysis
gdb ./zig-out/bin/cursed-zig core
(gdb) bt  # Get stack trace
```

### Memory Issues  
```bash
# Check for leaks
valgrind --leak-check=full --show-leak-kinds=all cursed-zig test.csd

# Check memory usage
/usr/bin/time -v cursed-zig large_test.csd
```

### Cross-Platform Issues
```bash
# Linux-specific
strace cursed-zig test.csd  # System call tracing

# macOS-specific  
dtruss cursed-zig test.csd  # System call tracing
otool -L ./zig-out/bin/cursed-zig  # Check dependencies

# Windows-specific (in WSL)
# Test both WSL and native Windows compilation
```

## Community Collaboration

### Discord Integration
- Join #bug-bash channel for real-time help
- Share interesting findings in #discoveries
- Ask questions in #q-and-a
- Coordinate testing efforts in #coordination

### GitHub Workflow
```bash
# Fork repository for contributions
gh repo fork ghuntley/cursed

# Create bug reports
gh issue create --template bug_report.md

# Track progress
gh issue list --label "bug-bash" --state open
```

### Progress Tracking
Keep a testing log:
```markdown
# My Bug Bash Progress

## Day 1 (Aug 21)
- [x] Environment setup complete
- [x] Basic smoke tests passed  
- [x] Found issue with array bounds checking (#123)
- [ ] Test concurrency features

## Day 2 (Aug 22)  
- [x] Standard library testing
- [x] Cross-compilation on Linux
- [ ] Performance benchmarking
```

## Success Metrics

Track your contributions:
- **Bugs Found**: Quality over quantity
- **Test Coverage**: Features and platforms tested
- **Community Help**: Assistance provided to others
- **Reproduction Quality**: Clear, minimal bug reports

## Next Steps

1. **Setup Complete**: Verify installation with smoke test
2. **Choose Testing Focus**: Pick area based on your expertise  
3. **Join Community**: Connect with other testers in Discord
4. **Start Testing**: Begin with test_scenarios.csd
5. **Report Issues**: Use GitHub Issues with proper templates
6. **Help Others**: Share knowledge and assist fellow testers

**Ready to hunt some bugs? Let's make CURSED v1.0.0 rock solid! 🚀**
