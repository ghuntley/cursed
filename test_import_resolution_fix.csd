# Test all import patterns that were previously inconsistent

# Direct stdlib module names
yeet "testz"
yeet "csv"
yeet "json"
yeet "math"
yeet "string"
yeet "crypto"

# Stdlib with prefix
yeet "stdlib/collections"
yeet "stdlib/network"

# Relative imports (commonly used in stdlib modules)
yeet "../testz/mod"
yeet "./mod"

# Legacy imports for backward compatibility
yeet "mathz"
yeet "stringz"
yeet "ioz"

slay main() {
    vibez.spill("✅ All import patterns resolved successfully!")
    vibez.spill("This demonstrates the fixed module import system.")
}
