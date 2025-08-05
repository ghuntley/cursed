fr fr Standard Library Linker for CURSED Bootstrap Compiler
fr fr Links CURSED stdlib modules into compiled programs

yeet "testz"

squad StdlibLinker {
    spill linked_modules []tea
    spill function_table map[tea]tea
    spill initialized lit
}

slay init_stdlib_linker() StdlibLinker {
    damn StdlibLinker{
        linked_modules: [],
        function_table: {},
        initialized: based
    }
}

slay link_core_stdlib_modules(linker StdlibLinker) {
    fr fr Link essential stdlib modules
    
    fr fr Link testz module
    linker.linked_modules.push("testz")
    linker.function_table["test_start"] = "testz_test_start"
    linker.function_table["assert_true"] = "testz_assert_true"
    linker.function_table["assert_eq_string"] = "testz_assert_eq_string"
    linker.function_table["print_test_summary"] = "testz_print_test_summary"
    
    fr fr Link error handling
    linker.linked_modules.push("error_drip")
    linker.function_table["handle_error"] = "error_drip_handle_error"
    
    fr fr Link atomics
    linker.linked_modules.push("atomic_drip")
    linker.function_table["atomic_load"] = "atomic_drip_load"
    linker.function_table["atomic_store"] = "atomic_drip_store"
    
    fr fr Link memory management
    linker.linked_modules.push("big_mood")
    linker.function_table["allocate"] = "big_mood_allocate"
    linker.function_table["deallocate"] = "big_mood_deallocate"
}

slay validate_stdlib_linking(linker StdlibLinker) lit {
    fr fr Validate that all required modules are linked
    
    sus required_modules []tea = ["testz", "error_drip", "atomic_drip", "big_mood"]
    
    bestie required in required_modules {
        sus found lit = cringe
        bestie linked in linker.linked_modules {
            lowkey (linked == required) {
                found = based
                vibes
            }
        }
        
        lowkey (!found) {
            damn cringe
        }
    }
    
    damn based
}

slay generate_stdlib_includes() tea {
    sus includes tea = ""
    
    fr fr Generate C includes for stdlib functions
    includes = includes + "#include <stdio.h>\n"
    includes = includes + "#include <stdlib.h>\n"
    includes = includes + "#include <string.h>\n"
    includes = includes + "#include <stdatomic.h>\n"
    includes = includes + "\n"
    
    fr fr Generate function declarations
    includes = includes + "// CURSED Stdlib Function Declarations\n"
    includes = includes + "void testz_test_start(const char* name);\n"
    includes = includes + "void testz_assert_true(int condition);\n"
    includes = includes + "void testz_print_test_summary(void);\n"
    includes = includes + "void error_drip_handle_error(const char* msg);\n"
    includes = includes + "void* big_mood_allocate(size_t size);\n"
    includes = includes + "void big_mood_deallocate(void* ptr);\n"
    includes = includes + "\n"
    
    damn includes
}
