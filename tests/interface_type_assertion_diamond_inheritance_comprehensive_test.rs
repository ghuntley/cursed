//! # Diamond Inheritance Interface Type Assertion Test
//!
//! This test verifies the correctness of interface type assertions when dealing with
//! diamond inheritance patterns, where a type implements multiple interfaces that
//! share a common ancestor.
//!
//! ## Key Test Scenarios
//!
//! 1. Asserting a concrete type through different interface paths
//! 2. Proper error handling when assertions fail in diamond inheritance
//! 3. Using the ? operator for clean error propagation across complex type hierarchies
//! 4. Visualizing type assertion paths in diamond inheritance relationships

use std::sync::Arc;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use cursed::codegen::llvm::jit::JitCompiler;
use cursed::parser::Parser;
use cursed::error::Error;
use tracing::{debug, info, warn, instrument};

// Import common test utilities
#[path = "common.rs"]
pub mod common;

use common::tracing::setup as init_tracing;
use common::timing::Timer;

/// Test interface type assertions with diamond inheritance patterns
#[test]
fn test_diamond_inheritance_type_assertions() {
    // Initialize tracing
    init_tracing();
    info!("Starting diamond inheritance type assertion test");
    let _timer = Timer::new("diamond_inheritance_type_assertion_test");
    
    // Create test code with diamond inheritance pattern
    let code = r#"
    vibe diamond_inheritance;
    
    // Error interface
    collab Error {
        slay error() tea;
    }
    
    // Type assertion error
    squad TypeAssertionError {
        expected tea,
        actual tea,
        path tea
    }
    
    slay (e TypeAssertionError) error() tea {
        return "Type assertion failed: expected " + e.expected + " but got " + e.actual + 
               (lowkey e.path != "" { return " (via " + e.path + ")"; } no cap { return ""; });
    }
    
    // Result type
    squad Result<T, E> {
        value T,
        err E,
        isOk lit
    }
    
    slay ok<T, E>(value T) Result<T, E> {
        return Result<T, E>{
            value: value,
            err: nofr as E,
            isOk: 1
        };
    }
    
    slay fail<T, E>(err E) Result<T, E> {
        return Result<T, E>{
            value: nofr as T,
            err: err,
            isOk: 0
        };
    }
    
    // Base interface for the diamond inheritance pattern
    collab Entity {
        slay id() tea;
        slay type() tea;
    }
    
    // Interfaces extending the base interface (middle layer)
    collab Physical extends Entity {
        slay position() tea;
        slay dimensions() tea;
    }
    
    collab Logical extends Entity {
        slay name() tea;
        slay description() tea;
    }
    
    // Interfaces extending both middle layer interfaces (diamond pattern)
    collab Asset extends Physical, Logical {
        slay value() normie;
        slay owner() tea;
    }
    
    collab Interactive extends Physical, Logical {
        slay interact(action tea) Result<tea, Error>;
        slay isEnabled() lit;
    }
    
    // Concrete implementation of the Asset interface
    squad Building {
        id_val tea,
        type_val tea,
        pos_x normie,
        pos_y normie,
        width normie,
        height normie,
        building_name tea,
        building_desc tea,
        building_value normie,
        building_owner tea
    }
    
    // Implement Entity methods
    slay (b Building) id() tea {
        return b.id_val;
    }
    
    slay (b Building) type() tea {
        return b.type_val;
    }
    
    // Implement Physical methods
    slay (b Building) position() tea {
        return "(" + b.pos_x + "," + b.pos_y + ")";
    }
    
    slay (b Building) dimensions() tea {
        return b.width + "x" + b.height;
    }
    
    // Implement Logical methods
    slay (b Building) name() tea {
        return b.building_name;
    }
    
    slay (b Building) description() tea {
        return b.building_desc;
    }
    
    // Implement Asset methods
    slay (b Building) value() normie {
        return b.building_value;
    }
    
    slay (b Building) owner() tea {
        return b.building_owner;
    }
    
    // Concrete implementation of the Interactive interface
    squad Device {
        id_val tea,
        type_val tea,
        pos_x normie,
        pos_y normie,
        width normie,
        height normie,
        device_name tea,
        device_desc tea,
        device_enabled lit,
        connection_status tea
    }
    
    // Implement Entity methods
    slay (d Device) id() tea {
        return d.id_val;
    }
    
    slay (d Device) type() tea {
        return d.type_val;
    }
    
    // Implement Physical methods
    slay (d Device) position() tea {
        return "(" + d.pos_x + "," + d.pos_y + ")";
    }
    
    slay (d Device) dimensions() tea {
        return d.width + "x" + d.height;
    }
    
    // Implement Logical methods
    slay (d Device) name() tea {
        return d.device_name;
    }
    
    slay (d Device) description() tea {
        return d.device_desc;
    }
    
    // Implement Interactive methods
    slay (d Device) interact(action tea) Result<tea, Error> {
        lowkey !d.device_enabled {
            return fail<tea, Error>(TypeAssertionError{
                expected: "enabled device",
                actual: "disabled device",
                path: ""
            });
        }
        
        lowkey action == "connect" {
            return ok<tea, Error>("Connected to device " + d.device_name);
        } lowkey action == "disconnect" {
            return ok<tea, Error>("Disconnected from device " + d.device_name);
        } no cap {
            return fail<tea, Error>(TypeAssertionError{
                expected: "valid action",
                actual: action,
                path: ""
            });
        }
    }
    
    slay (d Device) isEnabled() lit {
        return d.device_enabled;
    }
    
    // Type that implements both Asset and Interactive interfaces
    squad SmartDevice {
        id_val tea,
        type_val tea,
        pos_x normie,
        pos_y normie,
        width normie,
        height normie,
        smart_name tea,
        smart_desc tea,
        smart_value normie,
        smart_owner tea,
        smart_enabled lit,
        smart_status tea
    }
    
    // Implement Entity methods
    slay (s SmartDevice) id() tea {
        return s.id_val;
    }
    
    slay (s SmartDevice) type() tea {
        return s.type_val;
    }
    
    // Implement Physical methods
    slay (s SmartDevice) position() tea {
        return "(" + s.pos_x + "," + s.pos_y + ")";
    }
    
    slay (s SmartDevice) dimensions() tea {
        return s.width + "x" + s.height;
    }
    
    // Implement Logical methods
    slay (s SmartDevice) name() tea {
        return s.smart_name;
    }
    
    slay (s SmartDevice) description() tea {
        return s.smart_desc;
    }
    
    // Implement Asset methods
    slay (s SmartDevice) value() normie {
        return s.smart_value;
    }
    
    slay (s SmartDevice) owner() tea {
        return s.smart_owner;
    }
    
    // Implement Interactive methods
    slay (s SmartDevice) interact(action tea) Result<tea, Error> {
        lowkey !s.smart_enabled {
            return fail<tea, Error>(TypeAssertionError{
                expected: "enabled smart device",
                actual: "disabled smart device",
                path: ""
            });
        }
        
        lowkey action == "connect" {
            return ok<tea, Error>("Connected to smart device " + s.smart_name);
        } lowkey action == "disconnect" {
            return ok<tea, Error>("Disconnected from smart device " + s.smart_name);
        } lowkey action == "status" {
            return ok<tea, Error>("Smart device status: " + s.smart_status);
        } no cap {
            return fail<tea, Error>(TypeAssertionError{
                expected: "valid smart action",
                actual: action,
                path: ""
            });
        }
    }
    
    slay (s SmartDevice) isEnabled() lit {
        return s.smart_enabled;
    }
    
    // Functions for testing type assertions across the diamond hierarchy
    
    // Test assertion through base interface
    slay getEntityId(entity Entity) Result<tea, Error> {
        return ok<tea, Error>(entity.id());
    }
    
    // Test assertion through middle layer interfaces
    slay getPhysicalLocation(entity Entity) Result<tea, Error> {
        sus physical = entity.(Physical)?;
        return ok<tea, Error>(physical.position());
    }
    
    slay getLogicalName(entity Entity) Result<tea, Error> {
        sus logical = entity.(Logical)?;
        return ok<tea, Error>(logical.name());
    }
    
    // Test assertion through diamond interfaces
    slay getAssetValue(entity Entity) Result<normie, Error> {
        // Try multiple paths to Asset
        
        // Path 1: Direct asset assertion
        sus asset1, assetOk1 = entity.(Asset);
        lowkey assetOk1 {
            vibez.spill("Direct Asset assertion succeeded");
            return ok<normie, Error>(asset1.value());
        }
        
        // Path 2: First as Physical, then as Asset
        sus physical, physicalOk = entity.(Physical);
        lowkey physicalOk {
            sus asset2 = physical.(Asset)?;
            vibez.spill("Physical->Asset assertion path succeeded");
            return ok<normie, Error>(asset2.value());
        }
        
        // Path 3: First as Logical, then as Asset
        sus logical, logicalOk = entity.(Logical);
        lowkey logicalOk {
            sus asset3 = logical.(Asset)?;
            vibez.spill("Logical->Asset assertion path succeeded");
            return ok<normie, Error>(asset3.value());
        }
        
        // If all paths failed, return error
        return fail<normie, Error>(TypeAssertionError{
            expected: "Asset",
            actual: entity.type(),
            path: "all paths tried"
        });
    }
    
    slay interactWithEntity(entity Entity, action tea) Result<tea, Error> {
        // Try multiple paths to Interactive
        
        // Path 1: Direct interactive assertion
        sus interactive, interactiveOk = entity.(Interactive);
        lowkey interactiveOk {
            vibez.spill("Direct Interactive assertion succeeded");
            return interactive.interact(action);
        }
        
        // Path 2: First as Physical, then as Interactive
        sus physical, physicalOk = entity.(Physical);
        lowkey physicalOk {
            sus interactive2 = physical.(Interactive)?;
            vibez.spill("Physical->Interactive assertion path succeeded");
            return interactive2.interact(action);
        }
        
        // Path 3: First as Logical, then as Interactive  
        sus logical, logicalOk = entity.(Logical);
        lowkey logicalOk {
            sus interactive3 = logical.(Interactive)?;
            vibez.spill("Logical->Interactive assertion path succeeded");
            return interactive3.interact(action);
        }
        
        // If all paths failed, return error
        return fail<tea, Error>(TypeAssertionError{
            expected: "Interactive",
            actual: entity.type(),
            path: "all paths tried"
        });
    }
    
    // Complex function that tries multiple assertion paths
    slay processEntity(entity Entity) Result<tea, Error> {
        vibez.spill("\nProcessing entity: " + entity.id() + " of type " + entity.type());
        
        // Check if it's an asset
        sus assetResult = getAssetValue(entity);
        lowkey assetResult.isOk {
            vibez.spill("Entity is an Asset with value: " + assetResult.value);
        } no cap {
            vibez.spill("Entity is not an Asset: " + assetResult.err.error());
        }
        
        // Check if it's interactive
        sus interactResult = interactWithEntity(entity, "status");
        lowkey interactResult.isOk {
            vibez.spill("Entity is Interactive: " + interactResult.value);
        } no cap {
            vibez.spill("Entity is not Interactive: " + interactResult.err.error());
        }
        
        return ok<tea, Error>("Processed entity " + entity.id());
    }
    
    // Main test function
    slay diamond_inheritance() {
        // Create test objects
        sus building = Building{
            id_val: "B001",
            type_val: "Building",
            pos_x: 10,
            pos_y: 20,
            width: 100,
            height: 200,
            building_name: "Office Building",
            building_desc: "A tall office building",
            building_value: 1000000,
            building_owner: "Acme Corp"
        };
        
        sus device = Device{
            id_val: "D001",
            type_val: "Device",
            pos_x: 5,
            pos_y: 15,
            width: 30,
            height: 20,
            device_name: "Printer",
            device_desc: "LaserJet Printer",
            device_enabled: 1,
            connection_status: "online"
        };
        
        sus smartDevice = SmartDevice{
            id_val: "SD001",
            type_val: "SmartDevice",
            pos_x: 25,
            pos_y: 35,
            width: 10,
            height: 15,
            smart_name: "Smart Thermostat",
            smart_desc: "IoT-enabled temperature controller",
            smart_value: 250,
            smart_owner: "Building Maintenance",
            smart_enabled: 1,
            smart_status: "active"
        };
        
        // Test Entity interface - all should succeed
        vibez.spill("\n--- Entity Interface Tests ---");
        
        sus buildingId = getEntityId(building);
        vibez.spill("Building ID: " + buildingId.value);
        
        sus deviceId = getEntityId(device);
        vibez.spill("Device ID: " + deviceId.value);
        
        sus smartDeviceId = getEntityId(smartDevice);
        vibez.spill("Smart Device ID: " + smartDeviceId.value);
        
        // Test Physical interface - all should succeed
        vibez.spill("\n--- Physical Interface Tests ---");
        
        sus buildingPos = getPhysicalLocation(building);
        vibez.spill("Building position: " + buildingPos.value);
        
        sus devicePos = getPhysicalLocation(device);
        vibez.spill("Device position: " + devicePos.value);
        
        sus smartDevicePos = getPhysicalLocation(smartDevice);
        vibez.spill("Smart Device position: " + smartDevicePos.value);
        
        // Test Logical interface - all should succeed
        vibez.spill("\n--- Logical Interface Tests ---");
        
        sus buildingName = getLogicalName(building);
        vibez.spill("Building name: " + buildingName.value);
        
        sus deviceName = getLogicalName(device);
        vibez.spill("Device name: " + deviceName.value);
        
        sus smartDeviceName = getLogicalName(smartDevice);
        vibez.spill("Smart Device name: " + smartDeviceName.value);
        
        // Test Asset interface
        vibez.spill("\n--- Asset Interface Tests ---");
        
        // Building implements Asset - should succeed
        sus buildingValue = getAssetValue(building);
        lowkey buildingValue.isOk {
            vibez.spill("Building value: " + buildingValue.value);
        } no cap {
            vibez.spill("Error getting building value: " + buildingValue.err.error());
        }
        
        // Device doesn't implement Asset - should fail
        sus deviceValue = getAssetValue(device);
        lowkey deviceValue.isOk {
            vibez.spill("Device value: " + deviceValue.value);
        } no cap {
            vibez.spill("Error getting device value: " + deviceValue.err.error());
        }
        
        // SmartDevice implements Asset - should succeed
        sus smartDeviceValue = getAssetValue(smartDevice);
        lowkey smartDeviceValue.isOk {
            vibez.spill("Smart Device value: " + smartDeviceValue.value);
        } no cap {
            vibez.spill("Error getting smart device value: " + smartDeviceValue.err.error());
        }
        
        // Test Interactive interface
        vibez.spill("\n--- Interactive Interface Tests ---");
        
        // Building doesn't implement Interactive - should fail
        sus buildingInteract = interactWithEntity(building, "status");
        lowkey buildingInteract.isOk {
            vibez.spill("Building interaction: " + buildingInteract.value);
        } no cap {
            vibez.spill("Error interacting with building: " + buildingInteract.err.error());
        }
        
        // Device implements Interactive - should succeed
        sus deviceInteract = interactWithEntity(device, "connect");
        lowkey deviceInteract.isOk {
            vibez.spill("Device interaction: " + deviceInteract.value);
        } no cap {
            vibez.spill("Error interacting with device: " + deviceInteract.err.error());
        }
        
        // SmartDevice implements Interactive - should succeed
        sus smartDeviceInteract = interactWithEntity(smartDevice, "status");
        lowkey smartDeviceInteract.isOk {
            vibez.spill("Smart Device interaction: " + smartDeviceInteract.value);
        } no cap {
            vibez.spill("Error interacting with smart device: " + smartDeviceInteract.err.error());
        }
        
        // Test complex processing with multiple assertion paths
        vibez.spill("\n--- Complex Processing Tests ---");
        
        sus buildingProcess = processEntity(building);
        sus deviceProcess = processEntity(device);
        sus smartDeviceProcess = processEntity(smartDevice);
        
        vibez.spill("\nAll diamond inheritance type assertion tests completed");
    }
    "#;
    
    // Parse the code
    let mut parser = Parser::new(code);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => panic!("Failed to parse program: {}", e),
    };
    
    // Create JIT compiler
    let mut jit = JitCompiler::new("diamond_inheritance_type_assertion_test");
    
    // Configure JIT for proper interface type assertions
    jit.set_enable_debug(true);
    jit.set_enable_type_debugging(true);
    jit.set_enable_type_checks(true);
    jit.set_enable_interface_registry(true);
    jit.set_enable_enhanced_error_reporting(true);
    
    // Add enhanced error handlers
    jit.add_runtime_error_handlers();
    
    // Compile the program
    match jit.compile(&program) {
        Ok(_) => {
            info!("Successfully compiled diamond inheritance type assertion test program");
        },
        Err(e) => {
            panic!("Failed to compile diamond inheritance type assertion test program: {}", e);
        }
    };
    
    // Run the program
    let result = jit.run();
    
    // Check that execution completed successfully
    assert!(result.is_ok(), "Diamond inheritance type assertion test execution failed");
    
    info!("Diamond inheritance type assertion test completed successfully");
}