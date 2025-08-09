// Memory lifecycle test 29
sus outer29 drip = 29

ready (based) {
    sus inner29 tea = "scoped_29"
    sus calc29 drip = outer29 * 30
    vibez.spill("Scoped 29:", inner29, calc29)
}

slay memory_func_29() tea {
    sus local tea = "local_29"
    damn local + "_processed"
}

sus result tea = memory_func_29()
vibez.spill("Memory 29:", result)
