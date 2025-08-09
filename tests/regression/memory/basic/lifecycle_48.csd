// Memory lifecycle test 48
sus outer48 drip = 48

ready (based) {
    sus inner48 tea = "scoped_48"
    sus calc48 drip = outer48 * 49
    vibez.spill("Scoped 48:", inner48, calc48)
}

slay memory_func_48() tea {
    sus local tea = "local_48"
    damn local + "_processed"
}

sus result tea = memory_func_48()
vibez.spill("Memory 48:", result)
