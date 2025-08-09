// Memory lifecycle test 7
sus outer7 drip = 7

ready (based) {
    sus inner7 tea = "scoped_7"
    sus calc7 drip = outer7 * 8
    vibez.spill("Scoped 7:", inner7, calc7)
}

slay memory_func_7() tea {
    sus local tea = "local_7"
    damn local + "_processed"
}

sus result tea = memory_func_7()
vibez.spill("Memory 7:", result)
