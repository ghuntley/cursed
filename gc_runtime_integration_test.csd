fr fr GC Runtime Integration Test
fr fr Tests basic GC functionality with LLVM-generated code

yeet "testz"

fr fr Simple test structure
squad Point {
    spill x normie
    spill y normie
}

fr fr Test basic allocation and access
slay test_basic_gc_allocation() {
    test_start("Basic GC Allocation")
    
    fr fr Allocate objects through normal CURSED allocation
    sus points []Point = []
    
    bestie i := 0; i < 100; i = i + 1 {
        sus p Point = Point{x: i, y: i * 2}
        points.append(p)
    }
    
    fr fr Verify allocation worked
    assert_eq_int(points.len(), 100)
    assert_eq_int(points[50].x, 50)
    assert_eq_int(points[50].y, 100)
    
    vibez.spill("Basic allocation test passed")
}

fr fr Test that stack references are preserved
slay test_stack_preservation() {
    test_start("Stack Reference Preservation")
    
    sus local_point Point = Point{x: 42, y: 84}
    
    fr fr Allocate many objects to trigger GC
    sus temp_objects []Point = []
    bestie i := 0; i < 1000; i = i + 1 {
        temp_objects.append(Point{x: i, y: i})
    }
    
    fr fr Force garbage collection (if available)
    fr fr Local stack variable should be preserved
    
    fr fr Verify local object survived
    assert_eq_int(local_point.x, 42)
    assert_eq_int(local_point.y, 84)
    
    vibez.spill("Stack preservation test passed")
}

fr fr Test allocation of different sizes
slay test_varied_allocations() {
    test_start("Varied Size Allocations")
    
    fr fr Small objects
    sus small_arrays [][]normie = []
    bestie i := 0; i < 50; i = i + 1 {
        sus arr []normie = [i, i + 1, i + 2]
        small_arrays.append(arr)
    }
    
    fr fr Medium objects
    sus medium_arrays [][]normie = []
    bestie i := 0; i < 20; i = i + 1 {
        sus arr []normie = []
        bestie j := 0; j < 100; j = j + 1 {
            arr.append(i * 100 + j)
        }
        medium_arrays.append(arr)
    }
    
    fr fr Large objects
    sus large_arrays [][]normie = []
    bestie i := 0; i < 5; i = i + 1 {
        sus arr []normie = []
        bestie j := 0; j < 1000; j = j + 1 {
            arr.append(i * 1000 + j)
        }
        large_arrays.append(arr)
    }
    
    fr fr Verify all allocations
    assert_eq_int(small_arrays.len(), 50)
    assert_eq_int(medium_arrays.len(), 20)
    assert_eq_int(large_arrays.len(), 5)
    
    fr fr Verify some data integrity
    assert_eq_int(small_arrays[10][1], 11)
    assert_eq_int(medium_arrays[5][50], 550)
    assert_eq_int(large_arrays[2][500], 2500)
    
    vibez.spill("Varied allocation test passed")
}

fr fr Test object lifecycle
slay test_object_lifecycle() {
    test_start("Object Lifecycle")
    
    fr fr Create nested structure
    sus root Point = Point{x: 1, y: 1}
    
    {
        fr fr Create temporary objects in inner scope
        sus temp_points []Point = []
        bestie i := 0; i < 100; i = i + 1 {
            temp_points.append(Point{x: i + 10, y: i + 20})
        }
        
        fr fr Verify temporary objects exist
        assert_eq_int(temp_points[50].x, 60)
        
    } fr fr temp_points goes out of scope here
    
    fr fr Root object should still be accessible
    assert_eq_int(root.x, 1)
    assert_eq_int(root.y, 1)
    
    vibez.spill("Object lifecycle test passed")
}

fr fr Test memory management with function calls
slay create_point_array(size normie) []Point {
    sus result []Point = []
    bestie i := 0; i < size; i = i + 1 {
        result.append(Point{x: i, y: i * 3})
    }
    damn result
}

slay test_function_allocation() {
    test_start("Function Allocation")
    
    fr fr Allocate through function calls
    sus arrays [][]Point = []
    
    bestie i := 0; i < 10; i = i + 1 {
        sus arr = create_point_array(20 + i)
        arrays.append(arr)
        assert_eq_int(arr.len(), 20 + i)
    }
    
    fr fr Verify function-allocated data
    assert_eq_int(arrays.len(), 10)
    assert_eq_int(arrays[5][10].x, 10)
    assert_eq_int(arrays[5][10].y, 30)
    
    vibez.spill("Function allocation test passed")
}

fr fr Test string allocation (if supported)
slay test_string_allocation() {
    test_start("String Allocation")
    
    sus strings []tea = []
    
    bestie i := 0; i < 50; i = i + 1 {
        sus s tea = "test_string_" + i.to_string()
        strings.append(s)
    }
    
    fr fr Verify string allocation
    assert_eq_int(strings.len(), 50)
    assert_true(strings[25].contains("25"))
    
    vibez.spill("String allocation test passed")
}

fr fr Test recursive allocation
slay test_recursive_allocation() {
    test_start("Recursive Allocation")
    
    slay create_nested_points(depth normie) Point {
        if depth <= 0 {
            damn Point{x: 0, y: 0}
        }
        
        sus inner = create_nested_points(depth - 1)
        damn Point{x: depth, y: inner.x + inner.y}
    }
    
    sus result = create_nested_points(10)
    assert_eq_int(result.x, 10)
    
    vibez.spill("Recursive allocation test passed")
}

fr fr Main test function
slay main() {
    vibez.spill("Starting GC runtime integration tests...")
    
    test_basic_gc_allocation()
    test_stack_preservation()
    test_varied_allocations()
    test_object_lifecycle()
    test_function_allocation()
    test_string_allocation()
    test_recursive_allocation()
    
    print_test_summary()
    
    vibez.spill("GC runtime integration tests completed!")
}

main()
