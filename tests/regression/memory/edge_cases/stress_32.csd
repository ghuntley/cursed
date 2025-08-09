// Memory stress test 32
slay create_strings_32(count drip) []tea {
    sus strings []tea = []
    sus j drip = 0
    bestie (j < count) {
        sus str tea = "string_" + to_str_drip(j) + "_test_32"
        strings = append_tea(strings, str)
        j = j + 1
    }
    damn strings
}

sus result []tea = create_strings_32(64)
vibez.spill("Memory stress 32:", len(result))
