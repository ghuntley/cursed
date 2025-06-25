use crate::web::StatusCode;
// HTTP router implementation for GlowUpHTTP

// use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
// use crate::stdlib::glowup_http::handler::{Handler, HandlerFunc};
// use crate::stdlib::glowup_http::request::{VibeRequest, Method};
// use crate::stdlib::glowup_http::response::ResponderVibe;
// use crate::stdlib::glowup_http::middleware::MiddlewareFunc;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, instrument};
use crate::error::CursedError;

/// HTTP router and dispatcher
/// This follows the CURSED spec's `VibeRouter` naming
#[derive(Debug)]
pub struct VibeRouter {
impl VibeRouter {
    /// Create a new router
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add a route with a handler function
    #[instrument(skip(self, handler))]
    pub fn handle_func(&mut self, pattern: &str, handler: HandlerFunc) {
        // For now, we'll treat all patterns as exact matches for all methods
        for method in [Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::HEAD, Method::OPTIONS] {
            self.routes.insert((method.clone(), pattern.to_string()), handler.clone());
        }
        debug!("Added route: {}", pattern);
    /// Add a route with a handler
    #[instrument(skip(self, handler))]
    pub fn handle(&mut self, pattern: &str, handler: Arc<dyn Handler>) {
        for method in [Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::HEAD, Method::OPTIONS] {
            self.routes.insert((method.clone(), pattern.to_string()), handler.clone());
        }
        debug!("Added route: {}", pattern);
    /// Add GET route
    #[instrument(skip(self, handler))]
    pub fn get(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::GET, pattern.to_string()), handler);
        debug!("Added GET route: {}", pattern);
    /// Add POST route
    #[instrument(skip(self, handler))]
    pub fn post(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::POST, pattern.to_string()), handler);
        debug!("Added POST route: {}", pattern);
    /// Add PUT route
    #[instrument(skip(self, handler))]
    pub fn put(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::PUT, pattern.to_string()), handler);
        debug!("Added PUT route: {}", pattern);
    /// Add DELETE route
    #[instrument(skip(self, handler))]
    pub fn delete(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::DELETE, pattern.to_string()), handler);
        debug!("Added DELETE route: {}", pattern);
    /// Add PATCH route
    #[instrument(skip(self, handler))]
    pub fn patch(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::PATCH, pattern.to_string()), handler);
        debug!("Added PATCH route: {}", pattern);
    /// Add OPTIONS route
    #[instrument(skip(self, handler))]
    pub fn options(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::OPTIONS, pattern.to_string()), handler);
        debug!("Added OPTIONS route: {}", pattern);
    /// Add HEAD route
    #[instrument(skip(self, handler))]
    pub fn head(&mut self, pattern: &str, handler: HandlerFunc) {
        self.routes.insert((Method::HEAD, pattern.to_string()), handler);
        debug!("Added HEAD route: {}", pattern);
    /// Add middleware
    #[instrument(skip(self, middleware))]
    pub fn use_middleware(&mut self, middleware: MiddlewareFunc) {
        self.middleware.push(middleware);
        debug!("Added middleware");
    /// Find a route for the given request
    #[instrument(skip(self, request))]
    fn find_route(&self, request: &VibeRequest) -> Option<Arc<dyn Handler>> {
        // Simple exact match for now
        let key = (request.method.clone(), request.url.clone());
        self.routes.get(&key).cloned()
    /// Extract path parameters (placeholder for pattern matching)
    fn extract_path_params(&self, _pattern: &str, _path: &str) -> HashMap<String, String> {
        // Placeholder - would implement pattern matching like "/users/:id"
        HashMap::new()
    }
}

impl Handler for VibeRouter {
    #[instrument(skip(self, w, r))]
    fn handle_vibe(&self, w: &dyn ResponderVibe, r: &VibeRequest) -> GlowUpResult<()> {
        debug!("Routing request: {} {}", r.method, r.url);
        
        // Find matching route
        if let Some(handler) = self.find_route(r) {
            // Apply middleware chain
            let mut final_handler = handler;
            
            // For now, skip middleware application and call handler directly
            final_handler.handle_vibe(w, r)
        } else {
            // No route found
            use crate::web::StatusCode;
            w.write_header(StatusCode::NOT_FOUND);
            w.write(b"Not Found")?;
            Ok(())
        }
    }
impl Default for VibeRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new router (convenience function)
pub fn new_vibe_router() -> VibeRouter {
    VibeRouter::new()
// Convenience re-export for the spec function
pub use new_vibe_router as NewVibeRouter;

