# Package Publishing Utility for CURSED Package Manager
# Handles package creation, validation, signing, and publishing to registry

yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "jsonz"
yeet "timez"
yeet "cryptz"
yeet "http_client"
yeet "archive_handler"
yeet "security_verification"

# Package publishing configuration
squad PublishingConfig {
    sus registry_url tea
    sus api_key tea
    sus signing_key_path tea
    sus publisher_email tea
    sus publisher_name tea
    sus verify_before_publish lit
    sus dry_run lit
}

# Package manifest structure (cursed.toml equivalent)
squad PackageManifest {
    sus name tea
    sus version tea
    sus description tea
    sus authors tea[value]
    sus license tea
    sus homepage tea
    sus repository tea
    sus readme tea
    sus keywords tea[value]
    sus categories tea[value]
    sus dependencies PackageDependency[value]
    sus dev_dependencies PackageDependency[value]
    sus build_dependencies PackageDependency[value]
    sus exclude_patterns tea[value]
    sus include_patterns tea[value]
    sus minimum_cursed_version tea
}

# Publishing result information
squad PublishingResult {
    sus success lit
    sus package_url tea
    sus published_version tea
    sus upload_size drip
    sus upload_time_ms drip
    sus verification_details tea[value]
    sus error_message tea
    sus warnings tea[value]
}

# Package validation result
squad ValidationResult {
    sus is_valid lit
    sus errors tea[value]
    sus warnings tea[value]
    sus package_size drip
    sus file_count drip
    sus has_readme lit
    sus has_license lit
    sus has_tests lit
}

# Initialize publishing configuration
slay create_publishing_config(registry_url tea, api_key tea) PublishingConfig {
    damn PublishingConfig {
        registry_url: registry_url,
        api_key: api_key,
        signing_key_path: "",
        publisher_email: "",
        publisher_name: "",
        verify_before_publish: based,
        dry_run: cap
    }
}

# Load package manifest from cursed.toml file
slay load_package_manifest(manifest_path tea) PackageManifest {
    ready (!filez.file_exists(manifest_path)) {
        vibez.spill("Package manifest not found:", manifest_path)
        damn PackageManifest { name: "" }
    }
    
    sus manifest_content tea = filez.read_file(manifest_path)
    ready (manifest_content == "") {
        vibez.spill("Failed to read manifest file:", manifest_path)
        damn PackageManifest { name: "" }
    }
    
    # Parse TOML-like format (simplified implementation)
    sus manifest PackageManifest = parse_package_manifest_toml(manifest_content)
    
    ready (manifest.name == "") {
        vibez.spill("Invalid or incomplete package manifest")
        damn PackageManifest { name: "" }
    }
    
    vibez.spill("Loaded package manifest:", manifest.name, "version:", manifest.version)
    damn manifest
}

# Parse TOML-like package manifest
slay parse_package_manifest_toml(content tea) PackageManifest {
    sus lines tea[value] = stringz.split(content, "\n")
    sus manifest PackageManifest = PackageManifest {
        name: "",
        version: "",
        description: "",
        authors: [],
        license: "",
        homepage: "",
        repository: "",
        readme: "README.md",
        keywords: [],
        categories: [],
        dependencies: [],
        dev_dependencies: [],
        build_dependencies: [],
        exclude_patterns: [],
        include_patterns: [],
        minimum_cursed_version: "1.0.0"
    }
    
    sus current_section tea = ""
    
    bestie (sus i drip = 0; i < arrayz.len(lines); i = i + 1) {
        sus line tea = stringz.trim(lines[i])
        
        # Skip empty lines and comments
        ready (line == "" || stringz.starts_with(line, "#")) {
            continue
        }
        
        # Section headers
        ready (stringz.starts_with(line, "[") && stringz.ends_with(line, "]")) {
            current_section = stringz.substring(line, 1, stringz.len(line) - 1)
            continue
        }
        
        # Key-value pairs
        sus eq_pos drip = stringz.index_of(line, "=")
        ready (eq_pos == -1) {
            continue
        }
        
        sus key tea = stringz.trim(stringz.substring(line, 0, eq_pos))
        sus value tea = stringz.trim(stringz.substring(line, eq_pos + 1, stringz.len(line)))
        
        # Remove quotes
        ready (stringz.starts_with(value, "\"") && stringz.ends_with(value, "\"")) {
            value = stringz.substring(value, 1, stringz.len(value) - 1)
        }
        
        # Parse based on current section and key
        ready (current_section == "" || current_section == "package") {
            match key {
                "name" -> manifest.name = value
                "version" -> manifest.version = value
                "description" -> manifest.description = value
                "license" -> manifest.license = value
                "homepage" -> manifest.homepage = value
                "repository" -> manifest.repository = value
                "readme" -> manifest.readme = value
                "authors" -> manifest.authors = parse_string_array_toml(value)
                "keywords" -> manifest.keywords = parse_string_array_toml(value)
                "categories" -> manifest.categories = parse_string_array_toml(value)
                _ -> {
                    # Skip unknown keys
                }
            }
        }
    }
    
    damn manifest
}

# Parse TOML array of strings
slay parse_string_array_toml(value tea) tea[value]{
    # Simple implementation for ["item1", "item2", "item3"] format
    ready (!stringz.starts_with(value, "[") || !stringz.ends_with(value, "]")) {
        damn [value]  # Single value
    }
    
    sus inner tea = stringz.substring(value, 1, stringz.len(value) - 1)
    sus parts tea[value] = stringz.split(inner, ",")
    sus result tea[value] = []
    
    bestie (sus i drip = 0; i < arrayz.len(parts); i = i + 1) {
        sus part tea = stringz.trim(parts[i])
        ready (stringz.starts_with(part, "\"") && stringz.ends_with(part, "\"")) {
            part = stringz.substring(part, 1, stringz.len(part) - 1)
        }
        ready (part != "") {
            result = arrayz.append(result, part)
        }
    }
    
    damn result
}

# Validate package before publishing
slay validate_package_for_publishing(package_dir tea, manifest PackageManifest) ValidationResult {
    vibez.spill("Validating package for publishing...")
    
    sus result ValidationResult = ValidationResult {
        is_valid: based,
        errors: [],
        warnings: [],
        package_size: 0,
        file_count: 0,
        has_readme: cap,
        has_license: cap,
        has_tests: cap
    }
    
    # Check if package directory exists
    ready (!filez.dir_exists(package_dir)) {
        result.errors = arrayz.append(result.errors, "Package directory not found: " + package_dir)
        result.is_valid = cap
        damn result
    }
    
    # Validate manifest fields
    ready (manifest.name == "") {
        result.errors = arrayz.append(result.errors, "Package name is required")
        result.is_valid = cap
    }
    
    ready (manifest.version == "") {
        result.errors = arrayz.append(result.errors, "Package version is required")
        result.is_valid = cap
    }
    
    ready (manifest.description == "") {
        result.warnings = arrayz.append(result.warnings, "Package description is missing")
    }
    
    ready (arrayz.len(manifest.authors) == 0) {
        result.warnings = arrayz.append(result.warnings, "Package authors list is empty")
    }
    
    ready (manifest.license == "") {
        result.warnings = arrayz.append(result.warnings, "Package license is not specified")
    }
    
    # Validate version format
    ready (!is_valid_semver(manifest.version)) {
        result.errors = arrayz.append(result.errors, "Invalid semantic version: " + manifest.version)
        result.is_valid = cap
    }
    
    # Validate package name format
    ready (!is_valid_package_name(manifest.name)) {
        result.errors = arrayz.append(result.errors, "Invalid package name: " + manifest.name)
        result.is_valid = cap
    }
    
    # Check for essential files
    sus readme_path tea = package_dir + "/" + manifest.readme
    ready (filez.file_exists(readme_path)) {
        result.has_readme = based
    } otherwise {
        result.warnings = arrayz.append(result.warnings, "README file not found: " + manifest.readme)
    }
    
    sus license_path tea = package_dir + "/LICENSE"
    sus license_txt_path tea = package_dir + "/LICENSE.txt"
    ready (filez.file_exists(license_path) || filez.file_exists(license_txt_path)) {
        result.has_license = based
    } otherwise {
        result.warnings = arrayz.append(result.warnings, "LICENSE file not found")
    }
    
    # Check for tests
    sus tests_dir tea = package_dir + "/tests"
    sus test_files tea[value] = filez.list_files_pattern(package_dir, "**/*test*.csd")
    ready (filez.dir_exists(tests_dir) || arrayz.len(test_files) > 0) {
        result.has_tests = based
    } otherwise {
        result.warnings = arrayz.append(result.warnings, "No test files found")
    }
    
    # Calculate package statistics
    sus all_files tea[value] = filez.list_files_recursive(package_dir)
    result.file_count = arrayz.len(all_files)
    
    bestie (sus i drip = 0; i < arrayz.len(all_files); i = i + 1) {
        sus file_path tea = all_files[i]
        result.package_size = result.package_size + filez.file_size(file_path)
    }
    
    # Size warnings
    ready (result.package_size > 50 * 1024 * 1024) {  # 50MB
        result.warnings = arrayz.append(result.warnings, 
            "Package is quite large: " + stringz.from_int(result.package_size / (1024 * 1024)) + "MB")
    }
    
    ready (result.file_count > 1000) {
        result.warnings = arrayz.append(result.warnings, 
            "Package contains many files: " + stringz.from_int(result.file_count))
    }
    
    vibez.spill("Package validation completed:")
    vibez.spill("  Files:", result.file_count)
    vibez.spill("  Size:", result.package_size / 1024, "KB")
    vibez.spill("  Has README:", result.has_readme)
    vibez.spill("  Has LICENSE:", result.has_license)
    vibez.spill("  Has tests:", result.has_tests)
    vibez.spill("  Errors:", arrayz.len(result.errors))
    vibez.spill("  Warnings:", arrayz.len(result.warnings))
    
    damn result
}

# Create and publish package to registry
slay publish_package_to_registry(package_dir tea, manifest PackageManifest, config PublishingConfig) PublishingResult {
    vibez.spill("Publishing package:", manifest.name, "version:", manifest.version)
    
    sus start_time drip = timez.current_time_ms()
    sus result PublishingResult = PublishingResult {
        success: cap,
        package_url: "",
        published_version: manifest.version,
        upload_size: 0,
        upload_time_ms: 0,
        verification_details: [],
        error_message: "",
        warnings: []
    }
    
    # Validate package first
    sus validation ValidationResult = validate_package_for_publishing(package_dir, manifest)
    ready (!validation.is_valid) {
        result.error_message = "Package validation failed: " + stringz.join(validation.errors, "; ")
        damn result
    }
    
    # Add validation warnings to result
    result.warnings = validation.warnings
    
    # Check if dry run
    ready (config.dry_run) {
        vibez.spill("DRY RUN: Would publish package but dry_run is enabled")
        result.success = based
        result.verification_details = arrayz.append(result.verification_details, "Dry run completed successfully")
        damn result
    }
    
    # Create package archive
    sus temp_dir tea = "/tmp/cursed-publish-" + stringz.random_string(8)
    filez.create_dir_all(temp_dir)
    
    sus archive_path tea = temp_dir + "/" + manifest.name + "-" + manifest.version + ".tar.gz"
    
    sus archive_options ArchiveOptions = ArchiveOptions {
        format: ArchiveFormat.TarGz,
        compression_level: 6,
        include_hidden: cap,
        exclude_patterns: get_exclude_patterns(manifest),
        preserve_permissions: based
    }
    
    ready (!create_package_archive(package_dir, archive_path, archive_options)) {
        result.error_message = "Failed to create package archive"
        filez.remove_dir_all(temp_dir)
        damn result
    }
    
    result.upload_size = filez.file_size(archive_path)
    
    # Generate integrity metadata
    sus publisher_info PublisherInfo = PublisherInfo {
        name: config.publisher_name,
        email: config.publisher_email,
        public_key: "",
        certificate: "",
        is_verified: cap,
        trust_score: 50
    }
    
    sus integrity IntegrityMetadata = generate_integrity_metadata(archive_path, publisher_info)
    
    # Create package metadata for registry
    sus package_metadata PackageMetadata = convert_manifest_to_metadata(manifest, integrity)
    
    # Verify package integrity before upload
    ready (config.verify_before_publish) {
        sus security_policy SecurityPolicy = create_default_security_policy()
        sus verification VerificationResult = verify_package_integrity(archive_path, package_metadata, security_policy)
        
        ready (!verification.is_valid) {
            result.error_message = "Pre-publish verification failed: " + verification.error_message
            filez.remove_dir_all(temp_dir)
            damn result
        }
        
        result.verification_details = verification.verification_details
    }
    
    # Check if package already exists
    ready (package_exists_in_registry(config, manifest.name, manifest.version)) {
        result.error_message = "Package " + manifest.name + " version " + manifest.version + " already exists"
        filez.remove_dir_all(temp_dir)
        damn result
    }
    
    # Publish to registry
    ready (!upload_package_to_registry(config, package_metadata, archive_path, result)) {
        filez.remove_dir_all(temp_dir)
        damn result
    }
    
    # Clean up
    filez.remove_dir_all(temp_dir)
    
    sus end_time drip = timez.current_time_ms()
    result.upload_time_ms = end_time - start_time
    result.success = based
    result.package_url = config.registry_url + "/packages/" + manifest.name + "/" + manifest.version
    
    vibez.spill("Successfully published package:", manifest.name, "version:", manifest.version)
    vibez.spill("Package URL:", result.package_url)
    vibez.spill("Upload time:", result.upload_time_ms, "ms")
    vibez.spill("Package size:", result.upload_size / 1024, "KB")
    
    damn result
}

# Upload package to registry via HTTP API
slay upload_package_to_registry(config PublishingConfig, metadata PackageMetadata, archive_path tea, result PublishingResult) lit {
    # Step 1: Upload package metadata
    sus metadata_json tea = create_package_metadata_json(metadata)
    
    sus metadata_request HttpRequest = create_http_request("POST", config.registry_url + "/api/v1/packages")
    metadata_request = add_auth_bearer(metadata_request, config.api_key)
    metadata_request = add_user_agent(metadata_request, "cursed-pkg/1.0.0")
    metadata_request = set_json_body(metadata_request, metadata_json)
    
    sus metadata_response HttpResponse = execute_http_request(metadata_request)
    
    ready (!is_http_success(metadata_response)) {
        result.error_message = "Failed to upload package metadata: " + stringz.from_int(metadata_response.status_code) + " " + metadata_response.status_text
        damn cap
    }
    
    # Step 2: Upload package archive
    sus archive_data tea = filez.read_file_binary(archive_path)
    sus upload_url tea = config.registry_url + "/api/v1/packages/" + metadata.name + "/" + metadata.version + "/upload"
    
    sus upload_request HttpRequest = create_http_request("PUT", upload_url)
    upload_request = add_auth_bearer(upload_request, config.api_key)
    upload_request = add_user_agent(upload_request, "cursed-pkg/1.0.0")
    upload_request = set_binary_body(upload_request, archive_data, "application/gzip")
    
    sus upload_response HttpResponse = execute_http_request(upload_request)
    
    ready (!is_http_success(upload_response)) {
        result.error_message = "Failed to upload package archive: " + stringz.from_int(upload_response.status_code) + " " + upload_response.status_text
        damn cap
    }
    
    damn based
}

# Helper functions
slay get_exclude_patterns(manifest PackageManifest) tea[value]{
    sus patterns tea[value] = [
        "*.log",
        "*.tmp", 
        ".git/*",
        ".cursed-cache/*",
        "target/*",
        "node_modules/*",
        ".DS_Store",
        "Thumbs.db"
    ]
    
    # Add manifest-specified exclusions
    bestie (sus i drip = 0; i < arrayz.len(manifest.exclude_patterns); i = i + 1) {
        patterns = arrayz.append(patterns, manifest.exclude_patterns[i])
    }
    
    damn patterns
}

slay convert_manifest_to_metadata(manifest PackageManifest, integrity IntegrityMetadata) PackageMetadata {
    damn PackageMetadata {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        authors: manifest.authors,
        license: manifest.license,
        homepage: manifest.homepage,
        repository: manifest.repository,
        keywords: manifest.keywords,
        categories: manifest.categories,
        dependencies: manifest.dependencies,
        download_url: "",  # Will be set by registry
        checksum: integrity.sha256_checksum
    }
}

slay create_package_metadata_json(metadata PackageMetadata) tea {
    sus json_obj JsonValue = jsonz.json_create_object()
    
    json_obj = jsonz.json_object_set(json_obj, "name", jsonz.json_create_string(metadata.name))
    json_obj = jsonz.json_object_set(json_obj, "version", jsonz.json_create_string(metadata.version))
    json_obj = jsonz.json_object_set(json_obj, "description", jsonz.json_create_string(metadata.description))
    json_obj = jsonz.json_object_set(json_obj, "license", jsonz.json_create_string(metadata.license))
    json_obj = jsonz.json_object_set(json_obj, "homepage", jsonz.json_create_string(metadata.homepage))
    json_obj = jsonz.json_object_set(json_obj, "repository", jsonz.json_create_string(metadata.repository))
    json_obj = jsonz.json_object_set(json_obj, "checksum", jsonz.json_create_string(metadata.checksum))
    
    # Add authors array
    sus authors_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(metadata.authors); i = i + 1) {
        authors_array = jsonz.json_array_push(authors_array, jsonz.json_create_string(metadata.authors[i]))
    }
    json_obj = jsonz.json_object_set(json_obj, "authors", authors_array)
    
    # Add keywords array
    sus keywords_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(metadata.keywords); i = i + 1) {
        keywords_array = jsonz.json_array_push(keywords_array, jsonz.json_create_string(metadata.keywords[i]))
    }
    json_obj = jsonz.json_object_set(json_obj, "keywords", keywords_array)
    
    # Add dependencies array
    sus deps_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(metadata.dependencies); i = i + 1) {
        sus dep PackageDependency = metadata.dependencies[i]
        sus dep_obj JsonValue = jsonz.json_create_object()
        dep_obj = jsonz.json_object_set(dep_obj, "name", jsonz.json_create_string(dep.name))
        dep_obj = jsonz.json_object_set(dep_obj, "version_req", jsonz.json_create_string(dep.version_req))
        dep_obj = jsonz.json_object_set(dep_obj, "optional", jsonz.json_create_boolean(dep.optional))
        deps_array = jsonz.json_array_push(deps_array, dep_obj)
    }
    json_obj = jsonz.json_object_set(json_obj, "dependencies", deps_array)
    
    damn jsonz.json_stringify(json_obj)
}

slay package_exists_in_registry(config PublishingConfig, name tea, version tea) lit {
    sus check_url tea = config.registry_url + "/api/v1/packages/" + name + "/" + version
    
    sus request HttpRequest = create_http_request("HEAD", check_url)
    request = add_user_agent(request, "cursed-pkg/1.0.0")
    
    sus response HttpResponse = execute_http_request(request)
    
    # Package exists if we get a 200 response
    damn response.status_code == 200
}

slay is_valid_semver(version tea) lit {
    sus parts tea[value] = stringz.split(version, ".")
    ready (arrayz.len(parts) != 3) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        ready (!stringz.is_numeric(parts[i])) {
            damn cap
        }
    }
    
    damn based
}

slay is_valid_package_name(name tea) lit {
    ready (stringz.len(name) < 1 || stringz.len(name) > 64) {
        damn cap
    }
    
    # Must start with letter
    sus first_char tea = stringz.char_at(name, 0)
    ready (!stringz.is_alpha(first_char)) {
        damn cap
    }
    
    # Check all characters
    bestie (sus i drip = 0; i < stringz.len(name); i = i + 1) {
        sus ch tea = stringz.char_at(name, i)
        ready (!stringz.is_alphanumeric(ch) && ch != "-" && ch != "_") {
            damn cap
        }
    }
    
    damn based
}

# CLI-like interface for publishing
slay publish_package_from_directory(package_dir tea, registry_url tea, api_key tea) PublishingResult {
    vibez.spill("Publishing package from directory:", package_dir)
    
    # Load manifest
    sus manifest_path tea = package_dir + "/cursed.toml"
    sus manifest PackageManifest = load_package_manifest(manifest_path)
    ready (manifest.name == "") {
        damn PublishingResult {
            success: cap,
            error_message: "Failed to load package manifest from " + manifest_path
        }
    }
    
    # Create publishing config
    sus config PublishingConfig = create_publishing_config(registry_url, api_key)
    config.publisher_email = "unknown@example.com"  # Should be configured
    config.publisher_name = "Unknown Publisher"     # Should be configured
    
    # Publish package
    damn publish_package_to_registry(package_dir, manifest, config)
}
