// Memory lifecycle test 13
sus outer13 drip = 13

ready (based) {
    sus inner13 tea = "scoped_13"
    sus calc13 drip = outer13 * 14
    vibez.spill("Scoped 13:", inner13, calc13)
}

slay memory_func_13() tea {
    sus local tea = "local_13"
    damn local + "_processed"
}

sus result tea = memory_func_13()
vibez.spill("Memory 13:", result)
