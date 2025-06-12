# CURSED Package Registry API Documentation

## Overview

The CURSED Package Registry provides a RESTful HTTP API for package management operations including package search, metadata retrieval, and package downloads. This document describes the API endpoints, request/response formats, authentication, and error handling.

## Base URL

The default registry URL is `https://packages.cursed-lang.org/api/v1`

## Authentication

### Bearer Token Authentication

For private registries or authenticated operations, include an Authorization header:

```
Authorization: Bearer <your-token>
```

### API Key Authentication (Alternative)

Some registries may support API key authentication:

```
X-API-Key: <your-api-key>
```

## API Endpoints

### 1. Package Information

**GET** `/packages/{name}`
**GET** `/packages/{name}/{version}`

Retrieve metadata for a specific package or package version.

#### Parameters
- `name` (string, required): Package name
- `version` (string, optional): Specific version. If omitted, returns latest version.

#### Response

```json
{
  "name": "my-package",
  "version": "1.2.3",
  "description": "A useful CURSED package",
  "download_url": "https://packages.cursed-lang.org/files/my-package-1.2.3.tar.gz",
  "checksum": "sha256:a1b2c3d4e5f6789...",
  "size": 1024000,
  "published_at": "2024-01-15T10:30:00Z",
  "authors": ["John Doe <john@example.com>"],
  "license": "MIT",
  "repository": "https://github.com/example/my-package",
  "keywords": ["utility", "helper", "cursed"]
}
```

#### Error Responses
- `404 Not Found`: Package or version not found
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Access denied
- `500 Internal Server Error`: Server error

### 2. Package Search

**GET** `/packages?q={query}&limit={limit}&offset={offset}`

Search for packages matching a query string.

#### Query Parameters
- `q` (string, required): Search query
- `limit` (integer, optional): Maximum number of results (default: 20, max: 100)
- `offset` (integer, optional): Number of results to skip (default: 0)
- `sort` (string, optional): Sort order (`relevance`, `name`, `downloads`, `updated`)

#### Response

```json
{
  "packages": [
    {
      "name": "my-package",
      "version": "1.2.3",
      "description": "A useful CURSED package",
      "authors": ["John Doe"],
      "license": "MIT",
      "repository": "https://github.com/example/my-package",
      "keywords": ["utility", "helper"],
      "categories": ["development-tools"],
      "dependencies": {
        "other-package": "^2.0.0"
      },
      "dev_dependencies": {
        "test-framework": "^1.0.0"
      }
    }
  ],
  "total": 150,
  "limit": 20,
  "offset": 0
}
```

### 3. Package Download

**GET** `/packages/{name}/{version}/download`

Download the package archive file.

#### Parameters
- `name` (string, required): Package name
- `version` (string, required): Package version

#### Response
- **Content-Type**: `application/gzip` or `application/zip`
- **Body**: Binary package archive
- **Headers**:
  - `Content-Length`: File size in bytes
  - `X-Checksum-SHA256`: SHA-256 checksum of the file
  - `X-Package-Version`: Package version
  - `Content-Disposition`: `attachment; filename="package-version.tar.gz"`

#### Error Responses
- `404 Not Found`: Package or version not found
- `410 Gone`: Package version has been yanked/removed

### 4. Registry Index

**GET** `/index`

Get information about the registry and package index.

#### Response

```json
{
  "version": "1.0",
  "total_packages": 1500,
  "last_updated": "2024-01-15T12:00:00Z",
  "supported_formats": ["tar.gz", "zip"],
  "api_version": "v1",
  "features": ["search", "authentication", "webhooks"]
}
```

### 5. Package Versions

**GET** `/packages/{name}/versions`

List all available versions of a package.

#### Response

```json
{
  "name": "my-package",
  "versions": [
    {
      "version": "2.0.0",
      "published_at": "2024-01-15T10:30:00Z",
      "yanked": false
    },
    {
      "version": "1.2.3",
      "published_at": "2024-01-10T09:15:00Z",
      "yanked": false
    },
    {
      "version": "1.2.2",
      "published_at": "2024-01-05T14:45:00Z",
      "yanked": true
    }
  ]
}
```

## Package Archive Format

### Supported Formats
- **tar.gz** (preferred): Gzipped tar archive
- **zip**: ZIP archive

### Archive Structure
```
package-name-version/
├── CursedPackage.toml    # Package metadata
├── README.md             # Package documentation
├── LICENSE              # License file
├── src/                 # Source code
│   ├── main.csd
│   └── lib.csd
├── examples/            # Example code (optional)
│   └── basic.csd
├── tests/              # Test files (optional)
│   └── test_main.csd
└── docs/               # Documentation (optional)
    └── api.md
```

### Package Metadata (CursedPackage.toml)

```toml
[package]
name = "my-package"
version = "1.2.3"
description = "A useful CURSED package"
authors = ["John Doe <john@example.com>"]
license = "MIT"
repository = "https://github.com/example/my-package"
keywords = ["utility", "helper"]
categories = ["development-tools"]

[dependencies]
other-package = "^2.0.0"
math-utils = "1.5"

[dev-dependencies]
test-framework = "^1.0.0"
```

## Client Configuration

### HTTP Headers

All requests should include these headers:

```
User-Agent: cursed-pkg/0.1.0
Accept: application/json
Content-Type: application/json (for POST/PUT requests)
X-Registry-Client: cursed-pkg
```

### Timeout and Retry Strategy

- **Connection Timeout**: 10 seconds
- **Read Timeout**: 30 seconds
- **Retry Strategy**: Exponential backoff (1s, 2s, 4s)
- **Max Retries**: 3 attempts

### Rate Limiting

- **Anonymous requests**: 100 requests per hour per IP
- **Authenticated requests**: 1000 requests per hour per token
- **Download requests**: 50 downloads per hour per IP

Rate limit headers:
```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 950
X-RateLimit-Reset: 1642680000
```

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "PACKAGE_NOT_FOUND",
    "message": "Package 'nonexistent-package' not found",
    "details": {
      "package": "nonexistent-package",
      "suggestions": ["similar-package", "another-package"]
    }
  }
}
```

### Common Error Codes

- `PACKAGE_NOT_FOUND`: Requested package does not exist
- `VERSION_NOT_FOUND`: Requested package version does not exist
- `INVALID_PACKAGE_NAME`: Package name format is invalid
- `AUTHENTICATION_REQUIRED`: Request requires authentication
- `RATE_LIMIT_EXCEEDED`: Too many requests
- `VALIDATION_ERROR`: Request validation failed
- `INTERNAL_SERVER_ERROR`: Server-side error

### HTTP Status Codes

- `200 OK`: Request successful
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request parameters
- `401 Unauthorized`: Authentication required
- `403 Forbidden`: Access denied
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource conflict (e.g., package already exists)
- `410 Gone`: Resource has been permanently removed
- `422 Unprocessable Entity`: Request validation failed
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error
- `502 Bad Gateway`: Upstream service error
- `503 Service Unavailable`: Service temporarily unavailable

## Security Considerations

### HTTPS Requirements
- All API requests must use HTTPS
- TLS 1.2 or higher required
- Certificate validation enforced by default

### Input Validation
- Package names: alphanumeric, hyphens, underscores only
- Version strings: must follow semantic versioning
- Query strings: URL-encoded and length-limited

### Checksum Verification
- All package downloads include SHA-256 checksums
- Clients must verify checksums before installation
- Checksums are calculated server-side during upload

### Content Security
- Package archives are scanned for malicious content
- Binary executables are not allowed in packages
- File size limits: 100MB per package

## Client Implementation Example

```rust
use cursed::package_manager::registry::{PackageRegistry, RegistryConfig};
use std::time::Duration;

// Create registry client
let config = RegistryConfig {
    base_url: "https://packages.cursed-lang.org".to_string(),
    timeout: Duration::from_secs(30),
    max_retries: 3,
    auth_token: Some("your-token".to_string()),
    user_agent: "cursed-pkg/0.1.0".to_string(),
    verify_tls: true,
};

let mut registry = PackageRegistry::with_config(config)?;

// Search for packages
let packages = registry.search_packages("utility", Some(10)).await?;

// Get package info
let package_info = registry.search_package("my-package", Some("1.2.3")).await?;

// Download package
let package_data = registry.download_package("my-package", "1.2.3").await?;

// Verify integrity
let verified = registry.verify_package(
    "my-package",
    "1.2.3",
    &package_data.content,
    &package_data.checksum
).await?;
```

## Registry Implementation Guidelines

### For Registry Operators

1. **Implement all required endpoints** with proper error handling
2. **Use semantic versioning** for package versions
3. **Provide reliable checksums** for all packages
4. **Implement rate limiting** to prevent abuse
5. **Use HTTPS** for all communications
6. **Validate package contents** before accepting uploads
7. **Provide comprehensive search** functionality
8. **Monitor and log** all API requests for debugging

### For Client Developers

1. **Always verify checksums** after downloading packages
2. **Implement proper retry logic** with exponential backoff
3. **Handle all error conditions** gracefully
4. **Use appropriate timeouts** to prevent hanging
5. **Include proper User-Agent** headers
6. **Cache responses** where appropriate
7. **Respect rate limits** and handle 429 responses
8. **Log requests and errors** for debugging

## Future Enhancements

- **GraphQL API**: Alternative to REST API for complex queries
- **WebSocket Support**: Real-time package updates and notifications
- **Batch Operations**: Upload/download multiple packages in one request
- **Package Signing**: Digital signatures for package authenticity
- **Dependency Analysis**: Advanced dependency resolution APIs
- **Package Statistics**: Download counts, popularity metrics
- **Package Mirroring**: Support for distributed registry networks

This API specification provides a solid foundation for CURSED package management while allowing for future extensibility and enhancement.
