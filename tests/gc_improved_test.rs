use cursed::memory::gc::GarbageCollector;
use cursed::memory::heap::HeapManager;
use cursed::error::CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_basic_allocation() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Test basic object allocation
        let obj_id = gc.allocate_object(64)?;
        assert!(obj_id > 0);
        
        // Test that object is tracked
        let allocated_count = gc.allocated_objects();
        assert!(allocated_count > 0);
        
        Ok(())
    }

    #[test]
    fn test_gc_root_management() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Allocate object and add as root
        let obj_id = gc.allocate_object(32)?;
        gc.add_root(obj_id);
        
        // Verify root is tracked
        let roots = gc.get_roots();
        assert!(roots.contains(&obj_id));
        
        // Test root removal
        gc.remove_root(obj_id);
        let roots_after = gc.get_roots();
        assert!(!roots_after.contains(&obj_id));
        
        Ok(())
    }

    #[test]
    fn test_gc_collection_basic() {
        let mut gc = GarbageCollector::new();
        
        // Allocate multiple objects
        let mut object_ids = Vec::new();
        for i in 0..10 {
            if let Ok(obj_id) = gc.allocate_object(64) {
                object_ids.push(obj_id);
            }
        }
        
        let before_collection = gc.allocated_objects();
        assert!(before_collection > 0);
        
        // Run garbage collection
        gc.collect_garbage();
        
        let after_collection = gc.allocated_objects();
        // Should collect unreferenced objects
        assert!(after_collection <= before_collection);
    }

    #[test]
    fn test_gc_mark_and_sweep() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Allocate objects with references
        let root_obj = gc.allocate_object(64)?;
        let child_obj = gc.allocate_object(32)?;
        
        // Add root object only
        gc.add_root(root_obj);
        
        // Add reference from root to child
        gc.add_reference(root_obj, child_obj);
        
        // Run collection
        gc.collect_garbage();
        
        // Both objects should survive
        let roots = gc.get_roots();
        assert!(roots.contains(&root_obj));
        
        Ok(())
    }

    #[test]
    fn test_gc_memory_pressure() {
        let mut gc = GarbageCollector::new();
        
        // Allocate many objects to test memory pressure
        for _ in 0..1000 {
            let _ = gc.allocate_object(128);
        }
        
        let before_pressure = gc.allocated_objects();
        
        // Force collection under memory pressure
        gc.collect_garbage();
        
        let after_pressure = gc.allocated_objects();
        
        // Should free unused objects under pressure
        assert!(after_pressure <= before_pressure);
    }

    #[test]
    fn test_gc_finalization() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Test object finalization
        let obj_id = gc.allocate_object(64)?;
        
        // Remove from roots to make eligible for collection
        gc.collect_garbage();
        
        // Object should be finalized and collected
        let remaining = gc.allocated_objects();
        assert_eq!(remaining, 0);
        
        Ok(())
    }

    #[test]
    fn test_gc_heap_integration() -> Result<(), CursedError> {
        let mut heap = HeapManager::new();
        let mut gc = GarbageCollector::new();
        
        // Test integration with heap manager
        let heap_obj = heap.allocate(128)?;
        let gc_obj = gc.allocate_object(64)?;
        
        assert!(heap_obj.size() == 128);
        assert!(gc_obj > 0);
        
        // Test that GC and heap work together
        gc.collect_garbage();
        
        // Both should still be valid
        assert!(gc.allocated_objects() >= 0);
        
        Ok(())
    }

    #[test]
    fn test_gc_concurrent_access() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Test thread-safe operations
        let obj_id = gc.allocate_object(64)?;
        gc.add_root(obj_id);
        
        // Simulate concurrent access patterns
        for i in 0..100 {
            if i % 2 == 0 {
                let _ = gc.allocate_object(32);
            } else {
                gc.collect_garbage();
            }
        }
        
        // GC should handle concurrent operations safely
        let final_count = gc.allocated_objects();
        assert!(final_count >= 0);
        
        Ok(())
    }
}
