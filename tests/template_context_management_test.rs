/// Comprehensive tests for the enhanced template context variable management system
use std::collections::HashMap;
use std::sync::Arc;

use cursed::stdlib::template::template_core::{
    TemplateContext, TemplateEngine, TemplateConfig, FileSystemLoader, ContextIsolationLevel
};
use cursed::object::Object as CursedObject;
use cursed::error::Error as CursedError;

#[test]
fn test_basic_context_variable_operations() {
    let context = TemplateContext::new();
    
    // Test setting variables
    context.set("name", CursedObject::String("Alice".to_string())).unwrap();
    context.set("age", CursedObject::Integer(25)).unwrap();
    context.set("active", CursedObject::Boolean(true)).unwrap();
    
    // Test getting variables
    assert_eq!(context.get("name"), Some(CursedObject::String("Alice".to_string())));
    assert_eq!(context.get("age"), Some(CursedObject::Integer(25)));
    assert_eq!(context.get("active"), Some(CursedObject::Boolean(true)));
    assert_eq!(context.get("nonexistent"), None);
    
    // Test contains
    assert!(context.contains("name"));
    assert!(context.contains("age"));
    assert!(context.contains("active"));
    assert!(!context.contains("nonexistent"));
}

#[test]
fn test_context_inheritance_and_shadowing() {
    let parent = TemplateContext::new();
    parent.set("global_var", CursedObject::String("global".to_string())).unwrap();
    parent.set("override_me", CursedObject::String("parent".to_string())).unwrap();
    parent.set("parent_only", CursedObject::Integer(42)).unwrap();
    
    let child = TemplateContext::with_parent(parent);
    child.set("local_var", CursedObject::String("local".to_string())).unwrap();
    child.set("override_me", CursedObject::String("child".to_string())).unwrap();
    
    // Test variable lookup with inheritance
    assert_eq!(child.get("global_var"), Some(CursedObject::String("global".to_string())));
    assert_eq!(child.get("parent_only"), Some(CursedObject::Integer(42)));
    assert_eq!(child.get("local_var"), Some(CursedObject::String("local".to_string())));
    
    // Test variable shadowing
    assert_eq!(child.get("override_me"), Some(CursedObject::String("child".to_string())));
    
    // Test contains with inheritance
    assert!(child.contains("global_var"));
    assert!(child.contains("parent_only"));
    assert!(child.contains("local_var"));
    assert!(child.contains("override_me"));
    assert!(!child.contains("nonexistent"));
}

#[test]
fn test_loop_context_scoping() {
    let base_context = TemplateContext::new();
    base_context.set("global", CursedObject::String("value".to_string())).unwrap();
    
    // Create loop scope
    let loop_item = CursedObject::String("item1".to_string());
    let loop_scope = base_context.create_loop_scope("item".to_string(), loop_item.clone(), 0).unwrap();
    
    // Test loop variable access
    assert_eq!(loop_scope.get("item"), Some(loop_item));
    assert_eq!(loop_scope.get("global"), Some(CursedObject::String("value".to_string())));
    
    // Test loop metadata
    let loop_data = loop_scope.get("loop").unwrap();
    if let CursedObject::Map(map) = loop_data {
        assert_eq!(map.get("index"), Some(&CursedObject::Integer(0)));
        assert_eq!(map.get("index0"), Some(&CursedObject::Integer(0)));
        assert_eq!(map.get("index1"), Some(&CursedObject::Integer(1)));
        assert_eq!(map.get("first"), Some(&CursedObject::Boolean(true)));
    } else {
        panic!("Expected loop data to be a map");
    }
}

#[test]
fn test_include_context_merging() {
    let base_context = TemplateContext::new();
    base_context.set("base_var", CursedObject::String("base".to_string())).unwrap();
    
    let mut include_vars = HashMap::new();
    include_vars.insert("include_var".to_string(), CursedObject::String("included".to_string()));
    include_vars.insert("override_var".to_string(), CursedObject::String("override".to_string()));
    
    base_context.set("override_var", CursedObject::String("original".to_string())).unwrap();
    
    let include_context = base_context.create_include_context(include_vars).unwrap();
    
    // Test variable access
    assert_eq!(include_context.get("base_var"), Some(CursedObject::String("base".to_string())));
    assert_eq!(include_context.get("include_var"), Some(CursedObject::String("included".to_string())));
    
    // Test variable override (local should win)
    assert_eq!(include_context.get("override_var"), Some(CursedObject::String("override".to_string())));
}

#[test]
fn test_context_isolation_levels() {
    // Test Strict isolation
    let strict_context = TemplateContext::new_with_isolation(ContextIsolationLevel::Strict);
    strict_context.set("test_var", CursedObject::String("value".to_string())).unwrap();
    
    // Update should work for existing variable
    let updated = strict_context.update("test_var", CursedObject::String("updated".to_string())).unwrap();
    assert!(updated);
    assert_eq!(strict_context.get("test_var"), Some(CursedObject::String("updated".to_string())));
    
    // Update should fail for non-existing variable in strict mode
    let not_updated = strict_context.update("new_var", CursedObject::String("new".to_string())).unwrap();
    assert!(!not_updated);
    
    // Test Local isolation
    let local_context = TemplateContext::new_with_isolation(ContextIsolationLevel::Local);
    
    // Update should always work in local mode
    let updated = local_context.update("any_var", CursedObject::String("value".to_string())).unwrap();
    assert!(updated);
    assert_eq!(local_context.get("any_var"), Some(CursedObject::String("value".to_string())));
}

#[test]
fn test_shadow_scope_creation() {
    let base_context = TemplateContext::new();
    base_context.set("base_var", CursedObject::String("base".to_string())).unwrap();
    base_context.set("shadow_me", CursedObject::String("original".to_string())).unwrap();
    
    let shadow_scope = base_context.create_shadow_scope();
    shadow_scope.set("shadow_me", CursedObject::String("shadowed".to_string())).unwrap();
    shadow_scope.set("local_var", CursedObject::String("local".to_string())).unwrap();
    
    // Test variable shadowing
    assert_eq!(shadow_scope.get("shadow_me"), Some(CursedObject::String("shadowed".to_string())));
    assert_eq!(shadow_scope.get("base_var"), Some(CursedObject::String("base".to_string())));
    assert_eq!(shadow_scope.get("local_var"), Some(CursedObject::String("local".to_string())));
    
    // Original context should remain unchanged
    assert_eq!(base_context.get("shadow_me"), Some(CursedObject::String("original".to_string())));
    assert_eq!(base_context.get("local_var"), None);
}

#[test]
fn test_context_merging() {
    let context1 = TemplateContext::new();
    context1.set("var1", CursedObject::String("value1".to_string())).unwrap();
    context1.set("shared", CursedObject::String("context1".to_string())).unwrap();
    
    let context2 = TemplateContext::new();
    context2.set("var2", CursedObject::String("value2".to_string())).unwrap();
    context2.set("shared", CursedObject::String("context2".to_string())).unwrap();
    
    // Merge context2 into context1
    context1.merge(&context2).unwrap();
    
    // Test merged variables
    assert_eq!(context1.get("var1"), Some(CursedObject::String("value1".to_string())));
    assert_eq!(context1.get("var2"), Some(CursedObject::String("value2".to_string())));
    assert_eq!(context1.get("shared"), Some(CursedObject::String("context2".to_string()))); // Should be overridden
}

#[test]
fn test_context_local_vs_inherited_access() {
    let parent = TemplateContext::new();
    parent.set("parent_var", CursedObject::String("parent".to_string())).unwrap();
    
    let child = TemplateContext::with_parent(parent);
    child.set("child_var", CursedObject::String("child".to_string())).unwrap();
    
    // Test local-only access
    assert_eq!(child.get_local("child_var"), Some(CursedObject::String("child".to_string())));
    assert_eq!(child.get_local("parent_var"), None);
    
    // Test inherited access
    assert_eq!(child.get("child_var"), Some(CursedObject::String("child".to_string())));
    assert_eq!(child.get("parent_var"), Some(CursedObject::String("parent".to_string())));
    
    // Test contains local vs inherited
    assert!(child.contains_local("child_var"));
    assert!(!child.contains_local("parent_var"));
    assert!(child.contains("child_var"));
    assert!(child.contains("parent_var"));
}

#[test]
fn test_context_debugging_features() {
    let context = TemplateContext::new();
    context.set("test_var", CursedObject::String("value".to_string())).unwrap();
    
    // Test context ID generation
    let context_id = context.get_context_id();
    assert!(context_id.starts_with("ctx_"));
    assert_eq!(context_id.len(), 12); // "ctx_" + 8 character UUID prefix
    
    // Test isolation level access
    assert_eq!(context.get_isolation_level(), ContextIsolationLevel::Local);
    
    // Test getting all local variables
    let all_vars = context.get_all_local();
    assert_eq!(all_vars.len(), 1);
    assert_eq!(all_vars.get("test_var"), Some(&CursedObject::String("value".to_string())));
}

#[test]
fn test_thread_safety() {
    use std::thread;
    use std::sync::Arc;
    
    let context = Arc::new(TemplateContext::new());
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent access
    for i in 0..10 {
        let context_clone = Arc::clone(&context);
        let handle = thread::spawn(move || {
            let key = format!("var_{}", i);
            let value = CursedObject::Integer(i);
            
            // Set variable
            context_clone.set(key.clone(), value.clone()).unwrap();
            
            // Get variable
            let retrieved = context_clone.get(&key).unwrap();
            assert_eq!(retrieved, value);
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all variables were set
    for i in 0..10 {
        let key = format!("var_{}", i);
        let expected = CursedObject::Integer(i);
        assert_eq!(context.get(&key), Some(expected));
    }
}

#[test]
fn test_error_handling() {
    let context = TemplateContext::new();
    
    // Test successful operations
    assert!(context.set("valid_var", CursedObject::String("value".to_string())).is_ok());
    assert!(context.update("valid_var", CursedObject::String("updated".to_string())).is_ok());
    
    // Test error handling for invalid operations would need specific error conditions
    // This depends on the internal implementation, but basic operations should succeed
    let result = context.merge(&TemplateContext::new());
    assert!(result.is_ok());
}

#[test]
fn test_complex_nested_context_hierarchy() {
    // Create a complex hierarchy: grandparent -> parent -> child
    let grandparent = TemplateContext::new();
    grandparent.set("level", CursedObject::String("grandparent".to_string())).unwrap();
    grandparent.set("gp_var", CursedObject::String("grandparent_value".to_string())).unwrap();
    
    let parent = TemplateContext::with_parent(grandparent);
    parent.set("level", CursedObject::String("parent".to_string())).unwrap();
    parent.set("p_var", CursedObject::String("parent_value".to_string())).unwrap();
    
    let child = TemplateContext::with_parent(parent);
    child.set("level", CursedObject::String("child".to_string())).unwrap();
    child.set("c_var", CursedObject::String("child_value".to_string())).unwrap();
    
    // Test variable resolution through the hierarchy
    assert_eq!(child.get("level"), Some(CursedObject::String("child".to_string()))); // Shadowed
    assert_eq!(child.get("c_var"), Some(CursedObject::String("child_value".to_string())));
    assert_eq!(child.get("p_var"), Some(CursedObject::String("parent_value".to_string())));
    assert_eq!(child.get("gp_var"), Some(CursedObject::String("grandparent_value".to_string())));
    
    // Test contains through the hierarchy
    assert!(child.contains("level"));
    assert!(child.contains("c_var"));
    assert!(child.contains("p_var"));
    assert!(child.contains("gp_var"));
}
