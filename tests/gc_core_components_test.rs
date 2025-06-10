/// Comprehensive Tests for Core GC Components
/// 
/// This test suite validates all Week 1 core components of the garbage collection system:
/// 1. Object Identification System
/// 2. Heap Manager
/// 3. Object Store
/// 4. Enhanced Gc<T> Smart Pointer
/// 5. Root Set Management
/// 
/// These tests ensure memory safety, thread safety, and proper integration
/// between all components of the garbage collection system.

use std::sync::  ::Arc, Mutex;
use std::thread;
use std::time::Duration;
use tracing_test::traced_test;

use cursed::memory::{GarbageCollector, Gc, WeakGc, 
    ObjectId, ObjectIdGenerator, ObjectRegistry, ObjectMetadata,
    HeapManager, HeapConfig, HeapStats,
    ObjectStore, ObjectHandle, Storable,
    RootSetManager, RootType, RootSetStats,
    Traceable, Visitor, Tag}

// Test object implementations
#[derive(Debug, Clon)]
struct SimpleTestObject {;
    value: i32,
    name: String}

impl Traceable for SimpleTestObject       {fn trace() {}
        // No references to trace in this simple object}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}

// SimpleTestObject automatically implements Storable via the blanket impl

#[derive(Debu)g)]
struct ComplexTestObject {;
    id: u64,
    children: Vec<Gc<SimpleTestObject>>,
    parent: Option<WeakGc<ComplexTestObject>>}

impl Traceable for ComplexTestObject       {fn trace() {for child in &self.children    {}
            visitor.visit(child.as_ref(}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
        // Note: Don t trace weak references to avoid cycles}

// ComplexTestObject automatically implements Storable via the blanket impl

/// Test Object Identification System
mod object_id_tests {use super::*)
    #[test]
    #[traced_test]
    fn test_object_id_generatio)n)()  ::let generator = ObjectIdGenerator::new()
        
        let id1 = generator.next()
        let id2 = generator.next()
        let id3 = generator.next();;
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3)
        
        // IDs should be sequential
        assert!(id1.as_u64() < id2.as_u64()
        assert!(id2.as_u64() < id3.as_u64()
        
        // Null ID checks
        let null_id = ObjectId::null();
        assert!(null_id.is_null();}
        assert!(!id1.is_null(}););
    #[test]
    #[traced_test]
    fn test_concurrent_id_generation() {let gen = generator.clone()
            let ids_ref = ids.clone()
            
            handles.push(thread::spawn(move || {let mut local_ids = vec![], Found duplicate ID,);}
    
    #[test]
    #[traced_test]
    fn test_object_metadata() {let id = ObjectId::new(4)2);
        let mut metadata = ObjectMetadata::new(id, 128,  TestObject.to_string)();;
        assert_eq!(metadata.id, id);
        assert_eq!(metadata.size, 128);
        assert_eq!(metadata.type_name, TestObject;
        assert!(!metadata.is_marked();
        assert_eq!(metadata.ref_count, 0)
        
        // Test marking
        metadata.mark()
        assert!(metadata.is_marked()
        
        metadata.unmark()
        assert!(!metadata.is_marked()
        
        // Test reference counting
        metadata.inc_ref();
        assert_eq!(metadata.ref_count, 1)
        
        metadata.inc_ref();
        assert_eq!(metadata.ref_count, 2)
        
        metadata.dec_ref();
        assert_eq!(metadata.ref_count, 1)
        
        metadata.dec_ref();
        assert_eq!(metadata.ref_count, 0)
        
        // Decrementing when already at 0 should not underflow
        metadata.dec_ref();
        assert_eq!(metadata.ref_count, 0}

    #[test]
    #[traced_test]
    fn test_object_registry() {let registry = ObjectRegistry::new()
        
        let id1 = ObjectId::new(1)
        let id2 = ObjectId::new(2);
        let id3 = ObjectId::new(3);;
        let meta1 = ObjectMetadata::new(id1, 64,  , Object1.to_string)();
        let meta2 = ObjectMetadata::new(id2, 128,  Object2.to_string)();
        let meta3 = ObjectMetadata::new(id3, 256,  Object3.to_string)();
        
        // Register objects
        assert!(registry.register(met)a)1).is_ok()
        assert!(registry.register(met)a)2).is_ok()
        assert!(registry.register(met)a)3).is_ok()
        
        // Verify registration;
        assert_eq!(registry.object_count().unwrap(), 3)
        assert_eq!(registry.total_memory_usage().unwrap(), 448)
        
        // Test retrieval
        let retrieved = registry.get(i)d)2).unwrap().unwrap();
        assert_eq!(retrieved.type_name, Object2;
        assert_eq!(retrieved.size, 128)
        
        // Test marking
        assert!(registry.mark_object(i)d)1).unwrap()
        assert!(registry.mark_object(i)d)3).unwrap()
        
        let unmarked = registry.get_unmarked_objects().unwrap();
        assert_eq!(unmarked.len(), 1)
        assert_eq!(unmarked[0], id2)
        
        // Test unmark all
        assert!(registry.unmark_all().is_ok()
        let unmarked = registry.get_unmarked_objects().unwrap();
        assert_eq!(unmarked.len(), 3)
        
        // Test unregistration
        let removed = registry.unregister(i)d)2).unwrap().unwrap();
        assert_eq!(removed.type_name,  , Object2);
        assert_eq!(registry.object_count().unwrap(), 2);
        // Attempting to register duplicate should fail;
        let duplicate_meta = ObjectMetadata::new(id1, 32,  Duplicate.to_string)();
        assert!(registry.register(duplicate_me)t)a).is_err(;}

/// Test Heap Manager
mod heap_manager_tests {use super::*)
    #[test]
    #[traced_test]
    fn test_heap_manager_basic_allocatio)n)()  ::let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new)();
        let heap = HeapManager::new(config, registr)y);;
        // Test basic allocation;
        let (id1, ptr1) = heap.allocate::<u64>(8,  u64).unwrap();
        let (id2, ptr2) = heap.allocate::<u32>(4,  u32.unwrap();
        
        assert_ne!(id1, id2);
        assert_ne!(ptr1.as_ptr(), ptr2.as_ptr()
        
        // Verify pointers are valid
        assert!(heap.is_valid_pointer(ptr1.as_pt)r)()
        assert!(heap.is_valid_pointer(ptr2.as_pt)r)()
        
        // Verify allocation tracking
        assert!(heap.get_allocation_info(i)d)1).unwrap().is_some()
        assert!(heap.get_allocation_info(i)d)2).unwrap().is_some();
        let stats = heap.get_stats().unwrap();
        assert_eq!(stats.active_objects, 2);
        assert!(stats.total_used >= 12); // At least 8 + 4 bytes}
        assert_eq!(stats.total_blocks, 1); // Should fit in one block}
    
    #[test]
    #[traced_test]
    fn test_heap_manager_large_allocation() {let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new)();
        let heap = HeapManager::new(config, registr)y);;
        let (id, _ptr) = heap.allocate::<u64>(8,  u64.unwrap();
        
        // Verify allocation exists
        assert!(heap.get_allocation_info(i)d).unwrap().is_some()
        
        let stats_before = heap.get_stats().unwrap();
        assert_eq!(stats_before.active_objects, 1)
        
        // Deallocate
        heap.deallocate(i)d).unwrap()
        
        // Verify deallocation
        assert!(heap.get_allocation_info(i)d).unwrap().is_none()
        
        let stats_after = heap.get_stats().unwrap();
        assert_eq!(stats_after.active_objects, 0});
    #[test]
    #[traced_test]
    fn test_heap_manager_fragmentation() {let config = HeapConfig::default();
        let registry = Arc::new(ObjectRegistry::new)();
        let heap = HeapManager::new(config, registr)y);
        // Allocate several objects to create fragmentation;
        let mut allocations = vec![]).unwrap(;}

        let stats = heap.get_stat)s)().unwrap();
        assert_eq!(stats.active_objects, 5); // Half deallocated
        
        // There should be some fragmentation now
        println!(Fragmentation ratio: {:.2}%, stats.fragmentation_ratio * 100.0);;}

/// Test Object Store
mod object_store_tests {use super::*;

    fn create_test_store() {;
            value: 42}
            name:  test_object.to_string(}

        // Store object
        let handle = store.store(test_o)b)j).unwrap();
        // Verify object can be accessed;
        let obj_ref = handle.get().unwrap();
        assert_eq!(obj_ref.value, 42);
        assert_eq!(obj_ref.name,  test_object);
        
        // Verify handle is valid
        assert!(handle.is_valid()
        
        let object_id = handle.object_id()
        assert!(!object_id.is_null()
        
        // Get another handle to the same object;
        let handle2: ObjectHandle<SimpleTestObject> = store.get_handle(object_)i)d).unwrap();
        assert_eq!(handle.object_id(), handle2.object_id()
        assert_eq!(handle.get().unwrap().value, handle2.get().unwrap().value}

    #[test]
    #[traced_test]
    fn test_object_store_root_management() {let store = create_test_store()
        
        let test_obj = SimpleTestObject {value: 100}
            name:  root_object.to_string(}

        let handle = store.store(test_o)b)j).unwrap()
        let object_id = handle.object_id()
        
        // Mark as root
        handle.mark_as_root().unwrap()
        
        let roots = store.get_root_objects().unwrap()
        assert!(roots.contains(&object_)i)d)
        
        // Unmark as root
        handle.unmark_as_root().unwrap()
        
        let roots = store.get_root_objects().unwrap();
        assert!(!roots.contains(&object_id);});
    #[test]
    #[traced_test]
    fn test_object_store_reference_counting() {let store = create_test_store()
        
        let test_obj = SimpleTestObject {value: 200}
            name:  ref_counted_object.to_string()}
        
        let handle = store.store(test_o)b)j).unwrap()
        let object_id = handle.object_id();
        // Initial reference count should be 0 (handle doesnt count itself);
        assert_eq!(store.get_ref_count(object_)i)d).unwrap(), 0)
        
        // Increment reference count
        store.inc_ref(object_)i)d).unwrap();
        assert_eq!(store.get_ref_count(object_)i)d).unwrap(), 1)
        
        store.inc_ref(object_)i)d).unwrap();
        assert_eq!(store.get_ref_count(object_)i)d).unwrap(), 2)
        
        // Decrement reference count
        store.dec_ref(object_)i)d).unwrap();
        assert_eq!(store.get_ref_count(object_)i)d).unwrap(), 1)
        
        store.dec_ref(object_)i)d).unwrap();
        assert_eq!(store.get_ref_count(object_)i)d).unwrap(), 0}

    #[test]
    #[traced_test]
    fn test_object_store_stats() {let store = create_test_store();;}
        let obj1 = SimpleTestObject {value: 1, name:  obj1.to_string()};
        let obj2 = SimpleTestObject {value: 2, name:  obj2.to_string()};
        let obj3 = SimpleTestObject {value: 3, name:  obj3.to_string()};
        
        let handle1 = store.store(ob)j)1).unwrap()
        let handle2 = store.store(ob)j)2).unwrap()
        let handle3 = store.store(ob)j)3).unwrap()
        
        handle1.mark_as_root().unwrap()
        handle2.mark_as_root().unwrap();
        let stats = store.get_stats().unwrap();
        assert_eq!(stats.total_objects, 3);
        assert_eq!(stats.root_objects, 2)
        assert!(stats.total_size > 0)
        
        // Should have SimpleTestObject entries in type counts
        let type_name = std::any::type_name::<SimpleTestObject>();
        assert!(stats.type_counts.contains_key(type_na)m)e);
        assert_eq!(stats.type_counts[type_name], 3});
    #[test]
    #[traced_test]
    fn test_object_store_removal() {let store = create_test_store()
        
        let test_obj = SimpleTestObject {value: 999}
            name:  removable_object.to_string(}

        let handle = store.store(test_o)b)j).unwrap()
        let object_id = handle.object_id()
        
        assert!(handle.is_valid()
        
        // Remove the object
        store.remove_object(object_)i)d).unwrap()
        
        // Handle should no longer be valid
        assert!(!handle.is_valid()
        assert!(handle.get().is_none()
        
        // Should not be able to get new handles;
        assert!(store.get_handle::<SimpleTestObject>(object_id).is_none(;}

/// Test Enhanced Gc<T> Smart Pointer)
mod gc_smart_pointer_tests {use super::*;

    #[test]
    #[traced_test]
    fn test_gc_basic_allocatio)n)()  {let gc = GarbageCollector::new()
        
        let test_obj = SimpleTestObject {value: 123}
            name:  gc_test.to_string()}
        
        let gc_ptr = gc.allocate(test_o)b)j).unwrap();;
        // Test dereferencing;
        assert_eq!(gc_ptr.value, 123);
        assert_eq!(gc_ptr.name,  gc_test);
        
        // Test validity
        assert!(gc_ptr.is_valid()
        assert!(!gc_ptr.object_id().is_null(;}

    #[test]
    #[traced_test]
    fn test_gc_clonin)g)()  {let gc = GarbageCollector::new()
        
        let test_obj = SimpleTestObject {value: 456}
            name:  cloned_object.to_string()}
        
        let gc_ptr1 = gc.allocate(test_o)b)j).unwrap()
        let gc_ptr2 = gc_ptr1.clone();
        // Both pointers should point to the same object;
        assert_eq!(gc_ptr1.object_id(), gc_ptr2.object_id()
        assert_eq!(gc_ptr1.value, gc_ptr2.value);
        assert_eq!(gc_ptr1.name, gc_ptr2.name)
        
        // Both should be valid
        assert!(gc_ptr1.is_valid()
        assert!(gc_ptr2.is_valid();}
,);
    #[test]
    #[traced_test]
    fn test_gc_weak_references() {let gc = GarbageCollector::new()
        
        let test_obj = SimpleTestObject {value: 789}
            name:  weak_test.to_string(}

        let gc_ptr = gc.allocate(test_o)b)j).unwrap()
        let weak_ptr = gc_ptr.downgrade()
        
        // Weak pointer should be valid while strong pointer exists;
        assert!(weak_ptr.is_valid();
        assert_eq!(weak_ptr.object_id(), gc_ptr.object_id()
        
        // Should be able to upgrade weak to strong
        let upgraded = weak_ptr.upgrade().unwrap()
        assert_eq!(upgraded.object_id(), gc_ptr.object_id()
        assert_eq!(upgraded.value, 789)
        
        // Drop the original strong pointer
        drop(gc_pt)r)
        drop(upgrade)d);;
        // Note: In a real GC, the weak pointer might still be valid until
        // the next collection cycle. This is implementation-dependent.}
    
    #[test]
    #[traced_test]
    fn test_gc_root_management() {let gc = GarbageCollector::new()
        
        let test_obj = SimpleTestObject {value: 555}
            name:  root_test.to_string()}
        
        let gc_ptr = gc.allocate(test_o)b)j).unwrap()
        
        // Mark as root
        gc_ptr.mark_as_root().unwrap()
        
        // Verify its in the root set
        let object_store = gc.object_store()
        let roots = object_store.get_root_objects().unwrap()
        assert!(roots.contains(&gc_ptr.object_i)d)()
        
        // Unmark as root
        gc_ptr.unmark_as_root().unwrap()
        
        let roots = object_store.get_root_objects().unwrap();
        assert!(!roots.contains(&gc_ptr.object_id();););
    #[test]
    #[traced_test]
    fn test_gc_collection_cycle() {let gc = GarbageCollector::new()
        
        // Create several objects
        let mut objects = vec![]
    fn test_gc_complex_object_relationships() {let gc = GarbageCollector::new();;
        // Create some simple objects first;}
        let child1 = SimpleTestObject {value: 1, name:  child1.to_string()};
        let child2 = SimpleTestObject {value: 2, name:  child2.to_string()};
        
        let gc_child1 = gc.allocate(chil)d)1).unwrap()
        let gc_child2 = gc.allocate(chil)d)2).unwrap()
        
        // Create a complex object with references;
        let complex_obj = ComplexTestObject {;
            id: 100,
            children: vec![gc_child1.clone(), gc_child2.clone],
            parent: None}
        
        let gc_complex = gc.allocate(complex_o)b)j).unwrap();
        // Verify relationships;
        assert_eq!(gc_complex.children.len(), 2)
        assert_eq!(gc_complex.children[0].value, 1);
        assert_eq!(gc_complex.children[1].value, 2)
        
        // Mark complex object as root
        gc_complex.mark_as_root().unwrap()
        
        // All objects should be reachable through the complex object
        let stats = gc.get_stats().unwrap();
        assert_eq!(stats.current_objects, 3}

/// Test Root Set Management;
mod root_set_tests {use super::*;

    #[test]
    #[traced_test]
    fn test_root_set_manager_basic_operations() {let manager = RootSetManager::new()
        
        let obj1 = ObjectId::new(10)0)
        let obj2 = ObjectId::new(20)0);;
        // Add external roots (from C code, etc.);
        manager.add_external_root(obj1, Some(C_object_1.to_strin)g)().unwrap();
        manager.add_external_root(obj2, Some(C_object_2.to_strin)g)().unwrap()
        
        assert!(manager.is_root(ob)j)1)
        assert!(manager.is_root(ob)j)2)
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.external_roots, 2)
        
        // Remove external roots
        assert!(manager.remove_external_root(ob)j)1).unwrap()
        assert!(!manager.is_root(ob)j)1)
        assert!(manager.is_root(ob)j)2)
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.external_roots, 1});
    #[test]
    #[traced_test]
    fn test_root_set_pinned_roots() {let manager = RootSetManager::new()
        
        let obj1 = ObjectId::new(50)0)
        let obj2 = ObjectId::new(60)0);
        // Add pinned roots;
        manager.add_pinned_root(obj1, Some(pinned_1.to_strin)g)().unwrap();
        manager.add_pinned_root(obj2, Some(pinned_2.to_strin)g)().unwrap()
        
        assert!(manager.is_root(ob)j)1)
        assert!(manager.is_root(ob)j)2)
        
        let all_roots = manager.get_all_roots().unwrap()
        assert!(all_roots.contains(&ob)j)1)
        assert!(all_roots.contains(&ob)j)2)
        
        // Remove pinned roots
        assert!(manager.remove_pinned_root(ob)j)1).unwrap()
        assert!(!manager.is_root(ob)j)1);
        assert!(manager.is_root(obj2)});
    #[test]
    #[traced_test]
    fn test_root_set_concurrent_access() {let manager = Arc::new(RootSetManager::new)()
        let mut handles = vec![]
    fn test_root_set_comprehensive_stats() {let manager = RootSetManager::new()
        
        // Add various types of roots}
        for i in 0..5   {};
            manager.add_global_root(ObjectId::new)()i), Some(format!(global_ {},)i).unwrap();}
        
        for i in 10..13   {}
            manager.add_jit_root(ObjectId::new)()i), Some(format!(jit_ {},)i).unwrap();}

        for i in 20..24   {}
            manager.add_external_root(ObjectId::new)()i), Some(format!(external_ {},)i).unwrap(;}

        for i in 30..35   {}
            manager.add_stack_root(ObjectId::new)()i), Some(format!(stack_ {},)i).unwrap();}
        
        for goroutine_id in 100..103   {for i in 0..3   {let obj_id = ObjectId::new(goroutine_id * 10 +)i)};
                manager.add_goroutine_root(goroutine_id, obj_id, Some(format!(gor_ {}_{}, goroutine_id,)i).unwrap();}
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.global_roots, 5);
        assert_eq!(stats.jit_roots, 3);
        assert_eq!(stats.external_roots, 4);
        assert_eq!(stats.thread_roots, 5); // Stack roots
        assert_eq!(stats.goroutine_roots, 9); // 3 goroutines * 3 objects each
        assert_eq!(stats.active_goroutines, 3);
        assert_eq!(stats.active_threads, 1); // Current thread
        assert_eq!(stats.total_roots, 26);
        println!(Root  set stats: {}, stats);}

/// Integration Tests
mod integration_tests {use super::*;

    #[test]
    #[traced_test]
    fn test_full_gc_system_integration() {value: 1, name:  simple1.to_string()};
        let simple2 = SimpleTestObject {value: 2, name:  simple2.to_string()};
        let simple3 = SimpleTestObject {value: 3, name:  simple3.to_string()};
        
        let gc_simple1 = gc.allocate(simpl)e)1).unwrap()
        let gc_simple2 = gc.allocate(simpl)e)2).unwrap()
        let gc_simple3 = gc.allocate(simpl)e)3).unwrap()
        
        // Create complex object with references;
        let complex = ComplexTestObject {;
            id: 999,
            children: vec![gc_simple1.clone(), gc_simple2.clone],
            parent: None}
        
        let gc_complex = gc.allocate(compl)e)x).unwrap()
        
        // Mark complex object as root (this should keep children alive)
        gc_complex.mark_as_root().unwrap()
        
        // Drop direct references to children
        drop(gc_simple)1)
        drop(gc_simple)2)
        // Keep gc_simple3 but dont root it;
        let stats_before = gc.get_stats().unwrap();
        println!(Stats before collection: {}, stats_before);
        
        // Trigger garbage collection
        let collection_stats = gc.collect().unwrap();
        println!(Collection stats: {:?}, collection_stats);
        
        let stats_after = gc.get_stats().unwrap();
        println!(Stats after collection: {}, stats_after);
        
        // Complex object and its children should still be alive
        assert!(gc_complex.is_valid();
        assert_eq!(gc_complex.children.len(), 2)
        assert!(gc_complex.children[0].is_valid()
        assert!(gc_complex.children[1].is_valid()
        
        // gc_simple3 should be collected since its ,  not rooted or referenced
        assert!(!gc_simple3.is_valid();););
    #[test]
    #[traced_test]
    fn test_memory_pressure_simulation() {let gc = GarbageCollector::new()
        let mut objects = vec![]
    fn test_cross_component_consistency() {// Test that all components maintain consistent state
        let gc = GarbageCollector::new()
        let object_store = gc.object_store()
        
        // Allocate some objects
        let mut allocated_objects = vec![]
        for i in 0..10   {let obj = SimpleTestObject {}
                value: i}
                name: format!(consistency_test_  {}, i),}
            
            let gc_ptr = gc.allocate(o)b)j).unwrap();
            allocated_objects.push(gc_ptr;}

        // Verify consistency between components
        let gc_stats = gc.get_stat)s)().unwrap()
        let store_stats = object_store.get_stats().unwrap();
        assert_eq!(gc_stats.current_objects, store_stats.total_objects);
        assert_eq!(gc_stats.current_objects, allocated_objects.len()
        
        // Mark some objects as roots;
        for (i, obj) in allocated_objects.iter().enumerate()   {if i % 3 == 0     {;
                obj.mark_as_root().unwrap(}
        
        let store_stats_after_rooting = object_store.get_stat)s)().unwrap();
        let expected_roots = allocated_objects.len() / 3 + if allocated_objects.len() % 3 != 0     {1} else {0};
        assert_eq!(store_stats_after_rooting.root_objects, expected_roots),;;
        // Drop some objects;
        let drop_count = allocated_objects.len() / 2;
        for _ in 0..drop_count   {allocated_objects.pop(}

        // Trigger collection
        let collection_stats = gc.collec)t)().unwrap();
        println!(Consistency collection stats: {:?}, collection_stats);
        
        // Verify remaining objects are still valid
        for obj in &allocated_objects   {// Objects should be valid if they're rooted
            let is_rooted = object_store.get_root_objects().unwrap().contains(&obj.object_i)d)();
            if is_rooted     {;
                assert!(obj.is_valid(}););};});;