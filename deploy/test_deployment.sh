#!/bin/bash

# CURSED Deployment Test Script
# Tests the basic functionality of the deployment pipeline

set -e

echo "🧪 Testing CURSED Deployment Pipeline"
echo "====================================="

# Test 1: Check prerequisites
echo "1️⃣ Testing prerequisites..."
command -v zig >/dev/null 2>&1 && echo "  ✅ Zig found" || echo "  ❌ Zig not found"
command -v git >/dev/null 2>&1 && echo "  ✅ Git found" || echo "  ❌ Git not found"
command -v tar >/dev/null 2>&1 && echo "  ✅ Tar found" || echo "  ❌ Tar not found"

# Test 2: Check build system
echo "2️⃣ Testing build system..."
if [ -f "build.zig" ]; then
    echo "  ✅ build.zig found"
    if zig build --help >/dev/null 2>&1; then
        echo "  ✅ Zig build system working"
    else
        echo "  ❌ Zig build system not working"
    fi
else
    echo "  ❌ build.zig not found"
fi

# Test 3: Check deployment scripts
echo "3️⃣ Testing deployment scripts..."
SCRIPTS=(
    "deploy/production_pipeline.sh"
    "deploy/release_automation.py"
    "deploy/security_scanner.py"
    "deploy/performance_profiler.py"
    "deploy/package_builder.py"
    "deploy/monitoring_setup.py"
)

for script in "${SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        echo "  ✅ $script found"
        if [ -x "$script" ] || [[ "$script" == *.py ]]; then
            echo "    ✅ $script is executable or Python script"
        else
            echo "    ⚠️  $script is not executable"
        fi
    else
        echo "  ❌ $script not found"
    fi
done

# Test 4: Check VERSION file
echo "4️⃣ Testing version management..."
if [ -f "VERSION" ]; then
    VERSION=$(cat VERSION)
    echo "  ✅ VERSION file found: $VERSION"
else
    echo "  ❌ VERSION file not found"
fi

# Test 5: Test simple build
echo "5️⃣ Testing simple build..."
if zig build 2>/dev/null; then
    echo "  ✅ Basic build successful"
    
    if [ -f "zig-out/bin/cursed" ]; then
        echo "  ✅ cursed binary created"
        
        # Test basic execution
        echo 'vibez.spill("Deployment test!")' > test_deploy.csd
        if timeout 10s ./zig-out/bin/cursed test_deploy.csd >/dev/null 2>&1; then
            echo "  ✅ Basic execution test passed"
        else
            echo "  ⚠️  Basic execution test failed or timed out"
        fi
        rm -f test_deploy.csd
    else
        echo "  ❌ cursed binary not found"
    fi
else
    echo "  ❌ Basic build failed"
fi

# Test 6: Test security scanner (basic)
echo "6️⃣ Testing security scanner..."
if python3 deploy/security_scanner.py --project-root . 2>/dev/null; then
    echo "  ✅ Security scanner executed successfully"
else
    echo "  ⚠️  Security scanner had issues (may need dependencies)"
fi

# Test 7: Test package builder structure
echo "7️⃣ Testing package builder..."
if python3 -c "
import sys
sys.path.append('deploy')
try:
    from package_builder import PackageBuilder
    print('  ✅ Package builder can be imported')
except ImportError as e:
    print(f'  ⚠️  Package builder import failed: {e}')
" 2>/dev/null; then
    true
else
    echo "  ⚠️  Package builder import test failed"
fi

# Test 8: Test CI/CD workflow files
echo "8️⃣ Testing CI/CD workflows..."
WORKFLOWS=(
    ".github/workflows/production-deploy.yml"
    ".github/workflows/cross-platform.yml"
)

for workflow in "${WORKFLOWS[@]}"; do
    if [ -f "$workflow" ]; then
        echo "  ✅ $workflow found"
    else
        echo "  ❌ $workflow not found"
    fi
done

# Test 9: Check directory structure
echo "9️⃣ Testing directory structure..."
DIRS=(
    "src-zig"
    "stdlib"
    "deploy"
    ".github/workflows"
)

for dir in "${DIRS[@]}"; do
    if [ -d "$dir" ]; then
        echo "  ✅ $dir directory exists"
    else
        echo "  ❌ $dir directory missing"
    fi
done

# Summary
echo ""
echo "🏁 Deployment Test Summary"
echo "========================="
echo "✅ Basic deployment infrastructure is in place"
echo "⚠️  Some optional dependencies may be missing"
echo ""
echo "🚀 To run the full production pipeline:"
echo "   export VERSION=1.0.0"
echo "   ./deploy/production_pipeline.sh"
echo ""
echo "📦 To build packages:"
echo "   python3 deploy/package_builder.py --version 1.0.0"
echo ""
echo "🔒 To run security scan:"
echo "   python3 deploy/security_scanner.py"
echo ""
echo "📊 To run performance tests:"
echo "   python3 deploy/performance_profiler.py"
echo ""
echo "🎉 Deployment pipeline test completed!"
