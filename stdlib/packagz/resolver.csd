# CURSED Package Manager Dependency Resolution Engine
# Advanced dependency resolution with conflict detection and version constraints
yeet "arrayz"
yeet "stringz"
yeet "vibez"
yeet "mathz"

# Version constraint types
enum ConstraintType {
    Exact,      # =1.2.3
    GreaterThan,# >1.2.3
    GreaterEqual,# >=1.2.3
    LessThan,   # <1.2.3
    LessEqual,  # <=1.2.3
    Compatible, # ^1.2.3 (compatible with 1.x.x)
    Wildcard,   # ~1.2.3 (wildcard matching)
    Any         # *
}

# Version constraint specification
squad VersionConstraint {
    sus type ConstraintType
    sus version PackageVersion
}

# Dependency resolution node in the graph
squad ResolutionNode {
    sus name tea
    sus version tea
    sus metadata PackageMetadata
    sus dependencies []ResolutionNode
    sus resolved lit
    sus conflict_source tea
}

# Dependency resolution context
squad ResolutionContext {
    sus registry PackageRegistry
    sus resolved_packages []ResolutionNode
    sus pending_packages []tea
    sus conflicts []ResolutionConflict
    sus max_depth drip
    sus current_depth drip
}

# Resolution conflict information
squad ResolutionConflict {
    sus package_name tea
    sus conflicting_versions []tea
    sus required_by []tea
    sus conflict_type tea
}

# Resolution result
squad ResolutionResult {
    sus success lit
    sus resolved_packages []PackageMetadata
    sus conflicts []ResolutionConflict
    sus resolution_time drip
    sus packages_analyzed drip
}

# Initialize dependency resolver
slay init_resolver(registry PackageRegistry) ResolutionContext {
    damn ResolutionContext {
        registry: registry,
        resolved_packages: [],
        pending_packages: [],
        conflicts: [],
        max_depth: 50,
        current_depth: 0
    }
}

# Resolve dependencies for a package with conflict detection
slay resolve_dependencies(context ResolutionContext, root_packages []PackageMetadata) ResolutionResult {
    vibez.spill("Starting dependency resolution for", arrayz.len(root_packages), "root packages")
    sus start_time drip = get_timestamp_ms()
    sus packages_analyzed drip = 0
    
    # Initialize with root packages
    bestie (sus i drip = 0; i < arrayz.len(root_packages); i = i + 1) {
        sus root PackageMetadata = root_packages[i]
        sus node ResolutionNode = create_resolution_node(root, [])
        context.resolved_packages = arrayz.append(context.resolved_packages, node)
    }
    
    # Resolve dependencies recursively
    bestie (sus i drip = 0; i < arrayz.len(context.resolved_packages); i = i + 1) {
        sus result lit = resolve_node_dependencies(context, context.resolved_packages[i])
        packages_analyzed = packages_analyzed + 1
        
        ready (!result) {
            sus end_time drip = get_timestamp_ms()
            damn ResolutionResult {
                success: cap,
                resolved_packages: [],
                conflicts: context.conflicts,
                resolution_time: end_time - start_time,
                packages_analyzed: packages_analyzed
            }
        }
        
        # Prevent infinite recursion
        ready (context.current_depth > context.max_depth) {
            sus conflict ResolutionConflict = ResolutionConflict {
                package_name: "depth_limit",
                conflicting_versions: [],
                required_by: [],
                conflict_type: "Maximum dependency depth exceeded"
            }
            context.conflicts = arrayz.append(context.conflicts, conflict)
            break
        }
    }
    
    # Check for conflicts in final resolution
    sus final_conflicts []ResolutionConflict = detect_version_conflicts(context.resolved_packages)
    context.conflicts = arrayz.concat(context.conflicts, final_conflicts)
    
    sus resolved_metadata []PackageMetadata = extract_package_metadata(context.resolved_packages)
    sus end_time drip = get_timestamp_ms()
    
    vibez.spill("Resolution completed. Packages analyzed:", packages_analyzed, "Time:", end_time - start_time, "ms")
    
    damn ResolutionResult {
        success: arrayz.len(context.conflicts) == 0,
        resolved_packages: resolved_metadata,
        conflicts: context.conflicts,
        resolution_time: end_time - start_time,
        packages_analyzed: packages_analyzed
    }
}

# Resolve dependencies for a single node
slay resolve_node_dependencies(context ResolutionContext, node ResolutionNode) lit {
    ready (node.resolved) {
        damn based
    }
    
    context.current_depth = context.current_depth + 1
    vibez.spill("Resolving dependencies for", node.name, "v" + node.version, "depth:", context.current_depth)
    
    bestie (sus i drip = 0; i < arrayz.len(node.metadata.dependencies); i = i + 1) {
        sus dep PackageDependency = node.metadata.dependencies[i]
        
        ready (dep.optional && !should_install_optional_dependency(dep)) {
            continue
        }
        
        # Parse version constraint
        sus constraint VersionConstraint = parse_version_constraint(dep.version_req)
        
        # Find compatible version
        sus compatible_versions []PackageMetadata = find_compatible_versions(context.registry, dep.name, constraint)
        ready (arrayz.len(compatible_versions) == 0) {
            sus conflict ResolutionConflict = ResolutionConflict {
                package_name: dep.name,
                conflicting_versions: [],
                required_by: [node.name],
                conflict_type: "No compatible version found for constraint: " + dep.version_req
            }
            context.conflicts = arrayz.append(context.conflicts, conflict)
            damn cap
        }
        
        # Select best version (latest compatible)
        sus selected_metadata PackageMetadata = select_best_version(compatible_versions, constraint)
        
        # Check for existing resolution
        sus existing_node ResolutionNode = find_existing_resolution(context.resolved_packages, dep.name)
        ready (existing_node.name != "") {
            # Version conflict resolution
            ready (existing_node.version != selected_metadata.version) {
                ready (!can_resolve_version_conflict(existing_node, selected_metadata, constraint)) {
                    sus conflict ResolutionConflict = ResolutionConflict {
                        package_name: dep.name,
                        conflicting_versions: [existing_node.version, selected_metadata.version],
                        required_by: [existing_node.conflict_source, node.name],
                        conflict_type: "Version conflict"
                    }
                    context.conflicts = arrayz.append(context.conflicts, conflict)
                    damn cap
                }
                
                # Upgrade to newer compatible version if possible
                ready (version_is_newer_str(selected_metadata.version, existing_node.version)) {
                    existing_node.version = selected_metadata.version
                    existing_node.metadata = selected_metadata
                    existing_node.resolved = cap  # Re-resolve with new version
                }
            }
        } otherwise {
            # Add new dependency node
            sus dep_node ResolutionNode = create_resolution_node(selected_metadata, [node.name])
            context.resolved_packages = arrayz.append(context.resolved_packages, dep_node)
            node.dependencies = arrayz.append(node.dependencies, dep_node)
        }
    }
    
    node.resolved = based
    context.current_depth = context.current_depth - 1
    damn based
}

# Create resolution node from package metadata
slay create_resolution_node(metadata PackageMetadata, required_by []tea) ResolutionNode {
    damn ResolutionNode {
        name: metadata.name,
        version: metadata.version,
        metadata: metadata,
        dependencies: [],
        resolved: cap,
        conflict_source: stringz.join(required_by, ",")
    }
}

# Parse version constraint string into structured constraint
slay parse_version_constraint(constraint_str tea) VersionConstraint {
    ready (constraint_str == "" || constraint_str == "*") {
        damn VersionConstraint { type: ConstraintType.Any, version: PackageVersion{} }
    }
    
    ready (stringz.starts_with(constraint_str, "^")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.Compatible, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, "~")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.Wildcard, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, ">=")) {
        sus version_str tea = stringz.substring(constraint_str, 2, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.GreaterEqual, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, ">")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.GreaterThan, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, "<=")) {
        sus version_str tea = stringz.substring(constraint_str, 2, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.LessEqual, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, "<")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.LessThan, version: parse_version(version_str) }
    }
    
    ready (stringz.starts_with(constraint_str, "=")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.length(constraint_str))
        damn VersionConstraint { type: ConstraintType.Exact, version: parse_version(version_str) }
    }
    
    # Default to exact match
    damn VersionConstraint { type: ConstraintType.Exact, version: parse_version(constraint_str) }
}

# Find compatible package versions for constraint
slay find_compatible_versions(registry PackageRegistry, package_name tea, constraint VersionConstraint) []PackageMetadata {
    # Search registry for all versions of the package
    sus search_results []PackageMetadata = search_packages_registry(registry, package_name)
    sus compatible []PackageMetadata = []
    
    bestie (sus i drip = 0; i < arrayz.len(search_results); i = i + 1) {
        sus pkg PackageMetadata = search_results[i]
        ready (pkg.name == package_name) {
            ready (is_version_compatible(pkg.version, constraint)) {
                compatible = arrayz.append(compatible, pkg)
            }
        }
    }
    
    damn compatible
}

# Check if version satisfies constraint
slay is_version_compatible(version_str tea, constraint VersionConstraint) lit {
    sus version PackageVersion = parse_version(version_str)
    
    match constraint.type {
        ConstraintType.Any -> { damn based }
        ConstraintType.Exact -> { damn compare_versions(version, constraint.version) == 0 }
        ConstraintType.GreaterThan -> { damn compare_versions(version, constraint.version) > 0 }
        ConstraintType.GreaterEqual -> { damn compare_versions(version, constraint.version) >= 0 }
        ConstraintType.LessThan -> { damn compare_versions(version, constraint.version) < 0 }
        ConstraintType.LessEqual -> { damn compare_versions(version, constraint.version) <= 0 }
        ConstraintType.Compatible -> { damn is_compatible_version(version, constraint.version) }
        ConstraintType.Wildcard -> { damn is_wildcard_compatible(version, constraint.version) }
    }
}

# Check if version is compatible (^1.2.3 matches 1.x.x but not 2.x.x)
slay is_compatible_version(version PackageVersion, constraint_version PackageVersion) lit {
    ready (version.major != constraint_version.major) {
        damn cap
    }
    
    ready (version.minor < constraint_version.minor) {
        damn cap
    }
    
    ready (version.minor == constraint_version.minor && version.patch < constraint_version.patch) {
        damn cap
    }
    
    damn based
}

# Check if version matches wildcard constraint (~1.2.3 matches 1.2.x)
slay is_wildcard_compatible(version PackageVersion, constraint_version PackageVersion) lit {
    ready (version.major != constraint_version.major) {
        damn cap
    }
    
    ready (version.minor != constraint_version.minor) {
        damn cap
    }
    
    damn based
}

# Select best version from compatible versions (latest stable)
slay select_best_version(versions []PackageMetadata, constraint VersionConstraint) PackageMetadata {
    ready (arrayz.len(versions) == 0) {
        damn PackageMetadata{}
    }
    
    ready (arrayz.len(versions) == 1) {
        damn versions[0]
    }
    
    sus best PackageMetadata = versions[0]
    bestie (sus i drip = 1; i < arrayz.len(versions); i = i + 1) {
        sus current PackageMetadata = versions[i]
        
        # Prefer stable versions over pre-release
        ready (is_prerelease_version(best.version) && !is_prerelease_version(current.version)) {
            best = current
            continue
        }
        
        ready (!is_prerelease_version(best.version) && is_prerelease_version(current.version)) {
            continue
        }
        
        # Select newer version
        ready (version_is_newer_str(current.version, best.version)) {
            best = current
        }
    }
    
    damn best
}

# Detect version conflicts in resolved packages
slay detect_version_conflicts(resolved []ResolutionNode) []ResolutionConflict {
    sus conflicts []ResolutionConflict = []
    sus package_map map[tea]ResolutionNode = {}
    
    # Build package name to node mapping
    bestie (sus i drip = 0; i < arrayz.len(resolved); i = i + 1) {
        sus node ResolutionNode = resolved[i]
        ready (map_contains(package_map, node.name)) {
            sus existing ResolutionNode = map_get(package_map, node.name)
            ready (existing.version != node.version) {
                sus conflict ResolutionConflict = ResolutionConflict {
                    package_name: node.name,
                    conflicting_versions: [existing.version, node.version],
                    required_by: [existing.conflict_source, node.conflict_source],
                    conflict_type: "Multiple versions required"
                }
                conflicts = arrayz.append(conflicts, conflict)
            }
        } otherwise {
            package_map = map_set(package_map, node.name, node)
        }
    }
    
    damn conflicts
}

# Find existing resolution node by package name
slay find_existing_resolution(resolved []ResolutionNode, package_name tea) ResolutionNode {
    bestie (sus i drip = 0; i < arrayz.len(resolved); i = i + 1) {
        sus node ResolutionNode = resolved[i]
        ready (node.name == package_name) {
            damn node
        }
    }
    
    damn ResolutionNode { name: "", version: "", metadata: PackageMetadata{}, dependencies: [], resolved: cap, conflict_source: "" }
}

# Check if version conflict can be resolved
slay can_resolve_version_conflict(existing ResolutionNode, new_metadata PackageMetadata, constraint VersionConstraint) lit {
    # Try to find a version that satisfies both requirements
    sus existing_version PackageVersion = parse_version(existing.version)
    sus new_version PackageVersion = parse_version(new_metadata.version)
    
    # If new version is more recent and compatible, we can upgrade
    ready (is_version_compatible(new_metadata.version, constraint)) {
        ready (compare_versions(new_version, existing_version) > 0) {
            damn based
        }
    }
    
    damn cap
}

# Extract package metadata from resolution nodes
slay extract_package_metadata(nodes []ResolutionNode) []PackageMetadata {
    sus metadata []PackageMetadata = []
    bestie (sus i drip = 0; i < arrayz.len(nodes); i = i + 1) {
        sus node ResolutionNode = nodes[i]
        ready (node.resolved) {
            metadata = arrayz.append(metadata, node.metadata)
        }
    }
    damn metadata
}

# Circular dependency detection
slay detect_circular_dependencies(nodes []ResolutionNode) []ResolutionConflict {
    sus conflicts []ResolutionConflict = []
    sus visiting []tea = []
    sus visited []tea = []
    
    bestie (sus i drip = 0; i < arrayz.len(nodes); i = i + 1) {
        sus node ResolutionNode = nodes[i]
        ready (!arrayz.contains_string(visited, node.name)) {
            sus cycle []tea = find_circular_dependency(node, visiting, visited)
            ready (arrayz.len(cycle) > 0) {
                sus conflict ResolutionConflict = ResolutionConflict {
                    package_name: "circular_dependency",
                    conflicting_versions: [],
                    required_by: cycle,
                    conflict_type: "Circular dependency detected: " + stringz.join(cycle, " -> ")
                }
                conflicts = arrayz.append(conflicts, conflict)
            }
        }
    }
    
    damn conflicts
}

# Find circular dependency path
slay find_circular_dependency(node ResolutionNode, visiting []tea, visited []tea) []tea {
    ready (arrayz.contains_string(visiting, node.name)) {
        # Found cycle
        sus cycle_start drip = arrayz.index_of_string(visiting, node.name)
        damn arrayz.slice_string(visiting, cycle_start, arrayz.len(visiting))
    }
    
    ready (arrayz.contains_string(visited, node.name)) {
        damn []
    }
    
    visiting = arrayz.append(visiting, node.name)
    
    bestie (sus i drip = 0; i < arrayz.len(node.dependencies); i = i + 1) {
        sus dep_node ResolutionNode = node.dependencies[i]
        sus cycle []tea = find_circular_dependency(dep_node, visiting, visited)
        ready (arrayz.len(cycle) > 0) {
            damn cycle
        }
    }
    
    visiting = arrayz.remove_last_string(visiting)
    visited = arrayz.append(visited, node.name)
    
    damn []
}

# Helper functions (simplified implementations)
slay should_install_optional_dependency(dep PackageDependency) lit {
    # In real implementation, would check user preferences or feature flags
    damn cap
}

slay search_packages_registry(registry PackageRegistry, name tea) []PackageMetadata {
    # In real implementation, would query the registry
    damn []
}

slay is_prerelease_version(version_str tea) lit {
    damn stringz.contains(version_str, "-") || stringz.contains(version_str, "+")
}

slay version_is_newer_str(v1 tea, v2 tea) lit {
    sus version1 PackageVersion = parse_version(v1)
    sus version2 PackageVersion = parse_version(v2)
    damn compare_versions(version1, version2) > 0
}

slay get_timestamp_ms() drip {
    # Simplified timestamp for demonstration
    damn 1692703800000
}

# Simplified map operations (in real implementation would use proper hash map)
slay map_contains(m map[tea]ResolutionNode, key tea) lit {
    damn cap  # Simplified
}

slay map_get(m map[tea]ResolutionNode, key tea) ResolutionNode {
    damn ResolutionNode{}  # Simplified
}

slay map_set(m map[tea]ResolutionNode, key tea, value ResolutionNode) map[tea]ResolutionNode {
    damn m  # Simplified
}
