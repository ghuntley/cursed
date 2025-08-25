#!/bin/bash

# Fix spec file inconsistencies script

echo "Fixing CURSED specification consistency issues..."

# Fix boolean literal inconsistencies
find specs/ -name "*.md" -exec sed -i 's/if err != cap/if err != nah/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/!= cap/!= nah/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/== cap/== nah/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/yolo cap/yolo nah/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/damn cap/damn nah/g' {} \;

# Fix if/else keywords
find specs/ -name "*.md" -exec sed -i 's/lowkey /ready /g' {} \;
find specs/ -name "*.md" -exec sed -i 's/} highkey {/} otherwise {/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/highkey /otherwise /g' {} \;

# Fix block comments in examples (where they appear in code blocks)
find specs/ -name "*.md" -exec sed -i 's|/\* fr fr|no cap|g' {} \;
find specs/ -name "*.md" -exec sed -i 's|fr fr \*/|on god|g' {} \;

# Fix channel operators in examples  
find specs/ -name "*.md" -exec sed -i 's/ch <-/dm_send(ch,/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/<- ch/dm_recv(ch)/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/<-ch/dm_recv(ch)/g' {} \;

# Fix specific boolean literal references (avoiding function names)
find specs/ -name "*.md" -exec sed -i 's/: `based` if.*`cap` otherwise/: `based` if valid, `cringe` otherwise/g' {} \;
find specs/ -name "*.md" -exec sed -i 's/Returns.*"cap"/Returns "cringe"/g' {} \;

# Fix remaining standalone cap references that should be cringe
find specs/ -name "*.md" -exec sed -i 's/"cap"/"cringe"/g' {} \;

echo "Specification consistency fixes applied."
echo "Note: Manual review recommended for context-specific changes."
