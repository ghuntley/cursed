#!/bin/bash

# CURSED Minimal Build Enable Script
# This script reduces build scope to core compiler functionality only

echo "🔧 Enabling minimal build configuration for CURSED..."

# Backup current files
echo "📦 Backing up current files..."
cp Cargo.toml Cargo.full.toml
cp src/lib.rs src/lib.full.rs
cp src/main.rs src/main.full.rs

# Switch to minimal configuration
echo "⚡ Switching to minimal configuration..."
cp Cargo.minimal.toml Cargo.toml
cp src/lib.minimal.rs src/lib.rs
cp src/main.minimal.rs src/main.rs

# Comment out heavy modules in src/lib.rs that may cause issues
echo "🚫 Disabling heavy modules that may cause compilation issues..."

# Create a minimal version of modules that might have heavy dependencies
echo "// Minimal module - heavy features disabled" > src/stdlib/mod.rs
echo "// Minimal module - heavy features disabled" > src/package_manager/mod.rs
echo "// Minimal module - heavy features disabled" > src/tools/mod.rs
echo "// Minimal module - heavy features disabled" > src/config/mod.rs
echo "// Minimal module - heavy features disabled" > src/bootstrap/mod.rs
echo "// Minimal module - heavy features disabled" > src/optimization/mod.rs
echo "// Minimal module - heavy features disabled" > src/profiling/mod.rs
echo "// Minimal module - heavy features disabled" > src/docs/mod.rs
echo "// Minimal module - heavy features disabled" > src/documentation/mod.rs
echo "// Minimal module - heavy features disabled" > src/object/mod.rs
echo "// Minimal module - heavy features disabled" > src/debug/mod.rs
echo "// Minimal module - heavy features disabled" > src/build_system/mod.rs
echo "// Minimal module - heavy features disabled" > src/cli/mod.rs
echo "// Minimal module - heavy features disabled" > src/repl/mod.rs
echo "// Minimal module - heavy features disabled" > src/lsp/mod.rs
echo "// Minimal module - heavy features disabled" > src/testing/mod.rs
echo "// Minimal module - heavy features disabled" > src/type_system/mod.rs
echo "// Minimal module - heavy features disabled" > src/types/mod.rs

# Backup original modules if they don't already have backups
for module in stdlib package_manager tools config bootstrap optimization profiling docs documentation object debug build_system cli repl lsp testing type_system types; do
    if [ -f "src/${module}/mod.rs" ] && [ ! -f "src/${module}/mod.full.rs" ]; then
        cp "src/${module}/mod.rs" "src/${module}/mod.full.rs"
    fi
done

echo "✅ Minimal build configuration enabled!"
echo ""
echo "📋 Changes made:"
echo "   • Cargo.toml -> Cargo.full.toml (backup)"
echo "   • Cargo.minimal.toml -> Cargo.toml (active)"
echo "   • src/lib.rs -> src/lib.full.rs (backup)"
echo "   • src/lib.minimal.rs -> src/lib.rs (active)"
echo "   • src/main.rs -> src/main.full.rs (backup)"
echo "   • src/main.minimal.rs -> src/main.rs (active)"
echo "   • Heavy modules disabled with stub implementations"
echo ""
echo "🏗️ You can now build with:"
echo "   cargo build"
echo "   cargo run -- check examples/test_simple.csd"
echo ""
echo "🔄 To restore full build, run: ./restore_full_build.sh"
