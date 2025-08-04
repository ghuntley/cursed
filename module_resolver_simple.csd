fr fr Simple Module Resolver for Bootstrap Testing

squad ModuleConfig {
    spill stdlib_path tea
    spill resolved_modules []tea
}

slay init_module_resolver() ModuleConfig {
    sus resolved []tea = []tea{}
    damn ModuleConfig{
        stdlib_path: "stdlib/",
        resolved_modules: resolved
    }
}

slay resolve_all_stdlib_modules(config ModuleConfig) []tea {
    sus modules []tea = []tea{}
    modules.push("testz")
    modules.push("vibez") 
    modules.push("string_simple")
    modules.push("core")
    
    vibez.spill("Resolved", modules.len(), "stdlib modules")
    damn modules
}
