#!/bin/bash

# CURSED Full Build Restore Script
# This script restores the full build configuration

echo "🔄 Restoring full build configuration for CURSED..."

# Check if backup files exist
if [ ! -f "Cargo.full.toml" ] || [ ! -f "src/lib.full.rs" ] || [ ! -f "src/main.full.rs" ]; then
    echo "❌ Error: Backup files not found. Cannot restore full build."
    echo "   Make sure you ran enable_minimal_build.sh first."
    exit 1
fi

# Restore original files
echo "📦 Restoring original files..."
cp Cargo.full.toml Cargo.toml
cp src/lib.full.rs src/lib.rs
cp src/main.full.rs src/main.rs

# Restore original modules
echo "🔄 Restoring original modules..."
for module in stdlib package_manager tools config bootstrap optimization profiling docs documentation object debug build_system cli repl lsp testing type_system types; do
    if [ -f "src/${module}/mod.full.rs" ]; then
        cp "src/${module}/mod.full.rs" "src/${module}/mod.rs"
        echo "   Restored src/${module}/mod.rs"
    fi
done

echo "✅ Full build configuration restored!"
echo ""
echo "📋 Changes made:"
echo "   • Cargo.full.toml -> Cargo.toml (restored)"
echo "   • src/lib.full.rs -> src/lib.rs (restored)"
echo "   • src/main.full.rs -> src/main.rs (restored)"
echo "   • All module implementations restored"
echo ""
echo "🏗️ You can now build with full features:"
echo "   cargo build"
echo "   cargo run -- --help"
