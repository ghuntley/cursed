// Memory lifecycle test 50
sus outer50 drip = 50

ready (based) {
    sus inner50 tea = "scoped_50"
    sus calc50 drip = outer50 * 51
    vibez.spill("Scoped 50:", inner50, calc50)
}

slay memory_func_50() tea {
    sus local tea = "local_50"
    damn local + "_processed"
}

sus result tea = memory_func_50()
vibez.spill("Memory 50:", result)
