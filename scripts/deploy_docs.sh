#!/bin/bash

# CURSED Documentation Deployment Script
#
# This script demonstrates automated documentation deployment
# for the CURSED programming language using the publishing infrastructure.

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_DIR="$PROJECT_ROOT/examples/doc_configs"
DOCS_DIR="$PROJECT_ROOT/docs"
TEMP_DIR="/tmp/cursed_docs_deploy"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
TARGET="local"
CONFIG_FILE=""
DRY_RUN=false
VERBOSE=false
OPTIMIZE=true
PACKAGE=""
VERSION=""

# Logging functions
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

# Help function
show_help() {
    cat << EOF
CURSED Documentation Deployment Script

Usage: $0 [OPTIONS]

OPTIONS:
    -t, --target TARGET         Publishing target (local, s3, github-pages, custom)
    -c, --config CONFIG         Configuration file path
    -p, --package PACKAGE       Package name to deploy
    -v, --version VERSION       Package version to deploy
    -d, --dry-run              Perform a dry run without actual deployment
    --no-optimize              Disable optimization
    --verbose                  Enable verbose output
    -h, --help                 Show this help message

EXAMPLES:
    # Deploy to local directory with default configuration
    $0 --target local

    # Deploy to S3 with custom configuration
    $0 --target s3 --config /path/to/s3_config.toml

    # Dry run deployment to GitHub Pages
    $0 --target github-pages --dry-run

    # Deploy specific package version
    $0 --package mypackage --version 1.2.0

    # Deploy with verbose output and no optimization
    $0 --target local --verbose --no-optimize

CONFIGURATION:
    The script looks for configuration files in the following order:
    1. File specified with --config option
    2. $CONFIG_DIR/publish_config.toml
    3. ./cursed_publish.toml
    4. Default configuration

TARGETS:
    local         Deploy to local filesystem
    s3           Deploy to Amazon S3
    github-pages Deploy to GitHub Pages
    custom       Deploy to custom endpoint

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -t|--target)
                TARGET="$2"
                shift 2
                ;;
            -c|--config)
                CONFIG_FILE="$2"
                shift 2
                ;;
            -p|--package)
                PACKAGE="$2"
                shift 2
                ;;
            -v|--version)
                VERSION="$2"
                shift 2
                ;;
            -d|--dry-run)
                DRY_RUN=true
                shift
                ;;
            --no-optimize)
                OPTIMIZE=false
                shift
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Validate requirements
validate_requirements() {
    log_info "Validating deployment requirements..."

    # Check if cursed CLI is available
    if ! command -v cursed &> /dev/null; then
        log_error "CURSED CLI not found. Please install CURSED first."
        exit 1
    fi

    # Check if project directory exists
    if [[ ! -d "$PROJECT_ROOT" ]]; then
        log_error "Project root directory not found: $PROJECT_ROOT"
        exit 1
    fi

    # Check target-specific requirements
    case $TARGET in
        s3)
            if ! command -v aws &> /dev/null; then
                log_error "AWS CLI not found. Required for S3 deployment."
                exit 1
            fi
            ;;
        github-pages)
            if ! command -v git &> /dev/null; then
                log_error "Git not found. Required for GitHub Pages deployment."
                exit 1
            fi
            if [[ -z "${GITHUB_TOKEN:-}" ]]; then
                log_warning "GITHUB_TOKEN environment variable not set."
            fi
            ;;
    esac

    log_success "Requirements validation passed"
}

# Find configuration file
find_config_file() {
    if [[ -n "$CONFIG_FILE" ]]; then
        if [[ ! -f "$CONFIG_FILE" ]]; then
            log_error "Configuration file not found: $CONFIG_FILE"
            exit 1
        fi
        log_info "Using configuration file: $CONFIG_FILE"
        return 0
    fi

    # Try default locations
    local config_files=(
        "$CONFIG_DIR/publish_config.toml"
        "$PROJECT_ROOT/cursed_publish.toml"
        "$HOME/.cursed/publish_config.toml"
    )

    for config in "${config_files[@]}"; do
        if [[ -f "$config" ]]; then
            CONFIG_FILE="$config"
            log_info "Using configuration file: $CONFIG_FILE"
            return 0
        fi
    done

    log_info "No configuration file found, using default settings"
}

# Setup temporary directory
setup_temp_dir() {
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
    fi
    mkdir -p "$TEMP_DIR"
    log_info "Created temporary directory: $TEMP_DIR"
}

# Cleanup temporary directory
cleanup_temp_dir() {
    if [[ -d "$TEMP_DIR" ]]; then
        rm -rf "$TEMP_DIR"
        log_info "Cleaned up temporary directory"
    fi
}

# Trap to ensure cleanup
trap cleanup_temp_dir EXIT

# Validate configuration
validate_config() {
    if [[ -n "$CONFIG_FILE" ]]; then
        log_info "Validating configuration file..."
        
        if ! cursed doc validate --config "$CONFIG_FILE" --config-type publish; then
            log_error "Configuration validation failed"
            exit 1
        fi
        
        log_success "Configuration validation passed"
    fi
}

# Generate documentation
generate_docs() {
    log_info "Generating documentation..."

    local generate_args=()
    
    if [[ "$VERBOSE" == "true" ]]; then
        generate_args+=(--verbose)
    fi

    # Generate documentation
    cd "$PROJECT_ROOT"
    
    if ! cursed doc generate "${generate_args[@]}" --output "$TEMP_DIR/docs"; then
        log_error "Documentation generation failed"
        exit 1
    fi

    log_success "Documentation generated successfully"
}

# Test documentation
test_docs() {
    log_info "Testing documentation..."

    local test_args=(
        --check-links
        --verify-examples
        --check-completeness
        --format text
    )

    if [[ -n "$PACKAGE" ]]; then
        test_args+=(--package "$PACKAGE")
    fi

    if [[ -n "$VERSION" ]]; then
        test_args+=(--version "$VERSION")
    fi

    # Test documentation
    cd "$PROJECT_ROOT"
    
    if ! cursed doc test "${test_args[@]}"; then
        log_warning "Documentation tests failed, but continuing with deployment"
    else
        log_success "Documentation tests passed"
    fi
}

# Deploy documentation
deploy_docs() {
    log_info "Deploying documentation to $TARGET..."

    local deploy_args=(
        --target "$TARGET"
    )

    if [[ -n "$CONFIG_FILE" ]]; then
        deploy_args+=(--config "$CONFIG_FILE")
    fi

    if [[ -n "$PACKAGE" ]]; then
        deploy_args+=(--package "$PACKAGE")
    fi

    if [[ -n "$VERSION" ]]; then
        deploy_args+=(--version "$VERSION")
    fi

    if [[ "$OPTIMIZE" == "true" ]]; then
        deploy_args+=(--optimize)
    fi

    if [[ "$DRY_RUN" == "true" ]]; then
        deploy_args+=(--dry-run)
        log_info "Performing dry run deployment"
    fi

    # Deploy documentation
    cd "$PROJECT_ROOT"
    
    if ! cursed doc publish "${deploy_args[@]}"; then
        log_error "Documentation deployment failed"
        exit 1
    fi

    if [[ "$DRY_RUN" == "true" ]]; then
        log_success "Dry run deployment completed successfully"
    else
        log_success "Documentation deployed successfully to $TARGET"
    fi
}

# Update registry
update_registry() {
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "Skipping registry update in dry run mode"
        return 0
    fi

    log_info "Updating documentation registry..."

    cd "$PROJECT_ROOT"
    
    # Initialize registry if it doesn't exist
    if ! cursed doc registry list &> /dev/null; then
        log_info "Initializing documentation registry..."
        cursed doc registry init
    fi

    log_success "Registry updated successfully"
}

# Send deployment notification
send_notification() {
    if [[ "$DRY_RUN" == "true" ]]; then
        return 0
    fi

    # Send notification if webhook URL is configured
    if [[ -n "${WEBHOOK_URL:-}" ]]; then
        log_info "Sending deployment notification..."
        
        local payload=$(cat << EOF
{
    "target": "$TARGET",
    "package": "$PACKAGE",
    "version": "$VERSION",
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "status": "success"
}
EOF
        )

        if curl -s -X POST -H "Content-Type: application/json" -d "$payload" "$WEBHOOK_URL" > /dev/null; then
            log_success "Deployment notification sent"
        else
            log_warning "Failed to send deployment notification"
        fi
    fi
}

# Main deployment workflow
main() {
    log_info "Starting CURSED documentation deployment"
    log_info "Target: $TARGET"
    
    if [[ -n "$PACKAGE" ]]; then
        log_info "Package: $PACKAGE"
    fi
    
    if [[ -n "$VERSION" ]]; then
        log_info "Version: $VERSION"
    fi

    parse_args "$@"
    validate_requirements
    find_config_file
    setup_temp_dir
    validate_config
    generate_docs
    test_docs
    deploy_docs
    update_registry
    send_notification

    log_success "Documentation deployment completed successfully!"
    
    if [[ "$TARGET" == "local" && "$DRY_RUN" == "false" ]]; then
        log_info "Local documentation available at: file://$(pwd)/published_docs/index.html"
    fi
}

# Run main function with all arguments
main "$@"
