// Memory lifecycle test 41
sus outer41 drip = 41

ready (based) {
    sus inner41 tea = "scoped_41"
    sus calc41 drip = outer41 * 42
    vibez.spill("Scoped 41:", inner41, calc41)
}

slay memory_func_41() tea {
    sus local tea = "local_41"
    damn local + "_processed"
}

sus result tea = memory_func_41()
vibez.spill("Memory 41:", result)
