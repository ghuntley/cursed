// Memory lifecycle test 20
sus outer20 drip = 20

ready (based) {
    sus inner20 tea = "scoped_20"
    sus calc20 drip = outer20 * 21
    vibez.spill("Scoped 20:", inner20, calc20)
}

slay memory_func_20() tea {
    sus local tea = "local_20"
    damn local + "_processed"
}

sus result tea = memory_func_20()
vibez.spill("Memory 20:", result)
