fr fr Test advanced function parsing with array parameters

sus globals []tea = ["test1", "test2", "test3"]

slay process_array(items []tea, filter_func tea) []tea {
    sus result []tea = []
    
    bestie i < len(items) {
        lowkey filter_func == "all" {
            result = append(result, items[i])
        } otherwise ready filter_func == "first" && i == 0 {
            result = append(result, items[i])
        }
        i = i + 1
    }
    
    damn result
}

slay test_complex_function(
    param1 []tea,
    param2 drip,
    param3 lit
) tea {
    lowkey param3 && len(param1) > param2 {
        damn param1[param2]
    }
    damn "default"
}

sus filtered []tea = process_array(globals, "all")
sus first_only []tea = process_array(globals, "first")

lowkey len(filtered) > 0 {
    vibez.spill("Filtered result:", filtered[0])
}

sus complex_result tea = test_complex_function(globals, 1, based)
vibez.spill("Complex function result:", complex_result)
