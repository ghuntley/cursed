#!/bin/bash

# Fix all tests that reference the old common module
find tests -name "*.rs" -exec grep -l "mod common;" {} \; | while read file; do
    echo "Fixing $file"
    sed -i 's/mod common;/mod common;/' "$file"
    # Also handle the #[path = "common.rs"] lines
    sed -i '/#\[path = "common.rs"\]/d' "$file"
done

# Fix pub mod common; references
find tests -name "*.rs" -exec grep -l "pub mod common;" {} \; | while read file; do
    echo "Fixing pub mod common in $file"
    sed -i 's/pub mod common;/mod common;/' "$file"
    sed -i '/#\[path = "common.rs"\]/d' "$file"
done

echo "Fixed common module references"
