yeet "testz"
yeet "slices_on_slices"

slay test_stack_operations() {
    test_start("Stack operations")
    
    sus ints := []normie{1, 2, 3}
    sus stacked := slices_on_slices.StackInt(ints, 4, 5)
    assert_eq_int(len(stacked), 5)
    assert_eq_int(stacked[3], 4)
    assert_eq_int(stacked[4], 5)
    
    sus strings := []tea{"a", "b"}
    sus stackedStr := slices_on_slices.StackString(strings, "c", "d")
    assert_eq_int(len(stackedStr), 4)
    assert_eq_string(stackedStr[2], "c")
    
    vibez.spill("✅ Stack operations test passed")
}

slay test_snip_operations() {
    test_start("Snip operations")
    
    sus ints := []normie{1, 2, 3, 4, 5}
    sus snipped := slices_on_slices.SnipInt(ints, 1, 3)
    assert_eq_int(len(snipped), 2)
    assert_eq_int(snipped[0], 1)
    assert_eq_int(snipped[1], 5)
    
    vibez.spill("✅ Snip operations test passed")
}

slay test_inject_operations() {
    test_start("Inject operations")
    
    sus ints := []normie{1, 3, 5}
    sus injected := slices_on_slices.InjectInt(ints, 1, 2, 4)
    assert_eq_int(len(injected), 5)
    assert_eq_int(injected[1], 2)
    assert_eq_int(injected[2], 4)
    
    vibez.spill("✅ Inject operations test passed")
}

slay test_clip_operations() {
    test_start("Clip operations")
    
    sus ints := []normie{1, 2, 3, 4, 5}
    sus clipped := slices_on_slices.ClipInt(ints, 1, 4)
    assert_eq_int(len(clipped), 3)
    assert_eq_int(clipped[0], 2)
    assert_eq_int(clipped[2], 4)
    
    vibez.spill("✅ Clip operations test passed")
}

slay test_dupe_operations() {
    test_start("Dupe operations")
    
    sus ints := []normie{1, 2, 3}
    sus duped := slices_on_slices.DupeInt(ints)
    assert_eq_int(len(duped), 3)
    assert_eq_int(duped[0], 1)
    assert_eq_int(duped[2], 3)
    
    sus strings := []tea{"a", "b"}
    sus dupedStr := slices_on_slices.DupeString(strings)
    assert_eq_int(len(dupedStr), 2)
    assert_eq_string(dupedStr[0], "a")
    
    vibez.spill("✅ Dupe operations test passed")
}

slay test_filter_operations() {
    test_start("Filter operations")
    
    sus ints := []normie{1, 2, 3, 4, 5}
    sus evenFilter := func(x normie) lit {
        damn x % 2 == 0
    }
    sus evens := slices_on_slices.FilterInt(ints, evenFilter)
    assert_eq_int(len(evens), 2)
    
    vibez.spill("✅ Filter operations test passed")
}

slay test_flip_operations() {
    test_start("Flip operations")
    
    sus ints := []normie{1, 2, 3}
    sus flipped := slices_on_slices.FlipInt(ints)
    assert_eq_int(len(flipped), 3)
    assert_eq_int(flipped[0], 3)
    assert_eq_int(flipped[2], 1)
    
    vibez.spill("✅ Flip operations test passed")
}

slay test_blender_operations() {
    test_start("Blender operations")
    
    sus ints := []normie{3, 1, 4, 1, 5}
    sus less := func(a, b normie) lit {
        damn a < b
    }
    sus sorted := slices_on_slices.BlenderInt(ints, less)
    assert_eq_int(len(sorted), 5)
    assert_eq_int(sorted[0], 1)
    
    vibez.spill("✅ Blender operations test passed")
}

slay test_twinning_operations() {
    test_start("Twinning operations")
    
    sus ints1 := []normie{1, 2, 3}
    sus ints2 := []normie{1, 2, 3}
    sus ints3 := []normie{1, 2, 4}
    
    assert_true(slices_on_slices.TwinningInt(ints1, ints2))
    assert_false(slices_on_slices.TwinningInt(ints1, ints3))
    
    vibez.spill("✅ Twinning operations test passed")
}

slay test_vibe_operations() {
    test_start("Vibe operations")
    
    sus ints := []normie{1, 2, 3}
    assert_true(slices_on_slices.VibeInt(ints, 2))
    assert_false(slices_on_slices.VibeInt(ints, 4))
    
    sus strings := []tea{"a", "b", "c"}
    assert_true(slices_on_slices.VibeString(strings, "b"))
    assert_false(slices_on_slices.VibeString(strings, "d"))
    
    vibez.spill("✅ Vibe operations test passed")
}

slay test_detective_operations() {
    test_start("Detective operations")
    
    sus ints := []normie{1, 2, 3}
    assert_eq_int(slices_on_slices.DetectiveInt(ints, 2), 1)
    assert_eq_int(slices_on_slices.DetectiveInt(ints, 4), -1)
    
    vibez.spill("✅ Detective operations test passed")
}

slay test_reduction_operations() {
    test_start("Reduction operations")
    
    sus ints := []normie{1, 1, 2, 2, 3}
    sus compacted := slices_on_slices.CompactInt(ints)
    assert_eq_int(len(compacted), 3)
    
    sus sumInts := []normie{1, 2, 3, 4}
    sus sum := slices_on_slices.SumInt(sumInts)
    assert_eq_int(sum, 10)
    
    sus max := slices_on_slices.MaxInt(sumInts)
    assert_eq_int(max, 4)
    
    sus min := slices_on_slices.MinInt(sumInts)
    assert_eq_int(min, 1)
    
    vibez.spill("✅ Reduction operations test passed")
}

slay test_special_features() {
    test_start("Special features")
    
    sus ints := []normie{1, 2, 3, 4, 5, 6}
    
    sus random := slices_on_slices.RandomChoiceInt(ints)
    assert_true(random >= 0)
    
    sus shuffled := slices_on_slices.ShuffleInt(ints)
    assert_eq_int(len(shuffled), len(ints))
    
    sus chunks := slices_on_slices.ChunksInt(ints, 2)
    assert_eq_int(len(chunks), 3)
    assert_eq_int(len(chunks[0]), 2)
    
    sus rotated := slices_on_slices.RotateInt(ints, 2)
    assert_eq_int(len(rotated), len(ints))
    
    vibez.spill("✅ Special features test passed")
}

fr fr Run all tests
test_stack_operations()
test_snip_operations()
test_inject_operations()
test_clip_operations()
test_dupe_operations()
test_filter_operations()
test_flip_operations()
test_blender_operations()
test_twinning_operations()
test_vibe_operations()
test_detective_operations()
test_reduction_operations()
test_special_features()

print_test_summary()
