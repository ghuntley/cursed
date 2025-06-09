/// fr fr HTTP routing for web_vibez - path-based request routing
use std::collections::HashMap;
use std::sync::Arc;
use regex::Regex;

use crate::stdlib::packages::web_vibez::{
    request::HttpRequest,
    response::HttpResponse,
    handler::Handler,
    method::{HttpMethod, MethodSet},
    error::{WebError, WebResult},
};

/// fr fr Path parameters extracted from route patterns - dynamic segments
pub type PathParams = HashMap<String, String>;

/// fr fr Route entry containing pattern and handler - routing table entry
#[derive(Clone)]
pub struct RouteEntry {
    /// fr fr Route pattern (e.g., "/users/:id")
    pub pattern: String,
    /// fr fr Compiled regex for matching
    pub regex: Regex,
    /// fr fr Parameter names in order
    pub param_names: Vec<String>,
    /// fr fr Allowed HTTP methods
    pub methods: MethodSet,
    /// fr fr Handler for this route
    pub handler: Arc<dyn Handler>,
    /// fr fr Route name for debugging
    pub name: String,
}

impl RouteEntry {
    /// fr fr Create new route entry - single route definition
    pub fn new(
        pattern: String,
        methods: MethodSet,
        handler: Arc<dyn Handler>,
        name: String,
    ) -> WebResult<Self> {
        let (regex, param_names) = Self::compile_pattern(&pattern)?;
        
        Ok(Self {
            pattern,
            regex,
            param_names,
            methods,
            handler,
            name,
        })
    }

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
                            setting: "route_pattern".to_string(),
                            value: pattern.to_string(),
                            message: "Empty parameter name after ':'".to_string(),
                        });
                    }
                    
                    param_names.push(param_name);
                    regex_pattern.push_str("([^/]+)"); // Match non-slash characters
                }
                '*' => {
                    // Wildcard parameter - matches everything
                    if chars.peek().is_some() {
                        return Err(WebError::Configuration {
                            setting: "route_pattern".to_string(),
                            value: pattern.to_string(),
                            message: "Wildcard '*' must be at the end of pattern".to_string(),
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
        }

        // Anchor the regex to match the full path
        regex_pattern = format!("^{}$", regex_pattern);

        let regex = Regex::new(&regex_pattern).map_err(|e| {
            WebError::Configuration {
                setting: "route_pattern".to_string(),
                value: pattern.to_string(),
                message: format!("Invalid regex pattern: {}", e),
            }
        })?;

        Ok((regex, param_names))
    }

    /// fr fr Match request path and extract parameters - route matching
    pub fn matches(&self, path: &str, method: HttpMethod) -> Option<PathParams> {
        // Check if method is allowed
        if !self.methods.contains(method) {
            return None;
        }

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
}

/// fr fr HTTP router for handling requests - main routing engine
pub struct Router {
    /// fr fr Route entries in order of registration
    routes: Vec<RouteEntry>,
    /// fr fr Default handler for unmatched routes
    not_found_handler: Option<Arc<dyn Handler>>,
    /// fr fr Default handler for method not allowed
    method_not_allowed_handler: Option<Arc<dyn Handler>>,
    /// fr fr Route groups for organization
    groups: HashMap<String, Router>,
}

impl Router {
    /// fr fr Create new router - empty routing table
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            not_found_handler: None,
            method_not_allowed_handler: None,
            groups: HashMap::new(),
        }
    }

    /// fr fr Add route with specific methods - flexible method handling
    pub fn route<H: Handler + 'static>(
        mut self,
        pattern: impl Into<String>,
        methods: MethodSet,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        let route = RouteEntry::new(
            pattern.into(),
            methods,
            Arc::new(handler),
            name.into(),
        )?;
        self.routes.push(route);
        Ok(self)
    }

    /// fr fr Add GET route - common shortcut
    pub fn get<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(Vec::from([HttpMethod::Get])),
            handler,
            name,
        )
    }

    /// fr fr Add POST route - common shortcut
    pub fn post<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(Vec::from([HttpMethod::Post])),
            handler,
            name,
        )
    }

    /// fr fr Add PUT route - common shortcut
    pub fn put<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(Vec::from([HttpMethod::Put])),
            handler,
            name,
        )
    }

    /// fr fr Add DELETE route - common shortcut
    pub fn delete<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(Vec::from([HttpMethod::Delete])),
            handler,
            name,
        )
    }

    /// fr fr Add PATCH route - common shortcut
    pub fn patch<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(Vec::from([HttpMethod::Patch])),
            handler,
            name,
        )
    }

    /// fr fr Add route that handles all methods - catch-all
    pub fn any<H: Handler + 'static>(
        self,
        pattern: impl Into<String>,
        handler: H,
        name: impl Into<String>,
    ) -> WebResult<Self> {
        self.route(
            pattern,
            MethodSet::from_methods(HttpMethod::all()),
            handler,
            name,
        )
    }

    /// fr fr Set custom 404 handler - not found responses
    pub fn not_found<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.not_found_handler = Some(Arc::new(handler));
        self
    }

    /// fr fr Set custom 405 handler - method not allowed responses
    pub fn method_not_allowed<H: Handler + 'static>(mut self, handler: H) -> Self {
        self.method_not_allowed_handler = Some(Arc::new(handler));
        self
    }

    /// fr fr Add route group with prefix - organize related routes
    pub fn group(mut self, prefix: impl Into<String>, group_router: Router) -> Self {
        self.groups.insert(prefix.into(), group_router);
        self
    }

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
        }

        None
    }

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
    }

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
    }

    /// fr fr Route request and return response - main routing logic
    pub async fn route_request(&self, mut request: HttpRequest) -> WebResult<HttpResponse> {
        // Find matching route
        if let Some((route, params)) = self.find_route(&request.path, request.method) {
            // Add path parameters to request extensions
            if !params.is_empty() {
                request.set_extension(
                    "path_params".to_string(),
                    serde_json::to_value(params).unwrap(),
                );
            }

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
    }

    /// fr fr Get all route patterns - debugging/introspection
    pub fn route_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for route in &self.routes {
            patterns.push(route.pattern.clone());
        }
        
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
    }

    /// fr fr Get all path parameters - complete extraction
    pub fn path_params(&self) -> PathParams {
        self.extension("path_params")
            .and_then(|params| serde_json::from_value(params.clone()).ok())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::web_vibez::handler::FunctionHandler;

    #[test]
    fn test_route_pattern_compilation() {
        let (regex, params) = RouteEntry::compile_pattern("/users/:id/posts/:post_id").unwrap();
        assert_eq!(params, Vec::from(["id", "post_id"]));
        
        let captures = regex.captures("/users/123/posts/456").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "123");
        assert_eq!(captures.get(2).unwrap().as_str(), "456");
    }

    #[test]
    fn test_route_matching() {
        let handler = FunctionHandler::sync("test".to_string(), |_| {
            Ok(HttpResponse::ok())
        });
        
        let route = RouteEntry::new(
            "/users/:id".to_string(),
            MethodSet::from_methods(Vec::from([HttpMethod::Get])),
            Arc::new(handler),
            "user_detail".to_string(),
        ).unwrap();

        let params = route.matches("/users/123", HttpMethod::Get).unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));
        
        assert!(route.matches("/users/123", HttpMethod::Post).is_none());
        assert!(route.matches("/posts/123", HttpMethod::Get).is_none());
    }

    #[tokio::test]
    async fn test_router_basic() {
        let handler = FunctionHandler::sync("test".to_string(), |_| {
            Ok(HttpResponse::ok().with_text("Hello"))
        });

        let router = Router::new()
            .get("/hello", handler, "hello")
            .unwrap();

        let request = HttpRequest::new(HttpMethod::Get, "/hello".to_string());
        let response = router.route_request(request).await.unwrap();
        
        assert_eq!(response.body_text().unwrap(), "Hello");
    }

    #[tokio::test]
    async fn test_router_path_params() {
        let handler = FunctionHandler::sync("user_detail".to_string(), |req| {
            let user_id = req.path_param("id").unwrap_or("unknown".to_string());
            Ok(HttpResponse::ok().with_text(format!("User: {}", user_id)))
        });

        let router = Router::new()
            .get("/users/:id", handler, "user_detail")
            .unwrap();

        let request = HttpRequest::new(HttpMethod::Get, "/users/123".to_string());
        let response = router.route_request(request).await.unwrap();
        
        assert_eq!(response.body_text().unwrap(), "User: 123");
    }

    #[tokio::test]
    async fn test_router_method_not_allowed() {
        let handler = FunctionHandler::sync("test".to_string(), |_| {
            Ok(HttpResponse::ok())
        });

        let router = Router::new()
            .get("/test", handler, "test")
            .unwrap();

        let request = HttpRequest::new(HttpMethod::Post, "/test".to_string());
        let response = router.route_request(request).await.unwrap();
        
        assert_eq!(response.status, crate::stdlib::packages::web_vibez::status::StatusCode::MethodNotAllowed);
        assert_eq!(response.header("allow"), Some(&"GET".to_string()));
    }

    #[test]
    fn test_wildcard_pattern() {
        let (regex, params) = RouteEntry::compile_pattern("/static/*").unwrap();
        assert_eq!(params, Vec::from(["*"]));
        
        let captures = regex.captures("/static/css/main.css").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "css/main.css");
    }
}
