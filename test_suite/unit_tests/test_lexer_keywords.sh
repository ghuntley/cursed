#!/bin/bash

# Test lexer keyword recognition

set -e

# Test all CURSED keywords
cat > keyword_test.csd << 'EOF'
fr fr Test all CURSED keywords
sus variable drip = 42
slay function_name() normie {
    damn 42
}
squad StructName {
    spill field normie
}
collab InterfaceName {
    slay method()
}
bestie condition {
    vibes
}
match value {
    42 => vibez.spill("forty-two")
    _ => vibez.spill("other")
}
based
cringe
yeet "module"
EOF

# Test that keywords are properly recognized
./cursed-unified keyword_test.csd

# Cleanup
rm -f keyword_test.csd

exit 0
