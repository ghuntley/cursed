#!/bin/bash

# Test import system functionality

set -e

# Test basic import functionality
cat > import_test.csd << 'EOF'
yeet "testz"

test_start("import test")
print_test_summary()
EOF

echo "Testing import system..."
./cursed-unified import_test.csd

# Cleanup
rm -f import_test.csd

exit 0
