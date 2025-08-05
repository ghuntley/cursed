// Tests for grouped import functionality
yeet ( "fmt"; "strings"; "os" )
yeet "single_import"
yeet ( "math"; "net" )

slay test_grouped_imports() tea {
    vibez.spill("Testing grouped imports")
    vibez.spill("✅ Grouped imports: fmt, strings, os")
    vibez.spill("✅ Single import: single_import")
    vibez.spill("✅ Second grouped import: math, net")
    damn "All tests passed"
}

slay main() {
    sus result tea = test_grouped_imports()
    vibez.spill(result)
}
