vibe main;

// Interface Type Assertion Benchmark Program
// This file contains benchmarking code for interface type assertions
// with different inheritance patterns.

// Base interface at the top of all hierarchies
collab BaseObject {
    slay id() lit;
    slay name() tea;
}

// Simple inheritance pattern
collab SimpleInterface {
    slay doWork() tea;
}

// Diamond inheritance pattern interfaces
collab InterfaceA {
    slay methodA() tea;
}

collab InterfaceB {
    slay methodB() tea;
}

// Deep hierarchy interfaces
collab Level1 {
    slay level1Method() tea;
}

collab Level2 {
    slay level2Method() tea;
}

collab Level3 {
    slay level3Method() tea;
}

collab Level4 {
    slay level4Method() tea;
}

collab Level5 {
    slay level5Method() tea;
}

// Concrete types implementing the interfaces
squad SimpleType {
    identifier lit,
    type_name tea
}

// Implement BaseObject for SimpleType
slay (s SimpleType) id() lit {
    return s.identifier;
}

slay (s SimpleType) name() tea {
    return s.type_name;
}

// Implement SimpleInterface for SimpleType
slay (s SimpleType) doWork() tea {
    return "Simple work done by " + s.type_name;
}

// Diamond pattern implementation
squad DiamondType {
    identifier lit,
    type_name tea,
    dataA tea,
    dataB tea
}

// Implement BaseObject for DiamondType
slay (d DiamondType) id() lit {
    return d.identifier;
}

slay (d DiamondType) name() tea {
    return d.type_name;
}

// Implement InterfaceA for DiamondType
slay (d DiamondType) methodA() tea {
    return "Method A: " + d.dataA;
}

// Implement InterfaceB for DiamondType
slay (d DiamondType) methodB() tea {
    return "Method B: " + d.dataB;
}

// Deep hierarchy implementation
squad DeepType {
    identifier lit,
    type_name tea,
    level_data tea
}

// Implement BaseObject for DeepType
slay (d DeepType) id() lit {
    return d.identifier;
}

slay (d DeepType) name() tea {
    return d.type_name;
}

// Implement all level interfaces for DeepType
slay (d DeepType) level1Method() tea {
    return d.level_data + " - Level 1";
}

slay (d DeepType) level2Method() tea {
    return d.level_data + " - Level 2";
}

slay (d DeepType) level3Method() tea {
    return d.level_data + " - Level 3";
}

slay (d DeepType) level4Method() tea {
    return d.level_data + " - Level 4";
}

slay (d DeepType) level5Method() tea {
    return d.level_data + " - Level 5";
}

// Benchmark functions

// Benchmark simple type assertions
slay benchmarkSimpleAssertions(iterations lit) tea {
    sus start = vibe.time();
    
    // Create test objects
    sus simpleObj = SimpleType{identifier: 1, type_name: "SimpleBenchmark"};
    sus obj BaseObject = simpleObj;
    
    // Perform assertions in a loop
    periodt i := 0; i < iterations; i++ {
        sus simple, ok = obj.(SimpleType);
        if ok {
            // Use the object to prevent optimization
            if simple.identifier != 1 {
                return "Error: wrong id";
            }
        } else {
            return "Error: type assertion failed";
        }
        
        // Also test interface assertion
        sus iface, ifaceOk = obj.(SimpleInterface);
        if ifaceOk {
            // Use the interface to prevent optimization
            sus result = iface.doWork();
            if result == "" {
                return "Error: empty result";
            }
        } else {
            return "Error: interface assertion failed";
        }
    }
    
    sus end = vibe.time();
    sus duration = end - start;
    
    return "Simple assertions: " + iterations + " iterations in " + duration + "ms";
}

// Benchmark diamond inheritance pattern assertions
slay benchmarkDiamondAssertions(iterations lit) tea {
    sus start = vibe.time();
    
    // Create test object
    sus diamondObj = DiamondType{
        identifier: 2, 
        type_name: "DiamondBenchmark",
        dataA: "A data",
        dataB: "B data"
    };
    
    // Create interface references
    sus objBase BaseObject = diamondObj;
    sus objA InterfaceA = diamondObj;
    sus objB InterfaceB = diamondObj;
    
    // Perform assertions in a loop
    periodt i := 0; i < iterations; i++ {
        // Test BaseObject to concrete
        sus d1, ok1 = objBase.(DiamondType);
        if !ok1 {
            return "Error: base to concrete assertion failed";
        }
        
        // Test InterfaceA to concrete
        sus d2, ok2 = objA.(DiamondType);
        if !ok2 {
            return "Error: A to concrete assertion failed";
        }
        
        // Test InterfaceB to concrete
        sus d3, ok3 = objB.(DiamondType);
        if !ok3 {
            return "Error: B to concrete assertion failed";
        }
        
        // Test cross-interface assertions (should fail)
        sus _, ok4 = objA.(InterfaceB);
        if ok4 {
            return "Error: A to B assertion should fail";
        }
    }
    
    sus end = vibe.time();
    sus duration = end - start;
    
    return "Diamond assertions: " + iterations + " iterations in " + duration + "ms";
}

// Benchmark deep hierarchy assertions
slay benchmarkDeepAssertions(iterations lit) tea {
    sus start = vibe.time();
    
    // Create test object
    sus deepObj = DeepType{
        identifier: 3,
        type_name: "DeepBenchmark",
        level_data: "Deep data"
    };
    
    // Create interface references
    sus objBase BaseObject = deepObj;
    sus objL1 Level1 = deepObj;
    sus objL3 Level3 = deepObj;
    sus objL5 Level5 = deepObj;
    
    // Perform assertions in a loop
    periodt i := 0; i < iterations; i++ {
        // Test from base to concrete
        sus d1, ok1 = objBase.(DeepType);
        if !ok1 {
            return "Error: base to concrete assertion failed";
        }
        
        // Test from level 1 to concrete
        sus d2, ok2 = objL1.(DeepType);
        if !ok2 {
            return "Error: L1 to concrete assertion failed";
        }
        
        // Test from middle level to concrete
        sus d3, ok3 = objL3.(DeepType);
        if !ok3 {
            return "Error: L3 to concrete assertion failed";
        }
        
        // Test from deepest level to concrete
        sus d5, ok5 = objL5.(DeepType);
        if !ok5 {
            return "Error: L5 to concrete assertion failed";
        }
        
        // Test up and down the hierarchy
        sus l1, ok6 = objL5.(Level1);
        if !ok6 {
            return "Error: L5 to L1 assertion failed";
        }
        
        sus base, ok7 = objL3.(BaseObject);
        if !ok7 {
            return "Error: L3 to base assertion failed";
        }
    }
    
    sus end = vibe.time();
    sus duration = end - start;
    
    return "Deep hierarchy assertions: " + iterations + " iterations in " + duration + "ms";
}

// Run all benchmarks
slay main() {
    // Define iteration counts
    sus iterations = 10000;
    
    // Run benchmarks
    vibez.spill("Starting interface type assertion benchmarks...");
    
    vibez.spill(benchmarkSimpleAssertions(iterations));
    vibez.spill(benchmarkDiamondAssertions(iterations));
    vibez.spill(benchmarkDeepAssertions(iterations));
    
    vibez.spill("Benchmarks complete.");
}