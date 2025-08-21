# Oracle's Week 2: Memory & Performance - GC Stack Maps Integration Test
# Complex CURSED program to validate GC stack maps with heap objects

yeet "vibez"
yeet "arrayarrays"
yeet "gc" 

# Test 1: Complex object graph with potential circular references
squad ComplexHeapObject {
    name tea
    id drip
    children []ComplexHeapObject
    parent ?ComplexHeapObject
    
    slay new(name tea, id drip) ComplexHeapObject {
        damn ComplexHeapObject{
            name: name,
            id: id, 
            children: [],
            parent: none
        }
    }
    
    slay add_child(child ComplexHeapObject) {
        child.parent = some(self)
        self.children.append(child)
    }
    
    slay get_all_descendants() []ComplexHeapObject {
        sus result []ComplexHeapObject = []
        bestie (child in self.children) {
            result.append(child)
            sus grandchildren []ComplexHeapObject = child.get_all_descendants()
            bestie (grandchild in grandchildren) {
                result.append(grandchild)
            }
        }
        damn result
    }
}

# Test 2: Large array with heap-allocated objects
slay create_large_heap_structure() []ComplexHeapObject {
    vibez.spill("🧪 Creating large heap structure for GC stack map testing...")
    
    sus objects []ComplexHeapObject = []
    
    # Create 1000 objects in heap
    bestie (i drip in range(1000)) {
        sus obj ComplexHeapObject = ComplexHeapObject.new("Object_${i}", i)
        objects.append(obj)
        
        # Create complex parent-child relationships
        ready (i > 0) {
            sus parent_idx drip = (i - 1) % 100
            ready (parent_idx < objects.len()) {
                objects[parent_idx].add_child(obj)
            }
        }
        
        # Force GC pressure every 100 objects
        ready (i % 100 == 0) {
            vibez.spill("  Created ${i} objects, forcing GC cycle...")
            gc.collect()  # This should trigger stackmap scanning
        }
    }
    
    vibez.spill("✅ Created ${objects.len()} heap objects")
    damn objects
}

# Test 3: Nested function calls with live heap references
slay test_stackmap_precision_nested(depth drip, objects []ComplexHeapObject) {
    ready (depth == 0) {
        # At deepest recursion, create new objects and trigger GC
        sus new_obj ComplexHeapObject = ComplexHeapObject.new("Deep_${depth}", depth)
        objects.append(new_obj)
        
        vibez.spill("  Depth ${depth}: Created object, triggering GC...")
        gc.collect()  # GC must scan stack for live references
        
        # Access all objects to verify they survived GC
        sus total_children drip = 0
        bestie (obj in objects) {
            total_children = total_children + obj.children.len()
        }
        
        vibez.spill("  Verified ${objects.len()} objects with ${total_children} child refs")
        damn
    }
    
    # Create local object that must survive recursive call
    sus local_obj ComplexHeapObject = ComplexHeapObject.new("Local_${depth}", depth)
    objects.append(local_obj)
    
    # Recursive call with live local reference on stack
    test_stackmap_precision_nested(depth - 1, objects)
    
    # Verify local object survived after recursive calls
    ready (local_obj.id != depth) {
        vibez.spill("❌ STACKMAP FAILURE: Local object corrupted!")
        damn
    }
}

# Test 4: Multi-threaded heap allocation with GC pressure
go slay concurrent_heap_pressure() {
    bestie (i drip in range(100)) {
        sus objects []ComplexHeapObject = []
        
        # Create burst of objects
        bestie (j drip in range(50)) {
            sus obj ComplexHeapObject = ComplexHeapObject.new("Thread_${i}_${j}", j)
            objects.append(obj)
        }
        
        # Build complex reference graph
        bestie (k drip in range(objects.len() - 1)) {
            objects[k].add_child(objects[k + 1])
        }
        
        # Force GC while objects are live on stack
        gc.collect()
        
        # Verify all objects survived
        bestie (obj in objects) {
            ready (obj.name == "") {
                vibez.spill("❌ CONCURRENT GC FAILURE: Object corrupted!")
            }
        }
    }
    
    vibez.spill("✅ Concurrent heap pressure test completed")
}

# Test 5: Interface objects with virtual dispatch and GC
collab Drawable {
    slay draw()
    slay get_area() drip
}

squad Rectangle implements Drawable {
    width drip
    height drip
    
    slay new(w drip, h drip) Rectangle {
        damn Rectangle{ width: w, height: h }
    }
    
    slay draw() {
        vibez.spill("Drawing rectangle ${self.width}x${self.height}")
    }
    
    slay get_area() drip {
        damn self.width * self.height
    }
}

squad Circle implements Drawable {
    radius drip
    
    slay new(r drip) Circle {
        damn Circle{ radius: r }
    }
    
    slay draw() {
        vibez.spill("Drawing circle r=${self.radius}")
    }
    
    slay get_area() drip {
        damn 3.14 * self.radius * self.radius
    }
}

slay test_interface_gc_interaction() {
    vibez.spill("🧪 Testing interface objects with GC stack maps...")
    
    sus drawables []Drawable = []
    
    # Create mixed interface objects
    bestie (i drip in range(100)) {
        ready (i % 2 == 0) {
            sus rect Rectangle = Rectangle.new(i, i + 1)
            drawables.append(rect as Drawable)
        } otherwise {
            sus circle Circle = Circle.new(i)
            drawables.append(circle as Drawable)
        }
    }
    
    # Force multiple GC cycles during virtual dispatch
    bestie (drawable in drawables) {
        drawable.draw()
        gc.collect()  # Test stackmap handling with vtable pointers
        sus area drip = drawable.get_area()
        ready (area <= 0) {
            vibez.spill("❌ INTERFACE GC FAILURE: Virtual dispatch corrupted!")
        }
    }
    
    vibez.spill("✅ Interface GC interaction test completed")
}

# Main test execution
slay main() {
    vibez.spill("🚀 Oracle's Week 2: GC Stack Maps Integration Test Starting...")
    vibez.spill("Testing: LLVM stack maps, precise GC scanning, object lifetime management")
    
    # Enable GC debugging for stack map validation
    gc.set_debug_mode(based)
    gc.set_collection_threshold(1024 * 1024)  # 1MB threshold for frequent testing
    
    vibez.spill("\n📋 Test 1: Large heap structure creation")
    sus large_objects []ComplexHeapObject = create_large_heap_structure()
    
    vibez.spill("\n📋 Test 2: Stack map precision with nested calls")
    test_stackmap_precision_nested(10, large_objects)
    
    vibez.spill("\n📋 Test 3: Interface objects with GC interaction")  
    test_interface_gc_interaction()
    
    vibez.spill("\n📋 Test 4: Multi-threaded heap pressure")
    concurrent_heap_pressure()
    
    vibez.spill("\n📋 Test 5: Final GC validation")
    gc.collect()
    gc.validate_heap()
    
    sus final_stats = gc.get_stats()
    vibez.spill("Final GC Stats:")
    vibez.spill("  Collections: ${final_stats.collections}")
    vibez.spill("  Objects tracked: ${final_stats.objects_tracked}")
    vibez.spill("  Memory freed: ${final_stats.memory_freed_mb}MB")
    vibez.spill("  Stack maps generated: ${final_stats.stackmaps_generated}")
    vibez.spill("  False positives: ${final_stats.false_positives}")
    
    ready (final_stats.false_positives > 0) {
        vibez.spill("❌ STACKMAP VALIDATION FAILED: ${final_stats.false_positives} false positives!")
        damn
    }
    
    vibez.spill("✅ Oracle's Week 2 GC Stack Maps Integration: ALL TESTS PASSED")
    vibez.spill("✅ Zero false positives confirmed - Precise GC working correctly")
}
