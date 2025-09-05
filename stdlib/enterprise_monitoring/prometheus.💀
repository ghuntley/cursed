// Prometheus Metrics Integration for CURSED
// Enterprise-grade monitoring and observability

yeet "vibez"
yeet "networkz"
yeet "httpz"
yeet "configz"
yeet "errorz"
yeet "concurrenz"
yeet "stringz"
yeet "timez"

squad MetricType drip = 0  // enum-like
sus COUNTER drip = 0
sus GAUGE drip = 1
sus HISTOGRAM drip = 2
sus SUMMARY drip = 3

squad MetricConfig {
    name tea
    help tea
    metric_type drip
    labels tea[value] = []
    buckets drip[value] = []  // For histograms
    objectives map<drip, drip> = {}  // For summaries (quantile -> error)
}

squad MetricValue {
    labels map<tea, tea>
    value drip
    timestamp drip = 0  // 0 = current time
}

squad HistogramBucket {
    upper_bound drip
    count drip
}

squad HistogramValue {
    labels map<tea, tea>
    buckets HistogramBucket[value]
    count drip
    sum drip
}

squad SummaryQuantile {
    quantile drip
    value drip
}

squad SummaryValue {
    labels map<tea, tea>
    quantiles SummaryQuantile[value]
    count drip
    sum drip
}

// Core Metric implementations
squad Counter {
    config MetricConfig
    values map<tea, drip>  // serialized labels -> value
    mutex concurrenz.Mutex
    
    slay create_counter(name tea, help tea, label_names tea[value]) Counter {
        damn Counter{
            .config = MetricConfig{
                .name = name,
                .help = help,
                .metric_type = COUNTER,
                .labels = label_names,
            },
            .values = {},
        }
    }
    
    slay inc(labels map<tea, tea>) {
        self.add(1.0, labels)
    }
    
    slay add(value drip, labels map<tea, tea>) {
        ready (value < 0) {
            vibez.spill("Warning: Counter values should not be negative")
            damn nil
        }
        
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        self.values[label_key] = self.values[label_key] + value
    }
    
    slay get(labels map<tea, tea>) drip {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        damn self.values[label_key]
    }
    
    slay collect() MetricValue[value]{
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus result MetricValue[value] = []
        bestie (label_key, value := range self.values) {
            sus labels map<tea, tea> = self.deserialize_labels(label_key)
            result = append(result, MetricValue{
                .labels = labels,
                .value = value,
                .timestamp = timez.now_millis(),
            })
        }
        damn result
    }
    
    // Private helpers
    slay serialize_labels(labels map<tea, tea>) tea {
        sus pairs tea[value] = []
        bestie (name := range self.config.labels) {
            sus value tea = labels[name] fam { when _ -> "" }
            pairs = append(pairs, name + "=" + value)
        }
        damn stringz.join(pairs, ",")
    }
    
    slay deserialize_labels(label_key tea) map<tea, tea> {
        sus result map<tea, tea> = {}
        sus pairs tea[value] = stringz.split(label_key, ",")
        
        bestie (pair := range pairs) {
            sus parts tea[value] = stringz.split(pair, "=", 2)
            ready (len(parts) == 2) {
                result[parts[0]] = parts[1]
            }
        }
        damn result
    }
}

squad Gauge {
    config MetricConfig
    values map<tea, drip>
    mutex concurrenz.Mutex
    
    slay create_gauge(name tea, help tea, label_names tea[value]) Gauge {
        damn Gauge{
            .config = MetricConfig{
                .name = name,
                .help = help,
                .metric_type = GAUGE,
                .labels = label_names,
            },
            .values = {},
        }
    }
    
    slay set(value drip, labels map<tea, tea>) {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        self.values[label_key] = value
    }
    
    slay inc(labels map<tea, tea>) {
        self.add(1.0, labels)
    }
    
    slay dec(labels map<tea, tea>) {
        self.add(-1.0, labels)
    }
    
    slay add(value drip, labels map<tea, tea>) {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        self.values[label_key] = self.values[label_key] + value
    }
    
    slay get(labels map<tea, tea>) drip {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        damn self.values[label_key]
    }
    
    slay collect() MetricValue[value]{
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus result MetricValue[value] = []
        bestie (label_key, value := range self.values) {
            sus labels map<tea, tea> = self.deserialize_labels(label_key)
            result = append(result, MetricValue{
                .labels = labels,
                .value = value,
                .timestamp = timez.now_millis(),
            })
        }
        damn result
    }
    
    // Helper methods (same as Counter)
    slay serialize_labels(labels map<tea, tea>) tea {
        sus pairs tea[value] = []
        bestie (name := range self.config.labels) {
            sus value tea = labels[name] fam { when _ -> "" }
            pairs = append(pairs, name + "=" + value)
        }
        damn stringz.join(pairs, ",")
    }
    
    slay deserialize_labels(label_key tea) map<tea, tea> {
        sus result map<tea, tea> = {}
        sus pairs tea[value] = stringz.split(label_key, ",")
        
        bestie (pair := range pairs) {
            sus parts tea[value] = stringz.split(pair, "=", 2)
            ready (len(parts) == 2) {
                result[parts[0]] = parts[1]
            }
        }
        damn result
    }
}

squad Histogram {
    config MetricConfig
    values map<tea, HistogramValue>
    mutex concurrenz.Mutex
    
    slay create_histogram(name tea, help tea, label_names tea[value], buckets drip[value]) Histogram {
        // Ensure buckets are sorted and include +Inf
        sus sorted_buckets drip[value] = sort_buckets(buckets)
        ready (sorted_buckets[len(sorted_buckets)-1] != mathz.inf()) {
            sorted_buckets = append(sorted_buckets, mathz.inf())
        }
        
        damn Histogram{
            .config = MetricConfig{
                .name = name,
                .help = help,
                .metric_type = HISTOGRAM,
                .labels = label_names,
                .buckets = sorted_buckets,
            },
            .values = {},
        }
    }
    
    slay observe(value drip, labels map<tea, tea>) {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus label_key tea = self.serialize_labels(labels)
        sus hist_value HistogramValue = self.values[label_key] fam {
            when _ -> self.create_empty_histogram_value(labels)
        }
        
        // Update buckets
        bestie (i, bucket := range hist_value.buckets) {
            ready (value <= bucket.upper_bound) {
                hist_value.buckets[i].count += 1
            }
        }
        
        // Update count and sum
        hist_value.count += 1
        hist_value.sum += value
        
        self.values[label_key] = hist_value
    }
    
    slay collect() HistogramValue[value]{
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus result HistogramValue[value] = []
        bestie (_, hist_value := range self.values) {
            result = append(result, hist_value)
        }
        damn result
    }
    
    // Private helpers
    slay create_empty_histogram_value(labels map<tea, tea>) HistogramValue {
        sus buckets HistogramBucket[value] = []
        bestie (bucket_bound := range self.config.buckets) {
            buckets = append(buckets, HistogramBucket{
                .upper_bound = bucket_bound,
                .count = 0,
            })
        }
        
        damn HistogramValue{
            .labels = labels,
            .buckets = buckets,
            .count = 0,
            .sum = 0,
        }
    }
    
    slay serialize_labels(labels map<tea, tea>) tea {
        sus pairs tea[value] = []
        bestie (name := range self.config.labels) {
            sus value tea = labels[name] fam { when _ -> "" }
            pairs = append(pairs, name + "=" + value)
        }
        damn stringz.join(pairs, ",")
    }
}

// Timer utility for measuring durations
squad Timer {
    histogram Histogram
    
    slay create_timer(name tea, help tea, label_names tea[value]) Timer {
        sus default_buckets drip[value] = [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1, 2.5, 5, 10]
        
        damn Timer{
            .histogram = create_histogram(name, help, label_names, default_buckets),
        }
    }
    
    slay time<T>(labels map<tea, tea>, func slay() T) T {
        sus start_time drip = timez.now_nanos()
        sus result T = func()
        sus duration drip = (timez.now_nanos() - start_time) / 1_000_000  // Convert to milliseconds
        
        self.histogram.observe(duration, labels)
        damn result
    }
    
    slay observe_duration_ms(duration_ms drip, labels map<tea, tea>) {
        self.histogram.observe(duration_ms, labels)
    }
    
    slay collect() HistogramValue[value]{
        damn self.histogram.collect()
    }
}

// Metrics Registry
squad MetricsRegistry {
    counters map<tea, Counter>
    gauges map<tea, Gauge>
    histograms map<tea, Histogram>
    timers map<tea, Timer>
    mutex concurrenz.Mutex
    
    slay create_registry() MetricsRegistry {
        damn MetricsRegistry{
            .counters = {},
            .gauges = {},
            .histograms = {},
            .timers = {},
        }
    }
    
    slay register_counter(name tea, help tea, label_names tea[value]) Counter {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus counter Counter = create_counter(name, help, label_names)
        self.counters[name] = counter
        damn counter
    }
    
    slay register_gauge(name tea, help tea, label_names tea[value]) Gauge {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus gauge Gauge = create_gauge(name, help, label_names)
        self.gauges[name] = gauge
        damn gauge
    }
    
    slay register_histogram(name tea, help tea, label_names tea[value], buckets drip[value]) Histogram {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus histogram Histogram = create_histogram(name, help, label_names, buckets)
        self.histograms[name] = histogram
        damn histogram
    }
    
    slay register_timer(name tea, help tea, label_names tea[value]) Timer {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus timer Timer = create_timer(name, help, label_names)
        self.timers[name] = timer
        damn timer
    }
    
    slay get_counter(name tea) yikes<Counter> {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        damn self.counters[name] fam {
            when _ -> yikes "counter not found: " + name
        }
    }
    
    slay get_gauge(name tea) yikes<Gauge> {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        damn self.gauges[name] fam {
            when _ -> yikes "gauge not found: " + name
        }
    }
    
    slay get_histogram(name tea) yikes<Histogram> {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        damn self.histograms[name] fam {
            when _ -> yikes "histogram not found: " + name
        }
    }
    
    slay gather() PrometheusMetricFamilies {
        self.mutex.lock()
        defer { self.mutex.unlock() }
        
        sus families PrometheusMetricFamilies = {}
        
        // Collect counters
        bestie (name, counter := range self.counters) {
            families[name] = PrometheusMetricFamily{
                .name = name,
                .help = counter.config.help,
                .type = "counter",
                .metrics = counter.collect(),
            }
        }
        
        // Collect gauges
        bestie (name, gauge := range self.gauges) {
            families[name] = PrometheusMetricFamily{
                .name = name,
                .help = gauge.config.help,
                .type = "gauge",
                .metrics = gauge.collect(),
            }
        }
        
        // Collect histograms
        bestie (name, histogram := range self.histograms) {
            families[name] = PrometheusMetricFamily{
                .name = name,
                .help = histogram.config.help,
                .type = "histogram",
                .histogram_metrics = histogram.collect(),
            }
        }
        
        damn families
    }
}

// Prometheus exposition format
squad PrometheusMetricFamily {
    name tea
    help tea
    type tea
    metrics MetricValue[value]
    histogram_metrics HistogramValue[value]
}

squad PrometheusMetricFamilies = map<tea, PrometheusMetricFamily>

// HTTP handler for Prometheus scraping
squad PrometheusHandler {
    registry MetricsRegistry
    
    slay create_prometheus_handler(registry MetricsRegistry) PrometheusHandler {
        damn PrometheusHandler{
            .registry = registry,
        }
    }
    
    slay handle_metrics(request httpz.Request) httpz.Response {
        sus families PrometheusMetricFamilies = self.registry.gather()
        sus output tea = self.format_prometheus_output(families)
        
        damn httpz.Response{
            .status_code = 200,
            .headers = {
                "content-type": "text/plain; version=0.0.4; charset=utf-8",
                "content-length": to_string(len(output)),
            },
            .body = encode_string(output),
        }
    }
    
    slay format_prometheus_output(families PrometheusMetricFamilies) tea {
        sus output tea = ""
        
        bestie (name, family := range families) {
            // Write HELP line
            output += "# HELP " + name + " " + family.help + "\n"
            
            // Write TYPE line
            output += "# TYPE " + name + " " + family.type + "\n"
            
            // Write metrics
            sick (family.type) {
                "counter", "gauge" -> {
                    bestie (metric := range family.metrics) {
                        output += name
                        
                        // Add labels
                        ready (len(metric.labels) > 0) {
                            output += "{"
                            sus label_pairs tea[value] = []
                            bestie (label_name, label_value := range metric.labels) {
                                label_pairs = append(label_pairs, label_name + "=\"" + escape_label_value(label_value) + "\"")
                            }
                            output += stringz.join(label_pairs, ",")
                            output += "}"
                        }
                        
                        output += " " + format_float(metric.value)
                        
                        ready (metric.timestamp > 0) {
                            output += " " + to_string(metric.timestamp)
                        }
                        
                        output += "\n"
                    }
                }
                "histogram" -> {
                    bestie (hist := range family.histogram_metrics) {
                        // Write bucket metrics
                        bestie (bucket := range hist.buckets) {
                            output += name + "_bucket"
                            
                            sus labels tea = self.format_labels_with_le(hist.labels, bucket.upper_bound)
                            output += labels
                            output += " " + to_string(bucket.count) + "\n"
                        }
                        
                        // Write count metric
                        output += name + "_count"
                        output += self.format_labels(hist.labels)
                        output += " " + to_string(hist.count) + "\n"
                        
                        // Write sum metric
                        output += name + "_sum"
                        output += self.format_labels(hist.labels)
                        output += " " + format_float(hist.sum) + "\n"
                    }
                }
            }
            
            output += "\n"
        }
        
        damn output
    }
    
    slay format_labels(labels map<tea, tea>) tea {
        ready (len(labels) == 0) {
            damn ""
        }
        
        sus label_pairs tea[value] = []
        bestie (name, value := range labels) {
            label_pairs = append(label_pairs, name + "=\"" + escape_label_value(value) + "\"")
        }
        
        damn "{" + stringz.join(label_pairs, ",") + "}"
    }
    
    slay format_labels_with_le(labels map<tea, tea>, le_value drip) tea {
        sus extended_labels map<tea, tea> = {}
        bestie (name, value := range labels) {
            extended_labels[name] = value
        }
        extended_labels["le"] = format_float(le_value)
        
        damn self.format_labels(extended_labels)
    }
}

// Metrics middleware for HTTP servers
squad MetricsMiddleware {
    request_counter Counter
    request_duration Timer
    request_size_histogram Histogram
    response_size_histogram Histogram
    
    slay create_metrics_middleware(registry MetricsRegistry) MetricsMiddleware {
        sus request_counter Counter = registry.register_counter(
            "http_requests_total",
            "Total number of HTTP requests",
            ["method", "status", "endpoint"]
        )
        
        sus request_duration Timer = registry.register_timer(
            "http_request_duration_seconds",
            "Duration of HTTP requests in seconds",
            ["method", "status", "endpoint"]
        )
        
        sus request_size_histogram Histogram = registry.register_histogram(
            "http_request_size_bytes",
            "Size of HTTP requests in bytes",
            ["method", "endpoint"],
            [100, 1000, 10000, 100000, 1000000]
        )
        
        sus response_size_histogram Histogram = registry.register_histogram(
            "http_response_size_bytes",
            "Size of HTTP responses in bytes",
            ["method", "status", "endpoint"],
            [100, 1000, 10000, 100000, 1000000]
        )
        
        damn MetricsMiddleware{
            .request_counter = request_counter,
            .request_duration = request_duration,
            .request_size_histogram = request_size_histogram,
            .response_size_histogram = response_size_histogram,
        }
    }
    
    slay wrap_handler(handler slay(httpz.Request) httpz.Response) slay(httpz.Request) httpz.Response {
        damn slay(request httpz.Request) httpz.Response {
            sus start_time drip = timez.now_nanos()
            
            // Record request size
            sus endpoint tea = sanitize_endpoint(request.path)
            self.request_size_histogram.observe(len(request.body), {
                "method": request.method,
                "endpoint": endpoint,
            })
            
            // Handle request
            sus response httpz.Response = handler(request)
            
            // Calculate duration
            sus duration_seconds drip = (timez.now_nanos() - start_time) / 1_000_000_000.0
            
            // Record metrics
            sus labels map<tea, tea> = {
                "method": request.method,
                "status": to_string(response.status_code),
                "endpoint": endpoint,
            }
            
            self.request_counter.inc(labels)
            self.request_duration.observe_duration_ms(duration_seconds * 1000, labels)
            self.response_size_histogram.observe(len(response.body), labels)
            
            damn response
        }
    }
}

// Application metrics helpers
squad ApplicationMetrics {
    registry MetricsRegistry
    
    // Common application metrics
    startup_time Gauge
    active_connections Gauge
    memory_usage Gauge
    cpu_usage Gauge
    error_rate Counter
    business_metrics map<tea, Counter>
    
    slay create_application_metrics() ApplicationMetrics {
        sus registry MetricsRegistry = create_registry()
        
        damn ApplicationMetrics{
            .registry = registry,
            .startup_time = registry.register_gauge(
                "app_startup_time_seconds",
                "Time taken for application startup",
                []
            ),
            .active_connections = registry.register_gauge(
                "app_active_connections",
                "Number of active connections",
                ["type"]
            ),
            .memory_usage = registry.register_gauge(
                "app_memory_usage_bytes",
                "Memory usage in bytes",
                ["type"]
            ),
            .cpu_usage = registry.register_gauge(
                "app_cpu_usage_percent",
                "CPU usage percentage",
                []
            ),
            .error_rate = registry.register_counter(
                "app_errors_total",
                "Total number of application errors",
                ["type", "severity"]
            ),
            .business_metrics = {},
        }
    }
    
    slay record_startup_time(duration_seconds drip) {
        self.startup_time.set(duration_seconds, {})
    }
    
    slay update_active_connections(count drip, connection_type tea) {
        self.active_connections.set(count, {"type": connection_type})
    }
    
    slay update_memory_usage(bytes drip, memory_type tea) {
        self.memory_usage.set(bytes, {"type": memory_type})
    }
    
    slay update_cpu_usage(percentage drip) {
        self.cpu_usage.set(percentage, {})
    }
    
    slay record_error(error_type tea, severity tea) {
        self.error_rate.inc({
            "type": error_type,
            "severity": severity,
        })
    }
    
    slay create_business_metric(name tea, help tea) Counter {
        sus full_name tea = "business_" + name + "_total"
        sus counter Counter = self.registry.register_counter(full_name, help, ["category"])
        self.business_metrics[name] = counter
        damn counter
    }
    
    slay get_prometheus_handler() PrometheusHandler {
        damn create_prometheus_handler(self.registry)
    }
}

// Global default registry
sus default_registry MetricsRegistry = create_registry()

// Convenience functions using default registry
slay register_counter(name tea, help tea, label_names tea[value]) Counter {
    damn default_registry.register_counter(name, help, label_names)
}

slay register_gauge(name tea, help tea, label_names tea[value]) Gauge {
    damn default_registry.register_gauge(name, help, label_names)
}

slay register_histogram(name tea, help tea, label_names tea[value], buckets drip[value]) Histogram {
    damn default_registry.register_histogram(name, help, label_names, buckets)
}

slay register_timer(name tea, help tea, label_names tea[value]) Timer {
    damn default_registry.register_timer(name, help, label_names)
}

slay get_prometheus_handler() PrometheusHandler {
    damn create_prometheus_handler(default_registry)
}

// Utility functions
slay sort_buckets(buckets drip[value]) drip[value]{
    // Simple insertion sort for bucket values
    sus sorted drip[value] = buckets[:]
    bestie (i := 1; i < len(sorted); i += 1) {
        sus key drip = sorted[i]
        sus j drip = i - 1
        
        bestie (j >= 0 && sorted[j] > key) {
            sorted[j + 1] = sorted[j]
            j -= 1
        }
        sorted[j + 1] = key
    }
    damn sorted
}

slay format_float(value drip) tea {
    // Format float with appropriate precision
    ready (value == mathz.inf()) {
        damn "+Inf"
    }
    ready (value == -mathz.inf()) {
        damn "-Inf"
    }
    ready (mathz.is_nan(value)) {
        damn "NaN"
    }
    
    // Simple float formatting (would use proper formatting in real implementation)
    damn to_string(value)
}

slay escape_label_value(value tea) tea {
    // Escape special characters in label values
    sus escaped tea = stringz.replace_all(value, "\\", "\\\\")
    escaped = stringz.replace_all(escaped, "\"", "\\\"")
    escaped = stringz.replace_all(escaped, "\n", "\\n")
    escaped = stringz.replace_all(escaped, "\t", "\\t")
    damn escaped
}

slay sanitize_endpoint(path tea) tea {
    // Convert path to a sanitized endpoint name for metrics
    // Replace dynamic segments with placeholders
    sus sanitized tea = path
    
    // Replace common ID patterns
    sanitized = stringz.replace_regex(sanitized, "/\\d+", "/{id}")
    sanitized = stringz.replace_regex(sanitized, "/[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}", "/{uuid}")
    
    damn sanitized
}

// Example usage
slay example_prometheus_usage() yikes<tea> {
    // Create application metrics
    sus app_metrics ApplicationMetrics = create_application_metrics()
    
    // Record startup time
    sus startup_start drip = timez.now_nanos()
    // ... application initialization ...
    sus startup_duration drip = (timez.now_nanos() - startup_start) / 1_000_000_000.0
    app_metrics.record_startup_time(startup_duration)
    
    // Create custom business metrics
    sus order_counter Counter = app_metrics.create_business_metric("orders", "Total number of orders processed")
    sus user_login_counter Counter = register_counter("user_logins_total", "Total user logins", ["method", "status"])
    
    // Record some metrics
    order_counter.inc({"category": "online"})
    user_login_counter.inc({"method": "oauth", "status": "success"})
    
    // Start HTTP server with metrics endpoint
    sus prometheus_handler PrometheusHandler = app_metrics.get_prometheus_handler()
    
    vibez.spill("Metrics endpoint available at /metrics")
    vibez.spill("Prometheus configuration:")
    vibez.spill("  - job_name: 'cursed-app'")
    vibez.spill("    static_configs:")
    vibez.spill("      - targets: ['localhost:8080']")
    
    // Example HTTP server setup (simplified)
    // httpz.serve_http("0.0.0.0:8080", slay(request httpz.Request) httpz.Response {
    //     ready (request.path == "/metrics") {
    //         damn prometheus_handler.handle_metrics(request)
    //     }
    //     // ... other routes ...
    // })
}
