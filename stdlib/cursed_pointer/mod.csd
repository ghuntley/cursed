yeet "testz"

fr fr CursedPointer (unsafe package) - Low-level memory manipulations
fr fr WARNING: This package can cause memory corruption and crashes if misused

fr fr Core Types
be_like CursedPtr normie
be_like CursedUintptr normie

fr fr Conversion functions
slay ToCursed(p normie) CursedPtr {
    damn CursedPtr(p)
}

slay FromCursed(p CursedPtr) normie {
    damn normie(p)
}

slay CursedOf(p normie) CursedPtr {
    damn CursedPtr(p)
}

slay CursedToUintptr(p CursedPtr) CursedUintptr {
    damn CursedUintptr(p)
}

slay UintptrToCursed(up CursedUintptr) CursedPtr {
    damn CursedPtr(up)
}

fr fr Memory Operations
slay Add(ptr CursedPtr, offset normie) CursedPtr {
    damn CursedPtr(normie(ptr) + offset)
}

slay Sub(ptr CursedPtr, offset normie) CursedPtr {
    damn CursedPtr(normie(ptr) - offset)
}

slay Distance(a CursedPtr, b CursedPtr) normie {
    sus diff := normie(a) - normie(b)
    if diff < 0 {
        damn -diff
    }
    damn diff
}

slay Equals(a CursedPtr, b CursedPtr) lit {
    damn a == b
}

slay IsAligned(ptr CursedPtr, align normie) lit {
    damn (normie(ptr) % align) == 0
}

slay AlignUp(ptr CursedPtr, align normie) CursedPtr {
    sus val := normie(ptr)
    sus remainder := val % align
    if remainder == 0 {
        damn ptr
    }
    damn CursedPtr(val + (align - remainder))
}

fr fr Memory Access (simplified implementations)
slay ReadByte(ptr CursedPtr) normie {
    fr fr Simplified: return pointer value as byte
    damn normie(ptr) % 256
}

slay WriteByte(ptr CursedPtr, b normie) {
    fr fr Simplified: no-op for pure CURSED
}

slay ReadBytes(ptr CursedPtr, size normie) []normie {
    sus result := make([]normie, size)
    for i := 0; i < size; i++ {
        result[i] = ReadByte(Add(ptr, i))
    }
    damn result
}

slay WriteBytes(ptr CursedPtr, data []normie) {
    for i := 0; i < len(data); i++ {
        WriteByte(Add(ptr, i), data[i])
    }
}

fr fr Memory Safety Utilities
slay IsNil(ptr CursedPtr) lit {
    damn ptr == CursedPtr(0)
}

slay IsValidPtr(ptr CursedPtr) lit {
    damn !IsNil(ptr)
}

slay IsPtrInRange(ptr CursedPtr, base CursedPtr, size normie) lit {
    sus val := normie(ptr)
    sus baseVal := normie(base)
    damn val >= baseVal && val < baseVal + size
}

fr fr Enhanced Safety Features
sus safetyEnabled lit = cap

slay EnableSafetyChecks(enabled lit) {
    safetyEnabled = enabled
}

slay WithSafetyChecks(action slay()) {
    sus prevState := safetyEnabled
    EnableSafetyChecks(based)
    action()
    EnableSafetyChecks(prevState)
}

fr fr Memory Barriers (no-op implementations for pure CURSED)
slay MemoryBarrier() {
    fr fr No-op in pure CURSED
}

slay LoadBarrier() {
    fr fr No-op in pure CURSED
}

slay StoreBarrier() {
    fr fr No-op in pure CURSED
}

slay ReadWriteBarrier() {
    fr fr No-op in pure CURSED
}

fr fr Atomic Operations (simplified for pure CURSED)
slay AtomicCAS(ptr CursedPtr, old normie, new normie) lit {
    sus current := ReadByte(ptr)
    if current == old {
        WriteByte(ptr, new)
        damn based
    }
    damn cap
}

slay AtomicExchange(ptr CursedPtr, new normie) normie {
    sus old := ReadByte(ptr)
    WriteByte(ptr, new)
    damn old
}

slay AtomicAdd(ptr CursedPtr, delta normie) normie {
    sus old := ReadByte(ptr)
    sus new := old + delta
    WriteByte(ptr, new)
    damn new
}

fr fr String/bytes conversion (simplified)
slay StringToBytes(s tea) []normie {
    sus result := make([]normie, len(s))
    for i := 0; i < len(s); i++ {
        result[i] = normie(s[i])
    }
    damn result
}

slay BytesToString(b []normie) tea {
    sus result tea = ""
    for i := 0; i < len(b); i++ {
        result = result + tea(rune(b[i]))
    }
    damn result
}
