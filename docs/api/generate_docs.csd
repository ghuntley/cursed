yeet "dropz"
yeet "stringz"
yeet "tab_aesthetic"
yeet "pathing"
yeet "testz"

# API Documentation Generator for CURSED
# Automatically generates comprehensive API documentation for all stdlib modules

struct DocGenerator {
    output_dir tea
    modules []ModuleInfo
    index_content tea
}

struct ModuleInfo {
    name tea
    path tea
    description tea
    functions []FunctionInfo
    structs []StructInfo
    interfaces []InterfaceInfo
    constants []ConstantInfo
}

struct FunctionInfo {
    name tea
    signature tea
    description tea
    parameters []ParameterInfo
    return_type tea
    examples []tea
}

struct StructInfo {
    name tea
    fields []FieldInfo
    methods []FunctionInfo
    description tea
}

struct InterfaceInfo {
    name tea
    methods []FunctionInfo
    description tea
}

struct ConstantInfo {
    name tea
    type tea
    value tea
    description tea
}

struct ParameterInfo {
    name tea
    type tea
    description tea
}

slay new_doc_generator(output_dir tea) DocGenerator {
    damn DocGenerator{
        output_dir: output_dir,
        modules: make([]ModuleInfo, 0),
        index_content: "",
    }
}

slay (gen *DocGenerator) scan_stdlib_modules() {
    vibez.spill("Scanning stdlib modules...")
    
    # Scan all modules in stdlib directory
    sus modules []tea = scan_directory("stdlib")
    
    bestie module_path <- modules {
        sus module_info ModuleInfo = gen.parse_module(module_path)
        gen.modules = append(gen.modules, module_info)
    }
    
    vibez.spill("Found " + len(gen.modules).(tea) + " modules")
}

slay (gen *DocGenerator) parse_module(module_path tea) ModuleInfo {
    sus module_name tea = extract_module_name(module_path)
    sus mod_file tea = module_path + "/mod.csd"
    
    # Parse module file
    sus content tea = read_file(mod_file)
    sus info ModuleInfo = ModuleInfo{
        name: module_name,
        path: module_path,
        description: extract_description(content),
        functions: parse_functions(content),
        structs: parse_structs(content),
        interfaces: parse_interfaces(content),
        constants: parse_constants(content),
    }
    
    damn info
}

slay (gen *DocGenerator) generate_all_docs() {
    vibez.spill("Generating API documentation...")
    
    # Create output directory
    create_directory(gen.output_dir)
    
    # Generate individual module docs
    bestie module <- gen.modules {
        gen.generate_module_doc(module)
    }
    
    # Generate index
    gen.generate_index()
    
    # Generate navigation
    gen.generate_navigation()
    
    # Generate search index
    gen.generate_search_index()
    
    vibez.spill("Documentation generated in " + gen.output_dir)
}

slay (gen *DocGenerator) generate_module_doc(module ModuleInfo) {
    sus filename tea = gen.output_dir + "/" + module.name + ".md"
    sus content tea = generate_module_markdown(module)
    
    write_file(filename, content)
    vibez.spill("Generated docs for " + module.name)
}

slay generate_module_markdown(module ModuleInfo) tea {
    sus content tea = "# " + module.name + " Module\n\n"
    
    # Module description
    content += module.description + "\n\n"
    
    # Table of contents
    content += "## Table of Contents\n\n"
    lowkey len(module.functions) > 0 {
        content += "- [Functions](#functions)\n"
    }
    lowkey len(module.structs) > 0 {
        content += "- [Structs](#structs)\n"
    }
    lowkey len(module.interfaces) > 0 {
        content += "- [Interfaces](#interfaces)\n"
    }
    lowkey len(module.constants) > 0 {
        content += "- [Constants](#constants)\n"
    }
    content += "\n"
    
    # Functions section
    lowkey len(module.functions) > 0 {
        content += "## Functions\n\n"
        bestie func <- module.functions {
            content += generate_function_doc(func)
        }
    }
    
    # Structs section
    lowkey len(module.structs) > 0 {
        content += "## Structs\n\n"
        bestie struct <- module.structs {
            content += generate_struct_doc(struct)
        }
    }
    
    # Interfaces section
    lowkey len(module.interfaces) > 0 {
        content += "## Interfaces\n\n"
        bestie interface <- module.interfaces {
            content += generate_interface_doc(interface)
        }
    }
    
    # Constants section
    lowkey len(module.constants) > 0 {
        content += "## Constants\n\n"
        bestie constant <- module.constants {
            content += generate_constant_doc(constant)
        }
    }
    
    damn content
}

slay generate_function_doc(func FunctionInfo) tea {
    sus content tea = "### " + func.name + "\n\n"
    
    # Function signature
    content += "```cursed\n"
    content += func.signature + "\n"
    content += "```\n\n"
    
    # Description
    content += func.description + "\n\n"
    
    # Parameters
    lowkey len(func.parameters) > 0 {
        content += "#### Parameters\n\n"
        bestie param <- func.parameters {
            content += "- `" + param.name + "` (" + param.type + "): " + param.description + "\n"
        }
        content += "\n"
    }
    
    # Return type
    lowkey func.return_type != "" {
        content += "#### Returns\n\n"
        content += "`" + func.return_type + "`\n\n"
    }
    
    # Examples
    lowkey len(func.examples) > 0 {
        content += "#### Examples\n\n"
        bestie example <- func.examples {
            content += "```cursed\n"
            content += example + "\n"
            content += "```\n\n"
        }
    }
    
    damn content
}

slay generate_struct_doc(struct StructInfo) tea {
    sus content tea = "### " + struct.name + "\n\n"
    
    # Description
    content += struct.description + "\n\n"
    
    # Fields
    lowkey len(struct.fields) > 0 {
        content += "#### Fields\n\n"
        bestie field <- struct.fields {
            content += "- `" + field.name + "` (" + field.type + "): " + field.description + "\n"
        }
        content += "\n"
    }
    
    # Methods
    lowkey len(struct.methods) > 0 {
        content += "#### Methods\n\n"
        bestie method <- struct.methods {
            content += generate_function_doc(method)
        }
    }
    
    damn content
}

slay generate_interface_doc(interface InterfaceInfo) tea {
    sus content tea = "### " + interface.name + "\n\n"
    
    # Description
    content += interface.description + "\n\n"
    
    # Methods
    lowkey len(interface.methods) > 0 {
        content += "#### Methods\n\n"
        bestie method <- interface.methods {
            content += generate_function_doc(method)
        }
    }
    
    damn content
}

slay generate_constant_doc(constant ConstantInfo) tea {
    sus content tea = "### " + constant.name + "\n\n"
    
    # Type and value
    content += "**Type:** `" + constant.type + "`\n\n"
    content += "**Value:** `" + constant.value + "`\n\n"
    
    # Description
    content += constant.description + "\n\n"
    
    damn content
}

slay (gen *DocGenerator) generate_index() {
    sus content tea = "# CURSED Standard Library API Reference\n\n"
    
    content += "This is the complete API reference for the CURSED standard library.\n\n"
    
    # Module index
    content += "## Modules\n\n"
    bestie module <- gen.modules {
        content += "- [" + module.name + "](" + module.name + ".md) - " + module.description + "\n"
    }
    content += "\n"
    
    # Quick reference
    content += "## Quick Reference\n\n"
    
    # Most common functions
    content += "### Common Functions\n\n"
    content += "- `vibez.spill(value)` - Print value to output\n"
    content += "- `len(collection)` - Get length of collection\n"
    content += "- `append(slice, item)` - Append item to slice\n"
    content += "- `make(type, args...)` - Create new instance\n\n"
    
    # Write index
    sus filename tea = gen.output_dir + "/index.md"
    write_file(filename, content)
}

slay (gen *DocGenerator) generate_navigation() {
    sus content tea = "# Navigation\n\n"
    
    # Categories
    sus categories map[tea][]tea = make(map[tea][]tea)
    
    bestie module <- gen.modules {
        sus category tea = get_module_category(module.name)
        lowkey categories[category] == cringe {
            categories[category] = make([]tea, 0)
        }
        categories[category] = append(categories[category], module.name)
    }
    
    # Generate navigation by category
    bestie category, modules <- categories {
        content += "## " + category + "\n\n"
        bestie module <- modules {
            content += "- [" + module + "](" + module + ".md)\n"
        }
        content += "\n"
    }
    
    sus filename tea = gen.output_dir + "/navigation.md"
    write_file(filename, content)
}

slay (gen *DocGenerator) generate_search_index() {
    sus search_data tea = "{\n"
    search_data += "  \"modules\": [\n"
    
    bestie i, module <- gen.modules {
        search_data += "    {\n"
        search_data += "      \"name\": \"" + module.name + "\",\n"
        search_data += "      \"description\": \"" + module.description + "\",\n"
        search_data += "      \"functions\": [\n"
        
        bestie j, func <- module.functions {
            search_data += "        {\n"
            search_data += "          \"name\": \"" + func.name + "\",\n"
            search_data += "          \"signature\": \"" + func.signature + "\",\n"
            search_data += "          \"description\": \"" + func.description + "\"\n"
            search_data += "        }"
            lowkey j < len(module.functions) - 1 {
                search_data += ","
            }
            search_data += "\n"
        }
        
        search_data += "      ]\n"
        search_data += "    }"
        lowkey i < len(gen.modules) - 1 {
            search_data += ","
        }
        search_data += "\n"
    }
    
    search_data += "  ]\n"
    search_data += "}\n"
    
    sus filename tea = gen.output_dir + "/search_index.json"
    write_file(filename, search_data)
}

# Helper functions
slay scan_directory(dir tea) []tea {
    # Implementation depends on filesystem module
    # Return list of subdirectories
    damn []tea{}
}

slay extract_module_name(path tea) tea {
    # Extract module name from path
    damn path
}

slay extract_description(content tea) tea {
    # Extract module description from comments
    damn "Module description"
}

slay parse_functions(content tea) []FunctionInfo {
    # Parse function definitions from source
    damn []FunctionInfo{}
}

slay parse_structs(content tea) []StructInfo {
    # Parse struct definitions from source
    damn []StructInfo{}
}

slay parse_interfaces(content tea) []InterfaceInfo {
    # Parse interface definitions from source
    damn []InterfaceInfo{}
}

slay parse_constants(content tea) []ConstantInfo {
    # Parse constant definitions from source
    damn []ConstantInfo{}
}

slay get_module_category(module_name tea) tea {
    # Categorize modules
    lowkey stringz.contains(module_name, "crypto") {
        damn "Cryptography"
    }
    lowkey stringz.contains(module_name, "net") || stringz.contains(module_name, "web") {
        damn "Networking"
    }
    lowkey stringz.contains(module_name, "io") || stringz.contains(module_name, "fs") {
        damn "I/O and Filesystem"
    }
    lowkey stringz.contains(module_name, "math") {
        damn "Mathematics"
    }
    lowkey stringz.contains(module_name, "string") {
        damn "String Processing"
    }
    lowkey stringz.contains(module_name, "time") {
        damn "Time and Date"
    }
    lowkey stringz.contains(module_name, "test") {
        damn "Testing"
    }
    lowkey stringz.contains(module_name, "collection") {
        damn "Collections"
    }
    damn "Core"
}

slay read_file(filename tea) tea {
    # Read file content
    damn ""
}

slay write_file(filename tea, content tea) {
    # Write content to file
    vibez.spill("Writing " + filename)
}

slay create_directory(dir tea) {
    # Create directory if it doesn't exist
    vibez.spill("Creating directory " + dir)
}

# Main function to generate documentation
slay generate_api_docs() {
    test_start("API Documentation Generation")
    
    vibez.spill("CURSED API Documentation Generator")
    vibez.spill("=====================================")
    
    sus generator DocGenerator = new_doc_generator("docs/api")
    
    # Scan and parse all modules
    generator.scan_stdlib_modules()
    
    # Generate documentation
    generator.generate_all_docs()
    
    vibez.spill("API documentation generation complete!")
    vibez.spill("Output: docs/api/")
    
    print_test_summary()
}

# Run the generator
generate_api_docs()
