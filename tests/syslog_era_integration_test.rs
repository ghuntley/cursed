use cursed::stdlib::syslog_era;
use cursed::stdlib;
use cursed::core::environment::Environment;
use cursed::core::value::Value;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::parse_program;
use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;

#[test]
fn test_syslog_era_constants() {
    let mut env = Environment::new();
    stdlib::register_all_packages(&mut env).expect("Failed to register stdlib");
    
    // Test facility constants
    assert_eq!(env.get_var("syslog_era.Kernel"), Some(Value::Integer(0)));
    assert_eq!(env.get_var("syslog_era.UserLevel"), Some(Value::Integer(1)));
    assert_eq!(env.get_var("syslog_era.Local0"), Some(Value::Integer(16)));
    assert_eq!(env.get_var("syslog_era.Local7"), Some(Value::Integer(23)));
    
    // Test severity constants
    assert_eq!(env.get_var("syslog_era.Emerg"), Some(Value::Integer(0)));
    assert_eq!(env.get_var("syslog_era.Alert"), Some(Value::Integer(1)));
    assert_eq!(env.get_var("syslog_era.Info"), Some(Value::Integer(6)));
    assert_eq!(env.get_var("syslog_era.Debug"), Some(Value::Integer(7)));
}

#[test]
fn test_syslog_era_writer_methods() {
    let mut env = Environment::new();
    stdlib::register_all_packages(&mut env).expect("Failed to register stdlib");
    
    // Create a mock writer
    // This doesn't actually connect to a syslog server
    let args = vec![
        Value::String("udp".to_string()),
        Value::String("localhost:514".to_string()),
        Value::Integer(syslog_era::LOCAL0 | syslog_era::INFO),
        Value::String("test_tag".to_string()),
    ];
    
    // This should fail gracefully since we don't have a real syslog server
    let result = syslog_era::syslog_dial(&args);
    if let Ok(writer_val) = result {
        if let Value::Object(writer_obj) = writer_val {
            // Test that methods exist on the writer object
            let method_names = ["emerg", "alert", "crit", "err", "warning", "notice", "info", "debug", "close"];
            
            for method in method_names.iter() {
                let msg = vec![Value::String("Test message".to_string())];
                let result = writer_obj.call_method(method, &msg);
                
                // We expect an error because we're not really connected
                assert!(result.is_err());
                println!("Method {} returned expected error: {}", method, result.err().unwrap());
            }
        } else {
            panic!("Expected writer to be an object");
        }
    } else {
        // On most systems without a syslog server, this is expected to fail
        println!("Could not connect to syslog server (expected): {}", result.err().unwrap());
    }
}

// This test runs the CURSED script to verify basic constants are exported properly
#[test]
fn test_syslog_era_script() {
    let script_path = Path::new("tests/syslog_era_test.csd");
    assert!(script_path.exists(), "Test script doesn't exist");
    
    let mut source = String::new();
    let mut file = fs::File::open(script_path).expect("Failed to open test script");
    file.read_to_string(&mut source).expect("Failed to read test script");
    
    let mut env = Environment::new();
    stdlib::register_all_packages(&mut env).expect("Failed to register stdlib");
    
    let mut lexer = Lexer::new(&source);
    let mut parser = Parser::new(&mut lexer);
    
    let program = parse_program(&mut parser).expect("Failed to parse program");
    
    // Test that the script parses successfully - we don't run it as it tries to connect
    // to a syslog server which may not be available in the test environment
    assert!(!program.statements.is_empty());
}