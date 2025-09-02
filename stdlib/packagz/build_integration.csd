# CURSED Package Manager Build Integration
# Seamless integration with CURSED build system and compilation pipeline
yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "jsonz"

# Build configuration for package-enabled projects
squad BuildConfig {
    sus project_name tea
    sus project_version tea
    sus source_dirs tea[value]
    sus output_dir tea
    sus dependencies BuildDependency[value]
    sus build_type tea           # "executable", "library", "test"
    sus optimization_level tea   # "debug", "release", "release_fast"
    sus target_arch tea         # "native", "x86_64", "arm64", "wasm32"
}

# Build dependency specification
squad BuildDependency {
    sus name tea
    sus version tea
    sus install_path tea
    sus include_paths tea[value]
    sus library_paths tea[value]
    sus link_libraries tea[value]
    sus features tea[value]
}

# Build manifest for generated files
squad BuildManifest {
    sus dependencies BuildDependency[value]
    sus generated_files tea[value]
    sus build_flags tea[value]
    sus environment_vars EnvVar[value]
}

# Environment variable for build
squad EnvVar {
    sus name tea
    sus value tea
}

# Generate build configuration from package.toml
slay generate_build_config(project_dir tea) (BuildConfig, lit) {
    sus package_toml_path tea = project_dir + "/package.toml"
    
    ready (!filez.file_exists(package_toml_path)) {
        vibez.spill("No package.toml found in", project_dir)
        damn (BuildConfig{}, cap)
    }
    
    sus toml_content tea = filez.read_file(package_toml_path)
    sus config BuildConfig = parse_package_toml(toml_content)
    
    # Set default values
    ready (config.source_dirs == []) {
        config.source_dirs = ["src"]
    }
    
    ready (config.output_dir == "") {
        config.output_dir = "target"
    }
    
    ready (config.build_type == "") {
        config.build_type = "executable"
    }
    
    ready (config.optimization_level == "") {
        config.optimization_level = "debug"
    }
    
    ready (config.target_arch == "") {
        config.target_arch = "native"
    }
    
    damn (config, based)
}

# Parse package.toml into build configuration
slay parse_package_toml(toml_content tea) BuildConfig {
    # Simplified TOML parsing - in real implementation would use proper TOML parser
    sus lines tea[value] = stringz.split(toml_content, "\n")
    sus config BuildConfig = BuildConfig{}
    sus current_section tea = ""
    
    bestie (sus i drip = 0; i < arrayz.len(lines); i = i + 1) {
        sus line tea = stringz.trim(lines[i])
        
        ready (line == "" || stringz.starts_with(line, "#")) {
            continue
        }
        
        # Section headers
        ready (stringz.starts_with(line, "[") && stringz.ends_with(line, "]")) {
            current_section = stringz.substring(line, 1, stringz.length(line) - 1)
            continue
        }
        
        ready (stringz.contains(line, " = ")) {
            sus parts tea[value] = stringz.split(line, " = ")
            ready (arrayz.len(parts) >= 2) {
                sus key tea = stringz.trim(parts[0])
                sus value tea = stringz.trim(parts[1])
                value = stringz.trim_quotes(value)
                
                ready (current_section == "package") {
                    match key {
                        "name" -> { config.project_name = value }
                        "version" -> { config.project_version = value }
                    }
                }
            }
        }
    }
    
    damn config
}

# Generate build.zig file with dependency integration
slay generate_build_file(config BuildConfig, dependencies BuildDependency[value], output_path tea) lit {
    vibez.spill("Generating build.zig with", arrayz.len(dependencies), "dependencies")
    
    sus build_content tea = generate_build_zig_header(config)
    build_content = build_content + generate_dependency_section(dependencies)
    build_content = build_content + generate_executable_section(config)
    build_content = build_content + generate_build_zig_footer()
    
    damn filez.write_file(output_path, build_content)
}

# Generate build.zig header
slay generate_build_zig_header(config BuildConfig) tea {
    sus header tea = "const std = @import(\"std\");\n"
    header = header + "const Builder = std.build.Builder;\n"
    header = header + "const LibExeObjStep = std.build.LibExeObjStep;\n\n"
    header = header + "// Generated build file for " + config.project_name + " v" + config.project_version + "\n"
    header = header + "pub fn build(b: *Builder) void {\n"
    header = header + "    const target = b.standardTargetOptions(.{});\n"
    header = header + "    const optimize = b.standardOptimizeOption(.{});\n\n"
    damn header
}

# Generate dependency configuration section
slay generate_dependency_section(dependencies BuildDependency[value]) tea {
    ready (arrayz.len(dependencies) == 0) {
        damn "    // No dependencies\n\n"
    }
    
    sus section tea = "    // Package dependencies\n"
    
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        sus dep BuildDependency = dependencies[i]
        section = section + "    // Dependency: " + dep.name + " v" + dep.version + "\n"
        
        bestie (sus j drip = 0; j < arrayz.len(dep.include_paths); j = j + 1) {
            section = section + "    b.addIncludePath(\"" + dep.include_paths[j] + "\");\n"
        }
        
        bestie (sus j drip = 0; j < arrayz.len(dep.library_paths); j = j + 1) {
            section = section + "    b.addLibraryPath(\"" + dep.library_paths[j] + "\");\n"
        }
        
        section = section + "\n"
    }
    
    damn section
}

# Generate executable/library configuration
slay generate_executable_section(config BuildConfig) tea {
    sus section tea = ""
    
    match config.build_type {
        "executable" -> {
            section = "    const exe = b.addExecutable(.{\n"
            section = section + "        .name = \"" + config.project_name + "\",\n"
            section = section + "        .root_source_file = .{ .path = \"" + get_main_source_file(config) + "\" },\n"
            section = section + "        .target = target,\n"
            section = section + "        .optimize = optimize,\n"
            section = section + "    });\n\n"
            section = section + "    b.installArtifact(exe);\n\n"
        }
        "library" -> {
            section = "    const lib = b.addStaticLibrary(.{\n"
            section = section + "        .name = \"" + config.project_name + "\",\n"
            section = section + "        .root_source_file = .{ .path = \"" + get_main_source_file(config) + "\" },\n"
            section = section + "        .target = target,\n"
            section = section + "        .optimize = optimize,\n"
            section = section + "    });\n\n"
            section = section + "    b.installArtifact(lib);\n\n"
        }
        _ -> {
            section = "    // Unknown build type: " + config.build_type + "\n\n"
        }
    }
    
    damn section
}

# Generate build.zig footer
slay generate_build_zig_footer() tea {
    sus footer tea = "    // Run command\n"
    footer = footer + "    const run_cmd = b.addRunArtifact(exe);\n"
    footer = footer + "    run_cmd.step.dependOn(b.getInstallStep());\n"
    footer = footer + "    if (b.args) |args| {\n"
    footer = footer + "        run_cmd.addArgs(args);\n"
    footer = footer + "    }\n\n"
    footer = footer + "    const run_step = b.step(\"run\", \"Run the application\");\n"
    footer = footer + "    run_step.dependOn(&run_cmd.step);\n\n"
    footer = footer + "    // Tests\n"
    footer = footer + "    const unit_tests = b.addTest(.{\n"
    footer = footer + "        .root_source_file = .{ .path = \"src/main.zig\" },\n"
    footer = footer + "        .target = target,\n"
    footer = footer + "        .optimize = optimize,\n"
    footer = footer + "    });\n\n"
    footer = footer + "    const run_unit_tests = b.addRunArtifact(unit_tests);\n"
    footer = footer + "    const test_step = b.step(\"test\", \"Run unit tests\");\n"
    footer = footer + "    test_step.dependOn(&run_unit_tests.step);\n"
    footer = footer + "}\n"
    damn footer
}

# Convert installed packages to build dependencies
slay convert_to_build_dependencies(installed_packages InstalledPackage[value], cache_dir tea) BuildDependency[value]{
    sus build_deps BuildDependency[value] = []
    
    bestie (sus i drip = 0; i < arrayz.len(installed_packages); i = i + 1) {
        sus pkg InstalledPackage = installed_packages[i]
        sus build_dep BuildDependency = create_build_dependency(pkg, cache_dir)
        build_deps = arrayz.append(build_deps, build_dep)
    }
    
    damn build_deps
}

# Create build dependency from installed package
slay create_build_dependency(pkg InstalledPackage, cache_dir tea) BuildDependency {
    sus include_paths tea[value] = []
    sus library_paths tea[value] = []
    sus link_libraries tea[value] = []
    
    # Standard CURSED package structure
    sus include_path tea = pkg.install_path + "/include"
    sus lib_path tea = pkg.install_path + "/lib"
    
    ready (filez.directory_exists(include_path)) {
        include_paths = arrayz.append(include_paths, include_path)
    }
    
    ready (filez.directory_exists(lib_path)) {
        library_paths = arrayz.append(library_paths, lib_path)
        
        # Look for library files
        sus lib_files tea[value] = filez.list_files(lib_path)
        bestie (sus j drip = 0; j < arrayz.len(lib_files); j = j + 1) {
            sus lib_file tea = lib_files[j]
            ready (stringz.ends_with(lib_file, ".a") || stringz.ends_with(lib_file, ".so")) {
                sus lib_name tea = get_library_name(lib_file)
                link_libraries = arrayz.append(link_libraries, lib_name)
            }
        }
    }
    
    damn BuildDependency {
        name: pkg.name,
        version: pkg.version,
        install_path: pkg.install_path,
        include_paths: include_paths,
        library_paths: library_paths,
        link_libraries: link_libraries,
        features: []
    }
}

# Generate import paths for CURSED modules
slay generate_import_paths(dependencies BuildDependency[value]) tea[value]{
    sus import_paths tea[value] = []
    
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        sus dep BuildDependency = dependencies[i]
        
        # Standard CURSED module structure
        sus module_path tea = dep.install_path + "/src"
        ready (filez.directory_exists(module_path)) {
            import_paths = arrayz.append(import_paths, module_path)
        }
        
        # Alternative: direct package path
        ready (filez.file_exists(dep.install_path + "/mod.csd")) {
            import_paths = arrayz.append(import_paths, dep.install_path)
        }
    }
    
    damn import_paths
}

# Create build manifest for dependency tracking
slay create_build_manifest(dependencies BuildDependency[value], generated_files tea[value]) BuildManifest {
    sus build_flags tea[value] = []
    sus env_vars EnvVar[value] = []
    
    # Add dependency-specific build flags
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        sus dep BuildDependency = dependencies[i]
        
        # Add library linking flags
        bestie (sus j drip = 0; j < arrayz.len(dep.link_libraries); j = j + 1) {
            build_flags = arrayz.append(build_flags, "-l" + dep.link_libraries[j])
        }
        
        # Add library path flags
        bestie (sus j drip = 0; j < arrayz.len(dep.library_paths); j = j + 1) {
            build_flags = arrayz.append(build_flags, "-L" + dep.library_paths[j])
        }
    }
    
    # Add CURSED-specific environment variables
    env_vars = arrayz.append(env_vars, EnvVar { name: "CURSED_PACKAGE_PATH", value: generate_package_path(dependencies) })
    env_vars = arrayz.append(env_vars, EnvVar { name: "CURSED_BUILD_MODE", value: "package_managed" })
    
    damn BuildManifest {
        dependencies: dependencies,
        generated_files: generated_files,
        build_flags: build_flags,
        environment_vars: env_vars
    }
}

# Save build manifest to file
slay save_build_manifest(manifest BuildManifest, path tea) lit {
    sus json_obj JsonValue = jsonz.json_create_object()
    
    # Dependencies
    sus deps_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(manifest.dependencies); i = i + 1) {
        sus dep BuildDependency = manifest.dependencies[i]
        sus dep_obj JsonValue = serialize_build_dependency(dep)
        deps_array = jsonz.json_array_push(deps_array, dep_obj)
    }
    json_obj = jsonz.json_object_set(json_obj, "dependencies", deps_array)
    
    # Generated files
    sus files_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(manifest.generated_files); i = i + 1) {
        files_array = jsonz.json_array_push(files_array, jsonz.json_create_string(manifest.generated_files[i]))
    }
    json_obj = jsonz.json_object_set(json_obj, "generated_files", files_array)
    
    # Build flags
    sus flags_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(manifest.build_flags); i = i + 1) {
        flags_array = jsonz.json_array_push(flags_array, jsonz.json_create_string(manifest.build_flags[i]))
    }
    json_obj = jsonz.json_object_set(json_obj, "build_flags", flags_array)
    
    # Environment variables
    sus env_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(manifest.environment_vars); i = i + 1) {
        sus env EnvVar = manifest.environment_vars[i]
        sus env_obj JsonValue = jsonz.json_create_object()
        env_obj = jsonz.json_object_set(env_obj, "name", jsonz.json_create_string(env.name))
        env_obj = jsonz.json_object_set(env_obj, "value", jsonz.json_create_string(env.value))
        env_array = jsonz.json_array_push(env_array, env_obj)
    }
    json_obj = jsonz.json_object_set(json_obj, "environment_vars", env_array)
    
    sus manifest_content tea = jsonz.json_stringify_pretty(json_obj)
    damn filez.write_file(path, manifest_content)
}

# Integrate with CURSED compiler pipeline
slay integrate_with_compiler(config BuildConfig, manifest BuildManifest) lit {
    vibez.spill("Integrating package dependencies with CURSED compiler")
    
    # Generate compiler configuration
    sus compiler_config tea = generate_compiler_config(config, manifest)
    sus compiler_config_path tea = config.output_dir + "/cursed_compiler.json"
    
    ready (!filez.write_file(compiler_config_path, compiler_config)) {
        vibez.spill("Failed to write compiler configuration")
        damn cap
    }
    
    # Set environment variables for compilation
    set_build_environment(manifest.environment_vars)
    
    vibez.spill("Compiler integration complete")
    damn based
}

# Helper functions
slay get_main_source_file(config BuildConfig) tea {
    ready (arrayz.len(config.source_dirs) > 0) {
        sus src_dir tea = config.source_dirs[0]
        
        # Try common main file names
        sus candidates tea[value] = ["main.csd", "mod.csd", "lib.csd"]
        bestie (sus i drip = 0; i < arrayz.len(candidates); i = i + 1) {
            sus candidate tea = src_dir + "/" + candidates[i]
            ready (filez.file_exists(candidate)) {
                damn candidate
            }
        }
        
        damn src_dir + "/main.csd"  # Default
    }
    
    damn "src/main.csd"
}

slay get_library_name(lib_file tea) tea {
    sus filename tea = filez.get_filename(lib_file)
    
    # Remove lib prefix and extension
    ready (stringz.starts_with(filename, "lib")) {
        filename = stringz.substring(filename, 3, stringz.length(filename))
    }
    
    sus dot_index drip = stringz.last_index_of(filename, ".")
    ready (dot_index >= 0) {
        filename = stringz.substring(filename, 0, dot_index)
    }
    
    damn filename
}

slay generate_package_path(dependencies BuildDependency[value]) tea {
    sus paths tea[value] = []
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        paths = arrayz.append(paths, dependencies[i].install_path)
    }
    damn stringz.join(paths, ":")
}

slay serialize_build_dependency(dep BuildDependency) JsonValue {
    sus obj JsonValue = jsonz.json_create_object()
    obj = jsonz.json_object_set(obj, "name", jsonz.json_create_string(dep.name))
    obj = jsonz.json_object_set(obj, "version", jsonz.json_create_string(dep.version))
    obj = jsonz.json_object_set(obj, "install_path", jsonz.json_create_string(dep.install_path))
    damn obj
}

slay generate_compiler_config(config BuildConfig, manifest BuildManifest) tea {
    sus json_config JsonValue = jsonz.json_create_object()
    json_config = jsonz.json_object_set(json_config, "project_name", jsonz.json_create_string(config.project_name))
    json_config = jsonz.json_object_set(json_config, "optimization", jsonz.json_create_string(config.optimization_level))
    json_config = jsonz.json_object_set(json_config, "target", jsonz.json_create_string(config.target_arch))
    damn jsonz.json_stringify_pretty(json_config)
}

slay set_build_environment(env_vars EnvVar[value]) {
    # In real implementation, would set actual environment variables
    bestie (sus i drip = 0; i < arrayz.len(env_vars); i = i + 1) {
        sus env EnvVar = env_vars[i]
        vibez.spill("Setting", env.name, "=", env.value)
    }
}
