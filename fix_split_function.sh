#!/bin/bash

# Fix deprecated split function in all tools
for file in src-zig/tools/simple_*.zig; do
    if [ -f "$file" ]; then
        echo "Fixing split function in $file"
        # Replace std.mem.split(u8, text, "\n") with std.mem.splitScalar(u8, text, '\n')
        sed -i 's/std\.mem\.split(u8, \([^,]*\), "\\n")/std.mem.splitScalar(u8, \1, '"'"'\\n'"'"')/g' "$file"
        # Replace other split patterns
        sed -i 's/std\.mem\.split(u8, \([^,]*\), " ")/std.mem.splitScalar(u8, \1, '"'"' '"'"')/g' "$file"
    fi
done

echo "Split functions fixed"
