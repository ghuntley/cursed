use crate::memory::{MemoryManager, Traceable, Visitor, TaggedPtr, Tag, MemoryError};
use crate::object::Object;
use crate::compiler::{Bytecode, Instructions, Opcode, Compiler};
use crate::parser::{Parser};
use crate::lexer::Lexer;
use crate::vm::VM;
use proptest::prelude::*;
use std::rc::Rc;
use std::alloc::Layout;
use crate::memory::{align_up, MIN_ALIGNMENT, DEFAULT_BLOCK_SIZE};
use crate::memory::bump::BumpAllocator;
use crate::memory::block::BlockAllocator;
use crate::memory::gc::{GarbageCollector, Gc};
use crate::memory::tagged::TaggedPtr as TP;
use crate::memory::Allocated;
use crate::memory::allocator::Allocator;
use super::tagged::{TaggedPtr as TP, Tag, TypedPtr};

fn parse_compile(input: &str) -> Result<Bytecode, crate::error::Error> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    let mut compiler = Compiler::new();
    compiler.compile(program)?;
    
    Ok(compiler.bytecode())
}

// Helper struct to test the memory management integration
struct TestTraceable {
    value: i32,
    next: Option<Rc<TestTraceable>>,
}

impl Traceable for TestTraceable {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            visitor.visit(next.as_ref());
            
            visitor.visit_ptr(std::ptr::addr_of!(**next) as usize, Tag::Object);
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

// Basic memory tests moved from mod.rs

#[test]
fn test_bump_allocator() {
    let mut allocator = BumpAllocator::new(1024);
    
    // Allocate a few objects
    let layout = Layout::from_size_align(16, MIN_ALIGNMENT).unwrap();
    let ptr1 = allocator.allocate(layout).unwrap();
    let ptr2 = allocator.allocate(layout).unwrap();
    let ptr3 = allocator.allocate(layout).unwrap();
    
    // Make sure we got different pointers
    assert_ne!(ptr1.as_ptr(), ptr2.as_ptr());
    assert_ne!(ptr2.as_ptr(), ptr3.as_ptr());
    assert_ne!(ptr3.as_ptr(), ptr1.as_ptr());
    
    // Reset the allocator
    allocator.reset();
    
    // Allocate again, we should get the first pointer back
    let ptr4 = allocator.allocate(layout).unwrap();
    assert_eq!(ptr1.as_ptr(), ptr4.as_ptr());
}

#[test]
fn test_block_allocator() {
    let mut allocator = BlockAllocator::new(4096);
    
    // Allocate a few objects with different sizes
    let layout1 = Layout::from_size_align(16, MIN_ALIGNMENT).unwrap();
    let layout2 = Layout::from_size_align(64, MIN_ALIGNMENT).unwrap();
    let layout3 = Layout::from_size_align(256, MIN_ALIGNMENT).unwrap();
    
    let ptr1 = allocator.allocate(layout1).unwrap();
    let ptr2 = allocator.allocate(layout2).unwrap();
    let ptr3 = allocator.allocate(layout3).unwrap();
    
    // Make sure we got different pointers
    assert_ne!(ptr1.as_ptr(), ptr2.as_ptr());
    assert_ne!(ptr2.as_ptr(), ptr3.as_ptr());
    assert_ne!(ptr3.as_ptr(), ptr1.as_ptr());
    
    // Deallocate the second pointer
    unsafe {
        allocator.deallocate(ptr2, layout2);
    }
    
    // Allocate again with the same size, we should get the same pointer back
    let ptr4 = allocator.allocate(layout2).unwrap();
    assert_eq!(ptr2.as_ptr(), ptr4.as_ptr());
    
    // Reset the allocator
    allocator.reset();
}

#[test]
fn test_tagged_ptr() {
    // Create a dummy object to point to
    let mut x = 42;
    let ptr = std::ptr::NonNull::new(&mut x as *mut _).unwrap();
    
    // Create a tagged pointer
    let tagged = TP::new(ptr, Tag::Integer);
    
    // Check the tag
    assert_eq!(tagged.tag(), Tag::Integer);
    
    // Check the pointer
    assert_eq!(tagged.ptr().unwrap().as_ptr(), &mut x as *mut _);
    
    // Change the tag
    let mut tagged = tagged;
    tagged.set_tag(Tag::Float);
    assert_eq!(tagged.tag(), Tag::Float);
    
    // Create a null pointer
    let null_ptr = TP::<i32>::null(Tag::Null);
    assert_eq!(null_ptr.tag(), Tag::Null);
    assert!(null_ptr.is_null());
    assert!(null_ptr.ptr().is_none());
}

#[test]
fn test_memory_manager() {
    let manager = MemoryManager::new(1024 * 1024);
    
    // Allocate using the bump allocator
    let ptr1 = manager.allocate_bump(16).unwrap();
    let ptr2 = manager.allocate_bump(32).unwrap();
    
    // Allocate using the block allocator
    let ptr3 = manager.allocate_block(64).unwrap();
    let ptr4 = manager.allocate_block(128).unwrap();
    
    // Make sure we got different pointers
    assert_ne!(ptr1.as_ptr(), ptr2.as_ptr());
    assert_ne!(ptr2.as_ptr(), ptr3.as_ptr());
    assert_ne!(ptr3.as_ptr(), ptr4.as_ptr());
    assert_ne!(ptr4.as_ptr(), ptr1.as_ptr());
    
    // Reset the manager
    manager.reset().unwrap();
}

#[test]
fn test_allocated_wrapper() {
    // Create an allocator
    let allocator = Rc::new(BumpAllocator::new(1024)) as Rc<dyn Allocator>;
    
    // Allocate memory
    let layout = Layout::from_size_align(16, MIN_ALIGNMENT).unwrap();
    let ptr = allocator.borrow_mut().allocate(layout).unwrap();
    
    // Create an allocated wrapper
    let mut allocated = Allocated::<u8>::new(ptr, layout, allocator.clone());
    
    // Check that we can access the memory
    *allocated.get_mut() = 42;
    assert_eq!(*allocated.get(), 42);
    
    // Wrapper will deallocate the memory when dropped
}

#[test]
fn test_basic_gc_integration() {
    struct TestObject {
        value: i32,
    }
    
    impl Traceable for TestObject {
        fn trace(&self, _visitor: &mut dyn Visitor) {
            // No references to trace
        }
        
        fn size(&self) -> usize {
            std::mem::size_of::<TestObject>()
        }
    }
    
    let gc = GarbageCollector::with_heap_size(1024 * 1024);
    
    // Allocate some objects
    let obj1 = gc.allocate(TestObject { value: 42 }, Tag::Integer).unwrap();
    
    // Check that we can access the object
    assert_eq!(obj1.get().value, 42);
    
    // Run garbage collection
    obj1.collect();
    
    // Object should still be accessible because it's still in scope
    assert_eq!(obj1.get().value, 42);
}

// Property tests for memory management
proptest! {
    // Test that VM correctly uses memory allocator for object creation
    #[test]
    fn test_vm_memory_integration(input in "let x = 5; x + (2 * 3);") {
        let bytecode = parse_compile(&input).unwrap();
        let mut vm = VM::new(bytecode);
        
        // Execute the bytecode
        let result = vm.run();
        assert!(result.is_ok());
        
        // Verify the memory manager was used
        let bump_stats = vm.memory_manager.bump_allocator.borrow().stats();
        let block_stats = vm.memory_manager.block_allocator.borrow().stats();
        
        // Some memory should have been allocated during execution
        assert!(bump_stats.total_allocated > 0 || block_stats.total_allocated > 0);
    }
    
    // Test garbage collection integration with VM
    #[test]
    fn test_vm_gc_integration(iterations in 1..100u32) {
        // Set up memory manager with small heap to force GC
        let memory_manager = MemoryManager::new_with_sizes(1024, 1024, 4096);
        
        // Create objects that will be garbage collected
        for _ in 0..iterations {
            let result = memory_manager.allocate_with_gc(
                TestTraceable { 
                    value: 42, 
                    next: None 
                },
                Tag::Integer,
                |obj, visitor| {
                    let obj = obj.downcast_ref::<TestTraceable>().unwrap();
                    obj.trace(visitor);
                },
                |obj| {
                    let obj = obj.downcast_ref::<TestTraceable>().unwrap();
                    obj.size()
                }
            );
            assert!(result.is_ok());
        }
        
        // Trigger garbage collection
        memory_manager.garbage_collector.borrow_mut().collect();
        
        // Check GC stats
        let gc_stats = memory_manager.garbage_collector.borrow().stats();
        
        // Some objects should have been collected if we created enough iterations
        if iterations > 50 {
            assert!(gc_stats.collected_objects > 0);
        }
    }
}

// Integration test that exercises VM with memory management
#[test]
fn test_vm_with_memory_management() {
    // Test program with various types of objects
    let input = r#"
    let x = 5;
    let y = "hello";
    let arr = [1, 2, 3, 4, 5];
    let hash = {"key": "value"};
    
    let sum = 0;
    let i = 0;
    
    lowkey i < 5 {
        sum = sum + arr[i];
        i = i + 1;
    }
    
    sum
    "#;
    
    let bytecode = parse_compile(input).unwrap();
    
    // Create VM with custom memory size
    let mut vm = VM::new_with_memory_size(bytecode, 1024 * 1024);
    
    // Run the VM
    let result = vm.run().unwrap();
    
    // Verify result (sum should be 15)
    assert_eq!(result, Some(Object::Integer(15)));
    
    // Verify memory manager was used
    let gc_stats = vm.memory_manager.garbage_collector.borrow().stats();
    let bump_stats = vm.memory_manager.bump_allocator.borrow().stats();
    let block_stats = vm.memory_manager.block_allocator.borrow().stats();
    
    // Some memory should have been allocated
    assert!(bump_stats.total_allocated > 0 || block_stats.total_allocated > 0);
}

#[test]
fn test_typed_ptr() {
    // Create a test structure
    struct TestStruct { value: i32 }
    
    // Create an instance of the test structure
    let test_struct = TestStruct { value: 42 };
    
    // Create a tagged pointer to the test structure with Integer tag
    let ptr_raw = &test_struct as *const TestStruct as *mut TestStruct;
    let ptr = TP::from_raw(ptr_raw, Tag::Integer);
    
    // Test has_tag
    assert!(ptr.has_tag(Tag::Integer));
    assert!(!ptr.has_tag(Tag::String));
    
    // Test as_type with correct tag
    let ts_ref = ptr.as_type::<TestStruct>(Tag::Integer);
    assert!(ts_ref.is_some());
    assert_eq!(ts_ref.unwrap().value, 42);
    
    // Test as_type with wrong tag
    let wrong_ref = ptr.as_type::<TestStruct>(Tag::String);
    assert!(wrong_ref.is_none());
    
    // Test cast
    let cast_ptr = ptr.cast::<TestStruct>(Tag::Integer);
    assert!(cast_ptr.is_some());
    assert_eq!(cast_ptr.unwrap().as_ref().unwrap().value, 42);
    
    // Test cast with wrong tag
    let wrong_cast = ptr.cast::<TestStruct>(Tag::String);
    assert!(wrong_cast.is_none());
    
    // Test unwrap_as
    let unwrapped = ptr.unwrap_as::<TestStruct>(Tag::Integer);
    assert!(unwrapped.is_ok());
    assert_eq!(unwrapped.unwrap().value, 42);
    
    // Test unwrap_as with wrong tag
    let wrong_unwrap = ptr.unwrap_as::<TestStruct>(Tag::String);
    assert!(wrong_unwrap.is_err());
    
    // Create a null pointer and test type checking
    let null_ptr = TP::<TestStruct>::null(Tag::Null);
    assert!(null_ptr.as_type::<TestStruct>(Tag::Null).is_none()); // None because it's null
    assert!(null_ptr.as_type::<TestStruct>(Tag::Integer).is_none()); // None because tag doesn't match
} 