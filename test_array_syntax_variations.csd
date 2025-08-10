fr fr Test both array syntax variations

sus array1 []tea = ["hello", "world"]          fr fr Standard CURSED syntax
sus array2 [tea] = ["test1", "test2"]          fr fr Non-standard but used in stdlib

vibez.spill("Array1:", array1[0])
vibez.spill("Array2:", array2[0])

slay test_standard_syntax(items []tea) tea {
    damn items[0]
}

slay test_nonstandard_syntax(items [tea]) tea {
    damn items[0]
}

sus result1 tea = test_standard_syntax(array1)
sus result2 tea = test_nonstandard_syntax(array2)

vibez.spill("Standard result:", result1)
vibez.spill("Non-standard result:", result2)
