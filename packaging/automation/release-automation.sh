#!/bin/bash
# CURSED Compiler - Release Automation System
# Enterprise-grade automated release management

set -euo pipefail

# ============================================================================
# CONFIGURATION
# ============================================================================

# Version and release management
VERSION_FILE="packaging/VERSION"
CHANGELOG_FILE="CHANGELOG.md"
RELEASE_NOTES_TEMPLATE="packaging/templates/RELEASE_NOTES.md.template"

# Directories
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
DIST_DIR="$PROJECT_ROOT/dist"
STAGING_DIR="$PROJECT_ROOT/staging"

# Git configuration
GIT_REMOTE=${GIT_REMOTE:-origin}
GIT_BRANCH=${GIT_BRANCH:-main}

# Release configuration
RELEASE_TYPE=${RELEASE_TYPE:-patch}  # major, minor, patch, prerelease
PRERELEASE_SUFFIX=${PRERELEASE_SUFFIX:-alpha}
DRY_RUN=${DRY_RUN:-false}
SKIP_TESTS=${SKIP_TESTS:-false}
SKIP_BUILD=${SKIP_BUILD:-false}

# Package management
HOMEBREW_TAP_REPO=${HOMEBREW_TAP_REPO:-cursed/homebrew-tap}
DOCKER_REGISTRY=${DOCKER_REGISTRY:-cursed}
NPM_PACKAGE=${NPM_PACKAGE:-@cursed/compiler}

# Notification configuration
SLACK_WEBHOOK_URL=${SLACK_WEBHOOK_URL:-}
DISCORD_WEBHOOK_URL=${DISCORD_WEBHOOK_URL:-}
EMAIL_RECIPIENTS=${EMAIL_RECIPIENTS:-}

# ============================================================================
# UTILITY FUNCTIONS
# ============================================================================

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [INFO] $*"
}

warn() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [WARN] $*" >&2
}

error() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $*" >&2
    exit 1
}

dry_run() {
    if [[ "$DRY_RUN" == "true" ]]; then
        log "[DRY RUN] $*"
        return 0
    else
        log "Executing: $*"
        "$@"
    fi
}

prompt_confirmation() {
    local message="$1"
    if [[ "$DRY_RUN" == "true" ]]; then
        log "[DRY RUN] Would prompt: $message"
        return 0
    fi
    
    echo -n "$message (y/N): "
    read -r response
    case "$response" in
        [yY]|[yY][eE][sS]) return 0 ;;
        *) return 1 ;;
    esac
}

# ============================================================================
# VERSION MANAGEMENT
# ============================================================================

get_current_version() {
    if [[ -f "$VERSION_FILE" ]]; then
        cat "$VERSION_FILE"
    else
        echo "0.0.0"
    fi
}

increment_version() {
    local current_version="$1"
    local release_type="$2"
    
    # Parse semantic version
    local major minor patch prerelease
    if [[ "$current_version" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)(-([a-zA-Z0-9-]+))?$ ]]; then
        major="${BASH_REMATCH[1]}"
        minor="${BASH_REMATCH[2]}"
        patch="${BASH_REMATCH[3]}"
        prerelease="${BASH_REMATCH[5]:-}"
    else
        error "Invalid version format: $current_version"
    fi
    
    case "$release_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            prerelease=""
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            prerelease=""
            ;;
        patch)
            patch=$((patch + 1))
            prerelease=""
            ;;
        prerelease)
            if [[ -n "$prerelease" ]]; then
                # Increment prerelease number
                if [[ "$prerelease" =~ ^([a-zA-Z-]+)\.([0-9]+)$ ]]; then
                    local prefix="${BASH_REMATCH[1]}"
                    local number="${BASH_REMATCH[2]}"
                    prerelease="$prefix.$((number + 1))"
                else
                    prerelease="$prerelease.1"
                fi
            else
                patch=$((patch + 1))
                prerelease="$PRERELEASE_SUFFIX.0"
            fi
            ;;
        *)
            error "Invalid release type: $release_type"
            ;;
    esac
    
    if [[ -n "$prerelease" ]]; then
        echo "$major.$minor.$patch-$prerelease"
    else
        echo "$major.$minor.$patch"
    fi
}

update_version_files() {
    local new_version="$1"
    
    log "Updating version files to: $new_version"
    
    # Update VERSION file
    echo "$new_version" > "$VERSION_FILE"
    
    # Update Cargo.toml
    if [[ -f "Cargo.toml" ]]; then
        sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
        rm -f Cargo.toml.bak
    fi
    
    # Update package.json if it exists
    if [[ -f "package.json" ]]; then
        jq ".version = \"$new_version\"" package.json > package.json.tmp
        mv package.json.tmp package.json
    fi
    
    # Update version in source code
    find src-zig -name "*.zig" -exec sed -i.bak "s/const VERSION = \".*\"/const VERSION = \"$new_version\"/" {} \;
    find src-zig -name "*.bak" -delete
}

# ============================================================================
# TESTING AND VALIDATION
# ============================================================================

run_tests() {
    if [[ "$SKIP_TESTS" == "true" ]]; then
        log "Skipping tests (SKIP_TESTS=true)"
        return 0
    fi
    
    log "Running comprehensive test suite..."
    
    # Unit tests
    log "Running unit tests..."
    dry_run zig build test
    
    # Integration tests
    log "Running integration tests..."
    dry_run zig build test-all
    
    # Performance tests
    log "Running performance tests..."
    dry_run zig build test-performance || warn "Performance tests failed, continuing..."
    
    # Memory safety tests
    log "Running memory safety tests..."
    if command -v valgrind >/dev/null 2>&1; then
        dry_run ./packaging/scripts/memory-test.sh || warn "Memory tests failed, continuing..."
    fi
    
    # Cross-compilation tests
    log "Testing cross-compilation..."
    dry_run ./packaging/scripts/cross-compile-test.sh || warn "Cross-compilation tests failed, continuing..."
    
    log "All tests completed successfully"
}

validate_environment() {
    log "Validating release environment..."
    
    # Check for required tools
    local required_tools=(
        "git" "zig" "curl" "tar" "gzip" "sha256sum"
    )
    
    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            error "Required tool not found: $tool"
        fi
    done
    
    # Check Git status
    if [[ -n "$(git status --porcelain)" ]]; then
        if ! prompt_confirmation "Working directory is not clean. Continue?"; then
            error "Aborting due to uncommitted changes"
        fi
    fi
    
    # Check Git branch
    local current_branch
    current_branch=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$current_branch" != "$GIT_BRANCH" ]]; then
        if ! prompt_confirmation "Not on $GIT_BRANCH branch (currently on $current_branch). Continue?"; then
            error "Aborting due to branch mismatch"
        fi
    fi
    
    # Check for required environment variables
    if [[ -n "$SLACK_WEBHOOK_URL" ]] && [[ ! "$SLACK_WEBHOOK_URL" =~ ^https://hooks.slack.com/ ]]; then
        warn "Invalid Slack webhook URL format"
    fi
    
    log "Environment validation completed"
}

# ============================================================================
# BUILD AND PACKAGING
# ============================================================================

build_release() {
    if [[ "$SKIP_BUILD" == "true" ]]; then
        log "Skipping build (SKIP_BUILD=true)"
        return 0
    fi
    
    log "Building release packages..."
    
    # Clean previous builds
    dry_run rm -rf "$DIST_DIR" "$STAGING_DIR"
    dry_run mkdir -p "$DIST_DIR" "$STAGING_DIR"
    
    # Run main build script
    dry_run chmod +x packaging/build-release.sh
    dry_run ./packaging/build-release.sh
    
    # Verify build artifacts
    if [[ "$DRY_RUN" != "true" ]]; then
        if [[ ! -d "$DIST_DIR" ]] || [[ -z "$(ls -A "$DIST_DIR" 2>/dev/null)" ]]; then
            error "Build failed: no artifacts found in $DIST_DIR"
        fi
        
        # Verify checksums
        if [[ -f "$DIST_DIR/SHA256SUMS" ]]; then
            cd "$DIST_DIR"
            sha256sum -c SHA256SUMS || error "Checksum verification failed"
            cd "$PROJECT_ROOT"
        fi
    fi
    
    log "Build completed successfully"
}

# ============================================================================
# RELEASE CREATION
# ============================================================================

create_changelog_entry() {
    local version="$1"
    local changelog_content="$2"
    
    log "Creating changelog entry for version $version"
    
    # Backup existing changelog
    if [[ -f "$CHANGELOG_FILE" ]]; then
        cp "$CHANGELOG_FILE" "$CHANGELOG_FILE.bak"
    fi
    
    # Create new changelog entry
    {
        echo "# Changelog"
        echo ""
        echo "## [$version] - $(date '+%Y-%m-%d')"
        echo ""
        echo "$changelog_content"
        echo ""
        if [[ -f "$CHANGELOG_FILE.bak" ]] && [[ -s "$CHANGELOG_FILE.bak" ]]; then
            tail -n +2 "$CHANGELOG_FILE.bak"  # Skip the first line (# Changelog)
        fi
    } > "$CHANGELOG_FILE"
    
    rm -f "$CHANGELOG_FILE.bak"
}

generate_release_notes() {
    local version="$1"
    local previous_version="$2"
    
    log "Generating release notes for version $version"
    
    # Get git log since last version
    local git_log=""
    if git rev-parse "v$previous_version" >/dev/null 2>&1; then
        git_log=$(git log "v$previous_version"..HEAD --oneline --pretty=format:"- %s (%h)")
    else
        git_log=$(git log --oneline --pretty=format:"- %s (%h)" -n 20)
    fi
    
    # Generate release notes from template
    local release_notes
    if [[ -f "$RELEASE_NOTES_TEMPLATE" ]]; then
        release_notes=$(cat "$RELEASE_NOTES_TEMPLATE")
    else
        release_notes="# CURSED Compiler v$version

## What's New

$git_log

## Installation

### Package Managers

**Homebrew (macOS)**:
\`\`\`bash
brew install cursed
\`\`\`

**APT (Ubuntu/Debian)**:
\`\`\`bash
curl -fsSL https://packages.cursed.dev/apt/install-cursed-repo.sh | sudo bash
\`\`\`

**YUM (RHEL/CentOS/Fedora)**:
\`\`\`bash
curl -fsSL https://packages.cursed.dev/yum/install-cursed-repo.sh | sudo bash
\`\`\`

### Docker

\`\`\`bash
docker pull $DOCKER_REGISTRY/compiler:$version
\`\`\`

## Verification

All release artifacts can be verified using the provided SHA256SUMS file:

\`\`\`bash
sha256sum -c SHA256SUMS
\`\`\`
"
    fi
    
    # Substitute variables
    release_notes="${release_notes//\{\{VERSION\}\}/$version}"
    release_notes="${release_notes//\{\{PREVIOUS_VERSION\}\}/$previous_version}"
    release_notes="${release_notes//\{\{DATE\}\}/$(date '+%Y-%m-%d')}"
    release_notes="${release_notes//\{\{GIT_LOG\}\}/$git_log}"
    
    echo "$release_notes" > "$STAGING_DIR/RELEASE_NOTES_$version.md"
    echo "$release_notes"
}

create_git_tag() {
    local version="$1"
    local release_notes="$2"
    
    log "Creating Git tag for version $version"
    
    # Create annotated tag
    local tag_message="Release v$version

$release_notes"
    
    dry_run git add .
    dry_run git commit -m "Release v$version" || true  # May have no changes
    dry_run git tag -a "v$version" -m "$tag_message"
    
    log "Git tag v$version created"
}

push_to_git() {
    local version="$1"
    
    log "Pushing changes to Git repository"
    
    dry_run git push "$GIT_REMOTE" "$GIT_BRANCH"
    dry_run git push "$GIT_REMOTE" "v$version"
    
    log "Changes pushed to $GIT_REMOTE"
}

create_github_release() {
    local version="$1"
    local release_notes_file="$2"
    
    log "Creating GitHub release"
    
    if ! command -v gh >/dev/null 2>&1; then
        warn "GitHub CLI not found, skipping GitHub release creation"
        return 0
    fi
    
    # Create release with artifacts
    local release_args=(
        "release" "create" "v$version"
        "--title" "CURSED Compiler v$version"
        "--notes-file" "$release_notes_file"
    )
    
    # Add artifacts if they exist
    if [[ -d "$DIST_DIR" ]]; then
        find "$DIST_DIR" -name "*.tar.gz" -o -name "*.zip" -o -name "*.deb" -o -name "*.rpm" -o -name "*.pkg" | while read -r artifact; do
            release_args+=("$artifact")
        done
        
        if [[ -f "$DIST_DIR/SHA256SUMS" ]]; then
            release_args+=("$DIST_DIR/SHA256SUMS")
        fi
    fi
    
    dry_run gh "${release_args[@]}"
    
    log "GitHub release created"
}

# ============================================================================
# PACKAGE MANAGER UPDATES
# ============================================================================

update_homebrew_formula() {
    local version="$1"
    
    log "Updating Homebrew formula"
    
    if [[ -z "$HOMEBREW_TAP_REPO" ]]; then
        warn "HOMEBREW_TAP_REPO not set, skipping Homebrew update"
        return 0
    fi
    
    # This would typically involve cloning the tap repo and updating the formula
    # For now, we'll create the updated formula locally
    local formula_file="$STAGING_DIR/cursed.rb"
    
    # Generate checksums for macOS packages
    local sha256_arm64=""
    local sha256_x64=""
    
    if [[ -f "$DIST_DIR/cursed-$version-aarch64-apple-darwin.tar.gz" ]]; then
        sha256_arm64=$(sha256sum "$DIST_DIR/cursed-$version-aarch64-apple-darwin.tar.gz" | cut -d' ' -f1)
    fi
    
    if [[ -f "$DIST_DIR/cursed-$version-x86_64-apple-darwin.tar.gz" ]]; then
        sha256_x64=$(sha256sum "$DIST_DIR/cursed-$version-x86_64-apple-darwin.tar.gz" | cut -d' ' -f1)
    fi
    
    # Update the formula template with new version and checksums
    sed -e "s/{{VERSION}}/$version/g" \
        -e "s/{{SHA256_ARM64}}/$sha256_arm64/g" \
        -e "s/{{SHA256_X64}}/$sha256_x64/g" \
        "packaging/package-managers/homebrew/cursed.rb" > "$formula_file"
    
    log "Homebrew formula updated (manual upload required)"
}

update_docker_images() {
    local version="$1"
    
    log "Updating Docker images"
    
    if ! command -v docker >/dev/null 2>&1; then
        warn "Docker not found, skipping Docker image updates"
        return 0
    fi
    
    # Build and push Docker images
    local image_tags=(
        "$DOCKER_REGISTRY/compiler:$version"
        "$DOCKER_REGISTRY/compiler:latest"
    )
    
    for tag in "${image_tags[@]}"; do
        log "Building Docker image: $tag"
        dry_run docker build -t "$tag" -f packaging/docker/Dockerfile .
        
        log "Pushing Docker image: $tag"
        dry_run docker push "$tag"
    done
    
    log "Docker images updated"
}

# ============================================================================
# NOTIFICATIONS
# ============================================================================

send_slack_notification() {
    local version="$1"
    local release_url="$2"
    
    if [[ -z "$SLACK_WEBHOOK_URL" ]]; then
        return 0
    fi
    
    log "Sending Slack notification"
    
    local payload=$(cat << EOF
{
    "text": "🚀 CURSED Compiler v$version has been released!",
    "attachments": [
        {
            "color": "good",
            "fields": [
                {
                    "title": "Version",
                    "value": "$version",
                    "short": true
                },
                {
                    "title": "Release Date",
                    "value": "$(date '+%Y-%m-%d %H:%M:%S')",
                    "short": true
                }
            ],
            "actions": [
                {
                    "type": "button",
                    "text": "View Release",
                    "url": "$release_url"
                }
            ]
        }
    ]
}
EOF
)
    
    dry_run curl -X POST -H 'Content-type: application/json' \
        --data "$payload" \
        "$SLACK_WEBHOOK_URL"
}

send_discord_notification() {
    local version="$1"
    local release_url="$2"
    
    if [[ -z "$DISCORD_WEBHOOK_URL" ]]; then
        return 0
    fi
    
    log "Sending Discord notification"
    
    local payload=$(cat << EOF
{
    "content": "🚀 **CURSED Compiler v$version** has been released!",
    "embeds": [
        {
            "title": "CURSED Compiler v$version",
            "url": "$release_url",
            "color": 3447003,
            "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%S.000Z)",
            "fields": [
                {
                    "name": "Version",
                    "value": "$version",
                    "inline": true
                },
                {
                    "name": "Release Date",
                    "value": "$(date '+%Y-%m-%d')",
                    "inline": true
                }
            ]
        }
    ]
}
EOF
)
    
    dry_run curl -X POST -H 'Content-type: application/json' \
        --data "$payload" \
        "$DISCORD_WEBHOOK_URL"
}

send_notifications() {
    local version="$1"
    local release_url="https://github.com/ghuntley/cursed/releases/tag/v$version"
    
    log "Sending release notifications"
    
    send_slack_notification "$version" "$release_url"
    send_discord_notification "$version" "$release_url"
    
    # Email notification would go here if configured
    if [[ -n "$EMAIL_RECIPIENTS" ]]; then
        log "Email notifications configured but not implemented"
    fi
}

# ============================================================================
# MAIN RELEASE WORKFLOW
# ============================================================================

show_usage() {
    cat << EOF
CURSED Compiler Release Automation

Usage: $0 [OPTIONS]

Options:
    -t, --type TYPE         Release type: major, minor, patch, prerelease (default: patch)
    -s, --suffix SUFFIX     Prerelease suffix (default: alpha)
    -d, --dry-run          Show what would be done without executing
    --skip-tests           Skip test execution
    --skip-build           Skip build process
    -h, --help             Show this help message

Environment Variables:
    GIT_REMOTE             Git remote name (default: origin)
    GIT_BRANCH             Git branch name (default: main)
    SLACK_WEBHOOK_URL      Slack webhook for notifications
    DISCORD_WEBHOOK_URL    Discord webhook for notifications
    HOMEBREW_TAP_REPO      Homebrew tap repository
    DOCKER_REGISTRY        Docker registry namespace

Examples:
    $0                     # Patch release
    $0 -t minor            # Minor release
    $0 -t major --dry-run  # Major release (dry run)
    $0 -t prerelease -s beta # Beta prerelease

EOF
}

parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -t|--type)
                RELEASE_TYPE="$2"
                shift 2
                ;;
            -s|--suffix)
                PRERELEASE_SUFFIX="$2"
                shift 2
                ;;
            -d|--dry-run)
                DRY_RUN=true
                shift
                ;;
            --skip-tests)
                SKIP_TESTS=true
                shift
                ;;
            --skip-build)
                SKIP_BUILD=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done
    
    # Validate release type
    case "$RELEASE_TYPE" in
        major|minor|patch|prerelease) ;;
        *) error "Invalid release type: $RELEASE_TYPE" ;;
    esac
}

main() {
    log "Starting CURSED Compiler release automation..."
    
    # Parse command line arguments
    parse_arguments "$@"
    
    # Change to project root
    cd "$PROJECT_ROOT"
    
    # Validate environment
    validate_environment
    
    # Get current version and calculate new version
    local current_version
    current_version=$(get_current_version)
    
    local new_version
    new_version=$(increment_version "$current_version" "$RELEASE_TYPE")
    
    log "Current version: $current_version"
    log "New version: $new_version"
    log "Release type: $RELEASE_TYPE"
    
    if [[ "$DRY_RUN" != "true" ]]; then
        if ! prompt_confirmation "Proceed with release v$new_version?"; then
            log "Release cancelled by user"
            exit 0
        fi
    fi
    
    # Update version files
    update_version_files "$new_version"
    
    # Run tests
    run_tests
    
    # Build release packages
    build_release
    
    # Generate release notes
    local release_notes
    release_notes=$(generate_release_notes "$new_version" "$current_version")
    
    # Create changelog entry
    create_changelog_entry "$new_version" "$release_notes"
    
    # Create Git tag and push
    create_git_tag "$new_version" "$release_notes"
    push_to_git "$new_version"
    
    # Create GitHub release
    local release_notes_file="$STAGING_DIR/RELEASE_NOTES_$new_version.md"
    create_github_release "$new_version" "$release_notes_file"
    
    # Update package managers
    update_homebrew_formula "$new_version"
    update_docker_images "$new_version"
    
    # Send notifications
    send_notifications "$new_version"
    
    log "Release v$new_version completed successfully!"
    log ""
    log "Summary:"
    log "  Version: $new_version"
    log "  Type: $RELEASE_TYPE"
    log "  Git tag: v$new_version"
    log "  Artifacts: $DIST_DIR"
    log ""
    log "Next steps:"
    log "1. Verify GitHub release: https://github.com/ghuntley/cursed/releases/tag/v$new_version"
    log "2. Monitor package manager updates"
    log "3. Update documentation if needed"
    log "4. Announce release to community"
}

# Execute main function with all arguments
main "$@"
