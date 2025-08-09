// Memory lifecycle test 32
sus outer32 drip = 32

ready (based) {
    sus inner32 tea = "scoped_32"
    sus calc32 drip = outer32 * 33
    vibez.spill("Scoped 32:", inner32, calc32)
}

slay memory_func_32() tea {
    sus local tea = "local_32"
    damn local + "_processed"
}

sus result tea = memory_func_32()
vibez.spill("Memory 32:", result)
