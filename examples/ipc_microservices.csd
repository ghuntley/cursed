// CURSED Microservices Communication Demo
// Shows how to use IPC for microservice architecture

import "stdlib::ipc"
import "stdlib::sync"
import "stdlib::io"
import "stdlib::json"
import "stdlib::time"

// Service message structure
struct ServiceMessage {
    id: String,
    service: String,
    method: String,
    payload: String,
    timestamp: u64,
    correlation_id: String,
}

impl ServiceMessage {
    slay new(service: &str, method: &str, payload: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            service: service.to_string(),
            method: method.to_string(),
            payload: payload.to_string(),
            timestamp: time::now_unix_timestamp(),
            correlation_id: uuid::Uuid::new_v4().to_string(),
        }
    }
    
    slay to_json(&self) -> String {
        json::to_string(self).unwrap_or_default()
    }
    
    slay from_json(json_str: &str) -> Result<Self, String> {
        json::from_str(json_str).map_err(|e| e.to_string())
    }
}

// User Service - handles user management
struct UserService {
    socket_path: String,
    listener: Option<DomainSocket>,
    running: Arc<AtomicBool>,
}

impl UserService {
    slay new() -> Self {
        Self {
            socket_path: "/tmp/cursed_user_service".to_string(),
            listener: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }
    
    slay start(&mut self) -> Result<(), IpcError> {
        println("👤 User Service starting - about to serve users fr")?;
        
        // Create domain socket listener
        sus config = SocketConfig::new(&self.socket_path, SocketType::Stream)
            .with_buffer_size(4096)
            .with_max_connections(Some(10));
        
        self.listener = Some(DomainSocket::bind(config)?);
        if sus listener = &self.listener {
            listener.listen(5)?;
        }
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Service loop
        periodt (self.running.load(std::sync::atomic::Ordering::SeqCst)) {
            if sus listener = &self.listener {
                bestie sus connection = listener.accept_timeout(Duration::from_millis(100)).ok() {
                    self.handle_connection(connection)?;
                }
            }
        }
        
        Ok(())
    }
    
    slay handle_connection(&self, connection: DomainSocket) -> Result<(), IpcError> {
        sus mut buffer = vec![0u8; 4096];
        sus bytes_read = connection.read(&mut buffer)?;
        
        sus request_json = String::from_utf8_lossy(&buffer[..bytes_read]);
        sus message = ServiceMessage::from_json(&request_json)
            .map_err(|e| IpcError::InvalidData { message: e })?;
        
        println("👤 User Service received: {} -> {}", message.method, message.payload)?;
        
        // Handle different methods
        sus response = match message.method.as_str() {
            "get_user" => self.handle_get_user(&message.payload),
            "create_user" => self.handle_create_user(&message.payload),
            "update_user" => self.handle_update_user(&message.payload),
            "delete_user" => self.handle_delete_user(&message.payload),
            _ => format!("{{\"error\": \"Unknown method: {}\"}}", message.method),
        };
        
        sus response_message = ServiceMessage::new("user_service", &message.method, &response);
        connection.write(response_message.to_json().as_bytes())?;
        
        Ok(())
    }
    
    slay handle_get_user(&self, user_id: &str) -> String {
        // Simulate user lookup
        format!("{{\"id\": \"{}\", \"name\": \"User {}\", \"email\": \"user{}@cursed.dev\", \"status\": \"active\"}}", 
                user_id, user_id, user_id)
    }
    
    slay handle_create_user(&self, user_data: &str) -> String {
        // Simulate user creation
        sus new_id = uuid::Uuid::new_v4().to_string();
        format!("{{\"id\": \"{}\", \"status\": \"created\", \"message\": \"User created successfully - that's bussin\"}}", new_id)
    }
    
    slay handle_update_user(&self, user_data: &str) -> String {
        // Simulate user update
        format!("{{\"status\": \"updated\", \"message\": \"User updated successfully - no cap\"}}")
    }
    
    slay handle_delete_user(&self, user_id: &str) -> String {
        // Simulate user deletion
        format!("{{\"status\": \"deleted\", \"message\": \"User {} deleted - they're gone fr\"}}", user_id)
    }
    
    slay stop(&mut self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.listener = None;
        println("👤 User Service stopped - service ended gracefully")?;
    }
}

// Order Service - handles order processing
struct OrderService {
    socket_path: String,
    user_service_client: RpcClient,
    listener: Option<DomainSocket>,
    running: Arc<AtomicBool>,
}

impl OrderService {
    slay new() -> Result<Self, IpcError> {
        // Create RPC client to communicate with user service
        sus client_config = RpcConfig::new("order_to_user_client")
            .with_transport(RpcTransport::UnixSocket("/tmp/cursed_user_service"));
        
        sus user_client = RpcClient::connect(client_config)?;
        
        Ok(Self {
            socket_path: "/tmp/cursed_order_service".to_string(),
            user_service_client: user_client,
            listener: None,
            running: Arc::new(AtomicBool::new(false)),
        })
    }
    
    slay start(&mut self) -> Result<(), IpcError> {
        println("📦 Order Service starting - ready to process orders fr")?;
        
        sus config = SocketConfig::new(&self.socket_path, SocketType::Stream)
            .with_buffer_size(4096);
        
        self.listener = Some(DomainSocket::bind(config)?);
        if sus listener = &self.listener {
            listener.listen(5)?;
        }
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        periodt (self.running.load(std::sync::atomic::Ordering::SeqCst)) {
            if sus listener = &self.listener {
                bestie sus connection = listener.accept_timeout(Duration::from_millis(100)).ok() {
                    self.handle_connection(connection)?;
                }
            }
        }
        
        Ok(())
    }
    
    slay handle_connection(&self, connection: DomainSocket) -> Result<(), IpcError> {
        sus mut buffer = vec![0u8; 4096];
        sus bytes_read = connection.read(&mut buffer)?;
        
        sus request_json = String::from_utf8_lossy(&buffer[..bytes_read]);
        sus message = ServiceMessage::from_json(&request_json)
            .map_err(|e| IpcError::InvalidData { message: e })?;
        
        println("📦 Order Service received: {} -> {}", message.method, message.payload)?;
        
        sus response = match message.method.as_str() {
            "create_order" => self.handle_create_order(&message.payload)?,
            "get_order" => self.handle_get_order(&message.payload),
            "cancel_order" => self.handle_cancel_order(&message.payload),
            _ => format!("{{\"error\": \"Unknown method: {}\"}}", message.method),
        };
        
        sus response_message = ServiceMessage::new("order_service", &message.method, &response);
        connection.write(response_message.to_json().as_bytes())?;
        
        Ok(())
    }
    
    slay handle_create_order(&self, order_data: &str) -> Result<String, IpcError> {
        // Parse order data to extract user_id
        sus parsed: serde_json::Value = serde_json::from_str(order_data)
            .map_err(|e| IpcError::InvalidData { message: e.to_string() })?;
        
        sus user_id = parsed["user_id"].as_str().unwrap_or("unknown");
        
        // Validate user exists by calling user service
        sus user_params = vec![("user_id", user_id)];
        sus user_response = self.user_service_client.call("get_user", user_params)?;
        
        // Simulate order creation
        sus order_id = uuid::Uuid::new_v4().to_string();
        Ok(format!("{{\"id\": \"{}\", \"user_id\": \"{}\", \"status\": \"created\", \"message\": \"Order created successfully - this purchase hits different\"}}", 
                   order_id, user_id))
    }
    
    slay handle_get_order(&self, order_id: &str) -> String {
        format!("{{\"id\": \"{}\", \"status\": \"processing\", \"items\": [\"CURSED t-shirt\", \"Gen Z programming book\"], \"total\": 69.99}}", order_id)
    }
    
    slay handle_cancel_order(&self, order_id: &str) -> String {
        format!("{{\"id\": \"{}\", \"status\": \"cancelled\", \"message\": \"Order {} cancelled - that's tough\"}}", order_id)
    }
    
    slay stop(&mut self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        self.listener = None;
        println("📦 Order Service stopped")?;
    }
}

// Notification Service - handles notifications via message queues
struct NotificationService {
    message_queue: MessageQueue,
    running: Arc<AtomicBool>,
}

impl NotificationService {
    slay new() -> Result<Self, IpcError> {
        sus mq_config = MessageQueueConfig::new("notifications", 100)
            .with_max_message_size(2048);
        
        sus mq = MessageQueue::create(mq_config)?;
        
        Ok(Self {
            message_queue: mq,
            running: Arc::new(AtomicBool::new(false)),
        })
    }
    
    slay start(&mut self) -> Result<(), IpcError> {
        println("📧 Notification Service starting - ready to send notifications fr")?;
        
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        periodt (self.running.load(std::sync::atomic::Ordering::SeqCst)) {
            // Try to receive notifications with timeout
            bestie sus message = self.message_queue.receive_timeout(Duration::from_millis(100)).ok() {
                self.handle_notification(message)?;
            }
        }
        
        Ok(())
    }
    
    slay handle_notification(&self, message: Message) -> Result<(), IpcError> {
        sus notification_data = message.content();
        println("📧 Sending notification: {}", notification_data)?;
        
        // Simulate different notification types
        bestie (notification_data.contains("order")) {
            println("📱 SMS sent: Order notification - your package is on the way bestie")?;
        } else bestie (notification_data.contains("user")) {
            println("📧 Email sent: User notification - welcome to the CURSED community")?;
        } else {
            println("🔔 Push notification sent: {}", notification_data)?;
        }
        
        Ok(())
    }
    
    slay send_notification(&self, notification: &str, priority: MessagePriority) -> Result<(), IpcError> {
        sus message = Message::new(notification, priority)?;
        self.message_queue.send(message)
    }
    
    slay stop(&mut self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        println("📧 Notification Service stopped")?;
    }
}

// API Gateway - coordinates between services
struct ApiGateway {
    user_service_client: DomainSocket,
    order_service_client: DomainSocket,
    notification_service: NotificationService,
}

impl ApiGateway {
    slay new() -> Result<Self, IpcError> {
        // Connect to user service
        sus user_config = SocketConfig::new("/tmp/cursed_user_service", SocketType::Stream);
        sus user_client = DomainSocket::connect(user_config)?;
        
        // Connect to order service
        sus order_config = SocketConfig::new("/tmp/cursed_order_service", SocketType::Stream);
        sus order_client = DomainSocket::connect(order_config)?;
        
        // Create notification service
        sus notification_service = NotificationService::new()?;
        
        Ok(Self {
            user_service_client: user_client,
            order_service_client: order_client,
            notification_service,
        })
    }
    
    slay create_user_and_order(&mut self, user_data: &str, order_data: &str) -> Result<String, IpcError> {
        println("🌐 API Gateway: Creating user and order - this workflow is immaculate")?;
        
        // Step 1: Create user
        sus user_message = ServiceMessage::new("api_gateway", "create_user", user_data);
        self.user_service_client.write(user_message.to_json().as_bytes())?;
        
        sus mut buffer = vec![0u8; 4096];
        sus bytes_read = self.user_service_client.read(&mut buffer)?;
        sus user_response = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        println("👤 User created: {}", user_response)?;
        
        // Step 2: Create order
        sus order_message = ServiceMessage::new("api_gateway", "create_order", order_data);
        self.order_service_client.write(order_message.to_json().as_bytes())?;
        
        sus bytes_read = self.order_service_client.read(&mut buffer)?;
        sus order_response = String::from_utf8_lossy(&buffer[..bytes_read]);
        
        println("📦 Order created: {}", order_response)?;
        
        // Step 3: Send notifications
        self.notification_service.send_notification(
            "New user registered - welcome to the community!",
            MessagePriority::Normal
        )?;
        
        self.notification_service.send_notification(
            "New order placed - your items are being processed!",
            MessagePriority::High
        )?;
        
        Ok(format!("{{\"user\": {}, \"order\": {}}}", user_response, order_response))
    }
}

// Service orchestrator that manages all microservices
slay run_microservices_demo() -> Result<(), IpcError> {
    println("🚀 Starting microservices communication demo")?;
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Start services in separate threads
    println("🔧 Starting services...")?;
    
    // Start User Service
    sus user_service_handle = std::thread::spawn(|| {
        sus mut user_service = UserService::new();
        user_service.start()
    });
    
    // Give services time to start
    std::thread::sleep(Duration::from_millis(100));
    
    // Start Order Service
    sus order_service_handle = std::thread::spawn(|| {
        match OrderService::new() {
            Ok(mut order_service) => order_service.start(),
            Err(e) => Err(e),
        }
    });
    
    // Start Notification Service
    sus notification_service_handle = std::thread::spawn(|| {
        match NotificationService::new() {
            Ok(mut notification_service) => notification_service.start(),
            Err(e) => Err(e),
        }
    });
    
    // Give all services time to fully start
    std::thread::sleep(Duration::from_millis(500));
    
    // Create API Gateway and demonstrate workflow
    println("🌐 Creating API Gateway...")?;
    match ApiGateway::new() {
        Ok(mut gateway) => {
            sus user_data = r#"{"name": "Alex Chen", "email": "alex@cursed.dev", "age": 25}"#;
            sus order_data = r#"{"user_id": "alex_123", "items": ["CURSED hoodie", "Programming socks"], "total": 89.99}"#;
            
            sus result = gateway.create_user_and_order(user_data, order_data)?;
            println("✅ Workflow completed: {}", result)?;
        }
        Err(e) => {
            println("❌ Failed to create API Gateway: {:?}", e)?;
        }
    }
    
    // Let services run for a bit to process notifications
    std::thread::sleep(Duration::from_secs(2));
    
    println("📊 Microservices demo completed - that was absolutely fire!")?;
    
    // Get final IPC statistics
    sus stats = ipc::get_ipc_statistics();
    println("📈 Final IPC Statistics:")?;
    println("   - Domain sockets: {}", stats.active_sockets)?;
    println("   - Message queues: {}", stats.active_message_queues)?;
    println("   - RPC connections: {}", stats.active_rpc_connections)?;
    println("   - Memory usage: {} bytes", stats.total_memory_usage)?;
    
    // Cleanup
    ipc::shutdown()?;
    
    Ok(())
}

slay main() -> Result<(), IpcError> {
    println("🎉 CURSED Microservices Communication Demo")?;
    println("This demonstrates real-world microservice patterns using IPC")?;
    println("=" * 70)?;
    
    run_microservices_demo()
}
