#!/bin/bash
# YUM/DNF Repository Setup Script for CURSED Compiler
# Enterprise-ready RedHat/CentOS/Fedora package management

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

REPO_NAME="cursed"
REPO_DESCRIPTION="CURSED Compiler YUM Repository"
REPO_BASE_URL="https://packages.cursed.dev/yum"

# Directories
REPO_ROOT="/srv/yum/cursed"
REPODATA_DIR="$REPO_ROOT/repodata"

# GPG configuration
GPG_KEY_ID=""  # Set your GPG key ID
GPG_KEY_EMAIL="dev@cursed.dev"

# Supported distributions
DISTRIBUTIONS=(
    "el7"      # RHEL/CentOS 7
    "el8"      # RHEL/CentOS 8
    "el9"      # RHEL/CentOS 9
    "fc37"     # Fedora 37
    "fc38"     # Fedora 38
    "fc39"     # Fedora 39
)

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
    log "Setting up YUM repository structure..."
    
    # Create main repository directory
    mkdir -p "$REPO_ROOT"
    
    # Create distribution-specific directories
    for dist in "${DISTRIBUTIONS[@]}"; do
        mkdir -p "$REPO_ROOT/$dist"/{x86_64,noarch,SRPMS}
        log "Created directory structure for: $dist"
    done
    
    # Create common directories
    mkdir -p "$REPO_ROOT"/{incoming,keys}
    
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
%echo Generating a GPG key for CURSED YUM repository
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
    gpg --armor --export "$GPG_KEY_ID" > "$REPO_ROOT/keys/RPM-GPG-KEY-cursed"
    
    log "GPG key generated with ID: $GPG_KEY_ID"
    log "Public key exported to: $REPO_ROOT/keys/RPM-GPG-KEY-cursed"
}

create_rpm_macros() {
    log "Creating RPM signing macros..."
    
    # Create RPM macros for signing
    cat > ~/.rpmmacros << EOF
%_signature gpg
%_gpg_path ~/.gnupg
%_gpg_name $GPG_KEY_EMAIL
%_gpgbin /usr/bin/gpg2
%__gpg_sign_cmd %{__gpg} gpg --force-v3-sigs --batch --verbose --no-armor --passphrase-fd 3 --no-secmem-warning -u "%{_gpg_name}" -sbo %{__signature_filename} --digest-algo sha256 %{__plaintext_filename}
EOF
    
    log "RPM macros configured for package signing"
}

create_repository_config() {
    log "Creating repository configuration files..."
    
    # Create main repository configuration
    cat > "$REPO_ROOT/cursed.repo" << EOF
[cursed]
name=CURSED Compiler Repository
baseurl=$REPO_BASE_URL/\$releasever/\$basearch
enabled=1
gpgcheck=1
gpgkey=$REPO_BASE_URL/keys/RPM-GPG-KEY-cursed
metadata_expire=7d
skip_if_unavailable=False

[cursed-source]
name=CURSED Compiler Repository - Source
baseurl=$REPO_BASE_URL/\$releasever/SRPMS
enabled=0
gpgcheck=1
gpgkey=$REPO_BASE_URL/keys/RPM-GPG-KEY-cursed
metadata_expire=7d
skip_if_unavailable=False
EOF

    # Create distribution-specific configs
    for dist in "${DISTRIBUTIONS[@]}"; do
        case "$dist" in
            el*)
                distro_name="Enterprise Linux"
                ;;
            fc*)
                distro_name="Fedora"
                ;;
            *)
                distro_name="Linux"
                ;;
        esac
        
        cat > "$REPO_ROOT/cursed-$dist.repo" << EOF
[cursed-$dist]
name=CURSED Compiler Repository for $distro_name
baseurl=$REPO_BASE_URL/$dist/\$basearch
enabled=1
gpgcheck=1
gpgkey=$REPO_BASE_URL/keys/RPM-GPG-KEY-cursed
metadata_expire=7d
skip_if_unavailable=False
EOF
    done
    
    log "Repository configuration files created"
}

create_package_scripts() {
    log "Creating package management scripts..."
    
    # Script to add a package to the repository
    cat > "$REPO_ROOT/add-package.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <distribution> <package.rpm>"
    echo "Distributions: el7, el8, el9, fc37, fc38, fc39"
    exit 1
fi

DIST="$1"
PACKAGE="$2"
REPO_ROOT="$(dirname "$0")"

if [[ ! -f "$PACKAGE" ]]; then
    echo "Error: Package file not found: $PACKAGE"
    exit 1
fi

# Determine architecture
ARCH=$(rpm -qp --queryformat '%{ARCH}' "$PACKAGE")
if [[ "$ARCH" == "noarch" ]]; then
    TARGET_DIR="$REPO_ROOT/$DIST/noarch"
else
    TARGET_DIR="$REPO_ROOT/$DIST/$ARCH"
fi

echo "Adding package to repository:"
echo "  Distribution: $DIST"
echo "  Architecture: $ARCH"
echo "  Package: $PACKAGE"

# Copy package to repository
mkdir -p "$TARGET_DIR"
cp "$PACKAGE" "$TARGET_DIR/"

# Sign the package if GPG is configured
if [[ -n "${GPG_KEY_EMAIL:-}" ]]; then
    echo "Signing package..."
    rpm --addsign "$TARGET_DIR/$(basename "$PACKAGE")"
fi

# Update repository metadata
echo "Updating repository metadata..."
createrepo --update "$REPO_ROOT/$DIST"

echo "Package added successfully"
EOF

    # Script to remove a package from the repository
    cat > "$REPO_ROOT/remove-package.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <distribution> <package-name>"
    echo "Distributions: el7, el8, el9, fc37, fc38, fc39"
    exit 1
fi

DIST="$1"
PACKAGE_NAME="$2"
REPO_ROOT="$(dirname "$0")"

echo "Removing package from repository:"
echo "  Distribution: $DIST"
echo "  Package: $PACKAGE_NAME"

# Find and remove package files
find "$REPO_ROOT/$DIST" -name "${PACKAGE_NAME}*.rpm" -delete

# Update repository metadata
echo "Updating repository metadata..."
createrepo --update "$REPO_ROOT/$DIST"

echo "Package removed successfully"
EOF

    # Script to list packages in the repository
    cat > "$REPO_ROOT/list-packages.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

REPO_ROOT="$(dirname "$0")"

echo "Packages in repository:"
echo "======================"

for dist in el7 el8 el9 fc37 fc38 fc39; do
    if [[ -d "$REPO_ROOT/$dist" ]]; then
        echo ""
        echo "Distribution: $dist"
        echo "-------------------"
        find "$REPO_ROOT/$dist" -name "*.rpm" -printf "%f\n" | sort
    fi
done
EOF

    # Script to update all repository metadata
    cat > "$REPO_ROOT/update-repository.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

REPO_ROOT="$(dirname "$0")"

echo "Updating repository metadata for all distributions..."

for dist in el7 el8 el9 fc37 fc38 fc39; do
    if [[ -d "$REPO_ROOT/$dist" ]]; then
        echo "Updating metadata for: $dist"
        createrepo --update "$REPO_ROOT/$dist"
    fi
done

echo "All repository metadata updated"
EOF

    # Script to sign all packages
    cat > "$REPO_ROOT/sign-packages.sh" << 'EOF'
#!/bin/bash
set -euo pipefail

REPO_ROOT="$(dirname "$0")"

if [[ -z "${GPG_KEY_EMAIL:-}" ]]; then
    echo "Error: GPG_KEY_EMAIL environment variable not set"
    exit 1
fi

echo "Signing all packages in repository..."

find "$REPO_ROOT" -name "*.rpm" -not -path "*/repodata/*" | while read -r rpm_file; do
    echo "Signing: $rpm_file"
    rpm --addsign "$rpm_file"
done

echo "All packages signed"
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
    
    # YUM repository files
    location / {
        autoindex on;
        autoindex_exact_size off;
        autoindex_localtime on;
    }
    
    # Handle repository metadata requests
    location ~ /(repodata|keys)/ {
        add_header Cache-Control "public, max-age=3600";
    }
    
    location ~ /\.ht {
        deny all;
    }
    
    # Specific handling for repo files
    location ~ \.repo$ {
        add_header Content-Type "text/plain";
    }
    
    # Logging
    access_log /var/log/nginx/cursed-yum-access.log;
    error_log /var/log/nginx/cursed-yum-error.log;
}
EOF
    
    log "Nginx configuration created: $REPO_ROOT/nginx-site.conf"
}

create_client_setup_script() {
    log "Creating client setup script..."
    
    cat > "$REPO_ROOT/install-cursed-repo.sh" << 'EOF'
#!/bin/bash
# Client setup script for CURSED YUM repository
set -euo pipefail

REPO_URL="https://packages.cursed.dev/yum"

echo "Setting up CURSED YUM repository..."

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root (use sudo)"
    exit 1
fi

# Detect distribution
if [[ -f /etc/redhat-release ]]; then
    if grep -q "CentOS Linux release 7" /etc/redhat-release; then
        DIST="el7"
    elif grep -q "CentOS Linux release 8\|Red Hat Enterprise Linux.*release 8" /etc/redhat-release; then
        DIST="el8"
    elif grep -q "CentOS.*release 9\|Red Hat Enterprise Linux.*release 9" /etc/redhat-release; then
        DIST="el9"
    else
        echo "Unsupported RHEL/CentOS version"
        exit 1
    fi
elif [[ -f /etc/fedora-release ]]; then
    FEDORA_VERSION=$(grep -oP '(?<=Fedora release )\d+' /etc/fedora-release)
    DIST="fc$FEDORA_VERSION"
else
    echo "Unsupported distribution. This script supports RHEL/CentOS 7-9 and Fedora 37-39."
    exit 1
fi

echo "Detected distribution: $DIST"

# Install GPG key
echo "Installing repository GPG key..."
rpm --import "$REPO_URL/keys/RPM-GPG-KEY-cursed"

# Add repository configuration
echo "Adding repository configuration..."
if command -v dnf >/dev/null 2>&1; then
    # Fedora/RHEL 8+
    dnf config-manager --add-repo "$REPO_URL/cursed-$DIST.repo"
elif command -v yum-config-manager >/dev/null 2>&1; then
    # RHEL 7
    yum-config-manager --add-repo "$REPO_URL/cursed-$DIST.repo"
else
    # Manual installation
    curl -fsSL "$REPO_URL/cursed-$DIST.repo" -o /etc/yum.repos.d/cursed.repo
fi

# Update repository cache
echo "Updating repository cache..."
if command -v dnf >/dev/null 2>&1; then
    dnf makecache
else
    yum makecache fast
fi

echo "CURSED YUM repository setup complete!"
echo ""
echo "You can now install CURSED with:"
if command -v dnf >/dev/null 2>&1; then
    echo "  sudo dnf install cursed"
else
    echo "  sudo yum install cursed"
fi
echo ""
echo "Available packages:"
if command -v dnf >/dev/null 2>&1; then
    dnf search cursed
else
    yum search cursed
fi
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
    <title>CURSED YUM Repository</title>
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
            border-bottom: 3px solid #dc382d;
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
            background: #fff3cd;
            border-left: 4px solid #ffc107;
            padding: 15px;
            margin: 15px 0;
        }
        .distro-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin: 20px 0;
        }
        .distro-card {
            background: #f8f9fa;
            border: 1px solid #dee2e6;
            border-radius: 4px;
            padding: 15px;
            text-align: center;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>CURSED Compiler YUM Repository</h1>
        
        <p>Welcome to the official YUM repository for the CURSED Programming Language Compiler.</p>
        
        <h2>Quick Setup</h2>
        <p>Run this command to set up the repository and install CURSED:</p>
        <div class="command">
curl -fsSL https://packages.cursed.dev/yum/install-cursed-repo.sh | sudo bash
        </div>
        
        <h2>Supported Distributions</h2>
        <div class="distro-grid">
            <div class="distro-card">
                <h3>RHEL/CentOS 7</h3>
                <p>Enterprise Linux 7</p>
            </div>
            <div class="distro-card">
                <h3>RHEL/CentOS 8</h3>
                <p>Enterprise Linux 8</p>
            </div>
            <div class="distro-card">
                <h3>RHEL/CentOS 9</h3>
                <p>Enterprise Linux 9</p>
            </div>
            <div class="distro-card">
                <h3>Fedora 37+</h3>
                <p>Latest Fedora releases</p>
            </div>
        </div>
        
        <h2>Manual Setup</h2>
        
        <h3>1. Import GPG Key</h3>
        <div class="command">
sudo rpm --import https://packages.cursed.dev/yum/keys/RPM-GPG-KEY-cursed
        </div>
        
        <h3>2. Add Repository (DNF/Fedora)</h3>
        <div class="command">
sudo dnf config-manager --add-repo https://packages.cursed.dev/yum/cursed.repo
        </div>
        
        <h3>2. Add Repository (YUM/RHEL/CentOS)</h3>
        <div class="command">
sudo yum-config-manager --add-repo https://packages.cursed.dev/yum/cursed.repo
        </div>
        
        <h3>3. Install CURSED</h3>
        <div class="command">
# Fedora/RHEL 8+<br>
sudo dnf install cursed<br><br>
# RHEL/CentOS 7<br>
sudo yum install cursed
        </div>
        
        <h2>Verification</h2>
        <p>After installation, verify the compiler is working:</p>
        <div class="command">
cursed-zig --version<br>
cursed-zig --help
        </div>
        
        <div class="info">
            <strong>Note:</strong> This repository supports RHEL/CentOS 7-9 and Fedora 37+ on x86_64 architecture.
        </div>
        
        <h2>Available Packages</h2>
        <ul>
            <li><strong>cursed</strong> - Main CURSED compiler package</li>
            <li><strong>cursed-devel</strong> - Development tools and headers</li>
            <li><strong>cursed-doc</strong> - Documentation and examples</li>
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

initialize_repositories() {
    log "Initializing repository metadata..."
    
    for dist in "${DISTRIBUTIONS[@]}"; do
        if [[ -d "$REPO_ROOT/$dist" ]]; then
            log "Creating metadata for: $dist"
            createrepo "$REPO_ROOT/$dist"
        fi
    done
    
    log "Repository metadata initialized"
}

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
    log "Setting up CURSED YUM repository..."
    
    # Check if createrepo is installed
    if ! command -v createrepo >/dev/null 2>&1; then
        log "Installing createrepo..."
        if command -v dnf >/dev/null 2>&1; then
            dnf install -y createrepo_c
        elif command -v yum >/dev/null 2>&1; then
            yum install -y createrepo
        else
            error "Cannot install createrepo. Please install it manually."
        fi
    fi
    
    # Setup repository structure
    setup_repository_structure
    
    # Generate GPG key if needed
    if [[ -z "$GPG_KEY_ID" ]]; then
        generate_gpg_key
    fi
    
    # Create RPM macros
    create_rpm_macros
    
    # Create repository configuration
    create_repository_config
    
    # Create management scripts
    create_package_scripts
    
    # Create web server configuration
    create_nginx_config
    
    # Create client setup script
    create_client_setup_script
    
    # Create index page
    create_index_page
    
    # Initialize repositories
    initialize_repositories
    
    log "YUM repository setup completed!"
    log "Repository location: $REPO_ROOT"
    log "Next steps:"
    log "1. Configure your web server using: $REPO_ROOT/nginx-site.conf"
    log "2. Add packages using: $REPO_ROOT/add-package.sh <dist> <package.rpm>"
    log "3. Clients can install using: curl -fsSL https://packages.cursed.dev/yum/install-cursed-repo.sh | sudo bash"
}

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    error "This script must be run as root (use sudo)"
fi

main "$@"
