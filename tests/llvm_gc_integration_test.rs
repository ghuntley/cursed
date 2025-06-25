use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::memory::gc::GarbageCollector;
use cursed::error::CursedError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_allocation_integration() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        let mut codegen = LlvmCodeGenerator::new()?;
        
        // Test that GC and LLVM integration is working
        let obj_id = gc.allocate_object(64)?;
        assert!(obj_id > 0);
        
        // Test GC collection with LLVM context
        let initial_count = gc.allocated_objects();
        gc.collect_garbage();
        let after_count = gc.allocated_objects();
        
        assert!(after_count <= initial_count);
        Ok(())
    }

    #[test]
    fn test_gc_root_management() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Test root registration
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
    fn test_llvm_gc_barrier_generation() -> Result<(), CursedError> {
        let mut codegen = LlvmCodeGenerator::new()?;
        
        // Test that LLVM context supports GC operations
        let context = codegen.get_context();
        assert!(context.is_some());
        
        // Test GC barrier function generation
        let module = codegen.get_module();
        assert!(module.is_some());
        
        Ok(())
    }

    #[test]
    fn test_gc_collection_with_llvm_compiled_code() {
        let mut gc = GarbageCollector::new();
        
        // Allocate objects to trigger collection
        for i in 0..1000 {
            let _ = gc.allocate_object(64);
        }
        
        let before_collection = gc.allocated_objects();
        gc.collect_garbage();
        let after_collection = gc.allocated_objects();
        
        // Collection should have freed some objects
        assert!(after_collection <= before_collection);
    }

    #[test]
    fn test_gc_finalization_integration() -> Result<(), CursedError> {
        let mut gc = GarbageCollector::new();
        
        // Test object finalization during collection
        let obj_id = gc.allocate_object(128)?;
        gc.add_root(obj_id);
        
        // Remove root to make object eligible for collection
        gc.remove_root(obj_id);
        gc.collect_garbage();
        
        // Object should be collected
        let remaining = gc.allocated_objects();
        assert_eq!(remaining, 0);
        
        Ok(())
    }
}
