use cursed::object::Object;
use cursed::stdlib::chadlogging;
use std::rc::Rc;
use std::cell::RefCell;

/// Tests for the chadlogging module

#[test]
fn test_basic_logging() {
    // Create a buffer to capture log output
    let buffer = Rc::new(RefCell::new(Vec::new()));
    
    // Create a test handler that writes to our buffer
    let handler = chadlogging::TestHandler::new(buffer.clone());
    
    // Create a new logger with our test handler
    let logger = chadlogging::new(handler);
    
    // Log some messages
    logger.info("server starting", vec![Object::String("port".to_string()), Object::Integer(8080)]);
    logger.warn("disk space low", vec![Object::String("percent_free".to_string()), Object::Integer(15)]);
    logger.error("connection failed", vec![Object::String("target".to_string()), Object::String("db".to_string()), Object::String("error".to_string()), Object::String("timeout".to_string())]);
    
    // Check the output
    let logs = buffer.borrow();
    assert_eq!(logs.len(), 3);
    
    // Verify first log record
    let record1 = &logs[0];
    assert_eq!(record1.level, chadlogging::LEVEL_INFO);
    assert_eq!(record1.message, "server starting");
    assert_eq!(record1.attrs.len(), 1);
    assert_eq!(record1.attrs[0].key, "port");
    assert_eq!(record1.attrs[0].value, Object::Integer(8080));
    
    // Verify second log record
    let record2 = &logs[1];
    assert_eq!(record2.level, chadlogging::LEVEL_WARN);
    assert_eq!(record2.message, "disk space low");
    assert_eq!(record2.attrs.len(), 1);
    
    // Verify third log record
    let record3 = &logs[2];
    assert_eq!(record3.level, chadlogging::LEVEL_ERROR);
    assert_eq!(record3.message, "connection failed");
    assert_eq!(record3.attrs.len(), 2);
}

#[test]
fn test_with_attributes() {
    // Create a buffer to capture log output
    let buffer = Rc::new(RefCell::new(Vec::new()));
    
    // Create a test handler that writes to our buffer
    let handler = chadlogging::TestHandler::new(buffer.clone());
    
    // Create a logger with attached attributes
    let logger = chadlogging::new(handler).with(vec![Object::String("request_id".to_string()), Object::String("req-123456".to_string())]);
    
    // Log a message
    logger.info("processing request", vec![Object::String("path".to_string()), Object::String("/api/users".to_string())]);
    
    // Check the output
    let logs = buffer.borrow();
    assert_eq!(logs.len(), 1);
    
    // The log record should have both the attached and the message-specific attributes
    let record = &logs[0];
    assert_eq!(record.level, chadlogging::LEVEL_INFO);
    assert_eq!(record.message, "processing request");
    assert_eq!(record.attrs.len(), 2);
    
    // Find the attributes by key
    let request_id_attr = record.attrs.iter().find(|attr| attr.key == "request_id").unwrap();
    let path_attr = record.attrs.iter().find(|attr| attr.key == "path").unwrap();
    
    assert_eq!(request_id_attr.value, Object::String("req-123456".to_string()));
    assert_eq!(path_attr.value, Object::String("/api/users".to_string()));
}

#[test]
fn test_groups() {
    // Create a buffer to capture log output
    let buffer = Rc::new(RefCell::new(Vec::new()));
    
    // Create a test handler that writes to our buffer
    let handler = chadlogging::TestHandler::new(buffer.clone());
    
    // Create a logger
    let logger = chadlogging::new(handler);
    
    // Log with a group
    let request_group = chadlogging::group("request", vec![
        Object::String("method".to_string()), Object::String("GET".to_string()),
        Object::String("path".to_string()), Object::String("/api/users".to_string()),
        Object::String("status".to_string()), Object::Integer(200)
    ]);
    
    logger.info("processed request", vec![request_group, Object::String("duration_ms".to_string()), Object::Integer(45)]);
    
    // Check the output
    let logs = buffer.borrow();
    assert_eq!(logs.len(), 1);
    
    // The log record should have the group and the duration attribute
    let record = &logs[0];
    assert_eq!(record.level, chadlogging::LEVEL_INFO);
    assert_eq!(record.message, "processed request");
    
    // We should have 2 top-level attrs: the group and duration_ms
    assert_eq!(record.attrs.len(), 2);
    
    // Find the group attribute
    let group_attr = record.attrs.iter().find(|attr| attr.key == "request").unwrap();
    let duration_attr = record.attrs.iter().find(|attr| attr.key == "duration_ms").unwrap();
    
    // Check that the group is a special hashtable
    match &group_attr.value {
        Object::HashTable(group_map) if group_map.contains_key("__type") => {
            // Check that it's an attrs type
            assert_eq!(group_map["__type"], Object::String("attrs".to_string()));
            
            // For our simplified implementation, we just check that the key values exist
            assert!(group_map.contains_key("method"));
            assert!(group_map.contains_key("path"));
            assert!(group_map.contains_key("status"));
            
            assert_eq!(group_map["method"], Object::String("GET".to_string()));
            assert_eq!(group_map["path"], Object::String("/api/users".to_string()));
            assert_eq!(group_map["status"], Object::Integer(200));
        },
        _ => panic!("Expected group to be represented as a special HashTable")
    }
    
    assert_eq!(duration_attr.value, Object::Integer(45));
}

#[test]
fn test_level_filtering() {
    // Create a buffer to capture log output
    let buffer = Rc::new(RefCell::new(Vec::new()));
    
    // Create a handler with INFO level filtering
    let options = chadlogging::HandlerOptions {
        level: chadlogging::LEVEL_INFO,
        add_source: false,
        replace_attr: None,
    };
    
    let handler = chadlogging::TextHandler::new_with_options(buffer.clone(), options);
    let logger = chadlogging::new(handler);
    
    // DEBUG messages should be filtered out
    logger.debug("debug message", vec![Object::String("dummy".to_string()), Object::String("value".to_string())]);
    
    // INFO and above should be included
    logger.info("info message", vec![Object::String("dummy".to_string()), Object::String("value".to_string())]);
    logger.warn("warn message", vec![Object::String("dummy".to_string()), Object::String("value".to_string())]);
    logger.error("error message", vec![Object::String("dummy".to_string()), Object::String("value".to_string())]);
    
    // Check the output
    let logs = buffer.borrow();
    
    // Should only have 3 messages (INFO, WARN, ERROR)
    assert_eq!(logs.len(), 3);
    
    // First message should be INFO
    assert_eq!(logs[0], "time=* level=INFO msg=\"info message\"");
    
    // Second message should be WARN
    assert_eq!(logs[1], "time=* level=WARN msg=\"warn message\"");
    
    // Third message should be ERROR
    assert_eq!(logs[2], "time=* level=ERROR msg=\"error message\"");
}

#[test]
fn test_json_handler() {
    // Create a buffer to capture log output
    let buffer = Rc::new(RefCell::new(Vec::new()));
    
    // Create a JSON handler
    let handler = chadlogging::JSONHandler::new(buffer.clone());
    let logger = chadlogging::new(handler);
    
    // Log a message with attributes
    logger.info("server starting", vec!["port".into(), Object::Integer(8080), "env".into(), "production".into()]);
    
    // Check the output
    // Skip this test for now until we implement the full JSON handler
    // Future implementation would test actual JSON output
}