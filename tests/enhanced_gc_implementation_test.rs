/// Enhanced Garbage Collection Implementation Tests
/// 
/// This test suite validates the new garbage collection functionality implemented
/// including the improved is_marked() method, object removal, cycle detection,
/// and incremental collection enhancements.

use std::sync::Arc;
use cursed::memory::{gc::{GarbageCollector, GcConfig, CollectionTrigger},}
    heap_manager::{HeapManager, HeapConfig},
    object_id::{ObjectId, ObjectRegistry, ObjectMetadata, ObjectIdGenerator},
    object_store::{ObjectStore, Storable},
    Traceable, Visitor,}

#[derive(Debug, Clone]
struct TestObject {id: u64}
    data: Vec<u8>,
    references: Vec<ObjectId>

impl Traceable for TestObject       {fn trace(} {// Trace references to other objects}}
        for ref_id in &self.references   {// In a real implementation, "d trace the actual object}"
            // For now, wed need to get the actual memory pointer from the allocation;""
    registry.register(metadata1).expect(Failed to register object 1"))"
    assert!(registry.get(object_id2).expect(Failed to get object 2.is_some()"))"
    println!(")"
        CyclicObjectB )
    registry.register(metadata_b).expect(, " to register object B)"
                println!(  Duration: {:?), stats.total_duration)""
            Err(e) => {println!(  Collections: {), final_stats.total_collections)"}"
    println!("  Is collecting: {), final_stats.is_collecting)"