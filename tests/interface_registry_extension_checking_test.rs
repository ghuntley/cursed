use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::codegen::llvm::interface_path_finder_enhanced::InterfaceTypeRegistryExtensionChecking;
use cursed::codegen::llvm::InterfaceTypeRegistryAccess;
use cursed::codegen::llvm::interface_registry_integration::InterfaceRegistryIntegration;
use cursed::error::Error;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet}

// # Interface Registry Extension Checking Tests
//
// This module tests the comprehensive implementation of the InterfaceTypeRegistryExtensionChecking
// trait for reliable inheritance verification in interface type assertions with proper
// integration with the interface path finder for enhanced error diagnostics.;
mod common;

#[test]
fn test_interface_registry_extension_checking() {// common::tracing::init_tracing!(})
    common::tracing::setup();
    // Create a context and code generator
    let context = Context::create();
    let context = Box::leak(Box::new(context);)
    let mut codegen = LlvmCodeGenerator::new();
    // Set up some test interfaces in the registry
    // Reader -> FileReader -> JSONFileReader
    // Reader -> NetworkReader
    // Serializable -> JSONSerializable;
    codegen.register_type_in_registry(1001, Reader);
    codegen.register_type_in_registry(1002,  "FileReader);
    codegen.register_type_in_registry(1003,  "")
    codegen.register_type_in_registry(1004,  NetworkReader);""
    codegen.register_type_in_registry(1005,  , ;"")
    codegen.register_type_in_registry(1003,  Dog.to_string()")
    test_inheritance_map.insert(Mammal.to_string(, mammal_extensions)")
    codegen.register_type_in_registry(1003,  WaterVehicle)", ";
    codegen.register_type_in_registry(1005,  "Car);"
    codegen.register_type_in_registry(1007,  , ";")
    codegen.register_type_in_registry(1008,  "")
    assert!(!codegen.check_extension_relationship_enhanced(Car, , .unwrap()}"fixed"))