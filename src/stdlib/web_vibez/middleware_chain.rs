use crate::web::StatusCode;
/// Advanced middleware chain composition and execution system
/// 
/// Provides sophisticated middleware ordering, conditional execution,
/// and performance-optimized chain processing

use crate::stdlib::web_vibez::middleware::{Middleware, MiddlewareResult};
use crate::stdlib::web_vibez::context::{RequestContext, ResponseContext};
use crate::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
use crate::stdlib::web_vibez::error_handling::MiddlewareError;
use crate::error::Error;

use std::sync::Arc;
use std::collections::{HashMap, BTreeMap};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn, error, instrument, Span};

/// Middleware execution order configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiddlewareOrdering {
    /// Execute in registration order
    Registration,
    /// Execute by priority (lower numbers first)
    Priority,
    /// Execute by explicit dependency graph
    Dependency,
    /// Custom ordering function
    Custom(fn(&Arc<dyn Middleware>, &Arc<dyn Middleware>) -> std::cmp::Ordering),
}

/// Middleware execution strategy
#[derive(Debug, Clone)]
pub enum ChainExecution {
    /// Execute all middleware sequentially
    Sequential,
    /// Skip remaining middleware on first error
    FailFast,
    /// Continue on errors, collect all errors
    ContinueOnError,
    /// Execute with timeout per middleware
    WithTimeout(Duration),
    /// Conditional execution based on context
    Conditional,
}

/// Middleware dependency specification
#[derive(Debug, Clone)]
pub struct MiddlewareDependency {
    /// Middleware that has dependencies
    pub middleware_name: String,
    /// Middleware that must run before this one
    pub depends_on: Vec<String>,
    /// Middleware that must run after this one
    pub runs_before: Vec<String>,
}

/// Performance metrics for middleware execution
#[derive(Debug, Default, Clone)]
pub struct ChainMetrics {
    /// Total chain execution time
    pub total_execution_time: Duration,
    /// Per-middleware execution times
    pub middleware_times: HashMap<String, Duration>,
    /// Number of successful executions
    pub successful_executions: u64,
    /// Number of failed executions
    pub failed_executions: u64,
    /// Error counts by middleware
    pub error_counts: HashMap<String, u32>,
    /// Average execution time
    pub average_execution_time: Duration,
}

/// Advanced middleware chain builder
pub struct ChainBuilder {
    /// Registered middleware
    middleware: Vec<Arc<dyn Middleware>>,
    /// Middleware ordering strategy
    ordering: MiddlewareOrdering,
    /// Execution strategy
    execution: ChainExecution,
    /// Dependency specifications
    dependencies: Vec<MiddlewareDependency>,
    /// Middleware conditions
    conditions: HashMap<String, Box<dyn Fn(&RequestContext) -> bool + Send + Sync>>,
    /// Global middleware timeout
    global_timeout: Option<Duration>,
    /// Enable performance metrics
    enable_metrics: bool,
    /// Maximum chain depth
    max_chain_depth: usize,
}

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
            middleware: Vec::new(),
            ordering: MiddlewareOrdering::Priority,
            execution: ChainExecution::FailFast,
            dependencies: Vec::new(),
            conditions: HashMap::new(),
            global_timeout: None,
            enable_metrics: true,
            max_chain_depth: 50,
        }
    }

    /// Add middleware to the chain
    pub fn add(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware.push(middleware);
        self
    }

    /// Add middleware with condition
    pub fn add_conditional<F>(mut self, middleware: Arc<dyn Middleware>, condition: F) -> Self
    where
        F: Fn(&RequestContext) -> bool + Send + Sync + 'static,
    {
        let name = middleware.name().to_string();
        self.conditions.insert(name, Box::new(condition));
        self.middleware.push(middleware);
        self
    }

    /// Set middleware ordering strategy
    pub fn with_ordering(mut self, ordering: MiddlewareOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    /// Set execution strategy
    pub fn with_execution(mut self, execution: ChainExecution) -> Self {
        self.execution = execution;
        self
    }

    /// Add middleware dependency
    pub fn add_dependency(mut self, dependency: MiddlewareDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Set global timeout for entire chain
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.global_timeout = Some(timeout);
        self
    }

    /// Enable or disable performance metrics
    pub fn with_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    /// Set maximum chain depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_chain_depth = depth;
        self
    }

    /// Build the optimized middleware chain
    #[instrument(skip(self))]
    pub fn build(self) -> MiddlewareChain {
        if self.middleware.len() > self.max_chain_depth {
            warn!(
                middleware_count = self.middleware.len(),
                max_depth = self.max_chain_depth,
                "Middleware chain exceeds maximum depth"
            );
        }

        let ordered_middleware = self.order_middleware();
        
        debug!(
            middleware_count = ordered_middleware.len(),
            ordering = ?self.ordering,
            execution = ?self.execution,
            "Built middleware chain"
        );

        MiddlewareChain {
            middleware: ordered_middleware,
            execution: self.execution,
            conditions: self.conditions,
            global_timeout: self.global_timeout,
            enable_metrics: self.enable_metrics,
            metrics: Arc::new(std::sync::Mutex::new(ChainMetrics::default())),
        }
    }

    /// Order middleware according to the selected strategy
    fn order_middleware(&self) -> Vec<Arc<dyn Middleware>> {
        match self.ordering {
            MiddlewareOrdering::Registration => self.middleware.clone(),
            MiddlewareOrdering::Priority => {
                let mut middleware = self.middleware.clone();
                middleware.sort_by_key(|m| m.priority());
                middleware
            }
            MiddlewareOrdering::Dependency => self.order_by_dependencies(),
            MiddlewareOrdering::Custom(compare_fn) => {
                let mut middleware = self.middleware.clone();
                middleware.sort_by(compare_fn);
                middleware
            }
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
            name: &str,
            dependencies: &HashMap<String, Vec<String>>,
            visited: &mut std::collections::HashSet<String>,
            temp_visited: &mut std::collections::HashSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), String> {
            if temp_visited.contains(name) {
                return Err(format!("Circular dependency detected involving {}", name));
            }
            if visited.contains(name) {
                return Ok(());
            }

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
        }

        // Visit all middleware
        for middleware in &self.middleware {
            let name = middleware.name();
            if !visited.contains(name) {
                if let Err(e) = visit(name, &dependencies, &mut visited, &mut temp_visited, &mut sorted) {
                    warn!(error = %e, "Dependency resolution failed, falling back to priority ordering");
                    return self.middleware.clone();
                }
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
    middleware: Vec<Arc<dyn Middleware>>,
    /// Execution strategy
    execution: ChainExecution,
    /// Conditional execution rules
    conditions: HashMap<String, Box<dyn Fn(&RequestContext) -> bool + Send + Sync>>,
    /// Global timeout
    global_timeout: Option<Duration>,
    /// Enable metrics collection
    enable_metrics: bool,
    /// Performance metrics
    metrics: Arc<std::sync::Mutex<ChainMetrics>>,
}

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
        }
        
        builder.build()
    }

    /// Execute the middleware chain
    #[instrument(skip(self, context, response, handler))]
    pub async fn execute(
        &self,
        mut context: RequestContext,
        mut response: ResponseContext,
        handler: Arc<dyn RequestHandler>,
    ) -> HandlerResult {
        let chain_start = Instant::now();
        let execution_span = Span::current();
        
        // Apply global timeout if configured
        let execution_future = async {
            self.execute_internal(&mut context, &mut response, handler).await
        };

        let result = if let Some(timeout) = self.global_timeout {
            match tokio::time::timeout(timeout, execution_future).await {
                Ok(result) => result,
                Err(_) => {
                    error!(timeout_ms = timeout.as_millis(), "Middleware chain timed out");
                    response.set_status(crate::stdlib::web_vibez::StatusCode(408)); // Request Timeout
                    response.set_text("Request timeout");
                    Ok(response)
                }
            }
        } else {
            execution_future.await
        };

        // Update metrics if enabled
        if self.enable_metrics {
            let total_time = chain_start.elapsed();
            if let Ok(mut metrics) = self.metrics.lock() {
                metrics.total_execution_time = total_time;
                if result.is_ok() {
                    metrics.successful_executions += 1;
                } else {
                    metrics.failed_executions += 1;
                }

                let total_executions = metrics.successful_executions + metrics.failed_executions;
                if total_executions > 0 {
                    metrics.average_execution_time = Duration::from_nanos(
                        (metrics.total_execution_time.as_nanos() as u64) / total_executions
                    );
                }
            }
        }

        result
    }

    /// Internal execution logic
    async fn execute_internal(
        &self,
        context: &mut RequestContext,
        response: &mut ResponseContext,
        handler: Arc<dyn RequestHandler>,
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
                _ => middleware.before_request(context, response),
            };

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
                                error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
                            }
                            return Ok(response.clone());
                        }
                        ChainExecution::ContinueOnError => {
                            // Try to handle the error but continue
                            if let Err(handle_error) = middleware.on_error(context, response, &e) {
                                error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
                            }
                            errors.push((middleware.name().to_string(), e));
                        }
                        _ => {
                            // Try to handle the error
                            if let Err(handle_error) = middleware.on_error(context, response, &e) {
                                error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
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
                response.set_status(crate::stdlib::web_vibez::StatusCode::InternalServerError);
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
                _ => middleware.after_response(context, response),
            };

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
                            error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
                        }
                        errors.push((middleware.name().to_string(), e));
                    }
                    _ => {
                        // Try to handle the error
                        if let Err(handle_error) = middleware.on_error(context, response, &e) {
                            error!(middleware = middleware.name(), error = %handle_error, "Error in middleware error handler");
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
    }

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
    }

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
        use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, CorsMiddleware};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(CorsMiddleware::new()))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast)
    }

    /// Create a secure API chain with authentication
    pub fn secure_api() -> ChainBuilder {
        use crate::stdlib::web_vibez::middleware::{
            LoggingMiddleware, CorsMiddleware, AuthMiddleware, RateLimitMiddleware, AuthScheme
        };

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(RateLimitMiddleware::new(1000))) // 1000 req/min
            .add(Arc::new(AuthMiddleware::new(Vec::from([AuthScheme::Bearer]))))
            .add(Arc::new(CorsMiddleware::new()))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast)
    }

    /// Create a static file serving chain
    pub fn static_files(root_dir: std::path::PathBuf) -> ChainBuilder {
        use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, StaticFileMiddleware};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new().with_skip_paths(Vec::from(["/health".to_string()]))))
            .add(Arc::new(StaticFileMiddleware::new(root_dir, "/static")))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::Sequential)
    }

    /// Create a development chain with verbose logging
    pub fn development() -> ChainBuilder {
        use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, CorsMiddleware, LogLevel};

        ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new().with_body_logging(true, true)))
            .add(Arc::new(CorsMiddleware::new().with_origins(Vec::from(["*".to_string()]))))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::ContinueOnError)
            .with_metrics(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::web_vibez::middleware::{LoggingMiddleware, CorsMiddleware};
    use crate::stdlib::web_vibez::handlers::StaticHandler;
    use crate::stdlib::web_vibez::HttpMethod;

    #[tokio::test]
    async fn test_chain_builder() {
        let chain = ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(CorsMiddleware::new()))
            .with_ordering(MiddlewareOrdering::Priority)
            .with_execution(ChainExecution::FailFast)
            .build();

        assert_eq!(chain.middleware_count(), 2);
    }

    #[tokio::test]
    async fn test_chain_execution() {
        let chain = ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .build();

        let handler = Arc::new(StaticHandler::new("Test response"));
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let response = ResponseContext::new();

        let result = chain.execute(context, response, handler).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_conditional_middleware() {
        let chain = ChainBuilder::new()
            .add_conditional(
                Arc::new(CorsMiddleware::new()),
                |ctx| ctx.path.starts_with("/api")
            )
            .build();

        let handler = Arc::new(StaticHandler::new("Test"));
        
        // Test with API path
        let context = RequestContext::new("GET".to_string(), "/api/test".to_string());
        let response = ResponseContext::new();
        let result = chain.execute(context, response, handler.clone()).await;
        assert!(result.is_ok());

        // Test with non-API path
        let context = RequestContext::new("GET".to_string(), "/static/file.js".to_string());
        let response = ResponseContext::new();
        let result = chain.execute(context, response, handler).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_dependency_ordering() {
        let builder = ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .add(Arc::new(CorsMiddleware::new()))
            .add_dependency(MiddlewareDependency {
                middleware_name: "CORS".to_string(),
                depends_on: Vec::from(["Logging".to_string()]),
                runs_before: Vec::from([]),
            })
            .with_ordering(MiddlewareOrdering::Dependency);

        let chain = builder.build();
        let names = chain.middleware_names();
        
        // Logging should come before CORS due to dependency
        let logging_pos = names.iter().position(|n| n == "Logging");
        let cors_pos = names.iter().position(|n| n == "CORS");
        
        assert!(logging_pos.unwrap() < cors_pos.unwrap());
    }

    #[tokio::test]
    async fn test_chain_patterns() {
        let web_api_chain = ChainPatterns::web_api().build();
        assert!(web_api_chain.middleware_count() >= 2);

        let secure_api_chain = ChainPatterns::secure_api().build();
        assert!(secure_api_chain.middleware_count() >= 4);

        let dev_chain = ChainPatterns::development().build();
        assert!(dev_chain.middleware_count() >= 2);
    }

    #[tokio::test]
    async fn test_metrics_collection() {
        let chain = ChainBuilder::new()
            .add(Arc::new(LoggingMiddleware::new()))
            .with_metrics(true)
            .build();

        let handler = Arc::new(StaticHandler::new("Test"));
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let response = ResponseContext::new();

        let _ = chain.execute(context, response, handler).await;
        
        let metrics = chain.get_metrics().unwrap();
        assert_eq!(metrics.successful_executions, 1);
        assert!(metrics.total_execution_time > Duration::ZERO);
    }
}
