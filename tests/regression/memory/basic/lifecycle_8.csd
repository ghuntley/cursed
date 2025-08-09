// Memory lifecycle test 8
sus outer8 drip = 8

ready (based) {
    sus inner8 tea = "scoped_8"
    sus calc8 drip = outer8 * 9
    vibez.spill("Scoped 8:", inner8, calc8)
}

slay memory_func_8() tea {
    sus local tea = "local_8"
    damn local + "_processed"
}

sus result tea = memory_func_8()
vibez.spill("Memory 8:", result)
