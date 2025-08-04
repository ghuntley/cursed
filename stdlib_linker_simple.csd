fr fr Simple Stdlib Linker for Bootstrap Testing

squad StdlibLinker {
    spill linked_modules []tea
    spill symbol_count normie
}

slay init_stdlib_linker() StdlibLinker {
    sus modules []tea = []tea{}
    damn StdlibLinker{
        linked_modules: modules,
        symbol_count: 0
    }
}

slay link_core_stdlib_modules(linker StdlibLinker) {
    linker.linked_modules.push("testz")
    linker.linked_modules.push("vibez")
    linker.linked_modules.push("string_simple")
    linker.linked_modules.push("core")
    linker.symbol_count = 20
    
    vibez.spill("Linked", linker.linked_modules.len(), "core stdlib modules")
}

slay validate_stdlib_linking(linker StdlibLinker) lit {
    sus valid lit = linker.linked_modules.len() > 0 && linker.symbol_count > 0
    bestie (valid) {
        vibez.spill("Stdlib linking validation: PASSED")
    } capish {
        vibez.spill("Stdlib linking validation: FAILED")
    }
    damn valid
}
