use super::{HttpRequest, HttpResponse, WebResult};

/// Handler trait
pub trait Handler: Send + Sync {
    fn handle(&self, request: &HttpRequest) -> WebResult<HttpResponse>;
}

/// Function handler
pub struct FunctionHandler;
pub struct HandlerFunc;
pub struct HandlerChain;
pub struct StaticFileHandler;
pub struct JsonApiHandler;
pub struct RedirectHandler;
pub struct HealthCheckHandler;

impl Handler for FunctionHandler {
    fn handle(&self, _request: &HttpRequest) -> WebResult<HttpResponse> {
        Ok(HttpResponse {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: b"Hello from function handler".to_vec(),
        })
    }
}
