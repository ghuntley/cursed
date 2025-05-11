vibe benchmarks;

// Comprehensive benchmark for interface type assertions
// This file tests various inheritance patterns and their performance

// Basic Result type for error handling
squad Result<T, E> {
    value T,
    err E,
    isOk lit
}

// Helper to create a successful result
slay ok<T, E>(value T) Result<T, E> {
    return Result<T, E>{
        value: value,
        err: nofr as E,
        isOk: 1
    };
}

// Helper to create an error result
slay fail<T, E>(err E) Result<T, E> {
    return Result<T, E>{
        value: nofr as T,
        err: err,
        isOk: 0
    };
}

// Error interface
collab Error {
    slay error() tea;
}

// Basic type assertion error
squad TypeAssertionError {
    expected tea,
    actual tea
}

slay (e TypeAssertionError) error() tea {
    return "Type assertion failed: expected " + e.expected + " but got " + e.actual;
}

// ----- Simple Inheritance Pattern -----

// Base interface
collab SimpleBase {
    slay getValue() lit;
}

// Concrete implementation
squad SimpleImpl {
    value lit
}

slay (s SimpleImpl) getValue() lit {
    return s.value;
}

// ----- Nested Inheritance Pattern -----

// Base interface
collab NestedParent {
    slay getParentValue() lit;
}

// Child interface that extends base
collab NestedChild {
    slay getChildValue() lit;
}

// Implementation of both interfaces
squad NestedImpl {
    parentValue lit,
    childValue lit
}

slay (n NestedImpl) getParentValue() lit {
    return n.parentValue;
}

slay (n NestedImpl) getChildValue() lit {
    return n.childValue;
}

// ----- Diamond Inheritance Pattern -----

// Base interface
collab DiamondBase {
    slay getBaseValue() lit;
}

// Two interfaces that extend base
collab DiamondLeft {
    slay getLeftValue() lit;
}

collab DiamondRight {
    slay getRightValue() lit;
}

// Implementation of all three interfaces
squad DiamondImpl {
    baseValue lit,
    leftValue lit,
    rightValue lit
}

slay (d DiamondImpl) getBaseValue() lit {
    return d.baseValue;
}

slay (d DiamondImpl) getLeftValue() lit {
    return d.leftValue;
}

slay (d DiamondImpl) getRightValue() lit {
    return d.rightValue;
}

// ----- Deep Nested Inheritance Pattern -----

// Five level interface hierarchy
collab DeepLevel1 {
    slay level1() lit;
}

collab DeepLevel2 {
    slay level2() lit;
}

collab DeepLevel3 {
    slay level3() lit;
}

collab DeepLevel4 {
    slay level4() lit;
}

collab DeepLevel5 {
    slay level5() lit;
}

// Implementation of all five levels
squad DeepNestedImpl {
    val1 lit,
    val2 lit,
    val3 lit,
    val4 lit,
    val5 lit
}

slay (d DeepNestedImpl) level1() lit {
    return d.val1;
}

slay (d DeepNestedImpl) level2() lit {
    return d.val2;
}

slay (d DeepNestedImpl) level3() lit {
    return d.val3;
}

slay (d DeepNestedImpl) level4() lit {
    return d.val4;
}

slay (d DeepNestedImpl) level5() lit {
    return d.val5;
}

// ----- Benchmark Functions -----

// Perform a simple type assertion benchmark
slay benchmarkSimple(base SimpleBase, iterations lit) Result<lit, Error> {
    sus startTime = vibez.now();
    sus successCount = 0;
    
    bestie i := 0; i < iterations; i = i + 1 {
        // Try to assert as SimpleImpl
        sus impl, ok = base.(SimpleImpl);
        lowkey ok {
            successCount = successCount + 1;
        }
    }
    
    sus endTime = vibez.now();
    sus duration = endTime - startTime;
    
    vibez.spill("Simple inheritance benchmark: " + iterations + " iterations in " + duration + "ms");
    vibez.spill("Success rate: " + (successCount * 100 / iterations) + "%");
    vibez.spill("Average time per assertion: " + (duration * 1000 / iterations) + "μs");
    
    return ok<lit, Error>(successCount);
}

// Benchmark with nested inheritance
slay benchmarkNested(parent NestedParent, iterations lit) Result<lit, Error> {
    sus startTime = vibez.now();
    sus successCount = 0;
    
    bestie i := 0; i < iterations; i = i + 1 {
        // First assert as NestedChild
        sus child, childOk = parent.(NestedChild);
        lowkey childOk {
            // Then assert as NestedImpl
            sus impl = child.(NestedImpl)?;
            successCount = successCount + 1;
        }
    }
    
    sus endTime = vibez.now();
    sus duration = endTime - startTime;
    
    vibez.spill("Nested inheritance benchmark: " + iterations + " iterations in " + duration + "ms");
    vibez.spill("Success rate: " + (successCount * 100 / iterations) + "%");
    vibez.spill("Average time per assertion: " + (duration * 1000 / iterations) + "μs");
    
    return ok<lit, Error>(successCount);
}

// Benchmark with diamond inheritance
slay benchmarkDiamond(base DiamondBase, iterations lit) Result<lit, Error> {
    sus startTime = vibez.now();
    sus successCount = 0;
    
    bestie i := 0; i < iterations; i = i + 1 {
        // Try first path: Base -> Left -> Impl
        sus left, leftOk = base.(DiamondLeft);
        lowkey leftOk {
            // Continue to implementation
            sus impl1, implOk1 = left.(DiamondImpl);
            lowkey implOk1 {
                successCount = successCount + 1;
            }
        }
        
        // Try second path: Base -> Right -> Impl
        sus right, rightOk = base.(DiamondRight);
        lowkey rightOk {
            // This time with ? operator
            sus impl2 = right.(DiamondImpl)?;
            successCount = successCount + 1;
        } no cap {
            // Ignore errors
        }
    }
    
    sus endTime = vibez.now();
    sus duration = endTime - startTime;
    
    vibez.spill("Diamond inheritance benchmark: " + iterations + " iterations in " + duration + "ms");
    vibez.spill("Success rate: " + (successCount * 100 / iterations) + "%");
    vibez.spill("Average time per assertion: " + (duration * 1000 / iterations) + "μs");
    
    return ok<lit, Error>(successCount);
}

// Benchmark with deep nested inheritance
slay benchmarkDeepNested(level1 DeepLevel1, iterations lit) Result<lit, Error> {
    sus startTime = vibez.now();
    sus successCount = 0;
    
    bestie i := 0; i < iterations; i = i + 1 {
        // Chain of type assertions through all levels
        sus l2, ok2 = level1.(DeepLevel2);
        lowkey ok2 {
            sus l3, ok3 = l2.(DeepLevel3);
            lowkey ok3 {
                sus l4 = l3.(DeepLevel4)?;
                sus l5 = l4.(DeepLevel5)?;
                sus impl = l5.(DeepNestedImpl)?;
                successCount = successCount + 1;
            }
        }
    }
    
    sus endTime = vibez.now();
    sus duration = endTime - startTime;
    
    vibez.spill("Deep nested inheritance benchmark: " + iterations + " iterations in " + duration + "ms");
    vibez.spill("Success rate: " + (successCount * 100 / iterations) + "%");
    vibez.spill("Average time per assertion: " + (duration * 1000 / iterations) + "μs");
    
    return ok<lit, Error>(successCount);
}

// Run all benchmarks
slay runAllBenchmarks(iterations lit) {
    // Initialize objects
    sus simpleImpl = SimpleImpl{value: 42};
    sus nestedImpl = NestedImpl{parentValue: 10, childValue: 20};
    sus diamondImpl = DiamondImpl{baseValue: 1, leftValue: 2, rightValue: 3};
    sus deepImpl = DeepNestedImpl{val1: 1, val2: 2, val3: 3, val4: 4, val5: 5};
    
    // Cast to interfaces
    sus simpleBase SimpleBase = simpleImpl;
    sus nestedParent NestedParent = nestedImpl;
    sus diamondBase DiamondBase = diamondImpl;
    sus deepLevel1 DeepLevel1 = deepImpl;
    
    // Run benchmarks
    vibez.spill("\n==== INTERFACE TYPE ASSERTION BENCHMARKS ====");
    vibez.spill("Running benchmarks with " + iterations + " iterations each\n");
    
    sus simpleResult = benchmarkSimple(simpleBase, iterations);
    vibez.spill("");
    
    sus nestedResult = benchmarkNested(nestedParent, iterations);
    vibez.spill("");
    
    sus diamondResult = benchmarkDiamond(diamondBase, iterations);
    vibez.spill("");
    
    sus deepResult = benchmarkDeepNested(deepLevel1, iterations);
    vibez.spill("");
    
    // Summary
    vibez.spill("\n==== BENCHMARK SUMMARY ====");
    vibez.spill("Simple:      " + simpleResult.value);
    vibez.spill("Nested:      " + nestedResult.value);
    vibez.spill("Diamond:     " + diamondResult.value);
    vibez.spill("Deep Nested: " + deepResult.value);
}

// Main function
slay main() {
    // Number of iterations for each benchmark
    sus iterations = 10000;
    
    // Run all benchmarks
    runAllBenchmarks(iterations);
}