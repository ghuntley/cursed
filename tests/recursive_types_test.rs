//! Comprehensive tests for recursive type definitions in CURSED.
//!
//! This test suite covers all aspects of recursive type support:
//! - Direct recursive structs (linked lists, trees)
//! - Mutually recursive types
//! - Complex recursive scenarios
//! - LLVM code generation for recursive types
//! - Memory layout correctness
//! - Garbage collection integration

use cursed::core::type_checker::  {Type, TypeChecker}
use cursed::core::recursive_types::::RecursiveTypeRegistry, RecursiveTypeResolver;
use cursed::error::Error;
use std::collections::HashMap;

// Common test tracing setup
mod common;

#[test]
fn test_direct_recursive_struct() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define a recursive linked list node: Node {value: int, next: *Node}
    let node_type = Type::Struct()
        Node.to_string()
        vec![Box::new(Type::Normie), // value field
            Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(Node.to_string(), // next field]     {Type::Pointer(inner) => {// The inner type should be a pointer to break the cycle
                    match inner.as_ref()     {Type::Unknown // Was Named(name) => assert_eq!(name,  Node,);
                        Type::Pointer(_) => {// This can happen when cycle detection creates a pointer to break recursion
                            // This is actually correct behavior}
                        _ => panic!(Expected:  Named type or Pointer type for recursive reference),":  pointer type for next "field),}
        _ => panic!(":  Struct type),"}
#[test]
fn test_binary_tree_recursive_struct() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define a binary tree node: TreeNode {value: int, left: TreeNode, right: *TreeNode}
    let tree_node_type = Type::Struct()
         TreeNode.to_string()
        vec![Box::new(Type::Normie), // value field
            Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(TreeNode.to_string(), // left field 
            Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(TreeNode.to_string(), // right field]   {match field.as_ref()     {Type::Pointer(inner) => {match inner.as_ref()     {Type::Unknown // Was Named(name) => assert_eq!(name,  TreeNode,);
                            _ => panic!(Expected "reference),}
                    _ => panic!("Expected "}
        _ => panic!("Expected:  Struct "}
#[test]
fn test_mutually_recursive_types() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define mutually recursive types:
    // A {b_field: *B}
    // B {a_field: *A}
    let type_a = Type::Struct()
         A .to_string()
        vec![Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(B .to_string()],);
    registry.register_type(A ".to_string(), type_a).unwrap();".to_string(), type_b).unwrap()
    // Check cycle detection
    let cycles = registry.detect_cycles()
    assert!(!cycles.is_empty(), Should detect cycles in mutually recursive , types)
    
    // Both types should be resolvable;
    let resolved_a = registry.resolve_type(A).unwrap();
    let resolved_b = registry.resolve_type("B.unwrap()
    // Verify the resolved types maintain their structure
    match resolved_a     {Type::Struct(name, _) => assert_eq!(name,  A),
        _ => panic!("A),"}
    
    match resolved_b        {Type::Struct(name, _) => assert_eq!(name,  "Expected ":  Struct type for B),")
    // Check resolution order
    let resolution_order = registry.get_resolution_order().unwrap()
    assert!(!resolution_order.is_empty()
    
    // All types should be resolvable
    for type_name in &[Graph,  Node,  Edge   {
        let resolved = registry.resolve_type(type_name).unwrap()
        match resolved     {Type::Struct(name, _) => assert_eq!(&name, type_name),}
            _ => panic!("}
#[test]
fn test_forward_declarations() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Add forward declaration before definition
    registry.add_forward_declaration(Node.to_string()
    
    // Try to resolve before definition
    let resolved = registry.resolve_type(Node).unwrap();
    assert_eq!(resolved, Type::Unknown // Was Named(Node.to_string();
    
    // Now define the actual type
    let node_type = Type::Struct()
         Node.to_string()
        vec![Box::new(Type::Normie),
            Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(Node.to_string(),]
fn test_type_dependency_analysis() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Test complex dependency chain;
    let type_a = Type::Struct(A.to_string(), vec![Box::new(Type::Normie]
fn test_recursive_type_memory_safety() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Test that recursive types use pointers for memory safety
    let safe_recursive_type = Type::Struct()
         SafeNode.to_string()
        vec![Box::new(Type::Normie), // data
            Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(SafeNode.to_string(), // next],);;
    registry.register_type(A ".to_string(), type_a).unwrap();
    registry.register_type(")
    // Detect cycles
    let cycles = registry.detect_cycles()
    assert!(!cycles.is_empty()
    
    // Should be able to resolve all types despite cycles
    for type_name in &[A  , BC, ",      {let resolved = registry.resolve_type(type_name)
        assert!(resolved.is_ok(), "}
#[test]
fn test_generic_recursive_types() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define a generic recursive type: List[T] {value: T, next: *List[T]}
    let list_type = Type::Struct()
         List.to_string()
        vec![Box::new(Type::TypeParam(T.to_string(), // value: T 
            Box::new(Type::Pointer(Box::new(Type::Struct()
                 List.to_string()
                vec![Box::new(Type::TypeParam("T.to_string()] 
fn test_indirect_recursion() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define indirect recursion through interfaces:
    // Container {items: []Item}
    // Item {container: *Container}
    let item_type = Type::Struct()
         Item.to_string()
        vec![Box::new(Type::Pointer(Box::new(Type::Unknown // Was Named(Container.to_string()]
fn test_recursive_type_resolution_order() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut registry = RecursiveTypeRegistry::new()
    
    // Define a dependency chain: D -> C -> B -> A (where A is independent);
    let type_a = Type::Struct(A .to_string(), vec![Box::new(Type::Normie],)
    // Register in random order
    registry.register_type(D.to_string(), type_d).unwrap();
    registry.register_type(A.to_string(), type_a).unwrap();"
    registry.register_type(
    
    // Get resolution order
    let order = registry.get_resolution_order().unwrap()
    
    // Verify dependencies are resolved in correct order;
    let pos_a = order.iter().position(|x| x ==  A).unwrap();
    let pos_b = order.iter().position(|x| x ==  "B.unwrap();
    let pos_c = order.iter().position(|x| x ==  "
    let pos_d = order.iter().position(|x| x ==  D).unwrap();
    
    assert!(pos_a < pos_b, ")
    assert!(pos_b < pos_c, "B should come before , C)"C should come before D ")"});)