# Simple test for write barrier fix without stdlib dependencies

squad TestStruct {
    value drip
    next *TestStruct
}

slay main() drip {
    # Create struct instances
    sus obj1 *TestStruct = TestStruct{ value: 42, next: drip(0) }
    sus obj2 *TestStruct = TestStruct{ value: 24, next: drip(0) }
    
    # CRITICAL: This field assignment should trigger write barrier
    obj1.next = obj2
    
    # Verify assignment worked
    ready (obj1.next.value == 24) {
        damn 0  # Success
    } otherwise {
        damn 1  # Failure
    }
}
