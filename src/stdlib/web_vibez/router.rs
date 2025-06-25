use crate::web::StatusCode;
/// High-performance HTTP router with flexible pattern matching
/// 
/// Supports:
/// - HTTP method-specific routing
/// - Path parameters and wildcards
/// - Route groups and nesting
/// - Route priority and conflict resolution
/// - Middleware integration

// use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
// use crate::stdlib::web_vibez::route_matcher::{RouteMatcher, RouteMatch, RoutePattern};
// use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
// use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
// use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareChain};
use crate::error::CursedError;

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn, instrument};

/// Route priority levels for conflict resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutePriority {
    /// Highest priority - exact static matches
    /// High priority - patterns with parameters
    /// Medium priority - patterns with single wildcards
    /// Low priority - patterns with multi-segment wildcards
    /// Lowest priority - fallback routes
/// A registered route in the router
#[derive(Debug, Clone)]
pub struct Route {
    /// HTTP method for this route
    /// Route pattern (e.g., "/users/:id")
    /// Compiled route pattern for efficient matching
    /// Handler for this route
    /// Route-specific middleware
    /// Route priority
    /// Route name for debugging and reverse routing
    /// Route metadata
impl Route {
    /// Create a new route
    #[instrument(skip(handler, middleware))]
    pub fn new(
    ) -> crate::error::Result<()> {
        let compiled_pattern = RoutePattern::compile(pattern)
            .map_err(|e| RouterError::InvalidPattern(pattern.to_string(), e))?;
        
        let priority = Self::calculate_priority(&compiled_pattern);
        
        Ok(Route {
        })
    /// Calculate route priority based on pattern complexity
    fn calculate_priority(pattern: &RoutePattern) -> RoutePriority {
        if pattern.has_wildcards {
//             if pattern.segments.iter().any(|s| matches!(s, crate::stdlib::web_vibez::route_matcher::PathSegment::Wildcard(name) if name == "**")) {
                RoutePriority::CatchAll
            } else {
                RoutePriority::Wildcard
            }
        } else if !pattern.param_names.is_empty() {
            RoutePriority::Parameterized
        } else {
            RoutePriority::Exact
        }
    }

    /// Set route name for debugging and reverse routing
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    /// Add metadata to the route
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Route group for organizing related routes
#[derive(Debug)]
pub struct RouteGroup {
    /// Base path prefix for all routes in this group
    /// Group-level middleware applied to all routes
    /// Routes in this group
    /// Nested subgroups
    /// Group name for debugging
impl RouteGroup {
    /// Create a new route group
    pub fn new(prefix: &str) -> Self {
        Self {
        }
    }

    /// Add middleware to this group
    pub fn with_middleware(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    /// Set group name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    /// Add a route to this group
    pub fn add_route(&mut self, mut route: Route) {
        // Prepend group prefix to route pattern
        if !self.prefix.is_empty() {
            let full_pattern = format!("{}{}", self.prefix.trim_end_matches('/'), &route.pattern);
            route.pattern = full_pattern.clone();
            route.compiled_pattern = RoutePattern::compile(&full_pattern).unwrap();
        // Add group middleware to route
        let mut combined_middleware = self.middleware.clone();
        combined_middleware.extend(route.middleware.clone());
        route.middleware = combined_middleware;
        
        self.routes.push(route);
    /// Add a subgroup to this group
    pub fn add_subgroup(&mut self, name: &str, mut subgroup: RouteGroup) {
        // Update subgroup prefix to include parent prefix
        if !self.prefix.is_empty() {
            subgroup.prefix = format!("{}{}", self.prefix.trim_end_matches('/'), &subgroup.prefix);
        // Add parent middleware to subgroup
        let mut combined_middleware = self.middleware.clone();
        combined_middleware.extend(subgroup.middleware.clone());
        subgroup.middleware = combined_middleware;
        
        self.subgroups.insert(name.to_string(), subgroup);
    /// Get all routes including from subgroups
    pub fn all_routes(&self) -> Vec<Route> {
        let mut all_routes = self.routes.clone();
        
        for subgroup in self.subgroups.values() {
            all_routes.extend(subgroup.all_routes());
        all_routes
    }
}

/// Matched route information
#[derive(Debug)]
pub struct MatchedRoute {
    /// The matched route
    /// Route match details with parameters
    /// Combined middleware chain for this route
/// High-performance HTTP router
#[derive(Debug)]
pub struct Router {
    /// Route matchers for each HTTP method
    /// All registered routes
    /// Route groups
    /// Global middleware applied to all routes
    /// Router configuration
    /// Performance statistics
/// Router configuration
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Maximum size for route cache per method
    /// Enable route debugging
    /// Case sensitive route matching
    /// Strict slash matching (/path vs /path/)
    /// Maximum route priority conflicts to report
impl Default for RouterConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Router performance statistics
#[derive(Debug, Default)]
pub struct RouterStats {
impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self::with_config(RouterConfig::default())
    /// Create a new router with custom configuration
    pub fn with_config(config: RouterConfig) -> Self {
        Self {
        }
    }

    /// Add global middleware to all routes
    #[instrument(skip(self, middleware))]
    pub fn use_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        debug!("Adding global middleware");
        self.global_middleware.push(middleware);
    /// Register a new route
    #[instrument(skip(self, handler, middleware))]
    pub fn add_route(
    ) -> crate::error::Result<()> {
        let route = Route::new(method, pattern, handler, middleware)?;
        
        // Add global middleware to route
        let mut combined_middleware = self.global_middleware.clone();
        combined_middleware.extend(route.middleware.clone());
        let mut route = route;
        route.middleware = combined_middleware;
        
        // Add route to appropriate matcher
        let matcher = self.matchers.entry(method).or_insert_with(|| {
            RouteMatcher::new(self.config.max_cache_size_per_method)
        });
        
        matcher.add_route(pattern)?;
        
        // Check for priority conflicts
        self.check_priority_conflicts(&route);
        
        self.routes.push(route);
        self.stats.total_routes = self.routes.len();
        
        info!(method = %method, pattern = %pattern, "Route registered");
        Ok(())
    /// Convenience method for GET routes
    pub fn get(
    ) -> crate::error::Result<()> {
        self.add_route(HttpMethod::GET, pattern, handler, Vec::from([]))
    /// Convenience method for POST routes
    pub fn post(
    ) -> crate::error::Result<()> {
        self.add_route(HttpMethod::POST, pattern, handler, Vec::from([]))
    /// Convenience method for PUT routes
    pub fn put(
    ) -> crate::error::Result<()> {
        self.add_route(HttpMethod::PUT, pattern, handler, Vec::from([]))
    /// Convenience method for DELETE routes
    pub fn delete(
    ) -> crate::error::Result<()> {
        self.add_route(HttpMethod::DELETE, pattern, handler, Vec::from([]))
    /// Add a route group
    #[instrument(skip(self, group))]
    pub fn add_group(&mut self, name: &str, group: RouteGroup) {
        for route in group.all_routes() {
            if let Err(e) = self.add_route(route.method, &route.pattern, route.handler, route.middleware) {
                warn!(error = %e, "Failed to add route from group");
            }
        }
        
        self.groups.insert(name.to_string(), group);
        info!(name = %name, "Route group added");
    /// Find a matching route for the given method and path
    #[instrument(skip(self))]
    pub fn find_route(&mut self, method: HttpMethod, path: &str) -> Option<MatchedRoute> {
        let start_time = std::time::Instant::now();
        self.stats.total_lookups += 1;
        
        // Normalize path if needed
        let normalized_path = if self.config.case_sensitive {
            path.to_string()
        } else {
            path.to_lowercase()
        
        let matcher = self.matchers.get_mut(&method)?;
        let route_match = matcher.find_match(&normalized_path)?;
        
        // Find the corresponding route
        let route = self.routes.iter()
            .find(|r| r.method == method && r.pattern == route_match.pattern.pattern)?
            .clone();
        
        // Build middleware chain
        let middleware_chain = MiddlewareChain::new(route.middleware.clone());
        
        // Update statistics
        self.stats.successful_matches += 1;
        let elapsed = start_time.elapsed();
        let elapsed_ns = elapsed.as_nanos() as u64;
        self.stats.average_lookup_time_ns = 
            (self.stats.average_lookup_time_ns * (self.stats.total_lookups - 1) + elapsed_ns) 
            / self.stats.total_lookups;
        
        // Update cache hit rate
        let matcher_stats = matcher.get_stats();
        self.stats.cache_hit_rate = matcher_stats.cache_hits as f64 / matcher_stats.total_lookups as f64;
        
        debug!(
            "Route matched successfully"
        );
        
        Some(MatchedRoute {
        })
    /// Handle a request through the router
    #[instrument(skip(self, context))]
    pub async fn handle_request(
    ) -> HandlerResult {
        match self.find_route(method, path) {
            Some(matched_route) => {
                // Add route parameters to context
                for (key, value) in matched_route.route_match.params() {
                    context.add_param(key, value);
                // Execute middleware chain and handler
                let response_context = ResponseContext::new();
                matched_route.middleware_chain
                    .execute(context, response_context, matched_route.route.handler)
            }
            None => {
                self.stats.failed_matches += 1;
                warn!(method = %method, path = %path, "No route found");
                
                // Return 404 Not Found
                let mut response = ResponseContext::new();
                response.set_status(StatusCode::NotFound);
                response.set_body("Not Found".as_bytes().to_vec());
                Ok(response)
            }
        }
    /// Check for route priority conflicts
    fn check_priority_conflicts(&mut self, new_route: &Route) {
        if self.stats.priority_conflicts >= self.config.max_priority_conflicts as u32 {
            return;
        for existing_route in &self.routes {
            if existing_route.method == new_route.method 
                && existing_route.priority == new_route.priority
                && patterns_could_conflict(&existing_route.pattern, &new_route.pattern) {
                
                warn!(
                    "Potential route priority conflict detected"
                );
                
                self.stats.priority_conflicts += 1;
            }
        }
    /// Get router statistics
    pub fn get_stats(&self) -> &RouterStats {
        &self.stats
    /// Clear all route caches
    pub fn clear_caches(&mut self) {
        for matcher in self.matchers.values_mut() {
            matcher.clear_cache();
        }
        info!("Route caches cleared");
    /// Get all registered routes
    pub fn get_routes(&self) -> &[Route] {
        &self.routes
    /// Get route groups
    pub fn get_groups(&self) -> &HashMap<String, RouteGroup> {
        &self.groups
    }
}

/// Check if two route patterns could potentially conflict
fn patterns_could_conflict(pattern1: &str, pattern2: &str) -> bool {
    // Simple heuristic: patterns conflict if they have the same number of segments
    // and similar structure (both static, both parameterized, etc.)
    let segments1: Vec<&str> = pattern1.split('/').filter(|s| !s.is_empty()).collect();
    let segments2: Vec<&str> = pattern2.split('/').filter(|s| !s.is_empty()).collect();
    
    if segments1.len() != segments2.len() {
        return false;
    for (seg1, seg2) in segments1.iter().zip(segments2.iter()) {
        // If both are static and different, no conflict
        if !seg1.starts_with(':') && !seg1.starts_with('*') 
            && !seg2.starts_with(':') && !seg2.starts_with('*')
            && seg1 != seg2 {
            return false;
        }
    }
    
    true
impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

