// Test [drip] syntax vs []drip syntax
sus old_style [drip] = [10, 20, 30]
sus new_style []drip = [40, 50, 60]

vibez.spill("Old style [drip]:", old_style)
vibez.spill("New style []drip:", new_style)
