#!/bin/bash
# APT Repository Setup Script for CURSED Compiler
# Enterprise-ready Debian/Ubuntu package management

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

REPO_NAME="cursed"
REPO_DESCRIPTION="CURSED Compiler APT Repository"
REPO_ORIGIN="CURSED Development Team"
REPO_LABEL="CURSED"
REPO_SUITE="stable"
REPO_CODENAME="stable"
REPO_ARCHITECTURES="amd64 arm64"
REPO_COMPONENTS="main"

# Directories
REPO_ROOT="/srv/apt/cursed"
POOL_DIR="$REPO_ROOT/pool"
DISTS_DIR="$REPO_ROOT/dists"
CONF_DIR="$REPO_ROOT/conf"

# GPG configuration
GPG_KEY_ID=""  # Set your GPG key ID
GPG_KEY_EMAIL="dev@cursed.dev"

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [INFO] $*"
}

error() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $*" >&2
    exit 1
}

# ============================================================================
# REPOSITORY SETUP
# ============================================================================

setup_repository_structure() {
    log "Setting up APT repository structure..."
    
    # Create directory structure
    mkdir -p "$REPO_ROOT"/{pool/main/{c,l,m,s},dists/$REPO_CODENAME/{main/{binary-amd64,binary-arm64,source},Release.gpg}}
    mkdir -p "$CONF_DIR"
    
    # Create reprepro configuration
    cat > "$CONF_DIR/distributions" << EOF
Origin: $REPO_ORIGIN
Label: $REPO_LABEL
Suite: $REPO_SUITE
Codename: $REPO_CODENAME
Architectures: $REPO_ARCHITECTURES
Components: $REPO_COMPONENTS
Description: $REPO_DESCRIPTION
SignWith: $GPG_KEY_ID
EOF

    cat > "$CONF_DIR/options" << EOF
verbose
ask-passphrase
basedir $REPO_ROOT
EOF

    # Create incoming directory for new packages
    mkdir -p "$REPO_ROOT/incoming"
    
    log "Repository structure created at: $REPO_ROOT"
}

generate_gpg_key() {
    log "Generating GPG key for package signing..."
    
    if gpg --list-secret-keys | grep -q "$GPG_KEY_EMAIL"; then
        log "GPG key already exists for $GPG_KEY_EMAIL"
        return
    fi
    
    # Generate GPG key for package signing
    cat > /tmp/gpg-gen-key << EOF
%echo Generating a GPG key for CURSED APT repository
Key-Type: RSA
Key-Length: 4096
Subkey-Type: RSA
Subkey-Length: 4096
Name-Real: CURSED Development Team
Name-Email: $GPG_KEY_EMAIL
Expire-Date: 2y
Preferences: SHA512 SHA384 SHA256 AES256 AES192 AES CAST5 ZLIB BZIP2 ZIP Uncompressed
%commit
%echo done
EOF

    gpg --batch --generate-key /tmp/gpg-gen-key
    rm /tmp/gpg-gen-key
    
    # Export public key
    GPG_KEY_ID=$(gpg --list-secret-keys --keyid-format LONG | grep -A1 "sec" | grep -v "sec" | awk '{print $1}' | head -1)
    gpg --armor --export "$GPG_KEY_ID" > "$REPO_ROOT/key.gpg"
    
    log "GPG key generated with ID: $GPG_KEY_ID"
    log "Public key exported to: $REPO_ROOT/key.gpg"
}

create_package_scripts() {
    log "Creating package management scripts..."
    
    # Script to add a package to the repository
    cat > "$REPO_ROOT/add-package.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <package.deb>"
    exit 1
fi

PACKAGE="$1"
REPO_ROOT="$(dirname "$0")"

if [[ ! -f "$PACKAGE" ]]; then
    echo "Error: Package file not found: $PACKAGE"
    exit 1
fi

echo "Adding package to repository: $PACKAGE"

# Use reprepro to add the package
cd "$REPO_ROOT"
reprepro includedeb stable "$PACKAGE"

echo "Package added successfully"
echo "Repository updated"
EOF

    # Script to remove a package from the repository
    cat > "$REPO_ROOT/remove-package.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <package-name>"
    exit 1
fi

PACKAGE_NAME="$1"
REPO_ROOT="$(dirname "$0")"

echo "Removing package from repository: $PACKAGE_NAME"

# Use reprepro to remove the package
cd "$REPO_ROOT"
reprepro remove stable "$PACKAGE_NAME"

echo "Package removed successfully"
echo "Repository updated"
EOF

    # Script to list packages in the repository
    cat > "$REPO_ROOT/list-packages.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

REPO_ROOT="$(dirname "$0")"

echo "Packages in repository:"
echo "======================"

cd "$REPO_ROOT"
reprepro list stable
EOF

    # Script to update repository metadata
    cat > "$REPO_ROOT/update-repository.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

REPO_ROOT="$(dirname "$0")"

echo "Updating repository metadata..."

cd "$REPO_ROOT"
reprepro export stable

echo "Repository metadata updated"
EOF

    # Make scripts executable
    chmod +x "$REPO_ROOT"/*.sh
    
    log "Package management scripts created"
}

create_nginx_config() {
    log "Creating Nginx configuration..."
    
    cat > "$REPO_ROOT/nginx-site.conf" << EOF
server {
    listen 80;
    listen [::]:80;
    server_name packages.cursed.dev;
    
    # Redirect HTTP to HTTPS
    return 301 https://\$server_name\$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name packages.cursed.dev;
    
    # SSL configuration (update with your certificates)
    ssl_certificate /etc/ssl/certs/cursed.dev.crt;
    ssl_certificate_key /etc/ssl/private/cursed.dev.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    
    # Document root
    root $REPO_ROOT;
    index index.html;
    
    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload";
    
    # APT repository files
    location / {
        autoindex on;
        autoindex_exact_size off;
        autoindex_localtime on;
    }
    
    # Specific handling for APT files
    location ~ /(Release|Packages|Sources)(\.gz|\.bz2|\.xz)?$ {
        add_header Cache-Control "no-cache, must-revalidate";
    }
    
    location ~ /\.ht {
        deny all;
    }
    
    # Logging
    access_log /var/log/nginx/cursed-packages-access.log;
    error_log /var/log/nginx/cursed-packages-error.log;
}
EOF
    
    log "Nginx configuration created: $REPO_ROOT/nginx-site.conf"
}

create_client_setup_script() {
    log "Creating client setup script..."
    
    cat > "$REPO_ROOT/install-cursed-repo.sh" << 'EOF'
#!/bin/bash
# Client setup script for CURSED APT repository
set -euo pipefail

REPO_URL="https://packages.cursed.dev"
KEYRING_FILE="/usr/share/keyrings/cursed-archive-keyring.gpg"
SOURCES_FILE="/etc/apt/sources.list.d/cursed.list"

echo "Setting up CURSED APT repository..."

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root (use sudo)"
    exit 1
fi

# Install required packages
apt-get update
apt-get install -y curl gnupg lsb-release

# Download and install GPG key
echo "Installing repository GPG key..."
curl -fsSL "$REPO_URL/key.gpg" | gpg --dearmor -o "$KEYRING_FILE"

# Add repository to sources
echo "Adding repository to sources..."
echo "deb [arch=$(dpkg --print-architecture) signed-by=$KEYRING_FILE] $REPO_URL $(lsb_release -cs) main" > "$SOURCES_FILE"

# Update package lists
echo "Updating package lists..."
apt-get update

echo "CURSED APT repository setup complete!"
echo ""
echo "You can now install CURSED with:"
echo "  sudo apt install cursed"
echo ""
echo "Available packages:"
apt-cache search cursed | grep -E "^cursed"
EOF
    
    chmod +x "$REPO_ROOT/install-cursed-repo.sh"
    
    log "Client setup script created: $REPO_ROOT/install-cursed-repo.sh"
}

create_index_page() {
    log "Creating repository index page..."
    
    cat > "$REPO_ROOT/index.html" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED APT Repository</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #007acc;
            padding-bottom: 10px;
        }
        .install-box {
            background: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 4px;
            padding: 15px;
            margin: 20px 0;
            font-family: 'Courier New', monospace;
        }
        .command {
            background: #2d3748;
            color: #e2e8f0;
            padding: 10px;
            border-radius: 4px;
            margin: 10px 0;
            overflow-x: auto;
        }
        .info {
            background: #e6f3ff;
            border-left: 4px solid #007acc;
            padding: 15px;
            margin: 15px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>CURSED Compiler APT Repository</h1>
        
        <p>Welcome to the official APT repository for the CURSED Programming Language Compiler.</p>
        
        <h2>Quick Setup</h2>
        <p>Run this command to set up the repository and install CURSED:</p>
        <div class="command">
curl -fsSL https://packages.cursed.dev/install-cursed-repo.sh | sudo bash
        </div>
        
        <h2>Manual Setup</h2>
        
        <h3>1. Add Repository Key</h3>
        <div class="command">
curl -fsSL https://packages.cursed.dev/key.gpg | sudo gpg --dearmor -o /usr/share/keyrings/cursed-archive-keyring.gpg
        </div>
        
        <h3>2. Add Repository</h3>
        <div class="command">
echo "deb [arch=\$(dpkg --print-architecture) signed-by=/usr/share/keyrings/cursed-archive-keyring.gpg] https://packages.cursed.dev \$(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/cursed.list
        </div>
        
        <h3>3. Update and Install</h3>
        <div class="command">
sudo apt update<br>
sudo apt install cursed
        </div>
        
        <h2>Verification</h2>
        <p>After installation, verify the compiler is working:</p>
        <div class="command">
cursed-zig --version<br>
cursed-zig --help
        </div>
        
        <div class="info">
            <strong>Note:</strong> This repository supports Ubuntu 20.04+ and Debian 11+ on amd64 and arm64 architectures.
        </div>
        
        <h2>Available Packages</h2>
        <ul>
            <li><strong>cursed</strong> - Main CURSED compiler package</li>
            <li><strong>cursed-dev</strong> - Development tools and headers</li>
            <li><strong>cursed-docs</strong> - Documentation and examples</li>
        </ul>
        
        <h2>Support</h2>
        <ul>
            <li><a href="https://cursed.dev">Official Website</a></li>
            <li><a href="https://docs.cursed.dev">Documentation</a></li>
            <li><a href="https://github.com/ghuntley/cursed/issues">Issue Tracker</a></li>
        </ul>
        
        <hr>
        <p><small>Maintained by the CURSED Development Team | <a href="https://cursed.dev">cursed.dev</a></small></p>
    </div>
</body>
</html>
EOF
    
    log "Repository index page created"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
    log "Setting up CURSED APT repository..."
    
    # Check if reprepro is installed
    if ! command -v reprepro >/dev/null 2>&1; then
        log "Installing reprepro..."
        apt-get update
        apt-get install -y reprepro gnupg2
    fi
    
    # Setup repository structure
    setup_repository_structure
    
    # Generate GPG key if needed
    if [[ -z "$GPG_KEY_ID" ]]; then
        generate_gpg_key
    fi
    
    # Create management scripts
    create_package_scripts
    
    # Create web server configuration
    create_nginx_config
    
    # Create client setup script
    create_client_setup_script
    
    # Create index page
    create_index_page
    
    log "APT repository setup completed!"
    log "Repository location: $REPO_ROOT"
    log "Next steps:"
    log "1. Configure your web server using: $REPO_ROOT/nginx-site.conf"
    log "2. Add packages using: $REPO_ROOT/add-package.sh <package.deb>"
    log "3. Clients can install using: curl -fsSL https://packages.cursed.dev/install-cursed-repo.sh | sudo bash"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    error "This script must be run as root (use sudo)"
fi

main "$@"
