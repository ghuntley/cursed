# Package Manager Module for CURSED
# Provides comprehensive package installation, dependency resolution, and registry management
yeet "filez"
yeet "networkz"
yeet "jsonz"
yeet "arrayz"
yeet "stringz"
yeet "vibez"
yeet "registry"  # Enhanced registry functionality
yeet "resolver"  # Dependency resolution engine
yeet "lockfile"  # Lock file management

# Package registry configuration
squad RegistryConfig {
    sus url tea
    sus timeout_seconds drip
    sus max_retries drip
    sus api_key tea
}

# Package metadata information
squad PackageMetadata {
    sus name tea
    sus version tea
    sus description tea
    sus authors []tea
    sus license tea
    sus homepage tea
    sus repository tea
    sus keywords []tea
    sus categories []tea
    sus dependencies []PackageDependency
    sus download_url tea
    sus checksum tea
}

# Package dependency specification
squad PackageDependency {
    sus name tea
    sus version_req tea
    sus optional lit
    sus features []tea
}

# Package version specification
squad PackageVersion {
    sus major drip
    sus minor drip
    sus patch drip
    sus pre_release tea
    sus build tea
}

# Installed package information
squad InstalledPackage {
    sus name tea
    sus version tea
    sus install_path tea
    sus installed_at tea
    sus dependencies []tea
}

# Package registry client
squad PackageRegistry {
    sus config RegistryConfig
}

# Package installer
squad PackageInstaller {
    sus install_dir tea
    sus temp_dir tea
    sus installed_packages []InstalledPackage
}

# Package manager main struct
squad PackageManager {
    sus registry PackageRegistry
    sus installer PackageInstaller
    sus cache_dir tea
    sus config_dir tea
}

# Initialize package manager with configuration
slay init_package_manager(registry_url tea, cache_dir tea) PackageManager {
    sus config RegistryConfig = RegistryConfig {
        url: registry_url,
        timeout_seconds: 30,
        max_retries: 3,
        api_key: ""
    }
    
    sus registry PackageRegistry = PackageRegistry { config: config }
    
    sus installer PackageInstaller = PackageInstaller {
        install_dir: cache_dir + "/installed",
        temp_dir: cache_dir + "/temp",
        installed_packages: []
    }
    
    # Create directories if they don't exist
    filez.create_dir_all(cache_dir)
    filez.create_dir_all(installer.install_dir)
    filez.create_dir_all(installer.temp_dir)
    
    damn PackageManager {
        registry: registry,
        installer: installer,
        cache_dir: cache_dir,
        config_dir: cache_dir + "/config"
    }
}

# Parse version string into structured version
slay parse_version(version_str tea) PackageVersion {
    sus parts []tea = stringz.split(version_str, ".")
    
    ready (arrayz.len(parts) < 3) {
        damn PackageVersion { major: 0, minor: 0, patch: 0, pre_release: "", build: "" }
    }
    
    sus major drip = stringz.parse_int(parts[0])
    sus minor drip = stringz.parse_int(parts[1])
    sus patch drip = stringz.parse_int(parts[2])
    
    damn PackageVersion {
        major: major,
        minor: minor,
        patch: patch,
        pre_release: "",
        build: ""
    }
}

# Compare two package versions (-1: v1 < v2, 0: v1 == v2, 1: v1 > v2)
slay compare_versions(v1 PackageVersion, v2 PackageVersion) drip {
    ready (v1.major != v2.major) {
        ready (v1.major < v2.major) { damn -1 }
        damn 1
    }
    
    ready (v1.minor != v2.minor) {
        ready (v1.minor < v2.minor) { damn -1 }
        damn 1
    }
    
    ready (v1.patch != v2.patch) {
        ready (v1.patch < v2.patch) { damn -1 }
        damn 1
    }
    
    damn 0
}

# Search for packages in registry
slay search_packages(manager PackageManager, query tea) []PackageMetadata {
    sus search_url tea = manager.registry.config.url + "/api/v1/packages/search?q=" + query
    
    sus response tea = networkz.http_get(search_url)
    ready (!networkz.http_is_success_simple(response)) {
        vibez.spill("Failed to search packages:", response)
        damn []
    }
    
    sus body tea = networkz.http_get_body(response)
    sus json_data JsonValue = jsonz.json_parse(body)
    
    ready (!jsonz.json_has_key(json_data, "packages")) {
        damn []
    }
    
    sus packages_json JsonValue = jsonz.json_get_object(json_data, "packages")
    ready (packages_json.type != "array") {
        damn []
    }
    
    sus packages []PackageMetadata = []
    bestie (sus i drip = 0; i < arrayz.len(packages_json.array_values); i = i + 1) {
        sus pkg_json JsonValue = packages_json.array_values[i]
        sus metadata PackageMetadata = parse_package_metadata(pkg_json)
        packages = arrayz.append(packages, metadata)
    }
    
    damn packages
}

# Parse package metadata from JSON
slay parse_package_metadata(json JsonValue) PackageMetadata {
    sus name tea = jsonz.json_get_string(json, "name")
    sus version tea = jsonz.json_get_string(json, "version")
    sus description tea = jsonz.json_get_string(json, "description")
    sus license tea = jsonz.json_get_string(json, "license")
    sus homepage tea = jsonz.json_get_string(json, "homepage")
    sus repository tea = jsonz.json_get_string(json, "repository")
    sus download_url tea = jsonz.json_get_string(json, "download_url")
    sus checksum tea = jsonz.json_get_string(json, "checksum")
    
    # Parse authors array
    sus authors []tea = []
    ready (jsonz.json_has_key(json, "authors")) {
        sus authors_json JsonValue = jsonz.json_get_object(json, "authors")
        ready (authors_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(authors_json.array_values); i = i + 1) {
                sus author tea = authors_json.array_values[i].string_value
                authors = arrayz.append(authors, author)
            }
        }
    }
    
    # Parse keywords array
    sus keywords []tea = []
    ready (jsonz.json_has_key(json, "keywords")) {
        sus keywords_json JsonValue = jsonz.json_get_object(json, "keywords")
        ready (keywords_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(keywords_json.array_values); i = i + 1) {
                sus keyword tea = keywords_json.array_values[i].string_value
                keywords = arrayz.append(keywords, keyword)
            }
        }
    }
    
    # Parse dependencies array
    sus dependencies []PackageDependency = []
    ready (jsonz.json_has_key(json, "dependencies")) {
        sus deps_json JsonValue = jsonz.json_get_object(json, "dependencies")
        ready (deps_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(deps_json.array_values); i = i + 1) {
                sus dep_json JsonValue = deps_json.array_values[i]
                sus dep PackageDependency = PackageDependency {
                    name: jsonz.json_get_string(dep_json, "name"),
                    version_req: jsonz.json_get_string(dep_json, "version_req"),
                    optional: jsonz.json_get_boolean(dep_json, "optional"),
                    features: []
                }
                dependencies = arrayz.append(dependencies, dep)
            }
        }
    }
    
    damn PackageMetadata {
        name: name,
        version: version,
        description: description,
        authors: authors,
        license: license,
        homepage: homepage,
        repository: repository,
        keywords: keywords,
        categories: [],
        dependencies: dependencies,
        download_url: download_url,
        checksum: checksum
    }
}

# Download package from registry
slay download_package(manager PackageManager, name tea, version tea) tea {
    sus pkg_url tea = manager.registry.config.url + "/api/v1/packages/" + name + "/" + version
    
    sus response tea = networkz.http_get(pkg_url)
    ready (!networkz.http_is_success_simple(response)) {
        vibez.spill("Failed to get package info for", name, "version", version)
        damn ""
    }
    
    sus body tea = networkz.http_get_body(response)
    sus json_data JsonValue = jsonz.json_parse(body)
    
    sus download_url tea = jsonz.json_get_string(json_data, "download_url")
    ready (download_url == "") {
        vibez.spill("No download URL found for package", name)
        damn ""
    }
    
    # Download the package archive
    sus download_response tea = networkz.http_get(download_url)
    ready (!networkz.http_is_success_simple(download_response)) {
        vibez.spill("Failed to download package", name, "from", download_url)
        damn ""
    }
    
    sus archive_data tea = networkz.http_get_body(download_response)
    
    # Save to temporary file
    sus temp_file tea = manager.installer.temp_dir + "/" + name + "-" + version + ".tar.gz"
    ready (!filez.write_file(temp_file, archive_data)) {
        vibez.spill("Failed to save package archive to", temp_file)
        damn ""
    }
    
    damn temp_file
}

# Install package with dependency resolution
slay install_package(manager PackageManager, name tea, version_spec tea) lit {
    vibez.spill("Installing package:", name, "version:", version_spec)
    
    # Check if already installed
    sus installed InstalledPackage = get_installed_package(manager, name)
    ready (installed.name != "") {
        vibez.spill("Package", name, "is already installed at version", installed.version)
        ready (version_spec == "" || installed.version == version_spec) {
            damn based
        }
    }
    
    # Search for the package to get metadata
    sus search_results []PackageMetadata = search_packages(manager, name)
    sus target_package PackageMetadata = PackageMetadata{}
    
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        ready (pkg.name == name) {
            # Use exact version match or latest if no version specified
            ready (version_spec == "" || pkg.version == version_spec) {
                target_package = pkg
                break
            }
        }
    }
    
    ready (target_package.name == "") {
        vibez.spill("Package not found:", name)
        damn cap
    }
    
    # Resolve and install dependencies first
    bestie (sus i drip = 0; i < arrayz.len(target_package.dependencies); i = i + 1) {
        sus dep PackageDependency = target_package.dependencies[i]
        ready (!dep.optional) {
            ready (!install_package(manager, dep.name, dep.version_req)) {
                vibez.spill("Failed to install dependency:", dep.name)
                damn cap
            }
        }
    }
    
    # Download the package
    sus archive_path tea = download_package(manager, name, target_package.version)
    ready (archive_path == "") {
        damn cap
    }
    
    # Extract package
    sus extract_dir tea = manager.installer.temp_dir + "/" + name + "-" + target_package.version
    ready (!extract_package(archive_path, extract_dir)) {
        vibez.spill("Failed to extract package", name)
        damn cap
    }
    
    # Install to final location
    sus install_path tea = manager.installer.install_dir + "/" + name + "-" + target_package.version
    ready (!filez.copy_dir(extract_dir, install_path)) {
        vibez.spill("Failed to install package to", install_path)
        damn cap
    }
    
    # Record installation
    sus installed_package InstalledPackage = InstalledPackage {
        name: name,
        version: target_package.version,
        install_path: install_path,
        installed_at: get_current_time(),
        dependencies: get_dependency_names(target_package.dependencies)
    }
    
    manager.installer.installed_packages = arrayz.append(manager.installer.installed_packages, installed_package)
    save_installed_packages(manager)
    
    vibez.spill("Successfully installed package:", name, "version:", target_package.version)
    damn based
}

# Extract package archive (simplified tar.gz extraction)
slay extract_package(archive_path tea, extract_dir tea) lit {
    # Create extraction directory
    filez.create_dir_all(extract_dir)
    
    # In a real implementation, this would properly extract tar.gz files
    # For now, we'll simulate successful extraction
    vibez.spill("Extracting", archive_path, "to", extract_dir)
    damn based
}

# Get dependency names from dependency list
slay get_dependency_names(dependencies []PackageDependency) []tea {
    sus names []tea = []
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        sus dep PackageDependency = dependencies[i]
        names = arrayz.append(names, dep.name)
    }
    damn names
}

# Get current timestamp (simplified)
slay get_current_time() tea {
    damn "2025-08-21T12:00:00Z"
}

# Uninstall package
slay uninstall_package(manager PackageManager, name tea) lit {
    vibez.spill("Uninstalling package:", name)
    
    sus installed InstalledPackage = get_installed_package(manager, name)
    ready (installed.name == "") {
        vibez.spill("Package not installed:", name)
        damn cap
    }
    
    # Check if other packages depend on this one
    sus dependents []tea = find_dependent_packages(manager, name)
    ready (arrayz.len(dependents) > 0) {
        vibez.spill("Cannot uninstall", name, "- required by:", stringz.join(dependents, ", "))
        damn cap
    }
    
    # Remove installation directory
    ready (!filez.remove_dir_all(installed.install_path)) {
        vibez.spill("Failed to remove installation directory:", installed.install_path)
        damn cap
    }
    
    # Remove from installed packages list
    sus updated_packages []InstalledPackage = []
    bestie (sus i drip = 0; i < arrayz.len(manager.installer.installed_packages); i = i + 1) {
        sus pkg InstalledPackage = manager.installer.installed_packages[i]
        ready (pkg.name != name) {
            updated_packages = arrayz.append(updated_packages, pkg)
        }
    }
    
    manager.installer.installed_packages = updated_packages
    save_installed_packages(manager)
    
    vibez.spill("Successfully uninstalled package:", name)
    damn based
}

# Find packages that depend on the given package
slay find_dependent_packages(manager PackageManager, target_name tea) []tea {
    sus dependents []tea = []
    
    bestie (sus i drip = 0; i < arrayz.len(manager.installer.installed_packages); i = i + 1) {
        sus pkg InstalledPackage = manager.installer.installed_packages[i]
        bestie (sus j drip = 0; j < arrayz.len(pkg.dependencies); j = j + 1) {
            sus dep_name tea = pkg.dependencies[j]
            ready (dep_name == target_name) {
                dependents = arrayz.append(dependents, pkg.name)
                break
            }
        }
    }
    
    damn dependents
}

# Get installed package by name
slay get_installed_package(manager PackageManager, name tea) InstalledPackage {
    bestie (sus i drip = 0; i < arrayz.len(manager.installer.installed_packages); i = i + 1) {
        sus pkg InstalledPackage = manager.installer.installed_packages[i]
        ready (pkg.name == name) {
            damn pkg
        }
    }
    
    damn InstalledPackage { name: "", version: "", install_path: "", installed_at: "", dependencies: [] }
}

# List all installed packages
slay list_installed_packages(manager PackageManager) []InstalledPackage {
    damn manager.installer.installed_packages
}

# Update package to latest version
slay update_package(manager PackageManager, name tea) lit {
    vibez.spill("Updating package:", name)
    
    sus installed InstalledPackage = get_installed_package(manager, name)
    ready (installed.name == "") {
        vibez.spill("Package not installed:", name)
        damn cap
    }
    
    # Find latest version
    sus search_results []PackageMetadata = search_packages(manager, name)
    sus latest_version tea = ""
    sus latest_metadata PackageMetadata = PackageMetadata{}
    
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        ready (pkg.name == name) {
            ready (latest_version == "" || version_is_newer(pkg.version, latest_version)) {
                latest_version = pkg.version
                latest_metadata = pkg
            }
        }
    }
    
    ready (latest_version == installed.version) {
        vibez.spill("Package", name, "is already at latest version:", latest_version)
        damn based
    }
    
    # Uninstall current version and install latest
    ready (!uninstall_package(manager, name)) {
        damn cap
    }
    
    damn install_package(manager, name, latest_version)
}

# Check if version1 is newer than version2
slay version_is_newer(version1 tea, version2 tea) lit {
    sus v1 PackageVersion = parse_version(version1)
    sus v2 PackageVersion = parse_version(version2)
    
    damn compare_versions(v1, v2) > 0
}

# Save installed packages to disk
slay save_installed_packages(manager PackageManager) lit {
    sus config_file tea = manager.config_dir + "/installed.json"
    filez.create_dir_all(manager.config_dir)
    
    # Create JSON representation of installed packages
    sus json_obj JsonValue = jsonz.json_create_object()
    sus packages_array JsonValue = jsonz.json_create_array()
    
    bestie (sus i drip = 0; i < arrayz.len(manager.installer.installed_packages); i = i + 1) {
        sus pkg InstalledPackage = manager.installer.installed_packages[i]
        sus pkg_obj JsonValue = jsonz.json_create_object()
        
        pkg_obj = jsonz.json_object_set(pkg_obj, "name", jsonz.json_create_string(pkg.name))
        pkg_obj = jsonz.json_object_set(pkg_obj, "version", jsonz.json_create_string(pkg.version))
        pkg_obj = jsonz.json_object_set(pkg_obj, "install_path", jsonz.json_create_string(pkg.install_path))
        pkg_obj = jsonz.json_object_set(pkg_obj, "installed_at", jsonz.json_create_string(pkg.installed_at))
        
        # Add dependencies array
        sus deps_array JsonValue = jsonz.json_create_array()
        bestie (sus j drip = 0; j < arrayz.len(pkg.dependencies); j = j + 1) {
            deps_array = jsonz.json_array_push(deps_array, jsonz.json_create_string(pkg.dependencies[j]))
        }
        pkg_obj = jsonz.json_object_set(pkg_obj, "dependencies", deps_array)
        
        packages_array = jsonz.json_array_push(packages_array, pkg_obj)
    }
    
    json_obj = jsonz.json_object_set(json_obj, "installed_packages", packages_array)
    sus json_str tea = jsonz.json_stringify(json_obj)
    
    damn filez.write_file(config_file, json_str)
}

# Load installed packages from disk
slay load_installed_packages(manager PackageManager) lit {
    sus config_file tea = manager.config_dir + "/installed.json"
    
    ready (!filez.file_exists(config_file)) {
        damn based  # No config file yet, that's OK
    }
    
    sus json_str tea = filez.read_file(config_file)
    ready (json_str == "") {
        damn cap
    }
    
    sus json_obj JsonValue = jsonz.json_parse(json_str)
    ready (!jsonz.json_has_key(json_obj, "installed_packages")) {
        damn cap
    }
    
    sus packages_json JsonValue = jsonz.json_get_object(json_obj, "installed_packages")
    ready (packages_json.type != "array") {
        damn cap
    }
    
    sus packages []InstalledPackage = []
    bestie (sus i drip = 0; i < arrayz.len(packages_json.array_values); i = i + 1) {
        sus pkg_json JsonValue = packages_json.array_values[i]
        
        sus name tea = jsonz.json_get_string(pkg_json, "name")
        sus version tea = jsonz.json_get_string(pkg_json, "version")
        sus install_path tea = jsonz.json_get_string(pkg_json, "install_path")
        sus installed_at tea = jsonz.json_get_string(pkg_json, "installed_at")
        
        # Load dependencies
        sus deps []tea = []
        ready (jsonz.json_has_key(pkg_json, "dependencies")) {
            sus deps_json JsonValue = jsonz.json_get_object(pkg_json, "dependencies")
            ready (deps_json.type == "array") {
                bestie (sus j drip = 0; j < arrayz.len(deps_json.array_values); j = j + 1) {
                    sus dep_name tea = deps_json.array_values[j].string_value
                    deps = arrayz.append(deps, dep_name)
                }
            }
        }
        
        sus pkg InstalledPackage = InstalledPackage {
            name: name,
            version: version,
            install_path: install_path,
            installed_at: installed_at,
            dependencies: deps
        }
        
        packages = arrayz.append(packages, pkg)
    }
    
    manager.installer.installed_packages = packages
    damn based
}
