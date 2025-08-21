# Package Registry Implementation for CURSED
# Provides remote package registry connectivity and management
yeet "networkz"
yeet "jsonz"
yeet "filez"
yeet "cryptz"
yeet "stringz"
yeet "vibez"
yeet "configz"

# Registry authentication modes
enum AuthMode {
    None,
    ApiKey,
    OAuth,
    Certificate
}

# Enhanced registry configuration
squad RegistryConfig {
    sus url tea
    sus timeout_seconds drip
    sus max_retries drip
    sus auth_mode AuthMode
    sus api_key tea
    sus oauth_token tea
    sus cert_path tea
    sus key_path tea
    sus verify_ssl lit
    sus user_agent tea
    sus mirror_urls []tea
}

# Package publication status
enum PublishStatus {
    Success,
    Unauthorized,
    Conflict,
    ValidationError,
    NetworkError
}

# Package registry client with authentication
squad PackageRegistry {
    sus config RegistryConfig
    sus cache PackageCache
}

# Package cache for offline support
squad PackageCache {
    sus cache_dir tea
    sus index_file tea
    sus packages_cache map<tea, PackageMetadata>
    sus last_update tea
    sus ttl_seconds drip
}

# Initialize enhanced registry with authentication
slay init_registry(url tea, cache_dir tea, auth_config tea) PackageRegistry {
    sus config RegistryConfig = load_registry_config(auth_config)
    config.url = url
    
    sus cache PackageCache = PackageCache {
        cache_dir: cache_dir + "/registry",
        index_file: cache_dir + "/registry/index.json",
        packages_cache: {},
        last_update: "",
        ttl_seconds: 3600  # 1 hour cache TTL
    }
    
    filez.create_dir_all(cache.cache_dir)
    
    damn PackageRegistry { config: config, cache: cache }
}

# Load registry configuration from file
slay load_registry_config(config_path tea) RegistryConfig {
    ready (config_path == "" || !filez.file_exists(config_path)) {
        damn get_default_registry_config()
    }
    
    sus config_data tea = filez.read_file(config_path)
    sus config_json JsonValue = jsonz.json_parse(config_data)
    
    sus auth_mode AuthMode = AuthMode.None
    sus auth_mode_str tea = jsonz.json_get_string(config_json, "auth_mode")
    match auth_mode_str {
        "api_key" -> auth_mode = AuthMode.ApiKey
        "oauth" -> auth_mode = AuthMode.OAuth
        "certificate" -> auth_mode = AuthMode.Certificate
        _ -> auth_mode = AuthMode.None
    }
    
    damn RegistryConfig {
        url: jsonz.json_get_string(config_json, "url"),
        timeout_seconds: jsonz.json_get_int(config_json, "timeout_seconds"),
        max_retries: jsonz.json_get_int(config_json, "max_retries"),
        auth_mode: auth_mode,
        api_key: jsonz.json_get_string(config_json, "api_key"),
        oauth_token: jsonz.json_get_string(config_json, "oauth_token"),
        cert_path: jsonz.json_get_string(config_json, "cert_path"),
        key_path: jsonz.json_get_string(config_json, "key_path"),
        verify_ssl: jsonz.json_get_boolean(config_json, "verify_ssl"),
        user_agent: jsonz.json_get_string(config_json, "user_agent"),
        mirror_urls: parse_string_array(config_json, "mirror_urls")
    }
}

# Get default registry configuration
slay get_default_registry_config() RegistryConfig {
    damn RegistryConfig {
        url: "https://packages.cursedlang.org",
        timeout_seconds: 30,
        max_retries: 3,
        auth_mode: AuthMode.None,
        api_key: "",
        oauth_token: "",
        cert_path: "",
        key_path: "",
        verify_ssl: based,
        user_agent: "cursed-pkg/1.0.0",
        mirror_urls: [
            "https://mirror1.cursedlang.org",
            "https://mirror2.cursedlang.org"
        ]
    }
}

# Parse string array from JSON
slay parse_string_array(json JsonValue, key tea) []tea {
    sus result []tea = []
    ready (jsonz.json_has_key(json, key)) {
        sus array_json JsonValue = jsonz.json_get_object(json, key)
        ready (array_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(array_json.array_values); i = i + 1) {
                sus str_val tea = array_json.array_values[i].string_value
                result = arrayz.append(result, str_val)
            }
        }
    }
    damn result
}

# Make authenticated HTTP request to registry
slay make_registry_request(registry PackageRegistry, method tea, endpoint tea, body tea) tea {
    sus full_url tea = registry.config.url + endpoint
    sus headers map<tea, tea> = {}
    
    # Add authentication headers
    match registry.config.auth_mode {
        AuthMode.ApiKey -> {
            headers["Authorization"] = "Bearer " + registry.config.api_key
        }
        AuthMode.OAuth -> {
            headers["Authorization"] = "Bearer " + registry.config.oauth_token
        }
        _ -> {
            # No authentication needed
        }
    }
    
    # Add user agent
    headers["User-Agent"] = registry.config.user_agent
    headers["Content-Type"] = "application/json"
    
    # Make request with retries
    sus retry_count drip = 0
    bestie (retry_count <= registry.config.max_retries) {
        sus response tea = ""
        match method {
            "GET" -> response = networkz.http_get_with_headers(full_url, headers)
            "POST" -> response = networkz.http_post_with_headers(full_url, body, headers)
            "PUT" -> response = networkz.http_put_with_headers(full_url, body, headers)
            "DELETE" -> response = networkz.http_delete_with_headers(full_url, headers)
            _ -> damn ""
        }
        
        ready (networkz.http_is_success(response)) {
            damn response
        }
        
        # Check for rate limiting
        sus status_code drip = networkz.http_get_status_code(response)
        ready (status_code == 429) {
            # Rate limited - wait and retry
            sus retry_after drip = networkz.http_get_retry_after(response)
            vibez.spill("Rate limited, waiting", retry_after, "seconds...")
            # In real implementation: sleep(retry_after)
        }
        
        retry_count = retry_count + 1
    }
    
    # Try mirrors if main registry fails
    bestie (sus i drip = 0; i < arrayz.len(registry.config.mirror_urls); i = i + 1) {
        sus mirror_url tea = registry.config.mirror_urls[i]
        sus mirror_endpoint tea = mirror_url + endpoint
        
        sus response tea = ""
        match method {
            "GET" -> response = networkz.http_get_with_headers(mirror_endpoint, headers)
            _ -> continue  # Only GET requests to mirrors
        }
        
        ready (networkz.http_is_success(response)) {
            vibez.spill("Using mirror:", mirror_url)
            damn response
        }
    }
    
    damn ""  # All attempts failed
}

# Search packages with caching
slay search_packages_enhanced(registry PackageRegistry, query tea, category tea, limit drip) []PackageMetadata {
    # Check cache first
    sus cache_key tea = "search:" + query + ":" + category
    ready (is_cache_valid(registry.cache, cache_key)) {
        sus cached_results []PackageMetadata = get_cached_search(registry.cache, cache_key)
        ready (arrayz.len(cached_results) > 0) {
            vibez.spill("Using cached search results")
            damn limit_results(cached_results, limit)
        }
    }
    
    # Build search endpoint
    sus endpoint tea = "/api/v1/packages/search?q=" + networkz.url_encode(query)
    ready (category != "") {
        endpoint = endpoint + "&category=" + networkz.url_encode(category)
    }
    ready (limit > 0) {
        endpoint = endpoint + "&limit=" + stringz.from_int(limit)
    }
    
    sus response tea = make_registry_request(registry, "GET", endpoint, "")
    ready (response == "") {
        vibez.spill("Failed to search packages")
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
        sus metadata PackageMetadata = parse_package_metadata_enhanced(pkg_json)
        packages = arrayz.append(packages, metadata)
    }
    
    # Cache results
    cache_search_results(registry.cache, cache_key, packages)
    
    damn packages
}

# Get package information with version resolution
slay get_package_info(registry PackageRegistry, name tea, version_req tea) PackageMetadata {
    sus endpoint tea = "/api/v1/packages/" + name
    ready (version_req != "") {
        endpoint = endpoint + "/" + version_req
    }
    
    sus response tea = make_registry_request(registry, "GET", endpoint, "")
    ready (response == "") {
        damn PackageMetadata{}
    }
    
    sus body tea = networkz.http_get_body(response)
    sus json_data JsonValue = jsonz.json_parse(body)
    
    damn parse_package_metadata_enhanced(json_data)
}

# List all versions of a package
slay list_package_versions(registry PackageRegistry, name tea) []tea {
    sus endpoint tea = "/api/v1/packages/" + name + "/versions"
    
    sus response tea = make_registry_request(registry, "GET", endpoint, "")
    ready (response == "") {
        damn []
    }
    
    sus body tea = networkz.http_get_body(response)
    sus json_data JsonValue = jsonz.json_parse(body)
    
    ready (!jsonz.json_has_key(json_data, "versions")) {
        damn []
    }
    
    sus versions_json JsonValue = jsonz.json_get_object(json_data, "versions")
    ready (versions_json.type != "array") {
        damn []
    }
    
    sus versions []tea = []
    bestie (sus i drip = 0; i < arrayz.len(versions_json.array_values); i = i + 1) {
        sus version tea = versions_json.array_values[i].string_value
        versions = arrayz.append(versions, version)
    }
    
    damn versions
}

# Publish package to registry
slay publish_package(registry PackageRegistry, package_path tea, metadata PackageMetadata) PublishStatus {
    # Create package archive
    sus archive_path tea = create_package_archive(package_path, metadata)
    ready (archive_path == "") {
        damn PublishStatus.ValidationError
    }
    
    # Calculate checksum
    sus checksum tea = cryptz.sha256_file(archive_path)
    
    # Create publication payload
    sus publish_data JsonValue = jsonz.json_create_object()
    publish_data = jsonz.json_object_set(publish_data, "name", jsonz.json_create_string(metadata.name))
    publish_data = jsonz.json_object_set(publish_data, "version", jsonz.json_create_string(metadata.version))
    publish_data = jsonz.json_object_set(publish_data, "description", jsonz.json_create_string(metadata.description))
    publish_data = jsonz.json_object_set(publish_data, "license", jsonz.json_create_string(metadata.license))
    publish_data = jsonz.json_object_set(publish_data, "checksum", jsonz.json_create_string(checksum))
    
    # Add authors
    sus authors_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(metadata.authors); i = i + 1) {
        authors_array = jsonz.json_array_push(authors_array, jsonz.json_create_string(metadata.authors[i]))
    }
    publish_data = jsonz.json_object_set(publish_data, "authors", authors_array)
    
    # Add dependencies
    sus deps_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(metadata.dependencies); i = i + 1) {
        sus dep PackageDependency = metadata.dependencies[i]
        sus dep_obj JsonValue = jsonz.json_create_object()
        dep_obj = jsonz.json_object_set(dep_obj, "name", jsonz.json_create_string(dep.name))
        dep_obj = jsonz.json_object_set(dep_obj, "version_req", jsonz.json_create_string(dep.version_req))
        dep_obj = jsonz.json_object_set(dep_obj, "optional", jsonz.json_create_boolean(dep.optional))
        deps_array = jsonz.json_array_push(deps_array, dep_obj)
    }
    publish_data = jsonz.json_object_set(publish_data, "dependencies", deps_array)
    
    sus json_payload tea = jsonz.json_stringify(publish_data)
    
    # Publish metadata first
    sus endpoint tea = "/api/v1/packages"
    sus response tea = make_registry_request(registry, "POST", endpoint, json_payload)
    
    sus status_code drip = networkz.http_get_status_code(response)
    ready (status_code == 401) {
        damn PublishStatus.Unauthorized
    }
    ready (status_code == 409) {
        damn PublishStatus.Conflict
    }
    ready (status_code >= 400) {
        damn PublishStatus.ValidationError
    }
    
    # Upload package archive
    sus upload_endpoint tea = "/api/v1/packages/" + metadata.name + "/" + metadata.version + "/upload"
    sus archive_data tea = filez.read_file_binary(archive_path)
    
    sus upload_response tea = make_registry_request(registry, "PUT", upload_endpoint, archive_data)
    sus upload_status drip = networkz.http_get_status_code(upload_response)
    
    ready (upload_status >= 200 && upload_status < 300) {
        # Clean up temporary archive
        filez.remove_file(archive_path)
        damn PublishStatus.Success
    }
    
    damn PublishStatus.NetworkError
}

# Create package archive for publishing
slay create_package_archive(package_path tea, metadata PackageMetadata) tea {
    sus temp_dir tea = "/tmp/cursed-pkg-" + stringz.random_string(8)
    sus archive_name tea = metadata.name + "-" + metadata.version + ".tar.gz"
    sus archive_path tea = temp_dir + "/" + archive_name
    
    filez.create_dir_all(temp_dir)
    
    # Create tarball (simplified - in real implementation would use compression)
    ready (filez.create_archive(package_path, archive_path)) {
        damn archive_path
    }
    
    damn ""
}

# Enhanced package metadata parsing with validation
slay parse_package_metadata_enhanced(json JsonValue) PackageMetadata {
    sus name tea = jsonz.json_get_string(json, "name")
    sus version tea = jsonz.json_get_string(json, "version")
    sus description tea = jsonz.json_get_string(json, "description")
    sus license tea = jsonz.json_get_string(json, "license")
    sus homepage tea = jsonz.json_get_string(json, "homepage")
    sus repository tea = jsonz.json_get_string(json, "repository")
    sus download_url tea = jsonz.json_get_string(json, "download_url")
    sus checksum tea = jsonz.json_get_string(json, "checksum")
    
    # Parse authors with validation
    sus authors []tea = []
    ready (jsonz.json_has_key(json, "authors")) {
        sus authors_json JsonValue = jsonz.json_get_object(json, "authors")
        ready (authors_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(authors_json.array_values); i = i + 1) {
                sus author tea = authors_json.array_values[i].string_value
                ready (validate_author_format(author)) {
                    authors = arrayz.append(authors, author)
                }
            }
        }
    }
    
    # Parse keywords
    sus keywords []tea = parse_string_array(json, "keywords")
    sus categories []tea = parse_string_array(json, "categories")
    
    # Parse dependencies with enhanced validation
    sus dependencies []PackageDependency = []
    ready (jsonz.json_has_key(json, "dependencies")) {
        sus deps_json JsonValue = jsonz.json_get_object(json, "dependencies")
        ready (deps_json.type == "array") {
            bestie (sus i drip = 0; i < arrayz.len(deps_json.array_values); i = i + 1) {
                sus dep_json JsonValue = deps_json.array_values[i]
                sus dep_name tea = jsonz.json_get_string(dep_json, "name")
                sus version_req tea = jsonz.json_get_string(dep_json, "version_req")
                
                ready (validate_package_name(dep_name) && validate_version_requirement(version_req)) {
                    sus dep PackageDependency = PackageDependency {
                        name: dep_name,
                        version_req: version_req,
                        optional: jsonz.json_get_boolean(dep_json, "optional"),
                        features: parse_string_array(dep_json, "features")
                    }
                    dependencies = arrayz.append(dependencies, dep)
                }
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
        categories: categories,
        dependencies: dependencies,
        download_url: download_url,
        checksum: checksum
    }
}

# Validate author format (Name <email>)
slay validate_author_format(author tea) lit {
    damn stringz.contains(author, "<") && stringz.contains(author, ">")
}

# Validate package name format
slay validate_package_name(name tea) lit {
    ready (stringz.len(name) < 1 || stringz.len(name) > 64) {
        damn cap
    }
    
    # Check for valid characters (alphanumeric, hyphens, underscores)
    bestie (sus i drip = 0; i < stringz.len(name); i = i + 1) {
        sus ch tea = stringz.char_at(name, i)
        ready (!stringz.is_alphanumeric(ch) && ch != "-" && ch != "_") {
            damn cap
        }
    }
    
    damn based
}

# Validate version requirement format
slay validate_version_requirement(req tea) lit {
    ready (req == "") {
        damn based  # Empty version requirement means any version
    }
    
    # Support common version operators: ^, ~, >=, >, <=, <, =
    sus operators []tea = ["^", "~", ">=", ">", "<=", "<", "="]
    
    bestie (sus i drip = 0; i < arrayz.len(operators); i = i + 1) {
        sus op tea = operators[i]
        ready (stringz.starts_with(req, op)) {
            sus version_part tea = stringz.substring(req, stringz.len(op), stringz.len(req))
            damn validate_semver_format(version_part)
        }
    }
    
    # No operator means exact version
    damn validate_semver_format(req)
}

# Validate semantic version format (major.minor.patch)
slay validate_semver_format(version tea) lit {
    sus parts []tea = stringz.split(version, ".")
    ready (arrayz.len(parts) != 3) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        sus part tea = parts[i]
        ready (!stringz.is_numeric(part)) {
            damn cap
        }
    }
    
    damn based
}

# Cache management functions
slay is_cache_valid(cache PackageCache, key tea) lit {
    ready (cache.last_update == "") {
        damn cap  # Never cached
    }
    
    # In real implementation: check timestamp against TTL
    damn based
}

slay get_cached_search(cache PackageCache, key tea) []PackageMetadata {
    # In real implementation: retrieve from cache
    damn []
}

slay cache_search_results(cache PackageCache, key tea, results []PackageMetadata) {
    # In real implementation: store in cache with timestamp
}

slay limit_results(results []PackageMetadata, limit drip) []PackageMetadata {
    ready (limit <= 0 || limit >= arrayz.len(results)) {
        damn results
    }
    
    sus limited []PackageMetadata = []
    bestie (sus i drip = 0; i < limit; i = i + 1) {
        limited = arrayz.append(limited, results[i])
    }
    
    damn limited
}
