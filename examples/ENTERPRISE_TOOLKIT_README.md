# CURSED Enterprise Integration Toolkit

A comprehensive enterprise-grade toolkit for building production-ready applications with CURSED. This toolkit provides battle-tested integrations, monitoring, security, and deployment solutions for large-scale enterprise environments.

## 🏢 Enterprise Features

### Database Integration Suite
- **PostgreSQL**: Production-ready driver with connection pooling, prepared statements, and transactions
- **MySQL/MariaDB**: Enterprise-grade driver with clustering and replication support
- **MongoDB**: Modern ODM with aggregation pipeline and change streams
- **Redis**: High-performance client with cluster, sentinel, and pub/sub support
- **SQLite**: Embedded database with WAL mode and encryption
- **Migration Framework**: Database schema versioning and automated migrations

### Message Queue & Event Systems
- **Apache Kafka**: High-throughput producer/consumer with exactly-once semantics
- **RabbitMQ**: AMQP client with advanced routing and dead letter queues
- **AWS SQS/SNS**: Native cloud messaging integration
- **Event Sourcing**: CQRS pattern implementation with event replay
- **Stream Processing**: Real-time data processing with windowing and aggregation

### Cloud Service Integrations
- **AWS SDK**: Complete integration (S3, EC2, Lambda, RDS, DynamoDB)
- **Google Cloud Platform**: Comprehensive GCP service support
- **Azure SDK**: Microsoft Azure cloud services integration
- **Kubernetes**: Native deployment and service mesh integration
- **Docker**: Container orchestration and multi-stage builds

### Monitoring & Observability
- **Prometheus**: Native metrics collection and alerting
- **Grafana**: Pre-built dashboards and visualization templates
- **OpenTelemetry**: Distributed tracing and performance monitoring
- **Structured Logging**: JSON logging with correlation IDs
- **Health Checks**: Comprehensive application health monitoring

### Security & Compliance
- **OAuth 2.0/OpenID Connect**: Complete authentication framework
- **SAML**: Enterprise SSO integration
- **JWT**: Token-based authentication with rotation
- **Audit Logging**: Tamper-proof audit trails
- **GDPR**: Data privacy and compliance utilities

### Development & Deployment Tools
- **API Documentation**: Auto-generated OpenAPI/Swagger specs
- **Load Testing**: Built-in performance testing framework
- **Configuration Management**: Environment-based configuration
- **Secret Management**: HashiCorp Vault and cloud secret integration
- **CI/CD**: GitHub Actions and GitLab CI templates

## 🚀 Quick Start

### Installation

```bash
# Clone the CURSED repository
git clone https://github.com/ghuntley/cursed.git
cd cursed

# Build the enterprise toolkit
zig build

# Install enterprise modules
cursed-pkg install enterprise_db
cursed-pkg install enterprise_messaging
cursed-pkg install enterprise_cloud
cursed-pkg install enterprise_monitoring
cursed-pkg install enterprise_security
```

### Basic Usage

```cursed
// Database connection with pooling
yeet "enterprise_db"

sus pool enterprise_db.Pool = enterprise_db.create_default_pool(
    "postgres://user:pass@localhost:5432/mydb"
) fam {
    when err -> yikes "Database connection failed: " + err
}

sus users []User = pool.query(
    "SELECT * FROM users WHERE active = $1", 
    [based]
) fam {
    when err -> yikes "Query failed: " + err
}

// Kafka message production
yeet "enterprise_messaging"

sus producer enterprise_messaging.Producer = enterprise_messaging.create_simple_producer([
    "localhost:9092"
]) fam {
    when err -> yikes "Kafka producer failed: " + err
}

sus message enterprise_messaging.Message = {
    .topic = "user-events",
    .key = encode_string("user123"),
    .value = encode_json({"event": "login", "timestamp": timez.now()}),
}

producer.send(message) fam {
    when err -> yikes "Message send failed: " + err
}

// Cloud storage with S3
yeet "enterprise_cloud"

sus s3_client enterprise_cloud.S3Client = enterprise_cloud.create_s3_client({
    .access_key_id = "your-key",
    .secret_access_key = "your-secret",
    .region = "us-east-1",
})

s3_client.put_object("my-bucket", "file.txt", file_data, "text/plain") fam {
    when err -> yikes "S3 upload failed: " + err
}

// Prometheus metrics
yeet "enterprise_monitoring"

sus counter enterprise_monitoring.Counter = enterprise_monitoring.register_counter(
    "api_requests_total",
    "Total API requests",
    ["method", "status"]
)

counter.inc({"method": "GET", "status": "200"})
```

## 🏗️ Architecture Patterns

### Microservices Architecture

The enterprise toolkit supports modern microservices patterns:

```cursed
// Service registration and discovery
yeet "enterprise_monitoring"

squad UserService {
    db_pool enterprise_db.Pool
    kafka_producer enterprise_messaging.Producer
    metrics enterprise_monitoring.ApplicationMetrics
    
    slay create_user(user User) yikes<User> {
        // Database transaction
        sus created_user User = self.db_pool.transaction<User>(slay(conn enterprise_db.Connection) yikes<User> {
            // Insert user
            conn.query("INSERT INTO users (...) VALUES (...)", [user.data...]) fam {
                when err -> yikes err
            }
            damn user
        }) fam {
            when err -> yikes err
        }
        
        // Publish event
        self.publish_event("user.created", created_user) fam {
            when err -> vibez.spill("Event publish failed:", err)
        }
        
        // Update metrics
        self.metrics.record_business_metric("users_created", 1)
        
        damn created_user
    }
}
```

### Event-Driven Architecture

```cursed
// Event sourcing with Kafka
squad OrderEventProcessor {
    kafka_consumer enterprise_messaging.Consumer
    
    slay start_processing() {
        self.kafka_consumer.subscribe(["order-events", "payment-events"]) fam {
            when err -> yikes err
        }
        
        bestie (based) {
            sus messages []enterprise_messaging.Message = self.kafka_consumer.poll(1000) fam {
                when err -> continue
            }
            
            bestie (message := range messages) {
                self.process_event(message) fam {
                    when err -> vibez.spill("Event processing failed:", err)
                }
            }
        }
    }
    
    slay process_event(message enterprise_messaging.Message) yikes<tea> {
        sus event_type tea = decode_string(message.headers["event-type"])
        
        sick (event_type) {
            "order.created" -> self.handle_order_created(message)
            "payment.completed" -> self.handle_payment_completed(message)
            _ -> vibez.spill("Unknown event type:", event_type)
        }
    }
}
```

### API Gateway Pattern

```cursed
// Enterprise API Gateway with authentication and rate limiting
squad APIGateway {
    oauth_client enterprise_security.OAuthClient
    rate_limiter RateLimiter
    metrics enterprise_monitoring.ApplicationMetrics
    
    slay handle_request(request httpz.Request) httpz.Response {
        // Authentication
        sus user_info enterprise_security.UserInfo = self.authenticate_request(request) fam {
            when err -> damn httpz.error_response(401, "Unauthorized: " + err)
        }
        
        // Rate limiting
        ready (!self.rate_limiter.allow(user_info.sub)) {
            self.metrics.record_error("rate_limit", "warning")
            damn httpz.error_response(429, "Rate limit exceeded")
        }
        
        // Route to microservice
        damn self.proxy_to_service(request, user_info)
    }
}
```

## 🔧 Configuration Management

### Environment-Based Configuration

```cursed
yeet "configz"

squad ServiceConfig {
    database DatabaseConfig
    messaging MessagingConfig
    monitoring MonitoringConfig
    security SecurityConfig
}

slay load_config() ServiceConfig {
    damn ServiceConfig{
        .database = DatabaseConfig{
            .host = configz.get_env("DB_HOST", "localhost"),
            .port = configz.get_env_int("DB_PORT", 5432),
            .database = configz.get_env("DB_NAME", "myapp"),
            .username = configz.get_env("DB_USER", "user"),
            .password = configz.get_env("DB_PASSWORD", "password"),
            .max_connections = configz.get_env_int("DB_MAX_CONN", 20),
        },
        .messaging = MessagingConfig{
            .kafka_brokers = configz.get_env_list("KAFKA_BROKERS", ["localhost:9092"]),
            .consumer_group = configz.get_env("KAFKA_GROUP", "default-group"),
        },
        .monitoring = MonitoringConfig{
            .prometheus_port = configz.get_env_int("PROMETHEUS_PORT", 9090),
            .log_level = configz.get_env("LOG_LEVEL", "info"),
        },
        .security = SecurityConfig{
            .oauth_client_id = configz.get_env("OAUTH_CLIENT_ID", ""),
            .oauth_client_secret = configz.get_env("OAUTH_CLIENT_SECRET", ""),
            .jwt_secret = configz.get_env("JWT_SECRET", ""),
        },
    }
}
```

## 📊 Monitoring & Observability

### Prometheus Metrics

```cursed
yeet "enterprise_monitoring"

// Application metrics
sus app_metrics enterprise_monitoring.ApplicationMetrics = enterprise_monitoring.create_application_metrics()

// Business metrics
sus order_counter enterprise_monitoring.Counter = app_metrics.create_business_metric(
    "orders_processed", 
    "Total orders processed"
)

sus response_time_histogram enterprise_monitoring.Timer = enterprise_monitoring.register_timer(
    "http_request_duration_seconds",
    "HTTP request duration",
    ["method", "endpoint", "status"]
)

// Measure execution time
sus result T = response_time_histogram.time<T>(
    {"method": "POST", "endpoint": "/api/orders", "status": "200"}, 
    slay() T {
        // Your business logic here
        damn process_order()
    }
)
```

### Structured Logging

```cursed
yeet "enterprise_monitoring"

// Correlation ID for request tracing
sus correlation_id tea = generate_correlation_id()

// Structured logging with context
log_info("Order processing started", {
    "correlation_id": correlation_id,
    "user_id": user.id,
    "order_id": order.id,
    "timestamp": timez.now_iso8601(),
})

// Error logging with stack trace
log_error("Order processing failed", {
    "correlation_id": correlation_id,
    "error": error_message,
    "stack_trace": get_stack_trace(),
    "order_data": order,
})
```

## 🔐 Security Implementation

### OAuth 2.0 / OpenID Connect

```cursed
yeet "enterprise_security"

// OAuth client configuration
sus oauth_config enterprise_security.OAuthConfig = {
    .client_id = "your-client-id",
    .client_secret = "your-client-secret",
    .redirect_uri = "https://yourapp.com/callback",
    .authorization_endpoint = "https://auth.provider.com/oauth/authorize",
    .token_endpoint = "https://auth.provider.com/oauth/token",
    .userinfo_endpoint = "https://auth.provider.com/oauth/userinfo",
    .scope = ["openid", "profile", "email"],
    .pkce_enabled = based,  // Enable PKCE for security
}

sus oauth_client enterprise_security.OAuthClient = enterprise_security.create_oauth_client(oauth_config)

// Generate authorization URL
sus state tea = enterprise_security.generate_state()
sus nonce tea = enterprise_security.generate_nonce()
sus auth_url tea = oauth_client.get_authorization_url(state, nonce)

// Exchange code for tokens
sus token_response enterprise_security.TokenResponse = oauth_client.exchange_code_for_token(
    authorization_code, 
    state
) fam {
    when err -> yikes err
}

// Get user information
sus user_info enterprise_security.UserInfo = oauth_client.get_user_info(
    token_response.access_token
) fam {
    when err -> yikes err
}
```

### JWT Token Validation

```cursed
// Validate ID token
sus claims enterprise_security.JWTClaims = oauth_client.validate_id_token(
    token_response.id_token, 
    nonce
) fam {
    when err -> yikes "Invalid ID token: " + err
}

vibez.spill("Authenticated user:", claims.sub)
```

## ☁️ Cloud Integration

### AWS Services

```cursed
yeet "enterprise_cloud"

// AWS configuration
sus aws_config enterprise_cloud.AWSConfig = {
    .access_key_id = configz.get_env("AWS_ACCESS_KEY_ID", ""),
    .secret_access_key = configz.get_env("AWS_SECRET_ACCESS_KEY", ""),
    .region = configz.get_env("AWS_REGION", "us-east-1"),
}

sus aws_client enterprise_cloud.AWSClient = enterprise_cloud.create_aws_client(aws_config)

// S3 operations
sus s3_result enterprise_cloud.S3PutResult = aws_client.s3.put_object(
    "my-bucket", 
    "documents/file.pdf", 
    file_data, 
    "application/pdf"
) fam {
    when err -> yikes err
}

// EC2 operations
sus instances []enterprise_cloud.EC2Instance = aws_client.ec2.run_instances(
    "ami-0abcdef1234567890",  // Amazon Linux 2
    "t3.micro",
    1,  // min count
    1   // max count
) fam {
    when err -> yikes err
}

// Lambda invocation
sus lambda_result enterprise_cloud.LambdaInvokeResult = aws_client.lambda.invoke(
    "my-function",
    encode_json({"key": "value"}),
    "RequestResponse"
) fam {
    when err -> yikes err
}
```

## 🚢 Deployment

### Docker Containerization

```dockerfile
# Dockerfile for CURSED microservice
FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled CURSED binary
COPY zig-out/bin/cursed-microservice /usr/local/bin/

# Create non-root user
RUN useradd -r -s /bin/false cursed

# Switch to non-root user
USER cursed

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD cursed-microservice --health-check || exit 1

# Run the service
ENTRYPOINT ["/usr/local/bin/cursed-microservice"]
```

### Kubernetes Deployment

```bash
# Deploy the entire enterprise platform
kubectl apply -f examples/enterprise_deployment.yaml

# Scale services
kubectl scale deployment user-service --replicas=5 -n cursed-microservices

# Monitor deployment
kubectl get pods -n cursed-microservices
kubectl logs -f deployment/api-gateway -n cursed-microservices

# Access services
kubectl port-forward service/api-gateway 8080:8080 -n cursed-microservices
kubectl port-forward service/grafana 3000:3000 -n cursed-microservices
```

### Monitoring Setup

```bash
# Access Grafana dashboard
open http://localhost:3000
# Username: admin, Password: admin

# Access Prometheus
kubectl port-forward service/prometheus 9090:9090 -n cursed-microservices
open http://localhost:9090

# View metrics
curl http://localhost:8080/metrics
```

## 📈 Performance & Scalability

### Connection Pooling

```cursed
// Database connection pool configuration
sus pool_config enterprise_db.PoolConfig = {
    .min_connections = 5,
    .max_connections = 50,
    .acquire_timeout = 30000,      // 30 seconds
    .idle_timeout = 600000,        // 10 minutes
    .max_lifetime = 3600000,       // 1 hour
    .health_check_period = 30000,  // 30 seconds
}

sus pool enterprise_db.Pool = enterprise_db.create_pool(db_config, pool_config)
```

### Horizontal Pod Autoscaling

```yaml
# Automatic scaling based on CPU and memory
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: api-gateway-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: api-gateway
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Load Testing

```cursed
yeet "enterprise_monitoring"

// Built-in load testing framework
squad LoadTest {
    target_url tea
    concurrent_users drip
    duration drip
    
    slay run_load_test() yikes<LoadTestResults> {
        sus results LoadTestResults = {}
        
        // Spawn concurrent users
        sus workers []concurrenz.Goroutine = []
        bestie (i := 0; i < self.concurrent_users; i += 1) {
            workers = append(workers, go {
                self.simulate_user_load()
            })
        }
        
        // Wait for test duration
        concurrenz.sleep(self.duration * 1000)
        
        // Collect results
        damn self.collect_results()
    }
}
```

## 🔍 Troubleshooting

### Common Issues

1. **Database Connection Timeouts**
   ```cursed
   // Increase connection timeout
   sus config enterprise_db.ConnectionConfig = {
       .connect_timeout = 60000,  // 60 seconds
       .statement_timeout = 30000, // 30 seconds
   }
   ```

2. **Kafka Consumer Lag**
   ```cursed
   // Monitor consumer lag
   sus metrics enterprise_monitoring.ApplicationMetrics = create_application_metrics()
   sus lag_gauge enterprise_monitoring.Gauge = metrics.registry.register_gauge(
       "kafka_consumer_lag",
       "Kafka consumer lag",
       ["topic", "partition"]
   )
   ```

3. **Memory Leaks**
   ```bash
   # Run with memory profiling
   valgrind --leak-check=full ./zig-out/bin/cursed-microservice
   
   # Monitor memory usage
   kubectl top pods -n cursed-microservices
   ```

### Debugging

```cursed
// Enable debug logging
log_set_level("debug")

// Add correlation IDs for request tracing
sus correlation_id tea = request.headers["x-correlation-id"] fam {
    when _ -> generate_correlation_id()
}

// Trace database queries
sus query_timer enterprise_monitoring.Timer = enterprise_monitoring.register_timer(
    "database_query_duration",
    "Database query execution time",
    ["query_type"]
)
```

## 📚 Additional Resources

- [CURSED Language Documentation](https://cursedlang.org/docs)
- [Enterprise Patterns Guide](./ENTERPRISE_PATTERNS.md)
- [Security Best Practices](./SECURITY_GUIDE.md)
- [Performance Tuning](./PERFORMANCE_GUIDE.md)
- [Monitoring Setup](./MONITORING_GUIDE.md)
- [Deployment Guide](./DEPLOYMENT_GUIDE.md)

## 🤝 Contributing

We welcome contributions to the enterprise toolkit! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## 📄 License

This enterprise toolkit is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

---

**Enterprise Support**: For enterprise support, training, and consulting services, contact [support@cursedlang.org](mailto:support@cursedlang.org)
