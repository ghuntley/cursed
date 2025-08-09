// Memory stress test 47
slay create_strings_47(count drip) []tea {
    sus strings []tea = []
    sus j drip = 0
    bestie (j < count) {
        sus str tea = "string_" + to_str_drip(j) + "_test_47"
        strings = append_tea(strings, str)
        j = j + 1
    }
    damn strings
}

sus result []tea = create_strings_47(94)
vibez.spill("Memory stress 47:", len(result))
