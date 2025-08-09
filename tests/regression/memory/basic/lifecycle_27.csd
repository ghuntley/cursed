// Memory lifecycle test 27
sus outer27 drip = 27

ready (based) {
    sus inner27 tea = "scoped_27"
    sus calc27 drip = outer27 * 28
    vibez.spill("Scoped 27:", inner27, calc27)
}

slay memory_func_27() tea {
    sus local tea = "local_27"
    damn local + "_processed"
}

sus result tea = memory_func_27()
vibez.spill("Memory 27:", result)
