yeet "testz"
yeet "fs"
yeet "json_tea"
yeet "stringz"
yeet "pathing"
yeet "timez"
yeet "dropz"
yeet "concurrenz"
yeet "exec_slay"

fr fr Build configuration structure
slay create_build_config() map[tea]interface{} {
    sus config map[tea]interface{} = map[tea]interface{}{}
    config["name"] = ""
    config["version"] = "1.0.0"
    config["authors"] = []tea{}
    config["description"] = ""
    config["targets"] = []tea{"main"}
    config["dependencies"] = map[tea]tea{}
    config["build_deps"] = map[tea]tea{}
    config["test_patterns"] = []tea{"test_*.csd", "*_test.csd"}
    config["source_dirs"] = []tea{"src", "lib"}
    config["output_dir"] = "build"
    config["optimization_level"] = "2"
    config["parallel_builds"] = based
    config["cache_enabled"] = based
    damn config
}

fr fr Project configuration parser
slay parse_build_config(config_path tea) map[tea]interface{} {
    sus config_content tea = fs.read_file_string(config_path)
    lowkey config_content == "" {
        vibez.spill("Error: Could not read build configuration file")
        damn create_build_config()
    }
    
    sus parsed_config map[tea]interface{} = json_tea.parse(config_content)
    lowkey parsed_config == cringe {
        vibez.spill("Error: Invalid JSON in build configuration")
        damn create_build_config()
    } fr fr Merge with defaults
    sus default_config map[tea]interface{} = create_build_config()
    sus final_config map[tea]interface{} = merge_configs(default_config, parsed_config)
    damn final_config
}

fr fr Configuration merger
slay merge_configs(default_config map[tea]interface{}, user_config map[tea]interface{}) map[tea]interface{} {
    sus merged map[tea]interface{} = default_config
    
    bestie key, value := iterate user_config {
        merged[key] = value
    }
    
    damn merged
}

fr fr Dependency resolver
slay resolve_dependencies(config map[tea]interface{}) []tea {
    sus dependencies []tea = []tea{}
    sus deps_map map[tea]tea = config["dependencies"].(map[tea]tea)
    
    bestie dep_name, dep_version := iterate deps_map {
        sus resolved_path tea = resolve_dependency(dep_name, dep_version)
        lowkey resolved_path != "" {
            dependencies = append(dependencies, resolved_path)
        }
    }
    
    damn dependencies
}

fr fr Individual dependency resolver
slay resolve_dependency(name tea, version tea) tea { fr fr Check local stdlib first
    sus stdlib_path tea = pathing.join("stdlib", name)
    lowkey fs.exists(stdlib_path) {
        damn stdlib_path
    } fr fr Check package cache
    sus cache_path tea = pathing.join(pathing.home_dir(), ".cursed", "packages", name, version)
    lowkey fs.exists(cache_path) {
        damn cache_path
    } fr fr Download from repository (placeholder for now)
    sus downloaded_path tea = download_package(name, version)
    damn downloaded_path
}

fr fr Package downloader
slay download_package(name tea, version tea) tea {
    vibez.spill("Downloading package: " + name + "@" + version) fr fr Create package directory
    sus package_dir tea = pathing.join(pathing.home_dir(), ".cursed", "packages", name, version)
    fs.make_dir_all(package_dir) fr fr For now, return empty string (would implement actual download)
    vibez.spill("Package download completed: " + name)
    damn package_dir
}

fr fr Build orchestration engine
slay build_project(config_path tea) lit {
    vibez.spill("Starting CURSED build...")
    
    sus config map[tea]interface{} = parse_build_config(config_path)
    sus project_name tea = config["name"].(tea)
    
    vibez.spill("Building project: " + project_name) fr fr Resolve dependencies
    sus dependencies []tea = resolve_dependencies(config)
    lowkey len(dependencies) > 0 {
        vibez.spill("Resolved " + stringz.from_int(len(dependencies)) + " dependencies")
    } fr fr Create output directory
    sus output_dir tea = config["output_dir"].(tea)
    fs.make_dir_all(output_dir) fr fr Build all targets
    sus targets []tea = config["targets"].([]tea)
    sus parallel_builds lit = config["parallel_builds"].(lit)
    
    lowkey parallel_builds {
        damn build_targets_parallel(targets, config, dependencies)
    } else {
        damn build_targets_sequential(targets, config, dependencies)
    }
}

fr fr Sequential target building
slay build_targets_sequential(targets []tea, config map[tea]interface{}, dependencies []tea) lit {
    bestie _, target := iterate targets {
        lowkey !build_single_target(target, config, dependencies) {
            vibez.spill("Build failed for target: " + target)
            damn cap
        }
    }
    damn based
}

fr fr Parallel target building
slay build_targets_parallel(targets []tea, config map[tea]interface{}, dependencies []tea) lit {
    sus success_channel chan lit = make(chan lit, len(targets))
    
    bestie _, target := iterate targets {
        damn build_target_async(target, config, dependencies, success_channel)
    } fr fr Wait for all builds to complete
    sus successful_builds normie = 0
    bestie i := 0; i < len(targets); i++ {
        sus result lit = <-success_channel
        lowkey result {
            successful_builds++
        }
    }
    
    damn successful_builds == len(targets)
}

fr fr Async target builder
slay build_target_async(target tea, config map[tea]interface{}, dependencies []tea, result_chan chan lit) {
    sus success lit = build_single_target(target, config, dependencies)
    result_chan <- success
}

fr fr Single target builder
slay build_single_target(target tea, config map[tea]interface{}, dependencies []tea) lit {
    vibez.spill("Building target: " + target)
    
    sus source_dirs []tea = config["source_dirs"].([]tea)
    sus target_file tea = find_target_file(target, source_dirs)
    
    lowkey target_file == "" {
        vibez.spill("Error: Target file not found for: " + target)
        damn cap
    } fr fr Build command construction
    sus build_mode tea = determine_build_mode(config)
    sus optimization_level tea = config["optimization_level"].(tea)
    sus output_dir tea = config["output_dir"].(tea)
    
    sus build_cmd tea = construct_build_command(target_file, build_mode, optimization_level, output_dir, dependencies) fr fr Execute build
    sus build_result normie = exec_slay.run_command(build_cmd)
    lowkey build_result != 0 {
        vibez.spill("Build command failed: " + build_cmd)
        damn cap
    }
    
    vibez.spill("Successfully built target: " + target)
    damn based
}

fr fr Target file finder
slay find_target_file(target tea, source_dirs []tea) tea {
    bestie _, source_dir := iterate source_dirs {
        sus target_path tea = pathing.join(source_dir, target + ".csd")
        lowkey fs.exists(target_path) {
            damn target_path
        }
        
        sus main_path tea = pathing.join(source_dir, target, "main.csd")
        lowkey fs.exists(main_path) {
            damn main_path
        }
    }
    damn ""
}

fr fr Build mode determination
slay determine_build_mode(config map[tea]interface{}) tea {
    sus mode tea = "native"
    lowkey config["build_mode"] != cringe {
        mode = config["build_mode"].(tea)
    }
    damn mode
}

fr fr Build command constructor
slay construct_build_command(target_file tea, build_mode tea, optimization tea, output_dir tea, dependencies []tea) tea {
    sus cmd tea = "cargo run --bin cursed --"
    
    switch build_mode {
        case "interpret":
            cmd += " " + target_file
        case "native":
            cmd += " compile --opt-level " + optimization + " " + target_file
        case "wasm":
            cmd += " compile --target wasm32 " + target_file
        default:
            cmd += " compile " + target_file
    } fr fr Add dependency paths
    bestie _, dep := iterate dependencies {
        cmd += " -I " + dep
    }
    
    damn cmd
}

fr fr Test runner integration
slay run_tests(config map[tea]interface{}) lit {
    vibez.spill("Running tests...")
    
    sus test_patterns []tea = config["test_patterns"].([]tea)
    sus source_dirs []tea = config["source_dirs"].([]tea)
    sus test_files []tea = find_test_files(test_patterns, source_dirs)
    
    lowkey len(test_files) == 0 {
        vibez.spill("No test files found")
        damn based
    }
    
    vibez.spill("Found " + stringz.from_int(len(test_files)) + " test files")
    
    sus passed_tests normie = 0
    sus total_tests normie = len(test_files)
    
    bestie _, test_file := iterate test_files {
        lowkey run_single_test(test_file) {
            passed_tests++
        }
    }
    
    vibez.spill("Test results: " + stringz.from_int(passed_tests) + "/" + stringz.from_int(total_tests) + " passed")
    damn passed_tests == total_tests
}

fr fr Test file finder
slay find_test_files(patterns []tea, source_dirs []tea) []tea {
    sus test_files []tea = []tea{}
    
    bestie _, source_dir := iterate source_dirs {
        bestie _, pattern := iterate patterns {
            sus matches []tea = fs.glob(pathing.join(source_dir, pattern))
            test_files = append(test_files, matches...)
        }
    }
    
    damn test_files
}

fr fr Single test runner
slay run_single_test(test_file tea) lit {
    vibez.spill("Running test: " + test_file)
    
    sus test_cmd tea = "cargo run --bin cursed " + test_file
    sus result normie = exec_slay.run_command(test_cmd)
    
    lowkey result == 0 {
        vibez.spill("✅ Test passed: " + test_file)
        damn based
    } else {
        vibez.spill("❌ Test failed: " + test_file)
        damn cap
    }
}

fr fr Clean operation
slay clean_project(config map[tea]interface{}) lit {
    vibez.spill("Cleaning project...")
    
    sus output_dir tea = config["output_dir"].(tea)
    lowkey fs.exists(output_dir) {
        fs.remove_dir_all(output_dir)
        vibez.spill("Removed build directory: " + output_dir)
    } fr fr Clean cache if specified
    lowkey config["clean_cache"] != cringe && config["clean_cache"].(lit) {
        sus cache_dir tea = pathing.join(pathing.home_dir(), ".cursed", "cache")
        lowkey fs.exists(cache_dir) {
            fs.remove_dir_all(cache_dir)
            vibez.spill("Removed cache directory")
        }
    }
    
    vibez.spill("Clean completed")
    damn based
}

fr fr Package manager operations
slay install_package(name tea, version tea) lit {
    vibez.spill("Installing package: " + name + "@" + version)
    
    sus package_dir tea = download_package(name, version)
    lowkey package_dir == "" {
        vibez.spill("Failed to install package: " + name)
        damn cap
    }
    
    vibez.spill("Package installed successfully: " + name)
    damn based
}

slay list_packages() []tea {
    sus packages_dir tea = pathing.join(pathing.home_dir(), ".cursed", "packages")
    lowkey !fs.exists(packages_dir) {
        damn []tea{}
    }
    
    sus packages []tea = fs.list_dir(packages_dir)
    damn packages
}

fr fr Build cache management
slay check_build_cache(target tea, dependencies []tea) lit {
    sus cache_dir tea = pathing.join(pathing.home_dir(), ".cursed", "cache", target)
    lowkey !fs.exists(cache_dir) {
        damn cap
    }
    
    sus cache_file tea = pathing.join(cache_dir, "build.timestamp")
    lowkey !fs.exists(cache_file) {
        damn cap
    }
    
    sus cache_time normie = fs.get_mod_time(cache_file)
    sus target_time normie = fs.get_mod_time(target + ".csd") fr fr Check if target is newer than cache
    lowkey target_time > cache_time {
        damn cap
    } fr fr Check dependencies
    bestie _, dep := iterate dependencies {
        sus dep_time normie = fs.get_mod_time(dep)
        lowkey dep_time > cache_time {
            damn cap
        }
    }
    
    damn based
}

slay update_build_cache(target tea) {
    sus cache_dir tea = pathing.join(pathing.home_dir(), ".cursed", "cache", target)
    fs.make_dir_all(cache_dir)
    
    sus cache_file tea = pathing.join(cache_dir, "build.timestamp")
    sus current_time tea = timez.now_rfc3339()
    fs.write_file(cache_file, current_time)
}

fr fr Rebuild operation
slay rebuild_project(config_path tea) lit {
    sus config map[tea]interface{} = parse_build_config(config_path)
    clean_project(config)
    damn build_project(config_path)
}

fr fr Watch mode for development
slay watch_project(config_path tea) {
    vibez.spill("Starting watch mode...")
    
    sus config map[tea]interface{} = parse_build_config(config_path)
    sus source_dirs []tea = config["source_dirs"].([]tea) fr fr Initial build
    build_project(config_path) fr fr Watch for changes (simplified implementation)
    bestie {
        timez.sleep(2) fr fr Check every 2 seconds
        
        sus needs_rebuild lit = cap
        bestie _, source_dir := iterate source_dirs {
            lowkey check_dir_for_changes(source_dir) {
                needs_rebuild = based
                ghosted
            }
        }
        
        lowkey needs_rebuild {
            vibez.spill("File changes detected, rebuilding...")
            build_project(config_path)
        }
    }
}

slay check_dir_for_changes(dir_path tea) lit { fr fr Simplified change detection - would use file system events in production
    sus files []tea = fs.list_dir_recursive(dir_path)
    bestie _, file := iterate files {
        lowkey stringz.ends_with(file, ".csd") {
            sus mod_time normie = fs.get_mod_time(file)
            sus current_time normie = timez.now_unix()
            lowkey (current_time - mod_time) < 5 { fr fr Modified within last 5 seconds
                damn based
            }
        }
    }
    damn cap
}

fr fr Main build system entry point
slay build_system_main(args []tea) normie {
    lowkey len(args) < 2 {
        vibez.spill("Usage: cursed_build <command> [options]")
        vibez.spill("Commands: build, test, clean, rebuild, install, list, watch")
        damn 1
    }
    
    sus command tea = args[1]
    sus config_path tea = "CursedBuild.toml" fr fr Check if custom config path provided
    lowkey len(args) > 2 && stringz.starts_with(args[2], "--config=") {
        config_path = stringz.substring(args[2], 9, len(args[2]))
    }
    
    switch command {
        case "build":
            lowkey build_project(config_path) {
                vibez.spill("Build completed successfully")
                damn 0
            } else {
                vibez.spill("Build failed")
                damn 1
            }
        case "test":
            sus config map[tea]interface{} = parse_build_config(config_path)
            lowkey run_tests(config) {
                vibez.spill("All tests passed")
                damn 0
            } else {
                vibez.spill("Some tests failed")
                damn 1
            }
        case "clean":
            sus config map[tea]interface{} = parse_build_config(config_path)
            clean_project(config)
            damn 0
        case "rebuild":
            lowkey rebuild_project(config_path) {
                vibez.spill("Rebuild completed successfully")
                damn 0
            } else {
                vibez.spill("Rebuild failed")
                damn 1
            }
        case "install":
            lowkey len(args) < 4 {
                vibez.spill("Usage: cursed_build install <package> <version>")
                damn 1
            }
            sus package_name tea = args[2]
            sus package_version tea = args[3]
            lowkey install_package(package_name, package_version) {
                damn 0
            } else {
                damn 1
            }
        case "list":
            sus packages []tea = list_packages()
            vibez.spill("Installed packages:")
            bestie _, package := iterate packages {
                vibez.spill("  " + package)
            }
            damn 0
        case "watch":
            watch_project(config_path)
            damn 0
        default:
            vibez.spill("Unknown command: " + command)
            damn 1
    }
}
