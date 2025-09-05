# Interface Optimization Performance Benchmark
# Tests interface method call inlining performance improvements

yeet "testz"

# Define a simple interface for benchmarking
collab Drawable {
    slay draw(canvas tea) tea
    slay get_bounds() tea
}

# Implementation 1: Simple rectangle
struct Rectangle {
    x normie
    y normie
    width normie
    height normie
}

impl Rectangle vibes Drawable {
    slay draw(canvas tea) tea {
        damn "Drawing rectangle at (" + self.x + ", " + self.y + ")"
    }
    
    slay get_bounds() tea {
        damn "Rectangle bounds: " + self.width + "x" + self.height
    }
}

# Implementation 2: Simple circle
struct Circle {
    x normie
    y normie
    radius normie
}

impl Circle vibes Drawable {
    slay draw(canvas tea) tea {
        damn "Drawing circle at (" + self.x + ", " + self.y + ") radius " + self.radius
    }
    
    slay get_bounds() tea {
        damn "Circle bounds: " + (self.radius * 2) + "x" + (self.radius * 2)
    }
}

# Hot path function that should benefit from interface inlining
slay render_batch(drawables [Drawable]) normie {
    sus total_operations normie = 0
    
    # This loop should be optimized with interface method inlining
    sus i normie = 0
    bestie i < 1000; i++ {
        sus j normie = 0
        bestie j < drawables.length; j++ {
            sus drawable Drawable = drawables[j]
            
            # These interface method calls should be inlined
            # when the concrete type is known
            sus result tea = drawable.draw("canvas")
            sus bounds tea = drawable.get_bounds()
            
            total_operations += 2
        }
    }
    
    damn total_operations
}

# Test static dispatch optimization
slay test_static_dispatch() lit {
    # Create objects with known concrete types
    sus rect Rectangle = Rectangle{x: 10, y: 20, width: 100, height: 50}
    sus circle Circle = Circle{x: 5, y: 15, radius: 25}
    
    # These should be statically resolved and potentially inlined
    sus rect_drawable Drawable = rect as Drawable
    sus circle_drawable Drawable = circle as Drawable
    
    # Hot loop that should benefit from optimization
    sus operations normie = 0
    sus i normie = 0
    bestie i < 10000; i++ {
        # These calls should be devirtualized/inlined
        rect_drawable.draw("test_canvas")
        circle_drawable.draw("test_canvas")
        rect_drawable.get_bounds()
        circle_drawable.get_bounds()
        operations += 4
    }
    
    vibez.spill("Static dispatch test: " + operations + " operations")
    damn operations == 40000
}

# Test dynamic dispatch performance
slay test_dynamic_dispatch(use_rectangles lit) lit {
    sus drawables [Drawable] = []
    
    # Create mixed array to force dynamic dispatch
    sus i normie = 0
    bestie i < 100; i++ {
        lowkey use_rectangles {
            sus rect Rectangle = Rectangle{x: i, y: i*2, width: 50, height: 30}
            drawables.append(rect as Drawable)
        } elsewise {
            sus circle Circle = Circle{x: i, y: i*2, radius: 15}
            drawables.append(circle as Drawable)
        }
    }
    
    # Measure performance of dynamic dispatch
    sus start_time normie = time.now()
    sus total_ops normie = render_batch(drawables)
    sus end_time normie = time.now()
    
    sus duration normie = end_time - start_time
    vibez.spill("Dynamic dispatch: " + total_ops + " ops in " + duration + "ms")
    
    damn total_ops == 200000  # 1000 iterations * 100 objects * 2 methods
}

# Test interface method complexity for inlining decisions
collab ComplexInterface {
    slay simple_method() normie
    slay complex_method() normie
    slay recursive_method(depth normie) normie
}

struct ComplexImpl {
    value normie
}

impl ComplexImpl vibes ComplexInterface {
    # Simple method - should be inlined
    slay simple_method() normie {
        damn self.value + 1
    }
    
    # Complex method - may not be inlined due to size
    slay complex_method() normie {
        sus result normie = 0
        sus i normie = 0
        bestie i < 100; i++ {
            sus j normie = 0
            bestie j < 100; j++ {
                result += i * j + self.value
                lowkey result > 1000000 {
                    result = result % 1000000
                }
            }
        }
        damn result
    }
    
    # Recursive method - should not be inlined
    slay recursive_method(depth normie) normie {
        lowkey depth <= 0 {
            damn self.value
        } elsewise {
            damn self.recursive_method(depth - 1) + 1
        }
    }
}

# Test inlining heuristics
slay test_inlining_heuristics() lit {
    sus impl ComplexImpl = ComplexImpl{value: 42}
    sus iface ComplexInterface = impl as ComplexInterface
    
    sus simple_result normie = 0
    sus complex_result normie = 0
    sus recursive_result normie = 0
    
    # Call each method multiple times to test inlining decisions
    sus i normie = 0
    bestie i < 1000; i++ {
        # Simple method calls should be inlined
        simple_result += iface.simple_method()
        
        # Complex method calls may not be inlined
        lowkey i % 10 == 0 {
            complex_result += iface.complex_method()
        }
        
        # Recursive calls should not be inlined
        lowkey i % 100 == 0 {
            recursive_result += iface.recursive_method(5)
        }
    }
    
    vibez.spill("Inlining heuristics test completed")
    vibez.spill("Simple calls: " + simple_result)
    vibez.spill("Complex calls: " + complex_result)
    vibez.spill("Recursive calls: " + recursive_result)
    
    damn based
}

# Generic interface for testing generic method inlining
collab Container<T> {
    slay get(index normie) T
    slay set(index normie, value T) lit
    slay size() normie
}

struct Vector<T> {
    data [T]
    length normie
}

impl<T> Vector<T> vibes Container<T> {
    slay get(index normie) T {
        damn self.data[index]
    }
    
    slay set(index normie, value T) lit {
        self.data[index] = value
        damn based
    }
    
    slay size() normie {
        damn self.length
    }
}

# Test generic interface optimization
slay test_generic_interface_optimization() lit {
    # Create specialized containers
    sus int_vector Vector<normie> = Vector<normie>{data: [1, 2, 3, 4, 5], length: 5}
    sus str_vector Vector<tea> = Vector<tea>{data: ["a", "b", "c"], length: 3}
    
    sus int_container Container<normie> = int_vector as Container<normie>
    sus str_container Container<tea> = str_vector as Container<tea>
    
    # These generic method calls should be monomorphized and potentially inlined
    sus sum normie = 0
    sus i normie = 0
    bestie i < int_container.size(); i++ {
        sum += int_container.get(i)
    }
    
    sus concatenated tea = ""
    sus j normie = 0
    bestie j < str_container.size(); j++ {
        concatenated += str_container.get(j)
    }
    
    vibez.spill("Generic interface test: sum=" + sum + ", concat=" + concatenated)
    damn sum == 15 && concatenated == "abc"
}

# Performance comparison function
slay benchmark_interface_performance() lit {
    vibez.spill("Starting interface optimization benchmark...")
    
    # Test 1: Static dispatch optimization
    sus static_test lit = test_static_dispatch()
    assert_true(static_test)
    
    # Test 2: Dynamic dispatch performance
    sus dynamic_rect lit = test_dynamic_dispatch(based)
    sus dynamic_circle lit = test_dynamic_dispatch(cap)
    assert_true(dynamic_rect)
    assert_true(dynamic_circle)
    
    # Test 3: Inlining heuristics
    sus heuristics_test lit = test_inlining_heuristics()
    assert_true(heuristics_test)
    
    # Test 4: Generic interface optimization
    sus generic_test lit = test_generic_interface_optimization()
    assert_true(generic_test)
    
    vibez.spill("All interface optimization tests passed!")
    damn based
}

# Run the benchmark
test_start("Interface Optimization Benchmark")
sus benchmark_result lit = benchmark_interface_performance()
assert_true(benchmark_result)
print_test_summary()
