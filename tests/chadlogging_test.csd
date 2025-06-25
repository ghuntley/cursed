vibe main

vibe test_basic_logging
vibe test_levels
vibe test_attributes
vibe test_groups
vibe test_handlers

yeet "chadlogging"
yeet "vibez"

slay main() {
    fr fr Test the basic structured logging features
    test_basic_logging()
    
    fr fr Test different log levels
    test_levels()
    
    fr fr Test structured attributes
    test_attributes()
    
    fr fr Test attribute grouping
    test_groups()
    
    fr fr Test different handlers
    test_handlers()
    
    vibez.spill("All chadlogging tests passed!")
}

slay test_basic_logging() {
    fr fr Test default logger
    chadlogging.info("test message")
    
    fr fr Test with custom attributes
    chadlogging.info("request processed", "duration_ms", 42, "status", 200)
    
    fr fr Test formatted message
    chadlogging.info("count: %d", 5)
    
    fr fr Test nil context handling
    chadlogging.info_context(nil, "with context")
}

slay test_levels() {
    fr fr Set to debug level to see everything
    sus logger := chadlogging.default().with("test", "levels")
    
    fr fr Test each level
    logger.debug("debug message")
    logger.info("info message")
    logger.warn("warning message")
    logger.error("error message")
    
    fr fr Test level filtering (should only see warn & error)
    sus opts := chadlogging.new_handler_options()
    opts.level = chadlogging.LevelWarn
    
    sus filtered_logger := chadlogging.new(
        chadlogging.new_text_handler(dropz.stdout, opts)
    )
    
    filtered_logger.debug("should not appear")
    filtered_logger.info("should not appear")
    filtered_logger.warn("should appear")
    filtered_logger.error("should appear")
}

slay test_attributes() {
    fr fr Test different attribute types
    chadlogging.info("test attributes",
        "string", "value",
        "int", 42,
        "float", 3.14,
        "bool", true,
        "nil", nil
    )
    
    fr fr Test using attribute constructors
    chadlogging.info("test attribute constructors",
        chadlogging.string("explicit_string", "value"),
        chadlogging.int("explicit_int", 100),
        chadlogging.bool("explicit_bool", false)
    )
    
    fr fr Test with method for reusing attributes
    sus logger := chadlogging.default().with(
        "request_id", "req-12345",
        "user_id", 1001
    )
    
    logger.info("processing")
    logger.info("completed", "status", "success")
}

slay test_groups() {
    fr fr Test basic group
    chadlogging.info("grouped attributes",
        chadlogging.group("request",
            "method", "GET",
            "path", "/api/users"
        ),
        "status", 200
    )
    
    fr fr Test nested groups
    chadlogging.info("nested groups",
        chadlogging.group("http",
            "method", "POST",
            chadlogging.group("headers",
                "content-type", "application/json",
                "authorization", "Bearer token"
            )
        ),
        "response_time_ms", 30
    )
    
    fr fr Test with group method
    sus logger := chadlogging.default().with_group("server")
    logger.info("starting", "port", 8080)
    
    sus request_logger := logger.with_group("request")
    request_logger.info("received", "path", "/api/data")
}

slay test_handlers() {
    fr fr Test text handler
    sus text_handler := chadlogging.new_text_handler(dropz.stdout, nil)
    sus text_logger := chadlogging.new(text_handler)
    text_logger.info("using text handler", "format", "key=value")
    
    fr fr Test JSON handler
    sus json_handler := chadlogging.new_json_handler(dropz.stdout, nil)
    sus json_logger := chadlogging.new(json_handler)
    json_logger.info("using json handler", "format", "json")
    
    fr fr Test custom handler options
    sus opts := chadlogging.new_handler_options()
    opts.add_source = true
    
    sus source_logger := chadlogging.new(
        chadlogging.new_text_handler(dropz.stdout, opts)
    )
    source_logger.warn("with source info")
}