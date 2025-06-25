/// High-performance route matching using radix tree algorithm
/// 
/// Implements efficient path pattern matching with support for:
/// - Static segments: /users/profile
/// - Named parameters: /users/:id 
/// - Wildcards: /files/*
/// - Optional segments: /api/v1?/users
/// - Complex patterns: /users/:id/posts/:post_id

use std::collections::HashMap;
use std::fmt;
use tracing::{debug, trace, instrument};

/// Types of path segments in route patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    /// Static segment: exact string match
    /// Named parameter: captures value as :name
    /// Wildcard: captures remaining path as *
    /// Optional segment: may or may not be present
/// Types of wildcard matching
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WildcardType {
    /// Single segment wildcard (*)
    /// Multi-segment wildcard (**)
    /// Named wildcard (*name)
/// A compiled route pattern for efficient matching
#[derive(Debug, Clone)]
pub struct RoutePattern {
    /// Original pattern string
    /// Compiled segments
    /// Parameter names in order
    /// Whether pattern has wildcards
    /// Pattern priority for conflict resolution
impl RoutePattern {
    /// Compile a route pattern string into optimized segments
    #[instrument]
    pub fn compile(pattern: &str) -> Result<Self, String> {
        let mut segments = Vec::new();
        let mut param_names = Vec::new();
        let mut has_wildcards = false;
        let mut priority = 0;

        // Split pattern into segments
        let parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
        
        for part in parts {
            let segment = if part.starts_with(':') {
                // Named parameter: :id
                let name = part[1..].to_string();
                if name.is_empty() {
                    return Err("Empty parameter name".to_string());
                }
                param_names.push(name.clone());
                priority += 1; // Parameters have lower priority than static
                PathSegment::Parameter(name)
            } else if part.starts_with('*') {
                // Wildcard: *, *name, **
                has_wildcards = true;
                priority += 2; // Wildcards have lowest priority
                if part == "*" {
                    PathSegment::Wildcard("*".to_string())
                } else if part == "**" {
                    PathSegment::Wildcard("**".to_string())
                } else {
                    let name = part[1..].to_string();
                    param_names.push(name.clone());
                    PathSegment::Wildcard(name)
                }
            } else if part.ends_with('?') {
                // Optional segment: segment?
                let base_part = &part[..part.len() - 1];
                let base_segment = if base_part.starts_with(':') {
                    let name = base_part[1..].to_string();
                    param_names.push(name.clone());
                    PathSegment::Parameter(name)
                } else {
                    PathSegment::Static(base_part.to_string())
                priority += 1;
                PathSegment::Optional(Box::new(base_segment))
            } else {
                // Static segment
                PathSegment::Static(part.to_string())
            
            segments.push(segment);
        Ok(RoutePattern {
        })
    /// Check if this pattern matches the given path
    #[instrument(skip(self))]
    pub fn matches(&self, path: &str) -> Option<RouteMatch> {
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut params = HashMap::new();
        let mut segment_idx = 0;
        let mut path_idx = 0;

        while segment_idx < self.segments.len() && path_idx < path_segments.len() {
            match &self.segments[segment_idx] {
                PathSegment::Static(expected) => {
                    if path_segments[path_idx] != expected {
                        return None;
                    }
                    path_idx += 1;
                }
                PathSegment::Parameter(name) => {
                    params.insert(name.clone(), path_segments[path_idx].to_string());
                    path_idx += 1;
                }
                PathSegment::Wildcard(name) => {
                    if name == "*" {
                        // Single segment wildcard
                        path_idx += 1;
                    } else if name == "**" {
                        // Multi-segment wildcard - consume rest of path
                        let remaining: Vec<&str> = path_segments[path_idx..].to_vec();
                        params.insert("**".to_string(), remaining.join("/"));
                        path_idx = path_segments.len();
                    } else {
                        // Named wildcard
                        params.insert(name.clone(), path_segments[path_idx].to_string());
                        path_idx += 1;
                    }
                }
                PathSegment::Optional(inner) => {
                    // Try to match optional segment
                    match inner.as_ref() {
                        PathSegment::Static(expected) => {
                            if path_segments[path_idx] == expected {
                                path_idx += 1;
                            }
                            // If no match, skip this optional segment
                        }
                        PathSegment::Parameter(name) => {
                            // Optional parameter always captures if segment exists
                            params.insert(name.clone(), path_segments[path_idx].to_string());
                            path_idx += 1;
                        }
                        _ => {
                            // Other optional types not supported yet
                        }
                    }
                }
            }
            segment_idx += 1;
        // Handle remaining segments
        if segment_idx < self.segments.len() {
            // Check if remaining segments are all optional
            for remaining_segment in &self.segments[segment_idx..] {
                if !matches!(remaining_segment, PathSegment::Optional(_)) {
                    return None;
                }
            }
        // Check if path fully consumed (unless we have wildcards)
        if path_idx < path_segments.len() && !self.has_wildcards {
            return None;
        Some(RouteMatch {
        })
    }
}

/// Result of a successful route match
#[derive(Debug, Clone)]
pub struct RouteMatch {
    /// The pattern that matched
    /// Extracted parameters
    /// The path that was matched
impl RouteMatch {
    /// Get a parameter value by name
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.get(name).map(|s| s.as_str())
    /// Get all parameters
    pub fn params(&self) -> &HashMap<String, String> {
        &self.params
    }
}

/// Node in the radix tree for efficient route lookup
#[derive(Debug)]
pub struct RouteNode {
    /// Segment this node represents
    /// Child nodes
    /// Parameter child (for :param segments)
    /// Wildcard child (for * segments)
    /// Route patterns that end at this node
impl RouteNode {
    pub fn new() -> Self {
        Self {
        }
    }
/// High-performance route matcher using radix tree
#[derive(Debug)]
pub struct RouteMatcher {
    /// Root of the radix tree
    /// Cache for frequently matched patterns
    /// Maximum cache size
    /// Statistics for performance monitoring
/// Performance statistics for route matching
#[derive(Debug, Default)]
pub struct MatcherStats {
impl RouteMatcher {
    /// Create a new route matcher
    pub fn new(max_cache_size: usize) -> Self {
        Self {
        }
    }

    /// Add a route pattern to the matcher
    #[instrument(skip(self))]
    pub fn add_route(&mut self, pattern: &str) -> Result<(), String> {
        let route_pattern = RoutePattern::compile(pattern)?;
        self.stats.patterns_compiled += 1;
        
        // Insert into radix tree
        self.insert_pattern(&route_pattern);
        
        // Clear cache when new routes are added
        self.cache.clear();
        
        debug!(pattern = %pattern, priority = route_pattern.priority, "Added route pattern");
        Ok(())
    /// Insert a pattern into the radix tree
    fn insert_pattern(&mut self, pattern: &RoutePattern) {
        let mut current = &mut self.root;
        
        for segment in &pattern.segments {
            match segment {
                PathSegment::Static(text) => {
                    current = current.children.entry(text.clone()).or_insert_with(RouteNode::new);
                }
                PathSegment::Parameter(_) => {
                    if current.param_child.is_none() {
                        current.param_child = Some(Box::new(RouteNode::new()));
                    }
                    current = current.param_child.as_mut().unwrap();
                }
                PathSegment::Wildcard(_) => {
                    if current.wildcard_child.is_none() {
                        current.wildcard_child = Some(Box::new(RouteNode::new()));
                    }
                    current = current.wildcard_child.as_mut().unwrap();
                }
                PathSegment::Optional(_) => {
                    // Optional segments are handled during matching
                }
            }
        current.endpoints.push(pattern.clone());
        // Sort endpoints by priority (lower number = higher priority)
        current.endpoints.sort_by_key(|p| p.priority);
    /// Find matching route for a path
    #[instrument(skip(self))]
    pub fn find_match(&mut self, path: &str) -> Option<RouteMatch> {
        let start_time = std::time::Instant::now();
        self.stats.total_lookups += 1;
        
        // Check cache first
        if let Some(cached_result) = self.cache.get(path) {
            self.stats.cache_hits += 1;
            trace!(path = %path, cached = true, "Route lookup from cache");
            return cached_result.clone();
        self.stats.cache_misses += 1;
        
        // Perform tree traversal to find matches
        let result = self.find_matches_in_tree(path);
        
        // Update cache if under size limit
        if self.cache.len() < self.max_cache_size {
            self.cache.insert(path.to_string(), result.clone());
        // Update timing statistics
        let elapsed = start_time.elapsed();
        let elapsed_ns = elapsed.as_nanos() as u64;
        self.stats.average_lookup_time_ns = 
            (self.stats.average_lookup_time_ns * (self.stats.total_lookups - 1) + elapsed_ns) 
            / self.stats.total_lookups;
        
        trace!(
            "Route lookup completed"
        );
        
        result
    /// Find all possible matches in the radix tree
    fn find_matches_in_tree(&self, path: &str) -> Option<RouteMatch> {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut candidates = Vec::new();
        
        self.collect_matches(&self.root, &segments, 0, &mut HashMap::new(), &mut candidates);
        
        // Return the highest priority match (lowest priority number)
        candidates.into_iter()
            .filter_map(|pattern| pattern.matches(path))
            .min_by_key(|m| m.pattern.priority)
    /// Recursively collect matching patterns from the tree
    fn collect_matches(
    ) {
        // If we've consumed all segments, check for endpoints
        if segment_idx >= segments.len() {
            candidates.extend(node.endpoints.iter().cloned());
            return;
        let current_segment = segments[segment_idx];
        
        // Try static children
        if let Some(static_child) = node.children.get(current_segment) {
            self.collect_matches(static_child, segments, segment_idx + 1, params, candidates);
        // Try parameter child
        if let Some(param_child) = &node.param_child {
            let old_len = params.len();
            // We'll let the actual pattern matching handle parameter extraction
            self.collect_matches(param_child, segments, segment_idx + 1, params, candidates);
            // Restore params state
            while params.len() > old_len {
                params.remove(&format!("temp_{}", params.len()));
            }
        }
        
        // Try wildcard child
        if let Some(wildcard_child) = &node.wildcard_child {
            // Wildcards can match remaining segments
            self.collect_matches(wildcard_child, segments, segments.len(), params, candidates);
        }
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> &MatcherStats {
        &self.stats
    /// Clear the route cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    /// Get number of cached routes
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        }
    }
impl fmt::Display for RoutePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/{}", self.segments.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("/"))
    }
}

