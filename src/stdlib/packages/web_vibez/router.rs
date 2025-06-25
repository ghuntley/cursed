use crate::web::StatusCode;
use crate::error::CursedError;
/// fr fr HTTP routing for web_vibez - path-based request routing
use std::collections::HashMap;
use std::sync::Arc;
use regex::Regex;

// Placeholder imports disabled
// };

/// fr fr Path parameters extracted from route patterns - dynamic segments
pub type PathParams = HashMap<String, String>;

/// fr fr Route entry containing pattern and handler - routing table entry
#[derive(Clone)]
pub struct RouteEntry {
    /// fr fr Route pattern (e.g., "/users/:id")
    /// fr fr Compiled regex for matching
    /// fr fr Parameter names in order
    /// fr fr Allowed HTTP methods
    /// fr fr Handler for this route
    /// fr fr Route name for debugging
impl RouteEntry {
    /// fr fr Create new route entry - single route definition
    pub fn new(
    ) -> WebResult<Self> {
        let (regex, param_names) = Self::compile_pattern(&pattern)?;
        
        Ok(Self {
        })
    /// fr fr Compile route pattern to regex - pattern matching setup
    fn compile_pattern(pattern: &str) -> WebResult<(Regex, Vec<String>)> {
        let mut regex_pattern = String::new();
        let mut param_names = Vec::new();
        let mut chars = pattern.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                ':' => {
                    // Named parameter like :id or :name
                    let mut param_name = String::new();
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            param_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    
                    if param_name.is_empty() {
                        return Err(WebError::Configuration {
                        });
                    param_names.push(param_name);
                    regex_pattern.push_str("([^/]+)"); // Match non-slash characters
                }
                '*' => {
                    // Wildcard parameter - matches everything
                    if chars.peek().is_some() {
                        return Err(WebError::Configuration {
                        });
                    }
                    param_names.push("*".to_string());
                    regex_pattern.push_str("(.*)"); // Match everything
                }
                '.' | '+' | '?' | '^' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
                    // Escape regex special characters
                    regex_pattern.push('\\');
                    regex_pattern.push(ch);
                }
                _ => {
                    regex_pattern.push(ch);
                }
            }
        // Anchor the regex to match the full path
        regex_pattern = format!("^{}$", regex_pattern);

        let regex = Regex::new(&regex_pattern).map_err(|e| {
            WebError::Configuration {
            }
        })?;

        Ok((regex, param_names))
    /// fr fr Match request path and extract parameters - route matching
    pub fn matches(&self, path: &str, method: HttpMethod) -> Option<PathParams> {
        // Check if method is allowed
        if !self.methods.contains(method) {
            return None;
        // Check if path matches pattern
        if let Some(captures) = self.regex.captures(path) {
            let mut params = PathParams::new();
            
            // Extract parameter values
            for (i, param_name) in self.param_names.iter().enumerate() {
                if let Some(capture) = captures.get(i + 1) {
                    params.insert(param_name.clone(), capture.as_str().to_string());
                }
            }
            
            Some(params)
        } else {
            None
        }
    }
/// fr fr HTTP router for handling requests - main routing engine
pub struct Router {
    /// fr fr Route entries in order of registration
    /// fr fr Default handler for unmatched routes
    /// fr fr Default handler for method not allowed
    /// fr fr Route groups for organization
impl Router {
    /// fr fr Create new router - empty routing table
    pub fn new() -> Self {
        Self {
        }
    }

    /// fr fr Add route with specific methods - flexible method handling
    pub fn route<H: Handler + 'static>(
    ) -> WebResult<Self> {
        let route = RouteEntry::new(
        )?;
        self.routes.push(route);
        Ok(self)
    /// fr fr Add GET route - common shortcut
    pub fn get<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Add POST route - common shortcut
    pub fn post<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Add PUT route - common shortcut
    pub fn put<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Add DELETE route - common shortcut
    pub fn delete<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Add PATCH route - common shortcut
    pub fn patch<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Add route that handles all methods - catch-all
    pub fn any<H: Handler + 'static>(
    ) -> WebResult<Self> {
        self.route(
        )
    /// fr fr Set custom 404 handler - not found responses
    pub fn not_found<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.not_found_handler = Some(Arc::new(handler));
        self
    /// fr fr Set custom 405 handler - method not allowed responses
    pub fn method_not_allowed<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.method_not_allowed_handler = Some(Arc::new(handler));
        self
    /// fr fr Add route group with prefix - organize related routes
    pub fn group(mut self, prefix: impl Into<String>, group_router: Router) -> Self {
        self.groups.insert(prefix.into(), group_router);
        self
    /// fr fr Find matching route for request - route resolution
    pub fn find_route(&self, path: &str, method: HttpMethod) -> Option<(&RouteEntry, PathParams)> {
        // First check direct routes
        for route in &self.routes {
            if let Some(params) = route.matches(path, method) {
                return Some((route, params));
            }
        }

        // Then check route groups
        for (prefix, group) in &self.groups {
            if path.starts_with(prefix) {
                let group_path = &path[prefix.len()..];
                if let Some((route, params)) = group.find_route(group_path, method) {
                    return Some((route, params));
                }
            }
        None
    /// fr fr Get all routes for a path - method checking
    pub fn routes_for_path(&self, path: &str) -> Vec<&RouteEntry> {
        let mut matching_routes = Vec::new();

        // Check direct routes
        for route in &self.routes {
            if route.regex.is_match(path) {
                matching_routes.push(route);
            }
        }

        // Check route groups
        for (prefix, group) in &self.groups {
            if path.starts_with(prefix) {
                let group_path = &path[prefix.len()..];
                matching_routes.extend(group.routes_for_path(group_path));
            }
        }

        matching_routes
    /// fr fr Get allowed methods for a path - OPTIONS support
    pub fn allowed_methods(&self, path: &str) -> MethodSet {
        let routes = self.routes_for_path(path);
        let mut allowed = MethodSet::new();

        for route in routes {
            for method in route.methods.methods() {
                allowed.add(*method);
            }
        }

        allowed
    /// fr fr Route request and return response - main routing logic
    pub async fn route_request(&self, mut request: HttpRequest) -> WebResult<HttpResponse> {
        // Find matching route
        if let Some((route, params)) = self.find_route(&request.path, request.method) {
            // Add path parameters to request extensions
            if !params.is_empty() {
                request.set_extension(
                );
            // Handle the request
            route.handler.handle(request).await
        } else {
            // Check if path exists but method is not allowed
            let allowed_methods = self.allowed_methods(&request.path);
            if !allowed_methods.is_empty() {
                if let Some(handler) = &self.method_not_allowed_handler {
                    let mut response = handler.handle(request).await?;
                    response = response.with_header("allow", allowed_methods.to_allow_header());
                    Ok(response)
                } else {
                    Ok(HttpResponse::method_not_allowed("Method not allowed")
                        .with_header("allow", allowed_methods.to_allow_header()))
                }
            } else {
                // Path not found
                if let Some(handler) = &self.not_found_handler {
                    handler.handle(request).await
                } else {
                    Ok(HttpResponse::not_found().with_text("Route not found"))
                }
            }
        }
    }

    /// fr fr Get route count - statistics
    pub fn route_count(&self) -> usize {
        let mut count = self.routes.len();
        for group in self.groups.values() {
            count += group.route_count();
        }
        count
    /// fr fr Get all route patterns - debugging/introspection
    pub fn route_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for route in &self.routes {
            patterns.push(route.pattern.clone());
        for (prefix, group) in &self.groups {
            for pattern in group.route_patterns() {
                patterns.push(format!("{}{}", prefix, pattern));
            }
        }
        
        patterns
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Helper functions for path parameter extraction
impl HttpRequest {
    /// fr fr Get path parameter by name - extract from route
    pub fn path_param(&self, name: &str) -> Option<String> {
        self.extension("path_params")
            .and_then(|params| params.get(name))
            .and_then(|value| value.as_str())
            .map(|s| s.to_string())
    /// fr fr Get all path parameters - complete extraction
    pub fn path_params(&self) -> PathParams {
        self.extension("path_params")
            .and_then(|params| serde_json::from_value(params.clone()).ok())
            .unwrap_or_default()
    }
}

