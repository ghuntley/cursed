// Test all types of import statements
yeet ( "fmt"; "strings"; "os"; "math" )
yeet "single_import"
yeet alias_import "aliased_module"
yeet { Symbol1, Symbol2 } from "selective_import"
yeet * from "wildcard_import"
yeet ( "net"; "http" )

slay main() {
    vibez.spill("Testing all import types")
    vibez.spill("Grouped imports: fmt, strings, os, math")
    vibez.spill("Single import: single_import")
    vibez.spill("Aliased import: alias_import")
    vibez.spill("Selective import: Symbol1, Symbol2")
    vibez.spill("Wildcard import: *")
    vibez.spill("Second grouped import: net, http")
}
