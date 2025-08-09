// Memory lifecycle test 5
sus outer5 drip = 5

ready (based) {
    sus inner5 tea = "scoped_5"
    sus calc5 drip = outer5 * 6
    vibez.spill("Scoped 5:", inner5, calc5)
}

slay memory_func_5() tea {
    sus local tea = "local_5"
    damn local + "_processed"
}

sus result tea = memory_func_5()
vibez.spill("Memory 5:", result)
