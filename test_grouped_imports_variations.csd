vibe test_grouped_imports_variations

fr fr Test various grouped import syntax patterns

fr fr Single line grouped imports
yeet ( "stringz"; "mathz" )

fr fr Multi-line grouped imports
yeet (
    "crypto";
    "net";
    "os"
)

fr fr Mixed syntax
yeet "single_import"

yeet (
    "another_group";
    "more_modules"
)

slay main() {
    vibez.spill("Testing all grouped import variations")
    vibez.spill("All variations should compile successfully")
}
