#!/bin/bash

# Test Linux x64 specific functionality

set -e

echo "Testing Linux x64 platform support..."

# Create platform test
cat > platform_test.💀 << 'EOF'
vibez.spill("Platform test for Linux x64")
sus arch tea = "x64"
sus platform tea = "linux"
vibez.spill("Architecture:", arch)
vibez.spill("Platform:", platform)
EOF

# Test both modes
./cursed-unified platform_test.💀
./cursed-unified --compile platform_test.💀

if [ -f ./platform_test ]; then
    ./platform_test
    rm -f platform_test
fi

# Cleanup
rm -f platform_test.💀

echo "Linux x64 platform test completed"
exit 0
