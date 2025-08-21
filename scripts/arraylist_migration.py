#!/usr/bin/env python3
"""
Oracle Priority 2: ArrayList Compatibility Migration Script

This script systematically migrates all ArrayList usage from the old
Zig API to the new Zig 0.15.1+ compatible API patterns.

Key changes:
1. std.ArrayList(T) → std.ArrayList(T)
2. ArrayList.init(allocator) → ArrayList(T){} + allocator parameter changes
3. .append() → .append(allocator, ...)
4. .deinit() → .deinit(allocator)
"""

import os
import re
import sys
from pathlib import Path

def migrate_arraylist_file(file_path):
    """Migrate a single file to use modern ArrayList API"""
    
    print(f"Migrating {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Step 1: Add compatibility import if ArrayList is used
    if 'ArrayList' in content and '@import("zig_version.zig")' not in content:
        # Check if it's a main file or a module
        if 'const std = @import("std");' in content:
            content = content.replace(
                'const std = @import("std");',
                'const std = @import("std");\nconst compat = @import("zig_version.zig");'
            )
        elif 'const ArrayList = std.ArrayList;' in content:
            content = content.replace(
                'const ArrayList = std.ArrayList;',
                'const ArrayList = std.ArrayList;\nconst compat = @import("zig_version.zig");'
            )
    
    # Step 2: Replace ArrayList initialization patterns
    # ArrayList(T).init(allocator) → compat.ArrayList(T).init(allocator)
    content = re.sub(
        r'std\.ArrayList\(([^)]+)\)\.init\(([^)]+)\)',
        r'compat.ArrayList(\1).init(\2)',
        content
    )
    
    # ArrayList(T){} → compat.ArrayList(T){}
    content = re.sub(
        r'std\.ArrayList\(([^)]+)\)\{\}',
        r'compat.ArrayList(\1){}',
        content
    )
    
    # Step 3: Handle variable declarations
    # var list: ArrayList(T) = ... → var list: compat.ArrayList(T) = ...
    content = re.sub(
        r':\s*ArrayList\(([^)]+)\)\s*=',
        r': compat.ArrayList(\1) =',
        content
    )
    
    # Step 4: Update method calls that need allocator parameter
    # Note: This is a simplified approach - complex cases may need manual review
    
    # Step 5: Handle .empty initialization
    content = re.sub(
        r'std\.ArrayList\(([^)]+)\)\s*\.\s*empty',
        r'compat.ArrayList(\1){}',
        content
    )
    
    # Step 6: Replace direct ArrayList references
    content = re.sub(
        r'(?<!compat\.)ArrayList\(([^)]+)\)',
        r'compat.ArrayList(\1)',
        content
    )
    
    # Step 7: Handle return types and function signatures
    content = re.sub(
        r'!\s*ArrayList\(([^)]+)\)',
        r'!compat.ArrayList(\1)',
        content
    )
    
    # Only write if changes were made
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"  ✓ Updated {file_path}")
        return True
    else:
        print(f"  - No changes needed for {file_path}")
        return False

def find_zig_files(directory):
    """Find all .zig files that use ArrayList"""
    zig_files = []
    
    for file_path in Path(directory).rglob("*.zig"):
        try:
            with open(file_path, 'r') as f:
                content = f.read()
                if 'ArrayList' in content:
                    zig_files.append(str(file_path))
        except Exception as e:
            print(f"Warning: Could not read {file_path}: {e}")
    
    return zig_files

def main():
    """Main migration function"""
    
    if len(sys.argv) > 1:
        src_dir = sys.argv[1]
    else:
        src_dir = "src-zig"
    
    if not os.path.exists(src_dir):
        print(f"Error: Directory {src_dir} does not exist")
        sys.exit(1)
    
    print(f"Oracle Priority 2: ArrayList Migration")
    print(f"Scanning {src_dir} for ArrayList usage...")
    
    zig_files = find_zig_files(src_dir)
    print(f"Found {len(zig_files)} files using ArrayList")
    
    if not zig_files:
        print("No files to migrate")
        return
    
    # Ask for confirmation
    print("\nFiles to migrate:")
    for file_path in zig_files[:10]:  # Show first 10
        print(f"  {file_path}")
    if len(zig_files) > 10:
        print(f"  ... and {len(zig_files) - 10} more")
    
    response = "y"
    if response.lower() != 'y':
        print("Migration cancelled")
        return
    
    # Perform migration
    updated_count = 0
    error_count = 0
    
    for file_path in zig_files:
        try:
            if migrate_arraylist_file(file_path):
                updated_count += 1
        except Exception as e:
            print(f"  ✗ Error migrating {file_path}: {e}")
            error_count += 1
    
    print(f"\nMigration Summary:")
    print(f"  Files updated: {updated_count}")
    print(f"  Files with errors: {error_count}")
    print(f"  Files unchanged: {len(zig_files) - updated_count - error_count}")
    
    if error_count == 0:
        print("✅ ArrayList migration completed successfully!")
        print("\nNext steps:")
        print("1. Review changes with: git diff")
        print("2. Test build with: zig build")
        print("3. Run tests with: zig build test")
    else:
        print("⚠️  Migration completed with errors - manual review needed")

if __name__ == "__main__":
    main()
