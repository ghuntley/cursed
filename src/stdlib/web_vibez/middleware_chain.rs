use crate::web::StatusCode;
/// Advanced middleware chain composition and execution system
/// 
/// Provides sophisticated middleware ordering, conditional execution,
/// and performance-optimized chain processing

// use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareResult};
// use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
// use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
use crate::error::CursedError;

use std::sync::Arc;
use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, error, instrument, Span};

/// Middleware execution order configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiddlewareOrdering {
    /// Execute in registration order
    /// Execute by priority (lower numbers first)
    /// Execute by explicit dependency graph
    /// Custom ordering function
/// Middleware execution strategy
#[derive(Debug, Clone)]
pub enum ChainExecution {
    /// Execute all middleware sequentially
    /// Skip remaining middleware on first error
    /// Continue on errors, collect all errors
    /// Execute with timeout per middleware
    /// Conditional execution based on context
/// Middleware dependency specification
#[derive(Debug, Clone)]
pub struct MiddlewareDependency {
    /// Middleware that has dependencies
    /// Middleware that must run before this one
    /// Middleware that must run after this one
/// Performance metrics for middleware execution
#[derive(Debug, Default, Clone)]
pub struct ChainMetrics {
    /// Total chain execution time
    /// Per-middleware execution times
    /// Number of successful executions
    /// Number of failed executions
    /// CursedError counts by middleware
    /// Average execution time
/// Advanced middleware chain builder
pub struct ChainBuilder {
    /// Registered middleware
    /// Middleware ordering strategy
    /// Execution strategy
    /// Dependency specifications
    /// Middleware conditions
    /// Global middleware timeout
    /// Enable performance metrics
    /// Maximum chain depth
impl std::fmt::Debug for ChainBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainBuilder")
            .field("middleware_count", &self.middleware.len())
            .field("ordering", &self.ordering)
            .field("execution", &self.execution)
            .field("dependency_count", &self.dependencies.len())
            .field("condition_count", &self.conditions.len())
            .field("global_timeout", &self.global_timeout)
            .field("enable_metrics", &self.enable_metrics)
            .field("max_chain_depth", &self.max_chain_depth)
            .finish()
    }
}

impl ChainBuilder {
    /// Create a new chain builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add middleware to the chain
    pub fn add(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    /// Add middleware with condition
    pub fn add_conditional<F>(mut self, middleware: Arc<dyn Middleware>, condition: F) -> Self
    where
    {
        let name = middleware.name().to_string();
        self.conditions.insert(name, Box::new(condition));
        self.middleware.push(middleware);
        self
    /// Set middleware ordering strategy
    pub fn with_ordering(mut self, ordering: MiddlewareOrdering) -> Self {
        self.ordering = ordering;
        self
    /// Set execution strategy
    pub fn with_execution(mut self, execution: ChainExecution) -> Self {
        self.execution = execution;
        self
    /// Add middleware dependency
    pub fn add_dependency(mut self, dependency: MiddlewareDependency) -> Self {
        self.dependencies.push(dependency);
        self
    /// Set global timeout for entire chain
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.global_timeout = Some(timeout);
        self
    /// Enable or disable performance metrics
    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    /// Set maximum chain depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_chain_depth = depth;
        self
    /// Build the optimized middleware chain
    #[instrument(skip(self))]
    pub fn build(self) -> MiddlewareChain {
        if self.middleware.len() > self.max_chain_depth {
            warn!(
                "Middleware chain exceeds maximum depth"
            );
        let ordered_middleware = self.order_middleware();
        
        debug!(
            "Built middleware chain"
        );

        MiddlewareChain {
        }
    }

    /// Order middleware according to the selected strategy
    fn order_middleware(&self) -> Vec<Arc<dyn Middleware>> {
        match self.ordering {
            MiddlewareOrdering::Priority => {
                let mut middleware = self.middleware.clone();
                middleware.sort_by_key(|m| m.priority());
                middleware
            }
            MiddlewareOrdering::Custom(compare_fn) => {
                let mut middleware = self.middleware.clone();
                middleware.sort_by(compare_fn);
                middleware
            }
        }
    /// Order middleware by dependency graph using topological sort
    fn order_by_dependencies(&self) -> Vec<Arc<dyn Middleware>> {
        // Create a map of middleware names to middleware
        let middleware_map: HashMap<String, Arc<dyn Middleware>> = self
            .middleware
            .iter()
            .map(|m| (m.name().to_string(), m.clone()))
            .collect();

        // Build dependency graph
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

        for dep in &self.dependencies {
            dependencies.insert(dep.middleware_name.clone(), dep.depends_on.clone());
            for dependency in &dep.depends_on {
                dependents.entry(dependency.clone())
                    .or_insert_with(Vec::new)
                    .push(dep.middleware_name.clone());
            }
        }

        // Topological sort
        let mut sorted = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();

        fn visit(
        ) -> Result<(), String> {
            if temp_visited.contains(name) {
                return Err(format!("Circular dependency detected involving {}", name));
            }
            if visited.contains(name) {
                return Ok(());
            temp_visited.insert(name.to_string());

            if let Some(deps) = dependencies.get(name) {
                for dep in deps {
                    visit(dep, dependencies, visited, temp_visited, sorted)?;
                }
            }

            temp_visited.remove(name);
            visited.insert(name.to_string());
            sorted.push(name.to_string());

            Ok(())
        // Visit all middleware
        for middleware in &self.middleware {
            let name = middleware.name();
            if !visited.contains(name) {
                if let Err(e) = visit(name, &dependencies, &mut visited, &mut temp_visited, &mut sorted) {
                    warn!(error = %e, "Dependency resolution failed, falling back to priority ordering");
                    return self.middleware.clone();
                }
            }
        // Convert sorted names back to middleware
        let mut ordered = Vec::new();
        for name in sorted {
            if let Some(middleware) = middleware_map.get(&name) {
                ordered.push(middleware.clone());
            }
        }

        // Add any middleware not in dependency graph
        for middleware in &self.middleware {
            if !ordered.iter().any(|m| m.name() == middleware.name()) {
                ordered.push(middleware.clone());
            }
        }

        ordered
    }
}

impl Default for ChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimized middleware chain with advanced execution capabilities
pub struct MiddlewareChain {
    /// Ordered middleware
    /// Execution strategy
    /// Conditional execution rules
    /// Global timeout
    /// Enable metrics collection
    /// Performance metrics
impl std::fmt::Debug for MiddlewareChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddlewareChain")
            .field("middleware_count", &self.middleware.len())
            .field("execution", &self.execution)
            .field("condition_count", &self.conditions.len())
            .field("global_timeout", &self.global_timeout)
            .field("enable_metrics", &self.enable_metrics)
            .finish()
    }
}

impl MiddlewareChain {
    /// Create a simple middleware chain
    pub fn new(middleware: Vec<Arc<dyn Middleware>>) -> Self {
        let mut builder = ChainBuilder::new()
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast);
        
        for m in middleware {
            builder = builder.add(m);
        builder.build()
    /// Execute the middleware chain
    #[instrument(skip(self, context, response, handler))]
    pub async fn execute(
    ) -> HandlerResult {
        let chain_start = Instant::now();
        let execution_span = Span::current();
        
        // Apply global timeout if configured
        let execution_future = async {
            self.execute_internal(&mut context, &mut response, handler).await

        let result = if let Some(timeout) = self.global_timeout {
            match tokio::time::timeout(timeout, execution_future).await {
                Err(_) => {
                    error!(timeout_ms = timeout.as_millis(), "Middleware chain timed out");
//                     response.set_status(crate::stdlib::web_vibez::StatusCode(408)); // Request Timeout
                    response.set_text("Request timeout");
                    Ok(response)
                }
            }
        } else {
            execution_future.await

        // Update metrics if enabled
        if self.enable_metrics {
            let total_time = chain_start.elapsed();
            if let Ok(mut metrics) = self.metrics.lock() {
                metrics.total_execution_time = total_time;
                if result.is_ok() {
                    metrics.successful_executions += 1;
                } else {
                    metrics.failed_executions += 1;
                let total_executions = metrics.successful_executions + metrics.failed_executions;
                if total_executions > 0 {
                    metrics.average_execution_time = Duration::from_nanos(
                        (metrics.total_execution_time.as_nanos() as u64) / total_executions
                    );
                }
            }
        result
    /// Internal execution logic
    async fn execute_internal(
    ) -> HandlerResult {
        let mut errors = Vec::new();

        // Execute before_request middleware
        for middleware in &self.middleware {
            // Check conditions
            if let Some(condition) = self.conditions.get(middleware.name()) {
                if !condition(context) {
                    debug!(middleware = middleware.name(), "Skipping middleware due to condition");
                    continue;
                }
            }

            let middleware_start = Instant::now();
            let result = match &self.execution {
                ChainExecution::WithTimeout(timeout) => {
                    // For sync middleware, we can't apply timeout in the same way
                    // This is a limitation - async timeout would require async trait
                    middleware.before_request(context, response)
                }

            // Update per-middleware metrics
            if self.enable_metrics {
                let execution_time = middleware_start.elapsed();
                if let Ok(mut metrics) = self.metrics.lock() {
                    metrics.middleware_times.insert(middleware.name().to_string(), execution_time);
                }
            }

            match result {
                Ok(_) => {
                    debug!(middleware = middleware.name(), "Middleware before_request completed");
                }
                Err(e) => {
                    error!(middleware = middleware.name(), error = %e, "Middleware before_request failed");
                    
                    // Update error metrics
                    if self.enable_metrics {
                        if let Ok(mut metrics) = self.metrics.lock() {
                            *metrics.error_counts.entry(middleware.name().to_string()).or_insert(0) += 1;
                        }
                    }

                    match &self.execution {
                        ChainExecution::FailFast => {
                            // Try to handle the error
                            if let Err(handle_error) = middleware.on_error(context, response, &e) {
                                error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                            }
                            return Ok(response.clone());
                        }
                        ChainExecution::ContinueOnError => {
                            // Try to handle the error but continue
                            if let Err(handle_error) = middleware.on_error(context, response, &e) {
                                error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                            }
                            errors.push((middleware.name().to_string(), e));
                        }
                        _ => {
                            // Try to handle the error
                            if let Err(handle_error) = middleware.on_error(context, response, &e) {
                                error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                            }
                            return Ok(response.clone());
                        }
                    }
                }
            }

            // If response is marked as sent, skip remaining middleware and handler
            if response.is_sent() {
                debug!(middleware = middleware.name(), "Response sent by middleware, skipping remaining chain");
                return Ok(response.clone());
            }
        }

        // Execute handler if no middleware sent response
        if !response.is_sent() {
            debug!("Executing request handler");
            if let Err(e) = handler.handle(context, response).await {
                error!(error = %e, "Handler execution failed");
//                 response.set_status(crate::stdlib::web_vibez::StatusCode::InternalServerError);
                response.set_text(&format!("Handler error: {}", e));
            }
        }

        // Execute after_response middleware in reverse order
        for middleware in self.middleware.iter().rev() {
            // Check conditions
            if let Some(condition) = self.conditions.get(middleware.name()) {
                if !condition(context) {
                    continue;
                }
            }

            let middleware_start = Instant::now();
            let result = match &self.execution {
                ChainExecution::WithTimeout(timeout) => {
                    // For sync middleware, we can't apply timeout in the same way
                    // This is a limitation - async timeout would require async trait
                    middleware.after_response(context, response)
                }

            // Update metrics
            if self.enable_metrics {
                let execution_time = middleware_start.elapsed();
                if let Ok(mut metrics) = self.metrics.lock() {
                    let total_time = *metrics.middleware_times.get(middleware.name()).unwrap_or(&Duration::ZERO);
                    metrics.middleware_times.insert(middleware.name().to_string(), total_time + execution_time);
                }
            }

            if let Err(e) = result {
                error!(middleware = middleware.name(), error = %e, "Middleware after_response failed");
                
                // Update error metrics
                if self.enable_metrics {
                    if let Ok(mut metrics) = self.metrics.lock() {
                        *metrics.error_counts.entry(middleware.name().to_string()).or_insert(0) += 1;
                    }
                }

                match &self.execution {
                    ChainExecution::ContinueOnError => {
                        // Try to handle the error but continue
                        if let Err(handle_error) = middleware.on_error(context, response, &e) {
                            error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                        }
                        errors.push((middleware.name().to_string(), e));
                    }
                    _ => {
                        // Try to handle the error
                        if let Err(handle_error) = middleware.on_error(context, response, &e) {
                            error!(middleware = middleware.name(), error = %handle_error, "CursedError in middleware error handler");
                        }
                    }
                }
            } else {
                debug!(middleware = middleware.name(), "Middleware after_response completed");
            }
        }

        // Log accumulated errors if using ContinueOnError strategy
        if !errors.is_empty() && matches!(self.execution, ChainExecution::ContinueOnError) {
            warn!(error_count = errors.len(), "Multiple middleware errors occurred");
            for (middleware_name, error) in errors {
                warn!(middleware = %middleware_name, error = %error, "Middleware error");
            }
        }

        Ok(response.clone())
    /// Get performance metrics
    pub fn get_metrics(&self) -> Option<ChainMetrics> {
        if self.enable_metrics {
            self.metrics.lock().ok().map(|m| m.clone())
        } else {
            None
        }
    }

    /// Reset performance metrics
    pub fn reset_metrics(&self) {
        if let Ok(mut metrics) = self.metrics.lock() {
            *metrics = ChainMetrics::default();
        }
    }

    /// Get middleware count
    pub fn middleware_count(&self) -> usize {
        self.middleware.len()
    /// Get middleware names in execution order
    pub fn middleware_names(&self) -> Vec<String> {
        self.middleware.iter().map(|m| m.name().to_string()).collect()
    }
}

/// Helper functions for common middleware chain patterns
pub struct ChainPatterns;

impl ChainPatterns {
    /// Create a basic web API chain
    pub fn web_api() -> ChainBuilder {
//         use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, CorsMiddleware};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(CorsMiddleware::new()))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast)
    /// Create a secure API chain with authentication
    pub fn secure_api() -> ChainBuilder {
//         use crate::stdlib::web_vibez::middleware::{
            LoggingMiddleware, CorsMiddleware, AuthMiddleware, RateLimitMiddleware, AuthScheme

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(RateLimitMiddleware::new(1000))) // 1000 req/min
            .add(Arc::new(AuthMiddleware::new(Vec::from([AuthScheme::Bearer]))))
            .add(Arc::new(CorsMiddleware::new()))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast)
    /// Create a static file serving chain
    pub fn static_files(root_dir: std::path::PathBuf) -> ChainBuilder {
//         use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, StaticFileMiddleware};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new().with_skip_paths(Vec::from(["/health".to_string()]))))
            .add(Arc::new(StaticFileMiddleware::new(root_dir, "/static")))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::Sequential)
    /// Create a development chain with verbose logging
    pub fn development() -> ChainBuilder {
//         use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, CorsMiddleware, LogLevel};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new().with_body_logging(true, true)))
            .add(Arc::new(CorsMiddleware::new().with_origins(Vec::from(["*".to_string()]))))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::ContinueOnError)
            .with_metrics(true)
    }
}

