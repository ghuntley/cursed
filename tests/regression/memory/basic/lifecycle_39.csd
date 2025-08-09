// Memory lifecycle test 39
sus outer39 drip = 39

ready (based) {
    sus inner39 tea = "scoped_39"
    sus calc39 drip = outer39 * 40
    vibez.spill("Scoped 39:", inner39, calc39)
}

slay memory_func_39() tea {
    sus local tea = "local_39"
    damn local + "_processed"
}

sus result tea = memory_func_39()
vibez.spill("Memory 39:", result)
