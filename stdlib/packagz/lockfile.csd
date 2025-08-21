# Lock File Management for CURSED Package Manager
# Ensures reproducible builds with cursed.lock files
yeet "filez"
yeet "jsonz"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "cryptz"

# Lock file format version
sus LOCK_FILE_VERSION tea = "1"
sus LOCK_FILE_NAME tea = "cursed.lock"

# Locked package entry
squad LockedPackage {
    sus name tea
    sus version tea
    sus source tea           # registry, git, local
    sus checksum tea         # SHA-256 hash for integrity
    sus dependencies []LockedDependency
    sus resolved_at tea      # ISO timestamp when resolved
}

# Locked dependency reference
squad LockedDependency {
    sus name tea
    sus version tea
    sus checksum tea
}

# Lock file structure
squad LockFile {
    sus version tea
    sus generated_at tea
    sus packages []LockedPackage
    sus metadata LockMetadata
}

# Lock file metadata
squad LockMetadata {
    sus cursed_version tea
    sus resolver_version tea
    sus platform tea
    sus checksum tea         # Checksum of the entire lock file content
}

# Lock file manager
squad LockFileManager {
    sus project_path tea
    sus lock_file_path tea
}

# Initialize lock file manager
slay init_lock_manager(project_path tea) LockFileManager {
    sus lock_path tea = project_path + "/" + LOCK_FILE_NAME
    
    damn LockFileManager {
        project_path: project_path,
        lock_file_path: lock_path
    }
}

# Generate lock file from resolved dependencies
slay generate_lock_file(manager LockFileManager, resolved_packages []InstalledPackage, registry PackageRegistry) lit {
    vibez.spill("Generating lock file...")
    
    sus locked_packages []LockedPackage = []
    
    # Convert resolved packages to locked packages
    bestie (sus i drip = 0; i < arrayz.len(resolved_packages); i = i + 1) {
        sus pkg InstalledPackage = resolved_packages[i]
        
        # Get package metadata for checksum
        sus metadata PackageMetadata = get_package_info(registry, pkg.name, pkg.version)
        ready (metadata.name == "") {
            vibez.spill("Warning: Could not get metadata for", pkg.name, pkg.version)
            continue
        }
        
        # Convert dependencies to locked dependencies
        sus locked_deps []LockedDependency = []
        bestie (sus j drip = 0; j < arrayz.len(pkg.dependencies); j = j + 1) {
            sus dep_name tea = pkg.dependencies[j]
            
            # Find the resolved version of this dependency
            sus dep_version tea = find_resolved_version(resolved_packages, dep_name)
            ready (dep_version == "") {
                continue  # Skip unresolved dependencies
            }
            
            # Get checksum for dependency
            sus dep_metadata PackageMetadata = get_package_info(registry, dep_name, dep_version)
            sus dep_checksum tea = dep_metadata.checksum
            ready (dep_checksum == "") {
                # Generate checksum if not provided
                dep_checksum = generate_package_checksum(dep_name, dep_version)
            }
            
            sus locked_dep LockedDependency = LockedDependency {
                name: dep_name,
                version: dep_version,
                checksum: dep_checksum
            }
            locked_deps = arrayz.append(locked_deps, locked_dep)
        }
        
        # Create locked package
        sus locked_pkg LockedPackage = LockedPackage {
            name: pkg.name,
            version: pkg.version,
            source: "registry",
            checksum: metadata.checksum,
            dependencies: locked_deps,
            resolved_at: get_current_iso_timestamp()
        }
        
        locked_packages = arrayz.append(locked_packages, locked_pkg)
    }
    
    # Sort packages by name for consistent output
    locked_packages = sort_locked_packages(locked_packages)
    
    # Create lock file metadata
    sus metadata LockMetadata = LockMetadata {
        cursed_version: get_cursed_version(),
        resolver_version: get_resolver_version(),
        platform: get_platform_info(),
        checksum: ""  # Will be calculated after serialization
    }
    
    # Create lock file
    sus lock_file LockFile = LockFile {
        version: LOCK_FILE_VERSION,
        generated_at: get_current_iso_timestamp(),
        packages: locked_packages,
        metadata: metadata
    }
    
    # Serialize and calculate checksum
    sus json_content tea = serialize_lock_file(lock_file)
    sus content_checksum tea = cryptz.sha256_string(json_content)
    lock_file.metadata.checksum = content_checksum
    
    # Re-serialize with checksum
    json_content = serialize_lock_file(lock_file)
    
    # Write lock file
    ready (!filez.write_file(manager.lock_file_path, json_content)) {
        vibez.spill("Failed to write lock file:", manager.lock_file_path)
        damn cap
    }
    
    vibez.spill("Lock file generated successfully")
    damn based
}

# Load existing lock file
slay load_lock_file(manager LockFileManager) LockFile {
    ready (!filez.file_exists(manager.lock_file_path)) {
        vibez.spill("No lock file found:", manager.lock_file_path)
        damn LockFile{}
    }
    
    sus content tea = filez.read_file(manager.lock_file_path)
    ready (content == "") {
        vibez.spill("Failed to read lock file")
        damn LockFile{}
    }
    
    sus lock_file LockFile = deserialize_lock_file(content)
    
    # Validate lock file integrity
    ready (!validate_lock_file(lock_file, content)) {
        vibez.spill("Lock file integrity check failed")
        damn LockFile{}
    }
    
    damn lock_file
}

# Validate lock file integrity
slay validate_lock_file(lock_file LockFile, content tea) lit {
    # Check version compatibility
    ready (lock_file.version != LOCK_FILE_VERSION) {
        vibez.spill("Lock file version mismatch. Expected:", LOCK_FILE_VERSION, "Got:", lock_file.version)
        damn cap
    }
    
    # Validate checksum if present
    ready (lock_file.metadata.checksum != "") {
        # Create temporary lock file without checksum for verification
        sus temp_lock_file LockFile = lock_file
        temp_lock_file.metadata.checksum = ""
        sus temp_content tea = serialize_lock_file(temp_lock_file)
        sus calculated_checksum tea = cryptz.sha256_string(temp_content)
        
        ready (calculated_checksum != lock_file.metadata.checksum) {
            vibez.spill("Lock file checksum mismatch")
            damn cap
        }
    }
    
    # Validate package checksums
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus pkg LockedPackage = lock_file.packages[i]
        ready (pkg.checksum == "") {
            vibez.spill("Warning: Package", pkg.name, "has no checksum")
        }
        
        # Validate dependency checksums
        bestie (sus j drip = 0; j < arrayz.len(pkg.dependencies); j = j + 1) {
            sus dep LockedDependency = pkg.dependencies[j]
            ready (dep.checksum == "") {
                vibez.spill("Warning: Dependency", dep.name, "has no checksum")
            }
        }
    }
    
    damn based
}

# Install from lock file
slay install_from_lock_file(manager LockFileManager, pkg_manager PackageManager) lit {
    vibez.spill("Installing from lock file...")
    
    sus lock_file LockFile = load_lock_file(manager)
    ready (lock_file.version == "") {
        vibez.spill("No valid lock file found")
        damn cap
    }
    
    sus success_count drip = 0
    sus total_count drip = arrayz.len(lock_file.packages)
    
    # Install packages in dependency order
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus locked_pkg LockedPackage = lock_file.packages[i]
        
        vibez.spill("Installing", locked_pkg.name, "version", locked_pkg.version)
        
        # Check if already installed with correct version and checksum
        sus existing InstalledPackage = get_installed_package(pkg_manager, locked_pkg.name)
        ready (existing.name != "" && existing.version == locked_pkg.version) {
            ready (verify_package_checksum(existing, locked_pkg.checksum)) {
                vibez.spill("Package", locked_pkg.name, "already installed with correct version and checksum")
                success_count = success_count + 1
                continue
            }
        }
        
        # Install the specific locked version
        ready (install_locked_package(pkg_manager, locked_pkg)) {
            success_count = success_count + 1
        } otherwise {
            vibez.spill("Failed to install", locked_pkg.name, "version", locked_pkg.version)
        }
    }
    
    ready (success_count == total_count) {
        vibez.spill("All packages installed successfully from lock file")
        damn based
    } otherwise {
        vibez.spill("Installed", success_count, "of", total_count, "packages")
        damn cap
    }
}

# Install a specific locked package
slay install_locked_package(manager PackageManager, locked_pkg LockedPackage) lit {
    # Get package metadata
    sus metadata PackageMetadata = get_package_info(manager.registry, locked_pkg.name, locked_pkg.version)
    ready (metadata.name == "") {
        vibez.spill("Package not found:", locked_pkg.name, "version", locked_pkg.version)
        damn cap
    }
    
    # Verify checksum before installation
    ready (locked_pkg.checksum != "" && metadata.checksum != locked_pkg.checksum) {
        vibez.spill("Checksum mismatch for", locked_pkg.name)
        vibez.spill("Expected:", locked_pkg.checksum)
        vibez.spill("Got:", metadata.checksum)
        damn cap
    }
    
    # Download and install
    sus archive_path tea = download_package(manager, locked_pkg.name, locked_pkg.version)
    ready (archive_path == "") {
        damn cap
    }
    
    # Verify downloaded package checksum
    sus downloaded_checksum tea = cryptz.sha256_file(archive_path)
    ready (locked_pkg.checksum != "" && downloaded_checksum != locked_pkg.checksum) {
        vibez.spill("Downloaded package checksum mismatch for", locked_pkg.name)
        filez.remove_file(archive_path)
        damn cap
    }
    
    # Extract and install
    sus extract_dir tea = manager.installer.temp_dir + "/" + locked_pkg.name + "-" + locked_pkg.version
    ready (!extract_package(archive_path, extract_dir)) {
        vibez.spill("Failed to extract package", locked_pkg.name)
        damn cap
    }
    
    sus install_path tea = manager.installer.install_dir + "/" + locked_pkg.name + "-" + locked_pkg.version
    ready (!filez.copy_dir(extract_dir, install_path)) {
        vibez.spill("Failed to install package to", install_path)
        damn cap
    }
    
    # Record installation
    sus installed_package InstalledPackage = InstalledPackage {
        name: locked_pkg.name,
        version: locked_pkg.version,
        install_path: install_path,
        installed_at: get_current_iso_timestamp(),
        dependencies: extract_dependency_names(locked_pkg.dependencies)
    }
    
    manager.installer.installed_packages = arrayz.append(manager.installer.installed_packages, installed_package)
    save_installed_packages(manager)
    
    # Cleanup temporary files
    filez.remove_file(archive_path)
    filez.remove_dir_all(extract_dir)
    
    damn based
}

# Update lock file after dependency changes
slay update_lock_file(manager LockFileManager, pkg_manager PackageManager) lit {
    vibez.spill("Updating lock file...")
    
    # Load existing lock file for comparison
    sus existing_lock LockFile = load_lock_file(manager)
    
    # Get current installed packages
    sus installed []InstalledPackage = list_installed_packages(pkg_manager)
    
    # Check if lock file needs updating
    ready (lock_file_up_to_date(existing_lock, installed)) {
        vibez.spill("Lock file is already up to date")
        damn based
    }
    
    # Generate new lock file
    damn generate_lock_file(manager, installed, pkg_manager.registry)
}

# Check if lock file is up to date with installed packages
slay lock_file_up_to_date(lock_file LockFile, installed []InstalledPackage) lit {
    ready (arrayz.len(lock_file.packages) != arrayz.len(installed)) {
        damn cap
    }
    
    # Check each installed package against lock file
    bestie (sus i drip = 0; i < arrayz.len(installed); i = i + 1) {
        sus pkg InstalledPackage = installed[i]
        sus locked_pkg LockedPackage = find_locked_package(lock_file, pkg.name)
        
        ready (locked_pkg.name == "" || locked_pkg.version != pkg.version) {
            damn cap
        }
    }
    
    damn based
}

# Serialize lock file to JSON
slay serialize_lock_file(lock_file LockFile) tea {
    sus json_obj JsonValue = jsonz.json_create_object()
    
    # Set basic fields
    json_obj = jsonz.json_object_set(json_obj, "version", jsonz.json_create_string(lock_file.version))
    json_obj = jsonz.json_object_set(json_obj, "generated_at", jsonz.json_create_string(lock_file.generated_at))
    
    # Serialize packages
    sus packages_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus pkg LockedPackage = lock_file.packages[i]
        sus pkg_obj JsonValue = serialize_locked_package(pkg)
        packages_array = jsonz.json_array_push(packages_array, pkg_obj)
    }
    json_obj = jsonz.json_object_set(json_obj, "packages", packages_array)
    
    # Serialize metadata
    sus metadata_obj JsonValue = serialize_lock_metadata(lock_file.metadata)
    json_obj = jsonz.json_object_set(json_obj, "metadata", metadata_obj)
    
    damn jsonz.json_stringify_pretty(json_obj)
}

# Serialize locked package to JSON
slay serialize_locked_package(pkg LockedPackage) JsonValue {
    sus pkg_obj JsonValue = jsonz.json_create_object()
    
    pkg_obj = jsonz.json_object_set(pkg_obj, "name", jsonz.json_create_string(pkg.name))
    pkg_obj = jsonz.json_object_set(pkg_obj, "version", jsonz.json_create_string(pkg.version))
    pkg_obj = jsonz.json_object_set(pkg_obj, "source", jsonz.json_create_string(pkg.source))
    pkg_obj = jsonz.json_object_set(pkg_obj, "checksum", jsonz.json_create_string(pkg.checksum))
    pkg_obj = jsonz.json_object_set(pkg_obj, "resolved_at", jsonz.json_create_string(pkg.resolved_at))
    
    # Serialize dependencies
    sus deps_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(pkg.dependencies); i = i + 1) {
        sus dep LockedDependency = pkg.dependencies[i]
        sus dep_obj JsonValue = jsonz.json_create_object()
        
        dep_obj = jsonz.json_object_set(dep_obj, "name", jsonz.json_create_string(dep.name))
        dep_obj = jsonz.json_object_set(dep_obj, "version", jsonz.json_create_string(dep.version))
        dep_obj = jsonz.json_object_set(dep_obj, "checksum", jsonz.json_create_string(dep.checksum))
        
        deps_array = jsonz.json_array_push(deps_array, dep_obj)
    }
    pkg_obj = jsonz.json_object_set(pkg_obj, "dependencies", deps_array)
    
    damn pkg_obj
}

# Serialize lock metadata to JSON
slay serialize_lock_metadata(metadata LockMetadata) JsonValue {
    sus meta_obj JsonValue = jsonz.json_create_object()
    
    meta_obj = jsonz.json_object_set(meta_obj, "cursed_version", jsonz.json_create_string(metadata.cursed_version))
    meta_obj = jsonz.json_object_set(meta_obj, "resolver_version", jsonz.json_create_string(metadata.resolver_version))
    meta_obj = jsonz.json_object_set(meta_obj, "platform", jsonz.json_create_string(metadata.platform))
    meta_obj = jsonz.json_object_set(meta_obj, "checksum", jsonz.json_create_string(metadata.checksum))
    
    damn meta_obj
}

# Deserialize lock file from JSON
slay deserialize_lock_file(content tea) LockFile {
    sus json_obj JsonValue = jsonz.json_parse(content)
    ready (json_obj.type != "object") {
        damn LockFile{}
    }
    
    sus version tea = jsonz.json_get_string(json_obj, "version")
    sus generated_at tea = jsonz.json_get_string(json_obj, "generated_at")
    
    # Deserialize packages
    sus packages []LockedPackage = []
    ready (jsonz.json_has_key(json_obj, "packages")) {
        sus packages_json JsonValue = jsonz.json_get_object(json_obj, "packages")
        ready (packages_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(packages_json.array_values); i = i + 1) {
                sus pkg_json JsonValue = packages_json.array_values[i]
                sus pkg LockedPackage = deserialize_locked_package(pkg_json)
                packages = arrayz.append(packages, pkg)
            }
        }
    }
    
    # Deserialize metadata
    sus metadata LockMetadata = LockMetadata{}
    ready (jsonz.json_has_key(json_obj, "metadata")) {
        sus meta_json JsonValue = jsonz.json_get_object(json_obj, "metadata")
        metadata = deserialize_lock_metadata(meta_json)
    }
    
    damn LockFile {
        version: version,
        generated_at: generated_at,
        packages: packages,
        metadata: metadata
    }
}

# Deserialize locked package from JSON
slay deserialize_locked_package(pkg_json JsonValue) LockedPackage {
    sus name tea = jsonz.json_get_string(pkg_json, "name")
    sus version tea = jsonz.json_get_string(pkg_json, "version")
    sus source tea = jsonz.json_get_string(pkg_json, "source")
    sus checksum tea = jsonz.json_get_string(pkg_json, "checksum")
    sus resolved_at tea = jsonz.json_get_string(pkg_json, "resolved_at")
    
    # Deserialize dependencies
    sus dependencies []LockedDependency = []
    ready (jsonz.json_has_key(pkg_json, "dependencies")) {
        sus deps_json JsonValue = jsonz.json_get_object(pkg_json, "dependencies")
        ready (deps_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(deps_json.array_values); i = i + 1) {
                sus dep_json JsonValue = deps_json.array_values[i]
                sus dep LockedDependency = LockedDependency {
                    name: jsonz.json_get_string(dep_json, "name"),
                    version: jsonz.json_get_string(dep_json, "version"),
                    checksum: jsonz.json_get_string(dep_json, "checksum")
                }
                dependencies = arrayz.append(dependencies, dep)
            }
        }
    }
    
    damn LockedPackage {
        name: name,
        version: version,
        source: source,
        checksum: checksum,
        dependencies: dependencies,
        resolved_at: resolved_at
    }
}

# Deserialize lock metadata from JSON
slay deserialize_lock_metadata(meta_json JsonValue) LockMetadata {
    damn LockMetadata {
        cursed_version: jsonz.json_get_string(meta_json, "cursed_version"),
        resolver_version: jsonz.json_get_string(meta_json, "resolver_version"),
        platform: jsonz.json_get_string(meta_json, "platform"),
        checksum: jsonz.json_get_string(meta_json, "checksum")
    }
}

# Utility functions
slay find_resolved_version(packages []InstalledPackage, name tea) tea {
    bestie (sus i drip = 0; i < arrayz.len(packages); i = i + 1) {
        sus pkg InstalledPackage = packages[i]
        ready (pkg.name == name) {
            damn pkg.version
        }
    }
    damn ""
}

slay find_locked_package(lock_file LockFile, name tea) LockedPackage {
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus pkg LockedPackage = lock_file.packages[i]
        ready (pkg.name == name) {
            damn pkg
        }
    }
    damn LockedPackage{}
}

slay sort_locked_packages(packages []LockedPackage) []LockedPackage {
    # Simple alphabetical sort by name
    sus sorted []LockedPackage = arrayz.copy(packages)
    sus n drip = arrayz.len(sorted)
    
    bestie (sus i drip = 0; i < n - 1; i = i + 1) {
        bestie (sus j drip = 0; j < n - i - 1; j = j + 1) {
            ready (stringz.compare(sorted[j].name, sorted[j + 1].name) > 0) {
                sus temp LockedPackage = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    
    damn sorted
}

slay extract_dependency_names(locked_deps []LockedDependency) []tea {
    sus names []tea = []
    bestie (sus i drip = 0; i < arrayz.len(locked_deps); i = i + 1) {
        sus dep LockedDependency = locked_deps[i]
        names = arrayz.append(names, dep.name)
    }
    damn names
}

slay verify_package_checksum(pkg InstalledPackage, expected_checksum tea) lit {
    ready (expected_checksum == "") {
        damn based  # No checksum to verify
    }
    
    # Calculate checksum of installed package
    sus calculated_checksum tea = generate_directory_checksum(pkg.install_path)
    damn calculated_checksum == expected_checksum
}

slay generate_package_checksum(name tea, version tea) tea {
    # Generate deterministic checksum for package
    sus input tea = name + ":" + version + ":" + get_current_iso_timestamp()
    damn cryptz.sha256_string(input)
}

slay generate_directory_checksum(dir_path tea) tea {
    # In real implementation: calculate checksum of all files in directory
    damn cryptz.sha256_string(dir_path + get_current_iso_timestamp())
}

slay get_current_iso_timestamp() tea {
    # In real implementation: return current ISO 8601 timestamp
    damn "2025-08-21T12:00:00Z"
}

slay get_cursed_version() tea {
    damn "1.0.0"
}

slay get_resolver_version() tea {
    damn "1.0.0"
}

slay get_platform_info() tea {
    # In real implementation: return platform information
    damn "linux-x86_64"
}
