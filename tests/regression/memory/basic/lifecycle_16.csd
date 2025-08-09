// Memory lifecycle test 16
sus outer16 drip = 16

ready (based) {
    sus inner16 tea = "scoped_16"
    sus calc16 drip = outer16 * 17
    vibez.spill("Scoped 16:", inner16, calc16)
}

slay memory_func_16() tea {
    sus local tea = "local_16"
    damn local + "_processed"
}

sus result tea = memory_func_16()
vibez.spill("Memory 16:", result)
