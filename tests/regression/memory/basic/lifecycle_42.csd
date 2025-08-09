// Memory lifecycle test 42
sus outer42 drip = 42

ready (based) {
    sus inner42 tea = "scoped_42"
    sus calc42 drip = outer42 * 43
    vibez.spill("Scoped 42:", inner42, calc42)
}

slay memory_func_42() tea {
    sus local tea = "local_42"
    damn local + "_processed"
}

sus result tea = memory_func_42()
vibez.spill("Memory 42:", result)
