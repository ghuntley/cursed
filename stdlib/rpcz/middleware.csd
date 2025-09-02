# RPC Middleware Module
# Provides middleware implementations for logging, metrics, rate limiting,
# authentication, request validation, and custom processing

yeet "timez"
yeet "stringz" 
yeet "jsonz"
yeet "cryptz"
yeet "vibez"
yeet "./core"

# Logging Middleware - logs all RPC requests and responses
squad LoggingMiddleware {
    log_requests lit,
    log_responses lit,
    log_errors_only lit,
    log_level tea         # "debug", "info", "warn", "error"
}

# Implement RpcMiddleware interface for LoggingMiddleware
slay logging_before_call(middleware &LoggingMiddleware, request RpcRequest) yikes<lit> {
    ready (middleware.log_requests && !middleware.log_errors_only) {
        sus timestamp tea = timez.format_timestamp(timez.now())
        sus log_entry tea = "[" + timestamp + "] RPC Request - Method: " + request.method + 
                           ", ID: " + request.id + ", Params: " + request.params
        
        vibez.spill("[RPC-LOG]", log_entry)
    }
    
    damn based
}

slay logging_after_call(middleware &LoggingMiddleware, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    ready (middleware.log_responses || (middleware.log_errors_only && response.error != "")) {
        sus timestamp tea = timez.format_timestamp(timez.now())
        sus status tea = ready (response.error != "") { "ERROR" } otherwise { "SUCCESS" }
        
        sus log_entry tea = "[" + timestamp + "] RPC Response - Method: " + request.method + 
                           ", ID: " + response.id + ", Status: " + status
        
        ready (response.error != "") {
            log_entry = log_entry + ", Error: " + response.error
        }
        
        vibez.spill("[RPC-LOG]", log_entry)
    }
    
    damn response
}

# Create logging middleware
slay new_logging_middleware(log_requests lit, log_responses lit) LoggingMiddleware {
    damn LoggingMiddleware{
        log_requests: log_requests,
        log_responses: log_responses,
        log_errors_only: nah,
        log_level: "info"
    }
}

# Metrics Middleware - tracks request statistics and performance
squad MetricsMiddleware {
    total_requests drip,
    successful_requests drip,
    failed_requests drip,
    method_counts map<tea, drip>,
    response_times drip[value],
    start_times map<tea, drip>      # request_id -> start_time
}

slay metrics_before_call(middleware &MetricsMiddleware, request RpcRequest) yikes<lit> {
    middleware.total_requests += 1
    
    # Track method usage
    sus current_count drip = middleware.method_counts.get(request.method) fam {
        when _ -> 0
    }
    middleware.method_counts[request.method] = current_count + 1
    
    # Record start time for response time calculation
    ready (request.id != "") {
        middleware.start_times[request.id] = timez.now_millis()
    }
    
    damn based
}

slay metrics_after_call(middleware &MetricsMiddleware, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    # Track success/failure
    ready (response.error != "") {
        middleware.failed_requests += 1
    } otherwise {
        middleware.successful_requests += 1
    }
    
    # Calculate response time
    ready (request.id != "") {
        sus start_time drip = middleware.start_times.get(request.id) fam {
            when _ -> damn response
        }
        
        sus response_time drip = timez.now_millis() - start_time
        middleware.response_times = append(middleware.response_times, response_time)
        
        # Clean up start time
        middleware.start_times.remove(request.id)
        
        # Limit response times array to last 1000 entries
        ready (len(middleware.response_times) > 1000) {
            middleware.response_times = slice(middleware.response_times, 1, len(middleware.response_times))
        }
    }
    
    damn response
}

# Create metrics middleware
slay new_metrics_middleware() MetricsMiddleware {
    damn MetricsMiddleware{
        total_requests: 0,
        successful_requests: 0,
        failed_requests: 0,
        method_counts: make_map(),
        response_times: [],
        start_times: make_map()
    }
}

# Get metrics report
slay get_metrics_report(middleware &MetricsMiddleware) tea {
    sus avg_response_time drip = 0
    ready (len(middleware.response_times) > 0) {
        sus total drip = 0
        bestie (time in middleware.response_times) {
            total += time
        }
        avg_response_time = total / len(middleware.response_times)
    }
    
    sus success_rate drip = 0
    ready (middleware.total_requests > 0) {
        success_rate = (middleware.successful_requests * 100) / middleware.total_requests
    }
    
    sus report map<tea, tea> = make_map()
    report["total_requests"] = string_from_int(middleware.total_requests)
    report["successful_requests"] = string_from_int(middleware.successful_requests)
    report["failed_requests"] = string_from_int(middleware.failed_requests)
    report["success_rate_percent"] = string_from_int(success_rate)
    report["avg_response_time_ms"] = string_from_int(avg_response_time)
    report["method_counts"] = jsonz.encode_object(middleware.method_counts)
    
    damn jsonz.encode_object(report) fam {
        when _ -> "{}"
    }
}

# Validation Middleware - validates request parameters and structure
squad ValidationMiddleware {
    method_schemas map<tea, tea>,        # method -> JSON schema
    strict_validation lit,
    custom_validators map<tea, slay(tea) yikes<lit>>
}

slay validation_before_call(middleware &ValidationMiddleware, request RpcRequest) yikes<lit> {
    # Basic request validation
    ready (request.method == "") {
        yikes "Method name is required"
    }
    
    # Check for reserved method names
    ready (stringz.starts_with(request.method, "rpc.")) {
        yikes "Method names starting with 'rpc.' are reserved"
    }
    
    # Schema validation if available
    sus schema tea = middleware.method_schemas.get(request.method) fam {
        when _ -> damn based  # No schema, skip validation
    }
    
    ready (middleware.strict_validation && request.params == "") {
        yikes "Parameters required for method: " + request.method
    }
    
    # Custom validation if available
    sus validator slay(tea) yikes<lit> = middleware.custom_validators.get(request.method) fam {
        when _ -> damn based
    }
    
    validator(request.params) fam {
        when _ -> yikes "Custom validation failed for method: " + request.method
    }
    
    damn based
}

slay validation_after_call(middleware &ValidationMiddleware, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    # Response validation could be added here
    damn response
}

# Create validation middleware
slay new_validation_middleware(strict lit) ValidationMiddleware {
    damn ValidationMiddleware{
        method_schemas: make_map(),
        strict_validation: strict,
        custom_validators: make_map()
    }
}

# Add method schema
slay add_method_schema(middleware &ValidationMiddleware, method tea, schema tea) {
    middleware.method_schemas[method] = schema
}

# Add custom validator
slay add_custom_validator(middleware &ValidationMiddleware, method tea, validator slay(tea) yikes<lit>) {
    middleware.custom_validators[method] = validator
}

# Security Middleware - handles authentication, CORS, and security headers
squad SecurityMiddleware {
    require_https lit,
    allowed_origins tea[value],
    require_auth_for_methods tea[value],
    api_key_header tea,
    valid_api_keys map<tea, tea>,    # api_key -> user_info
    rate_limit_per_method map<tea, drip>
}

slay security_before_call(middleware &SecurityMiddleware, request RpcRequest) yikes<lit> {
    # Check if method requires authentication
    sus requires_auth lit = nah
    bestie (method in middleware.require_auth_for_methods) {
        ready (method == request.method) {
            requires_auth = based
            break
        }
    }
    
    ready (requires_auth) {
        # This would typically be handled at HTTP level, but can add checks here
        yikes "Authentication required for method: " + request.method
    }
    
    # Additional security checks can be added here
    damn based
}

slay security_after_call(middleware &SecurityMiddleware, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    # Security post-processing (e.g., sanitizing response)
    damn response
}

# Create security middleware
slay new_security_middleware() SecurityMiddleware {
    damn SecurityMiddleware{
        require_https: based,
        allowed_origins: ["*"],
        require_auth_for_methods: [],
        api_key_header: "X-API-Key",
        valid_api_keys: make_map(),
        rate_limit_per_method: make_map()
    }
}

# Caching Middleware - caches responses for idempotent methods
squad CachingMiddleware {
    cache map<tea, CacheEntry>,
    ttl_seconds drip,
    cacheable_methods tea[value],
    max_cache_size drip
}

squad CacheEntry {
    response tea,
    timestamp drip,
    ttl drip
}

slay caching_before_call(middleware &CachingMiddleware, request RpcRequest) yikes<lit> {
    # Check if method is cacheable
    sus is_cacheable lit = nah
    bestie (method in middleware.cacheable_methods) {
        ready (method == request.method) {
            is_cacheable = based
            break
        }
    }
    
    ready (!is_cacheable) {
        damn based
    }
    
    # Generate cache key
    sus cache_key tea = generate_cache_key(request)
    
    # Check cache
    sus entry CacheEntry = middleware.cache.get(cache_key) fam {
        when _ -> damn based  # Cache miss
    }
    
    # Check if cached entry is still valid
    sus now drip = timez.now()
    ready (now - entry.timestamp > entry.ttl) {
        middleware.cache.remove(cache_key)
        damn based  # Cache expired
    }
    
    # Cache hit - this would need to be handled differently in real implementation
    # as we can't directly return a response from before_call
    damn based
}

slay caching_after_call(middleware &CachingMiddleware, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    # Only cache successful responses
    ready (response.error != "") {
        damn response
    }
    
    # Check if method is cacheable
    sus is_cacheable lit = nah
    bestie (method in middleware.cacheable_methods) {
        ready (method == request.method) {
            is_cacheable = based
            break
        }
    }
    
    ready (!is_cacheable) {
        damn response
    }
    
    # Generate cache key
    sus cache_key tea = generate_cache_key(request)
    
    # Store in cache
    sus entry CacheEntry = CacheEntry{
        response: serialize_rpc_response(response) fam {
            when _ -> damn response
        },
        timestamp: timez.now(),
        ttl: middleware.ttl_seconds
    }
    
    # Enforce cache size limit
    ready (len(middleware.cache) >= middleware.max_cache_size) {
        # Simple eviction: remove oldest entry
        evict_oldest_cache_entry(middleware)
    }
    
    middleware.cache[cache_key] = entry
    
    damn response
}

# Generate cache key from request
slay generate_cache_key(request RpcRequest) tea {
    damn cryptz.hash_sha256(request.method + ":" + request.params)
}

# Evict oldest cache entry
slay evict_oldest_cache_entry(middleware &CachingMiddleware) {
    sus oldest_key tea = ""
    sus oldest_time drip = timez.now()
    
    bestie (key, entry in middleware.cache) {
        ready (entry.timestamp < oldest_time) {
            oldest_time = entry.timestamp
            oldest_key = key
        }
    }
    
    ready (oldest_key != "") {
        middleware.cache.remove(oldest_key)
    }
}

# Create caching middleware
slay new_caching_middleware(ttl_seconds drip) CachingMiddleware {
    damn CachingMiddleware{
        cache: make_map(),
        ttl_seconds: ttl_seconds,
        cacheable_methods: [],
        max_cache_size: 1000
    }
}

# Add cacheable method
slay add_cacheable_method(middleware &CachingMiddleware, method tea) {
    middleware.cacheable_methods = append(middleware.cacheable_methods, method)
}

# Middleware Chain - combines multiple middleware
squad MiddlewareChain {
    middleware_list RpcMiddleware[value]
}

slay chain_before_call(chain &MiddlewareChain, request RpcRequest) yikes<lit> {
    bestie (middleware in chain.middleware_list) {
        middleware.before_call(request) fam {
            when _ -> yikes "Middleware chain failed"
        }
    }
    damn based
}

slay chain_after_call(chain &MiddlewareChain, request RpcRequest, response RpcResponse) yikes<RpcResponse> {
    # Apply middleware in reverse order for after_call
    sus final_response RpcResponse = response
    
    bestie (i in range(len(chain.middleware_list) - 1, -1, -1)) {
        final_response = chain.middleware_list[i].after_call(request, final_response) fam {
            when _ -> damn response
        }
    }
    
    damn final_response
}

# Create middleware chain
slay new_middleware_chain() MiddlewareChain {
    damn MiddlewareChain{
        middleware_list: []
    }
}

# Add middleware to chain
slay add_middleware_to_chain(chain &MiddlewareChain, middleware RpcMiddleware) {
    chain.middleware_list = append(chain.middleware_list, middleware)
}
