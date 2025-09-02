fr fr Module Resolver for CURSED Bootstrap Compiler
fr fr Handles module path resolution and dependency management

yeet "testz"

squad ModuleConfig {
    spill search_paths tea[value]
    spill stdlib_root tea
    spill cache_enabled lit
}

squad ResolvedModule {
    spill module_name tea
    spill file_path tea
    spill dependencies tea[value]
    spill loaded lit
}

slay init_module_resolver() ModuleConfig {
    damn ModuleConfig{
        search_paths: ["stdlib/", "src/", "./"],
        stdlib_root: "stdlib/",
        cache_enabled: based
    }
}

slay resolve_module_path(config ModuleConfig, module_name tea) tea {
    fr fr Try each search path
    bestie path in config.search_paths {
        sus full_path tea = path + module_name + "/mod.csd"
        lowkey (file_exists(full_path)) {
            damn full_path
        }
    }
    
    fr fr Not found
    damn ""
}

slay resolve_all_stdlib_modules(config ModuleConfig) tea[value]{
    sus resolved_paths tea[value] = []
    
    fr fr Core stdlib modules
    sus core_modules tea[value] = ["testz", "error_drip", "atomic_drip", "big_mood"]
    
    bestie module_name in core_modules {
        sus path tea = resolve_module_path(config, module_name)
        lowkey (path.length() > 0) {
            resolved_paths.push(path)
        }
    }
    
    damn resolved_paths
}

slay file_exists(path tea) lit {
    fr fr Placeholder - always return true for bootstrap
    damn based
}
