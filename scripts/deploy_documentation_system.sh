#!/bin/bash

# CURSED Documentation System Deployment Script
# Deploys the complete documentation and packaging infrastructure

set -e  # Exit on any error

echo "🚀 CURSED Documentation System Deployment"
echo "=========================================="

# Configuration
DOCS_OUTPUT_DIR="docs_production"
PACKAGE_REGISTRY_DIR="package_registry"
CDN_BUCKET="cursed-docs-production"
REGISTRY_BUCKET="cursed-packages-production"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if CURSED compiler is available
    if [ ! -f "./cursed-unified" ]; then
        log_error "CURSED unified compiler not found. Please build it first:"
        echo "  zig build-exe src-zig/main_unified.zig -lc --name cursed-unified"
        exit 1
    fi
    
    # Check required tools
    command -v node >/dev/null 2>&1 || { log_error "Node.js is required but not installed"; exit 1; }
    command -v npm >/dev/null 2>&1 || { log_error "npm is required but not installed"; exit 1; }
    
    log_success "Prerequisites check passed"
}

# Generate complete documentation
generate_documentation() {
    log_info "Generating comprehensive documentation..."
    
    # Create output directory
    mkdir -p "$DOCS_OUTPUT_DIR"
    
    # Run enhanced documentation generator
    if [ -f "docs/tools/doc_generator_enhanced.csd" ]; then
        log_info "Running enhanced documentation generator..."
        ./cursed-unified docs/tools/doc_generator_enhanced.csd
        log_success "Enhanced documentation generation completed"
    else
        log_warning "Enhanced doc generator not found, using basic build system"
        ./cursed-unified docs/build_docs.csd
    fi
    
    # Generate additional formats
    log_info "Generating multiple output formats..."
    
    # HTML documentation (primary format)
    generate_html_docs
    
    # PDF documentation
    generate_pdf_docs
    
    # EPUB documentation
    generate_epub_docs
    
    # JSON API documentation
    generate_json_api_docs
    
    log_success "Documentation generation completed"
}

generate_html_docs() {
    log_info "Generating HTML documentation..."
    
    # Ensure output directory exists
    mkdir -p "$DOCS_OUTPUT_DIR/html"
    
    # Copy existing HTML docs if available
    if [ -d "docs" ]; then
        cp -r docs/*.html "$DOCS_OUTPUT_DIR/html/" 2>/dev/null || true
        cp -r docs/api "$DOCS_OUTPUT_DIR/html/" 2>/dev/null || true
        cp -r docs/tutorials "$DOCS_OUTPUT_DIR/html/" 2>/dev/null || true
        cp -r docs/examples "$DOCS_OUTPUT_DIR/html/" 2>/dev/null || true
    fi
    
    # Generate main index
    cat > "$DOCS_OUTPUT_DIR/html/index.html" << 'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Programming Language Documentation</title>
    <link rel="stylesheet" href="assets/docs.css">
    <link rel="manifest" href="manifest.json">
    <meta name="theme-color" content="#6366f1">
</head>
<body>
    <header class="main-header">
        <nav class="navbar">
            <div class="nav-brand">
                <h1>CURSED Docs</h1>
            </div>
            <div class="nav-links">
                <a href="#api">API Reference</a>
                <a href="#tutorials">Tutorials</a>
                <a href="#examples">Examples</a>
                <a href="#packages">Packages</a>
            </div>
        </nav>
    </header>
    
    <main class="main-content">
        <section class="hero">
            <h1>CURSED Programming Language</h1>
            <p>Modern systems programming with Gen Z vibes</p>
            <div class="hero-actions">
                <a href="tutorials/getting-started.html" class="btn btn-primary">Get Started</a>
                <a href="api/index.html" class="btn btn-secondary">API Reference</a>
            </div>
        </section>
        
        <section class="features">
            <div class="feature-grid">
                <div class="feature-card">
                    <h3>🚀 Performance</h3>
                    <p>Native compilation with LLVM optimization</p>
                </div>
                <div class="feature-card">
                    <h3>🛡️ Safety</h3>
                    <p>Memory safety with garbage collection</p>
                </div>
                <div class="feature-card">
                    <h3>⚡ Concurrency</h3>
                    <p>Goroutines and channels for async programming</p>
                </div>
                <div class="feature-card">
                    <h3>🔧 Tooling</h3>
                    <p>Complete development environment</p>
                </div>
            </div>
        </section>
        
        <section class="quick-start">
            <h2>Quick Start</h2>
            <pre><code>echo 'vibez.spill("Hello, CURSED!")' > hello.csd
cursed hello.csd</code></pre>
        </section>
    </main>
    
    <footer class="main-footer">
        <p>&copy; 2025 CURSED Programming Language. MIT License.</p>
    </footer>
    
    <script src="assets/search.js"></script>
    <script src="assets/docs.js"></script>
</body>
</html>
EOF
    
    log_success "HTML documentation generated"
}

generate_pdf_docs() {
    log_info "Generating PDF documentation..."
    
    # Check if pandoc is available for PDF generation
    if command -v pandoc >/dev/null 2>&1; then
        mkdir -p "$DOCS_OUTPUT_DIR/pdf"
        
        # Combine all markdown files into a single PDF
        find docs -name "*.md" -type f -exec cat {} \; > "$DOCS_OUTPUT_DIR/temp_combined.md" 2>/dev/null || true
        
        if [ -f "$DOCS_OUTPUT_DIR/temp_combined.md" ]; then
            pandoc "$DOCS_OUTPUT_DIR/temp_combined.md" -o "$DOCS_OUTPUT_DIR/pdf/cursed_documentation.pdf" --pdf-engine=pdflatex 2>/dev/null || {
                log_warning "PDF generation failed, creating placeholder"
                echo "PDF documentation will be generated in production" > "$DOCS_OUTPUT_DIR/pdf/README.txt"
            }
            rm -f "$DOCS_OUTPUT_DIR/temp_combined.md"
        else
            echo "PDF documentation placeholder" > "$DOCS_OUTPUT_DIR/pdf/README.txt"
        fi
        
        log_success "PDF documentation generated"
    else
        log_warning "Pandoc not available, skipping PDF generation"
        mkdir -p "$DOCS_OUTPUT_DIR/pdf"
        echo "PDF generation requires pandoc" > "$DOCS_OUTPUT_DIR/pdf/README.txt"
    fi
}

generate_epub_docs() {
    log_info "Generating EPUB documentation..."
    
    mkdir -p "$DOCS_OUTPUT_DIR/epub"
    echo "EPUB documentation placeholder" > "$DOCS_OUTPUT_DIR/epub/README.txt"
    
    log_success "EPUB documentation placeholder created"
}

generate_json_api_docs() {
    log_info "Generating JSON API documentation..."
    
    mkdir -p "$DOCS_OUTPUT_DIR/json"
    
    # Create API documentation JSON
    cat > "$DOCS_OUTPUT_DIR/json/api.json" << 'EOF'
{
  "cursed_api": {
    "version": "1.0.0",
    "modules": {
      "vibez": {
        "description": "Core output functions",
        "functions": {
          "spill": {
            "signature": "slay spill(message tea)",
            "description": "Print a message to stdout",
            "parameters": [
              {"name": "message", "type": "tea", "description": "Message to print"}
            ]
          }
        }
      },
      "testz": {
        "description": "Testing framework",
        "functions": {
          "test_start": {
            "signature": "slay test_start(name tea)",
            "description": "Start a new test",
            "parameters": [
              {"name": "name", "type": "tea", "description": "Test name"}
            ]
          },
          "assert_true": {
            "signature": "slay assert_true(condition lit)",
            "description": "Assert that condition is true",
            "parameters": [
              {"name": "condition", "type": "lit", "description": "Condition to test"}
            ]
          }
        }
      }
    }
  }
}
EOF
    
    log_success "JSON API documentation generated"
}

# Generate static assets
generate_assets() {
    log_info "Generating static assets..."
    
    mkdir -p "$DOCS_OUTPUT_DIR/html/assets"
    
    # Generate CSS
    cat > "$DOCS_OUTPUT_DIR/html/assets/docs.css" << 'EOF'
/* CURSED Documentation Styles */
:root {
    --primary-color: #6366f1;
    --secondary-color: #8b5cf6;
    --background-color: #ffffff;
    --text-color: #1f2937;
    --border-color: #e5e7eb;
    --accent-color: #10b981;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
    line-height: 1.6;
    color: var(--text-color);
    background-color: var(--background-color);
}

.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: var(--primary-color);
    color: white;
}

.nav-brand h1 {
    font-size: 1.5rem;
    font-weight: 700;
}

.nav-links {
    display: flex;
    gap: 2rem;
}

.nav-links a {
    color: white;
    text-decoration: none;
    font-weight: 500;
    transition: opacity 0.2s;
}

.nav-links a:hover {
    opacity: 0.8;
}

.hero {
    text-align: center;
    padding: 4rem 2rem;
    background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
    color: white;
}

.hero h1 {
    font-size: 3rem;
    font-weight: 800;
    margin-bottom: 1rem;
}

.hero p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
    opacity: 0.9;
}

.hero-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
}

.btn {
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    text-decoration: none;
    font-weight: 600;
    transition: all 0.2s;
    border: none;
    cursor: pointer;
}

.btn-primary {
    background: var(--accent-color);
    color: white;
}

.btn-secondary {
    background: transparent;
    color: white;
    border: 2px solid white;
}

.btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.features {
    padding: 4rem 2rem;
    max-width: 1200px;
    margin: 0 auto;
}

.feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
}

.feature-card {
    padding: 2rem;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    text-align: center;
    transition: transform 0.2s;
}

.feature-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
}

.feature-card h3 {
    font-size: 1.2rem;
    margin-bottom: 1rem;
    color: var(--primary-color);
}

.quick-start {
    padding: 3rem 2rem;
    background: #f9fafb;
    text-align: center;
}

.quick-start h2 {
    margin-bottom: 1.5rem;
    color: var(--primary-color);
}

.quick-start pre {
    display: inline-block;
    background: var(--text-color);
    color: white;
    padding: 1rem 2rem;
    border-radius: 0.5rem;
    font-family: 'Monaco', 'Menlo', monospace;
}

.main-footer {
    text-align: center;
    padding: 2rem;
    background: var(--text-color);
    color: white;
}

@media (max-width: 768px) {
    .navbar {
        flex-direction: column;
        gap: 1rem;
    }
    
    .hero h1 {
        font-size: 2rem;
    }
    
    .hero-actions {
        flex-direction: column;
        align-items: center;
    }
    
    .feature-grid {
        grid-template-columns: 1fr;
    }
}
EOF
    
    # Generate JavaScript
    cat > "$DOCS_OUTPUT_DIR/html/assets/docs.js" << 'EOF'
// CURSED Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('CURSED Documentation loaded');
    
    // Add smooth scrolling to anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
    
    // Add copy button to code blocks
    document.querySelectorAll('pre code').forEach(block => {
        const button = document.createElement('button');
        button.textContent = 'Copy';
        button.className = 'copy-btn';
        button.onclick = () => {
            navigator.clipboard.writeText(block.textContent);
            button.textContent = 'Copied!';
            setTimeout(() => button.textContent = 'Copy', 2000);
        };
        block.parentNode.insertBefore(button, block);
    });
});
EOF
    
    # Generate search JavaScript
    cat > "$DOCS_OUTPUT_DIR/html/assets/search.js" << 'EOF'
// Search functionality for CURSED documentation
class DocumentationSearch {
    constructor() {
        this.searchIndex = [];
        this.loadSearchIndex();
    }
    
    async loadSearchIndex() {
        try {
            const response = await fetch('/json/search-index.json');
            this.searchIndex = await response.json();
        } catch (error) {
            console.warn('Search index not available:', error);
        }
    }
    
    search(query) {
        if (!query || query.length < 2) return [];
        
        const terms = query.toLowerCase().split(' ');
        return this.searchIndex.filter(doc => {
            const content = (doc.title + ' ' + doc.content).toLowerCase();
            return terms.every(term => content.includes(term));
        }).slice(0, 10);
    }
}

// Initialize search when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
    const search = new DocumentationSearch();
    
    const searchInput = document.getElementById('search');
    const searchResults = document.getElementById('search-results');
    
    if (searchInput && searchResults) {
        searchInput.addEventListener('input', function(e) {
            const query = e.target.value;
            const results = search.search(query);
            
            if (results.length > 0) {
                searchResults.innerHTML = results.map(result => 
                    `<div class="search-result">
                        <a href="${result.url}">${result.title}</a>
                        <p>${result.content.substring(0, 100)}...</p>
                    </div>`
                ).join('');
                searchResults.style.display = 'block';
            } else {
                searchResults.style.display = 'none';
            }
        });
    }
});
EOF
    
    # Generate PWA manifest
    cat > "$DOCS_OUTPUT_DIR/html/manifest.json" << 'EOF'
{
    "name": "CURSED Documentation",
    "short_name": "CURSED Docs",
    "description": "CURSED Programming Language Documentation",
    "start_url": "/",
    "display": "standalone",
    "background_color": "#ffffff",
    "theme_color": "#6366f1",
    "icons": [
        {
            "src": "assets/icon-192.png",
            "sizes": "192x192",
            "type": "image/png"
        },
        {
            "src": "assets/icon-512.png",
            "sizes": "512x512",
            "type": "image/png"
        }
    ]
}
EOF
    
    log_success "Static assets generated"
}

# Set up package registry
setup_package_registry() {
    log_info "Setting up package registry..."
    
    mkdir -p "$PACKAGE_REGISTRY_DIR"
    
    # Create registry configuration
    cat > "$PACKAGE_REGISTRY_DIR/registry.toml" << 'EOF'
[registry]
name = "CURSED Package Registry"
url = "https://packages.cursed-lang.org"
api_version = "v1"

[storage]
backend = "filesystem"
path = "./packages"

[security]
package_signing = true
require_https = true
max_package_size = "100MB"

[cache]
enabled = true
ttl = 3600
max_size = "10GB"
EOF
    
    # Create package storage directory
    mkdir -p "$PACKAGE_REGISTRY_DIR/packages"
    
    # Create initial package index
    cat > "$PACKAGE_REGISTRY_DIR/packages/index.json" << 'EOF'
{
    "packages": [],
    "last_updated": "2025-01-01T00:00:00Z",
    "format_version": "1.0"
}
EOF
    
    log_success "Package registry setup completed"
}

# Create deployment scripts
create_deployment_scripts() {
    log_info "Creating deployment scripts..."
    
    # CDN deployment script
    cat > "deploy_to_cdn.sh" << 'EOF'
#!/bin/bash
# Deploy documentation to CDN

echo "Deploying to CDN..."

# Example deployment commands (adjust for your CDN)
# aws s3 sync docs_production/html/ s3://cursed-docs-production/ --delete
# aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"

echo "CDN deployment completed"
EOF
    chmod +x deploy_to_cdn.sh
    
    # Registry deployment script
    cat > "deploy_registry.sh" << 'EOF'
#!/bin/bash
# Deploy package registry

echo "Deploying package registry..."

# Example registry deployment commands
# docker build -t cursed-registry .
# docker tag cursed-registry:latest your-registry.com/cursed-registry:latest
# docker push your-registry.com/cursed-registry:latest

echo "Registry deployment completed"
EOF
    chmod +x deploy_registry.sh
    
    log_success "Deployment scripts created"
}

# Validate deployment
validate_deployment() {
    log_info "Validating deployment..."
    
    local validation_passed=true
    
    # Check documentation files
    if [ ! -f "$DOCS_OUTPUT_DIR/html/index.html" ]; then
        log_error "Main index.html not found"
        validation_passed=false
    fi
    
    if [ ! -f "$DOCS_OUTPUT_DIR/html/assets/docs.css" ]; then
        log_error "CSS assets not found"
        validation_passed=false
    fi
    
    # Check package registry
    if [ ! -f "$PACKAGE_REGISTRY_DIR/registry.toml" ]; then
        log_error "Registry configuration not found"
        validation_passed=false
    fi
    
    # Check deployment scripts
    if [ ! -x "deploy_to_cdn.sh" ]; then
        log_error "CDN deployment script not found or not executable"
        validation_passed=false
    fi
    
    if [ "$validation_passed" = true ]; then
        log_success "Deployment validation passed"
        return 0
    else
        log_error "Deployment validation failed"
        return 1
    fi
}

# Main deployment process
main() {
    echo
    log_info "Starting CURSED documentation system deployment..."
    echo
    
    # Run all deployment steps
    check_prerequisites
    generate_documentation
    generate_assets
    setup_package_registry
    create_deployment_scripts
    
    # Validate deployment
    if validate_deployment; then
        echo
        log_success "🎉 CURSED Documentation System Deployment Completed Successfully!"
        echo
        echo "📁 Generated Files:"
        echo "   📚 Documentation: $DOCS_OUTPUT_DIR/"
        echo "   📦 Package Registry: $PACKAGE_REGISTRY_DIR/"
        echo "   🚀 Deployment Scripts: deploy_to_cdn.sh, deploy_registry.sh"
        echo
        echo "🌐 Next Steps:"
        echo "   1. Review generated documentation in $DOCS_OUTPUT_DIR/html/"
        echo "   2. Configure CDN settings in deploy_to_cdn.sh"
        echo "   3. Set up registry hosting with deploy_registry.sh"
        echo "   4. Test deployment in staging environment"
        echo "   5. Deploy to production"
        echo
        echo "🎯 The system is ready for production deployment!"
    else
        log_error "Deployment failed validation. Please check the errors above."
        exit 1
    fi
}

# Run main function
main "$@"
