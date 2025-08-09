// Memory lifecycle test 36
sus outer36 drip = 36

ready (based) {
    sus inner36 tea = "scoped_36"
    sus calc36 drip = outer36 * 37
    vibez.spill("Scoped 36:", inner36, calc36)
}

slay memory_func_36() tea {
    sus local tea = "local_36"
    damn local + "_processed"
}

sus result tea = memory_func_36()
vibez.spill("Memory 36:", result)
