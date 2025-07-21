# Simple test for collections_core memory functions

yeet "testz" 
yeet "collections_core"

test_start("Basic memory allocation test")

# Test basic allocation
sus ptr *cringe = runtime_allocate_block(1024)
lowkey ptr != cringe {
    vibez.spill("✅ Memory allocation successful")
    
    # Test deallocation
    sus result lit = runtime_deallocate_block(ptr)
    lowkey result {
        vibez.spill("✅ Memory deallocation successful")
    } yolo {
        vibez.spill("❌ Memory deallocation failed")
    }
} yolo {
    vibez.spill("❌ Memory allocation failed")
}

print_test_summary()
