// Test generic function instantiation and monomorphization
slay generic_func[T](val T) T {
    vibez.spill("Generic function called with value:", val)
    damn val
}

slay compare[T](a T, b T) lit {
    damn a == b
}

slay max[T Comparable](a T, b T) T {
    ready (a > b) {
        damn a
    } otherwise {
        damn b
    }
}

squad Container[T] {
    spill data T
    spill count drip
    
    slay get() T {
        damn data
    }
    
    slay set(value T) {
        data = value
    }
}

slay main() {
    // Test generic function with different types
    sus int_result drip = generic_func[drip](42)
    sus str_result tea = generic_func[tea]("hello")
    sus bool_result lit = generic_func[lit](based)
    
    vibez.spill("Int result:", int_result)
    vibez.spill("String result:", str_result)
    vibez.spill("Bool result:", bool_result)
    
    // Test generic comparison
    sus same_ints lit = compare[drip](5, 5)
    sus same_strings lit = compare[tea]("test", "test")
    
    vibez.spill("Same ints:", same_ints)
    vibez.spill("Same strings:", same_strings)
    
    // Test constrained generics
    sus max_int drip = max[drip](10, 20)
    sus max_float meal = max[meal](3.14, 2.71)
    
    vibez.spill("Max int:", max_int)
    vibez.spill("Max float:", max_float)
    
    // Test generic struct
    sus int_container Container[drip] = Container[drip]{data: 100, count: 1}
    sus str_container Container[tea] = Container[tea]{data: "generic", count: 1}
    
    vibez.spill("Int container:", int_container.get())
    vibez.spill("String container:", str_container.get())
    
    int_container.set(200)
    str_container.set("updated")
    
    vibez.spill("Updated int:", int_container.get())
    vibez.spill("Updated string:", str_container.get())
}
