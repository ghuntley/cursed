// Memory stress test 39
slay create_strings_39(count drip) []tea {
    sus strings []tea = []
    sus j drip = 0
    bestie (j < count) {
        sus str tea = "string_" + to_str_drip(j) + "_test_39"
        strings = append_tea(strings, str)
        j = j + 1
    }
    damn strings
}

sus result []tea = create_strings_39(78)
vibez.spill("Memory stress 39:", len(result))
