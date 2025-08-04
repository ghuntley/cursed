#!/bin/bash

# CURSED Build Cache System
# Advanced incremental compilation and build caching

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CACHE_DIR="$PROJECT_ROOT/.cursed_build_cache"
METADATA_DIR="$CACHE_DIR/metadata"
ARTIFACTS_DIR="$CACHE_DIR/artifacts"
CONFIG_FILE="$CACHE_DIR/cache_config.json"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Cache configuration
CACHE_MAX_SIZE_MB=${CACHE_MAX_SIZE_MB:-1024}  # 1GB default
CACHE_RETENTION_DAYS=${CACHE_RETENTION_DAYS:-7}
CACHE_COMPRESSION=${CACHE_COMPRESSION:-"true"}
PARALLEL_CACHE_OPS=${PARALLEL_CACHE_OPS:-4}

# Logging
log_info() {
    echo -e "${BLUE}[CACHE]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[CACHE]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[CACHE]${NC} $1"
}

log_error() {
    echo -e "${RED}[CACHE]${NC} $1"
}

# Initialize cache system
init_cache() {
    log_info "🗄️ Initializing CURSED build cache system"
    
    # Create cache directories
    mkdir -p "$CACHE_DIR" "$METADATA_DIR" "$ARTIFACTS_DIR"
    
    # Create cache configuration
    if [[ ! -f "$CONFIG_FILE" ]]; then
        cat > "$CONFIG_FILE" << EOF
{
    "version": "1.0",
    "max_size_mb": $CACHE_MAX_SIZE_MB,
    "retention_days": $CACHE_RETENTION_DAYS,
    "compression": $CACHE_COMPRESSION,
    "created": "$(date -Iseconds)",
    "stats": {
        "hits": 0,
        "misses": 0,
        "builds_cached": 0,
        "total_saved_time": 0
    }
}
EOF
    fi
    
    log_success "Cache system initialized at $CACHE_DIR"
}

# Generate cache key from source files and build configuration
generate_cache_key() {
    local target="$1"
    local optimize="$2"
    local additional_params="${3:-}"
    
    # Create hash of source files
    local source_hash
    source_hash=$(find "$PROJECT_ROOT/src-zig" -name "*.zig" -type f -exec sha256sum {} \; | \
                  sort | sha256sum | cut -d' ' -f1)
    
    # Create hash of build configuration
    local config_hash
    config_hash=$(echo "$target:$optimize:$additional_params" | sha256sum | cut -d' ' -f1)
    
    # Combine hashes
    local cache_key
    cache_key=$(echo "$source_hash:$config_hash" | sha256sum | cut -d' ' -f1)
    
    echo "$cache_key"
}

# Check if cached build exists
check_cache() {
    local cache_key="$1"
    local cache_path="$ARTIFACTS_DIR/$cache_key"
    
    if [[ -d "$cache_path" ]]; then
        # Verify cache integrity
        if [[ -f "$cache_path/metadata.json" && -f "$cache_path/binary" ]]; then
            # Check if cache is still valid (not expired)
            local cache_time
            cache_time=$(jq -r '.timestamp' "$cache_path/metadata.json" 2>/dev/null || echo "0")
            local current_time
            current_time=$(date +%s)
            local age_hours
            age_hours=$(( (current_time - cache_time) / 3600 ))
            
            if [[ $age_hours -lt $((CACHE_RETENTION_DAYS * 24)) ]]; then
                log_success "✅ Cache hit for key: $cache_key"
                update_cache_stats "hits"
                return 0
            else
                log_warning "⏰ Cache expired for key: $cache_key"
                rm -rf "$cache_path"
            fi
        else
            log_warning "💥 Corrupted cache for key: $cache_key"
            rm -rf "$cache_path"
        fi
    fi
    
    log_info "❌ Cache miss for key: $cache_key"
    update_cache_stats "misses"
    return 1
}

# Store build in cache
store_in_cache() {
    local cache_key="$1"
    local binary_path="$2"
    local build_time="$3"
    local target="$4"
    local optimize="$5"
    
    local cache_path="$ARTIFACTS_DIR/$cache_key"
    mkdir -p "$cache_path"
    
    # Copy binary to cache
    if [[ -f "$binary_path" ]]; then
        if [[ "$CACHE_COMPRESSION" == "true" ]]; then
            gzip -c "$binary_path" > "$cache_path/binary.gz"
        else
            cp "$binary_path" "$cache_path/binary"
        fi
    else
        log_error "Binary not found for caching: $binary_path"
        return 1
    fi
    
    # Store metadata
    cat > "$cache_path/metadata.json" << EOF
{
    "cache_key": "$cache_key",
    "timestamp": $(date +%s),
    "build_time": $build_time,
    "target": "$target",
    "optimize": "$optimize",
    "binary_size": $(stat -f%z "$binary_path" 2>/dev/null || stat -c%s "$binary_path" 2>/dev/null),
    "compressed": $CACHE_COMPRESSION,
    "zig_version": "$(zig version)"
}
EOF
    
    update_cache_stats "builds_cached"
    update_cache_stats "total_saved_time" "$build_time"
    log_success "💾 Stored build in cache: $cache_key"
}

# Retrieve build from cache
retrieve_from_cache() {
    local cache_key="$1"
    local output_path="$2"
    
    local cache_path="$ARTIFACTS_DIR/$cache_key"
    
    if [[ ! -d "$cache_path" ]]; then
        log_error "Cache path not found: $cache_path"
        return 1
    fi
    
    # Create output directory
    mkdir -p "$(dirname "$output_path")"
    
    # Restore binary
    if [[ -f "$cache_path/binary.gz" ]]; then
        gunzip -c "$cache_path/binary.gz" > "$output_path"
    elif [[ -f "$cache_path/binary" ]]; then
        cp "$cache_path/binary" "$output_path"
    else
        log_error "No binary found in cache: $cache_path"
        return 1
    fi
    
    # Make executable
    chmod +x "$output_path"
    
    # Update access time
    touch "$cache_path/metadata.json"
    
    log_success "📦 Retrieved build from cache: $cache_key"
    return 0
}

# Update cache statistics
update_cache_stats() {
    local stat_name="$1"
    local increment="${2:-1}"
    
    # Read current stats
    local current_value
    current_value=$(jq -r ".stats.$stat_name" "$CONFIG_FILE" 2>/dev/null || echo "0")
    
    # Update stats
    local new_value
    if [[ "$stat_name" == "total_saved_time" ]]; then
        new_value=$(echo "$current_value + $increment" | bc 2>/dev/null || echo "$current_value")
    else
        new_value=$((current_value + increment))
    fi
    
    # Write back to config
    jq ".stats.$stat_name = $new_value" "$CONFIG_FILE" > "$CONFIG_FILE.tmp" && mv "$CONFIG_FILE.tmp" "$CONFIG_FILE"
}

# Clean up old cache entries
cleanup_cache() {
    log_info "🧹 Cleaning up build cache"
    
    # Remove expired entries
    local expired_count=0
    local current_time
    current_time=$(date +%s)
    local retention_seconds
    retention_seconds=$((CACHE_RETENTION_DAYS * 24 * 3600))
    
    find "$ARTIFACTS_DIR" -name "metadata.json" | while read -r metadata_file; do
        local cache_time
        cache_time=$(jq -r '.timestamp' "$metadata_file" 2>/dev/null || echo "0")
        local age_seconds
        age_seconds=$((current_time - cache_time))
        
        if [[ $age_seconds -gt $retention_seconds ]]; then
            local cache_dir
            cache_dir=$(dirname "$metadata_file")
            rm -rf "$cache_dir"
            ((expired_count++))
        fi
    done
    
    # Check cache size and remove oldest entries if necessary
    local current_size_mb
    current_size_mb=$(du -sm "$CACHE_DIR" 2>/dev/null | cut -f1 || echo "0")
    
    if [[ $current_size_mb -gt $CACHE_MAX_SIZE_MB ]]; then
        log_warning "Cache size ($current_size_mb MB) exceeds limit ($CACHE_MAX_SIZE_MB MB)"
        
        # Remove oldest entries
        find "$ARTIFACTS_DIR" -name "metadata.json" -exec stat -f "%m %N" {} \; 2>/dev/null | \
        sort -n | head -n 5 | while read -r timestamp filepath; do
            local cache_dir
            cache_dir=$(dirname "$filepath")
            log_info "Removing old cache entry: $(basename "$cache_dir")"
            rm -rf "$cache_dir"
        done
    fi
    
    log_success "Cache cleanup completed"
}

# Display cache statistics
show_cache_stats() {
    if [[ ! -f "$CONFIG_FILE" ]]; then
        log_warning "Cache not initialized"
        return 1
    fi
    
    local stats
    stats=$(jq -r '.stats' "$CONFIG_FILE")
    local hits
    hits=$(echo "$stats" | jq -r '.hits')
    local misses
    misses=$(echo "$stats" | jq -r '.misses')
    local builds_cached
    builds_cached=$(echo "$stats" | jq -r '.builds_cached')
    local total_saved_time
    total_saved_time=$(echo "$stats" | jq -r '.total_saved_time')
    
    local hit_rate
    if [[ $((hits + misses)) -gt 0 ]]; then
        hit_rate=$(echo "scale=1; $hits * 100 / ($hits + $misses)" | bc)
    else
        hit_rate="0.0"
    fi
    
    local cache_size_mb
    cache_size_mb=$(du -sm "$CACHE_DIR" 2>/dev/null | cut -f1 || echo "0")
    
    local entry_count
    entry_count=$(find "$ARTIFACTS_DIR" -name "metadata.json" | wc -l | tr -d ' ')
    
    echo "📊 CURSED Build Cache Statistics"
    echo "================================"
    echo "Cache location: $CACHE_DIR"
    echo "Cache size: $cache_size_mb MB (max: $CACHE_MAX_SIZE_MB MB)"
    echo "Cache entries: $entry_count"
    echo "Cache hits: $hits"
    echo "Cache misses: $misses"
    echo "Hit rate: $hit_rate%"
    echo "Builds cached: $builds_cached"
    echo "Total saved time: ${total_saved_time}s"
    echo "Retention period: $CACHE_RETENTION_DAYS days"
    echo "Compression: $CACHE_COMPRESSION"
}

# Cached build wrapper
cached_build() {
    local target="$1"
    local optimize="$2"
    local output_path="$3"
    local additional_params="${4:-}"
    
    local cache_key
    cache_key=$(generate_cache_key "$target" "$optimize" "$additional_params")
    
    log_info "🔍 Checking cache for target: $target ($optimize)"
    
    # Check cache first
    if check_cache "$cache_key"; then
        if retrieve_from_cache "$cache_key" "$output_path"; then
            local build_time
            build_time=$(jq -r '.build_time' "$ARTIFACTS_DIR/$cache_key/metadata.json" 2>/dev/null || echo "0")
            log_success "⚡ Build completed from cache in 0.1s (original: ${build_time}s)"
            return 0
        else
            log_warning "Failed to retrieve from cache, building fresh"
        fi
    fi
    
    # Build fresh
    log_info "🔨 Building fresh for target: $target"
    local start_time
    start_time=$(date +%s.%N)
    
    # Execute actual build
    if zig build -Dtarget="$target" -Doptimize="$optimize" $additional_params; then
        local end_time
        end_time=$(date +%s.%N)
        local build_time
        build_time=$(echo "$end_time - $start_time" | bc)
        
        # Find the built binary
        local binary_path
        binary_path=$(find "$PROJECT_ROOT/zig-out" -name "cursed*" -executable -type f | head -n 1)
        
        if [[ -n "$binary_path" ]]; then
            # Copy to output location
            cp "$binary_path" "$output_path"
            
            # Store in cache
            store_in_cache "$cache_key" "$binary_path" "$build_time" "$target" "$optimize"
            
            log_success "✅ Build completed in ${build_time}s"
            return 0
        else
            log_error "Built binary not found"
            return 1
        fi
    else
        log_error "Build failed"
        return 1
    fi
}

# Incremental build system
incremental_build() {
    local targets=("$@")
    
    log_info "🔄 Starting incremental build system"
    
    # Parallel cache operations
    local pids=()
    local max_parallel=$PARALLEL_CACHE_OPS
    local running=0
    
    for target in "${targets[@]}"; do
        # Parse target (format: target:optimize)
        local zig_target
        local optimize
        if [[ "$target" == *":"* ]]; then
            zig_target="${target%:*}"
            optimize="${target#*:}"
        else
            zig_target="$target"
            optimize="ReleaseFast"
        fi
        
        # Wait if we've reached max parallel builds
        while [[ $running -ge $max_parallel ]]; do
            for i in "${!pids[@]}"; do
                if ! kill -0 "${pids[i]}" 2>/dev/null; then
                    wait "${pids[i]}"
                    unset "pids[i]"
                    ((running--))
                fi
            done
            sleep 0.1
        done
        
        # Start cached build in background
        (
            local output_file="$PROJECT_ROOT/zig-out/cursed-${zig_target}-${optimize}"
            cached_build "$zig_target" "$optimize" "$output_file"
        ) &
        
        pids+=($!)
        ((running++))
        
        log_info "Started incremental build for $zig_target:$optimize (PID: ${pids[-1]})"
    done
    
    # Wait for all builds
    for pid in "${pids[@]}"; do
        wait "$pid"
    done
    
    log_success "🎉 Incremental build system completed"
}

# Cache verification
verify_cache() {
    log_info "🔍 Verifying cache integrity"
    
    local verified=0
    local corrupted=0
    local total=0
    
    find "$ARTIFACTS_DIR" -name "metadata.json" | while read -r metadata_file; do
        ((total++))
        local cache_dir
        cache_dir=$(dirname "$metadata_file")
        local cache_key
        cache_key=$(basename "$cache_dir")
        
        # Check metadata
        if ! jq empty "$metadata_file" 2>/dev/null; then
            log_warning "Corrupted metadata: $cache_key"
            rm -rf "$cache_dir"
            ((corrupted++))
            continue
        fi
        
        # Check binary exists
        if [[ ! -f "$cache_dir/binary" && ! -f "$cache_dir/binary.gz" ]]; then
            log_warning "Missing binary: $cache_key"
            rm -rf "$cache_dir"
            ((corrupted++))
            continue
        fi
        
        ((verified++))
    done
    
    log_success "Cache verification: $verified verified, $corrupted corrupted out of $total total"
}

# Main function
main() {
    local command="${1:-help}"
    
    case "$command" in
        "init")
            init_cache
            ;;
        "build")
            shift
            if [[ $# -eq 0 ]]; then
                log_error "Usage: $0 build <target:optimize> [target:optimize...]"
                exit 1
            fi
            init_cache
            incremental_build "$@"
            ;;
        "stats")
            show_cache_stats
            ;;
        "cleanup")
            cleanup_cache
            ;;
        "verify")
            verify_cache
            ;;
        "clear")
            log_warning "Clearing entire build cache"
            rm -rf "$CACHE_DIR"
            log_success "Build cache cleared"
            ;;
        "help"|*)
            echo "CURSED Build Cache System"
            echo "Usage: $0 <command> [options]"
            echo ""
            echo "Commands:"
            echo "  init              Initialize cache system"
            echo "  build <targets>   Build with caching (format: target:optimize)"
            echo "  stats             Show cache statistics"
            echo "  cleanup           Clean expired cache entries"
            echo "  verify            Verify cache integrity"
            echo "  clear             Clear entire cache"
            echo "  help              Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 build x86_64-linux:ReleaseFast aarch64-macos:ReleaseFast"
            echo "  $0 build x86_64-linux:Debug"
            echo "  $0 stats"
            ;;
    esac
}

# Run main function
cd "$PROJECT_ROOT"
main "$@"
