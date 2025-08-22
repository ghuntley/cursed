# CURSED Package Manager Lock File Management
# Provides reproducible builds through dependency locking
yeet "filez"
yeet "jsonz"
yeet "arrayz"
yeet "stringz"
yeet "vibez"

# Lock file entry for a package
squad LockedPackage {
    sus name tea
    sus version tea
    sus source tea        # registry URL or git repo
    sus checksum tea      # SHA-256 checksum for verification
    sus resolved_deps []tea  # Direct dependencies resolved
}

# Lock file structure
squad LockFile {
    sus version tea       # Lock file format version
    sus metadata LockMetadata
    sus packages []LockedPackage
}

# Lock file metadata
squad LockMetadata {
    sus project_name tea
    sus project_version tea
    sus generated_at tea
    sus cursed_version tea
    sus checksum tea      # Overall lock file checksum
}

# Create new lock file for project
slay create_lock_file(project_name tea, project_version tea, packages []LockedPackage) LockFile {
    sus metadata LockMetadata = LockMetadata {
        project_name: project_name,
        project_version: project_version,
        generated_at: get_current_timestamp(),
        cursed_version: "1.0.0",
        checksum: ""
    }
    
    sus lock_file LockFile = LockFile {
        version: "1",
        metadata: metadata,
        packages: packages
    }
    
    # Generate checksum for integrity
    lock_file.metadata.checksum = generate_lock_checksum(lock_file)
    
    damn lock_file
}

# Save lock file to disk (cursed.lock format)
slay save_lock_file(lock_file LockFile, path tea) lit {
    vibez.spill("Generating lock file:", path)
    
    sus json_obj JsonValue = jsonz.json_create_object()
    
    # Metadata section
    sus metadata_obj JsonValue = jsonz.json_create_object()
    metadata_obj = jsonz.json_object_set(metadata_obj, "project_name", 
                                        jsonz.json_create_string(lock_file.metadata.project_name))
    metadata_obj = jsonz.json_object_set(metadata_obj, "project_version", 
                                        jsonz.json_create_string(lock_file.metadata.project_version))
    metadata_obj = jsonz.json_object_set(metadata_obj, "generated_at", 
                                        jsonz.json_create_string(lock_file.metadata.generated_at))
    metadata_obj = jsonz.json_object_set(metadata_obj, "cursed_version", 
                                        jsonz.json_create_string(lock_file.metadata.cursed_version))
    metadata_obj = jsonz.json_object_set(metadata_obj, "checksum", 
                                        jsonz.json_create_string(lock_file.metadata.checksum))
    
    json_obj = jsonz.json_object_set(json_obj, "version", jsonz.json_create_string(lock_file.version))
    json_obj = jsonz.json_object_set(json_obj, "metadata", metadata_obj)
    
    # Packages section
    sus packages_obj JsonValue = jsonz.json_create_object()
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus pkg LockedPackage = lock_file.packages[i]
        
        sus package_obj JsonValue = jsonz.json_create_object()
        package_obj = jsonz.json_object_set(package_obj, "version", jsonz.json_create_string(pkg.version))
        package_obj = jsonz.json_object_set(package_obj, "source", jsonz.json_create_string(pkg.source))
        package_obj = jsonz.json_object_set(package_obj, "checksum", jsonz.json_create_string(pkg.checksum))
        
        # Dependencies array
        sus deps_array JsonValue = jsonz.json_create_array()
        bestie (sus j drip = 0; j < arrayz.len(pkg.resolved_deps); j = j + 1) {
            deps_array = jsonz.json_array_push(deps_array, jsonz.json_create_string(pkg.resolved_deps[j]))
        }
        package_obj = jsonz.json_object_set(package_obj, "dependencies", deps_array)
        
        packages_obj = jsonz.json_object_set(packages_obj, pkg.name, package_obj)
    }
    
    json_obj = jsonz.json_object_set(json_obj, "packages", packages_obj)
    
    sus lock_content tea = jsonz.json_stringify_pretty(json_obj)
    damn filez.write_file(path, lock_content)
}

# Load lock file from disk
slay load_lock_file(path tea) (LockFile, lit) {
    ready (!filez.file_exists(path)) {
        vibez.spill("Lock file not found:", path)
        damn (LockFile{}, cap)
    }
    
    sus content tea = filez.read_file(path)
    ready (content == "") {
        vibez.spill("Failed to read lock file:", path)
        damn (LockFile{}, cap)
    }
    
    sus json_obj JsonValue = jsonz.json_parse(content)
    ready (json_obj.type != "object") {
        vibez.spill("Invalid lock file format")
        damn (LockFile{}, cap)
    }
    
    # Parse metadata
    sus metadata_json JsonValue = jsonz.json_get_object(json_obj, "metadata")
    sus metadata LockMetadata = LockMetadata {
        project_name: jsonz.json_get_string(metadata_json, "project_name"),
        project_version: jsonz.json_get_string(metadata_json, "project_version"),
        generated_at: jsonz.json_get_string(metadata_json, "generated_at"),
        cursed_version: jsonz.json_get_string(metadata_json, "cursed_version"),
        checksum: jsonz.json_get_string(metadata_json, "checksum")
    }
    
    # Parse packages
    sus packages_json JsonValue = jsonz.json_get_object(json_obj, "packages")
    sus packages []LockedPackage = []
    
    # In actual implementation, would iterate over object keys
    # This is simplified for demonstration
    sus package_names []tea = json_get_object_keys(packages_json)
    bestie (sus i drip = 0; i < arrayz.len(package_names); i = i + 1) {
        sus pkg_name tea = package_names[i]
        sus pkg_json JsonValue = jsonz.json_get_object(packages_json, pkg_name)
        
        # Parse dependencies
        sus deps_json JsonValue = jsonz.json_get_object(pkg_json, "dependencies")
        sus deps []tea = []
        ready (deps_json.type == "array") {
            bestie (sus j drip = 0; j < arrayz.len(deps_json.array_values); j = j + 1) {
                deps = arrayz.append(deps, deps_json.array_values[j].string_value)
            }
        }
        
        sus locked_pkg LockedPackage = LockedPackage {
            name: pkg_name,
            version: jsonz.json_get_string(pkg_json, "version"),
            source: jsonz.json_get_string(pkg_json, "source"),
            checksum: jsonz.json_get_string(pkg_json, "checksum"),
            resolved_deps: deps
        }
        
        packages = arrayz.append(packages, locked_pkg)
    }
    
    sus lock_file LockFile = LockFile {
        version: jsonz.json_get_string(json_obj, "version"),
        metadata: metadata,
        packages: packages
    }
    
    # Verify checksum integrity
    sus computed_checksum tea = generate_lock_checksum(lock_file)
    ready (computed_checksum != lock_file.metadata.checksum) {
        vibez.spill("Lock file checksum mismatch - file may be corrupted")
        damn (lock_file, cap)
    }
    
    damn (lock_file, based)
}

# Verify that installed packages match lock file
slay verify_lock_integrity(lock_file LockFile, installed_packages []InstalledPackage) lit {
    vibez.spill("Verifying lock file integrity...")
    
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus locked_pkg LockedPackage = lock_file.packages[i]
        sus found lit = cap
        
        bestie (sus j drip = 0; j < arrayz.len(installed_packages); j = j + 1) {
            sus installed InstalledPackage = installed_packages[j]
            ready (installed.name == locked_pkg.name) {
                ready (installed.version != locked_pkg.version) {
                    vibez.spill("Version mismatch for", locked_pkg.name, "- locked:", locked_pkg.version, "installed:", installed.version)
                    damn cap
                }
                found = based
                break
            }
        }
        
        ready (!found) {
            vibez.spill("Missing package from lock file:", locked_pkg.name)
            damn cap
        }
    }
    
    vibez.spill("Lock file verification successful")
    damn based
}

# Update lock file with new package resolution
slay update_lock_file(lock_file LockFile, new_packages []LockedPackage) LockFile {
    # Merge new packages with existing ones, replacing duplicates
    sus updated_packages []LockedPackage = []
    
    # Add existing packages that aren't being updated
    bestie (sus i drip = 0; i < arrayz.len(lock_file.packages); i = i + 1) {
        sus existing LockedPackage = lock_file.packages[i]
        sus replaced lit = cap
        
        bestie (sus j drip = 0; j < arrayz.len(new_packages); j = j + 1) {
            ready (new_packages[j].name == existing.name) {
                replaced = based
                break
            }
        }
        
        ready (!replaced) {
            updated_packages = arrayz.append(updated_packages, existing)
        }
    }
    
    # Add all new/updated packages
    bestie (sus i drip = 0; i < arrayz.len(new_packages); i = i + 1) {
        updated_packages = arrayz.append(updated_packages, new_packages[i])
    }
    
    # Update metadata
    sus new_metadata LockMetadata = lock_file.metadata
    new_metadata.generated_at = get_current_timestamp()
    
    sus updated_lock LockFile = LockFile {
        version: lock_file.version,
        metadata: new_metadata,
        packages: updated_packages
    }
    
    # Regenerate checksum
    updated_lock.metadata.checksum = generate_lock_checksum(updated_lock)
    
    damn updated_lock
}

# Generate comprehensive checksum for lock file integrity
slay generate_lock_checksum(lock_file LockFile) tea {
    # Create deterministic string representation
    sus checksum_input tea = lock_file.version + "|" + lock_file.metadata.project_name + "|" + 
                             lock_file.metadata.project_version + "|"
    
    # Sort packages by name for deterministic ordering
    sus sorted_packages []LockedPackage = sort_packages_by_name(lock_file.packages)
    
    bestie (sus i drip = 0; i < arrayz.len(sorted_packages); i = i + 1) {
        sus pkg LockedPackage = sorted_packages[i]
        checksum_input = checksum_input + pkg.name + ":" + pkg.version + ":" + pkg.checksum + "|"
    }
    
    # In real implementation, would use proper SHA-256
    damn calculate_sha256_simple(checksum_input)
}

# Sort packages by name for deterministic lock file generation
slay sort_packages_by_name(packages []LockedPackage) []LockedPackage {
    sus sorted []LockedPackage = []
    
    # Simple insertion sort for demonstration
    bestie (sus i drip = 0; i < arrayz.len(packages); i = i + 1) {
        sus current LockedPackage = packages[i]
        sus inserted lit = cap
        
        bestie (sus j drip = 0; j < arrayz.len(sorted); j = j + 1) {
            ready (stringz.compare(current.name, sorted[j].name) < 0) {
                sorted = arrayz.insert(sorted, j, current)
                inserted = based
                break
            }
        }
        
        ready (!inserted) {
            sorted = arrayz.append(sorted, current)
        }
    }
    
    damn sorted
}

# Create locked package from package metadata
slay create_locked_package(metadata PackageMetadata, resolved_deps []tea) LockedPackage {
    damn LockedPackage {
        name: metadata.name,
        version: metadata.version,
        source: metadata.download_url,
        checksum: metadata.checksum,
        resolved_deps: resolved_deps
    }
}

# Helper functions (simplified implementations)
slay get_current_timestamp() tea {
    damn "2025-08-22T10:30:00Z"
}

slay calculate_sha256_simple(input tea) tea {
    # Simplified hash function for demonstration
    sus hash_value drip = 0
    bestie (sus i drip = 0; i < stringz.length(input); i = i + 1) {
        sus char_code drip = stringz.char_code_at(input, i)
        hash_value = (hash_value * 31 + char_code) % 1000000
    }
    damn "sha256:" + stringz.from_int(hash_value)
}

slay json_get_object_keys(obj JsonValue) []tea {
    # In real implementation, would extract object keys from JsonValue
    # This is simplified for demonstration
    damn []
}
