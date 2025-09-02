# CURSED Package Registry Client
# Advanced registry operations with authentication, caching, and security
yeet "networkz"
yeet "jsonz"
yeet "filez" 
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "cryptz"

# Registry authentication methods
enum AuthMethod {
    None,
    ApiKey,
    OAuth,
    Certificate
}

# Registry client configuration
squad RegistryClient {
    sus base_url tea
    sus auth_method AuthMethod
    sus api_key tea
    sus oauth_token tea
    sus cert_path tea
    sus cache_dir tea
    sus timeout_seconds drip
    sus max_retries drip
    sus user_agent tea
}

# Package publication request
squad PublishRequest {
    sus package_metadata PackageMetadata
    sus archive_data tea
    sus signature tea
    sus checksums map[tea]tea
}

# Registry response
squad RegistryResponse {
    sus success lit
    sus status_code drip
    sus message tea
    sus data JsonValue
    sus headers map[tea]tea
}

# Authentication credentials
squad AuthCredentials {
    sus method AuthMethod  
    sus api_key tea
    sus username tea
    sus password tea
    sus oauth_token tea
    sus refresh_token tea
}

# Initialize registry client
slay init_registry_client(base_url tea, cache_dir tea) RegistryClient {
    filez.create_dir_all(cache_dir)
    
    damn RegistryClient {
        base_url: base_url,
        auth_method: AuthMethod.None,
        api_key: "",
        oauth_token: "",
        cert_path: "",
        cache_dir: cache_dir,
        timeout_seconds: 30,
        max_retries: 3,
        user_agent: "cursed-pkg/1.0.0"
    }
}

# Configure authentication for registry
slay configure_auth(client RegistryClient, credentials AuthCredentials) RegistryClient {
    client.auth_method = credentials.method
    
    match credentials.method {
        AuthMethod.ApiKey -> {
            client.api_key = credentials.api_key
        }
        AuthMethod.OAuth -> {
            client.oauth_token = credentials.oauth_token
        }
        AuthMethod.Certificate -> {
            client.cert_path = credentials.api_key  # Reuse field for cert path
        }
        _ -> {
            # No authentication needed
        }
    }
    
    damn client
}

# Search packages in registry with advanced filtering
slay search_packages_advanced(client RegistryClient, query tea, category tea, limit drip) (PackageMetadata[value], lit) {
    vibez.spill("Searching registry for:", query, "category:", category)
    
    sus search_url tea = build_search_url(client.base_url, query, category, limit)
    sus response RegistryResponse = make_authenticated_request(client, "GET", search_url, "")
    
    ready (!response.success) {
        vibez.spill("Search failed:", response.message)
        damn ([], cap)
    }
    
    sus packages PackageMetadata[value] = parse_search_results(response.data)
    
    # Cache search results for offline access
    cache_search_results(client, query, packages)
    
    damn (packages, based)
}

# Get detailed package information
slay get_package_info(client RegistryClient, name tea, version tea) (PackageMetadata, lit) {
    vibez.spill("Getting package info:", name, "version:", version)
    
    # Check cache first
    sus cached_info PackageMetadata = get_cached_package_info(client, name, version)
    ready (cached_info.name != "") {
        vibez.spill("Using cached package info for", name)
        damn (cached_info, based)
    }
    
    sus info_url tea = client.base_url + "/api/v1/packages/" + name
    ready (version != "") {
        info_url = info_url + "/" + version
    }
    
    sus response RegistryResponse = make_authenticated_request(client, "GET", info_url, "")
    
    ready (!response.success) {
        vibez.spill("Failed to get package info:", response.message)
        damn (PackageMetadata{}, cap)
    }
    
    sus package_info PackageMetadata = parse_package_metadata(response.data)
    
    # Cache the result
    cache_package_info(client, package_info)
    
    damn (package_info, based)
}

# Download package with integrity verification
slay download_package_verified(client RegistryClient, name tea, version tea) (tea, lit) {
    vibez.spill("Downloading package:", name, "v" + version)
    
    # Get package metadata for download URL and checksum
    sus (package_info, success) = get_package_info(client, name, version)
    ready (!success || package_info.download_url == "") {
        vibez.spill("Failed to get download information for", name)
        damn ("", cap)
    }
    
    # Download the package archive
    sus download_response RegistryResponse = make_authenticated_request(client, "GET", package_info.download_url, "")
    ready (!download_response.success) {
        vibez.spill("Failed to download package:", download_response.message)
        damn ("", cap)
    }
    
    # Extract binary data from response
    sus archive_data tea = extract_binary_data(download_response)
    
    # Verify checksum
    ready (package_info.checksum != "") {
        sus computed_checksum tea = cryptz.sha256(archive_data)
        ready (computed_checksum != package_info.checksum) {
            vibez.spill("Checksum mismatch for", name, "- download may be corrupted")
            damn ("", cap)
        }
        vibez.spill("Checksum verified for", name)
    }
    
    # Save to cache
    sus cache_path tea = client.cache_dir + "/downloads/" + name + "-" + version + ".tar.gz"
    filez.create_dir_all(client.cache_dir + "/downloads")
    ready (!filez.write_file(cache_path, archive_data)) {
        vibez.spill("Failed to cache package:", cache_path)
        damn ("", cap)
    }
    
    damn (cache_path, based)
}

# Publish package to registry
slay publish_package(client RegistryClient, request PublishRequest) lit {
    vibez.spill("Publishing package:", request.package_metadata.name, "v" + request.package_metadata.version)
    
    # Validate package before publishing
    ready (!validate_package_metadata(request.package_metadata)) {
        vibez.spill("Package metadata validation failed")
        damn cap
    }
    
    # Check authentication
    ready (client.auth_method == AuthMethod.None) {
        vibez.spill("Authentication required for publishing")
        damn cap
    }
    
    # Prepare publication request
    sus publish_url tea = client.base_url + "/api/v1/packages/publish"
    sus request_data JsonValue = serialize_publish_request(request)
    sus request_body tea = jsonz.json_stringify(request_data)
    
    sus response RegistryResponse = make_authenticated_request(client, "POST", publish_url, request_body)
    
    ready (!response.success) {
        vibez.spill("Publication failed:", response.message)
        damn cap
    }
    
    vibez.spill("Package published successfully!")
    damn based
}

# List user's published packages
slay list_published_packages(client RegistryClient, username tea) (PackageMetadata[value], lit) {
    sus list_url tea = client.base_url + "/api/v1/users/" + username + "/packages"
    sus response RegistryResponse = make_authenticated_request(client, "GET", list_url, "")
    
    ready (!response.success) {
        damn ([], cap)
    }
    
    sus packages PackageMetadata[value] = parse_package_list(response.data)
    damn (packages, based)
}

# Get package download statistics
slay get_package_stats(client RegistryClient, name tea) (JsonValue, lit) {
    sus stats_url tea = client.base_url + "/api/v1/packages/" + name + "/stats"
    sus response RegistryResponse = make_authenticated_request(client, "GET", stats_url, "")
    
    ready (!response.success) {
        damn (jsonz.json_create_object(), cap)
    }
    
    damn (response.data, based)
}

# Update package metadata
slay update_package_metadata(client RegistryClient, name tea, metadata PackageMetadata) lit {
    sus update_url tea = client.base_url + "/api/v1/packages/" + name + "/metadata"
    sus metadata_json JsonValue = serialize_package_metadata(metadata)
    sus request_body tea = jsonz.json_stringify(metadata_json)
    
    sus response RegistryResponse = make_authenticated_request(client, "PUT", update_url, request_body)
    damn response.success
}

# Yank/deprecate package version
slay yank_package_version(client RegistryClient, name tea, version tea, reason tea) lit {
    sus yank_url tea = client.base_url + "/api/v1/packages/" + name + "/" + version + "/yank"
    
    sus yank_data JsonValue = jsonz.json_create_object()
    yank_data = jsonz.json_object_set(yank_data, "reason", jsonz.json_create_string(reason))
    sus request_body tea = jsonz.json_stringify(yank_data)
    
    sus response RegistryResponse = make_authenticated_request(client, "POST", yank_url, request_body)
    ready (response.success) {
        vibez.spill("Successfully yanked", name, "v" + version)
    }
    
    damn response.success
}

# Get trending packages
slay get_trending_packages(client RegistryClient, time_period tea, limit drip) (PackageMetadata[value], lit) {
    sus trending_url tea = client.base_url + "/api/v1/trending?period=" + time_period + "&limit=" + stringz.from_int(limit)
    sus response RegistryResponse = make_authenticated_request(client, "GET", trending_url, "")
    
    ready (!response.success) {
        damn ([], cap)
    }
    
    sus packages PackageMetadata[value] = parse_package_list(response.data)
    damn (packages, based)
}

# Make authenticated HTTP request
slay make_authenticated_request(client RegistryClient, method tea, url tea, body tea) RegistryResponse {
    sus headers map[tea]tea = {}
    headers = map_set(headers, "User-Agent", client.user_agent)
    headers = map_set(headers, "Content-Type", "application/json")
    headers = map_set(headers, "Accept", "application/json")
    
    # Add authentication headers
    match client.auth_method {
        AuthMethod.ApiKey -> {
            headers = map_set(headers, "Authorization", "Bearer " + client.api_key)
        }
        AuthMethod.OAuth -> {
            headers = map_set(headers, "Authorization", "Bearer " + client.oauth_token)
        }
        AuthMethod.Certificate -> {
            # Certificate-based auth would be handled at TLS level
        }
        _ -> {
            # No authentication
        }
    }
    
    sus response tea = make_http_request_with_headers(method, url, body, headers, client.timeout_seconds)
    damn parse_registry_response(response)
}

# Build search URL with parameters
slay build_search_url(base_url tea, query tea, category tea, limit drip) tea {
    sus url tea = base_url + "/api/v1/packages/search?q=" + url_encode(query)
    
    ready (category != "") {
        url = url + "&category=" + url_encode(category)  
    }
    
    ready (limit > 0) {
        url = url + "&limit=" + stringz.from_int(limit)
    }
    
    damn url
}

# Parse registry HTTP response
slay parse_registry_response(http_response tea) RegistryResponse {
    # Parse HTTP response - simplified implementation
    sus status_code drip = extract_status_code(http_response)
    sus headers map[tea]tea = extract_headers(http_response)
    sus body tea = extract_body(http_response)
    
    sus success lit = (status_code >= 200 && status_code < 300)
    sus data JsonValue = jsonz.json_create_object()
    sus message tea = ""
    
    ready (body != "") {
        data = jsonz.json_parse(body)
        ready (jsonz.json_has_key(data, "message")) {
            message = jsonz.json_get_string(data, "message")
        }
    }
    
    ready (!success && message == "") {
        message = "HTTP " + stringz.from_int(status_code)
    }
    
    damn RegistryResponse {
        success: success,
        status_code: status_code,
        message: message,
        data: data,
        headers: headers
    }
}

# Cache management functions
slay cache_search_results(client RegistryClient, query tea, packages PackageMetadata[value]) {
    sus cache_key tea = "search_" + stringz.replace(query, " ", "_")
    sus cache_path tea = client.cache_dir + "/searches/" + cache_key + ".json"
    
    filez.create_dir_all(client.cache_dir + "/searches")
    
    sus cache_data JsonValue = jsonz.json_create_object()
    cache_data = jsonz.json_object_set(cache_data, "query", jsonz.json_create_string(query))
    cache_data = jsonz.json_object_set(cache_data, "timestamp", jsonz.json_create_string(get_current_time()))
    
    sus packages_array JsonValue = jsonz.json_create_array()
    bestie (sus i drip = 0; i < arrayz.len(packages); i = i + 1) {
        sus pkg_json JsonValue = serialize_package_metadata(packages[i])
        packages_array = jsonz.json_array_push(packages_array, pkg_json)
    }
    cache_data = jsonz.json_object_set(cache_data, "packages", packages_array)
    
    sus cache_content tea = jsonz.json_stringify_pretty(cache_data)
    filez.write_file(cache_path, cache_content)
}

slay cache_package_info(client RegistryClient, package_info PackageMetadata) {
    sus cache_path tea = client.cache_dir + "/packages/" + package_info.name + "-" + package_info.version + ".json"
    filez.create_dir_all(client.cache_dir + "/packages")
    
    sus package_json JsonValue = serialize_package_metadata(package_info)
    sus cache_content tea = jsonz.json_stringify_pretty(package_json)
    filez.write_file(cache_path, cache_content)
}

slay get_cached_package_info(client RegistryClient, name tea, version tea) PackageMetadata {
    sus cache_path tea = client.cache_dir + "/packages/" + name + "-" + version + ".json"
    
    ready (!filez.file_exists(cache_path)) {
        damn PackageMetadata{}
    }
    
    sus cache_content tea = filez.read_file(cache_path)
    ready (cache_content == "") {
        damn PackageMetadata{}
    }
    
    sus cache_data JsonValue = jsonz.json_parse(cache_content)
    damn parse_package_metadata(cache_data)
}

# Validation functions
slay validate_package_metadata(metadata PackageMetadata) lit {
    ready (metadata.name == "") {
        vibez.spill("Package name is required")
        damn cap
    }
    
    ready (metadata.version == "") {
        vibez.spill("Package version is required")
        damn cap
    }
    
    ready (!is_valid_package_name(metadata.name)) {
        vibez.spill("Invalid package name:", metadata.name)
        damn cap
    }
    
    ready (!is_valid_version(metadata.version)) {
        vibez.spill("Invalid version format:", metadata.version)
        damn cap
    }
    
    damn based
}

slay is_valid_package_name(name tea) lit {
    # Package names must be lowercase alphanumeric with hyphens
    ready (stringz.length(name) == 0 || stringz.length(name) > 64) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < stringz.length(name); i = i + 1) {
        sus char tea = stringz.char_at(name, i)
        ready (!(char >= "a" && char <= "z") && 
               !(char >= "0" && char <= "9") && 
               char != "-" && char != "_") {
            damn cap
        }
    }
    
    damn based
}

slay is_valid_version(version tea) lit {
    # Simple semantic version validation
    sus parts tea[value] = stringz.split(version, ".")
    ready (arrayz.len(parts) < 3) {
        damn cap
    }
    
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        sus part tea = parts[i]
        ready (!is_numeric(part)) {
            damn cap
        }
    }
    
    damn based
}

# Helper functions - simplified implementations
slay serialize_publish_request(request PublishRequest) JsonValue {
    sus obj JsonValue = jsonz.json_create_object()
    obj = jsonz.json_object_set(obj, "metadata", serialize_package_metadata(request.package_metadata))
    obj = jsonz.json_object_set(obj, "signature", jsonz.json_create_string(request.signature))
    damn obj
}

slay serialize_package_metadata(metadata PackageMetadata) JsonValue {
    sus obj JsonValue = jsonz.json_create_object()
    obj = jsonz.json_object_set(obj, "name", jsonz.json_create_string(metadata.name))
    obj = jsonz.json_object_set(obj, "version", jsonz.json_create_string(metadata.version))
    obj = jsonz.json_object_set(obj, "description", jsonz.json_create_string(metadata.description))
    obj = jsonz.json_object_set(obj, "license", jsonz.json_create_string(metadata.license))
    damn obj
}

slay parse_search_results(data JsonValue) PackageMetadata[value]{
    # Simplified parsing
    damn []
}

slay parse_package_list(data JsonValue) PackageMetadata[value]{
    # Simplified parsing
    damn []
}

slay extract_binary_data(response RegistryResponse) tea {
    # In real implementation, would extract binary data from HTTP response
    damn ""
}

slay make_http_request_with_headers(method tea, url tea, body tea, headers map[tea]tea, timeout drip) tea {
    # Simplified HTTP request - would use networkz module
    damn networkz.http_get(url)
}

slay url_encode(input tea) tea {
    # Simplified URL encoding
    damn stringz.replace(input, " ", "%20")
}

slay extract_status_code(response tea) drip {
    damn 200  # Simplified
}

slay extract_headers(response tea) map[tea]tea {
    damn {}  # Simplified
}

slay extract_body(response tea) tea {
    damn networkz.http_get_body(response)
}

slay get_current_time() tea {
    damn "2025-08-22T10:30:00Z"
}

slay is_numeric(s tea) lit {
    ready (stringz.length(s) == 0) { damn cap }
    bestie (sus i drip = 0; i < stringz.length(s); i = i + 1) {
        sus char tea = stringz.char_at(s, i)
        ready (!(char >= "0" && char <= "9")) {
            damn cap
        }
    }
    damn based
}

# Simplified map operations
slay map_set(m map[tea]tea, key tea, value tea) map[tea]tea {
    damn m  # Simplified implementation
}
