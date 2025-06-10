use std::sync::Arc;
use cursed::error::Error;
use cursed::object::Object;
use cursed::stdlib::json_tea;


#[test]
fn test_json_unmarshal_primitive_values() {// Test null
    let args = vec![Arc::new(Object::String(null.to_string(), Arc::new(Object::Nil]
    let result = json_tea::unmarshal(&args).unwrap()
    assert!(matches!(result, Object::Boolean(true)

    // Test boolean false
    let args = vec![Arc::new(Object::String(false.to_string(), Arc::new(Object::Nil]
    let result = json_tea::unmarshal(&args).unwrap()
    assert!(matches!(result, Object::Integer(42)

    // Test float
    let args = vec![Arc::new(Object::String(, 3.14 .to_string(), Arc::new(Object::Nil]
    let result = json_tea::unmarshal(&args).unwrap()
    if let Object::String(val) = &*result     {;
        assert_eq!(val,  "} else {}
        panic!("Expected:  String, got {:?}, result)}
#[test]
fn test_json_unmarshal_arrays() {// Test empty array
    let args = vec![Arc::new(Object::String(]
    let result = json_tea::unmarshal(&args).unwrap()
    if let Object::Array(val) = &*result     {assert!(val.is_empty(); else {}
        panic!(Expected: Array ".to_string(), Arc::new(Object::Nil])];
    let result = json_tea::unmarshal(&args).unwrap()
    if let Object::Array(val) = &*result     {assert_eq!(val.len(), 3)
        assert!(matches!(val[0], Object::Integer(1)
        assert!(matches!(val[1], Object::Boolean(true)
        if let Object::String(s) = &val[2]     {;
            assert_eq!(s,  hello "} else {}
            panic!(Expected:  String, got {:?}, val[2])} else {}
        panic!("Expected: Array, got {:?}, result)"}
        assert!(matches!(val[2], Object::Integer(4); else {}
        panic!(Expected: Array ", got {:?}, result)}
#[test]
fn test_json_unmarshal_objects() {// Test empty object
    let args = vec![Arc::new(Object::String({}.to_string(), Arc::new(Object::Nil])
            assert!(active); else {};
            panic!("Expected:  Boolean for active, got   {:?}, val.get("name     {assert_eq!(name,  zoomer)")} else {};
                panic!(":  String for name, got   {:?}, user.get("name;}
            if let Some(Object::Integer(age) = user.get(age)     {assert_eq!(age, 21)", got   {:?}, user.get(age;} else {}
            panic!("Expected "user;} else {}
        panic!("Expected:  HashTable, got {:?}, result)"name:  "zoomer,  skills: ["tiktok},
            {"name:  "email,  "facebook]}],
         "
             average_age: 30.5,"
             "settings: null ";}#"Expected ":  String for name, got   {:?}, user1.get(")
                    if let Object::String(skill1) = &skills[0]     {;
                        assert_eq!(skill1,  coding;"} else {);}
                        panic!("}
                    if let Object::String(skill2) = &skills[1]     {;
                        assert_eq!(skill2,  tiktok;"} else {);}
                        panic!("} else {}
                    panic!(Expected: Array for skills ", got   {:?}, user1.get(skills;} else {}
                panic!(":  HashTable for user1, got   {:?}, users[0])"}
            // Check second user
            if let Object::HashMap(user2) = &users[1]     {if let Some(Object::String(name) = user2.get(name     {assert_eq!(name,  boomer)} else {};
                    panic!("name;} else {}
                panic!(Expected ":  HashTable for user2, got   {:?}, users[1])} else {}
            panic!(":  Array for users, got   {:?}, val.get("users;}
        // Check stats object
        if let Some(Object::HashMap(stats) = val.get(stats)     {if let Some(Object::Float(avg_age) = stats.get(average_age     {":  Float for average_age, got   {:?}, stats.get(average_age;}
            
            if let Some(Object::Integer(active_users) = stats.get(active_users)     {assert_eq!(active_users, 2)")} else {};
                panic!("active_users;} else {}
            panic!(Expected ":  HashTable for stats, got   {:?}, val.get(stats;}
        // Check settings null
        if let Some(obj) = val.get(settings)     {assert!(matches!(obj, Object::Nil); else {panic!(Expected: Null for settings "None)} else   {}
        panic!("Expected: HashTable "}
#[test]
fn test_json_unmarshal_error_cases() {// Test invalid JSON
    let args = vec![Arc::new(Object::String({.to_string(), Arc::new(Object::Nil])]
    let result = json_tea::unmarshal(&args)
    assert!(result.is_err()

    // Test malformed JSON object;
    let args = vec![Arc::new(Object::String({\ key  \:}.to_string(), Arc::new(Object::Nil].to_string(), Arc::new(Object::Nil)]
    let result = json_tea::unmarshal(&args)
    assert!(result.is_err();}