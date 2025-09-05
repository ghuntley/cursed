// Registry Client for CURSED Package Manager
// Handles communication with package registries for search, download, and publishing

yeet "testz"
yeet "vibez"
yeet "jsonz"
yeet "httpz"
yeet "filez"
yeet "stringz"
yeet "cryptz"

// Package registry configuration
squad RegistryConfig {
    spill url tea
    spill auth_endpoint tea
    spill publish_endpoint tea
    spill search_endpoint tea
    spill api_version tea
    spill timeout_seconds drip
    spill max_retries drip
    
    slay new(url tea) RegistryConfig {
        damn RegistryConfig{
            url: url,
            auth_endpoint: concat_str(url, "/auth"),
            publish_endpoint: concat_str(url, "/publish"),
            search_endpoint: concat_str(url, "/search"),
            api_version: "v1",
            timeout_seconds: 30,
            max_retries: 3
        }
    }
    
    slay getPackageUrl(self RegistryConfig, package_name tea) tea {
        damn format_str("{}/api/{}/packages/{}", self.url, self.api_version, package_name)
    }
    
    slay getVersionUrl(self RegistryConfig, package_name tea, version tea) tea {
        damn format_str("{}/api/{}/packages/{}/{}", 
                       self.url, self.api_version, package_name, version)
    }
    
    slay getDownloadUrl(self RegistryConfig, package_name tea, version tea) tea {
        damn format_str("{}/api/{}/packages/{}/{}/download", 
                       self.url, self.api_version, package_name, version)
    }
}

// Authentication and authorization
squad RegistryAuth {
    spill username tea
    spill api_token tea
    spill refresh_token tea
    spill expires_at drip
    
    slay new() RegistryAuth {
        damn RegistryAuth{
            username: "",
            api_token: "",
            refresh_token: "",
            expires_at: 0
        }
    }
    
    slay isValid(self RegistryAuth) lit {
        ready (self.api_token == "") { damn cringe }
        // In production, would check current time against expires_at
        damn based
    }
    
    slay getAuthHeader(self RegistryAuth) tea {
        damn format_str("Bearer {}", self.api_token)
    }
    
    slay saveToFile(self RegistryAuth, path tea) {
        sus content tea = format_str("username={}\napi_token={}\nrefresh_token={}\nexpires_at={}\n",
                                    self.username, self.api_token, self.refresh_token, self.expires_at)
        write_file(path, content)
    }
    
    slay loadFromFile(path tea) RegistryAuth {
        sus auth RegistryAuth = RegistryAuth.new()
        
        ready (!file_exists(path)) {
            damn auth
        }
        
        sus content tea = read_file(path)
        sus lines tea[value] = split_str(content, "\n")
        
        sus i drip = 0
        bestie (i < len(lines)) {
            sus line tea = trim_str(lines[i])
            ready (starts_with(line, "username=")) {
                auth.username = slice_str(line, 9)
            }
            ready (starts_with(line, "api_token=")) {
                auth.api_token = slice_str(line, 10)
            }
            ready (starts_with(line, "refresh_token=")) {
                auth.refresh_token = slice_str(line, 14)
            }
            ready (starts_with(line, "expires_at=")) {
                auth.expires_at = parse_int(slice_str(line, 11))
            }
            i = i + 1
        }
        
        damn auth
    }
}

// Main registry client for package operations
squad PackageRegistryClient {
    spill config RegistryConfig
    spill auth RegistryAuth
    spill cache_dir tea
    spill user_agent tea
    
    slay new(registry_url tea, cache_dir tea) PackageRegistryClient {
        damn PackageRegistryClient{
            config: RegistryConfig.new(registry_url),
            auth: RegistryAuth.loadFromFile(concat_str(cache_dir, "/auth.txt")),
            cache_dir: cache_dir,
            user_agent: "CURSED Package Manager 1.0.0"
        }
    }
    
    slay authenticate(self PackageRegistryClient, username tea, password tea) lit {
        sus auth_data tea = format_str("{{\"username\":\"{}\",\"password\":\"{}\"}}", 
                                      username, password)
        
        sus headers tea[value] = tea[value]{}
        headers = append_array(headers, "Content-Type: application/json")
        headers = append_array(headers, format_str("User-Agent: {}", self.user_agent))
        
        sus response HttpResponse = http_post(self.config.auth_endpoint, auth_data, headers)
        ready (response.status_code != 200) {
            vibez.spill("Authentication failed: {}", response.body)
            damn cringe
        }
        
        sus auth_result AuthResult = parseAuthResponse(response.body)
        self.auth.username = username
        self.auth.api_token = auth_result.access_token
        self.auth.refresh_token = auth_result.refresh_token
        self.auth.expires_at = auth_result.expires_at
        
        // Save authentication to file
        sus auth_path tea = concat_str(self.cache_dir, "/auth.txt")
        self.auth.saveToFile(auth_path)
        
        vibez.spill("Successfully authenticated as {}", username)
        damn based
    }
    
    slay searchPackages(self PackageRegistryClient, query tea, limit drip) PackageSearchResult[value]{
        sus url tea = format_str("{}?q={}&limit={}", self.config.search_endpoint, query, limit)
        sus headers tea[value] = self.getDefaultHeaders()
        
        sus response HttpResponse = http_get_with_headers(url, headers)
        ready (response.status_code != 200) {
            vibez.spill("Search failed: {}", response.body)
            damn PackageSearchResult[value]{}
        }
        
        damn parseSearchResults(response.body)
    }
    
    slay getPackageInfo(self PackageRegistryClient, package_name tea) PackageInfo {
        sus url tea = self.config.getPackageUrl(package_name)
        sus headers tea[value] = self.getDefaultHeaders()
        
        sus response HttpResponse = http_get_with_headers(url, headers)
        ready (response.status_code == 404) {
            vibez.spill("Package {} not found", package_name)
            damn PackageInfo.new("", PackageVersion.new(0, 0, 0))
        }
        
        ready (response.status_code != 200) {
            vibez.spill("Failed to get package info: {}", response.body)
            damn PackageInfo.new("", PackageVersion.new(0, 0, 0))
        }
        
        damn parsePackageInfo(response.body)
    }
    
    slay getPackageVersions(self PackageRegistryClient, package_name tea) PackageVersion[value]{
        sus url tea = format_str("{}/versions", self.config.getPackageUrl(package_name))
        sus headers tea[value] = self.getDefaultHeaders()
        
        sus response HttpResponse = http_get_with_headers(url, headers)
        ready (response.status_code != 200) {
            vibez.spill("Failed to get package versions: {}", response.body)
            damn PackageVersion[value]{}
        }
        
        damn parseVersionList(response.body)
    }
    
    slay downloadPackage(self PackageRegistryClient, package_name tea, version tea) DownloadResult {
        sus url tea = self.config.getDownloadUrl(package_name, version)
        sus headers tea[value] = self.getDefaultHeaders()
        
        // Create package-specific cache directory
        sus package_cache_dir tea = format_str("{}/packages/{}/{}", 
                                             self.cache_dir, package_name, version)
        create_dir_recursive(package_cache_dir)
        
        sus archive_path tea = format_str("{}/package.tar.gz", package_cache_dir)
        
        // Check if already cached
        ready (file_exists(archive_path)) {
            vibez.spill("Package {} {} already cached", package_name, version)
            damn DownloadResult{
                success: based,
                path: archive_path,
                checksum: calculate_file_checksum(archive_path)
            }
        }
        
        vibez.spill("Downloading {} {}...", package_name, version)
        sus response HttpResponse = http_get_with_headers(url, headers)
        ready (response.status_code != 200) {
            vibez.spill("Download failed: {}", response.body)
            damn DownloadResult{success: cringe, path: "", checksum: ""}
        }
        
        // Write to cache
        write_binary_file(archive_path, response.body)
        sus checksum tea = calculate_file_checksum(archive_path)
        
        vibez.spill("Downloaded {} {} (checksum: {})", package_name, version, checksum)
        damn DownloadResult{
            success: based,
            path: archive_path,
            checksum: checksum
        }
    }
    
    slay publishPackage(self PackageRegistryClient, package_path tea, manifest PackageManifest) lit {
        ready (!self.auth.isValid()) {
            vibez.spill("Not authenticated. Please run 'cursed-pkg login' first.")
            damn cringe
        }
        
        // Create package archive
        sus archive_path tea = format_str("{}/publish-{}-{}.tar.gz", 
                                        self.cache_dir, manifest.name, manifest.version)
        ready (!create_tar_archive(package_path, archive_path)) {
            vibez.spill("Failed to create package archive")
            damn cringe
        }
        
        // Calculate checksum for integrity verification
        sus checksum tea = calculate_file_checksum(archive_path)
        
        // Prepare publish request
        sus publish_data PublishRequest = PublishRequest{
            name: manifest.name,
            version: manifest.version,
            description: manifest.description,
            authors: manifest.authors,
            license: manifest.license,
            keywords: manifest.keywords,
            repository: manifest.repository,
            dependencies: manifest.dependencies,
            checksum: checksum
        }
        
        sus headers tea[value] = self.getAuthenticatedHeaders()
        headers = append_array(headers, "Content-Type: multipart/form-data")
        
        vibez.spill("Publishing {} {}...", manifest.name, manifest.version)
        sus response HttpResponse = http_post_file(self.config.publish_endpoint, 
                                                  archive_path, 
                                                  serializePublishRequest(publish_data),
                                                  headers)
        
        ready (response.status_code == 409) {
            vibez.spill("Package {} {} already exists", manifest.name, manifest.version)
            damn cringe
        }
        
        ready (response.status_code != 201) {
            vibez.spill("Publish failed: {}", response.body)
            damn cringe
        }
        
        vibez.spill("Successfully published {} {}", manifest.name, manifest.version)
        
        // Clean up temporary archive
        delete_file(archive_path)
        damn based
    }
    
    slay unpublishPackage(self PackageRegistryClient, package_name tea, version tea) lit {
        ready (!self.auth.isValid()) {
            vibez.spill("Not authenticated. Please run 'cursed-pkg login' first.")
            damn cringe
        }
        
        sus url tea = self.config.getVersionUrl(package_name, version)
        sus headers tea[value] = self.getAuthenticatedHeaders()
        
        vibez.spill("Unpublishing {} {}...", package_name, version)
        sus response HttpResponse = http_delete(url, headers)
        
        ready (response.status_code == 404) {
            vibez.spill("Package {} {} not found", package_name, version)
            damn cringe
        }
        
        ready (response.status_code != 200) {
            vibez.spill("Unpublish failed: {}", response.body)
            damn cringe
        }
        
        vibez.spill("Successfully unpublished {} {}", package_name, version)
        damn based
    }
    
    slay clearCache(self PackageRegistryClient) {
        sus packages_cache tea = concat_str(self.cache_dir, "/packages")
        ready (dir_exists(packages_cache)) {
            delete_dir_recursive(packages_cache)
            vibez.spill("Package cache cleared")
        } otherwise {
            vibez.spill("No cache to clear")
        }
    }
    
    slay getDefaultHeaders(self PackageRegistryClient) tea[value]{
        sus headers tea[value] = tea[value]{}
        headers = append_array(headers, format_str("User-Agent: {}", self.user_agent))
        headers = append_array(headers, "Accept: application/json")
        damn headers
    }
    
    slay getAuthenticatedHeaders(self PackageRegistryClient) tea[value]{
        sus headers tea[value] = self.getDefaultHeaders()
        ready (self.auth.isValid()) {
            headers = append_array(headers, format_str("Authorization: {}", self.auth.getAuthHeader()))
        }
        damn headers
    }
}

// Response structures
squad HttpResponse {
    spill status_code drip
    spill body tea
    spill headers tea[value]
}

squad AuthResult {
    spill access_token tea
    spill refresh_token tea
    spill expires_at drip
    spill token_type tea
}

squad PackageSearchResult {
    spill name tea
    spill version tea
    spill description tea
    spill downloads drip
    spill updated_at tea
    spill relevance_score drip
}

squad DownloadResult {
    spill success lit
    spill path tea
    spill checksum tea
}

squad PublishRequest {
    spill name tea
    spill version tea
    spill description tea
    spill authors tea[value]
    spill license tea
    spill keywords tea[value]
    spill repository tea
    spill dependencies PackageDependency[value]
    spill checksum tea
}

// Registry response parsers
slay parseAuthResponse(json_str tea) AuthResult {
    // Simplified JSON parsing - in production would use proper JSON parser
    sus result AuthResult = AuthResult{
        access_token: "",
        refresh_token: "",
        expires_at: 0,
        token_type: "Bearer"
    }
    
    // Extract access_token
    sus token_start drip = find_str(json_str, "\"access_token\":\"") + 16
    ready (token_start > 15) {
        sus token_end drip = find_str(slice_str(json_str, token_start), "\"") + token_start
        result.access_token = slice_str(json_str, token_start, token_end)
    }
    
    // Extract refresh_token
    sus refresh_start drip = find_str(json_str, "\"refresh_token\":\"") + 17
    ready (refresh_start > 16) {
        sus refresh_end drip = find_str(slice_str(json_str, refresh_start), "\"") + refresh_start
        result.refresh_token = slice_str(json_str, refresh_start, refresh_end)
    }
    
    // Extract expires_at
    sus expires_start drip = find_str(json_str, "\"expires_at\":") + 13
    ready (expires_start > 12) {
        sus expires_end drip = find_str(slice_str(json_str, expires_start), ",") + expires_start
        ready (expires_end == expires_start - 1) {
            expires_end = find_str(slice_str(json_str, expires_start), "}") + expires_start
        }
        sus expires_str tea = slice_str(json_str, expires_start, expires_end)
        result.expires_at = parse_int(expires_str)
    }
    
    damn result
}

slay parseSearchResults(json_str tea) PackageSearchResult[value]{
    sus results PackageSearchResult[value] = PackageSearchResult[value]{}
    
    // Simplified parsing - in production would use proper JSON parser
    sus packages_start drip = find_str(json_str, "\"packages\":[") + 12
    ready (packages_start < 12) {
        damn results
    }
    
    sus packages_section tea = slice_str(json_str, packages_start)
    sus packages_end drip = find_str(packages_section, "]")
    sus packages_json tea = slice_str(packages_section, 0, packages_end)
    
    // Parse individual package objects
    sus package_objects tea[value] = split_str(packages_json, "},{")
    sus i drip = 0
    bestie (i < len(package_objects)) {
        sus pkg_json tea = package_objects[i]
        sus result PackageSearchResult = parsePackageSearchResult(pkg_json)
        results = append_array(results, result)
        i = i + 1
    }
    
    damn results
}

slay parsePackageSearchResult(json_str tea) PackageSearchResult {
    sus result PackageSearchResult = PackageSearchResult{
        name: "",
        version: "",
        description: "",
        downloads: 0,
        updated_at: "",
        relevance_score: 0
    }
    
    // Extract name
    sus name_start drip = find_str(json_str, "\"name\":\"") + 8
    ready (name_start > 7) {
        sus name_end drip = find_str(slice_str(json_str, name_start), "\"") + name_start
        result.name = slice_str(json_str, name_start, name_end)
    }
    
    // Extract version
    sus version_start drip = find_str(json_str, "\"version\":\"") + 11
    ready (version_start > 10) {
        sus version_end drip = find_str(slice_str(json_str, version_start), "\"") + version_start
        result.version = slice_str(json_str, version_start, version_end)
    }
    
    // Extract description
    sus desc_start drip = find_str(json_str, "\"description\":\"") + 15
    ready (desc_start > 14) {
        sus desc_end drip = find_str(slice_str(json_str, desc_start), "\"") + desc_start
        result.description = slice_str(json_str, desc_start, desc_end)
    }
    
    damn result
}

slay serializePublishRequest(request PublishRequest) tea {
    sus json tea = "{"
    json = concat_str(json, format_str("\"name\":\"{}\",", request.name))
    json = concat_str(json, format_str("\"version\":\"{}\",", request.version))
    json = concat_str(json, format_str("\"description\":\"{}\",", request.description))
    json = concat_str(json, format_str("\"license\":\"{}\",", request.license))
    json = concat_str(json, format_str("\"repository\":\"{}\",", request.repository))
    json = concat_str(json, format_str("\"checksum\":\"{}\",", request.checksum))
    
    // Serialize authors array
    json = concat_str(json, "\"authors\":[")
    sus i drip = 0
    bestie (i < len(request.authors)) {
        ready (i > 0) { json = concat_str(json, ",") }
        json = concat_str(json, format_str("\"{}\"", request.authors[i]))
        i = i + 1
    }
    json = concat_str(json, "],")
    
    // Serialize keywords array
    json = concat_str(json, "\"keywords\":[")
    i = 0
    bestie (i < len(request.keywords)) {
        ready (i > 0) { json = concat_str(json, ",") }
        json = concat_str(json, format_str("\"{}\"", request.keywords[i]))
        i = i + 1
    }
    json = concat_str(json, "],")
    
    // Serialize dependencies
    json = concat_str(json, "\"dependencies\":{")
    i = 0
    bestie (i < len(request.dependencies)) {
        ready (i > 0) { json = concat_str(json, ",") }
        sus dep PackageDependency = request.dependencies[i]
        json = concat_str(json, format_str("\"{}\":\"{}\"", dep.name, dep.version_constraint))
        i = i + 1
    }
    json = concat_str(json, "}")
    
    json = concat_str(json, "}")
    damn json
}

// HTTP utility functions (would be implemented using httpz stdlib)
slay http_get_with_headers(url tea, headers tea[value]) HttpResponse {
    // Mock implementation - would use real HTTP client
    damn HttpResponse{
        status_code: 200,
        body: "{\"name\":\"example\",\"version\":\"1.0.0\"}",
        headers: tea[value]{}
    }
}

slay http_post(url tea, data tea, headers tea[value]) HttpResponse {
    // Mock implementation
    damn HttpResponse{
        status_code: 200,
        body: "{\"access_token\":\"token123\",\"expires_at\":1234567890}",
        headers: tea[value]{}
    }
}

slay http_post_file(url tea, file_path tea, metadata tea, headers tea[value]) HttpResponse {
    // Mock implementation
    damn HttpResponse{
        status_code: 201,
        body: "{\"success\":true,\"id\":\"pkg123\"}",
        headers: tea[value]{}
    }
}

slay http_delete(url tea, headers tea[value]) HttpResponse {
    // Mock implementation
    damn HttpResponse{
        status_code: 200,
        body: "{\"success\":true}",
        headers: tea[value]{}
    }
}

slay create_tar_archive(source_dir tea, archive_path tea) lit {
    // Mock implementation - would use compression stdlib
    damn based
}

slay calculate_file_checksum(file_path tea) tea {
    // Mock implementation - would use cryptz stdlib
    damn "sha256:abc123def456"
}

slay write_binary_file(path tea, content tea) {
    // Mock implementation - would use filez stdlib
}

slay create_dir_recursive(path tea) {
    // Mock implementation - would use filez stdlib
}

slay delete_dir_recursive(path tea) {
    // Mock implementation - would use filez stdlib
}
