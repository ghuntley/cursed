sus test_array []tea = ["hello", "world"]

lowkey test_array[0] == "hello" {
    vibez.spill("CURSED condition without parens works!")
}

ready test_array[1] == "world" {
    vibez.spill("Ready statement works!")
}

sus counter drip = 0
bestie counter < 3 {
    vibez.spill("Bestie loop iteration:", counter)
    counter = counter + 1
}

slay test_function(params []tea) tea {
    lowkey len(params) > 0 {
        damn params[0]
    }
    damn "empty"
}

sus result tea = test_function(test_array)
vibez.spill("Function result:", result)
