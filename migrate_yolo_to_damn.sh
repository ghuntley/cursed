#!/bin/bash

# Script to migrate "yolo" return statements to "damn" in CURSED codebase
# This maintains backward compatibility while encouraging the preferred syntax

echo "Starting migration from 'yolo' to 'damn' return statements..."

# Count initial occurrences
echo "Before migration:"
echo "  yolo occurrences: $(grep -r "yolo" --include="*.csd" . | wc -l)"
echo "  damn occurrences: $(grep -r "damn" --include="*.csd" . | wc -l)"

# Create backup directory
mkdir -p /tmp/cursed_yolo_backup
cp -r stdlib/ tests/ examples/ /tmp/cursed_yolo_backup/ 2>/dev/null || true

# Function to safely replace yolo with damn in files
migrate_file() {
    local file="$1"
    echo "Processing: $file"
    
    # Use sed to replace standalone "yolo" with "damn" 
    # Only replace whole words to avoid replacing "yolo" inside other words
    sed -i 's/\byolo\b/damn/g' "$file"
}

# Find and process all .csd files
find . -name "*.csd" -type f | while read -r file; do
    if grep -q '\byolo\b' "$file"; then
        migrate_file "$file"
    fi
done

# Count final occurrences
echo -e "\nAfter migration:"
echo "  yolo occurrences: $(grep -r "yolo" --include="*.csd" . | wc -l)"
echo "  damn occurrences: $(grep -r "damn" --include="*.csd" . | wc -l)"

echo -e "\nMigration completed!"
echo "Backup created in: /tmp/cursed_yolo_backup"
echo "Please test the changes before committing."
