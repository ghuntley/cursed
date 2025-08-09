// Memory lifecycle test 49
sus outer49 drip = 49

ready (based) {
    sus inner49 tea = "scoped_49"
    sus calc49 drip = outer49 * 50
    vibez.spill("Scoped 49:", inner49, calc49)
}

slay memory_func_49() tea {
    sus local tea = "local_49"
    damn local + "_processed"
}

sus result tea = memory_func_49()
vibez.spill("Memory 49:", result)
