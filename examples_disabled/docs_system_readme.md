# CURSED Documentation Publishing Infrastructure

A comprehensive documentation system for the CURSED programming language featuring automated publishing, server hosting, registry management, and quality testing.

## Overview

The CURSED documentation infrastructure provides a complete solution for generating, publishing, hosting, and maintaining high-quality documentation for CURSED projects. It includes:

- **Documentation Publisher**: Automated deployment to multiple targets (local, S3, GitHub Pages, custom)
- **Documentation Server**: Production web server with search, analytics, and API endpoints
- **Documentation Registry**: Central package metadata management with cross-reference resolution
- **Documentation Testing**: Automated validation including link checking and example verification
- **CLI Integration**: Comprehensive command-line tools for all documentation operations

## Quick Start

### 1. Setup Infrastructure

```bash
# Setup complete documentation infrastructure
./scripts/setup_docs_infrastructure.sh

# Setup specific components
./scripts/setup_docs_infrastructure.sh --server --port 3000
./scripts/setup_docs_infrastructure.sh --registry --testing
```

### 2. Generate Documentation

```bash
# Generate documentation from source
cursed doc generate --source-dir src --output-dir docs

# Generate with specific format
cursed doc generate --format html --include-private

# Generate multiple formats
cursed doc generate --format html --format markdown --format json
```

### 3. Publish Documentation

```bash
# Publish to local directory
cursed doc publish --target local

# Publish to S3
cursed doc publish --target s3 --config s3_config.toml

# Publish to GitHub Pages
cursed doc publish --target github-pages --base-url https://user.github.io/project

# Dry run before publishing
cursed doc publish --target s3 --dry-run
```

### 4. Start Documentation Server

```bash
# Start server with default configuration
cursed doc serve

# Start with custom configuration
cursed doc serve --bind 0.0.0.0:8080 --document-root ./docs

# Start with HTTPS
cursed doc serve --https --cert cert.pem --key private.key
```

### 5. Test Documentation

```bash
# Run comprehensive tests
cursed doc test --check-links --verify-examples --check-completeness

# Test specific package
cursed doc test --package mypackage --version 1.0.0

# Generate HTML test report
cursed doc test --format html --output test_report.html
```

## Architecture

### Publisher (`src/docs/publisher.rs`)

The documentation publisher handles automated deployment with:

- **Multiple Targets**: Local filesystem, Amazon S3, GitHub Pages, custom endpoints
- **Optimization**: HTML/CSS/JS minification, image optimization, compression
- **CDN Integration**: Cloudflare, CloudFront integration with cache management
- **Performance Tracking**: Build times, upload speeds, propagation metrics

**Configuration Example:**
```toml
[target]
type = "s3"
bucket = "my-docs-bucket"
region = "us-west-2"
prefix = "docs"

base_url = "https://docs.myproject.dev"

[optimization]
minify_html = true
minify_css = true
optimize_images = true
gzip_compression = true
```

### Server (`src/docs/server.rs`)

Production-ready web server featuring:

- **Search API**: Full-text search across all documentation
- **Version Management**: Support for multiple package versions
- **Analytics**: Page views, search queries, download tracking
- **Performance Monitoring**: Response times, error rates, cache metrics
- **Security**: CORS, rate limiting, SSL/TLS support

**API Endpoints:**
- `GET /api/search?q=query` - Search documentation
- `GET /api/versions/{package}` - List package versions
- `GET /api/metrics` - Server performance metrics
- `GET /api/health` - Health check endpoint

### Registry (`src/docs/registry.rs`)

Central metadata management system:

- **Package Tracking**: All published documentation with metadata
- **Cross-References**: Automatic dependency documentation linking
- **Quality Metrics**: Coverage analysis, freshness tracking
- **Search Index**: Efficient full-text search across packages
- **Statistics**: Usage analytics and popular package tracking

**Features:**
- Package dependency resolution
- Documentation quality scoring
- Version compatibility tracking
- Cross-package reference validation

### Testing Infrastructure (`src/docs/testing.rs`)

Automated quality assurance:

- **Link Checking**: Validate all internal and external links
- **Example Verification**: Compile and test code examples
- **Completeness Analysis**: Ensure comprehensive documentation coverage
- **Accessibility Testing**: WCAG compliance validation
- **Performance Testing**: Page load times and optimization verification

**Test Categories:**
- Link health (broken links, redirects, timeouts)
- Example correctness (compilation, execution, output validation)
- Documentation coverage (missing docs, incomplete descriptions)
- Accessibility compliance (alt text, headings, color contrast)

## Configuration

### Publishing Configuration

Create `cursed_publish.toml`:

```toml
# Publishing target
[target]
type = "github-pages"
repo = "username/docs-repo"
branch = "gh-pages"
token = "${GITHUB_TOKEN}"

# Base URL for published docs
base_url = "https://username.github.io/docs-repo"

# CDN configuration
[cdn]
provider = "cloudflare"
domain = "docs.myproject.dev"

# Optimization settings
[optimization]
minify_html = true
minify_css = true
minify_js = true
optimize_images = true
gzip_compression = true

# Custom domain
[domain]
domain = "docs.myproject.dev"
[domain.ssl]
cert_type = "letsencrypt"
```

### Server Configuration

Create `cursed_server.toml`:

```toml
bind_address = "0.0.0.0:8080"
document_root = "./docs"
enable_https = false

[cors_config]
allowed_origins = ["https://myproject.dev"]
allowed_methods = ["GET", "POST", "OPTIONS"]

[rate_limiting]
requests_per_minute = 120
burst_capacity = 20
enabled = true

[search_config]
enabled = true
max_results = 100
full_text_search = true

[analytics_config]
enabled = true
retention_days = 90
track_page_views = true
track_search_queries = true
```

### Testing Configuration

Create `cursed_testing.toml`:

```toml
check_links = true
verify_examples = true
check_completeness = true
check_accessibility = true

request_timeout = 30
max_concurrent_requests = 10
min_coverage_percentage = 80.0

[link_checking]
follow_redirects = true
check_external_links = true
timeout = 30

[example_verification]
compile_examples = true
run_examples = false
sandbox_execution = true

[completeness_analysis]
require_function_docs = true
require_struct_docs = true
min_doc_length = 20

[accessibility_checking]
check_alt_text = true
check_heading_structure = true
wcag_level = "AA"
```

## CLI Commands

### Generate Documentation

```bash
# Basic generation
cursed doc generate

# With options
cursed doc generate \
  --source-dir src \
  --output-dir docs \
  --format html \
  --include-private
```

### Publish Documentation

```bash
# Local publishing
cursed doc publish --target local --base-url file://./docs

# Cloud publishing
cursed doc publish \
  --target s3 \
  --config publish_config.toml \
  --optimize

# Preview before publishing
cursed doc publish --target github-pages --dry-run
```

### Serve Documentation

```bash
# Development server
cursed doc serve --bind 127.0.0.1:3000

# Production server
cursed doc serve \
  --bind 0.0.0.0:8080 \
  --document-root /var/www/docs \
  --config server_config.toml
```

### Test Documentation

```bash
# Full testing
cursed doc test \
  --check-links \
  --verify-examples \
  --check-completeness \
  --check-accessibility

# Output formats
cursed doc test --format html --output report.html
cursed doc test --format json --output results.json
```

### Registry Management

```bash
# Initialize registry
cursed doc registry init --data-dir ./registry

# List packages
cursed doc registry list

# Search packages
cursed doc registry search "http client"

# Show package info
cursed doc registry show mypackage --version 1.0.0

# Clean registry
cursed doc registry clean --old-versions --orphaned
```

### Preview Documentation

```bash
# Local preview with hot reload
cursed doc preview --port 3000 --watch --open

# Preview specific directory
cursed doc preview --source-dir examples --port 8000
```

## Deployment

### Local Development

```bash
# 1. Setup infrastructure
./scripts/setup_docs_infrastructure.sh

# 2. Generate documentation
cursed doc generate --source-dir src

# 3. Start development server
./start_docs_server.sh

# 4. Open browser to http://localhost:8080
```

### Production Deployment

#### Option 1: GitHub Pages

```bash
# 1. Configure GitHub Pages target
cat > cursed_publish.toml << EOF
[target]
type = "github-pages"
repo = "username/docs"
branch = "gh-pages"
token = "\${GITHUB_TOKEN}"
base_url = "https://username.github.io/docs"
EOF

# 2. Deploy
export GITHUB_TOKEN="your-token"
cursed doc publish --config cursed_publish.toml
```

#### Option 2: Amazon S3 + CloudFront

```bash
# 1. Configure S3 target
cat > s3_config.toml << EOF
[target]
type = "s3"
bucket = "my-docs-bucket"
region = "us-west-2"

[cdn]
provider = "cloudfront"
domain = "d123456789.cloudfront.net"
EOF

# 2. Deploy
aws configure  # Setup AWS credentials
cursed doc publish --config s3_config.toml --optimize
```

#### Option 3: Custom Server

```bash
# 1. Setup server
./scripts/setup_docs_infrastructure.sh --server --port 80

# 2. Configure reverse proxy (nginx example)
cat > /etc/nginx/sites-available/docs << EOF
server {
    listen 80;
    server_name docs.myproject.dev;
    
    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
    }
}
EOF

# 3. Start documentation server
sudo systemctl enable cursed-docs
sudo systemctl start cursed-docs
```

### CI/CD Integration

#### GitHub Actions

```yaml
name: Deploy Documentation

on:
  push:
    branches: [main]

jobs:
  deploy-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install CURSED
        run: |
          curl -sSL https://install.cursed.dev | sh
          echo "$HOME/.cursed/bin" >> $GITHUB_PATH
      
      - name: Test Documentation
        run: |
          cursed doc test \
            --check-links \
            --verify-examples \
            --check-completeness
      
      - name: Deploy Documentation
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          cursed doc publish \
            --target github-pages \
            --optimize
```

#### GitLab CI

```yaml
deploy-docs:
  stage: deploy
  image: cursed/cli:latest
  script:
    - cursed doc test --check-links --verify-examples
    - cursed doc publish --target s3 --optimize
  only:
    - main
  artifacts:
    reports:
      junit: test_results.xml
```

## Monitoring and Analytics

### Server Metrics

The documentation server provides comprehensive metrics:

```bash
# Get server metrics
curl http://localhost:8080/api/metrics

# Response includes:
{
  "total_requests": 10000,
  "requests_per_second": 12.5,
  "avg_response_time_ms": 150,
  "error_rate": 0.5,
  "cache_hit_rate": 85.0,
  "uptime_seconds": 86400
}
```

### Analytics Dashboard

Access analytics at `http://your-server/api/analytics`:

- Page view statistics
- Search query analysis
- Popular documentation sections
- User geographic distribution
- Performance trends

### Health Monitoring

Set up monitoring with:

```bash
# Health check endpoint
curl http://localhost:8080/api/health

# Monitoring script
./monitor_docs.sh

# Output:
# ✅ Documentation server is running (port 8080)
# 📦 Registry contains 25 packages  
# 📚 Documentation is available
# 💾 Documentation size: 125MB
```

## Troubleshooting

### Common Issues

**Publishing Fails:**
```bash
# Check configuration
cursed doc validate --config publish_config.toml --config-type publish

# Test with dry run
cursed doc publish --dry-run

# Check permissions and credentials
```

**Server Won't Start:**
```bash
# Check port availability
netstat -tlnp | grep :8080

# Validate configuration  
cursed doc validate --config server_config.toml --config-type server

# Check document root permissions
ls -la /path/to/docs
```

**Tests Failing:**
```bash
# Run with verbose output
cursed doc test --verbose

# Test specific categories
cursed doc test --check-links --output link_report.html

# Check network connectivity for external links
```

### Debug Mode

Enable debug logging:

```bash
export RUST_LOG=debug
cursed doc serve --verbose
```

### Performance Issues

**Slow Documentation Generation:**
```bash
# Use optimization
cursed doc generate --optimize

# Generate specific formats only
cursed doc generate --format html

# Parallel processing
cursed doc generate --jobs 4
```

**Slow Server Response:**
```bash
# Check cache configuration
cursed doc serve --cache-size 256MB

# Enable compression
[cache_config]
enabled = true
static_cache_duration = 86400
```

## Contributing

### Development Setup

```bash
# Clone repository
git clone https://github.com/cursed-lang/cursed
cd cursed

# Setup development environment
./scripts/setup_docs_infrastructure.sh --testing

# Run tests
cargo test docs_
./test_docs.sh
```

### Adding New Features

1. **Publisher Targets**: Add new deployment targets in `src/docs/publisher.rs`
2. **Server Endpoints**: Add API endpoints in `src/docs/server.rs`  
3. **Test Categories**: Add validation types in `src/docs/testing.rs`
4. **CLI Commands**: Extend CLI in `src/cli/docs_enhanced.rs`

### Testing

```bash
# Unit tests
cargo test docs_publisher_test
cargo test docs_server_test

# Integration tests  
cargo test docs_integration_test

# End-to-end tests
./scripts/deploy_docs.sh --dry-run
```

## License

MIT License - see LICENSE file for details.

## Support

- **Documentation**: https://docs.cursed.dev/documentation/
- **Issues**: https://github.com/cursed-lang/cursed/issues
- **Discussions**: https://github.com/cursed-lang/cursed/discussions
- **Discord**: https://discord.gg/cursed-lang

---

For more information about the CURSED programming language, visit [cursed.dev](https://cursed.dev).
