# Package Manager Module for CURSED
# Provides comprehensive package installation, dependency resolution, and registry management
yeet "filez"
yeet "networkz"
yeet "jsonz"
yeet "arrayz"
yeet "stringz"
yeet "vibez"
yeet "timez"
yeet "cryptz"
yeet "registry"  # Enhanced registry functionality
yeet "resolver"  # Dependency resolution engine
yeet "lockfile"  # Lock file management
yeet "http_client"  # Real HTTP client implementation
yeet "archive_handler"  # Real archive handling
yeet "dependency_resolver_real"  # Advanced dependency resolution
yeet "security_verification"  # Package security verification
yeet "toml_parser_production"  # Full TOML specification parser
yeet "checksum_algorithms"  # Comprehensive checksum algorithms

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
    sus authors tea[value]
    sus license tea
    sus homepage tea
    sus repository tea
    sus keywords tea[value]
    sus categories tea[value]
    sus dependencies PackageDependency[value]
    sus download_url tea
    sus checksum tea
}

# Package dependency specification
squad PackageDependency {
    sus name tea
    sus version_req tea
    sus optional lit
    sus features tea[value]
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
    sus dependencies tea[value]
}

# Package registry client
squad PackageRegistry {
    sus config RegistryConfig
}

# Package installer
squad PackageInstaller {
    sus install_dir tea
    sus temp_dir tea
    sus installed_packages InstalledPackage[value]
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
    sus parts tea[value] = stringz.split(version_str, ".")
    
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

# Search for packages in registry with real HTTP client
slay search_packages(manager PackageManager, query tea) PackageMetadata[value]{
    sus search_url tea = manager.registry.config.url + "/api/v1/packages/search"
    
    # Build query parameters
    sus params map<tea, tea> = {}
    params["q"] = query
    sus query_string tea = build_query_string(params)
    sus full_url tea = search_url + query_string
    
    # Create HTTP request with proper headers
    sus request HttpRequest = create_http_request("GET", full_url)
    request = add_user_agent(request, "cursed-pkg/1.0.0")
    request = add_header(request, "Accept", "application/json")
    
    # Add authentication if configured
    ready (manager.registry.config.api_key != "") {
        request = add_auth_bearer(request, manager.registry.config.api_key)
    }
    
    sus response HttpResponse = execute_http_request(request)
    ready (!is_http_success(response)) {
        vibez.spill("Failed to search packages:", response.status_code, response.status_text)
        damn []
    }
    
    sus json_data JsonValue = jsonz.json_parse(response.body)
    
    ready (!jsonz.json_has_key(json_data, "packages")) {
        damn []
    }
    
    sus packages_json JsonValue = jsonz.json_get_object(json_data, "packages")
    ready (packages_json.type != "array") {
        damn []
    }
    
    sus packages PackageMetadata[value] = []
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
    sus authors tea[value] = []
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
    sus keywords tea[value] = []
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
    sus dependencies PackageDependency[value] = []
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

# Download package from registry with security verification
slay download_package(manager PackageManager, name tea, version tea) tea {
    sus pkg_url tea = manager.registry.config.url + "/api/v1/packages/" + name + "/" + version
    
    # Get package metadata with real HTTP client
    sus request HttpRequest = create_http_request("GET", pkg_url)
    request = add_user_agent(request, "cursed-pkg/1.0.0")
    request = add_header(request, "Accept", "application/json")
    
    ready (manager.registry.config.api_key != "") {
        request = add_auth_bearer(request, manager.registry.config.api_key)
    }
    
    sus response HttpResponse = execute_http_request(request)
    ready (!is_http_success(response)) {
        vibez.spill("Failed to get package info for", name, "version", version, 
                   "Status:", response.status_code)
        damn ""
    }
    
    sus json_data JsonValue = jsonz.json_parse(response.body)
    sus download_url tea = jsonz.json_get_string(json_data, "download_url")
    sus expected_checksum tea = jsonz.json_get_string(json_data, "checksum")
    
    ready (download_url == "") {
        vibez.spill("No download URL found for package", name)
        damn ""
    }
    
    # Download the package archive with progress tracking
    sus temp_file tea = manager.installer.temp_dir + "/" + name + "-" + version + ".tar.gz"
    
    sus download_success lit = download_file(download_url, temp_file, null)
    ready (!download_success) {
        vibez.spill("Failed to download package", name, "from", download_url)
        damn ""
    }
    
    # Verify checksum with production algorithms if provided
    ready (expected_checksum != "") {
        sus archive_data tea = filez.read_file_binary(temp_file)
        sus algorithm ChecksumAlgorithm = detect_checksum_algorithm(expected_checksum)
        sus computed_result ChecksumResult = compute_checksum(archive_data, algorithm)
        
        ready (!compare_checksums(computed_result.hex_digest, expected_checksum)) {
            vibez.spill("Checksum verification failed for package", name)
            vibez.spill("Expected:", expected_checksum)
            vibez.spill("Computed:", computed_result.hex_digest)
            vibez.spill("Algorithm:", get_algorithm_name(algorithm))
            filez.remove_file(temp_file)
            damn ""
        }
        vibez.spill("Checksum verified for package", name, "using", get_algorithm_name(algorithm))
    }
    
    damn temp_file
}

# Install package with advanced dependency resolution
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
    sus search_results PackageMetadata[value] = search_packages(manager, name)
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
    
    # Use advanced dependency resolver
    sus resolver DependencyResolver = init_dependency_resolver(manager.registry)
    sus root_packages tea[value] = [name]
    sus resolution_result ResolutionResult = resolve_dependencies_advanced(resolver, root_packages)
    
    ready (!resolution_result.success) {
        vibez.spill("Dependency resolution failed for package:", name)
        bestie (sus i drip = 0; i < arrayz.len(resolution_result.conflicts); i = i + 1) {
            sus conflict ResolutionConflict = resolution_result.conflicts[i]
            vibez.spill("Conflict:", conflict.package_name, "required versions:", 
                       stringz.join(conflict.conflicting_versions, ", "))
        }
        damn cap
    }
    
    vibez.spill("Resolved", arrayz.len(resolution_result.resolved_packages), "packages in", 
               resolution_result.resolution_time, "ms")
    
    # Install resolved packages in dependency order
    bestie (sus i drip = 0; i < arrayz.len(resolution_result.resolved_packages); i = i + 1) {
        sus pkg PackageMetadata = resolution_result.resolved_packages[i]
        ready (!install_single_package(manager, pkg)) {
            vibez.spill("Failed to install resolved package:", pkg.name)
            damn cap
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

# Extract package archive with real tar.gz handling
slay extract_package(archive_path tea, extract_dir tea) lit {
    # Create extraction options
    sus extraction_options ExtractionOptions = ExtractionOptions {
        destination_dir: extract_dir,
        preserve_permissions: based,
        overwrite_existing: based,
        verify_checksums: cap,  # Checksums already verified during download
        max_extract_size: 100 * 1024 * 1024  # 100MB limit
    }
    
    # Use real archive handler for extraction
    ready (!extract_package_archive(archive_path, extraction_options)) {
        vibez.spill("Failed to extract archive:", archive_path)
        damn cap
    }
    
    vibez.spill("Successfully extracted", archive_path, "to", extract_dir)
    damn based
}

# Get dependency names from dependency list
slay get_dependency_names(dependencies PackageDependency[value]) tea[value]{
    sus names tea[value] = []
    bestie (sus i drip = 0; i < arrayz.len(dependencies); i = i + 1) {
        sus dep PackageDependency = dependencies[i]
        names = arrayz.append(names, dep.name)
    }
    damn names
}

# Get current timestamp (real implementation)
slay get_current_time() tea {
    damn timez.format_iso8601(timez.current_time())
}

# Install a single resolved package (helper for dependency resolution)
slay install_single_package(manager PackageManager, pkg PackageMetadata) lit {
    vibez.spill("Installing resolved package:", pkg.name, "version:", pkg.version)
    
    # Check if already installed at correct version
    sus installed InstalledPackage = get_installed_package(manager, pkg.name)
    ready (installed.name != "" && installed.version == pkg.version) {
        vibez.spill("Package", pkg.name, "already installed at correct version")
        damn based
    }
    
    # Download the package
    sus archive_path tea = download_package(manager, pkg.name, pkg.version)
    ready (archive_path == "") {
        damn cap
    }
    
    # Verify package integrity and security
    sus security_policy SecurityPolicy = create_default_security_policy()
    sus verification_result VerificationResult = verify_package_integrity(archive_path, pkg, security_policy)
    
    ready (!verification_result.is_valid) {
        vibez.spill("Security verification failed for package:", pkg.name)
        vibez.spill("Error:", verification_result.error_message)
        bestie (sus i drip = 0; i < arrayz.len(verification_result.verification_details); i = i + 1) {
            vibez.spill("  ", verification_result.verification_details[i])
        }
        filez.remove_file(archive_path)
        damn cap
    }
    
    vibez.spill("Security verification passed for:", pkg.name, "trust level:", verification_result.trust_level)
    
    # Extract package
    sus extract_dir tea = manager.installer.temp_dir + "/" + pkg.name + "-" + pkg.version
    ready (!extract_package(archive_path, extract_dir)) {
        vibez.spill("Failed to extract package", pkg.name)
        filez.remove_file(archive_path)
        damn cap
    }
    
    # Install to final location
    sus install_path tea = manager.installer.install_dir + "/" + pkg.name + "-" + pkg.version
    ready (!filez.copy_dir(extract_dir, install_path)) {
        vibez.spill("Failed to install package to", install_path)
        filez.remove_file(archive_path)
        filez.remove_dir_all(extract_dir)
        damn cap
    }
    
    # Record installation
    sus installed_package InstalledPackage = InstalledPackage {
        name: pkg.name,
        version: pkg.version,
        install_path: install_path,
        installed_at: get_current_time(),
        dependencies: get_dependency_names(pkg.dependencies)
    }
    
    # Update or add to installed packages list
    sus updated_packages InstalledPackage[value] = []
    sus found lit = cap
    
    bestie (sus i drip = 0; i < arrayz.len(manager.installer.installed_packages); i = i + 1) {
        sus existing InstalledPackage = manager.installer.installed_packages[i]
        ready (existing.name == pkg.name) {
            # Replace existing installation
            updated_packages = arrayz.append(updated_packages, installed_package)
            found = based
        } otherwise {
            updated_packages = arrayz.append(updated_packages, existing)
        }
    }
    
    ready (!found) {
        updated_packages = arrayz.append(updated_packages, installed_package)
    }
    
    manager.installer.installed_packages = updated_packages
    save_installed_packages(manager)
    
    # Clean up temporary files
    filez.remove_file(archive_path)
    filez.remove_dir_all(extract_dir)
    
    vibez.spill("Successfully installed package:", pkg.name, "version:", pkg.version)
    damn based
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
    sus dependents tea[value] = find_dependent_packages(manager, name)
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
    sus updated_packages InstalledPackage[value] = []
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
slay find_dependent_packages(manager PackageManager, target_name tea) tea[value]{
    sus dependents tea[value] = []
    
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
slay list_installed_packages(manager PackageManager) InstalledPackage[value]{
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
    sus search_results PackageMetadata[value] = search_packages(manager, name)
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
    
    sus packages InstalledPackage[value] = []
    bestie (sus i drip = 0; i < arrayz.len(packages_json.array_values); i = i + 1) {
        sus pkg_json JsonValue = packages_json.array_values[i]
        
        sus name tea = jsonz.json_get_string(pkg_json, "name")
        sus version tea = jsonz.json_get_string(pkg_json, "version")
        sus install_path tea = jsonz.json_get_string(pkg_json, "install_path")
        sus installed_at tea = jsonz.json_get_string(pkg_json, "installed_at")
        
        # Load dependencies
        sus deps tea[value] = []
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
