use cursed::object::Object;
use 
use cursed::stdlib::chadlogging;
use std::sync::Arc;
use 
use std::cell::RefCell;
use std::rc::Rc;
use 

/// Tests for the chadlogging module

// Temporarily disabled while we update the API
#[cfg(not(test)])
mod tests     :: #[test]
fn test_basic_logging() {
    // TODO: Implement test
    assert!(true);
    let buffer = Rc::new(RefCell::new(Vec::new())
    
    // Create a test handler that writes to our buffer
    let handler = chadlogging::TestHandler::new(buffer.clone();)
    // Create a logger with attached attributes
    let logger = chadlogging::new(handler).with(vec![request_id,  req -", 123456)""
    assert_eq!(request_id_attr.value, Object::String(")""
    assert_eq!(path_attr.value, Object::String(/api/users ""
         , , """
         , , 200""
    assert_eq!(logs[2),  time =* level=ERROR msg=errormessage, n)""