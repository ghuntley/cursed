fr fr CROSS-PLATFORM PATH HANDLING DEMONSTRATION
fr fr Shows how CURSED handles Windows, Unix, and macOS paths seamlessly

yeet "filez"
yeet "vibez"

slay demo_windows_paths() {
    vibez.spill("\n🪟 === WINDOWS PATH HANDLING ===")
    
    fr fr Windows drive paths
    sus win_drive tea = "C:\\Users\\Developer\\Documents\\project\\file.txt"
    vibez.spill("Original Windows path: " + win_drive)
    vibez.spill("Is absolute: " + bool_to_string(is_absolute_path(win_drive)))
    vibez.spill("Normalized: " + cross_platform_normalize(win_drive))
    vibez.spill("Parent directory: " + get_parent_directory(win_drive))
    vibez.spill("Filename: " + get_filename_component(win_drive))
    vibez.spill("Extension: " + get_extension_component(win_drive))
    
    fr fr Windows UNC paths
    vibez.spill("\n--- Windows UNC Paths ---")
    sus unc_path tea = "\\\\fileserver\\shared\\projects\\cursed\\src\\main.csd"
    vibez.spill("UNC path: " + unc_path)
    
    sus unc_info DriveInfo = parse_drive_info(unc_path)
    vibez.spill("Is UNC: " + bool_to_string(unc_info.is_unc))
    vibez.spill("Server: " + unc_info.server_name)
    vibez.spill("Share: " + unc_info.share_name)
    vibez.spill("Root: " + get_root_path(unc_path))
    
    fr fr Mixed separators (Windows accepts both)
    vibez.spill("\n--- Mixed Separators ---")
    sus mixed_sep tea = "C:\\Users/Developer\\Documents/file.txt"
    vibez.spill("Mixed separators: " + mixed_sep)
    vibez.spill("Normalized: " + normalize_path_separators(mixed_sep))
    
    fr fr Path joining
    vibez.spill("\n--- Windows Path Joining ---")
    sus win_parts []tea = ["C:\\Program Files", "MyApp", "bin", "app.exe"]
    sus win_joined tea = cross_platform_join(win_parts)
    vibez.spill("Parts: [" + join_array_with_comma(win_parts) + "]")
    vibez.spill("Joined: " + win_joined)
}

slay demo_unix_paths() {
    vibez.spill("\n🐧 === UNIX/LINUX PATH HANDLING ===")
    
    fr fr Unix absolute paths
    sus unix_abs tea = "/home/developer/projects/cursed/src/main.csd"
    vibez.spill("Unix absolute path: " + unix_abs)
    vibez.spill("Is absolute: " + bool_to_string(is_absolute_path(unix_abs)))
    vibez.spill("Normalized: " + cross_platform_normalize(unix_abs))
    vibez.spill("Parent directory: " + get_parent_directory(unix_abs))
    vibez.spill("Filename: " + get_filename_component(unix_abs))
    
    fr fr Unix relative paths
    vibez.spill("\n--- Unix Relative Paths ---")
    sus unix_rel tea = "../src/utils/helpers.csd"
    vibez.spill("Relative path: " + unix_rel)
    vibez.spill("Is absolute: " + bool_to_string(is_absolute_path(unix_rel)))
    sus unix_abs_converted tea = cross_platform_absolute(unix_rel)
    vibez.spill("Converted to absolute: " + unix_abs_converted)
    
    fr fr Path joining
    vibez.spill("\n--- Unix Path Joining ---")
    sus unix_parts []tea = ["/usr", "local", "bin", "cursed"]
    sus unix_joined tea = cross_platform_join(unix_parts)
    vibez.spill("Parts: [" + join_array_with_comma(unix_parts) + "]")
    vibez.spill("Joined: " + unix_joined)
}

slay demo_path_normalization() {
    vibez.spill("\n🔧 === PATH NORMALIZATION ===")
    
    fr fr Complex paths with . and ..
    sus complex_paths []tea = [
        "/home/user/./documents/../downloads/file.txt",
        "C:\\Users\\..\\Users\\Developer\\Documents\\file.txt",
        "/usr/local/bin/../share/../lib/libcursed.so",
        "documents/./projects/../archives/./project.zip"
    ]
    
    sus i drip = 0
    bestie (i < array_length(complex_paths)) {
        sus original tea = complex_paths[i]
        sus normalized tea = cross_platform_normalize(original)
        vibez.spill("Original:   " + original)
        vibez.spill("Normalized: " + normalized)
        vibez.spill("")
        i = i + 1
    }
}

slay demo_relative_path_calculations() {
    vibez.spill("\n📐 === RELATIVE PATH CALCULATIONS ===")
    
    fr fr Calculate relative paths between different locations
    sus path_pairs [][]tea = [
        ["/home/user/projects/app1", "/home/user/projects/app2/lib.csd"],
        ["/home/user/documents", "/home/user/downloads/file.zip"],
        ["C:\\Users\\Dev\\Projects\\A", "C:\\Users\\Dev\\Projects\\B\\file.txt"],
        ["/usr/local/bin", "/usr/share/doc/cursed/manual.pdf"]
    ]
    
    sus i drip = 0
    bestie (i < array_length(path_pairs)) {
        sus from_path tea = path_pairs[i][0]
        sus to_path tea = path_pairs[i][1]
        sus relative tea = calculate_relative_path(from_path, to_path)
        
        vibez.spill("From: " + from_path)
        vibez.spill("To:   " + to_path)
        vibez.spill("Relative: " + relative)
        vibez.spill("")
        i = i + 1
    }
}

slay demo_file_operations() {
    vibez.spill("\n📁 === FILE OPERATIONS ===")
    
    fr fr Various file path operations
    sus file_paths []tea = [
        "/home/user/documents/report.pdf",
        "C:\\Users\\Developer\\source.cpp",
        "/usr/local/share/cursed/stdlib/filez.csd",
        "\\\\server\\share\\projects\\app.exe"
    ]
    
    sus i drip = 0
    bestie (i < array_length(file_paths)) {
        sus path tea = file_paths[i]
        vibez.spill("Path: " + path)
        vibez.spill("  Directory: " + get_parent_directory(path))
        vibez.spill("  Filename: " + get_filename_component(path))
        vibez.spill("  Name without ext: " + get_filename_without_extension(path))
        vibez.spill("  Extension: " + get_extension_component(path))
        vibez.spill("  Is absolute: " + bool_to_string(is_absolute_path(path)))
        vibez.spill("")
        i = i + 1
    }
}

slay demo_path_validation() {
    vibez.spill("\n✅ === PATH VALIDATION ===")
    
    fr fr Valid paths
    sus valid_paths []tea = [
        "C:\\Users\\Developer\\file.txt",
        "/home/user/documents/file.txt",
        "\\\\server\\share\\folder\\file.txt",
        "relative/path/to/file.txt"
    ]
    
    vibez.spill("--- Valid Paths ---")
    sus i drip = 0
    bestie (i < array_length(valid_paths)) {
        sus path tea = valid_paths[i]
        sus is_valid lit = validate_path_chars(path)
        sus length_valid lit = validate_path_length(path)
        vibez.spill("Path: " + path)
        vibez.spill("  Valid chars: " + bool_to_string(is_valid))
        vibez.spill("  Valid length: " + bool_to_string(length_valid))
        i = i + 1
    }
    
    fr fr Invalid paths (Windows)
    sus invalid_paths []tea = [
        "C:\\Users\\file<name>.txt",
        "C:\\Users\\file|name.txt",
        "C:\\Users\\file\"name\".txt",
        "C:\\Users\\file?.txt"
    ]
    
    vibez.spill("\n--- Invalid Paths (Windows) ---")
    i = 0
    bestie (i < array_length(invalid_paths)) {
        sus path tea = invalid_paths[i]
        sus is_valid lit = validate_path_chars(path)
        vibez.spill("Path: " + path)
        vibez.spill("  Valid chars: " + bool_to_string(is_valid))
        i = i + 1
    }
}

slay demo_cross_platform_scenarios() {
    vibez.spill("\n🌐 === CROSS-PLATFORM SCENARIOS ===")
    
    fr fr Show how the same logical path looks on different platforms
    sus logical_paths []tea = [
        "user_home/documents/project/src/main.csd",
        "system_root/usr/local/bin/cursed",
        "temp_dir/build_output/app.exe"
    ]
    
    sus i drip = 0
    bestie (i < array_length(logical_paths)) {
        sus logical tea = logical_paths[i]
        vibez.spill("Logical path: " + logical)
        
        fr fr Show how it would look on Windows
        sus win_base tea = "C:\\Users\\Developer\\"
        sus win_path tea = cross_platform_join([win_base, logical])
        vibez.spill("  Windows: " + normalize_path_separators(win_path))
        
        fr fr Show how it would look on Unix
        sus unix_base tea = "/home/developer/"
        sus unix_path tea = cross_platform_join([unix_base, logical])
        vibez.spill("  Unix: " + normalize_path_separators(unix_path))
        
        vibez.spill("")
        i = i + 1
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay bool_to_string(value lit) tea {
    ready (value) {
        damn "true"
    } otherwise {
        damn "false"
    }
}

slay join_array_with_comma(arr []tea) tea {
    ready (array_length(arr) == 0) {
        damn ""
    }
    
    sus result tea = arr[0]
    sus i drip = 1
    bestie (i < array_length(arr)) {
        result = result + ", " + arr[i]
        i = i + 1
    }
    damn result
}

fr fr ===== MAIN DEMONSTRATION =====

slay run_cross_platform_demo() {
    vibez.spill("🚀 CURSED Cross-Platform Path Handling Demonstration")
    vibez.spill("=" + repeat_string("=", 60))
    
    fr fr Detect current platform
    sus platform tea = detect_platform()
    sus current_sep tea = get_platform_separator()
    vibez.spill("Detected Platform: " + platform)
    vibez.spill("Path Separator: '" + current_sep + "'")
    
    fr fr Run all demonstrations
    demo_windows_paths()
    demo_unix_paths()
    demo_path_normalization()
    demo_relative_path_calculations()
    demo_file_operations()
    demo_path_validation()
    demo_cross_platform_scenarios()
    
    vibez.spill("\n" + repeat_string("=", 60))
    vibez.spill("✨ Cross-Platform Path Handling Demo Complete!")
    vibez.spill("CURSED now handles Windows, Unix, and macOS paths seamlessly.")
    vibez.spill("Features:")
    vibez.spill("  ✅ Windows drive letters (C:, D:, etc.)")
    vibez.spill("  ✅ UNC paths (\\\\server\\share)")
    vibez.spill("  ✅ Mixed separators (/ and \\)")
    vibez.spill("  ✅ Path normalization (. and .. resolution)")
    vibez.spill("  ✅ Relative path calculations")
    vibez.spill("  ✅ Comprehensive validation")
    vibez.spill("  ✅ Cross-platform compatibility")
}

slay repeat_string(str tea, count drip) tea {
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + str
        i = i + 1
    }
    damn result
}

fr fr Run demonstration
run_cross_platform_demo()
