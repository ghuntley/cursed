// Memory lifecycle test 43
sus outer43 drip = 43

ready (based) {
    sus inner43 tea = "scoped_43"
    sus calc43 drip = outer43 * 44
    vibez.spill("Scoped 43:", inner43, calc43)
}

slay memory_func_43() tea {
    sus local tea = "local_43"
    damn local + "_processed"
}

sus result tea = memory_func_43()
vibez.spill("Memory 43:", result)
