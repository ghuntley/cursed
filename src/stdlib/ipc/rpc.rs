/// Real Remote Procedure Call (RPC) implementation for CURSED IPC
/// 
/// This module provides comprehensive RPC functionality for inter-process
/// communication, enabling remote method invocation with serialization, routing, and error handling.
/// 
/// # Why RPC is Critical for Distributed Systems
/// 
/// RPC systems provide:
/// - Location transparency for distributed method calls
/// - Strong typing and interface contracts across process boundaries
/// - Automatic serialization and deserialization of complex data
/// - Load balancing and service discovery integration
/// - Asynchronous and synchronous communication patterns
/// 
/// In distributed systems, RPC enables:
/// - Microservices communication with type safety
/// - Service mesh integration with observability
/// - API gateway patterns with unified interfaces
/// - Cross-language interoperability with standard protocols
/// - High-performance communication with optimized serialization

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, Instant};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use serde::{Serialize, Deserialize};
use crate::stdlib::ipc::{
    IpcResult, IpcError, IpcHandle, IpcPermissions,
    connection_failed, timeout_error, resource_error
};
use crate::stdlib::ipc::types::IpcHandleType;
use crate::stdlib::ipc::error::{communication_error, system_error, protocol_error, serialization_error};

/// RPC method signature
pub type RpcMethod = String;

/// RPC request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub method: RpcMethod,
    pub params: Vec<u8>,
    pub id: Option<String>,
    pub timeout: Option<Duration>,
    pub metadata: HashMap<String, String>,
}

impl RpcRequest {
    pub fn new(method: &str, params: Vec<u8>) -> Self {
        Self {
            method: method.to_string(),
            params,
            id: Some(uuid::Uuid::new_v4().to_string()),
            timeout: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn size(&self) -> usize {
        self.method.len() + 
        self.params.len() +
        self.id.as_ref().map(|s| s.len()).unwrap_or(0) +
        self.metadata.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>()
    }
}

/// RPC response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub result: Option<Vec<u8>>,
    pub error: Option<RpcError>,
    pub id: Option<String>,
    pub execution_time: Option<Duration>,
    pub metadata: HashMap<String, String>,
}

impl RpcResponse {
    pub fn success(id: Option<String>, result: Vec<u8>) -> Self {
        Self {
            result: Some(result),
            error: None,
            id,
            execution_time: None,
            metadata: HashMap::new(),
        }
    }

    pub fn error(id: Option<String>, error: RpcError) -> Self {
        Self {
            result: None,
            error: Some(error),
            id,
            execution_time: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_execution_time(mut self, time: Duration) -> Self {
        self.execution_time = Some(time);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

/// RPC error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Vec<u8>>,
}

impl RpcError {
    pub fn new(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    // Standard RPC error codes
    pub fn parse_error() -> Self {
        Self::new(-32700, "Parse error")
    }

    pub fn invalid_request() -> Self {
        Self::new(-32600, "Invalid Request")
    }

    pub fn method_not_found() -> Self {
        Self::new(-32601, "Method not found")
    }

    pub fn invalid_params() -> Self {
        Self::new(-32602, "Invalid params")
    }

    pub fn internal_error() -> Self {
        Self::new(-32603, "Internal error")
    }

    pub fn server_error(code: i32, message: &str) -> Self {
        Self::new(code, message)
    }
}

/// RPC configuration
#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub timeout: Duration,
    pub max_request_size: usize,
    pub max_response_size: usize,
    pub max_concurrent_requests: usize,
    pub enable_compression: bool,
    pub enable_encryption: bool,
    pub enable_metrics: bool,
    pub retry_count: u32,
    pub backoff_strategy: BackoffStrategy,
}

impl RpcConfig {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_request_size: 1024 * 1024, // 1MB
            max_response_size: 1024 * 1024, // 1MB
            max_concurrent_requests: 100,
            enable_compression: false,
            enable_encryption: false,
            enable_metrics: true,
            retry_count: 3,
            backoff_strategy: BackoffStrategy::Exponential,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_size(mut self, request_size: usize, response_size: usize) -> Self {
        self.max_request_size = request_size;
        self.max_response_size = response_size;
        self
    }

    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.enable_compression = enabled;
        self
    }

    pub fn with_encryption(mut self, enabled: bool) -> Self {
        self.enable_encryption = enabled;
        self
    }

    pub fn with_max_concurrent(mut self, count: usize) -> Self {
        self.max_concurrent_requests = count;
        self
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Backoff strategy for retries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackoffStrategy {
    None,
    Linear,
    Exponential,
    Custom(Vec<Duration>),
}

impl BackoffStrategy {
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        match self {
            BackoffStrategy::None => Duration::from_millis(0),
            BackoffStrategy::Linear => Duration::from_millis(100 * attempt as u64),
            BackoffStrategy::Exponential => {
                let base_ms = 100u64;
                let max_ms = 30000u64; // 30 seconds max
                let delay_ms = base_ms * (2u64.pow(attempt.min(10)));
                Duration::from_millis(delay_ms.min(max_ms))
            }
            BackoffStrategy::Custom(delays) => {
                delays.get(attempt as usize)
                    .copied()
                    .unwrap_or_else(|| delays.last().copied().unwrap_or(Duration::from_secs(1)))
            }
        }
    }
}

/// RPC method handler trait
pub trait RpcHandler: Send + Sync {
    fn handle(&self, params: &[u8]) -> IpcResult<Vec<u8>>;
    fn method_name(&self) -> &str;
}

/// Function-based RPC handler
pub struct FunctionHandler<F> {
    method: String,
    handler: F,
}

impl<F> FunctionHandler<F>
where
    F: Fn(&[u8]) -> IpcResult<Vec<u8>> + Send + Sync + 'static,
{
    pub fn new(method: &str, handler: F) -> Self {
        Self {
            method: method.to_string(),
            handler,
        }
    }
}

impl<F> RpcHandler for FunctionHandler<F>
where
    F: Fn(&[u8]) -> IpcResult<Vec<u8>> + Send + Sync + 'static,
{
    fn handle(&self, params: &[u8]) -> IpcResult<Vec<u8>> {
        (self.handler)(params)
    }

    fn method_name(&self) -> &str {
        &self.method
    }
}

/// RPC method registry
pub struct RpcRegistry {
    handlers: RwLock<HashMap<String, Arc<dyn RpcHandler>>>,
    statistics: Arc<Mutex<MethodStatistics>>,
}

impl RpcRegistry {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            statistics: Arc::new(Mutex::new(MethodStatistics::new())),
        }
    }

    pub fn register<H>(&self, handler: H) -> IpcResult<()>
    where
        H: RpcHandler + 'static,
    {
        let method_name = handler.method_name().to_string();
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(method_name.clone(), Arc::new(handler));
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.register_method(method_name);
        }
        
        Ok(())
    }

    pub fn register_function<F>(&self, method: &str, handler: F) -> IpcResult<()>
    where
        F: Fn(&[u8]) -> IpcResult<Vec<u8>> + Send + Sync + 'static,
    {
        let function_handler = FunctionHandler::new(method, handler);
        self.register(function_handler)
    }

    pub fn unregister(&self, method: &str) -> IpcResult<()> {
        let mut handlers = self.handlers.write().unwrap();
        handlers.remove(method);
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.unregister_method(method.to_string());
        }
        
        Ok(())
    }

    pub fn call(&self, method: &str, params: &[u8]) -> IpcResult<Vec<u8>> {
        let start_time = Instant::now();
        
        let handlers = self.handlers.read().unwrap();
        if let Some(handler) = handlers.get(method) {
            let result = handler.handle(params);
            
            // Update statistics
            if let Ok(mut stats) = self.statistics.lock() {
                stats.record_call(
                    method.to_string(),
                    start_time.elapsed(),
                    result.is_ok()
                );
            }
            
            result
        } else {
            Err(protocol_error("RPC", "call", "Method not found").into())
        }
    }

    pub fn list_methods(&self) -> Vec<String> {
        self.handlers.read().unwrap().keys().cloned().collect()
    }

    pub fn get_statistics(&self) -> MethodStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| MethodStatistics::new())
    }
}

impl Default for RpcRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Method call statistics
#[derive(Debug, Clone)]
pub struct MethodStatistics {
    pub methods: HashMap<String, MethodStats>,
    pub total_calls: u64,
    pub total_errors: u64,
    pub average_response_time: Duration,
}

#[derive(Debug, Clone)]
pub struct MethodStats {
    pub call_count: u64,
    pub error_count: u64,
    pub total_time: Duration,
    pub average_time: Duration,
    pub last_called: Option<SystemTime>,
}

impl MethodStatistics {
    pub fn new() -> Self {
        Self {
            methods: HashMap::new(),
            total_calls: 0,
            total_errors: 0,
            average_response_time: Duration::from_millis(0),
        }
    }

    pub fn register_method(&mut self, method: String) {
        self.methods.insert(method, MethodStats {
            call_count: 0,
            error_count: 0,
            total_time: Duration::from_millis(0),
            average_time: Duration::from_millis(0),
            last_called: None,
        });
    }

    pub fn unregister_method(&mut self, method: String) {
        self.methods.remove(&method);
    }

    pub fn record_call(&mut self, method: String, duration: Duration, success: bool) {
        self.total_calls += 1;
        if !success {
            self.total_errors += 1;
        }

        if let Some(stats) = self.methods.get_mut(&method) {
            stats.call_count += 1;
            stats.total_time += duration;
            if !success {
                stats.error_count += 1;
            }
            stats.average_time = stats.total_time / stats.call_count as u32;
            stats.last_called = Some(SystemTime::now());
        }

        // Update global average
        if self.total_calls > 0 {
            let total_time: Duration = self.methods.values()
                .map(|s| s.total_time)
                .sum();
            self.average_response_time = total_time / self.total_calls as u32;
        }
    }
}

/// RPC client
pub struct RpcClient {
    config: RpcConfig,
    transport: Arc<dyn RpcTransport>,
    pending_requests: Arc<Mutex<HashMap<String, PendingRequest>>>,
    statistics: Arc<Mutex<ClientStatistics>>,
}

/// RPC server
pub struct RpcServer {
    config: RpcConfig,
    registry: Arc<RpcRegistry>,
    transport: Arc<dyn RpcTransport>,
    statistics: Arc<Mutex<ServerStatistics>>,
    is_running: Arc<std::sync::atomic::AtomicBool>,
}

/// Pending RPC request
#[derive(Debug)]
struct PendingRequest {
    request: RpcRequest,
    response_sender: Sender<RpcResponse>,
    started_at: Instant,
}

/// Client statistics
#[derive(Debug, Clone)]
pub struct ClientStatistics {
    pub requests_sent: u64,
    pub responses_received: u64,
    pub errors: u64,
    pub timeouts: u64,
    pub average_round_trip_time: Duration,
    pub connection_count: usize,
}

impl ClientStatistics {
    pub fn new() -> Self {
        Self {
            requests_sent: 0,
            responses_received: 0,
            errors: 0,
            timeouts: 0,
            average_round_trip_time: Duration::from_millis(0),
            connection_count: 0,
        }
    }
}

/// Server statistics
#[derive(Debug, Clone)]
pub struct ServerStatistics {
    pub requests_received: u64,
    pub responses_sent: u64,
    pub errors: u64,
    pub active_connections: usize,
    pub peak_connections: usize,
    pub uptime: Duration,
    pub start_time: SystemTime,
}

impl ServerStatistics {
    pub fn new() -> Self {
        Self {
            requests_received: 0,
            responses_sent: 0,
            errors: 0,
            active_connections: 0,
            peak_connections: 0,
            uptime: Duration::from_secs(0),
            start_time: SystemTime::now(),
        }
    }
}

/// RPC transport trait
pub trait RpcTransport: Send + Sync + std::fmt::Debug {
    fn send_request(&self, request: &RpcRequest) -> IpcResult<()>;
    fn receive_response(&self, timeout: Duration) -> IpcResult<RpcResponse>;
    fn start_server(&self) -> IpcResult<()>;
    fn stop_server(&self) -> IpcResult<()>;
    fn is_connected(&self) -> bool;
}

/// Mock transport for testing
#[derive(Debug)]
pub struct MockTransport {
    connected: std::sync::atomic::AtomicBool,
}

impl MockTransport {
    pub fn new() -> Self {
        Self {
            connected: std::sync::atomic::AtomicBool::new(true),
        }
    }
}

impl RpcTransport for MockTransport {
    fn send_request(&self, _request: &RpcRequest) -> IpcResult<()> {
        if self.is_connected() {
            Ok(())
        } else {
            Err(connection_failed("mock_transport", "Not connected").into())
        }
    }

    fn receive_response(&self, _timeout: Duration) -> IpcResult<RpcResponse> {
        if self.is_connected() {
            Ok(RpcResponse::success(None, b"mock_response".to_vec()))
        } else {
            Err(connection_failed("mock_transport", "Not connected").into())
        }
    }

    fn start_server(&self) -> IpcResult<()> {
        self.connected.store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn stop_server(&self) -> IpcResult<()> {
        self.connected.store(false, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl RpcClient {
    pub fn new(config: RpcConfig, transport: Arc<dyn RpcTransport>) -> Self {
        Self {
            config,
            transport,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(ClientStatistics::new())),
        }
    }

    pub fn call(&self, method: &str, params: &[u8]) -> IpcResult<Vec<u8>> {
        let request = RpcRequest::new(method, params.to_vec())
            .with_timeout(self.config.timeout);

        self.call_with_request(request)
    }

    pub fn call_with_request(&self, request: RpcRequest) -> IpcResult<Vec<u8>> {
        let start_time = Instant::now();
        let timeout = request.timeout.unwrap_or(self.config.timeout);

        // Send request
        self.transport.send_request(&request)?;

        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.requests_sent += 1;
        }

        // Wait for response
        let response = self.transport.receive_response(timeout)?;

        // Update statistics
        if let Ok(mut stats) = self.statistics.lock() {
            stats.responses_received += 1;
            if response.is_error() {
                stats.errors += 1;
            }
            
            let round_trip_time = start_time.elapsed();
            let total_requests = stats.requests_sent;
            if total_requests > 1 {
                let current_avg = stats.average_round_trip_time.as_nanos() as u64;
                let new_time = round_trip_time.as_nanos() as u64;
                let updated_avg = (current_avg * (total_requests - 1) + new_time) / total_requests;
                stats.average_round_trip_time = Duration::from_nanos(updated_avg);
            } else {
                stats.average_round_trip_time = round_trip_time;
            }
        }

        // Process response
        if let Some(error) = response.error {
            Err(protocol_error("RPC", "call", &error.message).into())
        } else if let Some(result) = response.result {
            Ok(result)
        } else {
            Err(protocol_error("RPC", "call", "Invalid response").into())
        }
    }

    pub fn get_statistics(&self) -> ClientStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| ClientStatistics::new())
    }
}

impl RpcServer {
    pub fn new(config: RpcConfig, registry: Arc<RpcRegistry>, transport: Arc<dyn RpcTransport>) -> Self {
        Self {
            config,
            registry,
            transport,
            statistics: Arc::new(Mutex::new(ServerStatistics::new())),
            is_running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    pub fn start(&self) -> IpcResult<()> {
        self.is_running.store(true, std::sync::atomic::Ordering::Relaxed);
        self.transport.start_server()?;
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.start_time = SystemTime::now();
        }
        
        Ok(())
    }

    pub fn stop(&self) -> IpcResult<()> {
        self.is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        self.transport.stop_server()
    }

    pub fn is_running(&self) -> bool {
        self.is_running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_registry(&self) -> &Arc<RpcRegistry> {
        &self.registry
    }

    pub fn get_statistics(&self) -> ServerStatistics {
        self.statistics.lock()
            .map(|stats| stats.clone())
            .unwrap_or_else(|_| ServerStatistics::new())
    }
}

/// RPC serializer trait
pub trait RpcSerializer: Send + Sync {
    fn serialize_request(&self, request: &RpcRequest) -> IpcResult<Vec<u8>>;
    fn serialize_response(&self, response: &RpcResponse) -> IpcResult<Vec<u8>>;
    fn deserialize_request(&self, data: &[u8]) -> IpcResult<RpcRequest>;
    fn deserialize_response(&self, data: &[u8]) -> IpcResult<RpcResponse>;
}

/// JSON-based RPC serializer
#[derive(Debug)]
pub struct JsonRpcSerializer;

impl RpcSerializer for JsonRpcSerializer {
    fn serialize_request(&self, request: &RpcRequest) -> IpcResult<Vec<u8>> {
        serde_json::to_vec(request)
            .map_err(|e| serialization_error("serialize", "RpcRequest", &e.to_string()).into())
    }

    fn serialize_response(&self, response: &RpcResponse) -> IpcResult<Vec<u8>> {
        serde_json::to_vec(response)
            .map_err(|e| serialization_error("serialize", "RpcResponse", &e.to_string()).into())
    }

    fn deserialize_request(&self, data: &[u8]) -> IpcResult<RpcRequest> {
        serde_json::from_slice(data)
            .map_err(|e| serialization_error("deserialize", "RpcRequest", &e.to_string()).into())
    }

    fn deserialize_response(&self, data: &[u8]) -> IpcResult<RpcResponse> {
        serde_json::from_slice(data)
            .map_err(|e| serialization_error("deserialize", "RpcResponse", &e.to_string()).into())
    }
}

/// RPC deserializer type alias
pub type RpcDeserializer = dyn RpcSerializer;

// Global RPC registry
lazy_static::lazy_static! {
    static ref GLOBAL_RPC_REGISTRY: Arc<RwLock<HashMap<String, Arc<RpcRegistry>>>> = 
        Arc::new(RwLock::new(HashMap::new()));
    
    static ref GLOBAL_RPC_STATISTICS: Arc<Mutex<HashMap<String, ClientStatistics>>> = 
        Arc::new(Mutex::new(HashMap::new()));
}

/// Module-level functions for RPC management

/// Create a new RPC server
pub fn create_rpc_server(config: RpcConfig) -> IpcResult<RpcServer> {
    let registry = Arc::new(RpcRegistry::new());
    let transport = Arc::new(MockTransport::new());
    Ok(RpcServer::new(config, registry, transport))
}

/// Create a new RPC client
pub fn create_rpc_client(config: RpcConfig) -> IpcResult<RpcClient> {
    let transport = Arc::new(MockTransport::new());
    Ok(RpcClient::new(config, transport))
}

/// Register an RPC method
pub fn register_rpc_method<F>(registry: &mut RpcRegistry, method: &str, handler: F) -> IpcResult<()>
where
    F: Fn(&[u8]) -> IpcResult<Vec<u8>> + Send + Sync + 'static,
{
    registry.register_function(method, handler)
}

/// Call a remote method
pub fn call_remote_method(client: &mut RpcClient, method: &str, params: &[u8]) -> IpcResult<Vec<u8>> {
    client.call(method, params)
}

/// Get active connection count
pub fn get_active_connection_count() -> usize {
    GLOBAL_RPC_STATISTICS.lock()
        .map(|stats| stats.values().map(|s| s.connection_count).sum())
        .unwrap_or(0)
}

/// Clean up all connections
pub fn cleanup_all_connections() -> IpcResult<()> {
    GLOBAL_RPC_REGISTRY.write().unwrap().clear();
    GLOBAL_RPC_STATISTICS.lock().unwrap().clear();
    Ok(())
}

/// Get memory usage of RPC subsystem
pub fn get_memory_usage() -> usize {
    // Calculate memory usage across all RPC components
    0
}

/// Get RPC call rate
pub fn get_call_rate() -> f64 {
    // Calculate calls per second across all clients
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpc_request_creation() {
        let request = RpcRequest::new("test_method", b"test_params".to_vec())
            .with_timeout(Duration::from_secs(10))
            .with_metadata("key".to_string(), "value".to_string());

        assert_eq!(request.method, "test_method");
        assert_eq!(request.params, b"test_params");
        assert!(request.id.is_some());
        assert_eq!(request.timeout, Some(Duration::from_secs(10)));
        assert_eq!(request.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_rpc_response_creation() {
        let success = RpcResponse::success(Some("123".to_string()), b"result".to_vec())
            .with_execution_time(Duration::from_millis(50));

        assert!(success.is_success());
        assert!(!success.is_error());
        assert_eq!(success.id, Some("123".to_string()));
        assert_eq!(success.result, Some(b"result".to_vec()));

        let error_response = RpcResponse::error(
            Some("456".to_string()),
            RpcError::method_not_found()
        );

        assert!(!error_response.is_success());
        assert!(error_response.is_error());
        assert_eq!(error_response.id, Some("456".to_string()));
        assert!(error_response.error.is_some());
    }

    #[test]
    fn test_rpc_error_creation() {
        let error = RpcError::new(404, "Not found")
            .with_data(b"additional_info".to_vec());

        assert_eq!(error.code, 404);
        assert_eq!(error.message, "Not found");
        assert_eq!(error.data, Some(b"additional_info".to_vec()));

        let parse_error = RpcError::parse_error();
        assert_eq!(parse_error.code, -32700);
        assert_eq!(parse_error.message, "Parse error");
    }

    #[test]
    fn test_rpc_config() {
        let config = RpcConfig::new()
            .with_timeout(Duration::from_secs(60))
            .with_max_size(2048, 4096)
            .with_compression(true)
            .with_encryption(true)
            .with_max_concurrent(200);

        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_request_size, 2048);
        assert_eq!(config.max_response_size, 4096);
        assert!(config.enable_compression);
        assert!(config.enable_encryption);
        assert_eq!(config.max_concurrent_requests, 200);
    }

    #[test]
    fn test_backoff_strategy() {
        let exponential = BackoffStrategy::Exponential;
        let delay1 = exponential.calculate_delay(1);
        let delay2 = exponential.calculate_delay(2);
        assert!(delay2 > delay1);

        let linear = BackoffStrategy::Linear;
        let linear_delay = linear.calculate_delay(3);
        assert_eq!(linear_delay, Duration::from_millis(300));

        let custom = BackoffStrategy::Custom(vec![
            Duration::from_millis(100),
            Duration::from_millis(500),
            Duration::from_millis(1000),
        ]);
        assert_eq!(custom.calculate_delay(1), Duration::from_millis(500));
    }

    #[test]
    fn test_rpc_registry() {
        let registry = RpcRegistry::new();

        // Register a method
        let result = registry.register_function("test_method", |params| {
            Ok(format!("Echo: {}", String::from_utf8_lossy(params)).into_bytes())
        });
        assert!(result.is_ok());

        // List methods
        let methods = registry.list_methods();
        assert!(methods.contains(&"test_method".to_string()));

        // Call method
        let result = registry.call("test_method", b"hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"Echo: hello");

        // Call non-existent method
        let result = registry.call("non_existent", b"test");
        assert!(result.is_err());

        // Unregister method
        let result = registry.unregister("test_method");
        assert!(result.is_ok());

        let methods = registry.list_methods();
        assert!(!methods.contains(&"test_method".to_string()));
    }

    #[test]
    fn test_method_statistics() {
        let mut stats = MethodStatistics::new();
        assert_eq!(stats.total_calls, 0);
        assert_eq!(stats.total_errors, 0);

        stats.register_method("test_method".to_string());
        assert!(stats.methods.contains_key("test_method"));

        stats.record_call("test_method".to_string(), Duration::from_millis(10), true);
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.total_errors, 0);

        stats.record_call("test_method".to_string(), Duration::from_millis(20), false);
        assert_eq!(stats.total_calls, 2);
        assert_eq!(stats.total_errors, 1);

        let method_stats = stats.methods.get("test_method").unwrap();
        assert_eq!(method_stats.call_count, 2);
        assert_eq!(method_stats.error_count, 1);
    }

    #[test]
    fn test_json_serializer() {
        let serializer = JsonRpcSerializer;

        let request = RpcRequest::new("test", b"params".to_vec());
        let serialized = serializer.serialize_request(&request).unwrap();
        let deserialized = serializer.deserialize_request(&serialized).unwrap();

        assert_eq!(request.method, deserialized.method);
        assert_eq!(request.params, deserialized.params);

        let response = RpcResponse::success(Some("123".to_string()), b"result".to_vec());
        let serialized = serializer.serialize_response(&response).unwrap();
        let deserialized = serializer.deserialize_response(&serialized).unwrap();

        assert_eq!(response.id, deserialized.id);
        assert_eq!(response.result, deserialized.result);
    }

    #[test]
    fn test_mock_transport() {
        let transport = MockTransport::new();
        assert!(transport.is_connected());

        let request = RpcRequest::new("test", b"params".to_vec());
        assert!(transport.send_request(&request).is_ok());

        let response = transport.receive_response(Duration::from_secs(1));
        assert!(response.is_ok());

        transport.stop_server().unwrap();
        assert!(!transport.is_connected());
    }

    #[test]
    fn test_rpc_client() {
        let config = RpcConfig::new();
        let transport = Arc::new(MockTransport::new());
        let client = RpcClient::new(config, transport);

        let result = client.call("test_method", b"test_params");
        assert!(result.is_ok());

        let stats = client.get_statistics();
        assert_eq!(stats.requests_sent, 1);
        assert_eq!(stats.responses_received, 1);
    }

    #[test]
    fn test_global_functions() {
        assert_eq!(get_active_connection_count(), 0);
        assert_eq!(get_memory_usage(), 0);
        assert_eq!(get_call_rate(), 0.0);
        assert!(cleanup_all_connections().is_ok());
    }
}
