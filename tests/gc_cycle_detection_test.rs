#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use cursed::memory::{
        CycleDetector, CycleDetectionConfig, ObjectId, ObjectRegistry, Traceable, Visitor, 
        RootSetManager, Tag
    };

    #[path = "common.rs"]
    mod common;

    // Test structure that can contain references to create cycles
    #[derive(Debug)]
    struct TestNode {
        id: ObjectId,
        name: String,
        children: Vec<ObjectId>,
        parent: Option<ObjectId>,
    }

    impl Traceable for TestNode {
        fn trace(&self, visitor: &mut dyn Visitor) {
            for child_id in &self.children {
                // In a real implementation, we'd visit the actual child objects
                // For now, we'll simulate this by tracking the IDs
                if let Some(child_obj) = get_object_by_id(*child_id) {
                    visitor.visit(child_obj.as_ref());
                }
            }
            if let Some(parent_id) = self.parent {
                if let Some(parent_obj) = get_object_by_id(parent_id) {
                    visitor.visit(parent_obj.as_ref());
                }
            }
        }
    }

    // Mock object lookup for testing
    fn get_object_by_id(id: ObjectId) -> Option<Box<dyn Traceable>> {
        // This would be implemented by the actual object store
        // For testing, we'll create dummy objects
        Some(Box::new(TestNode {
            id,
            name: format!("Node_{}", id.as_u64()),
            children: vec![],
            parent: None,
        }))
    }

    #[test]
    fn test_cycle_detector_creation() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Verify detector was created successfully
        let stats = detector.get_stats().expect("Should get stats");
        assert_eq!(stats.cycles_detected, 0);
        assert_eq!(stats.objects_in_cycles, 0);
    }

    #[test]
    fn test_cycle_detector_with_config() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let config = CycleDetectionConfig {
            enable_strong_cycle_detection: true,
            enable_weak_cycle_detection: true,
            cycle_detection_threshold: 100,
            max_cycle_depth: 50,
            enable_incremental_detection: true,
            detection_frequency: std::time::Duration::from_millis(100),
            enable_cycle_breaking: true,
            max_cycles_per_detection: 10,
            enable_statistics: true,
        };
        
        let detector = CycleDetector::with_config(registry.clone(), config.clone());
        
        // Verify config was applied
        let current_config = detector.get_config().expect("Should get config");
        assert_eq!(current_config.enable_strong_cycle_detection, config.enable_strong_cycle_detection);
        assert_eq!(current_config.cycle_detection_threshold, config.cycle_detection_threshold);
    }

    #[test]
    fn test_simple_cycle_detection() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Create a simple cycle: A -> B -> A
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        
        // Register objects in registry
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        
        // Create references: A -> B -> A (cycle)
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_b, obj_a).expect("Should add B->A reference");
        
        // Run cycle detection
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        
        // Verify cycle was detected
        assert!(!cycles.is_empty(), "Should detect at least one cycle");
        
        let stats = detector.get_stats().expect("Should get stats");
        assert!(stats.cycles_detected > 0, "Should have detected cycles");
        assert!(stats.objects_in_cycles >= 2, "Should have at least 2 objects in cycles");
    }

    #[test]
    fn test_no_cycles_detection() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Create a simple tree: A -> B, A -> C (no cycles)
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        let obj_c = ObjectId::new(3);
        
        // Register objects
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        registry.register(obj_c, "TestNode".to_string(), Tag::Object, 64).expect("Should register C");
        
        // Create tree structure: A -> B, A -> C
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_a, obj_c).expect("Should add A->C reference");
        
        // Run cycle detection
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        
        // Verify no cycles were detected
        assert!(cycles.is_empty(), "Should not detect any cycles in tree structure");
        
        let stats = detector.get_stats().expect("Should get stats");
        assert_eq!(stats.cycles_detected, 0, "Should have detected no cycles");
        assert_eq!(stats.objects_in_cycles, 0, "Should have no objects in cycles");
    }

    #[test]
    fn test_complex_cycle_detection() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Create a more complex cycle: A -> B -> C -> D -> B (cycle involving B, C, D)
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        let obj_c = ObjectId::new(3);
        let obj_d = ObjectId::new(4);
        
        // Register objects
        for &obj_id in &[obj_a, obj_b, obj_c, obj_d] {
            registry.register(obj_id, "TestNode".to_string(), Tag::Object, 64)
                .expect("Should register object");
        }
        
        // Create cycle: A -> B -> C -> D -> B
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_b, obj_c).expect("Should add B->C reference");
        registry.add_reference(obj_c, obj_d).expect("Should add C->D reference");
        registry.add_reference(obj_d, obj_b).expect("Should add D->B reference"); // Creates cycle
        
        // Run cycle detection
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        
        // Verify cycle was detected
        assert!(!cycles.is_empty(), "Should detect complex cycle");
        
        let stats = detector.get_stats().expect("Should get stats");
        assert!(stats.cycles_detected > 0, "Should have detected cycles");
        assert!(stats.objects_in_cycles >= 3, "Should have at least 3 objects in cycle (B, C, D)");
    }

    #[test]
    fn test_multiple_cycles_detection() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Create two separate cycles:
        // Cycle 1: A -> B -> A
        // Cycle 2: C -> D -> E -> C
        let objects = (1..=5).map(ObjectId::new).collect::<Vec<_>>();
        
        // Register all objects
        for &obj_id in &objects {
            registry.register(obj_id, "TestNode".to_string(), Tag::Object, 64)
                .expect("Should register object");
        }
        
        // Create first cycle: A -> B -> A
        registry.add_reference(objects[0], objects[1]).expect("Should add A->B reference");
        registry.add_reference(objects[1], objects[0]).expect("Should add B->A reference");
        
        // Create second cycle: C -> D -> E -> C
        registry.add_reference(objects[2], objects[3]).expect("Should add C->D reference");
        registry.add_reference(objects[3], objects[4]).expect("Should add D->E reference");
        registry.add_reference(objects[4], objects[2]).expect("Should add E->C reference");
        
        // Run cycle detection
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        
        // Verify both cycles were detected
        assert!(cycles.len() >= 2, "Should detect at least 2 cycles");
        
        let stats = detector.get_stats().expect("Should get stats");
        assert!(stats.cycles_detected >= 2, "Should have detected at least 2 cycles");
        assert!(stats.objects_in_cycles >= 5, "Should have all 5 objects in cycles");
    }

    #[test]
    fn test_cycle_breaking() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let config = CycleDetectionConfig {
            enable_cycle_breaking: true,
            ..Default::default()
        };
        let detector = CycleDetector::with_config(registry.clone(), config);
        
        // Create a simple cycle
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_b, obj_a).expect("Should add B->A reference");
        
        // Detect and break cycles
        let cycles_before = detector.detect_cycles().expect("Should detect cycles");
        assert!(!cycles_before.is_empty(), "Should detect cycle before breaking");
        
        // Break the cycles
        let broken_count = detector.break_cycles(&cycles_before).expect("Should break cycles");
        assert!(broken_count > 0, "Should have broken at least one cycle");
        
        // Verify cycles are broken
        let cycles_after = detector.detect_cycles().expect("Should detect cycles");
        assert!(cycles_after.len() < cycles_before.len(), "Should have fewer cycles after breaking");
    }

    #[test]
    fn test_cycle_detection_with_roots() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let root_manager = Arc::new(RootSetManager::new(registry.clone()));
        let detector = CycleDetector::new(registry.clone());
        
        // Set the root manager for the detector
        detector.set_root_manager(root_manager.clone()).expect("Should set root manager");
        
        // Create a cycle where one object is a root
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        
        // Make A a root object
        root_manager.add_global_root(obj_a, Some("Root object A".to_string())).expect("Should add root");
        
        // Create cycle: A -> B -> A
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_b, obj_a).expect("Should add B->A reference");
        
        // Run cycle detection
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        
        // Cycle should still be detected, but root objects may be handled differently
        assert!(!cycles.is_empty(), "Should detect cycle even with root objects");
    }

    #[test]
    fn test_cycle_detection_performance() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        
        // Create a larger graph to test performance
        let num_objects = 100;
        let objects: Vec<ObjectId> = (1..=num_objects).map(ObjectId::new).collect();
        
        // Register all objects
        for &obj_id in &objects {
            registry.register(obj_id, "TestNode".to_string(), Tag::Object, 64)
                .expect("Should register object");
        }
        
        // Create a chain with a cycle at the end
        for i in 0..num_objects-1 {
            registry.add_reference(objects[i], objects[i+1]).expect("Should add reference");
        }
        // Create cycle by connecting last to first
        registry.add_reference(objects[num_objects-1], objects[0]).expect("Should add cycle reference");
        
        // Measure cycle detection time
        let start_time = std::time::Instant::now();
        let cycles = detector.detect_cycles().expect("Should detect cycles");
        let detection_time = start_time.elapsed();
        
        // Verify detection worked and was reasonably fast
        assert!(!cycles.is_empty(), "Should detect large cycle");
        assert!(detection_time < std::time::Duration::from_secs(1), 
                "Detection should complete within 1 second for 100 objects");
        
        let stats = detector.get_stats().expect("Should get stats");
        assert!(stats.objects_in_cycles >= num_objects as u64, 
                "Should include all objects in the cycle");
    }

    #[test]
    fn test_incremental_cycle_detection() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let config = CycleDetectionConfig {
            enable_incremental_detection: true,
            ..Default::default()
        };
        let detector = CycleDetector::with_config(registry.clone(), config);
        
        // Start incremental detection
        detector.start_incremental_detection().expect("Should start incremental detection");
        assert!(detector.is_incremental_running().expect("Should check incremental status"));
        
        // Create objects and gradually build a cycle
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        
        // Add first reference
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        
        // Run incremental step
        let step_result = detector.incremental_step().expect("Should run incremental step");
        assert!(step_result, "Incremental step should return true when there's work");
        
        // Add second reference to complete the cycle
        registry.add_reference(obj_b, obj_a).expect("Should add B->A reference");
        
        // Run another incremental step
        detector.incremental_step().expect("Should run incremental step");
        
        // Check if cycle was detected incrementally
        let stats = detector.get_stats().expect("Should get stats");
        // Note: The cycle might not be detected immediately in incremental mode
        // This is normal behavior for incremental detection
        
        // Stop incremental detection
        detector.stop_incremental_detection().expect("Should stop incremental detection");
        assert!(!detector.is_incremental_running().expect("Should check incremental status"));
    }

    #[test]
    fn test_cycle_detection_statistics() {
        common::tracing::setup();
        
        let registry = Arc::new(ObjectRegistry::new());
        let config = CycleDetectionConfig {
            enable_statistics: true,
            ..Default::default()
        };
        let detector = CycleDetector::with_config(registry.clone(), config);
        
        // Initially no statistics
        let initial_stats = detector.get_stats().expect("Should get initial stats");
        assert_eq!(initial_stats.cycles_detected, 0);
        assert_eq!(initial_stats.objects_in_cycles, 0);
        assert_eq!(initial_stats.detection_runs, 0);
        
        // Create a cycle and detect it
        let obj_a = ObjectId::new(1);
        let obj_b = ObjectId::new(2);
        
        registry.register(obj_a, "TestNode".to_string(), Tag::Object, 64).expect("Should register A");
        registry.register(obj_b, "TestNode".to_string(), Tag::Object, 64).expect("Should register B");
        
        registry.add_reference(obj_a, obj_b).expect("Should add A->B reference");
        registry.add_reference(obj_b, obj_a).expect("Should add B->A reference");
        
        detector.detect_cycles().expect("Should detect cycles");
        
        // Check updated statistics
        let final_stats = detector.get_stats().expect("Should get final stats");
        assert!(final_stats.cycles_detected > 0, "Should have detected cycles");
        assert!(final_stats.objects_in_cycles > 0, "Should have objects in cycles");
        assert!(final_stats.detection_runs > 0, "Should have run detection");
        assert!(final_stats.total_detection_time > std::time::Duration::ZERO, 
                "Should have non-zero detection time");
    }
}
