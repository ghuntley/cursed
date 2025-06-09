# Package Registry Protocol Specification 🌐

This document specifies the HTTP-based protocol used for communication between CURSED package managers and package registries. It defines the API endpoints, data formats, authentication mechanisms, and security requirements for implementing compatible registries.

## Protocol Overview 📋

### Design Principles

- **RESTful API**: Standard HTTP methods and status codes
- **JSON Communication**: All payloads use JSON format
- **Semantic Versioning**: Follows semver for version handling
- **Security First**: Authentication, integrity verification, and audit trails
- **Caching Friendly**: Supports HTTP caching mechanisms
- **Extensible**: Forward-compatible design for future features

### Protocol Version

Current specification version: **v1.0**

All requests must include the protocol version in the `Accept` header:
```
Accept: application/vnd.cursed-registry.v1+json
```

### Base URL Structure

Registry endpoints follow this pattern:
```
https://registry.example.com/api/v1/{endpoint}
```

## Authentication 🔐

### Authentication Methods

#### 1. Token-Based Authentication

Primary authentication method using Bearer tokens:

```http
Authorization: Bearer {token}
```

#### 2. API Key Authentication

Alternative for programmatic access:

```http
X-API-Key: {api-key}
```

### Token Management

#### Obtain Token
```http
POST /api/v1/auth/login
Content-Type: application/json

{
    "username": "user@example.com",
    "password": "secure-password"
}
```

**Response:**
```json
{
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-12-31T23:59:59Z",
    "scope": ["read", "write", "publish"]
}
```

#### Refresh Token
```http
POST /api/v1/auth/refresh
Authorization: Bearer {current-token}
```

#### Revoke Token
```http
DELETE /api/v1/auth/token
Authorization: Bearer {token}
```

### Scopes and Permissions

| Scope | Permissions |
|-------|-------------|
| `read` | Search packages, download packages, read metadata |
| `write` | Publish packages, update package metadata |
| `admin` | User management, registry administration |
| `audit` | Access audit logs and security information |

## Package Search API 🔍

### Search Packages

```http
GET /api/v1/packages/search?q={query}&limit={limit}&offset={offset}
```

**Parameters:**
- `q` (string): Search query
- `limit` (integer, optional): Maximum results (default: 20, max: 100)
- `offset` (integer, optional): Result offset for pagination (default: 0)
- `category` (string, optional): Filter by category
- `sort` (string, optional): Sort order (`relevance`, `downloads`, `updated`, `name`)

**Example Request:**
```http
GET /api/v1/packages/search?q=json&limit=10&category=parsing&sort=downloads
Accept: application/vnd.cursed-registry.v1+json
```

**Response:**
```json
{
    "packages": [
        {
            "name": "json_parser",
            "latest_version": "2.1.0",
            "description": "Fast and safe JSON parser for CURSED",
            "authors": ["Alice Smith <alice@example.com>"],
            "license": "MIT",
            "homepage": "https://github.com/example/json_parser",
            "repository": "https://github.com/example/json_parser",
            "documentation": "https://docs.rs/json_parser",
            "keywords": ["json", "parser", "serde"],
            "categories": ["parsing", "data-structures"],
            "downloads": 15420,
            "updated_at": "2024-01-15T10:30:00Z",
            "created_at": "2023-06-01T09:00:00Z"
        }
    ],
    "total": 1,
    "limit": 10,
    "offset": 0,
    "next": null
}
```

### Get Package Information

```http
GET /api/v1/packages/{name}
```

**Response:**
```json
{
    "name": "json_parser",
    "description": "Fast and safe JSON parser for CURSED",
    "latest_version": "2.1.0",
    "authors": ["Alice Smith <alice@example.com>"],
    "license": "MIT OR Apache-2.0",
    "homepage": "https://github.com/example/json_parser",
    "repository": "https://github.com/example/json_parser",
    "documentation": "https://docs.rs/json_parser",
    "keywords": ["json", "parser", "serde"],
    "categories": ["parsing", "data-structures"],
    "versions": [
        {
            "version": "2.1.0",
            "published_at": "2024-01-15T10:30:00Z",
            "yanked": false,
            "checksum": "sha256:a1b2c3d4e5f6...",
            "download_url": "https://registry.example.com/api/v1/packages/json_parser/2.1.0/download",
            "dependencies": {
                "string_utils": "^1.0.0",
                "error_handling": "^2.1.0"
            },
            "features": ["default", "serde", "async"],
            "targets": ["lib"]
        }
    ],
    "downloads": 15420,
    "created_at": "2023-06-01T09:00:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
}
```

### Get Specific Version

```http
GET /api/v1/packages/{name}/{version}
```

**Response:**
```json
{
    "name": "json_parser",
    "version": "2.1.0",
    "authors": ["Alice Smith <alice@example.com>"],
    "description": "Fast and safe JSON parser for CURSED",
    "license": "MIT OR Apache-2.0",
    "homepage": "https://github.com/example/json_parser",
    "repository": "https://github.com/example/json_parser",
    "documentation": "https://docs.rs/json_parser/2.1.0",
    "keywords": ["json", "parser", "serde"],
    "categories": ["parsing", "data-structures"],
    "published_at": "2024-01-15T10:30:00Z",
    "yanked": false,
    "checksum": "sha256:a1b2c3d4e5f6789...",
    "download_url": "https://registry.example.com/api/v1/packages/json_parser/2.1.0/download",
    "dependencies": {
        "runtime": {
            "string_utils": {
                "version": "^1.0.0",
                "features": ["default"],
                "optional": false
            }
        },
        "dev": {
            "test_framework": {
                "version": "^2.0.0",
                "features": [],
                "optional": false
            }
        }
    },
    "features": {
        "default": ["serde"],
        "serde": ["serde_dep"],
        "async": ["tokio"]
    },
    "targets": [
        {
            "kind": "lib",
            "name": "json_parser"
        }
    ],
    "manifest": {
        // Raw CursedPackage.toml content
    }
}
```

## Package Download API 📦

### Download Package Archive

```http
GET /api/v1/packages/{name}/{version}/download
```

**Response:**
- **Content-Type**: `application/gzip`
- **Content-Disposition**: `attachment; filename="{name}-{version}.csd"`
- **X-Checksum**: `sha256:{checksum}`
- **X-Content-Length**: `{size-in-bytes}`

The response body contains the gzipped tar archive of the package.

### Verify Package Integrity

```http
HEAD /api/v1/packages/{name}/{version}/download
```

**Response Headers:**
```http
HTTP/1.1 200 OK
Content-Length: 12345
Content-Type: application/gzip
X-Checksum: sha256:a1b2c3d4e5f6789...
X-Published-At: 2024-01-15T10:30:00Z
Cache-Control: public, max-age=31536000
```

## Package Publishing API 📤

### Publish New Package

```http
POST /api/v1/packages/new
Authorization: Bearer {token}
Content-Type: multipart/form-data

--boundary123
Content-Disposition: form-data; name="metadata"
Content-Type: application/json

{
    "name": "my_package",
    "version": "1.0.0",
    "authors": ["Your Name <you@example.com>"],
    "description": "An awesome CURSED package",
    "license": "MIT",
    "repository": "https://github.com/user/my_package",
    "keywords": ["utility", "helper"],
    "categories": ["development-tools"]
}

--boundary123
Content-Disposition: form-data; name="package"; filename="my_package-1.0.0.csd"
Content-Type: application/gzip

{binary package data}

--boundary123--
```

**Response (Success):**
```json
{
    "message": "Package published successfully",
    "package": {
        "name": "my_package",
        "version": "1.0.0",
        "published_at": "2024-01-15T14:30:00Z",
        "download_url": "https://registry.example.com/api/v1/packages/my_package/1.0.0/download"
    }
}
```

**Response (Error):**
```json
{
    "error": "validation_failed",
    "message": "Package validation failed",
    "details": [
        {
            "field": "version",
            "message": "Version 1.0.0 already exists"
        },
        {
            "field": "license",
            "message": "Invalid license identifier"
        }
    ]
}
```

### Update Package Metadata

```http
PATCH /api/v1/packages/{name}/{version}
Authorization: Bearer {token}
Content-Type: application/json

{
    "description": "Updated description",
    "homepage": "https://new-homepage.com",
    "keywords": ["updated", "keywords"]
}
```

## Package Management API 🛠️

### Yank Package Version

```http
DELETE /api/v1/packages/{name}/{version}/yank
Authorization: Bearer {token}
Content-Type: application/json

{
    "reason": "Critical security vulnerability"
}
```

**Response:**
```json
{
    "message": "Version 1.2.3 has been yanked",
    "yanked_at": "2024-01-15T15:00:00Z",
    "reason": "Critical security vulnerability"
}
```

### Unyank Package Version

```http
POST /api/v1/packages/{name}/{version}/unyank
Authorization: Bearer {token}
```

### Transfer Package Ownership

```http
POST /api/v1/packages/{name}/owners
Authorization: Bearer {token}
Content-Type: application/json

{
    "new_owner": "newowner@example.com",
    "permissions": ["publish", "yank", "admin"]
}
```

## Package Statistics API 📊

### Get Download Statistics

```http
GET /api/v1/packages/{name}/stats/downloads?period={period}
```

**Parameters:**
- `period`: `day`, `week`, `month`, `year`, `all`

**Response:**
```json
{
    "package": "json_parser",
    "period": "month",
    "total_downloads": 1542,
    "downloads_by_version": {
        "2.1.0": 1200,
        "2.0.0": 300,
        "1.9.0": 42
    },
    "downloads_by_day": [
        {"date": "2024-01-01", "downloads": 45},
        {"date": "2024-01-02", "downloads": 52}
    ]
}
```

### Get Dependency Information

```http
GET /api/v1/packages/{name}/dependencies?reverse={boolean}
```

**Parameters:**
- `reverse` (boolean): Show packages that depend on this package

## Registry Metadata API ℹ️

### Get Registry Information

```http
GET /api/v1/registry/info
```

**Response:**
```json
{
    "name": "Official CURSED Registry",
    "version": "1.0.0",
    "api_version": "v1",
    "supported_features": [
        "search",
        "download",
        "publish",
        "yank",
        "statistics",
        "audit"
    ],
    "limits": {
        "max_package_size": "50MB",
        "max_packages_per_user": 1000,
        "rate_limit": {
            "requests_per_minute": 1000,
            "downloads_per_minute": 100
        }
    },
    "policies": {
        "package_name_policy": "https://registry.example.com/policies/naming",
        "content_policy": "https://registry.example.com/policies/content",
        "security_policy": "https://registry.example.com/policies/security"
    }
}
```

### Get API Status

```http
GET /api/v1/registry/status
```

**Response:**
```json
{
    "status": "operational",
    "version": "1.0.0",
    "uptime": "99.9%",
    "services": {
        "search": "operational",
        "download": "operational",
        "publish": "operational",
        "authentication": "operational"
    },
    "last_updated": "2024-01-15T16:00:00Z"
}
```

## User Management API 👤

### Get User Profile

```http
GET /api/v1/users/me
Authorization: Bearer {token}
```

**Response:**
```json
{
    "username": "alice",
    "email": "alice@example.com",
    "name": "Alice Smith",
    "avatar_url": "https://avatars.example.com/alice",
    "packages": ["json_parser", "web_framework"],
    "created_at": "2023-01-01T00:00:00Z",
    "permissions": ["read", "write", "publish"]
}
```

### List User Packages

```http
GET /api/v1/users/{username}/packages
```

### Get User Statistics

```http
GET /api/v1/users/me/stats
Authorization: Bearer {token}
```

## Error Handling 🚨

### HTTP Status Codes

| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | Successful request |
| 201 | Created | Package published successfully |
| 400 | Bad Request | Invalid request format or parameters |
| 401 | Unauthorized | Authentication required or invalid |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Package or version not found |
| 409 | Conflict | Version already exists, naming conflict |
| 413 | Payload Too Large | Package exceeds size limit |
| 422 | Unprocessable Entity | Validation failed |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |
| 503 | Service Unavailable | Registry maintenance |

### Error Response Format

```json
{
    "error": "error_code",
    "message": "Human-readable error message",
    "details": [
        {
            "field": "version",
            "code": "already_exists",
            "message": "Version 1.0.0 already exists for this package"
        }
    ],
    "documentation_url": "https://docs.registry.example.com/errors/validation_failed",
    "request_id": "req_12345"
}
```

### Common Error Codes

| Code | Description |
|------|-------------|
| `invalid_token` | Authentication token is invalid or expired |
| `insufficient_permissions` | User lacks required permissions |
| `package_not_found` | Requested package does not exist |
| `version_not_found` | Requested version does not exist |
| `validation_failed` | Package validation errors |
| `name_reserved` | Package name is reserved |
| `name_taken` | Package name already exists |
| `version_exists` | Version already published |
| `rate_limit_exceeded` | Too many requests |
| `package_too_large` | Package exceeds size limits |
| `yanked_version` | Requested version is yanked |

## Security Specifications 🔒

### Package Integrity

#### Checksums
All packages must include SHA-256 checksums:
```json
{
    "checksum": "sha256:a1b2c3d4e5f6789abcdef..."
}
```

#### Package Signing
Optional cryptographic signatures using Ed25519:
```json
{
    "signature": {
        "algorithm": "ed25519",
        "public_key": "ed25519:abcdef123456...",
        "signature": "base64-encoded-signature"
    }
}
```

### Content Security Policy

#### Allowed Content
- Source code files (`.csd`, `.md`, `.toml`, etc.)
- Documentation files
- License files
- Build configuration

#### Prohibited Content
- Executable binaries
- Sensitive information (passwords, keys)
- Malicious code
- Copyrighted material without permission

#### Content Scanning
Registries should implement:
- Malware scanning
- License compliance checking
- Vulnerability scanning
- Content policy enforcement

### Audit Trail

All registry operations are logged:
```json
{
    "event_id": "evt_12345",
    "timestamp": "2024-01-15T16:30:00Z",
    "event_type": "package_published",
    "user_id": "user_67890",
    "package": "json_parser",
    "version": "2.1.0",
    "ip_address": "192.168.1.100",
    "user_agent": "cursed-pkg/1.0.0"
}
```

## Rate Limiting 🚦

### Rate Limit Headers

All responses include rate limiting information:
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642345678
X-RateLimit-Window: 3600
```

### Rate Limit Tiers

| Operation | Limit | Window |
|-----------|-------|--------|
| Search API | 1000 requests | 1 hour |
| Download | 100 downloads | 1 minute |
| Publish | 10 publishes | 1 hour |
| Authentication | 5 attempts | 1 minute |

### Rate Limit Exceeded Response

```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1642345678
Retry-After: 3600

{
    "error": "rate_limit_exceeded",
    "message": "Rate limit exceeded. Try again in 1 hour.",
    "retry_after": 3600
}
```

## Caching and CDN 🗄️

### Cache Headers

Packages are immutable and heavily cached:
```http
Cache-Control: public, max-age=31536000, immutable
ETag: "sha256:a1b2c3d4e5f6789..."
Last-Modified: Mon, 15 Jan 2024 10:30:00 GMT
```

### CDN Distribution

Package downloads may be served from CDN:
```http
X-Served-By: cdn-edge-01.example.com
X-Cache: HIT
X-Cache-Age: 3600
```

### Conditional Requests

Support for efficient caching:
```http
GET /api/v1/packages/json_parser/2.1.0/download
If-None-Match: "sha256:a1b2c3d4e5f6789..."

HTTP/1.1 304 Not Modified
```

## Registry Implementation Guidelines 🏗️

### Minimum Requirements

A compliant registry must implement:
- Package search and metadata retrieval
- Package download with integrity verification
- Basic authentication and authorization
- Rate limiting and abuse prevention
- Audit logging

### Recommended Features

- Package signing and verification
- Statistics and analytics
- User management
- Webhook notifications
- Mirror/proxy capabilities

### Performance Requirements

- Search responses < 500ms
- Package downloads < 5s for 10MB packages
- 99.9% uptime SLA
- Global CDN distribution

### Storage Requirements

```
{registry-root}/
├── packages/
│   ├── {name}/
│   │   ├── {version}/
│   │   │   ├── package.csd
│   │   │   ├── metadata.json
│   │   │   └── signature.sig
│   │   └── index.json
│   └── index/
├── users/
├── audit/
└── cache/
```

This comprehensive protocol specification ensures interoperability between CURSED package managers and registries while maintaining security, performance, and reliability standards! 🌐✨

For implementation examples and client libraries, check out the [Package Manager API Documentation](package_manager_api.md) and [Registry Client Implementation Guide](registry_client_implementation.md).
