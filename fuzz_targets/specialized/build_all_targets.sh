#!/bin/bash
# Master build script for all fuzz targets

set -e

echo "🚀 Building all CURSED fuzz targets..."

# Run all build scripts
for build_script in build_*.sh; do
    if [ -f "$build_script" ] && [ "$build_script" != "build_all_targets.sh" ]; then
        echo "Running $build_script..."
        bash "$build_script"
    fi
done

echo "✅ All fuzz targets built successfully!"
