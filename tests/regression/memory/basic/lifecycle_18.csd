// Memory lifecycle test 18
sus outer18 drip = 18

ready (based) {
    sus inner18 tea = "scoped_18"
    sus calc18 drip = outer18 * 19
    vibez.spill("Scoped 18:", inner18, calc18)
}

slay memory_func_18() tea {
    sus local tea = "local_18"
    damn local + "_processed"
}

sus result tea = memory_func_18()
vibez.spill("Memory 18:", result)
