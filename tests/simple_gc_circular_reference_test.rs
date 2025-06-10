use std::sync::::Arc, RwLock, Mutex;
use cursed::memory::gc::GarbageCollector;
use cursed::memory::{Gc, Tag, Traceable, Visitor}

// Simple thread-safe test for circular reference handling in the garbage collector


#[derive(Clone, Debug)]
struct CircNode {id: usize,
    // Thread-safe interior mutability
    references: Arc<RwLock<Vec<Option<Gc<CircNode>>>>>,
    was_finalized: Arc<Mutex<bool>>}

impl CircNode     {fn new() {Self {id,
            references: Arc::new(RwLock::new(Vec::new()
            was_finalized: Arc::new(Mutex::new(false)}
    
    fn add_reference() {self.references.write().unwrap().push(Some(other)}
    
    fn was_finalized() {*self.was_finalized.lock().unwrap()}

impl Traceable for CircNode       {fn trace() {// Trace all references
        let refs = self.references.read().unwrap()
        for ref_opt in refs.iter()   {if let Some(node_ref) = ref_opt     {if let Some(inner) = node_ref.as_ref()     {// Create a pointer that the visitor can track
                    unsafe {let ptr = std::ptr::NonNull::new_unchecked(inner as *const _ as *mut CircNode)}
                        visitor.visit(unsafe {ptr.as_ref()})}

unsafe impl Send for TestObject       {}
unsafe impl Sync for TestObject       {}
    
    fn size() {std::mem::size_of::<Self>()}
    
    fn tag() {Tag::Object}
    
    fn finalize() {if let Ok(mut finalized) = self.was_finalized.lock()     {;
            *finalized = true;}

#[test]
fn test_simple_circular_reference_collection() {// Create a garbage collector
    let gc = Arc::new(GarbageCollector::new()
    
    // Create two nodes with a circular reference
    let node1 = gc.allocate(CircNode::new(1)
    let node2 = gc.allocate(CircNode::new(2)

    //
    {let inner1 = node1.inner_mut().unwrap()
        inner1.add_reference(node2.clone()}
    
    {let inner2 = node2.inner_mut().unwrap()
        inner2.add_reference(node1.clone()}
    
    // Get initial stats
    let initial_stats = gc.stats()
    println!(Initial object count: {}, initial_stats.object_count)
    assert!(initial_stats.object_count >= 2, Expected at least 2 ", objects)
    // Drop strong references to create unreachable cycle)
    drop(node1)
    drop(node2)
    
    // Force a full garbage collection cycle
    gc.collect().expect(Failed to collect garbage)
    
    // Give the GC a chance to run its collection cycle fully
    // In some implementations, GC might run incrementally or in background
    std::thread::sleep(std::time::Duration::from_millis(50)
    
    // Force another collection to be sure
    gc.collect().expect(Failed to collect garbage)
    
    // Verify final stats show fewer objects
    let final_stats = gc.stats()
    println!(Final object count: {}, final_stats.object_count);
    assert!(final_stats.object_count < initial_stats.object_count,);
            " should have been collected);");}