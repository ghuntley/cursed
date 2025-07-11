fr fr Debug Filesystem Test
fr fr Testing to find parsing issue

vibez.spill("Debug test start")

sus timestamp thicc = 1704067200
vibez.spill("Timestamp: ")
vibez.spill(timestamp)

sus file_perms normie = 644
vibez.spill("File permissions: ")
vibez.spill(file_perms)

sus owner_perms normie = (file_perms / 100) % 10
vibez.spill("Owner permissions: ")
vibez.spill(owner_perms)

sus has_read lit = owner_perms >= 4
vibez.spill("Has read: ")
vibez.spill(has_read)

vibez.spill("Debug test complete")
