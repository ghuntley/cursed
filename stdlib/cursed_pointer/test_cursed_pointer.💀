yeet "testz"
yeet "cursed_pointer"

test_start("cursed_pointer basic conversions")

fr fr Test basic conversions
sus ptr := cursed_pointer.ToCursed(42)
sus back := cursed_pointer.FromCursed(ptr)
assert_eq_int(back, 42)

fr fr Test pointer arithmetic
sus ptr2 := cursed_pointer.Add(ptr, 10)
sus distance := cursed_pointer.Distance(ptr2, ptr)
assert_eq_int(distance, 10)

fr fr Test subtraction
sus ptr3 := cursed_pointer.Sub(ptr2, 5)
sus distance2 := cursed_pointer.Distance(ptr3, ptr)
assert_eq_int(distance2, 5)

test_start("cursed_pointer equality and nil checks")

fr fr Test equality
sus ptr1 := cursed_pointer.ToCursed(100)
sus ptr2 := cursed_pointer.ToCursed(100)
assert_true(cursed_pointer.Equals(ptr1, ptr2))

fr fr Test nil pointer
sus nilPtr := cursed_pointer.ToCursed(0)
assert_true(cursed_pointer.IsNil(nilPtr))
assert_false(cursed_pointer.IsValidPtr(nilPtr))

sus validPtr := cursed_pointer.ToCursed(42)
assert_false(cursed_pointer.IsNil(validPtr))
assert_true(cursed_pointer.IsValidPtr(validPtr))

test_start("cursed_pointer alignment")

fr fr Test alignment checking
sus aligned8 := cursed_pointer.ToCursed(64)  fr fr 64 is divisible by 8
assert_true(cursed_pointer.IsAligned(aligned8, 8))

sus unaligned := cursed_pointer.ToCursed(65)  fr fr 65 is not divisible by 8
assert_false(cursed_pointer.IsAligned(unaligned, 8))

fr fr Test alignment up
sus alignedUp := cursed_pointer.AlignUp(unaligned, 8)
assert_true(cursed_pointer.IsAligned(alignedUp, 8))

test_start("cursed_pointer memory operations")

fr fr Test byte operations
sus basePtr := cursed_pointer.ToCursed(1000)
cursed_pointer.WriteByte(basePtr, 42)
sus readValue := cursed_pointer.ReadByte(basePtr)
assert_eq_int(readValue, 42)

fr fr Test bytes operations
sus data := normie[value]{1, 2, 3, 4, 5}
cursed_pointer.WriteBytes(basePtr, data)
sus readData := cursed_pointer.ReadBytes(basePtr, 5)
assert_eq_int(len(readData), 5)
assert_eq_int(readData[0], 1)
assert_eq_int(readData[4], 5)

test_start("cursed_pointer range checking")

fr fr Test pointer range checking
sus base := cursed_pointer.ToCursed(100)
sus size := 50
sus inRange := cursed_pointer.Add(base, 25)
sus outRange := cursed_pointer.Add(base, 75)

assert_true(cursed_pointer.IsPtrInRange(inRange, base, size))
assert_false(cursed_pointer.IsPtrInRange(outRange, base, size))

test_start("cursed_pointer atomic operations")

fr fr Test atomic compare and swap
sus atomicPtr := cursed_pointer.ToCursed(2000)
cursed_pointer.WriteByte(atomicPtr, 10)
sus swapped := cursed_pointer.AtomicCAS(atomicPtr, 10, 20)
assert_true(swapped)

sus newValue := cursed_pointer.ReadByte(atomicPtr)
assert_eq_int(newValue, 20)

fr fr Test atomic exchange
sus oldValue := cursed_pointer.AtomicExchange(atomicPtr, 30)
assert_eq_int(oldValue, 20)
sus currentValue := cursed_pointer.ReadByte(atomicPtr)
assert_eq_int(currentValue, 30)

test_start("cursed_pointer string conversion")

fr fr Test string to bytes conversion
sus testString tea = "hello"
sus bytes := cursed_pointer.StringToBytes(testString)
assert_eq_int(len(bytes), 5)
assert_eq_int(bytes[0], 104)  fr fr 'h' ascii

fr fr Test bytes to string conversion
sus backToString := cursed_pointer.BytesToString(bytes)
assert_eq_string(backToString, "hello")

test_start("cursed_pointer safety features")

fr fr Test safety checks
cursed_pointer.EnableSafetyChecks(based)

fr fr Test with safety checks wrapper
cursed_pointer.WithSafetyChecks(slay() {
    sus ptr := cursed_pointer.ToCursed(42)
    assert_true(cursed_pointer.IsValidPtr(ptr))
})

print_test_summary()
