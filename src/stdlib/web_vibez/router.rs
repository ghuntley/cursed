use crate::web::StatusCode;
/// High-performance HTTP router with flexible pattern matching
/// 
/// Supports:
/// - HTTP method-specific routing
/// - Path parameters and wildcards
/// - Route groups and nesting
/// - Route priority and conflict resolution
/// - Middleware integration

use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
use crate::stdlib::web_vibez::route_matcher::{RouteMatcher, RouteMatch, RoutePattern};
use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareChain};
use crate::stdlib::web_vibez::error_handling::RouterError;
use crate::error::Error;

use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn, instrument};

/// Route priority levels for conflict resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutePriority {
    /// Highest priority - exact static matches
    Exact = 0,
    /// High priority - patterns with parameters
    Parameterized = 1,
    /// Medium priority - patterns with single wildcards
    Wildcard = 2,
    /// Low priority - patterns with multi-segment wildcards
    CatchAll = 3,
    /// Lowest priority - fallback routes
    Fallback = 4,
}

/// A registered route in the router
#[derive(Debug, Clone)]
pub struct Route {
    /// HTTP method for this route
    pub method: HttpMethod,
    /// Route pattern (e.g., "/users/:id")
    pub pattern: String,
    /// Compiled route pattern for efficient matching
    pub compiled_pattern: RoutePattern,
    /// Handler for this route
    pub handler: Arc<dyn RequestHandler>,
    /// Route-specific middleware
    pub middleware: Vec<Arc<dyn Middleware>>,
    /// Route priority
    pub priority: RoutePriority,
    /// Route name for debugging and reverse routing
    pub name: Option<String>,
    /// Route metadata
    pub metadata: HashMap<String, String>,
}

impl Route {
    /// Create a new route
    #[instrument(skip(handler, middleware))]
    pub fn new(
        method: HttpMethod,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
        middleware: Vec<Arc<dyn Middleware>>,
    ) -> Result<(), Error> {
        let compiled_pattern = RoutePattern::compile(pattern)
            .map_err(|e| RouterError::InvalidPattern(pattern.to_string(), e))?;
        
        let priority = Self::calculate_priority(&compiled_pattern);
        
        Ok(Route {
            method,
            pattern: pattern.to_string(),
            compiled_pattern,
            handler,
            middleware,
            priority,
            name: None,
            metadata: HashMap::new(),
        })
    }

    /// Calculate route priority based on pattern complexity
    fn calculate_priority(pattern: &RoutePattern) -> RoutePriority {
        if pattern.has_wildcards {
            if pattern.segments.iter().any(|s| matches!(s, crate::stdlib::web_vibez::route_matcher::PathSegment::Wildcard(name) if name == "**")) {
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
    }

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
    pub prefix: String,
    /// Group-level middleware applied to all routes
    pub middleware: Vec<Arc<dyn Middleware>>,
    /// Routes in this group
    pub routes: Vec<Route>,
    /// Nested subgroups
    pub subgroups: HashMap<String, RouteGroup>,
    /// Group name for debugging
    pub name: Option<String>,
}

impl RouteGroup {
    /// Create a new route group
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            middleware: Vec::new(),
            routes: Vec::new(),
            subgroups: HashMap::new(),
            name: None,
        }
    }

    /// Add middleware to this group
    pub fn with_middleware(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    }

    /// Set group name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Add a route to this group
    pub fn add_route(&mut self, mut route: Route) {
        // Prepend group prefix to route pattern
        if !self.prefix.is_empty() {
            let full_pattern = format!("{}{}", self.prefix.trim_end_matches('/'), &route.pattern);
            route.pattern = full_pattern.clone();
            route.compiled_pattern = RoutePattern::compile(&full_pattern).unwrap();
        }
        
        // Add group middleware to route
        let mut combined_middleware = self.middleware.clone();
        combined_middleware.extend(route.middleware.clone());
        route.middleware = combined_middleware;
        
        self.routes.push(route);
    }

    /// Add a subgroup to this group
    pub fn add_subgroup(&mut self, name: &str, mut subgroup: RouteGroup) {
        // Update subgroup prefix to include parent prefix
        if !self.prefix.is_empty() {
            subgroup.prefix = format!("{}{}", self.prefix.trim_end_matches('/'), &subgroup.prefix);
        }
        
        // Add parent middleware to subgroup
        let mut combined_middleware = self.middleware.clone();
        combined_middleware.extend(subgroup.middleware.clone());
        subgroup.middleware = combined_middleware;
        
        self.subgroups.insert(name.to_string(), subgroup);
    }

    /// Get all routes including from subgroups
    pub fn all_routes(&self) -> Vec<Route> {
        let mut all_routes = self.routes.clone();
        
        for subgroup in self.subgroups.values() {
            all_routes.extend(subgroup.all_routes());
        }
        
        all_routes
    }
}

/// Matched route information
#[derive(Debug)]
pub struct MatchedRoute {
    /// The matched route
    pub route: Route,
    /// Route match details with parameters
    pub route_match: RouteMatch,
    /// Combined middleware chain for this route
    pub middleware_chain: MiddlewareChain,
}

/// High-performance HTTP router
#[derive(Debug)]
pub struct Router {
    /// Route matchers for each HTTP method
    matchers: HashMap<HttpMethod, RouteMatcher>,
    /// All registered routes
    routes: Vec<Route>,
    /// Route groups
    groups: HashMap<String, RouteGroup>,
    /// Global middleware applied to all routes
    global_middleware: Vec<Arc<dyn Middleware>>,
    /// Router configuration
    config: RouterConfig,
    /// Performance statistics
    pub stats: RouterStats,
}

/// Router configuration
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Maximum size for route cache per method
    pub max_cache_size_per_method: usize,
    /// Enable route debugging
    pub debug_mode: bool,
    /// Case sensitive route matching
    pub case_sensitive: bool,
    /// Strict slash matching (/path vs /path/)
    pub strict_slash: bool,
    /// Maximum route priority conflicts to report
    pub max_priority_conflicts: usize,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            max_cache_size_per_method: 1000,
            debug_mode: false,
            case_sensitive: true,
            strict_slash: false,
            max_priority_conflicts: 10,
        }
    }
}

/// Router performance statistics
#[derive(Debug, Default)]
pub struct RouterStats {
    pub total_routes: usize,
    pub total_lookups: u64,
    pub successful_matches: u64,
    pub failed_matches: u64,
    pub average_lookup_time_ns: u64,
    pub cache_hit_rate: f64,
    pub priority_conflicts: u32,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self::with_config(RouterConfig::default())
    }

    /// Create a new router with custom configuration
    pub fn with_config(config: RouterConfig) -> Self {
        Self {
            matchers: HashMap::new(),
            routes: Vec::new(),
            groups: HashMap::new(),
            global_middleware: Vec::new(),
            config,
            stats: RouterStats::default(),
        }
    }

    /// Add global middleware to all routes
    #[instrument(skip(self, middleware))]
    pub fn use_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        debug!("Adding global middleware");
        self.global_middleware.push(middleware);
    }

    /// Register a new route
    #[instrument(skip(self, handler, middleware))]
    pub fn add_route(
        &mut self,
        method: HttpMethod,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
        middleware: Vec<Arc<dyn Middleware>>,
    ) -> Result<(), Error> {
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
    }

    /// Convenience method for GET routes
    pub fn get(
        &mut self,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), Error> {
        self.add_route(HttpMethod::GET, pattern, handler, Vec::from([]))
    }

    /// Convenience method for POST routes
    pub fn post(
        &mut self,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), Error> {
        self.add_route(HttpMethod::POST, pattern, handler, Vec::from([]))
    }

    /// Convenience method for PUT routes
    pub fn put(
        &mut self,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), Error> {
        self.add_route(HttpMethod::PUT, pattern, handler, Vec::from([]))
    }

    /// Convenience method for DELETE routes
    pub fn delete(
        &mut self,
        pattern: &str,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), Error> {
        self.add_route(HttpMethod::DELETE, pattern, handler, Vec::from([]))
    }

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
    }

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
        };
        
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
            method = %method,
            path = %path,
            pattern = %route.pattern,
            elapsed_ns = elapsed_ns,
            "Route matched successfully"
        );
        
        Some(MatchedRoute {
            route,
            route_match,
            middleware_chain,
        })
    }

    /// Handle a request through the router
    #[instrument(skip(self, context))]
    pub async fn handle_request(
        &mut self,
        method: HttpMethod,
        path: &str,
        mut context: RequestContext,
    ) -> HandlerResult {
        match self.find_route(method, path) {
            Some(matched_route) => {
                // Add route parameters to context
                for (key, value) in matched_route.route_match.params() {
                    context.add_param(key, value);
                }
                
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
                response.set_status(StatusCode::NOT_FOUND);
                response.set_body("Not Found".as_bytes().to_vec());
                Ok(response)
            }
        }
    }

    /// Check for route priority conflicts
    fn check_priority_conflicts(&mut self, new_route: &Route) {
        if self.stats.priority_conflicts >= self.config.max_priority_conflicts as u32 {
            return;
        }
        
        for existing_route in &self.routes {
            if existing_route.method == new_route.method 
                && existing_route.priority == new_route.priority
                && patterns_could_conflict(&existing_route.pattern, &new_route.pattern) {
                
                warn!(
                    existing_pattern = %existing_route.pattern,
                    new_pattern = %new_route.pattern,
                    priority = ?new_route.priority,
                    "Potential route priority conflict detected"
                );
                
                self.stats.priority_conflicts += 1;
            }
        }
    }

    /// Get router statistics
    pub fn get_stats(&self) -> &RouterStats {
        &self.stats
    }

    /// Clear all route caches
    pub fn clear_caches(&mut self) {
        for matcher in self.matchers.values_mut() {
            matcher.clear_cache();
        }
        info!("Route caches cleared");
    }

    /// Get all registered routes
    pub fn get_routes(&self) -> &[Route] {
        &self.routes
    }

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
    }
    
    for (seg1, seg2) in segments1.iter().zip(segments2.iter()) {
        // If both are static and different, no conflict
        if !seg1.starts_with(':') && !seg1.starts_with('*') 
            && !seg2.starts_with(':') && !seg2.starts_with('*')
            && seg1 != seg2 {
            return false;
        }
    }
    
    true
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::web_vibez::handlers::StaticHandler;
    use std::sync::Arc;

    #[test]
    fn test_router_basic_routing() {
        let mut router = Router::new();
        let handler = Arc::new(StaticHandler::new("Hello World"));
        
        router.get("/hello", handler.clone()).unwrap();
        router.post("/users", handler.clone()).unwrap();
        
        // Test route registration and finding instead of full request handling
        let matched = router.find_route(HttpMethod::GET, "/hello");
        assert!(matched.is_some());
        
        let matched = router.find_route(HttpMethod::POST, "/users");
        assert!(matched.is_some());
        
        // Test that non-existent routes return None
        let not_found = router.find_route(HttpMethod::GET, "/nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_router_route_parameters() {
        let mut router = Router::new();
        let handler = Arc::new(StaticHandler::new("User Profile"));
        
        router.get("/users/:id", handler).unwrap();
        
        let matched = router.find_route(HttpMethod::GET, "/users/123").unwrap();
        
        assert_eq!(matched.route_match.param("id"), Some("123"));
    }

    #[test]
    fn test_route_priority_calculation() {
        let static_route = RoutePattern::compile("/users/profile").unwrap();
        let param_route = RoutePattern::compile("/users/:id").unwrap();
        let wildcard_route = RoutePattern::compile("/files/*").unwrap();
        
        assert_eq!(Route::calculate_priority(&static_route), RoutePriority::Exact);
        assert_eq!(Route::calculate_priority(&param_route), RoutePriority::Parameterized);
        assert_eq!(Route::calculate_priority(&wildcard_route), RoutePriority::Wildcard);
    }

    #[test]
    fn test_route_group() {
        let mut group = RouteGroup::new("/api/v1");
        let handler = Arc::new(StaticHandler::new("API Response"));
        
        let route = Route::new(HttpMethod::GET, "/users", handler, Vec::from([])).unwrap();
        group.add_route(route);
        
        let routes = group.all_routes();
        assert_eq!(routes.len(), 1);
        assert_eq!(routes[0].pattern, "/api/v1/users");
    }
}
