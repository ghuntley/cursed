# Stdlib Module Naming Consistency Fixes

## Issues Found

### 1. JSON Module Duplication
- **Problem**: Both `json` and `json_tea` modules exist
- **Recommendation**: Keep `json` as primary, deprecate `json_tea`
- **Action**: 
  ```bash
  mv stdlib/json_tea stdlib/json_deprecated
  # Update any imports from json_tea to json
  ```

### 2. Regex Module Confusion  
- **Problem**: Both `regex` and `regex_vibez` modules exist
- **Recommendation**: Keep `regex` for core functionality, `regex_vibez` for advanced features
- **Action**: Clearly document differences and use cases

### 3. Crypto Module Proliferation
- **Problem**: `crypto`, `cryptz`, `crypto_complete` modules
- **Recommendation**: 
  - `crypto` - Core cryptographic functions
  - `crypto_complete` - Extended crypto with additional algorithms
  - Deprecate `cryptz` (unclear naming)
- **Action**:
  ```bash
  mv stdlib/cryptz stdlib/cryptz_deprecated
  # Merge useful functions into crypto or crypto_complete
  ```

### 4. Collections Module Hierarchy
- **Problem**: `collections`, `collections_simple`, `collections_advanced`
- **Recommendation**: Maintain hierarchy but clarify purpose:
  - `collections` - Standard data structures (HashMap, Vec, etc.)
  - `collections_simple` - Basic implementations for learning
  - `collections_advanced` - High-performance, specialized structures
- **Action**: Add clear documentation to each module's README

## Standardized Naming Convention

### Module Name Patterns
1. **Core modules**: Simple names (`math`, `string`, `json`)
2. **Enhanced modules**: `module_enhanced` or `module_pro`
3. **Simple/learning modules**: `module_simple`
4. **Advanced modules**: `module_advanced`
5. **Experimental modules**: `module_experimental`

### Deprecation Strategy
1. Mark deprecated modules with `_deprecated` suffix
2. Add deprecation warnings in module comments
3. Update all imports to use new names
4. Remove deprecated modules after 1 version cycle

## Implementation Commands

```bash
# Fix JSON duplication
cd stdlib
mv json_tea json_tea_deprecated
echo "# DEPRECATED: Use 'json' module instead" > json_tea_deprecated/DEPRECATED.md

# Fix crypto proliferation  
mv cryptz cryptz_deprecated
echo "# DEPRECATED: Functions moved to 'crypto' or 'crypto_complete'" > cryptz_deprecated/DEPRECATED.md

# Add clear documentation
for module in collections collections_simple collections_advanced; do
    echo "# Purpose and usage documentation needed" >> $module/README.md
done

# Verify no broken imports
grep -r "yeet.*json_tea" . || echo "No json_tea imports found"
grep -r "yeet.*cryptz" . || echo "No cryptz imports found"
```

## Testing After Fixes

```bash
# Test primary modules work
cargo run --bin cursed stdlib/json/test_json.csd
cargo run --bin cursed stdlib/crypto/test_crypto.csd  
cargo run --bin cursed stdlib/collections/test_collections.csd

# Verify deprecated modules are not imported
find stdlib -name "*.csd" -exec grep -l "json_tea\|cryptz" {} \;
```

## Documentation Updates Needed

1. **Module Index**: Update main stdlib README with clear module hierarchy
2. **Migration Guide**: Document changes for users updating code
3. **Best Practices**: Guidelines for choosing between simple/standard/advanced variants

## Priority
- **HIGH**: JSON and crypto fixes (affect many projects)
- **MEDIUM**: Collections clarification (affects advanced users)
- **LOW**: Regex disambiguation (specialized use cases)
