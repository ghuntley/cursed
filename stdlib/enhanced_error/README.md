# Advanced Error Recovery Patterns

Enterprise-grade error handling system with correlation IDs, circuit breakers, distributed tracing, and automated recovery patterns for production applications.

## Features

### 🔗 Correlation ID and Trace Context
- **Distributed Tracing**: Full request correlation across microservices
- **Context Propagation**: Baggage and metadata propagation
- **Parent-Child Relationships**: Hierarchical span relationships
- **Cross-Service Correlation**: Unified error tracking

### 🔄 Enterprise Circuit Breakers
- **Adaptive Failure Detection**: Dynamic threshold adjustment
- **Success Rate Tracking**: Real-time performance monitoring
- **Escalation Patterns**: Automatic escalation on repeated failures
- **Recovery Strategies**: Half-open state testing and recovery

### 📊 Error Aggregation and Batch Processing
- **Intelligent Grouping**: Correlation and service-based grouping
- **Pattern Detection**: Critical error pattern recognition
- **Batch Processing**: Efficient bulk error handling
- **Recovery Action Triggering**: Automated recovery workflows

### 🚨 Advanced Error Types
- **Rich Context**: Metadata, recovery actions, propagation chains
- **Categorization**: Business logic, transient, timeout classification
- **Recovery Policies**: Configurable retry and escalation policies
- **Service Impact Tracking**: Multi-service failure analysis

### 🔍 Distributed Tracing Integration
- **Span Lifecycle**: Automatic span creation and management
- **Error Attribution**: Error-to-span correlation
- **Performance Tracking**: Response time and throughput metrics
- **Export Capabilities**: Batch export to tracing backends

## Quick Start

```cursed
yeet "enhanced_error.enterprise_recovery"

fr fr Initialize enterprise error handling
init_distributed_tracing("my_service")
init_error_monitoring()

fr fr Create correlation ID for request tracking
sus correlation correlation_id = new_correlation_id()
correlation.user_id = "user123"

fr fr Create trace context with baggage and tags
sus ctx trace_context = new_trace_context("process_payment", correlation)
ctx = ctx.add_baggage("customer_tier", "premium")
ctx = ctx.add_tag("payment_method", "credit_card")

fr fr Create circuit breaker for external service
sus config circuit_config = default_circuit_config()
config.failure_threshold = 5
sus cb circuit_breaker = new_enterprise_circuit_breaker("payment_service", config)

fr fr Execute operation with full error recovery
sus operation slay(trace_context) result<any, enhanced_error> = slay(ctx trace_context) result<any, enhanced_error> {
    fr fr Your business logic here
    damn ok_result("success")
}

sus result result<any, enhanced_error> = cb.execute_with_trace(ctx, operation)
```

## Correlation IDs and Trace Context

### Creating Correlation IDs

```cursed
fr fr Generate unique correlation ID
sus correlation correlation_id = new_correlation_id()
vibez.spill("Trace ID: " + correlation.trace_id)
vibez.spill("Span ID: " + correlation.span_id) 
vibez.spill("Request ID: " + correlation.request_id)

fr fr Add user context
correlation.user_id = "user12345"
correlation.session_id = "session_abc123"
```

### Trace Context Management

```cursed
fr fr Create trace context for operation
sus ctx trace_context = new_trace_context("user_authentication", correlation)

fr fr Add baggage (propagated to child spans)
ctx = ctx.add_baggage("customer_tier", "premium")
ctx = ctx.add_baggage("region", "us_east_1")
ctx = ctx.add_baggage("experiment_variant", "new_ui")

fr fr Add tags (span-specific metadata)
ctx = ctx.add_tag("auth_method", "oauth2")
ctx = ctx.add_tag("client_version", "v2.1.0")
ctx = ctx.add_tag("feature_flags", "fast_checkout,new_ui")

fr fr Create child context (inherits baggage, fresh tags)
sus child_ctx trace_context = ctx.child_context("database_query")
```

## Enterprise Circuit Breakers

### Configuration

```cursed
fr fr Create circuit breaker configuration
sus config circuit_config = circuit_config{
    failure_threshold: 5,        fr fr Open after 5 failures
    timeout_ms: 30000,           fr fr 30 second timeout
    max_failures: 20,            fr fr Max failures before escalation
    retry_after_ms: 60000,       fr fr 1 minute before retry
    escalation_threshold: 30,    fr fr Escalate after 30 failures
    tags: {                      fr fr Circuit breaker metadata
        "service_type": "payment",
        "criticality": "high",
        "sla": "99.9%"
    }
}

sus cb circuit_breaker = new_enterprise_circuit_breaker("payment_gateway", config)
```

### Circuit Breaker Execution

```cursed
fr fr Define operation to protect
sus payment_operation slay(trace_context) result<any, enhanced_error> = slay(ctx trace_context) result<any, enhanced_error> {
    fr fr Simulate payment processing
    sus span trace_span = global_tracer.start_span(ctx.child_context("process_payment"))
    
    fr fr Your payment logic here
    lowkey payment_successful {
        global_tracer.finish_span(span.span_id, enhanced_error{})
        damn ok_result("payment_success")
    } else {
        sus err enhanced_error = new_enhanced_error("Payment failed", ctx)
        global_tracer.finish_span(span.span_id, err)
        damn error_result(err)
    }
}

fr fr Execute with circuit breaker protection
sus result result<any, enhanced_error> = cb.execute_with_trace(ctx, payment_operation)

lowkey result.is_error() {
    sus err enhanced_error = result.unwrap_error()
    lowkey err.error_category == "circuit_breaker" {
        vibez.spill("Circuit breaker is open - using fallback")
        fr fr Implement fallback logic
    }
}
```

### Circuit Breaker Metrics

```cursed
vibez.spill("Circuit Breaker Metrics:")
vibez.spill("  Total Requests: " + string(cb.metrics.total_requests))
vibez.spill("  Successful Requests: " + string(cb.metrics.successful_requests))  
vibez.spill("  Failed Requests: " + string(cb.metrics.failed_requests))
vibez.spill("  Circuit Opens: " + string(cb.metrics.circuit_opens))
vibez.spill("  Success Rate: " + string(cb.success_rate))
vibez.spill("  Average Response Time: " + string(cb.metrics.avg_response_time_ms) + "ms")
```

## Enhanced Error Types

### Creating Enhanced Errors

```cursed
fr fr Create enhanced error with trace context
sus err enhanced_error = new_enhanced_error("Payment processing failed", ctx)

fr fr Set error classification
err.error_category = "payment"
err.is_retriable = based
err.is_timeout = cringe
err.is_transient = based
err.is_business_logic = cringe

fr fr Add metadata
err = err.add_metadata("payment_processor", "stripe")
err = err.add_metadata("attempt_count", 3)
err = err.add_metadata("customer_tier", "premium")

fr fr Add recovery actions
sus retry_action recovery_action = recovery_action{
    action_type: "retry",
    action_data: {
        "delay_ms": 5000,
        "max_attempts": 3,
        "backoff_multiplier": 2.0
    },
    execution_timeout_ms: 30000,
    rollback_required: cringe
}

sus fallback_action recovery_action = recovery_action{
    action_type: "fallback", 
    action_data: {
        "fallback_processor": "paypal",
        "reduced_features": based
    },
    execution_timeout_ms: 15000,
    rollback_required: cringe
}

err = err.add_recovery_action(retry_action)
err = err.add_recovery_action(fallback_action)
```

### Error Propagation

```cursed
fr fr Track error propagation across services
err = err.propagate_to_service("user_service", "get_payment_methods")
err = err.propagate_to_service("order_service", "update_order_status")
err = err.propagate_to_service("notification_service", "send_failure_email")

vibez.spill("Propagation Chain: " + string(len(err.propagation_chain)))
vibez.spill("Affected Services: " + string(len(err.affected_services)))
```

## Error Aggregation and Batch Processing

### Setting Up Error Aggregation

```cursed
fr fr Create error aggregator
sus aggregator error_aggregator = new_error_aggregator(
    100,    fr fr Max 100 errors before flush
    60000   fr fr Flush every 60 seconds
)

fr fr Add errors for batch processing
aggregator.add_error(payment_error)
aggregator.add_error(inventory_error) 
aggregator.add_error(shipping_error)

fr fr Check if flush needed
lowkey aggregator.should_flush() {
    sus batch_result batch_processing_result = aggregator.flush()
    
    vibez.spill("Batch Processing Results:")
    vibez.spill("  Total Errors: " + string(batch_result.total_errors))
    vibez.spill("  Correlation Groups: " + string(batch_result.correlation_groups_processed))
    vibez.spill("  Services Affected: " + string(batch_result.services_affected))
    vibez.spill("  Recovery Actions: " + string(batch_result.recovery_actions_triggered))
    vibez.spill("  Escalations: " + string(batch_result.escalations_created))
}
```

### Error Pattern Detection

```cursed
fr fr Aggregator automatically detects patterns
bestie category, count := range aggregator.category_counts {
    lowkey count > 10 {
        vibez.spill("Critical pattern detected: " + category + " (" + string(count) + " occurrences)")
        fr fr Automatic escalation triggered
    }
}
```

## Distributed Tracing

### Span Management

```cursed
fr fr Initialize distributed tracing
init_distributed_tracing("order_service")

fr fr Start span
sus span trace_span = global_tracer.start_span(ctx)

fr fr Add span logs
span = span.log("info", "Processing order", {
    "order_id": "order_123",
    "customer_id": "customer_456", 
    "order_value": 299.99
})

span = span.log("debug", "Inventory check completed", {
    "items_available": based,
    "reserved_count": 3
})

fr fr Add span tags
span = span.add_tag("service.version", "v2.1.0")
span = span.add_tag("environment", "production")

fr fr Finish span
global_tracer.finish_span(span.span_id, enhanced_error{})
```

### Tracing with Errors

```cursed
fr fr Finish span with error
sus error enhanced_error = new_enhanced_error("Order processing failed", ctx)
global_tracer.finish_span(span.span_id, error)

fr fr Span automatically tagged with error information
fr fr span.tags["error"] = "true"
fr fr span.tags["error.category"] = error.error_category
fr fr span.status = "error"
```

### Span Export

```cursed
fr fr Configure export settings
global_tracer.export_batch_size = 100
global_tracer.export_interval_ms = 10000

fr fr Force export
global_tracer.export_spans()

vibez.spill("Exported spans to tracing backend")
```

## Recovery Actions and Escalation

### Recovery Action Types

```cursed
fr fr Retry action
sus retry_action recovery_action = recovery_action{
    action_type: "retry",
    action_data: {
        "delay_ms": 1000,
        "max_attempts": 3,
        "backoff_multiplier": 2.0,
        "jitter_enabled": based
    },
    execution_timeout_ms: 30000,
    rollback_required: cringe
}

fr fr Circuit breaker reset action  
sus circuit_reset_action recovery_action = recovery_action{
    action_type: "circuit_breaker_reset",
    action_data: {
        "circuit_name": "payment_service",
        "reset_delay_ms": 60000
    },
    execution_timeout_ms: 120000,
    rollback_required: based
}

fr fr Fallback action
sus fallback_action recovery_action = recovery_action{
    action_type: "fallback",
    action_data: {
        "fallback_service": "backup_payment_processor",
        "reduced_functionality": based,
        "fallback_timeout_ms": 15000
    },
    execution_timeout_ms: 30000,
    rollback_required: cringe
}

fr fr Auto-scaling action
sus scaling_action recovery_action = recovery_action{
    action_type: "auto_scale",
    action_data: {
        "target_service": "payment_service",
        "scale_factor": 2,
        "max_instances": 10,
        "scale_down_delay_ms": 600000
    },
    execution_timeout_ms: 300000,
    rollback_required: based
}
```

### Escalation Configuration

```cursed
fr fr Configure escalation
sus escalation escalation_config = escalation_config{
    escalation_threshold: 10,
    escalation_timeout_ms: 300000,  fr fr 5 minutes
    notification_channels: ["slack", "email", "pagerduty"],
    severity_level: "high",
    runbook_url: "https://runbooks.company.com/payment-failures"
}

fr fr Apply to error
err.escalation_config = escalation
```

## Monitoring and Metrics

### Error Metrics

```cursed
fr fr Initialize error monitoring
init_error_monitoring()

fr fr Update metrics for each error
update_error_metrics(error)

fr fr Get global metrics
sus metrics error_metrics = global_error_metrics
vibez.spill("Total Errors: " + string(metrics.total_errors))
vibez.spill("Services Affected: " + string(len(metrics.errors_by_service)))
vibez.spill("Error Categories: " + string(len(metrics.errors_by_category)))
vibez.spill("Recovery Success Rate: " + string(metrics.recovery_success_rate))
vibez.spill("Average Recovery Time: " + string(metrics.avg_recovery_time_ms) + "ms")
```

### Circuit Breaker Monitoring

```cursed
bestie circuit_name, state := range global_error_metrics.circuit_breaker_states {
    vibez.spill("Circuit Breaker " + circuit_name + ": " + state)
}
```

## Production Use Cases

### E-commerce Order Processing

```cursed
fr fr Complete e-commerce flow with error recovery
fr fr See enterprise_demo.csd for full example
demo_ecommerce_order_processing()
```

### Financial Services

```cursed  
fr fr High-frequency trading with zero-tolerance error handling
fr fr Strict SLA requirements and compliance
demo_financial_services_error_handling()
```

### Healthcare Systems

```cursed
fr fr HIPAA-compliant error handling
fr fr Privacy-preserving error tracking
demo_healthcare_error_handling()
```

## Configuration Examples

### Development Environment

```cursed
sus dev_config circuit_config = circuit_config{
    failure_threshold: 10,
    timeout_ms: 60000,
    max_failures: 50,
    retry_after_ms: 30000,
    escalation_threshold: 100,
    tags: {"environment": "development"}
}
```

### Production Environment

```cursed
sus prod_config circuit_config = circuit_config{
    failure_threshold: 5,
    timeout_ms: 30000,
    max_failures: 20,
    retry_after_ms: 120000,
    escalation_threshold: 30,
    tags: {"environment": "production", "criticality": "high"}
}
```

### High-Performance Environment

```cursed
sus hperf_config circuit_config = circuit_config{
    failure_threshold: 2,
    timeout_ms: 5000,
    max_failures: 10, 
    retry_after_ms: 60000,
    escalation_threshold: 15,
    tags: {"environment": "hft", "latency": "ultra_low"}
}
```

## Best Practices

### 1. Correlation ID Management
- Always generate correlation IDs at service boundaries
- Propagate correlation IDs through all downstream calls
- Include user and session context when available
- Use structured logging with correlation IDs

### 2. Circuit Breaker Configuration
- Set appropriate failure thresholds based on SLA requirements
- Configure different thresholds for different service criticality levels
- Monitor success rates and adjust thresholds based on historical data
- Use tags for circuit breaker categorization and monitoring

### 3. Error Classification
- Classify errors as retriable, transient, timeout, or business logic
- Use appropriate error categories for different failure types
- Add rich metadata for debugging and analysis
- Define recovery actions for each error type

### 4. Distributed Tracing
- Create spans for all significant operations
- Include performance metrics in span logs
- Use consistent span naming conventions
- Export spans in batches for efficiency

### 5. Recovery Actions
- Define multiple recovery strategies for critical errors
- Set appropriate timeouts for recovery actions
- Implement rollback procedures for destructive recovery actions
- Monitor recovery action success rates

### 6. Escalation Policies
- Set escalation thresholds based on business impact
- Configure multiple notification channels for redundancy
- Include runbook links in escalation notifications
- Track escalation resolution times

## Integration with Existing Systems

### Logging Integration

```cursed
fr fr Enhanced errors integrate with existing logging
yeet "error_management"

sus err enhanced_error = new_enhanced_error("Service unavailable", ctx)
log_error_enhanced(err, {"additional_context": "from_integration"})
```

### Monitoring Integration

```cursed
fr fr Export metrics to monitoring systems
export_error_metrics_to_prometheus()
export_circuit_breaker_metrics_to_datadog()
export_trace_data_to_jaeger()
```

### Alerting Integration

```cursed
fr fr Configure alerting rules
create_error_rate_alert("payment_service", 0.05)  fr fr 5% error rate threshold
create_circuit_breaker_alert("critical_service")
create_cascade_failure_alert(3)  fr fr 3+ services affected
```

## Performance Considerations

- **Memory Usage**: Error aggregation uses bounded buffers
- **CPU Overhead**: Circuit breaker checks are O(1) operations  
- **Network Impact**: Batch export minimizes tracing overhead
- **Storage**: Configurable retention policies for error history

## Security and Compliance

- **PII Handling**: Automatic scrubbing of sensitive data in errors
- **Audit Trails**: Complete error and recovery action logging
- **Access Control**: Role-based access to error data
- **Compliance**: HIPAA, SOX, GDPR-compliant error handling

## Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/enhanced_error/test_enterprise_recovery.csd
```

Run the production demos:

```bash
./zig-out/bin/cursed-zig stdlib/enhanced_error/enterprise_demo.csd
```

## License

Part of the CURSED programming language standard library. See main project license.
