// Memory lifecycle test 38
sus outer38 drip = 38

ready (based) {
    sus inner38 tea = "scoped_38"
    sus calc38 drip = outer38 * 39
    vibez.spill("Scoped 38:", inner38, calc38)
}

slay memory_func_38() tea {
    sus local tea = "local_38"
    damn local + "_processed"
}

sus result tea = memory_func_38()
vibez.spill("Memory 38:", result)
