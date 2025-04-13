//! Tests for the object storage system

use std::sync::Arc;

use cursed::memory::{Gc, Tag, Traceable, Visitor, global_object_storage, StorageWrapper};
use cursed::memory::gc::GarbageCollector;

// Test object with finalization tracking
#[derive(Clone, Debug)]
struct TestObject {
    id: usize,
    name: String,
    finalized: bool,
}

impl TestObject {
    fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            finalized: false,
        }
    }
    
    fn mark_finalized(&mut self) {
        self.finalized = true;
    }
}

impl Traceable for TestObject {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // No references to trace
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
    
    fn finalize(&mut self) {
        println!("Finalizing TestObject id={}, name={}", self.id, self.name);
        self.mark_finalized();
    }
}

#[test]
fn test_object_storage_basic() {
    // Create a direct storage object
    let obj = TestObject::new(1, "test1");
    let storage = global_object_storage();
    
    // Store the object
    let addr = storage.store(obj);
    
    // Check if the object is in storage
    assert!(storage.contains(addr), "Object should be in storage");
    
    // Remove the object and check it was finalized
    let result = storage.remove_and_finalize(addr);
    assert!(result, "Object should have been removed and finalized");
    
    // Object should no longer be in storage
    assert!(!storage.contains(addr), "Object should no longer be in storage");
}

#[test]
fn test_object_storage_with_gc() {
    // Get a clean GC
    let gc = Arc::new(GarbageCollector::new());
    
    // Create an object and allocate it
    let obj = TestObject::new(2, "gc-test");
    let gc_obj = gc.allocate(obj);
    
    // Get the address
    let addr = gc_obj.as_ptr() as usize;
    
    // The object should be stored in the global object storage
    assert!(global_object_storage().contains(addr), "Object should be in global storage");
    
    // Get a reference to the object through the global storage
    let obj_ref = global_object_storage().get::<TestObject>(addr)
        .expect("Should be able to get object reference");
    
    assert_eq!(obj_ref.id, 2);
    assert_eq!(obj_ref.name, "gc-test");
    
    // Force garbage collection
    drop(gc_obj);
    gc.collect_garbage();
    
    // The object should be finalized and removed from storage
    assert!(!global_object_storage().contains(addr), "Object should be removed from storage after collection");
}

#[test]
fn test_multiple_objects() {
    // Create several objects
    let obj1 = StorageWrapper::new(TestObject::new(1, "first"));
    let obj2 = StorageWrapper::new(TestObject::new(2, "second"));
    let obj3 = StorageWrapper::new(TestObject::new(3, "third"));
    
    // Get their addresses
    let addr1 = obj1.address();
    let addr2 = obj2.address();
    let addr3 = obj3.address();
    
    // Verify they're all in storage
    assert!(global_object_storage().contains(addr1));
    assert!(global_object_storage().contains(addr2));
    assert!(global_object_storage().contains(addr3));
    
    // Remove second object
    global_object_storage().remove_and_finalize(addr2);
    
    // Check that only second is removed
    assert!(global_object_storage().contains(addr1));
    assert!(!global_object_storage().contains(addr2));
    assert!(global_object_storage().contains(addr3));
    
    // Modify third object
    let obj3_mut = obj3.get_mut().expect("Should get mutable reference");
    obj3_mut.name = "modified third".to_string();
    
    // Verify the modification
    let obj3_ref = obj3.get().expect("Should get reference");
    assert_eq!(obj3_ref.name, "modified third");
    
    // Clear all objects
    global_object_storage().clear();
    
    // All objects should be gone
    assert_eq!(global_object_storage().len(), 0);
}