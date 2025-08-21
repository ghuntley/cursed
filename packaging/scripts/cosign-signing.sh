#!/bin/bash
# CURSED Programming Language - Cosign Signing Script  
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch
set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DIST_DIR="${PROJECT_ROOT}/dist"
VERSION="${VERSION:-$(cat ${PROJECT_ROOT}/VERSION 2>/dev/null || echo "1.0.0")}"

# Cosign configuration
COSIGN_EXPERIMENTAL="${COSIGN_EXPERIMENTAL:-1}"
COSIGN_PRIVATE_KEY="${COSIGN_PRIVATE_KEY:-}"
COSIGN_PASSWORD="${COSIGN_PASSWORD:-}"
COSIGN_PUBLIC_KEY="${COSIGN_PUBLIC_KEY:-}"
SIGNING_KEY_PATH="${SIGNING_KEY_PATH:-${HOME}/.cursed/signing/cosign.key}"
PUBLIC_KEY_PATH="${PUBLIC_KEY_PATH:-${HOME}/.cursed/signing/cosign.pub}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if cosign is installed
check_cosign() {
    if ! command -v cosign >/dev/null 2>&1; then
        log_error "cosign is not installed. Please install it first:"
        echo "  # Install using Go:"
        echo "  go install github.com/sigstore/cosign/v2/cmd/cosign@latest"
        echo ""
        echo "  # Install using package manager:"
        echo "  # Debian/Ubuntu: apt install cosign"
        echo "  # Homebrew: brew install cosign" 
        echo "  # Arch: pacman -S cosign"
        exit 1
    fi
    
    local version=$(cosign version --json 2>/dev/null | jq -r '.gitVersion // "unknown"' 2>/dev/null || echo "unknown")
    log_info "Using cosign version: ${version}"
}

# Generate signing keys if they don't exist
generate_keys() {
    if [[ ! -f "${SIGNING_KEY_PATH}" ]]; then
        log_info "Generating new cosign key pair..."
        
        # Create directory for keys
        mkdir -p "$(dirname "${SIGNING_KEY_PATH}")"
        
        # Generate key pair
        cosign generate-key-pair --output-key-prefix "$(basename "${SIGNING_KEY_PATH}" .key)"
        
        # Move keys to proper location
        mv "$(basename "${SIGNING_KEY_PATH}" .key).key" "${SIGNING_KEY_PATH}"
        mv "$(basename "${SIGNING_KEY_PATH}" .key).pub" "${PUBLIC_KEY_PATH}"
        
        log_success "Generated signing key pair:"
        log_info "  Private key: ${SIGNING_KEY_PATH}"
        log_info "  Public key: ${PUBLIC_KEY_PATH}"
        
        # Set proper permissions
        chmod 600 "${SIGNING_KEY_PATH}"
        chmod 644 "${PUBLIC_KEY_PATH}"
        
        log_warning "⚠️  IMPORTANT: Back up your private key securely!"
        log_warning "⚠️  Share the public key for signature verification"
        
    else
        log_info "Using existing signing key: ${SIGNING_KEY_PATH}"
    fi
}

# Sign a single file
sign_file() {
    local file_path="$1"
    local signature_path="${file_path}.sig"
    
    if [[ ! -f "${file_path}" ]]; then
        log_error "File not found: ${file_path}"
        return 1
    fi
    
    log_info "Signing: $(basename "${file_path}")"
    
    # Sign with cosign
    if [[ -n "${COSIGN_PRIVATE_KEY}" ]]; then
        # Use environment variable for key
        echo "${COSIGN_PRIVATE_KEY}" | cosign sign-blob --key=- --output-signature="${signature_path}" "${file_path}"
    else
        # Use key file
        cosign sign-blob --key="${SIGNING_KEY_PATH}" --output-signature="${signature_path}" "${file_path}"
    fi
    
    if [[ -f "${signature_path}" ]]; then
        log_success "Created signature: $(basename "${signature_path}")"
        
        # Create detached signature with standard naming
        local detached_sig="${file_path}.cosign"
        cp "${signature_path}" "${detached_sig}"
        
        return 0
    else
        log_error "Failed to create signature for: $(basename "${file_path}")"
        return 1
    fi
}

# Verify a signature
verify_signature() {
    local file_path="$1"
    local signature_path="${file_path}.sig"
    
    if [[ ! -f "${file_path}" ]]; then
        log_error "File not found: ${file_path}"
        return 1
    fi
    
    if [[ ! -f "${signature_path}" ]]; then
        log_error "Signature not found: ${signature_path}"
        return 1
    fi
    
    log_info "Verifying signature for: $(basename "${file_path}")"
    
    # Verify with cosign
    if [[ -n "${COSIGN_PUBLIC_KEY}" ]]; then
        # Use environment variable for public key
        echo "${COSIGN_PUBLIC_KEY}" | cosign verify-blob --key=- --signature="${signature_path}" "${file_path}"
    else
        # Use public key file
        cosign verify-blob --key="${PUBLIC_KEY_PATH}" --signature="${signature_path}" "${file_path}"
    fi
    
    if [[ $? -eq 0 ]]; then
        log_success "Signature verification passed: $(basename "${file_path}")"
        return 0
    else
        log_error "Signature verification failed: $(basename "${file_path}")"
        return 1
    fi
}

# Sign all distribution artifacts
sign_all_artifacts() {
    local artifacts_count=0
    local signed_count=0
    local failed_count=0
    
    log_info "Signing all distribution artifacts..."
    
    # Create dist directory if it doesn't exist
    mkdir -p "${DIST_DIR}"
    
    # Find all binary artifacts to sign
    local file_patterns=(
        "*.tar.gz"
        "*.zip" 
        "*.deb"
        "*.rpm"
        "*.pkg"
        "*.msi"
        "*.exe"
        "*.wasm"
        "*cursed*"
    )
    
    for pattern in "${file_patterns[@]}"; do
        while IFS= read -r -d '' file; do
            # Skip already signed files
            if [[ "${file}" == *.sig ]] || [[ "${file}" == *.cosign ]]; then
                continue
            fi
            
            # Skip checksums and metadata files
            if [[ "$(basename "${file}")" =~ ^(SHA256SUMS|checksums|metadata) ]]; then
                continue
            fi
            
            ((artifacts_count++))
            
            if sign_file "${file}"; then
                ((signed_count++))
            else
                ((failed_count++))
            fi
            
        done < <(find "${DIST_DIR}" -name "${pattern}" -type f -print0 2>/dev/null)
    done
    
    log_info "Signing summary:"
    log_info "  Total artifacts: ${artifacts_count}"
    log_success "  Successfully signed: ${signed_count}"
    
    if [[ ${failed_count} -gt 0 ]]; then
        log_error "  Failed to sign: ${failed_count}"
        return 1
    else
        log_success "All artifacts signed successfully!"
        return 0
    fi
}

# Verify all signatures
verify_all_signatures() {
    local signatures_count=0
    local verified_count=0
    local failed_count=0
    
    log_info "Verifying all signatures..."
    
    while IFS= read -r -d '' sig_file; do
        local artifact_file="${sig_file%.sig}"
        
        if [[ -f "${artifact_file}" ]]; then
            ((signatures_count++))
            
            if verify_signature "${artifact_file}"; then
                ((verified_count++))
            else
                ((failed_count++))
            fi
        else
            log_warning "Orphaned signature found: $(basename "${sig_file}")"
        fi
        
    done < <(find "${DIST_DIR}" -name "*.sig" -type f -print0 2>/dev/null)
    
    log_info "Verification summary:"
    log_info "  Total signatures: ${signatures_count}"
    log_success "  Successfully verified: ${verified_count}"
    
    if [[ ${failed_count} -gt 0 ]]; then
        log_error "  Failed to verify: ${failed_count}"
        return 1
    else
        log_success "All signatures verified successfully!"
        return 0
    fi
}

# Create public key bundle for distribution
create_public_key_bundle() {
    local bundle_path="${DIST_DIR}/cursed-${VERSION}-cosign-public-keys.pem"
    
    log_info "Creating public key bundle..."
    
    if [[ -f "${PUBLIC_KEY_PATH}" ]]; then
        cp "${PUBLIC_KEY_PATH}" "${bundle_path}"
        log_success "Created public key bundle: $(basename "${bundle_path}")"
        
        # Create verification instructions
        cat > "${DIST_DIR}/SIGNATURE_VERIFICATION.md" << 'EOF'
# CURSED Package Signature Verification

This document explains how to verify the authenticity of CURSED distribution packages using cosign.

## Prerequisites

Install cosign:
```bash
# Using Go
go install github.com/sigstore/cosign/v2/cmd/cosign@latest

# Using package managers
# Debian/Ubuntu: apt install cosign
# Homebrew: brew install cosign
# Arch: pacman -S cosign
```

## Verification Steps

1. Download the public key bundle:
   ```bash
   curl -O https://github.com/ghuntley/cursed/releases/download/v1.0.0/cursed-1.0.0-cosign-public-keys.pem
   ```

2. Verify a package signature:
   ```bash
   # For a tar.gz package
   cosign verify-blob --key cursed-1.0.0-cosign-public-keys.pem --signature cursed-1.0.0-linux-x64.tar.gz.sig cursed-1.0.0-linux-x64.tar.gz
   
   # For a Windows package
   cosign verify-blob --key cursed-1.0.0-cosign-public-keys.pem --signature cursed-1.0.0-windows-x64.zip.sig cursed-1.0.0-windows-x64.zip
   ```

3. Successful verification will output:
   ```
   Verified OK
   ```

## Automated Verification Script

Download and run the verification script:
```bash
curl -s https://raw.githubusercontent.com/ghuntley/cursed/main/packaging/scripts/verify-signatures.sh | bash
```

## Security Notes

- Always verify signatures before installing CURSED packages
- Only use the official public key from the GitHub releases page
- Report any signature verification failures to security@cursedlang.org

## Public Key Fingerprint

The public key fingerprint for verification:
```
EOF
        
        # Add fingerprint if available
        if command -v openssl >/dev/null 2>&1; then
            echo '```' >> "${DIST_DIR}/SIGNATURE_VERIFICATION.md"
            openssl rsa -in "${PUBLIC_KEY_PATH}" -pubin -outform DER | openssl dgst -sha256 -hex >> "${DIST_DIR}/SIGNATURE_VERIFICATION.md" 2>/dev/null || true
            echo '```' >> "${DIST_DIR}/SIGNATURE_VERIFICATION.md"
        fi
        
        log_success "Created verification documentation: SIGNATURE_VERIFICATION.md"
    else
        log_error "Public key not found: ${PUBLIC_KEY_PATH}"
        return 1
    fi
}

# Show usage information
show_usage() {
    echo "CURSED Cosign Signing Script"
    echo ""
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  sign [FILE]        Sign a specific file or all artifacts"
    echo "  verify [FILE]      Verify a specific file or all signatures"
    echo "  generate-keys      Generate new cosign key pair"
    echo "  public-key-bundle  Create public key bundle for distribution"
    echo "  help               Show this help message"
    echo ""
    echo "Options:"
    echo "  --key-path PATH    Path to private key file"
    echo "  --pub-key PATH     Path to public key file"
    echo "  --dist-dir PATH    Path to distribution directory"
    echo ""
    echo "Environment Variables:"
    echo "  COSIGN_PRIVATE_KEY    Private key content (alternative to key file)"
    echo "  COSIGN_PUBLIC_KEY     Public key content (alternative to key file)"
    echo "  COSIGN_PASSWORD       Password for private key"
    echo "  SIGNING_KEY_PATH      Path to private key file"
    echo "  PUBLIC_KEY_PATH       Path to public key file"
    echo ""
    echo "Examples:"
    echo "  $0 generate-keys              # Generate new key pair"
    echo "  $0 sign                       # Sign all artifacts"
    echo "  $0 sign cursed-linux.tar.gz   # Sign specific file"
    echo "  $0 verify                     # Verify all signatures"
    echo "  $0 public-key-bundle          # Create public key bundle"
    echo ""
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --key-path)
                SIGNING_KEY_PATH="$2"
                shift 2
                ;;
            --pub-key)
                PUBLIC_KEY_PATH="$2"
                shift 2
                ;;
            --dist-dir)
                DIST_DIR="$2"
                shift 2
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                break
                ;;
        esac
    done
}

# Main execution
main() {
    parse_args "$@"
    
    local command="${1:-help}"
    
    log_info "CURSED Cosign Signing Script v${VERSION}"
    log_info "Distribution directory: ${DIST_DIR}"
    
    check_cosign
    
    case "${command}" in
        "generate-keys")
            generate_keys
            ;;
        "sign")
            generate_keys
            if [[ -n "${2:-}" ]]; then
                sign_file "$2"
            else
                sign_all_artifacts
            fi
            ;;
        "verify")
            if [[ -n "${2:-}" ]]; then
                verify_signature "$2"
            else
                verify_all_signatures
            fi
            ;;
        "public-key-bundle")
            create_public_key_bundle
            ;;
        "help"|"--help"|"-h")
            show_usage
            ;;
        *)
            log_error "Unknown command: ${command}"
            show_usage
            exit 1
            ;;
    esac
}

# Execute main function if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
