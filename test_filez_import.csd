fr fr TEST FILEZ MODULE IMPORT

yeet "vibez"
yeet "filez"

slay test_filez_basic() {
    vibez.spill("Testing filez module...")
    
    fr fr Test path separator
    sus sep tea = path_separator()
    vibez.spill("Path separator: '" + sep + "'")
    
    fr fr Test path joining
    sus parts []tea = ["home", "user", "documents"]
    sus joined tea = path_join(parts)
    vibez.spill("Joined path: " + joined)
    
    vibez.spill("Filez test completed!")
}

test_filez_basic()
