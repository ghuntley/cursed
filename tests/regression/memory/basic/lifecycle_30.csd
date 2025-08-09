// Memory lifecycle test 30
sus outer30 drip = 30

ready (based) {
    sus inner30 tea = "scoped_30"
    sus calc30 drip = outer30 * 31
    vibez.spill("Scoped 30:", inner30, calc30)
}

slay memory_func_30() tea {
    sus local tea = "local_30"
    damn local + "_processed"
}

sus result tea = memory_func_30()
vibez.spill("Memory 30:", result)
