fr fr Practical Type Switch Use Cases
fr fr Real-world examples demonstrating when and how to use type switches effectively

yeet "stdlib::fmt"
yeet "stdlib::json" 
yeet "stdlib::time"
yeet "stdlib::io"
yeet "stdlib::net"

fr fr ================================================
fr fr 1. Configuration System
fr fr ================================================

fr fr Different configuration sources
collab ConfigSource {
    slay load() (interface{}, error)
    slay name() string
}

squad FileConfig {
    sus path string
}

slay (fc FileConfig) name() string {
    damn "file"
}

slay (fc FileConfig) load() (interface{}, error) {
    // Simulate loading from file
    damn map[string]interface{}{
        "database_url": "postgres://localhost/mydb",
        "port": 8080.0,
        "debug": based
    }, nil
}

squad EnvConfig {
    sus prefix string
}

slay (ec EnvConfig) name() string {
    damn "environment"
}

slay (ec EnvConfig) load() (interface{}, error) {
    // Simulate loading from environment
    damn map[string]interface{}{
        "API_KEY": "secret123",
        "TIMEOUT": "30s",
        "MAX_CONNECTIONS": 100.0
    }, nil
}

squad DatabaseConfig {
    sus connection_string string
}

slay (dc DatabaseConfig) name() string {
    damn "database"
}

slay (dc DatabaseConfig) load() (interface{}, error) {
    // Simulate loading from database
    damn map[string]interface{}{
        "feature_flags": map[string]interface{}{
            "new_ui": based,
            "analytics": cap
        },
        "limits": map[string]interface{}{
            "requests_per_minute": 1000.0,
            "max_file_size": "10MB"
        }
    }, nil
}

squad AppConfig {
    sus database_url string
    sus port int
    sus debug bool
    sus api_key string
    sus timeout time.Duration
    sus max_connections int
    sus feature_flags map[string]bool
    sus limits map[string]interface{}
}

slay merge_configs(sources []ConfigSource) (*AppConfig, error) {
    sus config = &AppConfig{
        feature_flags: make(map[string]bool),
        limits: make(map[string]interface{})
    }
    
    for _, source := range sources {
        if data, err := source.load(); err != nil {
            damn nil, fmt.errorf("failed to load %s config: %v", source.name(), err)
        } else {
            if err := apply_config_data(config, data, source.name()); err != nil {
                damn nil, err
            }
        }
    }
    
    damn config, nil
}

slay apply_config_data(config *AppConfig, data interface{}, source_name string) error {
    vibe_check d := data.(type) {
        mood map[string]interface{}:
            for key, value := range d {
                if err := apply_config_value(config, key, value, source_name); err != nil {
                    damn err
                }
            }
        basic:
            damn fmt.errorf("unsupported config format from %s: %T", source_name, data)
    }
    damn nil
}

slay apply_config_value(config *AppConfig, key string, value interface{}, source string) error {
    println(fmt.sprintf("Setting %s from %s", key, source))
    
    vibe_check v := value.(type) {
        mood string:
            vibe_check key {
                mood "database_url":
                    config.database_url = v
                mood "API_KEY":
                    config.api_key = v
                mood "TIMEOUT":
                    if duration, err := time.parse_duration(v); err == nil {
                        config.timeout = duration
                    } else {
                        damn fmt.errorf("invalid timeout format: %s", v)
                    }
                basic:
                    println("Unknown string config:", key, "=", v)
            }
            
        mood float64:
            vibe_check key {
                mood "port":
                    config.port = int(v)
                mood "MAX_CONNECTIONS":
                    config.max_connections = int(v)
                basic:
                    println("Unknown numeric config:", key, "=", v)
            }
            
        mood bool:
            vibe_check key {
                mood "debug":
                    config.debug = v
                basic:
                    println("Unknown boolean config:", key, "=", v)
            }
            
        mood map[string]interface{}:
            vibe_check key {
                mood "feature_flags":
                    for flag_name, flag_value := range v {
                        if flag_bool, ok := flag_value.(bool); ok {
                            config.feature_flags[flag_name] = flag_bool
                        }
                    }
                mood "limits":
                    for limit_name, limit_value := range v {
                        config.limits[limit_name] = limit_value
                    }
                basic:
                    println("Unknown object config:", key)
            }
            
        basic:
            println("Unknown config type for", key, ":", fmt.sprintf("%T", v))
    }
    
    damn nil
}

slay demonstrate_configuration_system() {
    println("=== Configuration System with Type Switches ===")
    
    sus sources = []ConfigSource{
        FileConfig{path: "config.json"},
        EnvConfig{prefix: "APP_"},
        DatabaseConfig{connection_string: "postgres://..."}
    }
    
    if config, err := merge_configs(sources); err != nil {
        println("Configuration error:", err.error())
    } else {
        println("\nFinal Configuration:")
        println("Database URL:", config.database_url)
        println("Port:", config.port)
        println("Debug:", config.debug)
        println("API Key:", config.api_key)
        println("Timeout:", config.timeout)
        println("Max Connections:", config.max_connections)
        println("Feature Flags:", config.feature_flags)
        println("Limits:", config.limits)
    }
}

fr fr ================================================
fr fr 2. Message Queue System
fr fr ================================================

collab Message {
    slay id() string
    slay timestamp() time.Time
}

squad TextMessage {
    sus message_id string
    sus created_at time.Time
    sus content string
    sus sender string
}

slay (tm TextMessage) id() string {
    damn tm.message_id
}

slay (tm TextMessage) timestamp() time.Time {
    damn tm.created_at
}

squad ImageMessage {
    sus message_id string
    sus created_at time.Time
    sus image_url string
    sus thumbnail_url string
    sus width int
    sus height int
}

slay (im ImageMessage) id() string {
    damn im.message_id
}

slay (im ImageMessage) timestamp() time.Time {
    damn im.created_at
}

squad SystemMessage {
    sus message_id string
    sus created_at time.Time
    sus event_type string
    sus metadata map[string]interface{}
}

slay (sm SystemMessage) id() string {
    damn sm.message_id
}

slay (sm SystemMessage) timestamp() time.Time {
    damn sm.created_at
}

squad CommandMessage {
    sus message_id string
    sus created_at time.Time
    sus command string
    sus arguments []string
    sus require_auth bool
}

slay (cm CommandMessage) id() string {
    damn cm.message_id
}

slay (cm CommandMessage) timestamp() time.Time {
    damn cm.created_at
}

fr fr Message processor using type switches
slay process_message(msg Message) (string, error) {
    vibe_check m := msg.(type) {
        mood TextMessage:
            // Process text message
            sus words = strings.split(m.content, " ")
            sus word_count = len(words)
            
            // Check for mentions or hashtags
            sus mentions = 0
            sus hashtags = 0
            for _, word := range words {
                if strings.has_prefix(word, "@") {
                    mentions++
                } else if strings.has_prefix(word, "#") {
                    hashtags++
                }
            }
            
            damn fmt.sprintf("Text from %s: %d words, %d mentions, %d hashtags", 
                           m.sender, word_count, mentions, hashtags), nil
            
        mood ImageMessage:
            // Process image message
            sus aspect_ratio = float64(m.width) / float64(m.height)
            sus resolution = m.width * m.height
            
            sus size_category = ""
            vibe_check {
                mood resolution > 2000000:
                    size_category = "high"
                mood resolution > 500000:
                    size_category = "medium"
                basic:
                    size_category = "low"
            }
            
            damn fmt.sprintf("Image %dx%d (%.2f ratio, %s quality) at %s", 
                           m.width, m.height, aspect_ratio, size_category, m.image_url), nil
            
        mood SystemMessage:
            // Process system message
            vibe_check m.event_type {
                mood "user_joined":
                    if username, ok := m.metadata["username"].(string); ok {
                        damn fmt.sprintf("User %s joined the system", username), nil
                    }
                mood "user_left":
                    if username, ok := m.metadata["username"].(string); ok {
                        damn fmt.sprintf("User %s left the system", username), nil
                    }
                mood "settings_changed":
                    if setting, ok := m.metadata["setting"].(string); ok {
                        damn fmt.sprintf("Setting %s was changed", setting), nil
                    }
                basic:
                    damn fmt.sprintf("System event: %s", m.event_type), nil
            }
            
        mood CommandMessage:
            // Process command message
            if m.require_auth {
                // Simulate auth check
                if m.command == "admin" {
                    damn "", fmt.errorf("unauthorized command: %s", m.command)
                }
            }
            
            sus args_str = strings.join(m.arguments, ", ")
            damn fmt.sprintf("Command: %s with args [%s]", m.command, args_str), nil
            
        basic:
            damn "", fmt.errorf("unknown message type: %T", msg)
    }
}

fr fr Batch message processing
slay process_message_batch(messages []Message) {
    println("\n=== Message Processing Results ===")
    
    sus stats = map[string]int{
        "text": 0,
        "image": 0,
        "system": 0,
        "command": 0,
        "errors": 0
    }
    
    for i, msg := range messages {
        printf("Message %d [%s]: ", i+1, msg.id())
        
        if result, err := process_message(msg); err != nil {
            println("ERROR -", err.error())
            stats["errors"]++
        } else {
            println(result)
            
            // Update stats based on message type
            vibe_check msg.(type) {
                mood TextMessage:
                    stats["text"]++
                mood ImageMessage:
                    stats["image"]++
                mood SystemMessage:
                    stats["system"]++
                mood CommandMessage:
                    stats["command"]++
            }
        }
    }
    
    println("\nProcessing Statistics:")
    for msg_type, count := range stats {
        println(fmt.sprintf("  %s: %d", msg_type, count))
    }
}

slay demonstrate_message_queue() {
    println("\n=== Message Queue System with Type Switches ===")
    
    sus messages = []Message{
        TextMessage{
            message_id: "txt1",
            created_at: time.now(),
            content: "Hello @john! Check out this #tutorial",
            sender: "alice"
        },
        ImageMessage{
            message_id: "img1",
            created_at: time.now(),
            image_url: "https://example.com/image.jpg",
            thumbnail_url: "https://example.com/thumb.jpg",
            width: 1920,
            height: 1080
        },
        SystemMessage{
            message_id: "sys1",
            created_at: time.now(),
            event_type: "user_joined",
            metadata: map[string]interface{}{"username": "bob"}
        },
        CommandMessage{
            message_id: "cmd1",
            created_at: time.now(),
            command: "list",
            arguments: []string{"users", "--active"},
            require_auth: cap
        },
        CommandMessage{
            message_id: "cmd2",
            created_at: time.now(),
            command: "admin",
            arguments: []string{"reset"},
            require_auth: based
        }
    }
    
    process_message_batch(messages)
}

fr fr ================================================
fr fr 3. Database Query Builder
fr fr ================================================

collab QueryComponent {
    slay sql() string
    slay parameters() []interface{}
}

squad SelectQuery {
    sus columns []string
    sus table string
    sus conditions []QueryComponent
    sus limit_value int
}

slay (sq SelectQuery) sql() string {
    sus query = fmt.sprintf("SELECT %s FROM %s", 
                           strings.join(sq.columns, ", "), sq.table)
    
    if len(sq.conditions) > 0 {
        sus where_clauses = []string{}
        for _, condition := range sq.conditions {
            where_clauses = append(where_clauses, condition.sql())
        }
        query += " WHERE " + strings.join(where_clauses, " AND ")
    }
    
    if sq.limit_value > 0 {
        query += fmt.sprintf(" LIMIT %d", sq.limit_value)
    }
    
    damn query
}

slay (sq SelectQuery) parameters() []interface{} {
    sus params = []interface{}{}
    for _, condition := range sq.conditions {
        params = append(params, condition.parameters()...)
    }
    damn params
}

squad WhereCondition {
    sus column string
    sus operator string
    sus value interface{}
}

slay (wc WhereCondition) sql() string {
    damn fmt.sprintf("%s %s ?", wc.column, wc.operator)
}

slay (wc WhereCondition) parameters() []interface{} {
    damn []interface{}{wc.value}
}

squad InCondition {
    sus column string
    sus values []interface{}
}

slay (ic InCondition) sql() string {
    sus placeholders = make([]string, len(ic.values))
    for i := range placeholders {
        placeholders[i] = "?"
    }
    damn fmt.sprintf("%s IN (%s)", ic.column, strings.join(placeholders, ", "))
}

slay (ic InCondition) parameters() []interface{} {
    damn ic.values
}

squad BetweenCondition {
    sus column string
    sus min_value interface{}
    sus max_value interface{}
}

slay (bc BetweenCondition) sql() string {
    damn fmt.sprintf("%s BETWEEN ? AND ?", bc.column)
}

slay (bc BetweenCondition) parameters() []interface{} {
    damn []interface{}{bc.min_value, bc.max_value}
}

fr fr Query builder that handles different condition types
slay build_query(components []QueryComponent) (string, []interface{}) {
    if len(components) == 0 {
        damn "", []interface{}{}
    }
    
    // Assume first component is the main query
    sus main_query = components[0]
    sus all_params = []interface{}{}
    
    vibe_check q := main_query.(type) {
        mood SelectQuery:
            sus sql = q.sql()
            sus params = q.parameters()
            
            // Add any additional components
            for i := 1; i < len(components); i++ {
                component := components[i]
                
                vibe_check c := component.(type) {
                    mood WhereCondition:
                        sql += " AND " + c.sql()
                        params = append(params, c.parameters()...)
                    mood InCondition:
                        sql += " AND " + c.sql()
                        params = append(params, c.parameters()...)
                    mood BetweenCondition:
                        sql += " AND " + c.sql()
                        params = append(params, c.parameters()...)
                    basic:
                        // Unknown component type
                        sql += " AND " + c.sql()
                        params = append(params, c.parameters()...)
                }
            }
            
            damn sql, params
            
        basic:
            // Handle other query types
            damn main_query.sql(), main_query.parameters()
    }
}

slay demonstrate_query_builder() {
    println("\n=== Database Query Builder with Type Switches ===")
    
    // Build different types of queries
    sus queries = [][]QueryComponent{
        // Simple select with where condition
        {
            SelectQuery{
                columns: []string{"id", "name", "email"},
                table: "users",
                conditions: []QueryComponent{
                    WhereCondition{column: "active", operator: "=", value: based}
                },
                limit_value: 10
            }
        },
        
        // Complex query with multiple condition types
        {
            SelectQuery{
                columns: []string{"*"},
                table: "orders",
                conditions: []QueryComponent{},
                limit_value: 0
            },
            WhereCondition{column: "status", operator: "=", value: "pending"},
            InCondition{column: "customer_id", values: []interface{}{1, 2, 3, 4}},
            BetweenCondition{column: "created_at", min_value: "2024-01-01", max_value: "2024-12-31"}
        },
        
        // Query with just IN condition
        {
            SelectQuery{
                columns: []string{"product_name", "price"},
                table: "products",
                conditions: []QueryComponent{
                    InCondition{column: "category", values: []interface{}{"electronics", "books", "clothing"}}
                },
                limit_value: 50
            }
        }
    }
    
    for i, components := range queries {
        println(fmt.sprintf("\nQuery %d:", i+1))
        if sql, params := build_query(components); sql != "" {
            println("SQL:", sql)
            println("Parameters:", params)
        } else {
            println("Failed to build query")
        }
    }
}

fr fr ================================================
fr fr 4. Event Processing System
fr fr ================================================

collab Event {
    slay event_id() string
    slay timestamp() time.Time
    slay event_type() string
}

squad UserEvent {
    sus id string
    sus time time.Time
    sus user_id string
    sus action string
    sus metadata map[string]interface{}
}

slay (ue UserEvent) event_id() string { damn ue.id }
slay (ue UserEvent) timestamp() time.Time { damn ue.time }
slay (ue UserEvent) event_type() string { damn "user" }

squad SystemEvent {
    sus id string
    sus time time.Time
    sus component string
    sus level string
    sus message string
}

slay (se SystemEvent) event_id() string { damn se.id }
slay (se SystemEvent) timestamp() time.Time { damn se.time }
slay (se SystemEvent) event_type() string { damn "system" }

squad MetricEvent {
    sus id string
    sus time time.Time
    sus metric_name string
    sus value float64
    sus tags map[string]string
}

slay (me MetricEvent) event_id() string { damn me.id }
slay (me MetricEvent) timestamp() time.Time { damn me.time }
slay (me MetricEvent) event_type() string { damn "metric" }

fr fr Event processors based on type
squad EventProcessor {
    sus processed_count map[string]int
    sus error_count int
}

slay new_event_processor() *EventProcessor {
    damn &EventProcessor{
        processed_count: make(map[string]int),
        error_count: 0
    }
}

slay (ep *EventProcessor) process_event(event Event) error {
    vibe_check e := event.(type) {
        mood UserEvent:
            damn ep.process_user_event(e)
        mood SystemEvent:
            damn ep.process_system_event(e)
        mood MetricEvent:
            damn ep.process_metric_event(e)
        basic:
            ep.error_count++
            damn fmt.errorf("unknown event type: %T", event)
    }
}

slay (ep *EventProcessor) process_user_event(event UserEvent) error {
    println(fmt.sprintf("Processing user event: %s performed %s", event.user_id, event.action))
    
    // Different handling based on action
    vibe_check event.action {
        mood "login":
            if ip, ok := event.metadata["ip"].(string); ok {
                println(fmt.sprintf("  Login from IP: %s", ip))
            }
        mood "logout":
            if session_duration, ok := event.metadata["session_duration"].(float64); ok {
                println(fmt.sprintf("  Session lasted %.0f minutes", session_duration))
            }
        mood "purchase":
            if amount, ok := event.metadata["amount"].(float64); ok {
                println(fmt.sprintf("  Purchase amount: $%.2f", amount))
            }
        basic:
            println(fmt.sprintf("  Action: %s", event.action))
    }
    
    ep.processed_count["user"]++
    damn nil
}

slay (ep *EventProcessor) process_system_event(event SystemEvent) error {
    println(fmt.sprintf("Processing system event: %s [%s] %s", event.component, event.level, event.message))
    
    // Handle based on severity level
    vibe_check event.level {
        mood "error":
            println("  🚨 Critical system error - alerting on-call team")
        mood "warning":
            println("  ⚠️  System warning - monitoring for escalation")
        mood "info":
            println("  ℹ️  System information logged")
        basic:
            println("  📝 System event recorded")
    }
    
    ep.processed_count["system"]++
    damn nil
}

slay (ep *EventProcessor) process_metric_event(event MetricEvent) error {
    println(fmt.sprintf("Processing metric: %s = %.2f", event.metric_name, event.value))
    
    // Handle different metric types
    vibe_check event.metric_name {
        mood "cpu_usage":
            if event.value > 80.0 {
                println("  🔥 High CPU usage detected")
            }
        mood "memory_usage":
            if event.value > 90.0 {
                println("  💾 High memory usage detected")
            }
        mood "response_time":
            if event.value > 1000.0 {
                println("  🐌 Slow response time detected")
            }
        basic:
            println(fmt.sprintf("  📊 Metric %s recorded", event.metric_name))
    }
    
    // Print tags if available
    if len(event.tags) > 0 {
        sus tag_strings = []string{}
        for key, value := range event.tags {
            tag_strings = append(tag_strings, fmt.sprintf("%s=%s", key, value))
        }
        println(fmt.sprintf("  Tags: %s", strings.join(tag_strings, ", ")))
    }
    
    ep.processed_count["metric"]++
    damn nil
}

slay (ep *EventProcessor) get_stats() {
    println("\nEvent Processing Statistics:")
    for event_type, count := range ep.processed_count {
        println(fmt.sprintf("  %s events: %d", event_type, count))
    }
    println(fmt.sprintf("  errors: %d", ep.error_count))
}

slay demonstrate_event_processing() {
    println("\n=== Event Processing System with Type Switches ===")
    
    sus events = []Event{
        UserEvent{
            id: "ue1",
            time: time.now(),
            user_id: "user123",
            action: "login",
            metadata: map[string]interface{}{"ip": "192.168.1.100"}
        },
        SystemEvent{
            id: "se1",
            time: time.now(),
            component: "database",
            level: "error",
            message: "Connection pool exhausted"
        },
        MetricEvent{
            id: "me1",
            time: time.now(),
            metric_name: "cpu_usage",
            value: 85.5,
            tags: map[string]string{"host": "web01", "region": "us-east"}
        },
        UserEvent{
            id: "ue2",
            time: time.now(),
            user_id: "user456",
            action: "purchase",
            metadata: map[string]interface{}{"amount": 99.99, "product": "laptop"}
        },
        MetricEvent{
            id: "me2",
            time: time.now(),
            metric_name: "response_time",
            value: 1250.0,
            tags: map[string]string{"endpoint": "/api/users", "method": "GET"}
        }
    }
    
    sus processor = new_event_processor()
    
    for _, event := range events {
        if err := processor.process_event(event); err != nil {
            println("Error processing event:", err.error())
        }
    }
    
    processor.get_stats()
}

fr fr Main demonstration function
slay main() {
    demonstrate_configuration_system()
    demonstrate_message_queue()
    demonstrate_query_builder()
    demonstrate_event_processing()
    
    println("\n=== Practical Type Switch Use Cases Summary ===")
    println("1. Configuration systems - Handle different config sources and formats")
    println("2. Message queues - Process different message types with specific logic")
    println("3. Query builders - Build SQL from different condition types")
    println("4. Event processing - Handle different event types with appropriate actions")
    println("\nType switches enable clean, maintainable code for polymorphic data processing!")
}
