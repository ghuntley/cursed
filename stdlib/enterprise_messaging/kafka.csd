// Apache Kafka Producer/Consumer for CURSED
// Enterprise-grade messaging with high throughput and reliability

yeet "vibez"
yeet "networkz"
yeet "configz"
yeet "errorz"
yeet "concurrenz"
yeet "cryptz"
yeet "compressionz"

squad KafkaConfig {
    brokers tea[value] = ["localhost:9092"]
    client_id tea = "cursed-kafka-client"
    security_protocol tea = "PLAINTEXT"  // PLAINTEXT, SSL, SASL_PLAINTEXT, SASL_SSL
    sasl_mechanism tea = "PLAIN"         // PLAIN, SCRAM-SHA-256, SCRAM-SHA-512
    sasl_username tea = ""
    sasl_password tea = ""
    ssl_ca_location tea = ""
    ssl_cert_location tea = ""
    ssl_key_location tea = ""
    compression_type tea = "none"        // none, gzip, snappy, lz4, zstd
    request_timeout_ms drip = 30000
    retry_backoff_ms drip = 100
    max_retries drip = 3
    api_version drip = 2                 // Kafka API version
}

squad ProducerConfig {
    acks tea = "all"                     // 0, 1, all
    retries drip = 3
    batch_size drip = 16384
    linger_ms drip = 5
    buffer_memory drip = 33554432        // 32MB
    max_block_ms drip = 60000
    compression_type tea = "gzip"
    idempotent lit = based               // Enable idempotent producer
    enable_transactions lit = false
    transaction_timeout_ms drip = 60000
    max_in_flight_requests drip = 5
}

squad ConsumerConfig {
    group_id tea
    auto_offset_reset tea = "latest"     // earliest, latest, none
    enable_auto_commit lit = based
    auto_commit_interval_ms drip = 5000
    session_timeout_ms drip = 30000
    heartbeat_interval_ms drip = 3000
    max_poll_records drip = 500
    max_poll_interval_ms drip = 300000
    fetch_min_bytes drip = 1
    fetch_max_bytes drip = 52428800      // 50MB
    fetch_max_wait_ms drip = 500
    isolation_level tea = "read_uncommitted"  // read_uncommitted, read_committed
}

squad Message {
    topic tea
    partition drip = -1                  // -1 for automatic assignment
    key lit[value]
    value lit[value]
    headers map<tea, lit[value]>
    timestamp drip = 0                   // 0 for current time
    offset drip = -1                     // Set by Kafka
}

squad RecordMetadata {
    topic tea
    partition drip
    offset drip
    timestamp drip
    serialized_key_size drip
    serialized_value_size drip
}

squad TopicPartition {
    topic tea
    partition drip
}

squad OffsetAndMetadata {
    offset drip
    metadata tea = ""
    leader_epoch drip = -1
}

// Kafka Producer Implementation
squad Producer {
    config KafkaConfig
    producer_config ProducerConfig
    broker_connections map<tea, networkz.TcpSocket>
    metadata TopicMetadata
    pending_batches map<tea, Message[value]>
    send_buffer chan<Message>
    metrics ProducerMetrics
    transaction_id tea = ""
    in_transaction lit = false
    
    slay create_producer(kafka_config KafkaConfig, producer_config ProducerConfig) yikes<Producer> {
        sus producer Producer = {
            .config = kafka_config,
            .producer_config = producer_config,
            .broker_connections = {},
            .pending_batches = {},
            .send_buffer = concurrenz.make_channel<Message>(1000),
            .metrics = ProducerMetrics{},
        }
        
        // Initialize connections to brokers
        producer.connect_to_brokers() fam {
            when err -> yikes "failed to connect to brokers: " + err
        }
        
        // Fetch initial metadata
        producer.refresh_metadata() fam {
            when err -> yikes "failed to fetch metadata: " + err
        }
        
        // Start background sender
        producer.start_sender()
        
        // Initialize transactions if enabled
        ready (producer_config.enable_transactions) {
            producer.init_transactions() fam {
                when err -> yikes "failed to initialize transactions: " + err
            }
        }
        
        damn producer
    }
    
    slay send(message Message) yikes<RecordMetadata> {
        // Validate message
        ready (len(message.topic) == 0) {
            yikes "topic cannot be empty"
        }
        
        // Add to send buffer
        select {
            self.send_buffer <- message -> {
                // Message queued successfully
            }
            timeout(self.config.request_timeout_ms) -> {
                self.metrics.send_timeouts += 1
                yikes "send buffer full, timeout after " + to_string(self.config.request_timeout_ms) + "ms"
            }
        }
        
        // Return future metadata (simplified for demo)
        damn RecordMetadata{
            .topic = message.topic,
            .partition = 0,
            .offset = -1,
            .timestamp = timez.now_millis(),
        }
    }
    
    slay send_async(message Message, callback slay(RecordMetadata, tea)) {
        go {
            sus metadata RecordMetadata = self.send(message) fam {
                when err -> {
                    callback(RecordMetadata{}, err)
                    damn nil
                }
            }
            callback(metadata, "")
        }
    }
    
    slay begin_transaction() yikes<tea> {
        ready (!self.producer_config.enable_transactions) {
            yikes "transactions not enabled"
        }
        
        ready (self.in_transaction) {
            yikes "transaction already in progress"
        }
        
        // Send BeginTxn request
        self.send_begin_transaction_request() fam {
            when err -> yikes "failed to begin transaction: " + err
        }
        
        self.in_transaction = based
    }
    
    slay commit_transaction() yikes<tea> {
        ready (!self.in_transaction) {
            yikes "no transaction in progress"
        }
        
        // Flush pending messages
        self.flush() fam {
            when err -> yikes "failed to flush before commit: " + err
        }
        
        // Send EndTxn request with commit=true
        self.send_end_transaction_request(based) fam {
            when err -> yikes "failed to commit transaction: " + err
        }
        
        self.in_transaction = false
    }
    
    slay abort_transaction() yikes<tea> {
        ready (!self.in_transaction) {
            yikes "no transaction in progress"
        }
        
        // Send EndTxn request with commit=false
        self.send_end_transaction_request(false) fam {
            when err -> yikes "failed to abort transaction: " + err
        }
        
        self.in_transaction = false
    }
    
    slay flush() yikes<tea> {
        // Force send all pending messages
        self.force_send_all_batches() fam {
            when err -> yikes "failed to flush: " + err
        }
    }
    
    slay close() {
        // Flush and close producer
        self.flush()
        
        // Close send buffer
        close(self.send_buffer)
        
        // Close broker connections
        bestie (broker, conn := range self.broker_connections) {
            conn.close()
        }
    }
    
    slay get_metrics() ProducerMetrics {
        damn self.metrics
    }
    
    // Private methods
    slay start_sender() {
        go {
            bestie (message := range self.send_buffer) {
                self.add_to_batch(message)
                
                // Check if we should send batch
                ready (self.should_send_batch(message.topic)) {
                    self.send_batch(message.topic) fam {
                        when err -> {
                            vibez.spill("Failed to send batch:", err)
                            self.metrics.send_errors += 1
                        }
                    }
                }
            }
        }
    }
    
    slay connect_to_brokers() yikes<tea> {
        bestie (broker := range self.config.brokers) {
            sus conn networkz.TcpSocket = networkz.connect_tcp(broker) fam {
                when err -> {
                    vibez.spill("Failed to connect to broker", broker, ":", err)
                    continue
                }
            }
            
            // Configure SSL/SASL if needed
            self.configure_security(conn) fam {
                when err -> {
                    conn.close()
                    yikes "failed to configure security for " + broker + ": " + err
                }
            }
            
            self.broker_connections[broker] = conn
        }
        
        ready (len(self.broker_connections) == 0) {
            yikes "failed to connect to any brokers"
        }
    }
    
    slay refresh_metadata() yikes<tea> {
        // Send metadata request to any broker
        sus broker tea = ""
        bestie (b, _ := range self.broker_connections) {
            broker = b
            break
        }
        
        sus conn networkz.TcpSocket = self.broker_connections[broker]
        sus metadata_request lit[value] = build_metadata_request()
        
        conn.write(metadata_request) fam {
            when _ -> yikes "failed to send metadata request"
        }
        
        sus response lit[value] = read_response(conn) fam {
            when _ -> yikes "failed to read metadata response"
        }
        
        self.metadata = parse_metadata_response(response) fam {
            when err -> yikes "failed to parse metadata: " + err
        }
    }
}

// Kafka Consumer Implementation  
squad Consumer {
    config KafkaConfig
    consumer_config ConsumerConfig
    broker_connections map<tea, networkz.TcpSocket>
    metadata TopicMetadata
    subscribed_topics tea[value]
    assigned_partitions TopicPartition[value]
    position map<TopicPartition, drip>
    committed_offsets map<TopicPartition, OffsetAndMetadata>
    metrics ConsumerMetrics
    coordinator_connection networkz.TcpSocket
    generation_id drip = -1
    member_id tea = ""
    
    slay create_consumer(kafka_config KafkaConfig, consumer_config ConsumerConfig) yikes<Consumer> {
        sus consumer Consumer = {
            .config = kafka_config,
            .consumer_config = consumer_config,
            .broker_connections = {},
            .subscribed_topics = [],
            .assigned_partitions = [],
            .position = {},
            .committed_offsets = {},
            .metrics = ConsumerMetrics{},
        }
        
        // Initialize connections
        consumer.connect_to_brokers() fam {
            when err -> yikes "failed to connect to brokers: " + err
        }
        
        // Refresh metadata
        consumer.refresh_metadata() fam {
            when err -> yikes "failed to fetch metadata: " + err
        }
        
        // Find and connect to group coordinator
        consumer.find_coordinator() fam {
            when err -> yikes "failed to find coordinator: " + err
        }
        
        damn consumer
    }
    
    slay subscribe(topics tea[value]) yikes<tea> {
        self.subscribed_topics = topics
        
        // Join consumer group
        self.join_group() fam {
            when err -> yikes "failed to join group: " + err
        }
        
        // Sync group (get partition assignment)
        self.sync_group() fam {
            when err -> yikes "failed to sync group: " + err
        }
    }
    
    slay poll(timeout_ms drip) yikes<Message[value]> {
        sus messages Message[value] = []
        sus start_time drip = timez.now_millis()
        
        bestie (timez.now_millis() - start_time < timeout_ms) {
            // Send heartbeat if needed
            ready (self.should_send_heartbeat()) {
                self.send_heartbeat() fam {
                    when err -> {
                        vibez.spill("Heartbeat failed:", err)
                        self.metrics.heartbeat_errors += 1
                    }
                }
            }
            
            // Fetch messages from assigned partitions
            bestie (tp := range self.assigned_partitions) {
                sus partition_messages Message[value] = self.fetch_from_partition(tp) fam {
                    when err -> {
                        vibez.spill("Fetch failed for", tp.topic, "partition", tp.partition, ":", err)
                        continue
                    }
                }
                
                messages = append(messages, partition_messages...)
                
                // Update position
                ready (len(partition_messages) > 0) {
                    sus last_message Message = partition_messages[len(partition_messages) - 1]
                    self.position[tp] = last_message.offset + 1
                }
                
                // Check if we have enough messages
                ready (len(messages) >= self.consumer_config.max_poll_records) {
                    break
                }
            }
            
            // Auto-commit offsets if enabled
            ready (self.consumer_config.enable_auto_commit && len(messages) > 0) {
                ready (self.should_auto_commit()) {
                    self.commit_sync() fam {
                        when err -> {
                            vibez.spill("Auto-commit failed:", err)
                            self.metrics.commit_errors += 1
                        }
                    }
                }
            }
            
            // Break if we have messages or timeout
            ready (len(messages) > 0) {
                break
            }
            
            concurrenz.sleep(10)  // Small sleep to avoid busy polling
        }
        
        self.metrics.messages_consumed += len(messages)
        damn messages
    }
    
    slay commit_sync() yikes<tea> {
        sus offsets map<TopicPartition, OffsetAndMetadata> = {}
        
        // Build commit request with current positions
        bestie (tp, offset := range self.position) {
            offsets[tp] = OffsetAndMetadata{
                .offset = offset,
                .metadata = "",
            }
        }
        
        self.commit_offsets(offsets) fam {
            when err -> yikes "failed to commit offsets: " + err
        }
    }
    
    slay commit_async(callback slay(map<TopicPartition, OffsetAndMetadata>, tea)) {
        go {
            sus err tea = self.commit_sync() fam {
                when e -> e
            }
            
            ready (callback != nil) {
                callback(self.committed_offsets, err)
            }
        }
    }
    
    slay seek(partition TopicPartition, offset drip) {
        self.position[partition] = offset
    }
    
    slay seek_to_beginning(partitions TopicPartition[value]) yikes<tea> {
        bestie (tp := range partitions) {
            sus earliest_offset drip = self.get_earliest_offset(tp) fam {
                when err -> yikes "failed to get earliest offset: " + err
            }
            self.position[tp] = earliest_offset
        }
    }
    
    slay seek_to_end(partitions TopicPartition[value]) yikes<tea> {
        bestie (tp := range partitions) {
            sus latest_offset drip = self.get_latest_offset(tp) fam {
                when err -> yikes "failed to get latest offset: " + err
            }
            self.position[tp] = latest_offset
        }
    }
    
    slay close() {
        // Leave consumer group
        self.leave_group()
        
        // Close connections
        bestie (broker, conn := range self.broker_connections) {
            conn.close()
        }
        
        ready (self.coordinator_connection != nil) {
            self.coordinator_connection.close()
        }
    }
    
    slay get_metrics() ConsumerMetrics {
        damn self.metrics
    }
}

// Message Streaming and Kafka Streams-like functionality
squad StreamProcessor {
    input_topics tea[value]
    output_topic tea
    processor slay(Message) yikes<Message>
    consumer Consumer
    producer Producer
    
    slay create_stream_processor(
        input_topics tea[value],
        output_topic tea,
        processor slay(Message) yikes<Message>,
        kafka_config KafkaConfig
    ) yikes<StreamProcessor> {
        sus consumer_config ConsumerConfig = {
            .group_id = "stream-processor-" + generate_uuid(),
            .auto_offset_reset = "earliest",
        }
        
        sus producer_config ProducerConfig = {
            .acks = "all",
            .idempotent = based,
        }
        
        sus consumer Consumer = create_consumer(kafka_config, consumer_config) fam {
            when err -> yikes "failed to create consumer: " + err
        }
        
        sus producer Producer = create_producer(kafka_config, producer_config) fam {
            when err -> yikes "failed to create producer: " + err
        }
        
        damn StreamProcessor{
            .input_topics = input_topics,
            .output_topic = output_topic,
            .processor = processor,
            .consumer = consumer,
            .producer = producer,
        }
    }
    
    slay start() yikes<tea> {
        self.consumer.subscribe(self.input_topics) fam {
            when err -> yikes "failed to subscribe: " + err
        }
        
        go {
            bestie (based) {
                sus messages Message[value] = self.consumer.poll(1000) fam {
                    when err -> {
                        vibez.spill("Poll failed:", err)
                        continue
                    }
                }
                
                bestie (input_message := range messages) {
                    sus output_message Message = self.processor(input_message) fam {
                        when err -> {
                            vibez.spill("Processing failed:", err)
                            continue
                        }
                    }
                    
                    output_message.topic = self.output_topic
                    
                    self.producer.send(output_message) fam {
                        when err -> {
                            vibez.spill("Send failed:", err)
                        }
                    }
                }
            }
        }
    }
    
    slay stop() {
        self.consumer.close()
        self.producer.close()
    }
}

// Metrics and monitoring
squad ProducerMetrics {
    messages_sent drip = 0
    bytes_sent drip = 0
    send_errors drip = 0
    send_timeouts drip = 0
    batches_sent drip = 0
    records_per_batch_avg drip = 0
    compression_ratio_avg drip = 0
}

squad ConsumerMetrics {
    messages_consumed drip = 0
    bytes_consumed drip = 0
    fetch_errors drip = 0
    heartbeat_errors drip = 0
    commit_errors drip = 0
    rebalances drip = 0
    lag_max drip = 0
}

// Admin client for topic management
squad AdminClient {
    config KafkaConfig
    broker_connections map<tea, networkz.TcpSocket>
    
    slay create_admin_client(config KafkaConfig) yikes<AdminClient> {
        sus admin AdminClient = {
            .config = config,
            .broker_connections = {},
        }
        
        admin.connect_to_brokers() fam {
            when err -> yikes "failed to connect to brokers: " + err
        }
        
        damn admin
    }
    
    slay create_topics(topics TopicConfig[value]) yikes<tea> {
        sus request lit[value] = build_create_topics_request(topics)
        
        sus conn networkz.TcpSocket = self.get_controller_connection() fam {
            when err -> yikes "failed to get controller connection: " + err
        }
        
        conn.write(request) fam {
            when _ -> yikes "failed to send create topics request"
        }
        
        sus response lit[value] = read_response(conn) fam {
            when _ -> yikes "failed to read create topics response"
        }
        
        parse_create_topics_response(response) fam {
            when err -> yikes "create topics failed: " + err
        }
    }
    
    slay delete_topics(topic_names tea[value]) yikes<tea> {
        sus request lit[value] = build_delete_topics_request(topic_names)
        
        sus conn networkz.TcpSocket = self.get_controller_connection() fam {
            when err -> yikes "failed to get controller connection: " + err
        }
        
        conn.write(request) fam {
            when _ -> yikes "failed to send delete topics request"
        }
        
        sus response lit[value] = read_response(conn) fam {
            when _ -> yikes "failed to read delete topics response"
        }
        
        parse_delete_topics_response(response) fam {
            when err -> yikes "delete topics failed: " + err
        }
    }
    
    slay list_topics() yikes<TopicMetadata[value]> {
        sus request lit[value] = build_metadata_request()
        
        sus conn networkz.TcpSocket = self.get_any_broker_connection()
        
        conn.write(request) fam {
            when _ -> yikes "failed to send metadata request"
        }
        
        sus response lit[value] = read_response(conn) fam {
            when _ -> yikes "failed to read metadata response"
        }
        
        sus metadata TopicMetadata = parse_metadata_response(response) fam {
            when err -> yikes "failed to parse metadata: " + err
        }
        
        damn metadata.topics
    }
}

squad TopicConfig {
    name tea
    num_partitions drip = 1
    replication_factor drip = 1
    config map<tea, tea>  // Topic configuration properties
}

// Factory functions and utilities
slay create_simple_producer(brokers tea[value]) yikes<Producer> {
    sus kafka_config KafkaConfig = {
        .brokers = brokers,
        .client_id = "cursed-producer",
    }
    
    sus producer_config ProducerConfig = {
        .acks = "all",
        .retries = 3,
        .idempotent = based,
    }
    
    damn create_producer(kafka_config, producer_config)
}

slay create_simple_consumer(brokers tea[value], group_id tea) yikes<Consumer> {
    sus kafka_config KafkaConfig = {
        .brokers = brokers,
        .client_id = "cursed-consumer",
    }
    
    sus consumer_config ConsumerConfig = {
        .group_id = group_id,
        .auto_offset_reset = "latest",
    }
    
    damn create_consumer(kafka_config, consumer_config)
}

// Example usage patterns
slay example_producer_usage() yikes<tea> {
    sus producer Producer = create_simple_producer(["localhost:9092"]) fam {
        when err -> yikes err
    }
    defer { producer.close() }
    
    sus message Message = {
        .topic = "user-events",
        .key = encode_string("user123"),
        .value = encode_json({
            "user_id": "user123",
            "event": "login",
            "timestamp": timez.now_iso8601(),
        }),
        .headers = {
            "content-type": encode_string("application/json"),
        },
    }
    
    sus metadata RecordMetadata = producer.send(message) fam {
        when err -> yikes err
    }
    
    vibez.spill("Message sent to topic", metadata.topic, 
                "partition", metadata.partition, 
                "offset", metadata.offset)
}

slay example_consumer_usage() yikes<tea> {
    sus consumer Consumer = create_simple_consumer(["localhost:9092"], "user-event-processors") fam {
        when err -> yikes err
    }
    defer { consumer.close() }
    
    consumer.subscribe(["user-events"]) fam {
        when err -> yikes err
    }
    
    bestie (based) {
        sus messages Message[value] = consumer.poll(1000) fam {
            when err -> {
                vibez.spill("Poll error:", err)
                continue
            }
        }
        
        bestie (message := range messages) {
            vibez.spill("Received message from topic", message.topic,
                        "partition", message.partition,
                        "offset", message.offset,
                        "key", decode_string(message.key),
                        "value", decode_string(message.value))
        }
    }
}
