# Dependency Resolution Engine for CURSED Package Manager
# Handles complex dependency resolution with conflict detection
yeet "arrayz"
yeet "stringz"
yeet "vibez"
yeet "mathz"

# Dependency resolution result
enum ResolutionResult {
    Success,
    Conflict,
    CircularDependency,
    UnresolvableVersion,
    MissingPackage
}

# Version constraint types
enum VersionConstraint {
    Exact,      # =1.2.3
    Caret,      # ^1.2.3 (compatible version)
    Tilde,      # ~1.2.3 (reasonably close)
    GreaterThan, # >1.2.3
    GreaterEqual, # >=1.2.3
    LessThan,    # <1.2.3
    LessEqual,   # <=1.2.3
    Wildcard     # * (any version)
}

# Parsed version constraint
squad ParsedConstraint {
    sus constraint_type VersionConstraint
    sus version PackageVersion
    sus original tea
}

# Dependency resolution node
squad ResolutionNode {
    sus name tea
    sus version tea
    sus constraints []ParsedConstraint
    sus dependencies []tea
    sus dependents []tea
    sus resolved lit
    sus conflict_reason tea
}

# Dependency graph for resolution
squad DependencyGraph {
    sus nodes map<tea, ResolutionNode>
    sus root_dependencies []tea
    sus resolution_order []tea
}

# Dependency resolver with conflict detection
squad DependencyResolver {
    sus registry PackageRegistry
    sus graph DependencyGraph
    sus resolution_cache map<tea, []tea>
    sus version_cache map<tea, []tea>
}

# Initialize dependency resolver
slay init_resolver(registry PackageRegistry) DependencyResolver {
    sus graph DependencyGraph = DependencyGraph {
        nodes: {},
        root_dependencies: [],
        resolution_order: []
    }
    
    damn DependencyResolver {
        registry: registry,
        graph: graph,
        resolution_cache: {},
        version_cache: {}
    }
}

# Resolve dependencies for a package
slay resolve_dependencies(resolver DependencyResolver, root_deps []PackageDependency) (ResolutionResult, []InstalledPackage) {
    vibez.spill("Starting dependency resolution...")
    
    # Clear previous resolution
    resolver.graph.nodes = {}
    resolver.graph.root_dependencies = []
    resolver.graph.resolution_order = []
    
    # Add root dependencies to graph
    bestie (sus i drip = 0; i < arrayz.len(root_deps); i = i + 1) {
        sus dep PackageDependency = root_deps[i]
        resolver.graph.root_dependencies = arrayz.append(resolver.graph.root_dependencies, dep.name)
        
        ready (!add_dependency_to_graph(resolver, dep.name, dep.version_req, "")) {
            damn (ResolutionResult.MissingPackage, [])
        }
    }
    
    # Perform recursive resolution
    bestie (sus i drip = 0; i < arrayz.len(resolver.graph.root_dependencies); i = i + 1) {
        sus dep_name tea = resolver.graph.root_dependencies[i]
        sus result ResolutionResult = resolve_recursive(resolver, dep_name, [])
        
        match result {
            ResolutionResult.Conflict -> damn (ResolutionResult.Conflict, [])
            ResolutionResult.CircularDependency -> damn (ResolutionResult.CircularDependency, [])
            ResolutionResult.UnresolvableVersion -> damn (ResolutionResult.UnresolvableVersion, [])
            ResolutionResult.MissingPackage -> damn (ResolutionResult.MissingPackage, [])
            _ -> continue
        }
    }
    
    # Check for conflicts
    sus conflict_result ResolutionResult = check_version_conflicts(resolver)
    ready (conflict_result != ResolutionResult.Success) {
        damn (conflict_result, [])
    }
    
    # Generate resolution order (topological sort)
    sus order_result ResolutionResult = generate_resolution_order(resolver)
    ready (order_result != ResolutionResult.Success) {
        damn (order_result, [])
    }
    
    # Create installation plan
    sus installation_plan []InstalledPackage = create_installation_plan(resolver)
    
    vibez.spill("Dependency resolution successful:", arrayz.len(installation_plan), "packages")
    damn (ResolutionResult.Success, installation_plan)
}

# Add dependency to resolution graph
slay add_dependency_to_graph(resolver DependencyResolver, name tea, version_req tea, dependent tea) lit {
    ready (name == "") {
        damn cap
    }
    
    # Check if already in graph
    ready (map_has_key(resolver.graph.nodes, name)) {
        sus existing ResolutionNode = resolver.graph.nodes[name]
        
        # Add constraint if not already present
        sus constraint ParsedConstraint = parse_version_constraint(version_req)
        ready (!constraint_already_exists(existing.constraints, constraint)) {
            existing.constraints = arrayz.append(existing.constraints, constraint)
        }
        
        # Add dependent relationship
        ready (dependent != "" && !arrayz.contains(existing.dependents, dependent)) {
            existing.dependents = arrayz.append(existing.dependents, dependent)
        }
        
        resolver.graph.nodes[name] = existing
        damn based
    }
    
    # Get available versions for this package
    sus available_versions []tea = get_available_versions(resolver, name)
    ready (arrayz.len(available_versions) == 0) {
        vibez.spill("No versions found for package:", name)
        damn cap
    }
    
    # Create new resolution node
    sus constraint ParsedConstraint = parse_version_constraint(version_req)
    sus node ResolutionNode = ResolutionNode {
        name: name,
        version: "",  # Will be resolved later
        constraints: [constraint],
        dependencies: [],
        dependents: [],
        resolved: cap,
        conflict_reason: ""
    }
    
    ready (dependent != "") {
        node.dependents = arrayz.append(node.dependents, dependent)
    }
    
    resolver.graph.nodes[name] = node
    damn based
}

# Parse version constraint string
slay parse_version_constraint(constraint_str tea) ParsedConstraint {
    ready (constraint_str == "" || constraint_str == "*") {
        damn ParsedConstraint {
            constraint_type: VersionConstraint.Wildcard,
            version: PackageVersion{},
            original: constraint_str
        }
    }
    
    # Check for operators
    ready (stringz.starts_with(constraint_str, "^")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.Caret,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, "~")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.Tilde,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, ">=")) {
        sus version_str tea = stringz.substring(constraint_str, 2, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.GreaterEqual,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, "<=")) {
        sus version_str tea = stringz.substring(constraint_str, 2, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.LessEqual,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, ">")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.GreaterThan,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, "<")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.LessThan,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    ready (stringz.starts_with(constraint_str, "=")) {
        sus version_str tea = stringz.substring(constraint_str, 1, stringz.len(constraint_str))
        damn ParsedConstraint {
            constraint_type: VersionConstraint.Exact,
            version: parse_version(version_str),
            original: constraint_str
        }
    }
    
    # Default to exact version
    damn ParsedConstraint {
        constraint_type: VersionConstraint.Exact,
        version: parse_version(constraint_str),
        original: constraint_str
    }
}

# Check if constraint already exists in list
slay constraint_already_exists(constraints []ParsedConstraint, new_constraint ParsedConstraint) lit {
    bestie (sus i drip = 0; i < arrayz.len(constraints); i = i + 1) {
        sus existing ParsedConstraint = constraints[i]
        ready (existing.original == new_constraint.original) {
            damn based
        }
    }
    damn cap
}

# Get available versions for a package (with caching)
slay get_available_versions(resolver DependencyResolver, name tea) []tea {
    ready (map_has_key(resolver.version_cache, name)) {
        damn resolver.version_cache[name]
    }
    
    sus versions []tea = list_package_versions(resolver.registry, name)
    resolver.version_cache[name] = versions
    damn versions
}

# Recursively resolve dependencies
slay resolve_recursive(resolver DependencyResolver, name tea, resolution_path []tea) ResolutionResult {
    # Check for circular dependency
    ready (arrayz.contains(resolution_path, name)) {
        vibez.spill("Circular dependency detected:", name, "in path:", stringz.join(resolution_path, " -> "))
        damn ResolutionResult.CircularDependency
    }
    
    sus node ResolutionNode = resolver.graph.nodes[name]
    ready (node.resolved) {
        damn ResolutionResult.Success  # Already resolved
    }
    
    # Find compatible version
    sus compatible_version tea = find_compatible_version(resolver, name, node.constraints)
    ready (compatible_version == "") {
        node.conflict_reason = "No compatible version found"
        resolver.graph.nodes[name] = node
        damn ResolutionResult.UnresolvableVersion
    }
    
    node.version = compatible_version
    node.resolved = based
    resolver.graph.nodes[name] = node
    
    # Get package metadata to find dependencies
    sus metadata PackageMetadata = get_package_info(resolver.registry, name, compatible_version)
    ready (metadata.name == "") {
        damn ResolutionResult.MissingPackage
    }
    
    # Add new resolution path entry
    sus new_path []tea = arrayz.append(resolution_path, name)
    
    # Recursively resolve dependencies
    bestie (sus i drip = 0; i < arrayz.len(metadata.dependencies); i = i + 1) {
        sus dep PackageDependency = metadata.dependencies[i]
        ready (!dep.optional) {  # Skip optional dependencies for now
            # Add dependency to graph
            ready (!add_dependency_to_graph(resolver, dep.name, dep.version_req, name)) {
                damn ResolutionResult.MissingPackage
            }
            
            # Update node dependencies
            node.dependencies = arrayz.append(node.dependencies, dep.name)
            resolver.graph.nodes[name] = node
            
            # Recursively resolve
            sus result ResolutionResult = resolve_recursive(resolver, dep.name, new_path)
            ready (result != ResolutionResult.Success) {
                damn result
            }
        }
    }
    
    damn ResolutionResult.Success
}

# Find version that satisfies all constraints
slay find_compatible_version(resolver DependencyResolver, name tea, constraints []ParsedConstraint) tea {
    sus available_versions []tea = get_available_versions(resolver, name)
    ready (arrayz.len(available_versions) == 0) {
        damn ""
    }
    
    # Sort versions in descending order (prefer latest)
    sus sorted_versions []tea = sort_versions_descending(available_versions)
    
    # Test each version against all constraints
    bestie (sus i drip = 0; i < arrayz.len(sorted_versions); i = i + 1) {
        sus version tea = sorted_versions[i]
        sus version_obj PackageVersion = parse_version(version)
        sus compatible lit = based
        
        # Check against all constraints
        bestie (sus j drip = 0; j < arrayz.len(constraints); j = j + 1) {
            sus constraint ParsedConstraint = constraints[j]
            ready (!version_satisfies_constraint(version_obj, constraint)) {
                compatible = cap
                break
            }
        }
        
        ready (compatible) {
            damn version
        }
    }
    
    damn ""  # No compatible version found
}

# Check if version satisfies constraint
slay version_satisfies_constraint(version PackageVersion, constraint ParsedConstraint) lit {
    match constraint.constraint_type {
        VersionConstraint.Wildcard -> damn based
        
        VersionConstraint.Exact -> {
            damn compare_versions(version, constraint.version) == 0
        }
        
        VersionConstraint.Caret -> {
            # ^1.2.3 allows >=1.2.3 but <2.0.0
            ready (version.major != constraint.version.major) {
                damn cap
            }
            damn compare_versions(version, constraint.version) >= 0
        }
        
        VersionConstraint.Tilde -> {
            # ~1.2.3 allows >=1.2.3 but <1.3.0
            ready (version.major != constraint.version.major || version.minor != constraint.version.minor) {
                damn cap
            }
            damn compare_versions(version, constraint.version) >= 0
        }
        
        VersionConstraint.GreaterThan -> {
            damn compare_versions(version, constraint.version) > 0
        }
        
        VersionConstraint.GreaterEqual -> {
            damn compare_versions(version, constraint.version) >= 0
        }
        
        VersionConstraint.LessThan -> {
            damn compare_versions(version, constraint.version) < 0
        }
        
        VersionConstraint.LessEqual -> {
            damn compare_versions(version, constraint.version) <= 0
        }
    }
    
    damn cap
}

# Sort versions in descending order (latest first)
slay sort_versions_descending(versions []tea) []tea {
    sus sorted []tea = arrayz.copy(versions)
    
    # Simple bubble sort by version comparison
    sus n drip = arrayz.len(sorted)
    bestie (sus i drip = 0; i < n - 1; i = i + 1) {
        bestie (sus j drip = 0; j < n - i - 1; j = j + 1) {
            sus v1 PackageVersion = parse_version(sorted[j])
            sus v2 PackageVersion = parse_version(sorted[j + 1])
            
            ready (compare_versions(v1, v2) < 0) {
                # Swap to get descending order
                sus temp tea = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    
    damn sorted
}

# Check for version conflicts in resolved graph
slay check_version_conflicts(resolver DependencyResolver) ResolutionResult {
    # Check each resolved package for conflicts
    sus package_names []tea = get_map_keys(resolver.graph.nodes)
    
    bestie (sus i drip = 0; i < arrayz.len(package_names); i = i + 1) {
        sus name tea = package_names[i]
        sus node ResolutionNode = resolver.graph.nodes[name]
        
        ready (!node.resolved) {
            continue
        }
        
        sus resolved_version PackageVersion = parse_version(node.version)
        
        # Check if resolved version satisfies all constraints
        bestie (sus j drip = 0; j < arrayz.len(node.constraints); j = j + 1) {
            sus constraint ParsedConstraint = node.constraints[j]
            ready (!version_satisfies_constraint(resolved_version, constraint)) {
                vibez.spill("Version conflict for package:", name)
                vibez.spill("Resolved version:", node.version, "does not satisfy constraint:", constraint.original)
                damn ResolutionResult.Conflict
            }
        }
    }
    
    damn ResolutionResult.Success
}

# Generate resolution order using topological sort
slay generate_resolution_order(resolver DependencyResolver) ResolutionResult {
    resolver.graph.resolution_order = []
    sus visited map<tea, lit> = {}
    sus temp_visited map<tea, lit> = {}
    
    sus package_names []tea = get_map_keys(resolver.graph.nodes)
    
    bestie (sus i drip = 0; i < arrayz.len(package_names); i = i + 1) {
        sus name tea = package_names[i]
        ready (!map_has_key(visited, name)) {
            sus result ResolutionResult = topological_sort_visit(resolver, name, visited, temp_visited)
            ready (result != ResolutionResult.Success) {
                damn result
            }
        }
    }
    
    # Reverse order (dependencies first)
    resolver.graph.resolution_order = arrayz.reverse(resolver.graph.resolution_order)
    
    damn ResolutionResult.Success
}

# Topological sort helper (recursive)
slay topological_sort_visit(resolver DependencyResolver, name tea, visited map<tea, lit>, temp_visited map<tea, lit>) ResolutionResult {
    ready (map_has_key(temp_visited, name)) {
        vibez.spill("Circular dependency detected in topological sort:", name)
        damn ResolutionResult.CircularDependency
    }
    
    ready (map_has_key(visited, name)) {
        damn ResolutionResult.Success
    }
    
    temp_visited[name] = based
    
    sus node ResolutionNode = resolver.graph.nodes[name]
    bestie (sus i drip = 0; i < arrayz.len(node.dependencies); i = i + 1) {
        sus dep_name tea = node.dependencies[i]
        sus result ResolutionResult = topological_sort_visit(resolver, dep_name, visited, temp_visited)
        ready (result != ResolutionResult.Success) {
            damn result
        }
    }
    
    temp_visited = map_remove(temp_visited, name)
    visited[name] = based
    resolver.graph.resolution_order = arrayz.append(resolver.graph.resolution_order, name)
    
    damn ResolutionResult.Success
}

# Create installation plan from resolved graph
slay create_installation_plan(resolver DependencyResolver) []InstalledPackage {
    sus plan []InstalledPackage = []
    
    bestie (sus i drip = 0; i < arrayz.len(resolver.graph.resolution_order); i = i + 1) {
        sus name tea = resolver.graph.resolution_order[i]
        sus node ResolutionNode = resolver.graph.nodes[name]
        
        ready (node.resolved && node.version != "") {
            sus package InstalledPackage = InstalledPackage {
                name: name,
                version: node.version,
                install_path: "",  # Will be set during installation
                installed_at: "",  # Will be set during installation
                dependencies: node.dependencies
            }
            plan = arrayz.append(plan, package)
        }
    }
    
    damn plan
}

# Print dependency resolution graph for debugging
slay print_resolution_graph(resolver DependencyResolver) {
    vibez.spill("Dependency Resolution Graph:")
    vibez.spill("===========================")
    
    sus package_names []tea = get_map_keys(resolver.graph.nodes)
    
    bestie (sus i drip = 0; i < arrayz.len(package_names); i = i + 1) {
        sus name tea = package_names[i]
        sus node ResolutionNode = resolver.graph.nodes[name]
        
        vibez.spill("Package:", name)
        vibez.spill("  Version:", node.version)
        vibez.spill("  Resolved:", node.resolved)
        
        ready (arrayz.len(node.constraints) > 0) {
            vibez.spill("  Constraints:")
            bestie (sus j drip = 0; j < arrayz.len(node.constraints); j = j + 1) {
                sus constraint ParsedConstraint = node.constraints[j]
                vibez.spill("    -", constraint.original)
            }
        }
        
        ready (arrayz.len(node.dependencies) > 0) {
            vibez.spill("  Dependencies:", stringz.join(node.dependencies, ", "))
        }
        
        ready (arrayz.len(node.dependents) > 0) {
            vibez.spill("  Dependents:", stringz.join(node.dependents, ", "))
        }
        
        ready (node.conflict_reason != "") {
            vibez.spill("  Conflict:", node.conflict_reason)
        }
        
        vibez.spill("")
    }
    
    ready (arrayz.len(resolver.graph.resolution_order) > 0) {
        vibez.spill("Resolution Order:", stringz.join(resolver.graph.resolution_order, " -> "))
    }
}

# Utility functions
slay get_map_keys(m map<tea, ResolutionNode>) []tea {
    # In real implementation: extract keys from map
    damn []
}

slay map_has_key(m map<tea, ResolutionNode>, key tea) lit {
    # In real implementation: check if key exists in map
    damn cap
}

slay map_remove(m map<tea, lit>, key tea) map<tea, lit> {
    # In real implementation: remove key from map
    damn m
}
