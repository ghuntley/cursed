yeet "testz"

squad LargeStruct {
    spill data []normie
    spill text tea
    spill nested [][]normie
}

test_start("Memory Allocation Test")

fr fr Allocate many objects
sus objects []LargeStruct = []
bestie i := 0; i < 1000; i = i + 1 {
    sus data []normie = []
    bestie j := 0; j < 100; j = j + 1 {
        data.push(j * i)
    }
    
    sus nested [][]normie = []
    bestie k := 0; k < 10; k = k + 1 {
        sus inner []normie = []
        bestie l := 0; l < 20; l = l + 1 {
            inner.push(l + k)
        }
        nested.push(inner)
    }
    
    sus obj = LargeStruct{
        data: data,
        text: "Object number " + (i as tea),
        nested: nested
    }
    objects.push(obj)
}

assert_eq_int(objects.len(), 1000)
vibez.spillf("Allocated {} large objects", objects.len())
print_test_summary()
